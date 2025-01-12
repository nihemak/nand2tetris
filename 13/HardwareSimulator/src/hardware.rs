use crate::boolean_logic::*;
use crate::helper::*;
use crate::boolean_arithmetic::*;
use crate::sequential_circuit::*;

#[derive(Copy, Clone)]
pub struct Screen {
    rams: [RAM4K; 2],
    screen: [bit; 131072],
}

impl Screen {
    pub fn new() -> Self {
        Screen { 
            rams: [RAM4K::new(); 2],
            screen: [false; 131072],
        }
    }

    fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 13]) {
        let (a, b) = dmux(load, address[12]);
        let address_low = bit13_to_bit12(address);
        self.rams[0].update(clk, input, a, address_low);
        self.rams[1].update(clk, input, b, address_low);

        if load {
            let address_num = bit13_to_u16(address);
            if address_num <= 24575 {
                let screen_address: u32 = 16 * (address_num as u32);
                for n in 0..16 {
                    self.screen[(screen_address + n) as usize] = input[n as usize];
                }
            }
        }
    }

    fn get(&self, clk: bit, address: [bit; 13]) -> word {
        let address_low = bit13_to_bit12(address);
        mux16(
            self.rams[0].get(clk, address_low),
            self.rams[1].get(clk, address_low),
            address[12]
        )
    }

    pub fn get_all(&self) -> [bit; 131072] {
        // let mut screen = [false; 131072];
        // let mut x = 0;
        // for i in 0..8192 {
        //     let address = u16_to_13bit(i);
        //     let word = self.get(false, address);
        //     for j in 0..16 {
        //         screen[x] = word[j];
        //         x += 1;
        //     }
        // }
        // screen
        self.screen
    }
}

#[derive(Copy, Clone)]
pub struct ScreenBuiltIn {
    screen: [bit; 131072],
}

impl ScreenBuiltIn {
    pub fn new() -> Self {
        ScreenBuiltIn { 
            screen: [false; 131072],
        }
    }

    fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 13]) {
        if load {
            let address_num = bit13_to_u16(address);
            if address_num <= 24575 {
                let screen_address: u32 = 16 * (address_num as u32);
                for n in 0..16 {
                    self.screen[(screen_address + n) as usize] = input[n as usize];
                }
            }
        }
    }

    fn get(&self, clk: bit, address: [bit; 13]) -> word {
        let address_num = bit13_to_u16(address);
        let screen_address: u32 = 16 * (address_num as u32);
        let mut word = u16_to_word(0b0000000000000000);
        for n in 0..16 {
            word[n as usize] = self.screen[(screen_address + n) as usize];
        }
        word
    }

    pub fn get_all(&self) -> [bit; 131072] {
        self.screen
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Keyboard {
    key_code: Register,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { key_code: Register::new() }
    }

    fn update(&mut self, clk: bit, key_code: word) {
        self.key_code.update(clk, key_code, true);
    }

    fn get(&self, clk: bit) -> word {
        self.key_code.get(clk)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KeyboardBuiltIn {
    key_code: word,
}

impl KeyboardBuiltIn {
    pub fn new() -> Self {
        KeyboardBuiltIn { key_code: u16_to_word(0b0000000000000000) }
    }

    fn update(&mut self, clk: bit, key_code: word) {
        self.key_code = key_code;
    }

    fn get(&self, clk: bit) -> word {
        self.key_code
    }
}

#[derive(Copy, Clone)]
pub struct Memory {
    ram: RAM16K,
    screen: Screen,
    keyboard: Keyboard,
}

impl Memory {
    pub fn new() -> Self {
        Memory { 
            ram: RAM16K::new(), 
            screen: Screen::new(),
            keyboard: Keyboard::new()
        }
    }

    fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 15], key_code: word) {
        let (ram_load, screen_load) = dmux(load, address[14]);
        self.ram.update(clk, input, ram_load, bit15_to_bit14(address));
        self.screen.update(clk, input, screen_load, bit15_to_bit13(address));
        self.keyboard.update(clk, key_code);
    }

    fn get(&self, clk: bit, address: [bit; 15]) -> word {
        let ram_output = self.ram.get(clk, bit15_to_bit14(address));
        let screen_output = self.screen.get(clk, bit15_to_bit13(address));
        let keyboard_output = self.keyboard.get(clk);
        mux4way16(ram_output, ram_output, screen_output, keyboard_output, [address[13], address[14]])
    }

    pub fn get_screen(&self) -> [bit; 131072] {
        self.screen.get_all()
    }
}

#[derive(Copy, Clone)]
pub struct MemoryBuiltIn {
    ram: RAM16KBuiltIn,
    screen: ScreenBuiltIn,
    keyboard: KeyboardBuiltIn,
}

impl MemoryBuiltIn {
    pub fn new() -> Self {
        MemoryBuiltIn { 
            ram: RAM16KBuiltIn::new(), 
            screen: ScreenBuiltIn::new(),
            keyboard: KeyboardBuiltIn::new()
        }
    }

    fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 15], key_code: word) {
        let (ram_load, screen_load) = dmux_built_in(load, address[14]);
        self.ram.update(clk, input, ram_load, bit15_to_bit14(address));
        self.screen.update(clk, input, screen_load, bit15_to_bit13(address));
        self.keyboard.update(clk, key_code);
    }

    fn get(&self, clk: bit, address: [bit; 15]) -> word {
        let ram_output = self.ram.get(clk, bit15_to_bit14(address));
        let screen_output = self.screen.get(clk, bit15_to_bit13(address));
        let keyboard_output = self.keyboard.get(clk);
        mux4way16_built_in(ram_output, ram_output, screen_output, keyboard_output, [address[13], address[14]])
    }

    pub fn get_screen(&self) -> [bit; 131072] {
        self.screen.get_all()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CPU {
    a_register: Register,
    d_register: Register,
    out_m: word,
    write_m: bit,
    pc: PC
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a_register: Register::new(),
            d_register: Register::new(),
            out_m: [false; 16],
            write_m: false,
            pc: PC::new()
        }
    }

    fn update(&mut self, clk: bit, in_m: word, instruction: word, reset: bit) {
        self.write_m = and(instruction[15], instruction[3]);

        let out_d = self.d_register.get(false);
        let out_a = self.a_register.get(false);
        let out_a_or_m = mux16(out_a, in_m, instruction[12]);
        let (out_m, out_zr, out_ng) = alu(
            out_d,  /* x */
            out_a_or_m, /* y */
            instruction[11], /* zx */
            instruction[10], /* nx */
            instruction[9], /* zy */
            instruction[8], /* ny */
            instruction[7], /* f */
            instruction[6] /* no */
        );
        self.out_m = out_m;

        let in_a = mux16(instruction, out_m, instruction[15]);
        let not15 = not(instruction[15]);
        let write_a = or(not15, instruction[5]);
        self.a_register.update(clk, in_a, write_a);

        let write_d = and(instruction[15], instruction[4]);
        self.d_register.update(clk, out_m, write_d);

        let w0 = and(instruction[2], out_ng);
        let w1 = and(instruction[1], out_zr);
        let out_zr_or_ng = or(out_zr, out_ng);
        let out_pg = not(out_zr_or_ng);
        let w2 = and(instruction[0], out_pg);
        let w3 = or(w0, w1);
        let out_jump = or(w3, w2);
        let write_pc = and(instruction[15], out_jump);
        self.pc.update(clk, out_a, write_pc, true, reset);
    }

    fn get(&self, clk: bit) -> (word, bit, word, [bit; 15]) {
        let pc = self.pc.get(clk);
        let out_a = self.a_register.get(clk);
        let address_m = word_to_bit15(out_a);
        (self.out_m, self.write_m, pc, address_m)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CPUBuiltIn {
    a_register: Register,
    d_register: Register,
    out_m: word,
    write_m: bit,
    pc: PC
}

impl CPUBuiltIn {
    pub fn new() -> Self {
        CPUBuiltIn {
            a_register: Register::new(),
            d_register: Register::new(),
            out_m: [false; 16],
            write_m: false,
            pc: PC::new()
        }
    }

    fn update(&mut self, clk: bit, in_m: word, instruction: word, reset: bit) {
        self.write_m = instruction[15] && instruction[3];

        let out_d = self.d_register.get(false);
        let out_a = self.a_register.get(false);
        let out_a_or_m = mux16_built_in(out_a, in_m, instruction[12]);
        let (out_m, out_zr, out_ng) = alu_built_in(
            out_d,  /* x */
            out_a_or_m, /* y */
            instruction[11], /* zx */
            instruction[10], /* nx */
            instruction[9], /* zy */
            instruction[8], /* ny */
            instruction[7], /* f */
            instruction[6] /* no */
        );
        self.out_m = out_m;

        let in_a = mux16_built_in(instruction, out_m, instruction[15]);

        let not15 = !instruction[15];
        let write_a = not15 || instruction[5];
        self.a_register.update(clk, in_a, write_a);

        let write_d = instruction[15] && instruction[4];
        self.d_register.update(clk, out_m, write_d);

        let w0 = instruction[2] && out_ng;
        let w1 = instruction[1] && out_zr;
        let out_zr_or_ng = out_zr || out_ng;
        let out_pg = !out_zr_or_ng;
        let w2 = instruction[0] && out_pg;
        let w3 = w0 || w1;
        let out_jump = w3 || w2;
        let write_pc = instruction[15] && out_jump;
        self.pc.update(clk, out_a, write_pc, true, reset);
    }

    fn get(&self, clk: bit) -> (word, bit, word, [bit; 15]) {
        let pc = self.pc.get(clk);
        let out_a = self.a_register.get(clk);
        let address_m = word_to_bit15(out_a);
        (self.out_m, self.write_m, pc, address_m)
    }
}

#[derive(Copy, Clone)]
pub struct ROM32K {
    rams: [RAM4K; 8]
}

impl ROM32K {
    pub fn new() -> Self {
        ROM32K {
            rams: [RAM4K::new(); 8]
        }
    }

    pub fn update(&mut self, clk: bit, input: word, address: [bit; 15]) {
        let address_low = bit15_to_bit12(address);
        let address_high = [address[12], address[13], address[14]];
        let (a, b, c, d, e, f, g, h) = dmux8way(true, address_high);
        self.rams[0].update(clk, input, a, address_low);
        self.rams[1].update(clk, input, b, address_low);
        self.rams[2].update(clk, input, c, address_low);
        self.rams[3].update(clk, input, d, address_low);
        self.rams[4].update(clk, input, e, address_low);
        self.rams[5].update(clk, input, f, address_low);
        self.rams[6].update(clk, input, g, address_low);
        self.rams[7].update(clk, input, h, address_low);
    }

    fn get(&self, clk: bit, address: [bit; 15]) -> word {
        let address_low = bit15_to_bit12(address);
        let address_high = [address[12], address[13], address[14]];
        mux8way16(
            self.rams[0].get(clk, address_low),
            self.rams[1].get(clk, address_low),
            self.rams[2].get(clk, address_low),
            self.rams[3].get(clk, address_low),
            self.rams[4].get(clk, address_low),
            self.rams[5].get(clk, address_low),
            self.rams[6].get(clk, address_low),
            self.rams[7].get(clk, address_low),
            address_high
        )
    }

    pub fn load(&mut self, instructions: Vec<&str>) {
        let mut counter = u16_to_word(0b0000000000000000);
        for instruction in instructions {
            let mut decorded_instruction = u16_to_word(0b0000000000000000);
            for (i, c) in instruction.chars().enumerate() {
                if c == '1' {
                    decorded_instruction[15 - i] = true;
                }
            }
            // println!("instruction: {}", word_to_u16(decorded_instruction));
    
            let address = word_to_bit15(counter);
            self.update(true, decorded_instruction, address);
            counter = add16(counter, u16_to_word(0b0000000000000001));
        }
    }
}

#[derive(Copy, Clone)]
pub struct ROM32KBuiltIn {
    rams: [RAM4KBuiltIn; 8],
}

impl ROM32KBuiltIn {
    pub fn new() -> Self {
        ROM32KBuiltIn {
            rams: [RAM4KBuiltIn::new(); 8],
        }
    }

    pub fn update(&mut self, clk: bit, input: word, address: [bit; 15]) {
        let address_low = bit15_to_bit12(address);
        let address_high = [address[12], address[13], address[14]];
        let (a, b, c, d, e, f, g, h) = dmux8way_built_in(true, address_high);
        self.rams[0].update(clk, input, a, address_low);
        self.rams[1].update(clk, input, b, address_low);
        self.rams[2].update(clk, input, c, address_low);
        self.rams[3].update(clk, input, d, address_low);
        self.rams[4].update(clk, input, e, address_low);
        self.rams[5].update(clk, input, f, address_low);
        self.rams[6].update(clk, input, g, address_low);
        self.rams[7].update(clk, input, h, address_low);
    }

    fn get(&self, clk: bit, address: [bit; 15]) -> word {
        let address_low = bit15_to_bit12(address);
        let address_high = [address[12], address[13], address[14]];
        mux8way16_built_in(
            self.rams[0].get(clk, address_low),
            self.rams[1].get(clk, address_low),
            self.rams[2].get(clk, address_low),
            self.rams[3].get(clk, address_low),
            self.rams[4].get(clk, address_low),
            self.rams[5].get(clk, address_low),
            self.rams[6].get(clk, address_low),
            self.rams[7].get(clk, address_low),
            address_high
        )
    }

    pub fn load(&mut self, instructions: Vec<&str>) {
        let mut counter = u16_to_word(0b0000000000000000);
        for instruction in instructions {
            let mut decorded_instruction = u16_to_word(0b0000000000000000);
            for (i, c) in instruction.chars().enumerate() {
                if c == '1' {
                    decorded_instruction[15 - i] = true;
                }
            }
    
            let address = word_to_bit15(counter);
            self.update(true, decorded_instruction, address);
            counter = add16_built_in(counter, u16_to_word(0b0000000000000001));
        }
    }
}

#[derive(Copy, Clone)]
pub struct Computer {
    rom: ROM32K,
    cpu: CPU,
    memory: Memory,
    in_m: word,
    pc_address: [bit; 15]
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            rom: ROM32K::new(),
            cpu: CPU::new(),
            memory: Memory::new(),
            in_m: [false; 16],
            pc_address: [false; 15]
        }
    }

    pub fn load_program(&mut self, instructions: Vec<&str>) {
        self.rom.load(instructions);
    }

    fn update(&mut self, clk: bit, reset: bit, key_code: word) {
        let instruction = self.rom.get(clk, self.pc_address);
        // println!("instruction: {}", word_to_u16(instruction));
        self.cpu.update(clk, self.in_m, instruction, reset);
        let (out_m, write_m, pc, address_m) = self.cpu.get(clk);
        self.pc_address = word_to_bit15(pc);
        // println!("  pc: {} address {}", word_to_u16(instruction), bit15_to_u16(self.pc_address));
        self.memory.update(clk, out_m, write_m, address_m, key_code);
        self.in_m = self.memory.get(clk, address_m);
    }

    pub fn step(&mut self, reset: bit, word: u16) {
        let word = u16_to_word(word);
        let mut clk = true;
        self.update(clk, reset, word);
        clk = !clk;
        self.update(clk, reset, word);
    }

    pub fn get_screen(&self) -> [bit; 131072] {
        self.memory.get_screen()
    }
}

#[derive(Copy, Clone)]
pub struct ComputerBuiltIn {
    rom: ROM32KBuiltIn,
    cpu: CPUBuiltIn,
    memory: MemoryBuiltIn,
    in_m: word,
    pc_address: [bit; 15]
}

impl ComputerBuiltIn {
    pub fn new() -> Self {
        ComputerBuiltIn {
            rom: ROM32KBuiltIn::new(),
            cpu: CPUBuiltIn::new(),
            memory: MemoryBuiltIn::new(),
            in_m: [false; 16],
            pc_address: [false; 15]
        }
    }

    pub fn load_program(&mut self, instructions: Vec<&str>) {
        self.rom.load(instructions);
    }

    fn update(&mut self, clk: bit, reset: bit, key_code: word) {
        let instruction = self.rom.get(clk, self.pc_address);
        // println!("instruction: {}", word_to_u16(instruction));
        self.cpu.update(clk, self.in_m, instruction, reset);
        let (out_m, write_m, pc, address_m) = self.cpu.get(clk);
        self.pc_address = word_to_bit15(pc);
        // println!("  pc: {} address {}", word_to_u16(instruction), bit15_to_u16(self.pc_address));
        self.memory.update(clk, out_m, write_m, address_m, key_code);
        self.in_m = self.memory.get(clk, address_m);
    }

    pub fn step(&mut self, reset: bit, word: u16) {
        let word = u16_to_word(word);
        let mut clk = true;
        self.update(clk, reset, word);
        clk = !clk;
        self.update(clk, reset, word);
    }

    pub fn get_screen(&self) -> [bit; 131072] {
        self.memory.get_screen()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {
        let mut clk = true;
        let mut cpu = CPU::new();

        let word0 = u16_to_word(0b0000_0000_0000_0000);
        let word1 = u16_to_word(0b0010_1011_0110_0111); // 11111
        let mut in_m = word0;

        let mut instruction = u16_to_word(0b0011_0000_0011_1001); // @12345
        let mut reset = false;

        cpu.update(clk, in_m, instruction, reset);
        let (_, mut write_m, mut pc, mut address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_0000)), address_m);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), pc);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), cpu.d_register.get(clk));    // 0

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0011_0000_0011_1001)), address_m);    // 12345
        assert_eq!(u16_to_word(0b0000_0000_0000_0001), pc);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), cpu.d_register.get(clk));    // 0

        clk = !clk;
        instruction = u16_to_word(0b1110_1100_0001_0000); // D=A

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0011_0000_0011_1001)), address_m);    // 12345
        assert_eq!(u16_to_word(0b0000_0000_0000_0001), pc);

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0011_0000_0011_1001)), address_m);    // 12345
        assert_eq!(u16_to_word(0b0000_0000_0000_0010), pc); // 2

        clk = !clk;
        instruction = u16_to_word(0b0101_1011_1010_0000); // @23456

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0011_0000_0011_1001)), address_m);    // 12345
        assert_eq!(u16_to_word(0b0000_0000_0000_0010), pc); // 2

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0101_1011_1010_0000)), address_m);    // 23456
        assert_eq!(u16_to_word(0b0000_0000_0000_0011), pc); // 3

        clk = !clk;
        instruction = u16_to_word(0b1110_0001_1101_0000); // D=A-D

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0101_1011_1010_0000)), address_m);    // 23456
        assert_eq!(u16_to_word(0b0000_0000_0000_0011), pc); // 3

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0101_1011_1010_0000)), address_m);    // 23456
        assert_eq!(u16_to_word(0b0000_0000_0000_0100), pc); // 4

        clk = !clk;
        instruction = u16_to_word(0b0000_0011_1110_1000); // @1000

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0101_1011_1010_0000)), address_m);    // 23456
        assert_eq!(u16_to_word(0b0000_0000_0000_0100), pc); // 4

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_0101), pc); // 5

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_1000); // M=D

        cpu.update(clk, in_m, instruction, reset);
        let (mut out_m, mut write_m, mut pc, mut address_m) = cpu.get(clk);
        assert_eq!(u16_to_word(0b0010_1011_0110_0111), out_m); // 11111
        assert_eq!(true, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_0101), pc); // 5

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (out_m, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(u16_to_word(0b0010_1011_0110_0111), out_m); // 11111
        assert_eq!(true, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_0110), pc); // 6

        clk = !clk;
        instruction = u16_to_word(0b0000_0011_1110_1001); // @1001

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_0110), pc); // 6

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1001)), address_m);    // 1001
        assert_eq!(u16_to_word(0b0000_0000_0000_0111), pc); // 7

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_1001_1000); // MD=D-1

        cpu.update(clk, in_m, instruction, reset);
        (out_m, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(u16_to_word(0b0010_1011_0110_0110), out_m); // 11110
        assert_eq!(true, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1001)), address_m);    // 1001
        assert_eq!(u16_to_word(0b0000_0000_0000_0111), pc); // 7

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (out_m, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(u16_to_word(0b0010_1011_0110_0101), out_m); // 11109
        assert_eq!(true, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1001)), address_m);    // 1001
        assert_eq!(u16_to_word(0b0000_0000_0000_1000), pc); // 8

        clk = !clk;
        instruction = u16_to_word(0b0000_0011_1110_1000); // @1000

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1001)), address_m);    // 1001
        assert_eq!(u16_to_word(0b0000_0000_0000_1000), pc); // 8

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_1001), pc); // 9

        clk = !clk;
        in_m = word1;
        instruction = u16_to_word(0b1111_0100_1101_0000); // D=D-M

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_1001), pc); // 9

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_1010), pc); // 10

        clk = !clk;
        instruction = u16_to_word(0b0000_0000_0000_1110); // @14

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_1010), pc); // 10

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_1110)), address_m);    // 14
        assert_eq!(u16_to_word(0b0000_0000_0000_1011), pc); // 11

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0100); // D;jlt

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_1110)), address_m);    // 14
        assert_eq!(u16_to_word(0b0000_0000_0000_1011), pc); // 11

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_1110)), address_m);    // 14
        assert_eq!(u16_to_word(0b0000_0000_0000_1110), pc); // 14

        clk = !clk;
        instruction = u16_to_word(0b0000_0011_1110_0111); // @999

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_1110)), address_m);    // 14
        assert_eq!(u16_to_word(0b0000_0000_0000_1110), pc); // 14

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_0111)), address_m);    // 999
        assert_eq!(u16_to_word(0b0000_0000_0000_1111), pc); // 15

        clk = !clk;
        instruction = u16_to_word(0b1110_1101_1110_0000); // A=A+1

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_0111)), address_m);    // 999
        assert_eq!(u16_to_word(0b0000_0000_0000_1111), pc); // 15

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_0000), pc); // 16

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_1000); // M=D

        cpu.update(clk, in_m, instruction, reset);
        (out_m, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(u16_to_word(0b1111_1111_1111_1111), out_m); // -1
        assert_eq!(true, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_0000), pc); // 16

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (out_m, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(u16_to_word(0b1111_1111_1111_1111), out_m); // -1
        assert_eq!(true, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_0001), pc); // 17

        clk = !clk;
        instruction = u16_to_word(0b0000_0000_0001_0101); // @21

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_0001), pc); // 17

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0001_0101)), address_m);    // 21
        assert_eq!(u16_to_word(0b0000_0000_0001_0010), pc); // 18

        clk = !clk;
        instruction = u16_to_word(0b1110_0111_1100_0010); // D+1;jeq

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0001_0101)), address_m);    // 21
        assert_eq!(u16_to_word(0b0000_0000_0001_0010), pc); // 18

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0001_0101)), address_m);    // 21
        assert_eq!(u16_to_word(0b0000_0000_0001_0101), pc); // 21

        clk = !clk;
        instruction = u16_to_word(0b0000_0000_0000_0010); // @2

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0001_0101)), address_m);    // 21
        assert_eq!(u16_to_word(0b0000_0000_0001_0101), pc); // 21

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_0010)), address_m);    // 2
        assert_eq!(u16_to_word(0b0000_0000_0001_0110), pc); // 22

        clk = !clk;
        instruction = u16_to_word(0b1110_0000_1001_0000); // D=D+A

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_0010)), address_m);    // 2
        assert_eq!(u16_to_word(0b0000_0000_0001_0110), pc); // 22

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_0010)), address_m);    // 2
        assert_eq!(u16_to_word(0b0000_0000_0001_0111), pc); // 23

        clk = !clk;
        instruction = u16_to_word(0b0000_0011_1110_1000); // @1000

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0000_0000_0010)), address_m);    // 2
        assert_eq!(u16_to_word(0b0000_0000_0001_0111), pc); // 23

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1000), pc); // 24

        clk = !clk;
        instruction = u16_to_word(0b1110_1110_1001_0000); // D=-1

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1000), pc); // 24

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1001), pc); // 25

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0001); // D;JGT

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1001), pc); // 25

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1010), pc); // 26

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0010); // D;JEQ

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1010), pc); // 26

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1011), pc); // 27

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0011); // D;JGE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1011), pc); // 27

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1100), pc); // 28

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0100); // D;JLT

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0001_1100), pc); // 28
    
        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000
    
        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0101); // D;JNE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0110); // D;JLE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0111); // D;JMP

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_1010_1001_0000); // D=0

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0001); // D;JGT

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1010), pc); // 1002

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0010); // D;JEQ

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1010), pc); // 1002

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0011); // D;JGE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0100); // D;JLT

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0101); // D;JNE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1010), pc); // 1002

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0110); // D;JLE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1010), pc); // 1002

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0111); // D;JMP

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_1111_1101_0000); // D=1

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0001); // D;JGT

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0010); // D;JEQ

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0011); // D;JGE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0100); // D;JLT

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0101); // D;JNE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0110); // D;JLE

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;
        instruction = u16_to_word(0b1110_0011_0000_0111); // D;JMP

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1001), pc); // 1001

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;
        reset = true;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0011_1110_1000), pc); // 1000

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), pc); // 0

        clk = !clk;
        instruction = u16_to_word(0b0111_1111_1111_1111); // @32767
        reset = false;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0000_0011_1110_1000)), address_m);    // 1000
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), pc); // 0

        clk = !clk;

        cpu.update(clk, in_m, instruction, reset);
        (_, write_m, pc, address_m) = cpu.get(clk);
        assert_eq!(false, write_m);
        assert_eq!(word_to_bit15(u16_to_word(0b0111_1111_1111_1111)), address_m);    // 32767
        assert_eq!(u16_to_word(0b0000_0000_0000_0001), pc); // 1
    }

    #[test]
    fn test_screen() {
        let mut clk = true;
        let mut screen = Screen::new();

        let word_i = u16_to_word(0b1100_1010_0011_0101);
        let word_o = u16_to_word(0b0011_0101_1100_1010);
        let word_0 = u16_to_word(0b0000_0000_0000_0000);

        let mut input = word_i;
        let mut load = true;
        let mut address = word_to_bit13(u16_to_word(0b000_0000_0000_0000));
        screen.update(clk, input, load, address);
        let mut output = screen.get(clk, address);
        assert_eq!(word_0, output);

        clk = !clk;
        input = word_o;
        load = false;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_i, output);

        clk = !clk;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_i, output);

        clk = !clk;
        load = true;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_i, output);

        clk = !clk;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_i, output);

        clk = !clk;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_o, output);

        clk = !clk;
        address = word_to_bit13(u16_to_word(0b000_1001_0010_0100));

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_0, output);

        clk = !clk;
        load = false;
        input = word_i;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_o, output);

        clk = !clk;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_o, output);

        clk = !clk;
        load = true;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_o, output);

        clk = !clk;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_o, output);

        clk = !clk;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_i, output);

        clk = !clk;
        address = word_to_bit13(u16_to_word(0b000_0010_0100_1001));

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_0, output);

        clk = !clk;
        input = word_o;
        load = false;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_i, output);

        clk = !clk;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_i, output);

        clk = !clk;
        load = true;

        screen.update(clk, input, load, address);
        output = screen.get(clk, address);
        assert_eq!(word_i, output);
    }

    #[test]
    fn test_keyboard() {
        let mut clk = true;
        let mut keyboard = Keyboard::new();

        let word_0 = u16_to_word(0b0000_0000_0000_0000);
        let word_a = u16_to_word(0b0000_0000_0100_0001);  // a
        let word_A = u16_to_word(0b0000_0000_0110_0001);  // A
        
        let mut word = word_a;
        keyboard.update(clk, word);
        let mut key_code = keyboard.get(clk);
        assert_eq!(word_0, key_code);

        clk = !clk;

        keyboard.update(clk, word);
        key_code = keyboard.get(clk);
        assert_eq!(word_a, key_code);

        clk = !clk;
        word = word_A;

        keyboard.update(clk, word);
        key_code = keyboard.get(clk);
        assert_eq!(word_a, key_code);

        clk = !clk;

        keyboard.update(clk, word);
        key_code = keyboard.get(clk);
        assert_eq!(word_A, key_code);
    }

    #[test]
    fn test_memory() {
        let mut clk = true;
        let mut memory = Memory::new();

        // Set RAM[0] = -1
        let mut input = u16_to_word(0b1111_1111_1111_1111); // -1
        let mut load = true;
        let mut address = word_to_bit15(u16_to_word(0b000_0000_0000_0000));
        let mut key_code = u16_to_word(0b0000_0000_0000_0000);

        memory.update(clk, input, load, address, key_code);
        let mut output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b1111_1111_1111_1111), output); // -1

        // RAM[0] holds value
        clk = !clk;
        input = u16_to_word(0b0010_0111_0000_1111); // 9999
        load = false;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b1111_1111_1111_1111), output); // -1

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b1111_1111_1111_1111), output); // -1

        // Did not also write to upper RAM or Screen
        clk = !clk;
        address = word_to_bit15(u16_to_word(0b010_0000_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_0000_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        // Set RAM[2000] = 2222
        clk = !clk;
        input = u16_to_word(0b0000_1000_1010_1110); // 2222
        load = true;
        address = word_to_bit15(u16_to_word(0b010_0000_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_1000_1010_1110), output); // 2222

        // RAM[2000] holds value
        clk = !clk;
        input = u16_to_word(0b0010_0111_0000_1111); // 9999
        load = false;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_1000_1010_1110), output); // 2222

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_1000_1010_1110), output); // 2222

        // Did not also write to lower RAM or Screen
        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b1111_1111_1111_1111), output); // -1

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_0000_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        // Low order address bits connected
        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_0000_0001));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_0000_0010));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_0000_0100));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_0000_1000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_0001_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_0010_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_0100_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0000_1000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0001_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0010_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_0100_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_1000_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b001_0000_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b010_0000_0000_0000));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_1000_1010_1110), output); // 2222

        // RAM[1234] = 1234
        clk = !clk;
        input = u16_to_word(0b0000_0100_1101_0010); // 1234
        load = true;
        address = word_to_bit15(u16_to_word(0b001_0010_0011_0100));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0100_1101_0010), output); // 1234

        // Did not also write to upper RAM or Screen 
        clk = !clk;
        load = false;
        address = word_to_bit15(u16_to_word(0b010_0010_0011_0100));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0
    
        clk = !clk;
        address = word_to_bit15(u16_to_word(0b110_0010_0011_0100));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        // RAM[2345] = 2345
        clk = !clk;
        input = u16_to_word(0b0000_1001_0010_1001); // 2345
        load = true;
        address = word_to_bit15(u16_to_word(0b010_0011_0100_0101));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_1001_0010_1001), output); // 2345

        // Did not also write to lower RAM or Screen
        clk = !clk;
        load = false;
        address = word_to_bit15(u16_to_word(0b000_0011_0100_0101));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_0011_0100_0101));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        // Keyboard test
        clk = !clk;
        address = word_to_bit15(u16_to_word(0b110_0000_0000_0000)); // 24576
        key_code = u16_to_word(0b0000_0000_0100_1011);   // K

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0100_1011), output); // 75

        // Screen test
        clk = !clk;
        input = u16_to_word(0b1111_1111_1111_1111); // -1
        load = true;
        address = word_to_bit15(u16_to_word(0b100_1111_1100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b1111_1111_1111_1111), output); // -1

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b101_0000_0100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b1111_1111_1111_1111), output); // -1

        // Did not also write to lower or upper RAM
        clk = !clk;
        address = word_to_bit15(u16_to_word(0b000_1111_1100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b010_1111_1100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        // Low order address bits connected
        clk = !clk;
        load = false;
        address = word_to_bit15(u16_to_word(0b100_1111_1100_1110));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1111_1100_1101));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1111_1100_1011));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1111_1100_0111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1111_1101_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1111_1110_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1111_1000_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1111_0100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1110_1100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1101_1100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_1011_1100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b100_0111_1100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        clk = !clk;
        address = word_to_bit15(u16_to_word(0b101_1111_1100_1111));

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output); // 0

        // Keyboard test
        clk = !clk;
        address = word_to_bit15(u16_to_word(0b110_0000_0000_0000)); // 24576
        key_code = u16_to_word(0b0000_0000_0101_1001);   // Y

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);

        clk = !clk;

        memory.update(clk, input, load, address, key_code);
        output = memory.get(clk, address);

        assert_eq!(u16_to_word(0b0000_0000_0101_1001), output); // 89
    }

    #[test]
    fn test_rom32k() {
        let mut rom = ROM32K::new();
        let instructions: Vec<&str> = vec![
            "0110000000000000",
            "1111110000010000",
            "0000000000001000",
            "1110001100000010",
            "0000000000000000",
            "1110110010010000",
            "0000000000001010",
            "1110101010000111",
            "0000000000000000",
            "1110110000010000",
            "0000000000010000",
            "1110001100001000",
            "0100000000000000",
            "1110110000010000",
            "0000000000010001",
            "1110001100001000",
            "0010000000000000",
            "1110110000010000",
            "0000000000010010",
            "1110001100001000",
            "0000000000010010",
            "1111110000010000",
            "0000000000100011",
            "1110001100000010",
            "0000000000010000",
            "1111110000010000",
            "0000000000010001",
            "1111110000100000",
            "1110001100001000",
            "0000000000010001",
            "1111110111001000",
            "0000000000010010",
            "1111110010001000",
            "0000000000010100",
            "1110101010000111",
            "0000000000000000",
            "1110101010000111",
        ];
        rom.load(instructions);

        let mut clk = true;
        let mut address = word_to_bit15(u16_to_word(0));
        let mut output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b0110_0000_0000_0000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(1));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b1111_1100_0001_0000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(2));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_1000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(3));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b1110_0011_0000_0010), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(4));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(5));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b1110_1100_1001_0000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(6));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_1010), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(7));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b1110_1010_1000_0111), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(8));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0000_0000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(9));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b1110_1100_0001_0000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(10));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b0000_0000_0001_0000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(11));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b1110_0011_0000_1000), output);

        clk = !clk;
        address = word_to_bit15(u16_to_word(12));
        output = rom.get(clk, address);
        assert_eq!(u16_to_word(0b0100_0000_0000_0000), output);
    }

    #[test]
    fn test_computer() {
        assert!(true);
    }
}
