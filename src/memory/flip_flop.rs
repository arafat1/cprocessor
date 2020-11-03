use crate::gate::logic_gates::{nand2, nor2, xor2};
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub struct SRNorLatch {
    q: u8,
    cq: u8,
}

impl SRNorLatch {
    pub fn new() -> SRNorLatch {
        SRNorLatch { q: 0, cq: 1 }
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
    cq: u8,
}

impl SRNandLatch {
    pub fn new() -> SRNandLatch {
        SRNandLatch { q: 0, cq: 1 }
    }

    pub fn input(&mut self, r: u8, s: u8) {
        self.q = nand2(r, self.cq);
        self.cq = nand2(s, self.q);
        // to ensure new cq is reflected
        self.q = nand2(r, self.cq);
    }
}

#[derive(Debug, PartialEq)]
pub struct DNandLatch {
    q: u8,
    cq: u8,
}

impl DNandLatch {
    pub fn new() -> DNandLatch {
        DNandLatch { q: 0, cq: 1 }
    }

    pub fn input(&mut self, d: u8) {
        let cd = 1 ^ d;
        self.q = nand2(cd, self.cq);
        self.cq = nand2(d, self.q);
        // to ensure new cq is reflected
        self.q = nand2(cd, self.cq);
    }
}

#[derive(Debug, PartialEq)]
pub struct LevelClockedNandLatch {
    q: u8,
    cq: u8,
}

impl LevelClockedNandLatch {
    pub fn new() -> LevelClockedNandLatch {
        LevelClockedNandLatch { q: 0, cq: 1 }
    }

    pub fn input(&mut self, r: u8, s: u8, clk: u8) {
        let cr = nand2(s, clk);
        let cs = nand2(r, clk);

        let mut sr_latch = SRNandLatch::new();
        sr_latch.input(cr, cs);

        self.q = sr_latch.q;
        self.cq = sr_latch.cq;
    }
}

#[derive(Debug, PartialEq)]
pub struct LevelClockedDNandLatch {
    q: u8,
    cq: u8,
}

impl LevelClockedDNandLatch {
    pub fn new() -> LevelClockedDNandLatch {
        LevelClockedDNandLatch { q: 0, cq: 1 }
    }

    pub fn input(&mut self, d: u8, clk: u8) {
        let cd = 1 ^ d;

        let mut lcn_latch = LevelClockedNandLatch::new();
        lcn_latch.input(d, cd, clk);

        self.q = lcn_latch.q;
        self.cq = lcn_latch.cq;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rs_nor_nochange_0_0() {
        let mut latch = SRNorLatch::new();
        latch.input(0, 0);
        assert_eq!(latch, SRNorLatch { q: 0, cq: 1 });
    }

    #[test]
    fn rs_nor_set_0_1() {
        let mut latch = SRNorLatch::new();
        latch.input(0, 1);
        assert_eq!(latch, SRNorLatch { q: 1, cq: 0 });
    }

    #[test]
    fn rs_nor_reset_1_0() {
        let mut latch = SRNorLatch::new();
        latch.input(1, 0);
        assert_eq!(latch, SRNorLatch { q: 0, cq: 1 });
    }

    #[test]
    fn rs_nor_race_1_1() {
        let mut latch = SRNorLatch::new();
        latch.input(1, 1);
        assert_eq!(latch, SRNorLatch { q: 0, cq: 0 });
    }

    #[test]
    fn rs_nand_nochange_1_1() {
        let mut latch = SRNandLatch::new();
        latch.input(1, 1);
        assert_eq!(latch, SRNandLatch { q: 0, cq: 1 });
    }

    #[test]
    fn rs_nand_set_0_1() {
        let mut latch = SRNandLatch::new();
        latch.input(0, 1);
        assert_eq!(latch, SRNandLatch { q: 1, cq: 0 });
    }

    #[test]
    fn rs_nand_reset_1_0() {
        let mut latch = SRNandLatch::new();
        latch.input(1, 0);
        assert_eq!(latch, SRNandLatch { q: 0, cq: 1 });
    }

    #[test]
    fn rs_nand_race_0_0() {
        let mut latch = SRNandLatch::new();
        latch.input(0, 0);
        assert_eq!(latch, SRNandLatch { q: 1, cq: 1 });
    }
}
