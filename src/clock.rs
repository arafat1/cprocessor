use std::borrow::{Borrow, BorrowMut};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum Edge {
    Rise,
    Fall,
    None,
}

pub struct Clock {
    level: Arc<Mutex<u8>>,
    edge: Arc<Mutex<Edge>>,
    stop: Arc<Mutex<bool>>,
    edge_ct: u64, //edge change time; in millisecond; should be << period
    period: u64,  // in millisecond
}

impl Clock {
    pub fn new(edge_ct: u64, period: u64) -> Clock {
        Clock {
            level: Arc::new(Mutex::new(0)),
            edge: Arc::new(Mutex::new(Edge::None)),
            stop: Arc::new(Mutex::new(false)),
            edge_ct,
            period,
        }
    }

    pub fn start(&mut self) {
        // reset before each start
        {
            let mut edge = self.edge.lock().unwrap();
            *edge = Edge::None;
        }
        {
            let mut level = self.level.lock().unwrap();
            *level = 0;
        }

        let level = Arc::clone(&self.level);
        let edge = Arc::clone(&self.edge);
        let stop = Arc::clone(&self.stop);
        let edge_ct = self.edge_ct;
        let period = self.period;

        thread::spawn(move || {
            loop {
                {
                    // break if stop is true
                    let status = stop.lock().unwrap();
                    if *status {
                        break;
                    }
                }
                {
                    // change edge to up or down
                    let mut edge = edge.lock().unwrap();
                    let mut level = level.lock().unwrap();
                    if *level == 0 {
                        *edge = Edge::Rise;
                    } else {
                        *edge = Edge::Fall;
                    }
                }
                thread::sleep(Duration::from_millis(edge_ct));
                {
                    // after edge transition, change edge to none
                    let mut edge = edge.lock().unwrap();
                    *edge = Edge::None;
                }
                {
                    // change level to 0 or 1
                    let mut level = level.lock().unwrap();
                    *level = (*level + 1) & 1;
                }
                thread::sleep(Duration::from_millis(period))
            }
        });
    }

    pub fn stop(&mut self) {
        let mut status = self.stop.lock().unwrap();
        *status = true;
    }
}
