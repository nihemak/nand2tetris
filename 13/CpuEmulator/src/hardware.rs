#[derive(Clone)]
enum InstructionCComp {
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
enum InstructionCDest {
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
enum InstructionCJump {
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
enum Instruction {
    A(u16),
    C(InstructionCComp, InstructionCDest, InstructionCJump),
}

impl Instruction {
    pub fn new(instruction: &str) -> Self {
        let inst = Self::decode_instruction(instruction);
        if Self::is_a_instruction(inst) {
            Self::A(inst)
        }
        else if Self::is_c_instruction(inst) {
            Self::C(
                Self::decode_instruction_c_comp(inst),
                Self::decode_instruction_c_dest(inst),
                Self::decode_instruction_c_jump(inst),
            )
        }
        else {
            panic!("error: instruction kind {:#018b}", inst);
        }
    }

    fn decode_instruction(instruction: &str) -> u16 {
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

    fn decode_instruction_c_comp(inst: u16) -> InstructionCComp {
        let (comp_a, comp_cccccc) = ((inst & 0b0001000000000000) >> 12, (inst & 0b0000111111000000) >> 6);
        match (comp_a, comp_cccccc) {
            (0, 0b101010) => InstructionCComp::Zero,        /* 0 */
            (0, 0b111111) => InstructionCComp::One,         /* 1 */
            (0, 0b111010) => InstructionCComp::MinusOne,    /* -1 */
            (0, 0b001100) => InstructionCComp::D,           /* D */
            (0, 0b110000) => InstructionCComp::A,           /* A */
            (1, 0b110000) => InstructionCComp::M,           /* M */
            (0, 0b001101) => InstructionCComp::NotD,        /* !D */
            (0, 0b110001) => InstructionCComp::NotA,        /* !A */
            (1, 0b110001) => InstructionCComp::NotM,        /* !M */
            (0, 0b001111) => InstructionCComp::MinusD,      /* -D */
            (0, 0b110011) => InstructionCComp::MinusA,      /* -A */
            (1, 0b110011) => InstructionCComp::MinusM,      /* -M */
            (0, 0b011111) => InstructionCComp::DPlusOne,    /* D+1 */
            (0, 0b110111) => InstructionCComp::APlusOne,    /* A+1 */
            (1, 0b110111) => InstructionCComp::MPlusOne,    /* M+1 */
            (0, 0b001110) => InstructionCComp::DMinusOne,   /* D-1 */
            (0, 0b110010) => InstructionCComp::AMinusOne,   /* A-1 */
            (1, 0b110010) => InstructionCComp::MMinusOne,   /* M-1 */
            (0, 0b000010) => InstructionCComp::DPlusA,      /* D+A */
            (1, 0b000010) => InstructionCComp::DPlusM,      /* D+M */
            (0, 0b010011) => InstructionCComp::DMinusA,     /* D-A */
            (1, 0b010011) => InstructionCComp::DMinusM,     /* D-M */
            (0, 0b000111) => InstructionCComp::AMinusD,     /* A-D */
            (1, 0b000111) => InstructionCComp::MMinusD,     /* M-D */
            (0, 0b000000) => InstructionCComp::DAndA,       /* D&A */
            (1, 0b000000) => InstructionCComp::DAndM,       /* D&M */
            (0, 0b010101) => InstructionCComp::DOrA,        /* D|A */
            (1, 0b010101) => InstructionCComp::AOrM,        /* D|M */
            _ => panic!("error: comp {} {:#018b}", comp_a, comp_cccccc),
        }
    }

    fn decode_instruction_c_dest(inst: u16) -> InstructionCDest {
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

    fn decode_instruction_c_jump(inst: u16) -> InstructionCJump {
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

#[derive(Clone)]
pub struct Computer {
    pc: i16,
    a: i16,
    d: i16,
    ram: Vec<i16>,
    rom: Vec<Instruction>,
    update_screen_addrs: Vec<i16>,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            pc: 0,
            a: 0b0000000000000000,
            d: 0b0000000000000000,
            ram: Vec::new(),
            rom: Vec::new(),
            update_screen_addrs: Vec::new(),
        }
    }

    fn reset_ram(&mut self) {
        self.ram.clear();
        for _ in 0..65535 {
            self.ram.push(0b0000000000000000);
        }
    }

    pub fn load_program(&mut self, instructions: Vec<&str>) {
        self.rom.clear();
        for instruction in instructions {
            self.rom.push(Instruction::new(instruction));
        }
    }

    fn is_screen_addr(addr: i16) -> bool {
        addr >= 16384 /* SCREEN */ && addr <= 24575
    }

    pub fn step(&mut self, reset: bool, key_code: i16) {
        if reset {
            self.reset_ram();
            self.pc = 0
        }

        self.ram[24576 /* KBD */] = key_code;

        let inst = &self.rom[self.pc as usize];
        match inst {
            Instruction::A(a) => {
                self.a = *a as i16;
                self.pc += 1;
            },
            Instruction::C(comp, dest, jump) => {
                let m = self.ram[self.a as usize];
                let comp: i16 = match comp {
                    InstructionCComp::Zero      => 0,                   /* 0 */
                    InstructionCComp::One       => 1,                   /* 1 */
                    InstructionCComp::MinusOne  => -1,                  /* -1 */
                    InstructionCComp::D         => self.d,              /* D */
                    InstructionCComp::A         => self.a,              /* A */
                    InstructionCComp::M         => m,                   /* M */
                    InstructionCComp::NotD      => !self.d,             /* !D */
                    InstructionCComp::NotA      => !self.a,             /* !A */
                    InstructionCComp::NotM      => !m,                  /* !M */
                    InstructionCComp::MinusD    => -self.d,             /* -D */
                    InstructionCComp::MinusA    => -self.a,             /* -A */
                    InstructionCComp::MinusM    => -m,                  /* -M */
                    InstructionCComp::DPlusOne  => self.d + 1,          /* D+1 */
                    InstructionCComp::APlusOne  => self.a + 1,          /* A+1 */
                    InstructionCComp::MPlusOne  => m + 1,               /* M+1 */
                    InstructionCComp::DMinusOne => self.d - 1,          /* D-1 */
                    InstructionCComp::AMinusOne => self.a - 1,          /* A-1 */
                    InstructionCComp::MMinusOne => m - 1,               /* M-1 */
                    InstructionCComp::DPlusA    => self.d + self.a,     /* D+A */
                    InstructionCComp::DPlusM    => self.d + m,          /* D+M */
                    InstructionCComp::DMinusA   => self.d - self.a,     /* D-A */
                    InstructionCComp::DMinusM   => self.d - m,          /* D-M */
                    InstructionCComp::AMinusD   => self.a - self.d,     /* A-D */
                    InstructionCComp::MMinusD   => m - self.d,          /* M-D */
                    InstructionCComp::DAndA     => self.d & self.a,     /* D&A */
                    InstructionCComp::DAndM     => self.d & m,          /* D&M */
                    InstructionCComp::DOrA      => self.d | self.a,     /* D|A */
                    InstructionCComp::AOrM      => self.d | m,          /* D|M */
                };

                match dest {
                    InstructionCDest::Null          => { }, /* null */
                    InstructionCDest::RamA          => {    /* RAM[A] */
                        self.ram[self.a as usize] = comp;
                        if Self::is_screen_addr(self.a) {
                            self.update_screen_addrs.push(self.a);
                        }
                    },
                    InstructionCDest::D             => {    /* D */
                        self.d = comp;
                    },
                    InstructionCDest::DAndRamA      => {    /* D, RAM[A] */
                        self.d = comp;
                        self.ram[self.a as usize] = comp;
                        if Self::is_screen_addr(self.a) {
                            self.update_screen_addrs.push(self.a);
                        }
                    },
                    InstructionCDest::A             => {    /* A */
                        self.a = comp;
                    }, 
                    InstructionCDest::AAndRamA      => {    /* A, RAM[A] */
                        self.a = comp;
                        self.ram[self.a as usize] = comp;
                        if Self::is_screen_addr(self.a) {
                            self.update_screen_addrs.push(self.a);
                        }
                    },
                    InstructionCDest::AAndD         => {    /* A, D */
                        self.a = comp;
                        self.d = comp;
                    },
                    InstructionCDest::AAndDAndRamA  => {    /* A, D, RAM[A] */
                        self.a = comp;
                        self.d = comp;
                        self.ram[self.a as usize] = comp;
                        if Self::is_screen_addr(self.a) {
                            self.update_screen_addrs.push(self.a);
                        }
                    },
                }
                println!("pc {}, a {:#018b}, d {:#018b}, comp {}", self.pc, self.a, self.d, comp);

                let jump = match jump {
                    InstructionCJump::None                  => { false },       /* none */
                    InstructionCJump::GreaterThan           => { comp >  0 },   /* if comp > 0 jump */
                    InstructionCJump::EqualTo               => { comp == 0 },   /* if comp = 0 jump */
                    InstructionCJump::GreaterThanAndEqualTo => { comp >= 0 },   /* if comp >= 0 jump */
                    InstructionCJump::LessThan              => { comp <  0 },   /* if comp < 0 jump */
                    InstructionCJump::NotEqualTo            => { comp != 0 },   /* if comp != 0 jump */
                    InstructionCJump::LessThanAndEqualTo    => { comp <= 0 },   /* if comp <= 0 jump */
                    InstructionCJump::True                  => { true },        /* if true jump */
                };
                self.pc = if jump { self.a } else { self.pc + 1 };
            },
        }
    }

    pub fn get_screen(&self) -> [bool; 131072] {
        let mut screen = [false; 131072];
        let mut x = 0;
        for i in 0..8192 {
            let word = self.ram[16384 /* SCREEN */ + i];
            let mut bit: i16 = 0b0000000000000001;
            for _ in 0..16 {
                if word & bit != 0 {
                    screen[x] = true;
                }
                bit <<= 1;
                x += 1;
            }
        }
        screen
    }

    pub fn get_update_screen_pixels(&mut self) -> Vec<(i32, i32, bool)> {
        let mut pixels = Vec::new();
        for addr in &self.update_screen_addrs {
            let base = (addr - 16384 /* SCREEN */) * 16;
            let word = self.ram[*addr as usize];
            let mut bit: i16 = 0b0000000000000001;
            for i in 0..16 {
                let px: i32 = (base + i).into();
                let x: i32 = px % 512;
                let y: i32 = px / 512;
                let color = word & bit != 0;
                pixels.push((x, y, color));
                bit <<= 1;
            }
        }
        self.update_screen_addrs.clear();
        pixels
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
    fn test_decode_instruction(#[case] input: &str, #[case] output: u16) {
        assert_eq!(output, Instruction::decode_instruction(input));
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
}