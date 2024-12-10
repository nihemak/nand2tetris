use crate::nand2tetris::helper::*;

pub type bit = bool;
pub type word = [bit; 16];

pub fn nand(a: bit, b: bit) -> bit {
    !(a && b)
}

pub fn not(a: bit) -> bit {
    nand(a, a)
}

pub fn and(a: bit, b: bit) -> bit {
    not(nand(a, b))
}

pub fn or(a: bit, b: bit) -> bit {
    not(and(not(a), not(b)))
}

pub fn xor(a: bit, b: bit) -> bit {
    or(and(a, not(b)), and(not(a), b))
}

pub fn mux(a: bit, b: bit, sel: bit) -> bit {
    and(or(a, sel), or(b, not(sel)))
}

pub fn dmux(input: bit, sel: bit) -> (bit, bit) {
    (and(input, not(sel)), and(input, sel))
}

pub fn not16(input: word) -> word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = not(input[i]);
    }
    word
}

pub fn and16(a: word, b: word) -> word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = and(a[i],  b[i]);
    }
    word
}

pub fn or16(a: word, b: word) -> word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = or(a[i],  b[i]);
    }
    word
}

pub fn mux16(a: word, b: word, sel: bit) -> word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = mux(a[i],  b[i],  sel);
    }
    word
}

pub fn or8way(input: [bit; 8]) -> bit {
    or(
        or(or(input[0], input[1]), or(input[2], input[3])), 
        or(or(input[4], input[5]), or(input[6], input[7]))
    )
}

pub fn mux4way16(a: word, b: word, c: word, d: word, sel: [bit; 2]) -> word {
    mux16(mux16(a, b, sel[0]), mux16(c, d, sel[0]), sel[1])
}

pub fn mux8way16(a: word, b: word, c: word, d: word, e: word, f: word, g: word, h: word, sel: [bit; 3]) -> word {
    let sel1 = [sel[0], sel[1]];
    mux16(
        mux4way16(a ,b, c, d, sel1),
        mux4way16(e ,f, g, h, sel1),
        sel[2]
    )
}

pub fn dmux4way(input: bit, sel: [bit; 2]) -> (bit, bit, bit, bit) {
    let (w0, w1) = dmux(input, sel[0]);
    let (a, c) = dmux(w0, sel[1]);
    let (b, d) = dmux(w1, sel[1]);
    (a, b, c, d)
}

pub fn dmux8way(input: bit, sel: [bit; 3]) -> (bit, bit, bit, bit, bit, bit, bit, bit) {
    let (w0, w1, w2, w3) = dmux4way(input, [sel[0], sel[1]]);
    let (a, e) = dmux(w0, sel[2]);
    let (b, f) = dmux(w1, sel[2]);
    let (c, g) = dmux(w2, sel[2]);
    let (d, h) = dmux(w3, sel[2]);
    (a, b, c, d, e, f, g, h)
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case((false, false), true)]
    #[case((false, true),  true)]
    #[case((true,  false), true)]
    #[case((true,  true),  false)]
    fn test_nand(#[case] input: (bit, bit), #[case] output: bit) {
        let (a, b) = input;
        assert_eq!(output, nand(a, b));
    }

    #[rstest]
    #[case(false, true)]
    #[case(true,  false)]
    fn test_not(#[case] input: bit, #[case] output: bit) {
        assert_eq!(output, not(input));
    }

    #[rstest]
    #[case((false, false), false)]
    #[case((false, true),  false)]
    #[case((true,  false), false)]
    #[case((true,  true),  true)]
    fn test_and(#[case] input: (bit, bit), #[case] output: bit) {
        let (a, b) = input;
        assert_eq!(output, and(a, b));
    }

    #[rstest]
    #[case((false, false), false)]
    #[case((false, true),  true)]
    #[case((true,  false), true)]
    #[case((true,  true),  true)]
    fn test_or(#[case] input: (bit, bit), #[case] output: bit) {
        let (a, b) = input;
        assert_eq!(output, or(a, b));
    }

    #[rstest]
    #[case((false, false), false)]
    #[case((false, true),  true)]
    #[case((true,  false), true)]
    #[case((true,  true),  false)]
    fn test_xor(#[case] input: (bit, bit), #[case] output: bit) {
        let (a, b) = input;
        assert_eq!(output, xor(a, b));
    }

    #[rstest]
    #[case((false, false, false), false)]
    #[case((false, true, false), false)]
    #[case((true, false, false), true)]
    #[case((true, true, false), true)]
    #[case((false, false, true), false)]
    #[case((false, true, true), true)]
    #[case((true, false, true), false)]
    #[case((true, true, true), true)]
    fn test_mux(#[case] input: (bit, bit, bit), #[case] output: bit) {
        let (a, b, sel) = input;
        assert_eq!(output, mux(a, b, sel));
    }

    #[rstest]
    #[case((false, false), (false, false))]
    #[case((true,  false), (true, false))]
    #[case((false, true),  (false, false))]
    #[case((true,  true),  (false, true))]
    fn test_dmux(#[case] input: (bit, bit), #[case] output: (bit, bit)) {
        let (x, sel) = input;
        assert_eq!(output, dmux(x, sel));
    }

    #[rstest]
    #[case(u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b1111_1111_1111_1111))]
    #[case(u16_to_word(0b1111_1111_1111_1111), u16_to_word(0b0000_0000_0000_0000))]
    #[case(u16_to_word(0b1010_1010_1010_1010), u16_to_word(0b0101_0101_0101_0101))]
    #[case(u16_to_word(0b0011_1100_1100_0011), u16_to_word(0b1100_0011_0011_1100))]
    #[case(u16_to_word(0b0001_0010_0011_0100), u16_to_word(0b1110_1101_1100_1011))]
    fn test_not16(#[case] input: word, #[case] output: word) {
        assert_eq!(output, not16(input));
    }

    #[rstest]
    #[case(
        (u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b0000_0000_0000_0000)),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b1111_1111_1111_1111)),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (u16_to_word(0b1111_1111_1111_1111), u16_to_word(0b1111_1111_1111_1111)),
        u16_to_word(0b1111_1111_1111_1111)
    )]
    #[case(
        (u16_to_word(0b1010_1010_1010_1010), u16_to_word(0b0101_0101_0101_0101)),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (u16_to_word(0b0011_1100_1100_0011), u16_to_word(0b0000_1111_1111_0000)),
        u16_to_word(0b0000_1100_1100_0000)
    )]
    #[case(
        (u16_to_word(0b0001_0010_0011_0100), u16_to_word(0b1001_1000_0111_0110)),
        u16_to_word(0b0001_0000_0011_0100)
    )]
    fn test_and16(#[case] input: (word, word), #[case] output: word) {
        let (a, b) = input;
        assert_eq!(output, and16(a, b));
    }

    #[rstest]
    #[case(
        (u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b0000_0000_0000_0000)),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b1111_1111_1111_1111)),
        u16_to_word(0b1111_1111_1111_1111)
    )]
    #[case(
        (u16_to_word(0b1111_1111_1111_1111), u16_to_word(0b1111_1111_1111_1111)),
        u16_to_word(0b1111_1111_1111_1111)
    )]
    #[case(
        (u16_to_word(0b1010_1010_1010_1010), u16_to_word(0b0101_0101_0101_0101)),
        u16_to_word(0b1111_1111_1111_1111)
    )]
    #[case(
        (u16_to_word(0b0011_1100_1100_0011), u16_to_word(0b0000_1111_1111_0000)),
        u16_to_word(0b0011_1111_1111_0011)
    )]
    #[case(
        (u16_to_word(0b0001_0010_0011_0100), u16_to_word(0b1001_1000_0111_0110)),
        u16_to_word(0b1001_1010_0111_0110)
    )]
    fn test_or16(#[case] input: (word, word), #[case] output: word) {
        let (a, b) = input;
        assert_eq!(output, or16(a, b));
    }

    #[rstest]
    #[case(
        (u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b0000_0000_0000_0000), false),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b0000_0000_0000_0000), true),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b0001_0010_0011_0100), false),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (u16_to_word(0b0000_0000_0000_0000), u16_to_word(0b0001_0010_0011_0100), true),
        u16_to_word(0b0001_0010_0011_0100)
    )]
    #[case(
        (u16_to_word(0b1001_1000_0111_0110), u16_to_word(0b0000_0000_0000_0000), false),
        u16_to_word(0b1001_1000_0111_0110)
    )]
    #[case(
        (u16_to_word(0b1001_1000_0111_0110), u16_to_word(0b0000_0000_0000_0000), true),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (u16_to_word(0b1010_1010_1010_1010), u16_to_word(0b0101_0101_0101_0101), false),
        u16_to_word(0b1010_1010_1010_1010)
    )]
    #[case(
        (u16_to_word(0b1010_1010_1010_1010), u16_to_word(0b0101_0101_0101_0101), true),
        u16_to_word(0b0101_0101_0101_0101)
    )]
    fn test_mux16(#[case] input: (word, word, bit), #[case] output: word) {
        let (a, b, sel) = input;
        assert_eq!(output, mux16(a, b, sel));
    }

    #[rstest]
    #[case(u8_to_bits(0b0000_0000), false)]
    #[case(u8_to_bits(0b1111_1111), true)]
    #[case(u8_to_bits(0b0001_0000), true)]
    #[case(u8_to_bits(0b0000_0001), true)]
    #[case(u8_to_bits(0b0010_0110), true)]
    fn test_or8way(#[case] input: [bit; 8], #[case] output: bit) {
        assert_eq!(output, or8way(input));
    }

    #[rstest]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [false, false]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [true, false]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [false, true]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [true, true]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b1001_1000_0111_0110), 
            u16_to_word(0b1010_1010_1010_1010), 
            u16_to_word(0b0101_0101_0101_0101), 
            [false, false]
        ),
        u16_to_word(0b0001_0010_0011_0100)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b1001_1000_0111_0110), 
            u16_to_word(0b1010_1010_1010_1010), 
            u16_to_word(0b0101_0101_0101_0101), 
            [true, false]
        ),
        u16_to_word(0b1001_1000_0111_0110)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b1001_1000_0111_0110), 
            u16_to_word(0b1010_1010_1010_1010), 
            u16_to_word(0b0101_0101_0101_0101), 
            [false, true]
        ),
        u16_to_word(0b1010_1010_1010_1010)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b1001_1000_0111_0110), 
            u16_to_word(0b1010_1010_1010_1010), 
            u16_to_word(0b0101_0101_0101_0101), 
            [true, true]
        ),
        u16_to_word(0b0101_0101_0101_0101)
    )]
    fn test_mux4way16(#[case] input: (word, word, word, word, [bit; 2]), #[case] output: word) {
        let (a, b, c, d, sel) = input;
        assert_eq!(output, mux4way16(a, b, c, d, sel));
    }

    #[rstest]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [false, false, false]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [true, false, false]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [false, true, false]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [true, true, false]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [false, false, true]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [true, false, true]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [false, true, true]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            u16_to_word(0b0000_0000_0000_0000), 
            [true, true, true]
        ),
        u16_to_word(0b0000_0000_0000_0000)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b0010_0011_0100_0101), 
            u16_to_word(0b0011_0100_0101_0110), 
            u16_to_word(0b0100_0101_0110_0111), 
            u16_to_word(0b0101_0110_0111_1000), 
            u16_to_word(0b0110_0111_1000_1001), 
            u16_to_word(0b0111_1000_1001_1010), 
            u16_to_word(0b1000_1001_1010_1011), 
            [false, false, false]
        ),
        u16_to_word(0b0001_0010_0011_0100)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b0010_0011_0100_0101), 
            u16_to_word(0b0011_0100_0101_0110), 
            u16_to_word(0b0100_0101_0110_0111), 
            u16_to_word(0b0101_0110_0111_1000), 
            u16_to_word(0b0110_0111_1000_1001), 
            u16_to_word(0b0111_1000_1001_1010), 
            u16_to_word(0b1000_1001_1010_1011), 
            [true, false, false]
        ),
        u16_to_word(0b0010_0011_0100_0101)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b0010_0011_0100_0101), 
            u16_to_word(0b0011_0100_0101_0110), 
            u16_to_word(0b0100_0101_0110_0111), 
            u16_to_word(0b0101_0110_0111_1000), 
            u16_to_word(0b0110_0111_1000_1001), 
            u16_to_word(0b0111_1000_1001_1010), 
            u16_to_word(0b1000_1001_1010_1011), 
            [false, true, false]
        ),
        u16_to_word(0b0011_0100_0101_0110)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b0010_0011_0100_0101), 
            u16_to_word(0b0011_0100_0101_0110), 
            u16_to_word(0b0100_0101_0110_0111), 
            u16_to_word(0b0101_0110_0111_1000), 
            u16_to_word(0b0110_0111_1000_1001), 
            u16_to_word(0b0111_1000_1001_1010), 
            u16_to_word(0b1000_1001_1010_1011), 
            [true, true, false]
        ),
        u16_to_word(0b0100_0101_0110_0111)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b0010_0011_0100_0101), 
            u16_to_word(0b0011_0100_0101_0110), 
            u16_to_word(0b0100_0101_0110_0111), 
            u16_to_word(0b0101_0110_0111_1000), 
            u16_to_word(0b0110_0111_1000_1001), 
            u16_to_word(0b0111_1000_1001_1010), 
            u16_to_word(0b1000_1001_1010_1011), 
            [false, false, true]
        ),
        u16_to_word(0b0101_0110_0111_1000)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b0010_0011_0100_0101), 
            u16_to_word(0b0011_0100_0101_0110), 
            u16_to_word(0b0100_0101_0110_0111), 
            u16_to_word(0b0101_0110_0111_1000), 
            u16_to_word(0b0110_0111_1000_1001), 
            u16_to_word(0b0111_1000_1001_1010), 
            u16_to_word(0b1000_1001_1010_1011), 
            [true, false, true]
        ),
        u16_to_word(0b0110_0111_1000_1001)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b0010_0011_0100_0101), 
            u16_to_word(0b0011_0100_0101_0110), 
            u16_to_word(0b0100_0101_0110_0111), 
            u16_to_word(0b0101_0110_0111_1000), 
            u16_to_word(0b0110_0111_1000_1001), 
            u16_to_word(0b0111_1000_1001_1010), 
            u16_to_word(0b1000_1001_1010_1011), 
            [false, true, true]
        ),
        u16_to_word(0b0111_1000_1001_1010)
    )]
    #[case(
        (
            u16_to_word(0b0001_0010_0011_0100), 
            u16_to_word(0b0010_0011_0100_0101), 
            u16_to_word(0b0011_0100_0101_0110), 
            u16_to_word(0b0100_0101_0110_0111), 
            u16_to_word(0b0101_0110_0111_1000), 
            u16_to_word(0b0110_0111_1000_1001), 
            u16_to_word(0b0111_1000_1001_1010), 
            u16_to_word(0b1000_1001_1010_1011), 
            [true, true, true]
        ),
        u16_to_word(0b1000_1001_1010_1011)
    )]
    fn test_mux8way16(#[case] input: (word, word, word, word, word, word, word, word, [bit; 3]), #[case] output: word) {
        let (a, b, c, d, e, f, g, h, sel) = input;
        assert_eq!(output, mux8way16(a, b, c, d, e, f, g, h, sel));
    }

    #[rstest]
    #[case((false, [false, false]), (false, false, false, false))]
    #[case((false, [true,  false]), (false, false, false, false))]
    #[case((false, [false, true]),  (false, false, false, false))]
    #[case((false, [true,  true]),  (false, false, false, false))]
    #[case((true,  [false, false]), (true,  false, false, false))]
    #[case((true,  [true,  false]), (false, true,  false, false))]
    #[case((true,  [false, true]),  (false, false, true,  false))]
    #[case((true,  [true,  true]),  (false, false, false, true))]
    fn test_dmux4way(#[case] input: (bit, [bit; 2]), #[case] output: (bit, bit, bit, bit)) {
        let (x, sel) = input;
        let (a, b, c, d) = output;
        assert_eq!((a, b, c, d), dmux4way(x, sel));
    }

    #[rstest]
    #[case((false, [false, false, false]), (false, false, false, false, false, false, false, false))]
    #[case((false, [true,  false, false]), (false, false, false, false, false, false, false, false))]
    #[case((false, [false, true,  false]), (false, false, false, false, false, false, false, false))]
    #[case((false, [true,  true,  false]), (false, false, false, false, false, false, false, false))]
    #[case((false, [false, false, true]),  (false, false, false, false, false, false, false, false))]
    #[case((false, [true,  false, true]),  (false, false, false, false, false, false, false, false))]
    #[case((false, [false, true,  true]),  (false, false, false, false, false, false, false, false))]
    #[case((false, [true,  true,  true]),  (false, false, false, false, false, false, false, false))]
    #[case((true,  [false, false, false]), (true,  false, false, false, false, false, false, false))]
    #[case((true,  [true,  false, false]), (false, true,  false, false, false, false, false, false))]
    #[case((true,  [false, true,  false]), (false, false, true,  false, false, false, false, false))]
    #[case((true,  [true,  true,  false]), (false, false, false, true,  false, false, false, false))]
    #[case((true,  [false, false, true]),  (false, false, false, false, true,  false, false, false))]
    #[case((true,  [true,  false, true]),  (false, false, false, false, false, true,  false, false))]
    #[case((true,  [false, true,  true]),  (false, false, false, false, false, false, true,  false))]
    #[case((true,  [true,  true,  true]),  (false, false, false, false, false, false, false, true))]
    fn test_dmux8way(#[case] input: (bit, [bit; 3]), #[case] output: (bit, bit, bit, bit, bit, bit, bit, bit)) {
        let (x, sel) = input;
        let (a, b, c, d, e, f, g, h) = output;
        assert_eq!((a, b, c, d, e, f, g, h), dmux8way(x, sel));
    }
}