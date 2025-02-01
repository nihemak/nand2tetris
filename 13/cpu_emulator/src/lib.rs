mod instruction;

use instruction::{
    Instruction,
    CComp as InstructionCComp,
    CDest as InstructionCDest,
    CJump as InstructionCJump,
};

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
            a: 0b0000_0000_0000_0000,
            d: 0b0000_0000_0000_0000,
            ram: Vec::new(),
            rom: Vec::new(),
            update_screen_addrs: Vec::new(),
        }
    }

    pub fn load_program(&mut self, instructions: Vec<&str>) {
        self.rom.clear();
        for instruction in instructions {
            self.rom.push(Instruction::new(instruction));
        }
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
                let comp: i16 = Self::comp(comp, self.a, self.d, m);

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

                self.pc = if Self::jump(jump, comp) { self.a } else { self.pc + 1 };
            },
        }
    }

    fn reset_ram(&mut self) {
        self.ram.clear();
        for _ in 0..65535 {
            self.ram.push(0b0000_0000_0000_0000);
        }
    }

    fn is_screen_addr(addr: i16) -> bool {
        addr >= 16384 /* SCREEN */ && addr <= 24575
    }

    fn comp(comp: &InstructionCComp, a: i16, d: i16, m: i16) -> i16 {
        match comp {
            InstructionCComp::Zero      => 0,       /* 0 */
            InstructionCComp::One       => 1,       /* 1 */
            InstructionCComp::MinusOne  => -1,      /* -1 */
            InstructionCComp::D         => d,       /* D */
            InstructionCComp::A         => a,       /* A */
            InstructionCComp::M         => m,       /* M */
            InstructionCComp::NotD      => !d,      /* !D */
            InstructionCComp::NotA      => !a,      /* !A */
            InstructionCComp::NotM      => !m,      /* !M */
            InstructionCComp::MinusD    => -d,      /* -D */
            InstructionCComp::MinusA    => -a,      /* -A */
            InstructionCComp::MinusM    => -m,      /* -M */
            InstructionCComp::DPlusOne  => d + 1,   /* D+1 */
            InstructionCComp::APlusOne  => a + 1,   /* A+1 */
            InstructionCComp::MPlusOne  => m + 1,   /* M+1 */
            InstructionCComp::DMinusOne => d - 1,   /* D-1 */
            InstructionCComp::AMinusOne => a - 1,   /* A-1 */
            InstructionCComp::MMinusOne => m - 1,   /* M-1 */
            InstructionCComp::DPlusA    => d + a,   /* D+A */
            InstructionCComp::DPlusM    => d + m,   /* D+M */
            InstructionCComp::DMinusA   => d - a,   /* D-A */
            InstructionCComp::DMinusM   => d - m,   /* D-M */
            InstructionCComp::AMinusD   => a - d,   /* A-D */
            InstructionCComp::MMinusD   => m - d,   /* M-D */
            InstructionCComp::DAndA     => d & a,   /* D&A */
            InstructionCComp::DAndM     => d & m,   /* D&M */
            InstructionCComp::DOrA      => d | a,   /* D|A */
            InstructionCComp::DOrM      => d | m,   /* D|M */
        }
    }

    fn jump(jump: &InstructionCJump, comp: i16) -> bool {
        match jump {
            InstructionCJump::None                  => { false },       /* none */
            InstructionCJump::GreaterThan           => { comp >  0 },   /* if comp > 0 jump */
            InstructionCJump::EqualTo               => { comp == 0 },   /* if comp = 0 jump */
            InstructionCJump::GreaterThanAndEqualTo => { comp >= 0 },   /* if comp >= 0 jump */
            InstructionCJump::LessThan              => { comp <  0 },   /* if comp < 0 jump */
            InstructionCJump::NotEqualTo            => { comp != 0 },   /* if comp != 0 jump */
            InstructionCJump::LessThanAndEqualTo    => { comp <= 0 },   /* if comp <= 0 jump */
            InstructionCJump::True                  => { true },        /* if true jump */
        }
    }

    pub fn get_screen(&self) -> [bool; 131072] {
        let mut screen = [false; 131072];
        let mut x = 0;
        for i in 0..8192 {
            let word = self.ram[16384 /* SCREEN */ + i];
            let mut bit: i16 = 0b0000_0000_0000_0001;
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
            let mut bit: i16 = 0b0000_0000_0000_0001;
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
    #[case(16383, false)]
    #[case(16384, true)]
    #[case(24575, true)]
    #[case(24576, false)]
    fn test_is_screen_addr(#[case] input: i16, #[case] output: bool) {
        assert_eq!(output, Computer::is_screen_addr(input));
    }

    #[rstest]
    #[case((InstructionCComp::Zero, 10, 20, 30), 0)]
    #[case((InstructionCComp::One, 10, 20, 30), 1)]
    #[case((InstructionCComp::MinusOne, 10, 20, 30), -1)]
    #[case((InstructionCComp::D, 10, 20, 30), 20)]
    #[case((InstructionCComp::A, 10, 20, 30), 10)]
    #[case((InstructionCComp::M, 10, 20, 30), 30)]
    // #[case((InstructionCComp::NotD, 0b0001, 0b0010, 0b0100), 0b1111_1111_1111_1101)]
    // #[case((InstructionCComp::NotA, 0b0001, 0b0010, 0b0100), 0b1111_1111_1111_1110)]
    // #[case((InstructionCComp::NotM, 0b0001, 0b0010, 0b0100), 0b1111_1111_1111_1011)]
    #[case((InstructionCComp::MinusD, 10, 20, 30), -20)]
    #[case((InstructionCComp::MinusA, 10, 20, 30), -10)]
    #[case((InstructionCComp::MinusM, 10, 20, 30), -30)]
    #[case((InstructionCComp::DPlusOne, 10, 20, 30), 21)]
    #[case((InstructionCComp::APlusOne, 10, 20, 30), 11)]
    #[case((InstructionCComp::MPlusOne, 10, 20, 30), 31)]
    #[case((InstructionCComp::DMinusOne, 10, 20, 30), 19)]
    #[case((InstructionCComp::AMinusOne, 10, 20, 30), 9)]
    #[case((InstructionCComp::MMinusOne, 10, 20, 30), 29)]
    #[case((InstructionCComp::DPlusA, 10, 20, 50), 30)]
    #[case((InstructionCComp::DPlusM, 10, 20, 50), 70)]
    #[case((InstructionCComp::DMinusA, 10, 20, 50), 10)]
    #[case((InstructionCComp::DMinusM, 10, 20, 50), -30)]
    #[case((InstructionCComp::AMinusD, 10, 20, 50), -10)]
    #[case((InstructionCComp::MMinusD, 10, 20, 50), 30)]
    #[case((InstructionCComp::DAndA, 0b0001, 0b0011, 0b0010), 0b0001)]
    #[case((InstructionCComp::DAndM, 0b0001, 0b0011, 0b0010), 0b0010)]
    #[case((InstructionCComp::DOrA, 0b0000, 0b0011, 0b1100), 0b0011)]
    #[case((InstructionCComp::DOrM, 0b0000, 0b0011, 0b1100), 0b1111)]
    fn test_comp(#[case] input: (InstructionCComp, i16, i16, i16), #[case] output: i16) {
        let (comp, a, d, m) = input;
        assert_eq!(output, Computer::comp(&comp, a, d, m));
    }

    #[rstest]
    #[case(InstructionCJump::None,                  (false, false, false))]
    #[case(InstructionCJump::GreaterThan,           (false, false, true))]
    #[case(InstructionCJump::EqualTo,               (false, true,  false))]
    #[case(InstructionCJump::GreaterThanAndEqualTo, (false, true,  true))]
    #[case(InstructionCJump::LessThan,              (true,  false, false))]
    #[case(InstructionCJump::NotEqualTo,            (true,  false, true))]
    #[case(InstructionCJump::LessThanAndEqualTo,    (true,  true,  false))]
    #[case(InstructionCJump::True,                  (true,  true,  true))]
    fn test_jump(#[case] input: InstructionCJump, #[case] output: (bool, bool, bool)) {
        let (minus_one, zero, plus_one) = output;

        let comp = -1;
        assert_eq!(minus_one, Computer::jump(&input, comp));

        let comp = 0;
        assert_eq!(zero, Computer::jump(&input, comp));

        let comp = 1;
        assert_eq!(plus_one, Computer::jump(&input, comp));
    }
}