#[derive(Debug)]
struct Not {
    input: u8,
    output: u8,
}

pub fn not(input: u8) -> u8 {
    (input + 1) & 1
}

pub fn or2(input1: u8, input2: u8) -> u8 {
    input1 | input2
}

pub fn or3(input1: u8, input2: u8, input3: u8) -> u8 {
    input1 | input2 | input3
}

pub fn and2(input1: u8, input2: u8) -> u8 {
    input1 & input2
}

pub fn and8(i1: u8, i2: u8, i3: u8, i4: u8, i5: u8, i6: u8, i7: u8, i8: u8) -> u8 {
    i1 & i2 & i3 & i4 & i5 & i6 & i7 & i8
}

pub fn nor2(input1: u8, input2: u8) -> u8 {
    not(or2(input1, input2))
}

pub fn nand2(input1: u8, input2: u8) -> u8 {
    not(and2(input1, input2))
}

pub fn xor2(input1: u8, input2: u8) -> u8 {
    (input1 + input2) & 1
}

pub fn xor3(input1: u8, input2: u8, input3: u8) -> u8 {
    (input1 + input2 + input3) & 1
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