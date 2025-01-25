use crate::boolean_logic::*;
use crate::helper::*;
use crate::boolean_arithmetic::*;

#[derive(Debug, Copy, Clone)]
pub struct DFF {
    past_bit: bit,
    new_bit: bit
}

impl DFF {
    pub fn new() -> Self {
        DFF {
            past_bit: false,
            new_bit: false
        }
    }

    pub fn update(&mut self, clk: bit, a: bit) {
        if clk {
            self.past_bit = self.new_bit;
            self.new_bit = a
        }
    }

    pub fn get(self, clk: bit) -> bit {
        if clk { self.past_bit } else { self.new_bit }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bit {
    dff: DFF
}

impl Bit {
    pub fn new() -> Self {
        Bit { dff: DFF::new() }
    }

    pub fn update(&mut self, clk: bit, input: bit, load: bit) {
        self.dff.update(clk, mux(self.get(!clk), input, load))
    }

    pub fn get(&self, clk: bit) -> bit {
        self.dff.get(clk)
    }
}

#[derive(Debug, Clone)]
pub struct Register {
    bits: Vec<Bit>,
}

impl Register {
    pub fn new() -> Self {
        let mut bits = Vec::new();
        for _ in 0..16 {
            bits.push(Bit::new());
        }
        Register { bits }
    }

    pub fn update(&mut self, clk: bit, input: word, load: bit) {
        for i in 0..16 {
            self.bits[i].update(clk, input[i], load);
        }
    }

    pub fn get(&self, clk: bit) -> word {
        let mut word = u16_to_word(0b0000_0000_0000_0000);
        for i in 0..16 {
            word[i] = self.bits[i].get(clk);
        }
        word
    }
}

#[derive(Debug, Clone)]
pub struct RAM8 {
    registers: Vec<Register>,
}

impl RAM8 {
    fn new() -> Self {
        let mut registers = Vec::new();
        for _ in 0..8 {
            registers.push(Register::new());
        }
        RAM8 { registers }
    }

    fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 3]) {
        let (a, b, c, d, e, f, g, h) = dmux8way(load, address);
        self.registers[0].update(clk, input, a);
        self.registers[1].update(clk, input, b);
        self.registers[2].update(clk, input, c);
        self.registers[3].update(clk, input, d);
        self.registers[4].update(clk, input, e);
        self.registers[5].update(clk, input, f);
        self.registers[6].update(clk, input, g);
        self.registers[7].update(clk, input, h);
    }

    fn get(&self, clk: bit, address: [bit; 3]) -> word {
        mux8way16(
            self.registers[0].get(clk),
            self.registers[1].get(clk),
            self.registers[2].get(clk),
            self.registers[3].get(clk),
            self.registers[4].get(clk),
            self.registers[5].get(clk),
            self.registers[6].get(clk),
            self.registers[7].get(clk),
            address
        )
    }
}

#[derive(Debug, Clone)]
pub struct RAM64 {
    rams: Vec<RAM8>,
}

impl RAM64 {
    fn new() -> Self {
        let mut rams = Vec::new();
        for _ in 0..8 {
            rams.push(RAM8::new());
        }
        RAM64 { rams }
    }

    fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 6]) {
        let address_low = [address[0], address[1], address[2]];
        let address_high = [address[3], address[4], address[5]];
        let (a, b, c, d, e, f, g, h) = dmux8way(load, address_high);
        self.rams[0].update(clk, input, a, address_low);
        self.rams[1].update(clk, input, b, address_low);
        self.rams[2].update(clk, input, c, address_low);
        self.rams[3].update(clk, input, d, address_low);
        self.rams[4].update(clk, input, e, address_low);
        self.rams[5].update(clk, input, f, address_low);
        self.rams[6].update(clk, input, g, address_low);
        self.rams[7].update(clk, input, h, address_low);
    }

    fn get(&self, clk: bit, address: [bit; 6]) -> word {
        let address_low = [address[0], address[1], address[2]];
        let address_high = [address[3], address[4], address[5]];
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
}

#[derive(Debug, Clone)]
pub struct RAM512 {
    rams: Vec<RAM64>,
}

impl RAM512 {
    fn new() -> Self {
        let mut rams = Vec::new();
        for _ in 0..8 {
            rams.push(RAM64::new());
        }
        RAM512 { rams }
    }

    fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 9]) {
        let address_low = [address[0], address[1], address[2], address[3], address[4], address[5]];
        let address_high = [address[6], address[7], address[8]];
        let (a, b, c, d, e, f, g, h) = dmux8way(load, address_high);
        self.rams[0].update(clk, input, a, address_low);
        self.rams[1].update(clk, input, b, address_low);
        self.rams[2].update(clk, input, c, address_low);
        self.rams[3].update(clk, input, d, address_low);
        self.rams[4].update(clk, input, e, address_low);
        self.rams[5].update(clk, input, f, address_low);
        self.rams[6].update(clk, input, g, address_low);
        self.rams[7].update(clk, input, h, address_low);
    }

    fn get(&self, clk: bit, address: [bit; 9]) -> word {
        let address_low = [address[0], address[1], address[2], address[3], address[4], address[5]];
        let address_high = [address[6], address[7], address[8]];
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
}

#[derive(Debug, Clone)]
pub struct RAM4K {
    rams: Vec<RAM512>,
}

impl RAM4K {
    pub fn new() -> Self {
        let mut rams = Vec::new();
        for _ in 0..8 {
            rams.push(RAM512::new());
        }
        RAM4K { rams }
    }

    pub fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 12]) {
        let address_low = [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]];
        let address_high = [address[9], address[10], address[11]];
        let (a, b, c, d, e, f, g, h) = dmux8way(load, address_high);
        self.rams[0].update(clk, input, a, address_low);
        self.rams[1].update(clk, input, b, address_low);
        self.rams[2].update(clk, input, c, address_low);
        self.rams[3].update(clk, input, d, address_low);
        self.rams[4].update(clk, input, e, address_low);
        self.rams[5].update(clk, input, f, address_low);
        self.rams[6].update(clk, input, g, address_low);
        self.rams[7].update(clk, input, h, address_low);
    }

    pub fn get(&self, clk: bit, address: [bit; 12]) -> word {
        let address_low = [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]];
        let address_high = [address[9], address[10], address[11]];
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
}

#[derive(Debug, Clone)]
pub struct RAM4KBuiltIn {
    ram: Vec<word>,
}

impl RAM4KBuiltIn {
    pub fn new() -> Self {
        let mut ram = Vec::new();
        for _ in 0..4096 {
            ram.push(u16_to_word(0b0000_0000_0000_0000));
        }
        RAM4KBuiltIn { ram }
    }

    pub fn update(&mut self, _clk: bit, input: word, _load: bit, address: [bit; 12]) {
        self.ram[bit12_to_u16(address) as usize] = input;
    }

    pub fn get(&self, _clk: bit, address: [bit; 12]) -> word {
        self.ram[bit12_to_u16(address) as usize]
    }
}


#[derive(Debug, Clone)]
pub struct RAM16K {
    rams: Vec<RAM4K>,
}

impl RAM16K {
    pub fn new() -> Self {
        let mut rams = Vec::new();
        for _ in 0..4 {
            rams.push(RAM4K::new());
        }
        RAM16K { rams }
    }

    pub fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 14]) {
        let address_low = [
            address[0], address[1], address[2], address[3],
            address[4], address[5], address[6], address[7],
            address[8], address[9], address[10], address[11]
        ];
        let address_high = [address[12], address[13]];
        let (a, b, c, d) = dmux4way(load, address_high);
        self.rams[0].update(clk, input, a, address_low);
        self.rams[1].update(clk, input, b, address_low);
        self.rams[2].update(clk, input, c, address_low);
        self.rams[3].update(clk, input, d, address_low);
    }

    pub fn get(&self, clk: bit, address: [bit; 14]) -> word {
        let address_low = [
            address[0], address[1], address[2], address[3],
            address[4], address[5], address[6], address[7],
            address[8], address[9], address[10], address[11]
        ];
        let address_high = [address[12], address[13]];
        mux4way16(
            self.rams[0].get(clk, address_low),
            self.rams[1].get(clk, address_low),
            self.rams[2].get(clk, address_low),
            self.rams[3].get(clk, address_low),
            address_high
        )
    }
}

#[derive(Debug, Clone)]
pub struct RAM16KBuiltIn {
    ram: Vec<word>,
}

impl RAM16KBuiltIn {
    pub fn new() -> Self {
        let mut ram = Vec::new();
        for _ in 0..16384 /* 14bit */ {
            ram.push(u16_to_word(0b0000_0000_0000_0000));
        }
        RAM16KBuiltIn { ram }
    }

    pub fn update(&mut self, clk: bit, input: word, load: bit, address: [bit; 14]) {
        if clk && load {
            self.ram[bit14_to_u16(address) as usize] = input;
        }
    }

    pub fn get(&self, _clk: bit, address: [bit; 14]) -> word {
        self.ram[bit14_to_u16(address) as usize]
    }
}

#[derive(Debug, Clone)]
pub struct PC {
    counter: Register
}

impl PC {
    pub fn new() -> Self {
        PC { counter: Register::new() }
    }

    pub fn update(&mut self, clk: bit, input: word, load: bit, inc: bit, reset: bit) {
        let out0 = self.get(!clk);
        // let incout = add16(out0, u16_to_word(0b0000_0000_0000_0001));
        let incout = inc16(out0);
        let out1 = mux16(out0, incout, inc);
        let out2 = mux4way16(
            out1,
            input,
            u16_to_word(0b0000_0000_0000_0000),
            u16_to_word(0b0000_0000_0000_0000),
            [load, reset]
        );
        self.counter.update(clk, out2, true);
    }

    pub fn get(&self, clk: bit) -> word {
        self.counter.get(clk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dff() {
        let mut dff = DFF::new();
        let mut clk = true;

        dff.update(clk, true);
        assert_eq!(false, dff.get(clk));
        clk = !clk;

        dff.update(clk, false);
        assert_eq!(true, dff.get(clk));
        clk = !clk;

        dff.update(clk, false);
        assert_eq!(true, dff.get(clk));
        clk = !clk;

        dff.update(clk, true);
        assert_eq!(false, dff.get(clk));
    }

    #[test]
    fn test_bit() {
        let mut bit = Bit::new();
        let mut clk = true;

        bit.update(clk, true, true);
        assert_eq!(false, bit.get(clk));
        clk = !clk;

        bit.update(clk, false, false);
        assert_eq!(true, bit.get(clk));
        clk = !clk;

        bit.update(clk, false, false);
        assert_eq!(true, bit.get(clk));
        clk = !clk;

        bit.update(clk, false, true);
        assert_eq!(true, bit.get(clk));
        clk = !clk;

        bit.update(clk, false, true);
        assert_eq!(true, bit.get(clk));
        clk = !clk;

        bit.update(clk, true, false);
        assert_eq!(false, bit.get(clk));
    }

    #[test]
    fn test_register() {
        let mut register = Register::new();
        let mut clk = true;

        let word_i = u16_to_word(0b1111_1111_1111_1111);
        let word_o = u16_to_word(0b0000_0000_0000_0000);

        let mut load = true;

        register.update(clk, word_i, load);
        assert_eq!(word_o, register.get(clk));
        clk = !clk;

        load = false;

        register.update(clk, word_o, load);
        assert_eq!(word_i, register.get(clk));
        clk = !clk;

        register.update(clk, word_o, load);
        assert_eq!(word_i, register.get(clk));
        clk = !clk;

        load = true;

        register.update(clk, word_o, load);
        assert_eq!(word_i, register.get(clk));
        clk = !clk;

        register.update(clk, word_o, load);
        assert_eq!(word_i, register.get(clk));
        clk = !clk;

        load = false;

        register.update(clk, word_i, load);
        assert_eq!(word_o, register.get(clk));
    }

    #[test]
    fn test_ram8() {
        let mut ram = RAM8::new();
        let mut clk = true;

        let word_i = u16_to_word(0b1100_1010_0011_0101);
        let word_o = u16_to_word(0b0011_0101_1100_1010);
        let word_0 = u16_to_word(0b0000_0000_0000_0000);

        let mut address = u8_to_3bit(0b000);
        let mut load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        address = u8_to_3bit(0b001);

        ram.update(clk, word_o, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_i, ram.get(clk, address));
    }

    #[test]
    fn test_ram64() {
        let mut ram = RAM64::new();
        let mut clk = true;

        let word_i = u16_to_word(0b1100_1010_0011_0101);
        let word_o = u16_to_word(0b0011_0101_1100_1010);
        let word_0 = u16_to_word(0b0000_0000_0000_0000);

        let mut address = u8_to_6bit(0b00_0000);
        let mut load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        address = u8_to_6bit(0b00_1001);

        ram.update(clk, word_o, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_i, ram.get(clk, address));
    }

    #[test]
    fn test_ram512() {
        let mut ram = RAM512::new();
        let mut clk = true;

        let word_i = u16_to_word(0b1100_1010_0011_0101);
        let word_o = u16_to_word(0b0011_0101_1100_1010);
        let word_0 = u16_to_word(0b0000_0000_0000_0000);

        let mut address = u16_to_9bit(0b0_0000_0000);
        let mut load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        address = u16_to_9bit(0b1_0010_0100);

        ram.update(clk, word_o, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_i, ram.get(clk, address));
    }

    #[test]
    fn test_ram4k() {
        let mut ram = RAM4K::new();
        let mut clk = true;

        let word_i = u16_to_word(0b1100_1010_0011_0101);
        let word_o = u16_to_word(0b0011_0101_1100_1010);
        let word_0 = u16_to_word(0b0000_0000_0000_0000);

        let mut address = u16_to_12bit(0b0000_0000_0000);
        let mut load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        address = u16_to_12bit(0b0010_0100_1001);

        ram.update(clk, word_o, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_i, ram.get(clk, address));
    }

    #[test]
    fn test_ram16k() {
        let mut ram = RAM16K::new();
        let mut clk = true;

        let word_i = u16_to_word(0b1100_1010_0011_0101);
        let word_o = u16_to_word(0b0011_0101_1100_1010);
        let word_0 = u16_to_word(0b0000_0000_0000_0000);

        let mut address = u16_to_14bit(0b00_0000_0000_0000);
        let mut load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_i, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_o, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        address = u16_to_14bit(0b00_1001_0010_0100);

        ram.update(clk, word_o, load, address);
        assert_eq!(word_0, ram.get(clk, address));
        clk = !clk;

        load = false;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        load = true;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_o, ram.get(clk, address));
        clk = !clk;

        ram.update(clk, word_i, load, address);
        assert_eq!(word_i, ram.get(clk, address));
    }

    #[test]
    fn test_pc() {
        let mut pc = PC::new();
        let mut clk = true;

        let word0 = u16_to_word(0b0000_0000_0000_0000);
        let word1 = u16_to_word(0b0000_0000_0000_0001);
        let input = u16_to_word(0b0100_1001_0010_0100);

        let mut inc = true;
        let mut load = false;
        let mut reset = false;

        pc.update(clk, word0, load, inc, reset);
        assert_eq!(word0, pc.get(clk));
        clk = !clk;

        pc.update(clk, word0, load, inc, reset);
        assert_eq!(add16(word0, word1), pc.get(clk));
        clk = !clk;

        pc.update(clk, word0, load, inc, reset);
        assert_eq!(add16(word0, word1), pc.get(clk));
        clk = !clk;

        pc.update(clk, word0, load, inc, reset);
        assert_eq!(add16(add16(word0, word1), word1), pc.get(clk));
        clk = !clk;

        inc = false;
        reset = true;

        pc.update(clk, word0, load, inc, reset);
        assert_eq!(add16(word1, word1), pc.get(clk));
        clk = !clk;

        load = true;
        reset = false;

        pc.update(clk, input, load, inc, reset);
        assert_eq!(word0, pc.get(clk));
        clk = !clk;

        pc.update(clk, input, load, inc, reset);
        assert_eq!(word0, pc.get(clk));
        clk = !clk;

        load = false;

        pc.update(clk, word0, load, inc, reset);
        assert_eq!(input, pc.get(clk));
        clk = !clk;

        pc.update(clk, word0, load, inc, reset);
        assert_eq!(input, pc.get(clk));
        clk = !clk;

        pc.update(clk, word0, load, inc, reset);
        assert_eq!(input, pc.get(clk));
    }
}