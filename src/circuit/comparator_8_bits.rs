use crate::register::register::Register8Bits;
use crate::gate::logic_gates::{and8, xor2, not};

/// Compare two 8 bit words and return 1 if they are equal else return 0
pub fn compare(r1: &Register8Bits, r2: &Register8Bits) -> u8 {
    and8(not(xor2(r1.a0, r2.a0)),
         not(xor2(r1.a1, r2.a1)),
         not(xor2(r1.a2, r2.a2)),
         not(xor2(r1.a3, r2.a3)),
         not(xor2(r1.a4, r2.a4)),
         not(xor2(r1.a5, r2.a5)),
         not(xor2(r1.a6, r2.a6)),
         not(xor2(r1.a7, r2.a7)))
}

#[cfg(test)]
mod tests {
     use super::*;

     #[test]
     fn compare_equal() {
        let r1 = Register8Bits::new();
        let r2 = Register8Bits::new();
        assert_eq!(compare(&r1, &r2), 1);
     }

    #[test]
    fn compare_not_equal() {
        let mut r1 = Register8Bits::new();
        let r2 = Register8Bits::new();
        r1.change_bit1(1);
        assert_eq!(compare(&r1, &r2), 0);
    }
}