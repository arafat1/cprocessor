use crate::circuit::full_adder::FullAdder;
use crate::circuit::half_adder::HalfAdder;
use crate::memory::register::Register4Bits;

pub fn add4(r1: &Register4Bits, r2: &Register4Bits) -> Register4Bits {
    let mut s0 = HalfAdder::new();
    s0.add(r1.a0, r2.a0);

    let mut s1 = FullAdder::new();
    s1.add(r1.a1, r2.a1, s0.carry);

    let mut s2 = FullAdder::new();
    s2.add(r1.a2, r2.a2, s1.carry);

    let mut s3 = FullAdder::new();
    s3.add(r1.a3, r2.a3, s2.carry);

    Register4Bits {
        a0: s0.sum,
        a1: s1.sum,
        a2: s2.sum,
        a3: s3.sum,
    }
}
