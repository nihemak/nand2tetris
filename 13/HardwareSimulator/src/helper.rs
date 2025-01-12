use crate::boolean_logic::*;

pub fn u8_to_bits(input: u8) -> [bit; 8] {
    let mut bits = [false; 8];
    let mut bit = 0b0000000000000001;
    for i in 0..8 {
        bits[i] = (input & bit) > 0;
        bit <<= 1;
    }
    bits
}

pub fn u8_to_3bit(input: u8) -> [bit; 3] {
    let bits = u8_to_bits(input);
    let mut res = [false; 3];
    for i in 0..3 {
        res[i] = bits[i];
    }
    res
}

pub fn u8_to_6bit(input: u8) -> [bit; 6] {
    let bits = u8_to_bits(input);
    let mut res = [false; 6];
    for i in 0..6 {
        res[i] = bits[i];
    }
    res
}

pub fn u16_to_word(input: u16) -> word {
    let mut word = [false; 16];
    let mut bit = 0b0000000000000001;
    for i in 0..16 {
        word[i] = (input & bit) > 0;
        bit <<= 1;
    }
    word
}

pub fn word_to_u16(word: word) -> u16 {
    let mut n: u16 = 0b0000000000000000;
    let base: u32 = 2;
    for i in 0..16 {
        if word[i] {
            n += base.pow(i as u32) as u16;
        }
    }
    n
}

pub fn bit15_to_u16(word: [bit; 15]) -> u16 {
    let mut n: u16 = 0b0000000000000000;
    let base: u32 = 2;
    for i in 0..15 {
        if word[i] {
            n += base.pow(i as u32) as u16;
        }
    }
    n
}

pub fn bit14_to_u16(word: [bit; 14]) -> u16 {
    let mut n: u16 = 0b0000000000000000;
    let base: u32 = 2;
    for i in 0..14 {
        if word[i] {
            n += base.pow(i as u32) as u16;
        }
    }
    n
}

pub fn bit13_to_u16(word: [bit; 13]) -> u16 {
    let mut n: u16 = 0b0000000000000000;
    let base: u32 = 2;
    for i in 0..13 {
        if word[i] {
            n += base.pow(i as u32) as u16;
        }
    }
    n
}

pub fn bit12_to_u16(word: [bit; 12]) -> u16 {
    let mut n: u16 = 0b0000000000000000;
    let base: u32 = 2;
    for i in 0..12 {
        if word[i] {
            n += base.pow(i as u32) as u16;
        }
    }
    n
}

pub fn u16_to_9bit(input: u16) -> [bit; 9] {
    let word = u16_to_word(input);
    let mut res = [false; 9];
    for i in 0..9 {
        res[i] = word[i];
    }
    res
}

pub fn u16_to_12bit(input: u16) -> [bit; 12] {
    let word = u16_to_word(input);
    let mut res = [false; 12];
    for i in 0..12 {
        res[i] = word[i];
    }
    res
}

pub fn u16_to_13bit(input: u16) -> [bit; 13] {
    let word = u16_to_word(input);
    let mut res = [false; 13];
    for i in 0..13 {
        res[i] = word[i];
    }
    res
}

pub fn u16_to_14bit(input: u16) -> [bit; 14] {
    let word = u16_to_word(input);
    let mut res = [false; 14];
    for i in 0..14 {
        res[i] = word[i];
    }
    res
}

pub fn bit13_to_bit12(input: [bit; 13]) -> [bit; 12] {
    [
        input[0], input[1], input[2], input[3], 
        input[4], input[5], input[6], input[7], 
        input[8], input[9], input[10], input[11]
    ]
}

pub fn bit15_to_bit14(input: [bit; 15]) -> [bit; 14] {
    [
        input[0], input[1], input[2], input[3], 
        input[4], input[5], input[6], input[7], 
        input[8], input[9], input[10], input[11], 
        input[12], input[13]
    ]
}

pub fn bit15_to_bit13(input: [bit; 15]) -> [bit; 13] {
    [
        input[0], input[1], input[2], input[3], 
        input[4], input[5], input[6], input[7], 
        input[8], input[9], input[10], input[11], 
        input[12]
    ]
}

pub fn word_to_bit15(input: word) -> [bit; 15] {
    [
        input[0], input[1], input[2], input[3], 
        input[4], input[5], input[6], input[7], 
        input[8], input[9], input[10], input[11], 
        input[12], input[13], input[14]
    ]
}

pub fn word_to_bit13(input: word) -> [bit; 13] {
    bit15_to_bit13(word_to_bit15(input))
}

pub fn bit15_to_bit12(input: [bit; 15]) -> [bit; 12] {
    [
        input[0], input[1], input[2], input[3], 
        input[4], input[5], input[6], input[7], 
        input[8], input[9], input[10], input[11]
    ]
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case(0b0000_0000, [false; 8])]
    #[case(0b1111_1111, [true; 8])]
    fn test_u8_to_bits(#[case] input: u8, #[case] output: [bit; 8]) {
        assert_eq!(output, u8_to_bits(input));
    }
    #[rstest]
    #[case(0b0000_0000_0000_0000, [false; 16])]
    #[case(0b1111_1111_1111_1111, [true; 16])]
    #[case(
        0b1111_0000_1111_0000,
        [false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true]
    )]
    fn test_u16_to_word(#[case] input: u16, #[case] output: word) {
        assert_eq!(output, u16_to_word(input));
    }
    #[rstest]
    #[case([false; 16], 0b0000_0000_0000_0000)]
    #[case([true; 16], 0b1111_1111_1111_1111)]
    #[case(
        [false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true],
        0b1111_0000_1111_0000
    )]
    fn test_word_to_u16(#[case] input:word, #[case] output: u16) {
        assert_eq!(output, word_to_u16(input));
    }
}