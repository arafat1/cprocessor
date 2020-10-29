pub fn not(input: u8) -> u8 {
    (input + 1) & 1
}

pub fn or2(inp1: u8, inp2: u8) -> u8 {
    inp1 | inp2
}

pub fn or3(inp1: u8, inp2: u8, inp3: u8) -> u8 {
    inp1 | inp2 | inp3
}

pub fn or4(inp1: u8, inp2: u8, inp3: u8, inp4: u8) -> u8 {
    inp1 | inp2 | inp3 | inp4
}

pub fn or5(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8) -> u8 {
    inp1 | inp2 | inp3 | inp4 | inp5
}

pub fn or6(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8) -> u8 {
    inp1 | inp2 | inp3 | inp4 | inp5 | inp6
}

pub fn or7(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8) -> u8 {
    inp1 | inp2 | inp3 | inp4 | inp5 | inp6 | inp7
}

pub fn or8(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8, inp8: u8) -> u8 {
    inp1 | inp2 | inp3 | inp4 | inp5 | inp6 | inp7 | inp8
}

pub fn and2(inp1: u8, inp2: u8) -> u8 {
    inp1 & inp2
}

pub fn and3(inp1: u8, inp2: u8, inp3: u8) -> u8 {
    inp1 & inp2 & inp3
}

pub fn and4(inp1: u8, inp2: u8, inp3: u8, inp4: u8) -> u8 {
    inp1 & inp2 & inp3 & inp4
}

pub fn and5(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8) -> u8 {
    inp1 & inp2 & inp3 & inp4 & inp5
}

pub fn and6(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8) -> u8 {
    inp1 & inp2 & inp3 & inp4 & inp5 & inp6
}

pub fn and7(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8) -> u8 {
    inp1 & inp2 & inp3 & inp4 & inp5 & inp6 & inp7
}

pub fn and8(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8, inp8: u8) -> u8 {
    inp1 & inp2 & inp3 & inp4 & inp5 & inp6 & inp7 & inp8
}

pub fn nor2(inp1: u8, inp2: u8) -> u8 {
    not(or2(inp1, inp2))
}

pub fn nor3(inp1: u8, inp2: u8, inp3: u8) -> u8 {
    not(or3(inp1, inp2, inp3))
}

pub fn nor4(inp1: u8, inp2: u8, inp3: u8, inp4: u8) -> u8 {
    not(or4(inp1, inp2, inp3, inp4))
}

pub fn nor5(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8) -> u8 {
    not(or5(inp1, inp2, inp3, inp4, inp5))
}

pub fn nor6(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8) -> u8 {
    not(or6(inp1, inp2, inp3, inp4, inp5, inp6))
}

pub fn nor7(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8) -> u8 {
    not(or7(inp1, inp2, inp3, inp4, inp5, inp6, inp7))
}

pub fn nor8(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8, inp8: u8) -> u8 {
    not(or8(inp1, inp2, inp3, inp4, inp5, inp6, inp7, inp8))
}

pub fn nand2(inp1: u8, inp2: u8) -> u8 {
    not(and2(inp1, inp2))
}

pub fn nand3(inp1: u8, inp2: u8, inp3: u8) -> u8 {
    not(and3(inp1, inp2, inp3))
}

pub fn nand4(inp1: u8, inp2: u8, inp3: u8, inp4: u8) -> u8 {
    not(and4(inp1, inp2, inp3, inp4))
}

pub fn nand5(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8) -> u8 {
    not(and5(inp1, inp2, inp3, inp4, inp5))
}

pub fn nand6(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8) -> u8 {
    not(and6(inp1, inp2, inp3, inp4, inp5, inp6))
}

pub fn nand7(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8) -> u8 {
    not(and7(inp1, inp2, inp3, inp4, inp5, inp6, inp7))
}

pub fn nand8(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8, inp8: u8) -> u8 {
    not(and8(inp1, inp2, inp3, inp4, inp5, inp6, inp7, inp8))
}

pub fn xor2(inp1: u8, inp2: u8) -> u8 {
    (inp1 + inp2) & 1
}

pub fn xor3(inp1: u8, inp2: u8, inp3: u8) -> u8 {
    (inp1 + inp2 + inp3) & 1
}

pub fn xor4(inp1: u8, inp2: u8, inp3: u8, inp4: u8) -> u8 {
    (inp1 + inp2 + inp3 + inp4) & 1
}

pub fn xor5(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8) -> u8 {
    (inp1 + inp2 + inp3 + inp4 + inp5) & 1
}

pub fn xor6(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8) -> u8 {
    (inp1 + inp2 + inp3 + inp4 + inp5 + inp6) & 1
}

pub fn xor7(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8) -> u8 {
    (inp1 + inp2 + inp3 + inp4 + inp5 + inp6 + inp7) & 1
}

pub fn xor8(inp1: u8, inp2: u8, inp3: u8, inp4: u8, inp5: u8, inp6: u8, inp7: u8, inp8: u8) -> u8 {
    (inp1 + inp2 + inp3 + inp4 + inp5 + inp6 + inp7 + inp8) & 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_input_0() {
        assert_eq!(not(0), 1);
    }

    #[test]
    fn not_input_1() {
        assert_eq!(not(1), 0);
    }

    #[test]
    fn or2_input_0_0() {
        assert_eq!(or2(0, 0), 0);
    }

    #[test]
    fn or2_input_1_0() {
        assert_eq!(or2(1, 0), 1);
    }

    #[test]
    fn or2_input_0_1() {
        assert_eq!(or2(0, 1), 1);
    }

    #[test]
    fn or2_input_1_1() {
        assert_eq!(or2(1, 1), 1);
    }

    #[test]
    fn and_input_0_0() {
        assert_eq!(and2(0, 0), 0);
    }

    #[test]
    fn and2_input_1_0() {
        assert_eq!(and2(1, 0), 0);
    }

    #[test]
    fn and2_input_0_1() {
        assert_eq!(and2(0, 1), 0);
    }

    #[test]
    fn and2_input_1_1() {
        assert_eq!(and2(1, 1), 1);
    }

    #[test]
    fn nor2_input_0_0() {
        assert_eq!(nor2(0, 0), 1);
    }

    #[test]
    fn nor2_input_1_0() {
        assert_eq!(nor2(1, 0), 0);
    }

    #[test]
    fn nor2_input_0_1() {
        assert_eq!(nor2(0, 1), 0);
    }

    #[test]
    fn nor2_input_1_1() {
        assert_eq!(nor2(1, 1), 0);
    }

    #[test]
    fn nand2_input_0_0() {
        assert_eq!(nand2(0, 0), 1);
    }

    #[test]
    fn nand2_input_1_0() {
        assert_eq!(nand2(1, 0), 1);
    }

    #[test]
    fn nand2_input_0_1() {
        assert_eq!(nand2(0, 1), 1);
    }

    #[test]
    fn nand2_input_1_1() {
        assert_eq!(nand2(1, 1), 0);
    }

    #[test]
    fn xor2_input_0_0() {
        assert_eq!(xor2(0, 0), 0);
    }

    #[test]
    fn xor2_input_1_0() {
        assert_eq!(xor2(1, 0), 1);
    }

    #[test]
    fn xor2_input_0_1() {
        assert_eq!(xor2(0, 1), 1);
    }

    #[test]
    fn xor2_input_1_1() {
        assert_eq!(xor2(1, 1), 0);
    }

    #[test]
    fn xor3_input_1_1_1() {
        assert_eq!(xor3(1, 1, 1), 1);
    }

    #[test]
    fn xor3_input_1_1_0() {
        assert_eq!(xor3(1, 1, 0), 0);
    }

    #[test]
    fn xor3_input_1_0_0() {
        assert_eq!(xor3(1, 0, 0), 1);
    }
}
