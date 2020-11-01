use crate::gate::logic_gates::{xor2, nor2, nand2};
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub struct SRNorLatch {
    q: u8,
    cq: u8
}

impl SRNorLatch {
    pub fn new() -> SRNorLatch {
        SRNorLatch {
            q: 0,
            cq: 1
        }
    }

    pub fn input(&mut self, r: u8, s: u8) {
        self.q = nor2(r, self.cq);
        self.cq = nor2(s, self.q);
        // to ensure new cq is reflected
        self.q = nor2(r, self.cq);
    }
}

#[derive(Debug, PartialEq)]
pub struct SRNandLatch {
    q: u8,
    cq: u8
}

impl SRNandLatch {
    pub fn new() -> SRNandLatch {
        SRNandLatch {
            q: 0,
            cq: 1
        }
    }

    pub fn input(&mut self, r: u8, s: u8) {
        self.q = nand2(r, self.cq);
        self.cq = nand2(s, self.q);
        // to ensure new cq is reflected
        self.q = nand2(r, self.cq);
    }
}

#[derive(Debug, PartialEq)]
pub struct LevelClockedNandLatch {
    q: u8,
    cq: u8
}

impl LevelClockedNandLatch {
    pub fn new() -> LevelClockedNandLatch {
        LevelClockedNandLatch {
            q: 0,
            cq: 1
        }
    }

    pub fn input(&mut self, r: u8, s: u8, clk: u8) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rs_nor_nochange_0_0() {
        let mut latch = SRNorLatch::new();
        latch.input(0, 0);
        assert_eq!(latch, SRNorLatch {q: 0, cq: 1});
    }

    #[test]
    fn rs_nor_set_0_1() {
        let mut latch = SRNorLatch::new();
        latch.input(0, 1);
        assert_eq!(latch, SRNorLatch {q: 1, cq: 0});
    }

    #[test]
    fn rs_nor_reset_1_0() {
        let mut latch = SRNorLatch::new();
        latch.input(1, 0);
        assert_eq!(latch, SRNorLatch {q: 0, cq: 1});
    }

    #[test]
    fn rs_nor_race_1_1() {
        let mut latch = SRNorLatch::new();
        latch.input(1, 1);
        assert_eq!(latch, SRNorLatch {q: 0, cq: 0});
    }

    #[test]
    fn rs_nand_nochange_1_1() {
        let mut latch = SRNandLatch::new();
        latch.input(1, 1);
        assert_eq!(latch, SRNandLatch {q: 0, cq: 1});
    }

    #[test]
    fn rs_nand_set_0_1() {
        let mut latch = SRNandLatch::new();
        latch.input(0, 1);
        assert_eq!(latch, SRNandLatch {q: 1, cq: 0});
    }

    #[test]
    fn rs_nand_reset_1_0() {
        let mut latch = SRNandLatch::new();
        latch.input(1, 0);
        assert_eq!(latch, SRNandLatch {q: 0, cq: 1});
    }

    #[test]
    fn rs_nand_race_0_0() {
        let mut latch = SRNandLatch::new();
        latch.input(0, 0);
        assert_eq!(latch, SRNandLatch {q: 1, cq: 1});
    }
}