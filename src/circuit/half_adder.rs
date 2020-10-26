use crate::gate::logic_gates::{xor2, and2};

pub struct HalfAdder {
    pub sum: u8,
    pub carry: u8
}

impl HalfAdder {
    pub fn new() -> HalfAdder {
        HalfAdder {
            sum: 0,
            carry: 0
        }
    }

    pub fn add(&mut self, input1: u8, input2: u8) {
        self.sum = xor2(input1, input2);
        self.carry = and2(input1, input2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_0_0() {
        let mut half_adder = HalfAdder::new();
        half_adder.add(0, 0);
        assert_eq!(half_adder.sum, 0);
        assert_eq!(half_adder.carry, 0);
    }

    #[test]
    fn add_0_1() {
        let mut half_adder = HalfAdder::new();
        half_adder.add(0, 1);
        assert_eq!(half_adder.sum, 1);
        assert_eq!(half_adder.carry, 0);
    }

    #[test]
    fn add_1_1() {
        let mut half_adder = HalfAdder::new();
        half_adder.add(1, 1);
        assert_eq!(half_adder.sum, 0);
        assert_eq!(half_adder.carry, 1);
    }
}