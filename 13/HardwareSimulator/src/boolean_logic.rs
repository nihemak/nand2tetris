use crate::helper::*;

pub type Binary = bool;
pub type Word = [Binary; 16];

pub fn nand(a: Binary, b: Binary) -> Binary {
    !(a && b)
}

pub fn not(a: Binary) -> Binary {
    nand(a, a)
}

pub fn and(a: Binary, b: Binary) -> Binary {
    not(nand(a, b))
}

pub fn or(a: Binary, b: Binary) -> Binary {
    not(and(not(a), not(b)))
}

pub fn xor(a: Binary, b: Binary) -> Binary {
    or(and(a, not(b)), and(not(a), b))
}

pub fn mux(a: Binary, b: Binary, sel: Binary) -> Binary {
    and(or(a, sel), or(b, not(sel)))
}

pub fn mux_built_in(a: Binary, b: Binary, sel: Binary) -> Binary {
    (a || sel) && (b || !sel)
}

pub fn dmux(input: Binary, sel: Binary) -> (Binary, Binary) {
    (and(input, not(sel)), and(input, sel))
}

pub fn dmux_built_in(input: Binary, sel: Binary) -> (Binary, Binary) {    
    (input && !sel, input && sel)
}

pub fn not16(input: Word) -> Word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = not(input[i]);
    }
    word
}

pub fn not16_built_in(input: Word) -> Word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = !input[i];
    }
    word
}

pub fn and16(a: Word, b: Word) -> Word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = and(a[i],  b[i]);
    }
    word
}

pub fn and16_built_in(a: Word, b: Word) -> Word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = a[i] && b[i];
    }
    word
}

pub fn or16(a: Word, b: Word) -> Word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = or(a[i],  b[i]);
    }
    word
}

pub fn mux16(a: Word, b: Word, sel: Binary) -> Word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = mux(a[i],  b[i],  sel);
    }
    word
}

pub fn mux16_built_in(a: Word, b: Word, sel: Binary) -> Word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    for i in 0..16 {
        word[i] = mux_built_in(a[i],  b[i],  sel);
    }
    word
}

pub fn or8way(input: [Binary; 8]) -> Binary {
    or(
        or(or(input[0], input[1]), or(input[2], input[3])), 
        or(or(input[4], input[5]), or(input[6], input[7]))
    )
}

pub fn or8way_built_in(input: [Binary; 8]) -> Binary {
    input[0] || input[1] || input[2] || input[3] || input[4] || input[5] || input[6] || input[7]
}

pub fn mux4way16(a: Word, b: Word, c: Word, d: Word, sel: [Binary; 2]) -> Word {
    mux16(mux16(a, b, sel[0]), mux16(c, d, sel[0]), sel[1])
}

pub fn mux4way16_built_in(a: Word, b: Word, c: Word, d: Word, sel: [Binary; 2]) -> Word {
    mux16_built_in(mux16_built_in(a, b, sel[0]), mux16_built_in(c, d, sel[0]), sel[1])
}

pub fn mux8way16(a: Word, b: Word, c: Word, d: Word, e: Word, f: Word, g: Word, h: Word, sel: [Binary; 3]) -> Word {
    let sel1 = [sel[0], sel[1]];
    mux16(
        mux4way16(a ,b, c, d, sel1),
        mux4way16(e ,f, g, h, sel1),
        sel[2]
    )
}

pub fn mux8way16_built_in(a: Word, b: Word, c: Word, d: Word, e: Word, f: Word, g: Word, h: Word, sel: [Binary; 3]) -> Word {
    let sel1 = [sel[0], sel[1]];
    mux16_built_in(
        mux4way16_built_in(a ,b, c, d, sel1),
        mux4way16_built_in(e ,f, g, h, sel1),
        sel[2]
    )
}

pub fn dmux4way(input: Binary, sel: [Binary; 2]) -> (Binary, Binary, Binary, Binary) {
    let (w0, w1) = dmux(input, sel[0]);
    let (a, c) = dmux(w0, sel[1]);
    let (b, d) = dmux(w1, sel[1]);
    (a, b, c, d)
}

pub fn dmux4way_built_in(input: Binary, sel: [Binary; 2]) -> (Binary, Binary, Binary, Binary) {
    let (w0, w1) = dmux_built_in(input, sel[0]);
    let (a, c) = dmux_built_in(w0, sel[1]);
    let (b, d) = dmux_built_in(w1, sel[1]);
    (a, b, c, d)
}

pub fn dmux8way(input: Binary, sel: [Binary; 3]) -> (Binary, Binary, Binary, Binary, Binary, Binary, Binary, Binary) {
    let (w0, w1, w2, w3) = dmux4way(input, [sel[0], sel[1]]);
    let (a, e) = dmux(w0, sel[2]);
    let (b, f) = dmux(w1, sel[2]);
    let (c, g) = dmux(w2, sel[2]);
    let (d, h) = dmux(w3, sel[2]);
    (a, b, c, d, e, f, g, h)
}

pub fn dmux8way_built_in(input: Binary, sel: [Binary; 3]) -> (Binary, Binary, Binary, Binary, Binary, Binary, Binary, Binary) {
    let (w0, w1, w2, w3) = dmux4way_built_in(input, [sel[0], sel[1]]);
    let (a, e) = dmux_built_in(w0, sel[2]);
    let (b, f) = dmux_built_in(w1, sel[2]);
    let (c, g) = dmux_built_in(w2, sel[2]);
    let (d, h) = dmux_built_in(w3, sel[2]);
    (a, b, c, d, e, f, g, h)
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case((0, 0), 1)]
    #[case((0, 1), 1)]
    #[case((1, 0), 1)]
    #[case((1, 1), 0)]
    fn test_nand(#[case] input: (u8, u8), #[case] output: u8) {
        let (a, b) = input;
        let (a, b) = (u8_to_1bit(a), u8_to_1bit(b));
        let output = u8_to_1bit(output);
        assert_eq!(output, nand(a, b));
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 0)]
    fn test_not(#[case] input: u8, #[case] output: u8) {
        let input = u8_to_1bit(input);
        let output = u8_to_1bit(output);
        assert_eq!(output, not(input));
    }

    #[rstest]
    #[case((0, 0), 0)]
    #[case((0, 1), 0)]
    #[case((1, 0), 0)]
    #[case((1, 1), 1)]
    fn test_and(#[case] input: (u8, u8), #[case] output: u8) {
        let (a, b) = input;
        let (a, b) = (u8_to_1bit(a), u8_to_1bit(b));
        let output = u8_to_1bit(output);
        assert_eq!(output, and(a, b));
    }

    #[rstest]
    #[case((0, 0), 0)]
    #[case((0, 1), 1)]
    #[case((1, 0), 1)]
    #[case((1, 1), 1)]
    fn test_or(#[case] input: (u8, u8), #[case] output: u8) {
        let (a, b) = input;
        let (a, b) = (u8_to_1bit(a), u8_to_1bit(b));
        let output = u8_to_1bit(output);
        assert_eq!(output, or(a, b));
    }

    #[rstest]
    #[case((0, 0), 0)]
    #[case((0, 1), 1)]
    #[case((1, 0), 1)]
    #[case((1, 1), 0)]
    fn test_xor(#[case] input: (u8, u8), #[case] output: u8) {
        let (a, b) = input;
        let (a, b) = (u8_to_1bit(a), u8_to_1bit(b));
        let output = u8_to_1bit(output);
        assert_eq!(output, xor(a, b));
    }

    #[rstest]
    #[case((0, 0, 0), 0)]
    #[case((0, 1, 0), 0)]
    #[case((1, 0, 0), 1)]
    #[case((1, 1, 0), 1)]
    #[case((0, 0, 1), 0)]
    #[case((0, 1, 1), 1)]
    #[case((1, 0, 1), 0)]
    #[case((1, 1, 1), 1)]
    fn test_mux(#[case] input: (u8, u8, u8), #[case] output: u8) {
        let (a, b, sel) = input;
        let (a, b, sel) = (u8_to_1bit(a), u8_to_1bit(b), u8_to_1bit(sel));
        let output = u8_to_1bit(output);
        assert_eq!(output, mux(a, b, sel));
    }

    #[rstest]
    #[case((0, 0), (0, 0))]
    #[case((1, 0), (1, 0))]
    #[case((0, 1), (0, 0))]
    #[case((1, 1), (0, 1))]
    fn test_dmux(#[case] input: (u8, u8), #[case] output: (u8, u8)) {
        let (x, sel) = input;
        let (x, sel) = (u8_to_1bit(x), u8_to_1bit(sel));
        let output = (u8_to_1bit(output.0), u8_to_1bit(output.1));
        assert_eq!(output, dmux(x, sel));
    }

    #[rstest]
    #[case(0b0000_0000_0000_0000, 0b1111_1111_1111_1111)]
    #[case(0b1111_1111_1111_1111, 0b0000_0000_0000_0000)]
    #[case(0b1010_1010_1010_1010, 0b0101_0101_0101_0101)]
    #[case(0b0011_1100_1100_0011, 0b1100_0011_0011_1100)]
    #[case(0b0001_0010_0011_0100, 0b1110_1101_1100_1011)]
    fn test_not16(#[case] input: u16, #[case] output: u16) {
        let input = u16_to_word(input);
        let output = u16_to_word(output);
        assert_eq!(output, not16(input));
    }

    #[rstest]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111), 0b0000_0000_0000_0000)]
    #[case((0b1111_1111_1111_1111, 0b1111_1111_1111_1111), 0b1111_1111_1111_1111)]
    #[case((0b1010_1010_1010_1010, 0b0101_0101_0101_0101), 0b0000_0000_0000_0000)]
    #[case((0b0011_1100_1100_0011, 0b0000_1111_1111_0000), 0b0000_1100_1100_0000)]
    #[case((0b0001_0010_0011_0100, 0b1001_1000_0111_0110), 0b0001_0000_0011_0100)]
    fn test_and16(#[case] input: (u16, u16), #[case] output: u16) {
        let (a, b) = input;
        let (a, b) = (u16_to_word(a), u16_to_word(b));
        let output = u16_to_word(output);
        assert_eq!(output, and16(a, b));
    }

    #[rstest]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111), 0b1111_1111_1111_1111)]
    #[case((0b1111_1111_1111_1111, 0b1111_1111_1111_1111), 0b1111_1111_1111_1111)]
    #[case((0b1010_1010_1010_1010, 0b0101_0101_0101_0101), 0b1111_1111_1111_1111)]
    #[case((0b0011_1100_1100_0011, 0b0000_1111_1111_0000), 0b0011_1111_1111_0011)]
    #[case((0b0001_0010_0011_0100, 0b1001_1000_0111_0110), 0b1001_1010_0111_0110)]
    fn test_or16(#[case] input: (u16, u16), #[case] output: u16) {
        let (a, b) = input;
        let (a, b) = (u16_to_word(a), u16_to_word(b));
        let output = u16_to_word(output);
        assert_eq!(output, or16(a, b));
    }

    #[rstest]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 1), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b0001_0010_0011_0100, 0), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b0001_0010_0011_0100, 1), 0b0001_0010_0011_0100)]
    #[case((0b1001_1000_0111_0110, 0b0000_0000_0000_0000, 0), 0b1001_1000_0111_0110)]
    #[case((0b1001_1000_0111_0110, 0b0000_0000_0000_0000, 1), 0b0000_0000_0000_0000)]
    #[case((0b1010_1010_1010_1010, 0b0101_0101_0101_0101, 0), 0b1010_1010_1010_1010)]
    #[case((0b1010_1010_1010_1010, 0b0101_0101_0101_0101, 1), 0b0101_0101_0101_0101)]
    fn test_mux16(#[case] input: (u16, u16, u8), #[case] output: u16) {
        let (a, b, sel) = input;
        let (a, b, sel) = (u16_to_word(a), u16_to_word(b), u8_to_1bit(sel));
        let output = u16_to_word(output);
        assert_eq!(output, mux16(a, b, sel));
    }

    #[rstest]
    #[case(0b0000_0000, 0)]
    #[case(0b1111_1111, 1)]
    #[case(0b0001_0000, 1)]
    #[case(0b0000_0001, 1)]
    #[case(0b0010_0110, 1)]
    fn test_or8way(#[case] input: u8, #[case] output: u8) {
        let input = u8_to_bits(input);
        let output = u8_to_1bit(output);
        assert_eq!(output, or8way(input));
    }

    #[rstest]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, [0, 0]), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, [1, 0]), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, [0, 1]), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, [1, 1]), 0b0000_0000_0000_0000)]
    #[case((0b0001_0010_0011_0100, 0b1001_1000_0111_0110, 0b1010_1010_1010_1010, 0b0101_0101_0101_0101, [0, 0]), 0b0001_0010_0011_0100)]
    #[case((0b0001_0010_0011_0100, 0b1001_1000_0111_0110, 0b1010_1010_1010_1010, 0b0101_0101_0101_0101, [1, 0]), 0b1001_1000_0111_0110)]
    #[case((0b0001_0010_0011_0100, 0b1001_1000_0111_0110, 0b1010_1010_1010_1010, 0b0101_0101_0101_0101, [0, 1]), 0b1010_1010_1010_1010)]
    #[case((0b0001_0010_0011_0100, 0b1001_1000_0111_0110, 0b1010_1010_1010_1010, 0b0101_0101_0101_0101, [1, 1]), 0b0101_0101_0101_0101)]
    fn test_mux4way16(#[case] input: (u16, u16, u16, u16, [u8; 2]), #[case] output: u16) {
        let (a, b, c, d, sel) = input;
        let (a, b, c, d, sel) = (u16_to_word(a), u16_to_word(b), u16_to_word(c), u16_to_word(d), sel.map(u8_to_1bit));
        let output = u16_to_word(output);
        assert_eq!(output, mux4way16(a, b, c, d, sel));
    }

    #[rstest]
    #[case(
        (
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            [0, 0, 0]
        ),
        0b0000_0000_0000_0000
    )]
    #[case(
        (
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            [1, 0, 0]
        ),
        0b0000_0000_0000_0000
    )]
    #[case(
        (
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            [0, 1, 0]
        ),
        0b0000_0000_0000_0000
    )]
    #[case(
        (
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            [1, 1, 0]
        ),
        0b0000_0000_0000_0000
    )]
    #[case(
        (
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            [0, 0, 1]
        ),
        0b0000_0000_0000_0000
    )]
    #[case(
        (
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            [1, 0, 1]
        ),
        0b0000_0000_0000_0000
    )]
    #[case(
        (
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            [0, 1, 1]
        ),
        0b0000_0000_0000_0000
    )]
    #[case(
        (
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 0b0000_0000_0000_0000, 
            [1, 1, 1]
        ),
        0b0000_0000_0000_0000
    )]
    #[case(
        (
            0b0001_0010_0011_0100, 0b0010_0011_0100_0101, 0b0011_0100_0101_0110, 0b0100_0101_0110_0111, 
            0b0101_0110_0111_1000, 0b0110_0111_1000_1001, 0b0111_1000_1001_1010, 0b1000_1001_1010_1011, 
            [0, 0, 0]
        ),
        0b0001_0010_0011_0100
    )]
    #[case(
        (
            0b0001_0010_0011_0100, 0b0010_0011_0100_0101, 0b0011_0100_0101_0110, 0b0100_0101_0110_0111, 
            0b0101_0110_0111_1000, 0b0110_0111_1000_1001, 0b0111_1000_1001_1010, 0b1000_1001_1010_1011, 
            [1, 0, 0]
        ),
        0b0010_0011_0100_0101
    )]
    #[case(
        (
            0b0001_0010_0011_0100, 0b0010_0011_0100_0101, 0b0011_0100_0101_0110, 0b0100_0101_0110_0111, 
            0b0101_0110_0111_1000, 0b0110_0111_1000_1001, 0b0111_1000_1001_1010, 0b1000_1001_1010_1011, 
            [0, 1, 0]
        ),
        0b0011_0100_0101_0110
    )]
    #[case(
        (
            0b0001_0010_0011_0100, 0b0010_0011_0100_0101, 0b0011_0100_0101_0110, 0b0100_0101_0110_0111, 
            0b0101_0110_0111_1000, 0b0110_0111_1000_1001, 0b0111_1000_1001_1010, 0b1000_1001_1010_1011, 
            [1, 1, 0]
        ),
        0b0100_0101_0110_0111
    )]
    #[case(
        (
            0b0001_0010_0011_0100, 0b0010_0011_0100_0101, 0b0011_0100_0101_0110, 0b0100_0101_0110_0111, 
            0b0101_0110_0111_1000, 0b0110_0111_1000_1001, 0b0111_1000_1001_1010, 0b1000_1001_1010_1011, 
            [0, 0, 1]
        ),
        0b0101_0110_0111_1000
    )]
    #[case(
        (
            0b0001_0010_0011_0100, 0b0010_0011_0100_0101, 0b0011_0100_0101_0110, 0b0100_0101_0110_0111, 
            0b0101_0110_0111_1000, 0b0110_0111_1000_1001, 0b0111_1000_1001_1010, 0b1000_1001_1010_1011, 
            [1, 0, 1]
        ),
        0b0110_0111_1000_1001
    )]
    #[case(
        (
            0b0001_0010_0011_0100, 0b0010_0011_0100_0101, 0b0011_0100_0101_0110, 0b0100_0101_0110_0111,
            0b0101_0110_0111_1000, 0b0110_0111_1000_1001, 0b0111_1000_1001_1010, 0b1000_1001_1010_1011, 
            [0, 1, 1]
        ),
        0b0111_1000_1001_1010
    )]
    #[case(
        (
            0b0001_0010_0011_0100, 0b0010_0011_0100_0101, 0b0011_0100_0101_0110, 0b0100_0101_0110_0111, 
            0b0101_0110_0111_1000, 0b0110_0111_1000_1001, 0b0111_1000_1001_1010, 0b1000_1001_1010_1011, 
            [1, 1, 1]
        ),
        0b1000_1001_1010_1011
    )]
    fn test_mux8way16(#[case] input: (u16, u16, u16, u16, u16, u16, u16, u16, [u8; 3]), #[case] output: u16) {
        let (a, b, c, d, e, f, g, h, sel) = input;
        let (a, b, c, d, e, f, g, h, sel) = (
            u16_to_word(a), u16_to_word(b), u16_to_word(c), u16_to_word(d), 
            u16_to_word(e), u16_to_word(f), u16_to_word(g), u16_to_word(h), 
            sel.map(u8_to_1bit));
        let output = u16_to_word(output);
        assert_eq!(output, mux8way16(a, b, c, d, e, f, g, h, sel));
    }

    #[rstest]
    #[case((0, [0, 0]), (0, 0, 0, 0))]
    #[case((0, [1, 0]), (0, 0, 0, 0))]
    #[case((0, [0, 1]), (0, 0, 0, 0))]
    #[case((0, [1, 1]), (0, 0, 0, 0))]
    #[case((1, [0, 0]), (1, 0, 0, 0))]
    #[case((1, [1, 0]), (0, 1, 0, 0))]
    #[case((1, [0, 1]), (0, 0, 1, 0))]
    #[case((1, [1, 1]), (0, 0, 0, 1))]
    fn test_dmux4way(#[case] input: (u8, [u8; 2]), #[case] output: (u8, u8, u8, u8)) {
        let (x, sel) = input;
        let (x, sel) = (u8_to_1bit(x), sel.map(u8_to_1bit));
        let (a, b, c, d) = output;
        let (a, b, c, d) = (u8_to_1bit(a), u8_to_1bit(b), u8_to_1bit(c), u8_to_1bit(d));
        assert_eq!((a, b, c, d), dmux4way(x, sel));
    }

    #[rstest]
    #[case((0, [0, 0, 0]), (0, 0, 0, 0, 0, 0, 0, 0))]
    #[case((0, [1, 0, 0]), (0, 0, 0, 0, 0, 0, 0, 0))]
    #[case((0, [0, 1, 0]), (0, 0, 0, 0, 0, 0, 0, 0))]
    #[case((0, [1, 1, 0]), (0, 0, 0, 0, 0, 0, 0, 0))]
    #[case((0, [0, 0, 1]), (0, 0, 0, 0, 0, 0, 0, 0))]
    #[case((0, [1, 0, 1]), (0, 0, 0, 0, 0, 0, 0, 0))]
    #[case((0, [0, 1, 1]), (0, 0, 0, 0, 0, 0, 0, 0))]
    #[case((0, [1, 1, 1]), (0, 0, 0, 0, 0, 0, 0, 0))]
    #[case((1, [0, 0, 0]), (1, 0, 0, 0, 0, 0, 0, 0))]
    #[case((1, [1, 0, 0]), (0, 1, 0, 0, 0, 0, 0, 0))]
    #[case((1, [0, 1, 0]), (0, 0, 1, 0, 0, 0, 0, 0))]
    #[case((1, [1, 1, 0]), (0, 0, 0, 1, 0, 0, 0, 0))]
    #[case((1, [0, 0, 1]), (0, 0, 0, 0, 1, 0, 0, 0))]
    #[case((1, [1, 0, 1]), (0, 0, 0, 0, 0, 1, 0, 0))]
    #[case((1, [0, 1, 1]), (0, 0, 0, 0, 0, 0, 1, 0))]
    #[case((1, [1, 1, 1]), (0, 0, 0, 0, 0, 0, 0, 1))]
    fn test_dmux8way(#[case] input: (u8, [u8; 3]), #[case] output: (u8, u8, u8, u8, u8, u8, u8, u8)) {
        let (x, sel) = input;
        let (x, sel) = (u8_to_1bit(x), sel.map(u8_to_1bit));
        let (a, b, c, d, e, f, g, h) = output;
        let (a, b, c, d, e, f, g, h) = (
            u8_to_1bit(a), u8_to_1bit(b), u8_to_1bit(c), u8_to_1bit(d), 
            u8_to_1bit(e), u8_to_1bit(f), u8_to_1bit(g), u8_to_1bit(h)
        );
        assert_eq!((a, b, c, d, e, f, g, h), dmux8way(x, sel));
    }
}