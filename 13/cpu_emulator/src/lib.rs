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

    fn reset_ram(&mut self) {
        self.ram.clear();
        for _ in 0..65535 {
            self.ram.push(0b0000_0000_0000_0000);
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