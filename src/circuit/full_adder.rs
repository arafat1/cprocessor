use crate::gate::logic_gates::{xor3, or3, and2};

pub struct FullAdder {
    pub sum: u8,
    pub carry: u8
}

impl FullAdder {
    pub fn new() -> FullAdder {
        FullAdder {
            sum: 0,
            carry: 0
        }
    }

    pub fn add(&mut self, input1: u8, input2: u8, carry: u8) {
        self.sum = xor3(input1, input2, carry);
        self.carry = or3(and2(input1, input2),
                         and2(input1, carry),
                         and2(input2, carry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_0_0_0() {
        let mut full_adder = FullAdder::new();
        full_adder.add(0, 0, 0);
        assert_eq!(full_adder.sum, 0);
        assert_eq!(full_adder.carry, 0);
    }

    #[test]
    fn add_0_0_1() {
        let mut full_adder = FullAdder::new();
        full_adder.add(0, 0, 1);
        assert_eq!(full_adder.sum, 1);
        assert_eq!(full_adder.carry, 0);
    }

    #[test]
    fn add_0_1_1() {
        let mut full_adder = FullAdder::new();
        full_adder.add(0, 1, 1);
        assert_eq!(full_adder.sum, 0);
        assert_eq!(full_adder.carry, 1);
    }

    #[test]
    fn add_1_1_1() {
        let mut full_adder = FullAdder::new();
        full_adder.add(1, 1, 1);
        assert_eq!(full_adder.sum, 1);
        assert_eq!(full_adder.carry, 1);
    }
}