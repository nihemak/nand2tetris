mod instruction;
mod word;

use instruction::{
    Instruction,
    Comp as InstructionCComp,
    Dest as InstructionCDest,
    Jump as InstructionCJump,
};

use word::Word;

#[derive(Clone)]
pub struct Computer {
    pc: Word,
    a: Word,
    d: Word,
    ram: Vec<Word>,
    rom: Vec<Instruction>,
    update_screen_addrs: Vec<u16>,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            pc: Word::new(),
            a: Word::new(),
            d: Word::new(),
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

    pub fn step(&mut self, reset: bool, key_code: u16) {
        if reset {
            self.reset_ram();
            self.pc = Word::new();
        }

        self.ram[24576 /* KBD */] = Word::from(key_code);

        let inst = &self.rom[self.pc.to_u16() as usize];
        match inst {
            Instruction::A(a) => {
                self.a = Word::from(*a);
                self.pc = self.pc.add(&Word::from(1));
            },
            Instruction::C(comp, dest, jump) => {
                let m = self.ram[self.a.to_u16() as usize];
                let comp: Word = Self::comp(comp, &self.a, &self.d, &m);

                match dest {
                    InstructionCDest::Null          => { }, /* null */
                    InstructionCDest::RamA          => {    /* RAM[A] */
                        self.ram[self.a.to_u16() as usize] = comp;
                        if Self::is_screen_addr(&self.a) {
                            self.update_screen_addrs.push(self.a.to_u16());
                        }
                    },
                    InstructionCDest::D             => {    /* D */
                        self.d = comp;
                    },
                    InstructionCDest::DAndRamA      => {    /* D, RAM[A] */
                        self.d = comp;
                        self.ram[self.a.to_u16() as usize] = comp;
                        if Self::is_screen_addr(&self.a) {
                            self.update_screen_addrs.push(self.a.to_u16());
                        }
                    },
                    InstructionCDest::A             => {    /* A */
                        self.a = comp;
                    }, 
                    InstructionCDest::AAndRamA      => {    /* A, RAM[A] */
                        self.a = comp;
                        self.ram[self.a.to_u16() as usize] = comp;
                        if Self::is_screen_addr(&self.a) {
                            self.update_screen_addrs.push(self.a.to_u16());
                        }
                    },
                    InstructionCDest::AAndD         => {    /* A, D */
                        self.a = comp;
                        self.d = comp;
                    },
                    InstructionCDest::AAndDAndRamA  => {    /* A, D, RAM[A] */
                        self.a = comp;
                        self.d = comp;
                        self.ram[self.a.to_u16() as usize] = comp;
                        if Self::is_screen_addr(&self.a) {
                            self.update_screen_addrs.push(self.a.to_u16());
                        }
                    },
                }
                println!("pc {}, a {:#018b}, d {:#018b}, comp {}", self.pc, self.a, self.d, comp);

                self.pc = if Self::jump(jump, &comp) { self.a.clone() } else { self.pc.add(&Word::from(1)) };
            },
        }
    }

    fn reset_ram(&mut self) {
        self.ram.clear();
        for _ in 0..65535 {
            self.ram.push(Word::new());
        }
    }

    fn is_screen_addr(addr: &Word) -> bool {
        let addr = addr.to_u16();
        addr >= 16384 /* SCREEN */ && addr <= 24575
    }

    fn comp(comp: &InstructionCComp, a: &Word, d: &Word, m: &Word) -> Word {
        match comp {
            InstructionCComp::Zero      => Word::new(),                     /* 0 */
            InstructionCComp::One       => Word::from(1),                   /* 1 */
            InstructionCComp::MinusOne  => Word::from(1).minus(),           /* -1 */
            InstructionCComp::D         => d.clone(),                       /* D */
            InstructionCComp::A         => a.clone(),                       /* A */
            InstructionCComp::M         => m.clone(),                       /* M */
            InstructionCComp::NotD      => d.not(),                         /* !D */
            InstructionCComp::NotA      => a.not(),                         /* !A */
            InstructionCComp::NotM      => m.not(),                         /* !M */
            InstructionCComp::MinusD    => d.minus(),                       /* -D */
            InstructionCComp::MinusA    => a.minus(),                       /* -A */
            InstructionCComp::MinusM    => m.minus(),                       /* -M */
            InstructionCComp::DPlusOne  => d.add(&Word::from(1)),           /* D+1 */
            InstructionCComp::APlusOne  => a.add(&Word::from(1)),           /* A+1 */
            InstructionCComp::MPlusOne  => m.add(&Word::from(1)),           /* M+1 */
            InstructionCComp::DMinusOne => d.add(&Word::from(1).minus()),   /* D-1 */
            InstructionCComp::AMinusOne => a.add(&Word::from(1).minus()),   /* A-1 */
            InstructionCComp::MMinusOne => m.add(&Word::from(1).minus()),   /* M-1 */
            InstructionCComp::DPlusA    => d.add(&a),                       /* D+A */
            InstructionCComp::DPlusM    => d.add(&m),                       /* D+M */
            InstructionCComp::DMinusA   => d.add(&a.minus()),               /* D-A */
            InstructionCComp::DMinusM   => d.add(&m.minus()),               /* D-M */
            InstructionCComp::AMinusD   => a.add(&d.minus()),               /* A-D */
            InstructionCComp::MMinusD   => m.add(&d.minus()),               /* M-D */
            InstructionCComp::DAndA     => d.and(&a),                       /* D&A */
            InstructionCComp::DAndM     => d.and(&m),                       /* D&M */
            InstructionCComp::DOrA      => d.or(&a),                        /* D|A */
            InstructionCComp::DOrM      => d.or(&m),                        /* D|M */
        }
    }

    fn jump(jump: &InstructionCJump, comp: &Word) -> bool {
        match jump {
            InstructionCJump::None                  => { false },               /* none */
            InstructionCJump::GreaterThan           => { comp.to_i16() >  0 },  /* if comp > 0 jump */
            InstructionCJump::EqualTo               => { comp.to_i16() == 0 },  /* if comp = 0 jump */
            InstructionCJump::GreaterThanAndEqualTo => { comp.to_i16() >= 0 },  /* if comp >= 0 jump */
            InstructionCJump::LessThan              => { comp.to_i16() <  0 },  /* if comp < 0 jump */
            InstructionCJump::NotEqualTo            => { comp.to_i16() != 0 },  /* if comp != 0 jump */
            InstructionCJump::LessThanAndEqualTo    => { comp.to_i16() <= 0 },  /* if comp <= 0 jump */
            InstructionCJump::True                  => { true },                /* if true jump */
        }
    }

    pub fn get_screen(&self) -> [bool; 131072] {
        let mut screen = [false; 131072];
        let mut x = 0;
        for i in 0..8192 {
            let word = self.ram[16384 /* SCREEN */ + i].to_u16();
            let mut bit: u16 = 0b0000_0000_0000_0001;
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
            let word = self.ram[*addr as usize].to_u16();
            let mut bit: u16 = 0b0000_0000_0000_0001;
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
    fn test_is_screen_addr(#[case] input: u16, #[case] output: bool) {
        assert_eq!(output, Computer::is_screen_addr(&Word::from(input)));
    }

    #[rstest]
    #[case((InstructionCComp::Zero, 10, 20, 30), 0)]
    #[case((InstructionCComp::One, 10, 20, 30), 1)]
    #[case((InstructionCComp::MinusOne, 10, 20, 30), -1)]
    #[case((InstructionCComp::D, 10, 20, 30), 20)]
    #[case((InstructionCComp::A, 10, 20, 30), 10)]
    #[case((InstructionCComp::M, 10, 20, 30), 30)]
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
    fn test_comp(#[case] input: (InstructionCComp, u16, u16, u16), #[case] output: i16) {
        let (comp, a, d, m) = input;
        assert_eq!(output, Computer::comp(&comp, &Word::from(a), &Word::from(d), &Word::from(m)).to_i16());
    }

    #[rstest]
    #[case((InstructionCComp::NotD, 0b0001, 0b0010, 0b0100), 0b1111_1111_1111_1101)]
    #[case((InstructionCComp::NotA, 0b0001, 0b0010, 0b0100), 0b1111_1111_1111_1110)]
    #[case((InstructionCComp::NotM, 0b0001, 0b0010, 0b0100), 0b1111_1111_1111_1011)]
    fn test_comp_not(#[case] input: (InstructionCComp, u16, u16, u16), #[case] output: u16) {
        let (comp, a, d, m) = input;
        assert_eq!(output, Computer::comp(&comp, &Word::from(a), &Word::from(d), &Word::from(m)).to_u16());
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

        let comp = Word::from(1).minus();   // -1
        assert_eq!(minus_one, Computer::jump(&input, &comp));

        let comp = Word::new();  // 1
        assert_eq!(zero, Computer::jump(&input, &comp));

        let comp = Word::from(1);   // 1
        assert_eq!(plus_one, Computer::jump(&input, &comp));
    }
}