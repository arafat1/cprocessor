use std::borrow::{Borrow, BorrowMut};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Edge {
    Rise,
    Fall,
    None,
}

pub struct Clock {
    level: Arc<Mutex<u8>>,
    edge: Arc<Mutex<Edge>>,
    stop: Arc<Mutex<bool>>,
    edge_tt: u64, // edge transition time; in millisecond; should be << period
    period: u64,  // in millisecond
}

impl Clock {
    pub fn new(edge_ct: u64, period: u64) -> Clock {
        Clock {
            level: Arc::new(Mutex::new(0)),
            edge: Arc::new(Mutex::new(Edge::None)),
            stop: Arc::new(Mutex::new(false)),
            edge_tt: edge_ct,
            period,
        }
    }

    pub fn start(&mut self) {
        // reset before each start
        let mut edge = self.edge.lock().unwrap();
        *edge = Edge::None;
        drop(edge);

        let mut level = self.level.lock().unwrap();
        *level = 0;
        drop(level);

        let c_level = Arc::clone(&self.level);
        let c_edge = Arc::clone(&self.edge);
        let c_stop = Arc::clone(&self.stop);
        let c_edge_tt = self.edge_tt;
        let c_period = self.period;

        thread::spawn(move || {
            loop {
                // break if stop is true
                let status = c_stop.lock().unwrap();
                if *status {
                    break;
                }
                drop(status);

                // change edge to rise or fall
                // FIXME: Deadlock prone
                let mut edge = c_edge.lock().unwrap();
                let mut level = c_level.lock().unwrap();
                if *level == 0 {
                    *edge = Edge::Rise;
                } else {
                    *edge = Edge::Fall;
                }
                drop(level);
                drop(edge);

                thread::sleep(Duration::from_millis(c_edge_tt));

                // after edge transition, change edge to none
                let mut edge = c_edge.lock().unwrap();
                *edge = Edge::None;
                drop(edge);

                // change level to 0 or 1
                let mut level = c_level.lock().unwrap();
                *level = (*level + 1) & 1;
                drop(level);

                thread::sleep(Duration::from_millis(c_period))
            }
        });
    }

    pub fn stop(&mut self) {
        let mut status = self.stop.lock().unwrap();
        *status = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rs_nor_nochange_0_0() {
        let mut clock = Clock::new(10, 100);
        let c_level = Arc::clone(&clock.level);

        clock.start();
        for i in 1..10 {
            thread::sleep(Duration::from_millis(clock.period + clock.edge_tt - 1));
            let level = c_level.lock().unwrap();
            if *level == 1 {
                print!("---------");
            } else {
                print!("_________");
            }
            drop(level);
        }
    }
}
