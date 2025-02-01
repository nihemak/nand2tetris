#[derive(Clone, PartialEq, Debug)]
pub enum InstructionCComp {
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

#[derive(Clone)]
pub enum InstructionCDest {
    Null,           /* null */
    RamA,           /* RAM[A] */
    D,              /* D */
    DAndRamA,       /* D, RAM[A] */
    A,              /* A */
    AAndRamA,       /* A, RAM[A] */
    AAndD,          /* A, D */
    AAndDAndRamA,   /* A, D, RAM[A] */
}

#[derive(Clone)]
pub enum InstructionCJump {
    None,                   /* none */
    GreaterThan,            /* if comp > 0 jump */
    EqualTo,                /* if comp = 0 jump */
    GreaterThanAndEqualTo,  /* if comp >= 0 jump */
    LessThan,               /* if comp < 0 jump */
    NotEqualTo,             /* if comp != 0 jump */
    LessThanAndEqualTo,     /* if comp <= 0 jump */
    True,                   /* if true jump */
}

#[derive(Clone)]
pub enum Instruction {
    A(u16),
    C(InstructionCComp, InstructionCDest, InstructionCJump),
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
        let mut inst: u16 = 0b0000000000000000;
        let mut bit: u16 = 0b1000000000000000;
        for (_, c) in instruction.chars().enumerate() {
            if c == '1' { inst |= bit; }
            bit >>= 1;
        }
        inst
    }

    fn is_a_instruction(inst: u16) -> bool {
        inst & 0b1000000000000000 == 0
    }

    fn is_c_instruction(inst: u16) -> bool {
        inst & 0b1110000000000000 == 0b1110000000000000
    }

    fn decode_c_comp(inst: u16) -> InstructionCComp {
        let comp = (inst & 0b0001111111000000) >> 6;
        match comp {
            0b0101010 => InstructionCComp::Zero,        /* 0 */
            0b0111111 => InstructionCComp::One,         /* 1 */
            0b0111010 => InstructionCComp::MinusOne,    /* -1 */
            0b0001100 => InstructionCComp::D,           /* D */
            0b0110000 => InstructionCComp::A,           /* A */
            0b1110000 => InstructionCComp::M,           /* M */
            0b0001101 => InstructionCComp::NotD,        /* !D */
            0b0110001 => InstructionCComp::NotA,        /* !A */
            0b1110001 => InstructionCComp::NotM,        /* !M */
            0b0001111 => InstructionCComp::MinusD,      /* -D */
            0b0110011 => InstructionCComp::MinusA,      /* -A */
            0b1110011 => InstructionCComp::MinusM,      /* -M */
            0b0011111 => InstructionCComp::DPlusOne,    /* D+1 */
            0b0110111 => InstructionCComp::APlusOne,    /* A+1 */
            0b1110111 => InstructionCComp::MPlusOne,    /* M+1 */
            0b0001110 => InstructionCComp::DMinusOne,   /* D-1 */
            0b0110010 => InstructionCComp::AMinusOne,   /* A-1 */
            0b1110010 => InstructionCComp::MMinusOne,   /* M-1 */
            0b0000010 => InstructionCComp::DPlusA,      /* D+A */
            0b1000010 => InstructionCComp::DPlusM,      /* D+M */
            0b0010011 => InstructionCComp::DMinusA,     /* D-A */
            0b1010011 => InstructionCComp::DMinusM,     /* D-M */
            0b0000111 => InstructionCComp::AMinusD,     /* A-D */
            0b1000111 => InstructionCComp::MMinusD,     /* M-D */
            0b0000000 => InstructionCComp::DAndA,       /* D&A */
            0b1000000 => InstructionCComp::DAndM,       /* D&M */
            0b0010101 => InstructionCComp::DOrA,        /* D|A */
            0b1010101 => InstructionCComp::AOrM,        /* D|M */
            _ => panic!("error: comp {:#018b}", comp),
        }
    }

    fn decode_c_dest(inst: u16) -> InstructionCDest {
        let dest = (inst & 0b0000000000111000) >> 3;
        match dest {
            0b000 => InstructionCDest::Null,            /* null */
            0b001 => InstructionCDest::RamA,            /* RAM[A] */
            0b010 => InstructionCDest::D,               /* D */
            0b011 => InstructionCDest::DAndRamA,        /* D, RAM[A] */
            0b100 => InstructionCDest::A,               /* A */
            0b101 => InstructionCDest::AAndRamA,        /* A, RAM[A] */
            0b110 => InstructionCDest::AAndD,           /* A, D */
            0b111 => InstructionCDest::AAndDAndRamA,    /* A, D, RAM[A] */
            _ => panic!("error: dest {:#018b}", dest),
        }
    }

    fn decode_c_jump(inst: u16) -> InstructionCJump {
        let jump = inst & 0b0000000000000111;
        match jump {
            0b000 => InstructionCJump::None,                    /* none */
            0b001 => InstructionCJump::GreaterThan,             /* if comp > 0 jump */
            0b010 => InstructionCJump::EqualTo,                 /* if comp = 0 jump */
            0b011 => InstructionCJump::GreaterThanAndEqualTo,   /* if comp >= 0 jump */
            0b100 => InstructionCJump::LessThan,                /* if comp < 0 jump */
            0b101 => InstructionCJump::NotEqualTo,              /* if comp != 0 jump */
            0b110 => InstructionCJump::LessThanAndEqualTo,      /* if comp <= 0 jump */
            0b111 => InstructionCJump::True,                    /* if true jump */
            _ => panic!("error: jump {:#018b}", jump),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case("0000000000000000", 0b0000000000000000)]
    #[case("1111111111111111", 0b1111111111111111)]
    #[case("0110000000000000", 0b0110000000000000)]
    #[case("1111110000010000", 0b1111110000010000)]
    #[case("0000000000001000", 0b0000000000001000)]
    fn test_decode_to_binary(#[case] input: &str, #[case] output: u16) {
        assert_eq!(output, Instruction::decode_to_binary(input));
    }

    #[rstest]
    #[case(0b0000000000000000, true)]
    #[case(0b1110000000000000, false)]
    fn test_is_a_instruction(#[case] input: u16, #[case] output: bool) {
        assert_eq!(output, Instruction::is_a_instruction(input));
    }

    #[rstest]
    #[case(0b0000000000000000, false)]
    #[case(0b1110000000000000, true)]
    fn test_is_c_instruction(#[case] input: u16, #[case] output: bool) {
        assert_eq!(output, Instruction::is_c_instruction(input));
    }

    #[rstest]
    #[case(0b1110101010000000, InstructionCComp::Zero)]
    #[case(0b1110111111000000, InstructionCComp::One)]
    #[case(0b1110111010000000, InstructionCComp::MinusOne)]
    #[case(0b1110001100000000, InstructionCComp::D)]
    #[case(0b1110110000000000, InstructionCComp::A)]
    #[case(0b1111110000000000, InstructionCComp::M)]
    #[case(0b1110001101000000, InstructionCComp::NotD)]
    #[case(0b1110110001000000, InstructionCComp::NotA)]
    #[case(0b1111110001000000, InstructionCComp::NotM)]
    #[case(0b1110001111000000, InstructionCComp::MinusD)]
    #[case(0b1110110011000000, InstructionCComp::MinusA)]
    #[case(0b1111110011000000, InstructionCComp::MinusM)]
    #[case(0b1110011111000000, InstructionCComp::DPlusOne)]
    #[case(0b1110110111000000, InstructionCComp::APlusOne)]
    #[case(0b1111110111000000, InstructionCComp::MPlusOne)]
    #[case(0b1110001110000000, InstructionCComp::DMinusOne)]
    #[case(0b1110110010000000, InstructionCComp::AMinusOne)]
    #[case(0b1111110010000000, InstructionCComp::MMinusOne)]
    #[case(0b1110000010000000, InstructionCComp::DPlusA)]
    #[case(0b1111000010000000, InstructionCComp::DPlusM)]
    #[case(0b1110010011000000, InstructionCComp::DMinusA)]
    #[case(0b1111010011000000, InstructionCComp::DMinusM)]
    #[case(0b1110000111000000, InstructionCComp::AMinusD)]
    #[case(0b1111000111000000, InstructionCComp::MMinusD)]
    #[case(0b1110000000000000, InstructionCComp::DAndA)]
    #[case(0b1111000000000000, InstructionCComp::DAndM)]
    #[case(0b1110010101000000, InstructionCComp::DOrA)]
    #[case(0b1111010101000000, InstructionCComp::AOrM)]
    fn test_decode_c_comp(#[case] input: u16, #[case] output: InstructionCComp) {
        assert_eq!(output, Instruction::decode_c_comp(input));
    }
}