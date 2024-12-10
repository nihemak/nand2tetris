mod boolean_logic;
mod boolean_arithmetic;
mod sequential_circuit;
mod hardware;
mod helper;

use anyhow::Result;
use async_trait::async_trait;
use hardware::Computer;
use boolean_logic::*;
use helper::*;

use crate::{
    engine::{ComputerSystem, KeyState, Renderer},
};

#[derive(Clone)]
pub struct Nand2Tetris {
    computer: Box<Computer>,
}

impl Nand2Tetris {
    pub fn new() -> Self {
    // Fill
    let instructions: Vec<&str> = vec![
                            //(LOOP_KBD)
        "0110000000000000", //        @KBD
        "1111110000010000", //        D=M
        "0000000000001000", //        @SELECT_BLACK
        "1110001100000010", //        D; JEQ
        "0000000000000000", //        @0
        "1110110000010000", //        D=A
        "0000000000001010", //        @SET_COLOR
        "1110101010000111", //        0; JMP
                            //(SELECT_BLACK)
        "0000000000000000", //        @0
        "1110110010010000", //        D=A-1
                            //(SET_COLOR)
        "0000000000010000", //        @color
        "1110001100001000", //        M=D

        "0100000000000000", //        @SCREEN
        "1110110000010000", //        D=A
        "0000000000010001", //        @pos
        "1110001100001000", //        M=D

                            //        // 32 * 256 = 8192
        "0010000000000000", //        @8192
        "1110110000010000", //        D=A
        "0000000000010010", //        @n
        "1110001100001000", //        M=D

                            //(LOOP_FILL)
        "0000000000010010", //        @n
        "1111110000010000", //        D=M
        "0000000000100011", //        @FILL_END
        "1110001100000010", //        D; JEQ

                            //        // print color
        "0000000000010000", //        @color
        "1111110000010000", //        D=M
        "0000000000010001", //        @pos
        "1111110000100000", //        A=M
        "1110001100001000", //        M=D

        "0000000000010001", //        @pos
        "1111110111001000", //        M=M+1
        "0000000000010010", //        @n
        "1111110010001000", //        M=M-1

        "0000000000010100", //        @LOOP_FILL
        "1110101010000111", //        0; JMP
                            //(FILL_END)
        "0000000000000000", //        @LOOP_KBD
        "1110101010000111", //        0; JMP

        ];
        let mut computer = Box::new(Computer::new());
        computer.load_program(instructions);
        Nand2Tetris {
            computer: computer,
        }
    }
}

#[async_trait(?Send)]
impl ComputerSystem for Nand2Tetris {
    async fn initialize(&mut self, renderer: &Renderer, keystate: &KeyState) -> Result<Box<dyn ComputerSystem>> {
        let mut nand2tetris = Box::new(Nand2Tetris::new());
        nand2tetris.computer.step(true, get_keyboard_press_code(keystate));
        Ok(nand2tetris)
    }

    fn update(&mut self, keystate: &KeyState) {
        self.computer.step(false, get_keyboard_press_code(keystate));
    }

    fn draw(&self, renderer: &Renderer) {
        let screen = self.computer.get_screen();
        for px in 0..screen.len() {
            let x = px % 512;
            let y = px / 512;
            let color = if screen[px] {"#000000"} else {"#FFFFFF"};
            renderer.draw_pixel(color, x.try_into().unwrap(), y.try_into().unwrap());
        }
    }
}

fn get_keyboard_press_code(keystate: &KeyState) -> word {
    let mut word = u16_to_word(0b0000_0000_0000_0000);
    if keystate.is_pressed("Digit0") {
        word = u16_to_word(0b0000_0000_0011_0000)
    } else if keystate.is_pressed("Digit1") {
        word = u16_to_word(0b0000_0000_0011_0001)
    } else if keystate.is_pressed("Digit2") {
        word = u16_to_word(0b0000_0000_0011_0010)
    } else if keystate.is_pressed("Digit3") {
        word = u16_to_word(0b0000_0000_0011_0011)
    } else if keystate.is_pressed("Digit4") {
        word = u16_to_word(0b0000_0000_0011_0100)
    } else if keystate.is_pressed("Digit5") {
        word = u16_to_word(0b0000_0000_0011_0101)
    } else if keystate.is_pressed("Digit6") {
        word = u16_to_word(0b0000_0000_0011_0110)
    } else if keystate.is_pressed("Digit7") {
        word = u16_to_word(0b0000_0000_0011_0111)
    } else if keystate.is_pressed("Digit8") {
        word = u16_to_word(0b0000_0000_0011_1000)
    } else if keystate.is_pressed("Digit9") {
        word = u16_to_word(0b0000_0000_0011_1001)
    } else if keystate.is_pressed("ArrowLeft") {
        word = u16_to_word(0b0000_0000_1000_0010)
    } else if keystate.is_pressed("ArrowUp") {
        word = u16_to_word(0b0000_0000_1000_0011)
    } else if keystate.is_pressed("ArrowRight") {
        word = u16_to_word(0b0000_0000_1000_0100)
    } else if keystate.is_pressed("ArrowDown") {
        word = u16_to_word(0b0000_0000_1000_0101)
    } else {
        let is_shift = keystate.is_pressed("ShiftLeft") || keystate.is_pressed("ShiftRight");
        if keystate.is_pressed("KeyA") && is_shift {
            word = u16_to_word(0b0000_0000_0100_0001)
        } else if keystate.is_pressed("KeyB") && is_shift {
            word = u16_to_word(0b0000_0000_0100_0010)
        } else if keystate.is_pressed("KeyC") && is_shift {
            word = u16_to_word(0b0000_0000_0100_0011)
        } else if keystate.is_pressed("KeyD") && is_shift {
            word = u16_to_word(0b0000_0000_0100_0100)
        } else if keystate.is_pressed("KeyE") && is_shift {
            word = u16_to_word(0b0000_0000_0100_0101)
        } else if keystate.is_pressed("KeyF") && is_shift {
            word = u16_to_word(0b0000_0000_0100_0110)
        } else if keystate.is_pressed("KeyG") && is_shift {
            word = u16_to_word(0b0000_0000_0100_0111)
        } else if keystate.is_pressed("KeyH") && is_shift {
            word = u16_to_word(0b0000_0000_0100_1000)
        } else if keystate.is_pressed("KeyI") && is_shift {
            word = u16_to_word(0b0000_0000_0100_1001)
        } else if keystate.is_pressed("KeyJ") && is_shift {
            word = u16_to_word(0b0000_0000_0100_1010)
        } else if keystate.is_pressed("KeyK") && is_shift {
            word = u16_to_word(0b0000_0000_0100_1011)
        } else if keystate.is_pressed("KeyL") && is_shift {
            word = u16_to_word(0b0000_0000_0100_1100)
        } else if keystate.is_pressed("KeyM") && is_shift {
            word = u16_to_word(0b0000_0000_0100_1101)
        } else if keystate.is_pressed("KeyN") && is_shift {
            word = u16_to_word(0b0000_0000_0100_1110)
        } else if keystate.is_pressed("KeyO") && is_shift {
            word = u16_to_word(0b0000_0000_0100_1111)
        } else if keystate.is_pressed("KeyP") && is_shift {
            word = u16_to_word(0b0000_0000_0101_0000)
        } else if keystate.is_pressed("KeyQ") && is_shift {
            word = u16_to_word(0b0000_0000_0101_0001)
        } else if keystate.is_pressed("KeyR") && is_shift {
            word = u16_to_word(0b0000_0000_0101_0010)
        } else if keystate.is_pressed("KeyS") && is_shift {
            word = u16_to_word(0b0000_0000_0101_0011)
        } else if keystate.is_pressed("KeyT") && is_shift {
            word = u16_to_word(0b0000_0000_0101_0100)
        } else if keystate.is_pressed("KeyU") && is_shift {
            word = u16_to_word(0b0000_0000_0101_0101)
        } else if keystate.is_pressed("KeyV") && is_shift {
            word = u16_to_word(0b0000_0000_0101_0110)
        } else if keystate.is_pressed("KeyW") && is_shift {
            word = u16_to_word(0b0000_0000_0101_0111)
        } else if keystate.is_pressed("KeyX") && is_shift {
            word = u16_to_word(0b0000_0000_0101_1000)
        } else if keystate.is_pressed("KeyY") && is_shift {
            word = u16_to_word(0b0000_0000_0101_1001)
        } else if keystate.is_pressed("KeyZ") && is_shift {
            word = u16_to_word(0b0000_0000_0101_1010)
        } else if keystate.is_pressed("KeyA") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_0001)
        } else if keystate.is_pressed("KeyB") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_0010)
        } else if keystate.is_pressed("KeyC") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_0011)
        } else if keystate.is_pressed("KeyD") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_0100)
        } else if keystate.is_pressed("KeyE") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_0101)
        } else if keystate.is_pressed("KeyF") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_0110)
        } else if keystate.is_pressed("KeyG") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_0111)
        } else if keystate.is_pressed("KeyH") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_1000)
        } else if keystate.is_pressed("KeyI") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_1001)
        } else if keystate.is_pressed("KeyJ") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_1010)
        } else if keystate.is_pressed("KeyK") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_1011)
        } else if keystate.is_pressed("KeyL") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_1100)
        } else if keystate.is_pressed("KeyM") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_1101)
        } else if keystate.is_pressed("KeyN") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_1110)
        } else if keystate.is_pressed("KeyO") && !is_shift {
            word = u16_to_word(0b0000_0000_0110_1111)
        } else if keystate.is_pressed("KeyP") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_0000)
        } else if keystate.is_pressed("KeyQ") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_0001)
        } else if keystate.is_pressed("KeyR") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_0010)
        } else if keystate.is_pressed("KeyS") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_0011)
        } else if keystate.is_pressed("KeyT") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_0100)
        } else if keystate.is_pressed("KeyU") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_0101)
        } else if keystate.is_pressed("KeyV") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_0110)
        } else if keystate.is_pressed("KeyW") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_0111)
        } else if keystate.is_pressed("KeyX") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_1000)
        } else if keystate.is_pressed("KeyY") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_1001)
        } else if keystate.is_pressed("KeyZ") && !is_shift {
            word = u16_to_word(0b0000_0000_0111_1010)
        }
    }
    word
}
