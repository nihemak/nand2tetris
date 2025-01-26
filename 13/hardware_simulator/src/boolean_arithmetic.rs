use crate::boolean_logic::*;
use crate::helper::*;

pub fn half_adder(a: Binary, b: Binary) -> (Binary, Binary) {
    let sum = xor(a, b);
    let carry = and(a, b);
    (sum, carry)
}

pub fn half_adder_built_in(a: Binary, b: Binary) -> (Binary, Binary) {
    let sum = a ^ b;
    let carry = a && b;
    (sum, carry)
}

pub fn full_adder(a: Binary, b: Binary, c: Binary) -> (Binary, Binary) {
    let (sum0, carry0) = half_adder(a, b);
    let (sum, carry1) = half_adder(sum0, c);
    let carry = or(carry0, carry1);
    (sum, carry)
}

pub fn full_adder_built_in(a: Binary, b: Binary, c: Binary) -> (Binary, Binary) {
    let (sum0, carry0) = half_adder_built_in(a, b);
    let (sum, carry1) = half_adder_built_in(sum0, c);
    let carry = carry0 || carry1;
    (sum, carry)
}

pub fn add16(a: Word, b: Word) -> Word {
    let (sum0, carry0) = half_adder(a[0], b[0]);
    let (sum1, carry1) = full_adder(a[1], b[1], carry0);
    let (sum2, carry2) = full_adder(a[2], b[2], carry1);
    let (sum3, carry3) = full_adder(a[3], b[3], carry2);
    let (sum4, carry4) = full_adder(a[4], b[4], carry3);
    let (sum5, carry5) = full_adder(a[5], b[5], carry4);
    let (sum6, carry6) = full_adder(a[6], b[6], carry5);
    let (sum7, carry7) = full_adder(a[7], b[7], carry6);
    let (sum8, carry8) = full_adder(a[8], b[8], carry7);
    let (sum9, carry9) = full_adder(a[9], b[9], carry8);
    let (sum10, carry10) = full_adder(a[10], b[10], carry9);
    let (sum11, carry11) = full_adder(a[11], b[11], carry10);
    let (sum12, carry12) = full_adder(a[12], b[12], carry11);
    let (sum13, carry13) = full_adder(a[13], b[13], carry12);
    let (sum14, carry14) = full_adder(a[14], b[14], carry13);
    let (sum15, _carry15) = full_adder(a[15], b[15], carry14);
    [
        sum0, sum1, sum2, sum3, sum4, sum5, sum6, sum7,
        sum8, sum9, sum10, sum11, sum12, sum13, sum14, sum15,
    ]
}

pub fn add16_built_in(a: Word, b: Word) -> Word {
    let (sum0, carry0) = half_adder_built_in(a[0], b[0]);
    let (sum1, carry1) = full_adder_built_in(a[1], b[1], carry0);
    let (sum2, carry2) = full_adder_built_in(a[2], b[2], carry1);
    let (sum3, carry3) = full_adder_built_in(a[3], b[3], carry2);
    let (sum4, carry4) = full_adder_built_in(a[4], b[4], carry3);
    let (sum5, carry5) = full_adder_built_in(a[5], b[5], carry4);
    let (sum6, carry6) = full_adder_built_in(a[6], b[6], carry5);
    let (sum7, carry7) = full_adder_built_in(a[7], b[7], carry6);
    let (sum8, carry8) = full_adder_built_in(a[8], b[8], carry7);
    let (sum9, carry9) = full_adder_built_in(a[9], b[9], carry8);
    let (sum10, carry10) = full_adder_built_in(a[10], b[10], carry9);
    let (sum11, carry11) = full_adder_built_in(a[11], b[11], carry10);
    let (sum12, carry12) = full_adder_built_in(a[12], b[12], carry11);
    let (sum13, carry13) = full_adder_built_in(a[13], b[13], carry12);
    let (sum14, carry14) = full_adder_built_in(a[14], b[14], carry13);
    let (sum15, _carry15) = full_adder_built_in(a[15], b[15], carry14);
    [
        sum0, sum1, sum2, sum3, sum4, sum5, sum6, sum7,
        sum8, sum9, sum10, sum11, sum12, sum13, sum14, sum15,
    ]
}

pub fn inc16(input: Word) -> Word {
    add16(input, u16_to_word(0b0000_0000_0000_0001))
}

pub fn alu(
    x: Word,
    y: Word, 
    zx: Binary, 
    nx: Binary, 
    zy: Binary, 
    ny: Binary, 
    f: Binary, 
    no: Binary
) -> (Word, Binary, Binary) {
    let x1 = mux16(x, u16_to_word(0b0000_0000_0000_0000), zx);
    let x2 = mux16(x1, not16(x1), nx);
    let y1 = mux16(y, u16_to_word(0b0000_0000_0000_0000), zy);
    let y2 = mux16(y1, not16(y1), ny);

    let xy = mux16(and16(x2, y2), add16(x2, y2), f);
    let output = mux16(xy, not16(xy), no);

    let output1 = [output[0], output[1], output[2], output[3], output[4], output[5], output[6], output[7]];
    let output2 = [output[8], output[9], output[10], output[11], output[12], output[13], output[14], output[15]];
    let zr = not(or(or8way(output1), or8way(output2)));
    let ng = output[15];
    (output, zr, ng)
}

pub fn alu_built_in(
    x: Word,
    y: Word, 
    zx: Binary, 
    nx: Binary, 
    zy: Binary, 
    ny: Binary, 
    f: Binary, 
    no: Binary
) -> (Word, Binary, Binary) {
    let x1 = mux16_built_in(x, u16_to_word(0b0000_0000_0000_0000), zx);
    let x2 = mux16_built_in(x1, not16(x1), nx);
    let y1 = mux16_built_in(y, u16_to_word(0b0000_0000_0000_0000), zy);
    let y2 = mux16_built_in(y1, not16(y1), ny);

    let xy = mux16_built_in(and16_built_in(x2, y2), add16_built_in(x2, y2), f);
    let output = mux16_built_in(xy, not16_built_in(xy), no);

    let output1 = [output[0], output[1], output[2], output[3], output[4], output[5], output[6], output[7]];
    let output2 = [output[8], output[9], output[10], output[11], output[12], output[13], output[14], output[15]];
    let zr = !(or8way_built_in(output1) || or8way_built_in(output2));
    let ng = output[15];
    (output, zr, ng)
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case((0, 0), (0, 0))]
    #[case((0, 1), (1, 0))]
    #[case((1, 0), (1, 0))]
    #[case((1, 1), (0, 1))]
    fn test_half_adder(#[case] input: (u8, u8), #[case] output: (u8, u8)) {
        let (a, b) = input;
        let (a, b) = (u8_to_1bit(a), u8_to_1bit(b));
        let (sum, carry) = output;
        let (sum, carry) = (u8_to_1bit(sum), u8_to_1bit(carry));
        assert_eq!((sum, carry), half_adder(a, b));
    }

    #[rstest]
    #[case((0, 0, 0), (0, 0))]
    #[case((0, 0, 1), (1, 0))]
    #[case((0, 1, 0), (1, 0))]
    #[case((0, 1, 1), (0, 1))]
    #[case((1, 0, 0), (1, 0))]
    #[case((1, 0, 1), (0, 1))]
    #[case((1, 1, 0), (0, 1))]
    #[case((1, 1, 1), (1, 1))]
    fn test_full_adder(#[case] input: (u8, u8, u8), #[case] output: (u8, u8)) {
        let (a, b, c) = input;
        let (a, b, c) = (u8_to_1bit(a), u8_to_1bit(b), u8_to_1bit(c));
        let (sum, carry) = output;
        let (sum, carry) = (u8_to_1bit(sum), u8_to_1bit(carry));
        assert_eq!((sum, carry), full_adder(a, b, c));
    }

    #[rstest]
    #[case((0b0000_0000_0000_0000, 0b0000_0000_0000_0000), 0b0000_0000_0000_0000)]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111), 0b1111_1111_1111_1111)]
    #[case((0b1111_1111_1111_1111, 0b1111_1111_1111_1111), 0b1111_1111_1111_1110)]
    #[case((0b1010_1010_1010_1010, 0b0101_0101_0101_0101), 0b1111_1111_1111_1111)]
    #[case((0b0011_1100_1100_0011, 0b0000_1111_1111_0000), 0b0100_1100_1011_0011)]
    #[case((0b0001_0010_0011_0100, 0b1001_1000_0111_0110), 0b1010_1010_1010_1010)]
    fn test_add16(#[case] input: (u16, u16), #[case] output: u16) {
        let (a, b) = input;
        let (a, b) = (u16_to_word(a), u16_to_word(b));
        let output = u16_to_word(output);
        assert_eq!(output, add16(a, b));
    }

    #[rstest]
    #[case(0b0000_0000_0000_0000, 0b0000_0000_0000_0001)]
    #[case(0b1111_1111_1111_1111, 0b0000_0000_0000_0000)]
    #[case(0b0000_0000_0000_0101, 0b0000_0000_0000_0110)]
    #[case(0b1111_1111_1111_1011, 0b1111_1111_1111_1100)]
    fn test_inc16(#[case] input: u16, #[case] output: u16) {
        let input = u16_to_word(input);
        let output = u16_to_word(output);
        assert_eq!(output, inc16(input));
    }

    #[rstest]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 1, 0, 1, 0, 1, 0), (0b0000_0000_0000_0000, 1, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 1, 1, 1, 1, 1, 1), (0b0000_0000_0000_0001, 0, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 1, 1, 1, 0, 1, 0), (0b1111_1111_1111_1111, 0, 1))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 0, 1, 1, 0, 0), (0b0000_0000_0000_0000, 1, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 1, 1, 0, 0, 0, 0), (0b1111_1111_1111_1111, 0, 1))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 0, 1, 1, 0, 1), (0b1111_1111_1111_1111, 0, 1))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 1, 1, 0, 0, 0, 1), (0b0000_0000_0000_0000, 1, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 0, 1, 1, 1, 1), (0b0000_0000_0000_0000, 1, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 1, 1, 0, 0, 1, 1), (0b0000_0000_0000_0001, 0, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 1, 1, 1, 1, 1), (0b0000_0000_0000_0001, 0, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 1, 1, 0, 1, 1, 1), (0b0000_0000_0000_0000, 1, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 0, 1, 1, 1, 0), (0b1111_1111_1111_1111, 0, 1))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 1, 1, 0, 0, 1, 0), (0b1111_1111_1111_1110, 0, 1))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 0, 0, 0, 1, 0), (0b1111_1111_1111_1111, 0, 1))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 1, 0, 0, 1, 1), (0b0000_0000_0000_0001, 0, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 0, 0, 1, 1, 1), (0b1111_1111_1111_1111, 0, 1))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 0, 0, 0, 0, 0), (0b0000_0000_0000_0000, 1, 0))]
    #[case((0b0000_0000_0000_0000, 0b1111_1111_1111_1111, 0, 1, 0, 1, 0, 1), (0b1111_1111_1111_1111, 0, 1))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 1, 0, 1, 0, 1, 0), (0b0000_0000_0000_0000, 1, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 1, 1, 1, 1, 1, 1), (0b0000_0000_0000_0001, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 1, 1, 1, 0, 1, 0), (0b1111_1111_1111_1111, 0, 1))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 0, 1, 1, 0, 0), (0b0000_0000_0001_0001, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 1, 1, 0, 0, 0, 0), (0b0000_0000_0000_0011, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 0, 1, 1, 0, 1), (0b1111_1111_1110_1110, 0, 1))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 1, 1, 0, 0, 0, 1), (0b1111_1111_1111_1100, 0, 1))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 0, 1, 1, 1, 1), (0b1111_1111_1110_1111, 0, 1))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 1, 1, 0, 0, 1, 1), (0b1111_1111_1111_1101, 0, 1))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 1, 1, 1, 1, 1), (0b0000_0000_0001_0010, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 1, 1, 0, 1, 1, 1), (0b0000_0000_0000_0100, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 0, 1, 1, 1, 0), (0b0000_0000_0001_0000, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 1, 1, 0, 0, 1, 0), (0b0000_0000_0000_0010, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 0, 0, 0, 1, 0), (0b0000_0000_0001_0100, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 1, 0, 0, 1, 1), (0b0000_0000_0000_1110, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 0, 0, 1, 1, 1), (0b1111_1111_1111_0010, 0, 1))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 0, 0, 0, 0, 0), (0b0000_0000_0000_0001, 0, 0))]
    #[case((0b0000_0000_0001_0001, 0b0000_0000_0000_0011, 0, 1, 0, 1, 0, 1), (0b0000_0000_0001_0011, 0, 0))]
    fn test_alu(#[case] input: (u16, u16, u8, u8, u8, u8, u8, u8), #[case] output: (u16, u8, u8)) {
        let (x, y, zx, nx, zy, ny, f, no) = input;
        let (x, y, zx, nx, zy, ny, f, no) = (
            u16_to_word(x), u16_to_word(y), u8_to_1bit(zx), u8_to_1bit(nx), 
            u8_to_1bit(zy), u8_to_1bit(ny), u8_to_1bit(f), u8_to_1bit(no)
        );
        let (o, zr, ng) = output;
        let (o, zr, ng) = (u16_to_word(o), u8_to_1bit(zr), u8_to_1bit(ng));
        assert_eq!((o, zr, ng), alu(x, y, zx, nx, zy, ny, f, no));
    }
}
