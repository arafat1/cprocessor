pub struct Register4Bits {
    pub a0: u8,
    pub a1: u8,
    pub a2: u8,
    pub a3: u8
}

impl Register4Bits {
    pub fn new() -> Register4Bits {
        Register4Bits {
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0
        }
    }

    pub fn change_bit1(&mut self, bit1: u8) {
        self.a0 = bit1;
    }
}

pub struct Register8Bits {
    pub a0: u8,
    pub a1: u8,
    pub a2: u8,
    pub a3: u8,
    pub a4: u8,
    pub a5: u8,
    pub a6: u8,
    pub a7: u8,
}

impl Register8Bits {
    pub fn new() -> Register8Bits {
        Register8Bits {
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
        }
    }

    pub fn change_bit1(&mut self, bit1: u8) {
        self.a0 = bit1;
    }
}
