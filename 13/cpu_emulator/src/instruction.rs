#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    A(u16),
    C(CComp, CDest, CJump),
}

#[derive(Clone, PartialEq, Debug)]
pub enum CComp {
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
    AOrM,       /* D|M */
}

#[derive(Clone, PartialEq, Debug)]
pub enum CDest {
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
pub enum CJump {
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

    fn decode_c_comp(inst: u16) -> CComp {
        let comp = (inst & 0b0001_1111_1100_0000) >> 6;
        match comp {
            0b0_101010 => CComp::Zero,        /* 0 */
            0b0_111111 => CComp::One,         /* 1 */
            0b0_111010 => CComp::MinusOne,    /* -1 */
            0b0_001100 => CComp::D,           /* D */
            0b0_110000 => CComp::A,           /* A */
            0b1_110000 => CComp::M,           /* M */
            0b0_001101 => CComp::NotD,        /* !D */
            0b0_110001 => CComp::NotA,        /* !A */
            0b1_110001 => CComp::NotM,        /* !M */
            0b0_001111 => CComp::MinusD,      /* -D */
            0b0_110011 => CComp::MinusA,      /* -A */
            0b1_110011 => CComp::MinusM,      /* -M */
            0b0_011111 => CComp::DPlusOne,    /* D+1 */
            0b0_110111 => CComp::APlusOne,    /* A+1 */
            0b1_110111 => CComp::MPlusOne,    /* M+1 */
            0b0_001110 => CComp::DMinusOne,   /* D-1 */
            0b0_110010 => CComp::AMinusOne,   /* A-1 */
            0b1_110010 => CComp::MMinusOne,   /* M-1 */
            0b0_000010 => CComp::DPlusA,      /* D+A */
            0b1_000010 => CComp::DPlusM,      /* D+M */
            0b0_010011 => CComp::DMinusA,     /* D-A */
            0b1_010011 => CComp::DMinusM,     /* D-M */
            0b0_000111 => CComp::AMinusD,     /* A-D */
            0b1_000111 => CComp::MMinusD,     /* M-D */
            0b0_000000 => CComp::DAndA,       /* D&A */
            0b1_000000 => CComp::DAndM,       /* D&M */
            0b0_010101 => CComp::DOrA,        /* D|A */
            0b1_010101 => CComp::AOrM,        /* D|M */
            _ => panic!("error: comp {:#018b}", comp),
        }
    }

    fn decode_c_dest(inst: u16) -> CDest {
        let dest = (inst & 0b0000_0000_0011_1000) >> 3;
        match dest {
            0b000 => CDest::Null,            /* null */
            0b001 => CDest::RamA,            /* RAM[A] */
            0b010 => CDest::D,               /* D */
            0b011 => CDest::DAndRamA,        /* D, RAM[A] */
            0b100 => CDest::A,               /* A */
            0b101 => CDest::AAndRamA,        /* A, RAM[A] */
            0b110 => CDest::AAndD,           /* A, D */
            0b111 => CDest::AAndDAndRamA,    /* A, D, RAM[A] */
            _ => panic!("error: dest {:#018b}", dest),
        }
    }

    fn decode_c_jump(inst: u16) -> CJump {
        let jump = inst & 0b0000_0000_0000_0111;
        match jump {
            0b000 => CJump::None,                    /* none */
            0b001 => CJump::GreaterThan,             /* if comp > 0 jump */
            0b010 => CJump::EqualTo,                 /* if comp = 0 jump */
            0b011 => CJump::GreaterThanAndEqualTo,   /* if comp >= 0 jump */
            0b100 => CJump::LessThan,                /* if comp < 0 jump */
            0b101 => CJump::NotEqualTo,              /* if comp != 0 jump */
            0b110 => CJump::LessThanAndEqualTo,      /* if comp <= 0 jump */
            0b111 => CJump::True,                    /* if true jump */
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
    #[case(0b1110_1010_1000_0000, CComp::Zero)]
    #[case(0b1110_1111_1100_0000, CComp::One)]
    #[case(0b1110_1110_1000_0000, CComp::MinusOne)]
    #[case(0b1110_0011_0000_0000, CComp::D)]
    #[case(0b1110_1100_0000_0000, CComp::A)]
    #[case(0b1111_1100_0000_0000, CComp::M)]
    #[case(0b1110_0011_0100_0000, CComp::NotD)]
    #[case(0b1110_1100_0100_0000, CComp::NotA)]
    #[case(0b1111_1100_0100_0000, CComp::NotM)]
    #[case(0b1110_0011_1100_0000, CComp::MinusD)]
    #[case(0b1110_1100_1100_0000, CComp::MinusA)]
    #[case(0b1111_1100_1100_0000, CComp::MinusM)]
    #[case(0b1110_0111_1100_0000, CComp::DPlusOne)]
    #[case(0b1110_1101_1100_0000, CComp::APlusOne)]
    #[case(0b1111_1101_1100_0000, CComp::MPlusOne)]
    #[case(0b1110_0011_1000_0000, CComp::DMinusOne)]
    #[case(0b1110_1100_1000_0000, CComp::AMinusOne)]
    #[case(0b1111_1100_1000_0000, CComp::MMinusOne)]
    #[case(0b1110_0000_1000_0000, CComp::DPlusA)]
    #[case(0b1111_0000_1000_0000, CComp::DPlusM)]
    #[case(0b1110_0100_1100_0000, CComp::DMinusA)]
    #[case(0b1111_0100_1100_0000, CComp::DMinusM)]
    #[case(0b1110_0001_1100_0000, CComp::AMinusD)]
    #[case(0b1111_0001_1100_0000, CComp::MMinusD)]
    #[case(0b1110_0000_0000_0000, CComp::DAndA)]
    #[case(0b1111_0000_0000_0000, CComp::DAndM)]
    #[case(0b1110_0101_0100_0000, CComp::DOrA)]
    #[case(0b1111_0101_0100_0000, CComp::AOrM)]
    fn test_decode_c_comp(#[case] input: u16, #[case] output: CComp) {
        assert_eq!(output, Instruction::decode_c_comp(input));
    }

    #[rstest]
    #[case(0b1110_0000_0000_0000, CDest::Null)]
    #[case(0b1110_0000_0000_1000, CDest::RamA)]
    #[case(0b1110_0000_0001_0000, CDest::D)]
    #[case(0b1110_0000_0001_1000, CDest::DAndRamA)]
    #[case(0b1110_0000_0010_0000, CDest::A)]
    #[case(0b1110_0000_0010_1000, CDest::AAndRamA)]
    #[case(0b1110_0000_0011_0000, CDest::AAndD)]
    #[case(0b1110_0000_0011_1000, CDest::AAndDAndRamA)]
    fn test_decode_c_dest(#[case] input: u16, #[case] output: CDest) {
        assert_eq!(output, Instruction::decode_c_dest(input));
    }

    #[rstest]
    #[case(0b1110_0000_0000_0000, CJump::None)]
    #[case(0b1110_0000_0000_0001, CJump::GreaterThan)]
    #[case(0b1110_0000_0000_0010, CJump::EqualTo)]
    #[case(0b1110_0000_0000_0011, CJump::GreaterThanAndEqualTo)]
    #[case(0b1110_0000_0000_0100, CJump::LessThan)]
    #[case(0b1110_0000_0000_0101, CJump::NotEqualTo)]
    #[case(0b1110_0000_0000_0110, CJump::LessThanAndEqualTo)]
    #[case(0b1110_0000_0000_0111, CJump::True)]
    fn test_decode_c_jump(#[case] input: u16, #[case] output: CJump) {
        assert_eq!(output, Instruction::decode_c_jump(input));
    }

    #[rstest]
    #[case("0000000000000000", Instruction::A(0b0000_0000_0000_0000))]
    #[case("0101010000000001", Instruction::A(0b0101_0100_0000_0001))]
    #[case("1110111111101011", Instruction::C(CComp::One, CDest::AAndRamA, CJump::GreaterThanAndEqualTo))]
    fn test_instruction(#[case] input: &str, #[case] output: Instruction) {
        assert_eq!(output, Instruction::new(input));
    }
}