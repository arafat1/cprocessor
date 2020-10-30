use crate::memory::register::Register4Bits;
use crate::gate::logic_gates::xor2;
use crate::circuit::full_adder::FullAdder;

pub fn add_sub4(r1: &Register4Bits, r2: &Register4Bits, sub: u8) -> Register4Bits {
    let in0 = xor2(r2.a0, sub);
    let mut s0 = FullAdder::new();
    s0.add(r1.a0, in0, sub);

    let in1 = xor2(r2.a1, sub);
    let mut s1 = FullAdder::new();
    s1.add(r1.a1, in1, s0.carry);

    let in2 = xor2(r2.a2, sub);
    let mut s2 = FullAdder::new();
    s2.add(r1.a2, in2, s1.carry);

    let in3 = xor2(r2.a3, sub);
    let mut s3 = FullAdder::new();
    s2.add(r1.a3, in3, s2.carry);

    Register4Bits {
        a0: s0.sum,
        a1: s1.sum,
        a2: s2.sum,
        a3: s3.sum
    }
}
