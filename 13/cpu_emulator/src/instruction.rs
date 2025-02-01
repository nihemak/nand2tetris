#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    A(u16),
    C(Comp, Dest, Jump),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Comp {
    Zero,       /* 0 */
    One,        /* 1 */
    MinusOne,   /* -1 */
    D,          /* D */
    A,          /* A */
    M,          /* M */
    NotD,       /* !D */
    NotA,       /* !A */
    NotM,       /* !M */
    MinusD,     /* -D */
    MinusA,     /* -A */
    MinusM,     /* -M */
    DPlusOne,   /* D+1 */
    APlusOne,   /* A+1 */
    MPlusOne,   /* M+1 */
    DMinusOne,  /* D-1 */
    AMinusOne,  /* A-1 */
    MMinusOne,  /* M-1 */
    DPlusA,     /* D+A */
    DPlusM,     /* D+M */
    DMinusA,    /* D-A */
    DMinusM,    /* D-M */
    AMinusD,    /* A-D */
    MMinusD,    /* M-D */
    DAndA,      /* D&A */
    DAndM,      /* D&M */
    DOrA,       /* D|A */
    DOrM,       /* D|M */
}

#[derive(Clone, PartialEq, Debug)]
pub enum Dest {
    Null,           /* null */
    RamA,           /* RAM[A] */
    D,              /* D */
    DAndRamA,       /* D, RAM[A] */
    A,              /* A */
    AAndRamA,       /* A, RAM[A] */
    AAndD,          /* A, D */
    AAndDAndRamA,   /* A, D, RAM[A] */
}

#[derive(Clone, PartialEq, Debug)]
pub enum Jump {
    None,                   /* none */
    GreaterThan,            /* if comp > 0 jump */
    EqualTo,                /* if comp = 0 jump */
    GreaterThanAndEqualTo,  /* if comp >= 0 jump */
    LessThan,               /* if comp < 0 jump */
    NotEqualTo,             /* if comp != 0 jump */
    LessThanAndEqualTo,     /* if comp <= 0 jump */
    True,                   /* if true jump */
}

impl Instruction {
    pub fn new(instruction: &str) -> Self {
        let inst = Self::decode_to_binary(instruction);
        if Self::is_a_instruction(inst) {
            Self::A(inst)
        }
        else if Self::is_c_instruction(inst) {
            Self::C(
                Self::decode_c_comp(inst),
                Self::decode_c_dest(inst),
                Self::decode_c_jump(inst),
            )
        }
        else {
            panic!("error: instruction kind {:#018b}", inst);
        }
    }

    fn decode_to_binary(instruction: &str) -> u16 {
        let mut inst: u16 = 0b0000_0000_0000_0000;
        let mut bit: u16 = 0b1000_0000_0000_0000;
        for (_, c) in instruction.chars().enumerate() {
            if c == '1' { inst |= bit; }
            bit >>= 1;
        }
        inst
    }

    fn is_a_instruction(inst: u16) -> bool {
        inst & 0b1000_0000_0000_0000 == 0b0000_0000_0000_0000
    }

    fn is_c_instruction(inst: u16) -> bool {
        inst & 0b1110_0000_0000_0000 == 0b1110_0000_0000_0000
    }

    fn decode_c_comp(inst: u16) -> Comp {
        let comp = (inst & 0b0001_1111_1100_0000) >> 6;
        match comp {
            0b0_101010 => Comp::Zero,        /* 0 */
            0b0_111111 => Comp::One,         /* 1 */
            0b0_111010 => Comp::MinusOne,    /* -1 */
            0b0_001100 => Comp::D,           /* D */
            0b0_110000 => Comp::A,           /* A */
            0b1_110000 => Comp::M,           /* M */
            0b0_001101 => Comp::NotD,        /* !D */
            0b0_110001 => Comp::NotA,        /* !A */
            0b1_110001 => Comp::NotM,        /* !M */
            0b0_001111 => Comp::MinusD,      /* -D */
            0b0_110011 => Comp::MinusA,      /* -A */
            0b1_110011 => Comp::MinusM,      /* -M */
            0b0_011111 => Comp::DPlusOne,    /* D+1 */
            0b0_110111 => Comp::APlusOne,    /* A+1 */
            0b1_110111 => Comp::MPlusOne,    /* M+1 */
            0b0_001110 => Comp::DMinusOne,   /* D-1 */
            0b0_110010 => Comp::AMinusOne,   /* A-1 */
            0b1_110010 => Comp::MMinusOne,   /* M-1 */
            0b0_000010 => Comp::DPlusA,      /* D+A */
            0b1_000010 => Comp::DPlusM,      /* D+M */
            0b0_010011 => Comp::DMinusA,     /* D-A */
            0b1_010011 => Comp::DMinusM,     /* D-M */
            0b0_000111 => Comp::AMinusD,     /* A-D */
            0b1_000111 => Comp::MMinusD,     /* M-D */
            0b0_000000 => Comp::DAndA,       /* D&A */
            0b1_000000 => Comp::DAndM,       /* D&M */
            0b0_010101 => Comp::DOrA,        /* D|A */
            0b1_010101 => Comp::DOrM,        /* D|M */
            _ => panic!("error: comp {:#018b}", comp),
        }
    }

    fn decode_c_dest(inst: u16) -> Dest {
        let dest = (inst & 0b0000_0000_0011_1000) >> 3;
        match dest {
            0b000 => Dest::Null,            /* null */
            0b001 => Dest::RamA,            /* RAM[A] */
            0b010 => Dest::D,               /* D */
            0b011 => Dest::DAndRamA,        /* D, RAM[A] */
            0b100 => Dest::A,               /* A */
            0b101 => Dest::AAndRamA,        /* A, RAM[A] */
            0b110 => Dest::AAndD,           /* A, D */
            0b111 => Dest::AAndDAndRamA,    /* A, D, RAM[A] */
            _ => panic!("error: dest {:#018b}", dest),
        }
    }

    fn decode_c_jump(inst: u16) -> Jump {
        let jump = inst & 0b0000_0000_0000_0111;
        match jump {
            0b000 => Jump::None,                    /* none */
            0b001 => Jump::GreaterThan,             /* if comp > 0 jump */
            0b010 => Jump::EqualTo,                 /* if comp = 0 jump */
            0b011 => Jump::GreaterThanAndEqualTo,   /* if comp >= 0 jump */
            0b100 => Jump::LessThan,                /* if comp < 0 jump */
            0b101 => Jump::NotEqualTo,              /* if comp != 0 jump */
            0b110 => Jump::LessThanAndEqualTo,      /* if comp <= 0 jump */
            0b111 => Jump::True,                    /* if true jump */
            _ => panic!("error: jump {:#018b}", jump),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case("0000000000000000", 0b0000_0000_0000_0000)]
    #[case("1111111111111111", 0b1111_1111_1111_1111)]
    #[case("0110000000000000", 0b0110_0000_0000_0000)]
    #[case("1111110000010000", 0b1111_1100_0001_0000)]
    #[case("0000000000001000", 0b0000_0000_0000_1000)]
    fn test_decode_to_binary(#[case] input: &str, #[case] output: u16) {
        assert_eq!(output, Instruction::decode_to_binary(input));
    }

    #[rstest]
    #[case(0b0000_0000_0000_0000, true)]
    #[case(0b1110_0000_0000_0000, false)]
    fn test_is_a_instruction(#[case] input: u16, #[case] output: bool) {
        assert_eq!(output, Instruction::is_a_instruction(input));
    }

    #[rstest]
    #[case(0b0000_0000_0000_0000, false)]
    #[case(0b1110_0000_0000_0000, true)]
    fn test_is_c_instruction(#[case] input: u16, #[case] output: bool) {
        assert_eq!(output, Instruction::is_c_instruction(input));
    }

    #[rstest]
    #[case(0b1110_1010_1000_0000, Comp::Zero)]
    #[case(0b1110_1111_1100_0000, Comp::One)]
    #[case(0b1110_1110_1000_0000, Comp::MinusOne)]
    #[case(0b1110_0011_0000_0000, Comp::D)]
    #[case(0b1110_1100_0000_0000, Comp::A)]
    #[case(0b1111_1100_0000_0000, Comp::M)]
    #[case(0b1110_0011_0100_0000, Comp::NotD)]
    #[case(0b1110_1100_0100_0000, Comp::NotA)]
    #[case(0b1111_1100_0100_0000, Comp::NotM)]
    #[case(0b1110_0011_1100_0000, Comp::MinusD)]
    #[case(0b1110_1100_1100_0000, Comp::MinusA)]
    #[case(0b1111_1100_1100_0000, Comp::MinusM)]
    #[case(0b1110_0111_1100_0000, Comp::DPlusOne)]
    #[case(0b1110_1101_1100_0000, Comp::APlusOne)]
    #[case(0b1111_1101_1100_0000, Comp::MPlusOne)]
    #[case(0b1110_0011_1000_0000, Comp::DMinusOne)]
    #[case(0b1110_1100_1000_0000, Comp::AMinusOne)]
    #[case(0b1111_1100_1000_0000, Comp::MMinusOne)]
    #[case(0b1110_0000_1000_0000, Comp::DPlusA)]
    #[case(0b1111_0000_1000_0000, Comp::DPlusM)]
    #[case(0b1110_0100_1100_0000, Comp::DMinusA)]
    #[case(0b1111_0100_1100_0000, Comp::DMinusM)]
    #[case(0b1110_0001_1100_0000, Comp::AMinusD)]
    #[case(0b1111_0001_1100_0000, Comp::MMinusD)]
    #[case(0b1110_0000_0000_0000, Comp::DAndA)]
    #[case(0b1111_0000_0000_0000, Comp::DAndM)]
    #[case(0b1110_0101_0100_0000, Comp::DOrA)]
    #[case(0b1111_0101_0100_0000, Comp::DOrM)]
    fn test_decode_c_comp(#[case] input: u16, #[case] output: Comp) {
        assert_eq!(output, Instruction::decode_c_comp(input));
    }

    #[rstest]
    #[case(0b1110_0000_0000_0000, Dest::Null)]
    #[case(0b1110_0000_0000_1000, Dest::RamA)]
    #[case(0b1110_0000_0001_0000, Dest::D)]
    #[case(0b1110_0000_0001_1000, Dest::DAndRamA)]
    #[case(0b1110_0000_0010_0000, Dest::A)]
    #[case(0b1110_0000_0010_1000, Dest::AAndRamA)]
    #[case(0b1110_0000_0011_0000, Dest::AAndD)]
    #[case(0b1110_0000_0011_1000, Dest::AAndDAndRamA)]
    fn test_decode_c_dest(#[case] input: u16, #[case] output: Dest) {
        assert_eq!(output, Instruction::decode_c_dest(input));
    }

    #[rstest]
    #[case(0b1110_0000_0000_0000, Jump::None)]
    #[case(0b1110_0000_0000_0001, Jump::GreaterThan)]
    #[case(0b1110_0000_0000_0010, Jump::EqualTo)]
    #[case(0b1110_0000_0000_0011, Jump::GreaterThanAndEqualTo)]
    #[case(0b1110_0000_0000_0100, Jump::LessThan)]
    #[case(0b1110_0000_0000_0101, Jump::NotEqualTo)]
    #[case(0b1110_0000_0000_0110, Jump::LessThanAndEqualTo)]
    #[case(0b1110_0000_0000_0111, Jump::True)]
    fn test_decode_c_jump(#[case] input: u16, #[case] output: Jump) {
        assert_eq!(output, Instruction::decode_c_jump(input));
    }

    #[rstest]
    #[case("0000000000000000", Instruction::A(0b0000_0000_0000_0000))]
    #[case("0101010000000001", Instruction::A(0b0101_0100_0000_0001))]
    #[case("1110111111101011", Instruction::C(Comp::One, Dest::AAndRamA, Jump::GreaterThanAndEqualTo))]
    fn test_instruction(#[case] input: &str, #[case] output: Instruction) {
        assert_eq!(output, Instruction::new(input));
    }
}