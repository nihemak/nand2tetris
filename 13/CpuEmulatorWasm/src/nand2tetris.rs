use anyhow::Result;
use async_trait::async_trait;
use cpu_emulator::hardware::Computer;

use crate::{
    engine::{ComputerSystem, KeyState, Renderer},
};

#[derive(Clone)]
pub struct Nand2Tetris {
    computer: Box<Computer>,
}

impl Nand2Tetris {
    pub fn new() -> Self {
        let mut computer = Box::new(Computer::new());
        computer.load_program(Nand2Tetris::get_instructions());
        Nand2Tetris {
            computer: computer,
        }
    }

    fn get_instructions() -> Vec<&'static str> {
        // Fill
        vec![
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
        ]
    }

    fn get_keyboard_press_code(keystate: &KeyState) -> i16 {
        if keystate.is_pressed("Digit0")     { 0b0000_0000_0011_0000 } else 
        if keystate.is_pressed("Digit1")     { 0b0000_0000_0011_0001 } else 
        if keystate.is_pressed("Digit2")     { 0b0000_0000_0011_0010 } else 
        if keystate.is_pressed("Digit3")     { 0b0000_0000_0011_0011 } else 
        if keystate.is_pressed("Digit4")     { 0b0000_0000_0011_0100 } else 
        if keystate.is_pressed("Digit5")     { 0b0000_0000_0011_0101 } else 
        if keystate.is_pressed("Digit6")     { 0b0000_0000_0011_0110 } else 
        if keystate.is_pressed("Digit7")     { 0b0000_0000_0011_0111 } else 
        if keystate.is_pressed("Digit8")     { 0b0000_0000_0011_1000 } else 
        if keystate.is_pressed("Digit9")     { 0b0000_0000_0011_1001 } else 
        if keystate.is_pressed("ArrowLeft")  { 0b0000_0000_1000_0010 } else 
        if keystate.is_pressed("ArrowUp")    { 0b0000_0000_1000_0011 } else 
        if keystate.is_pressed("ArrowRight") { 0b0000_0000_1000_0100 } else 
        if keystate.is_pressed("ArrowDown")  { 0b0000_0000_1000_0101 } else {
            let is_shift = keystate.is_pressed("ShiftLeft") || keystate.is_pressed("ShiftRight");
            if is_shift {
                if keystate.is_pressed("KeyA") { 0b0000_0000_0100_0001 } else 
                if keystate.is_pressed("KeyB") { 0b0000_0000_0100_0010 } else 
                if keystate.is_pressed("KeyC") { 0b0000_0000_0100_0011 } else 
                if keystate.is_pressed("KeyD") { 0b0000_0000_0100_0100 } else 
                if keystate.is_pressed("KeyE") { 0b0000_0000_0100_0101 } else 
                if keystate.is_pressed("KeyF") { 0b0000_0000_0100_0110 } else 
                if keystate.is_pressed("KeyG") { 0b0000_0000_0100_0111 } else 
                if keystate.is_pressed("KeyH") { 0b0000_0000_0100_1000 } else 
                if keystate.is_pressed("KeyI") { 0b0000_0000_0100_1001 } else 
                if keystate.is_pressed("KeyJ") { 0b0000_0000_0100_1010 } else 
                if keystate.is_pressed("KeyK") { 0b0000_0000_0100_1011 } else 
                if keystate.is_pressed("KeyL") { 0b0000_0000_0100_1100 } else 
                if keystate.is_pressed("KeyM") { 0b0000_0000_0100_1101 } else 
                if keystate.is_pressed("KeyN") { 0b0000_0000_0100_1110 } else 
                if keystate.is_pressed("KeyO") { 0b0000_0000_0100_1111 } else 
                if keystate.is_pressed("KeyP") { 0b0000_0000_0101_0000 } else 
                if keystate.is_pressed("KeyQ") { 0b0000_0000_0101_0001 } else 
                if keystate.is_pressed("KeyR") { 0b0000_0000_0101_0010 } else 
                if keystate.is_pressed("KeyS") { 0b0000_0000_0101_0011 } else 
                if keystate.is_pressed("KeyT") { 0b0000_0000_0101_0100 } else 
                if keystate.is_pressed("KeyU") { 0b0000_0000_0101_0101 } else 
                if keystate.is_pressed("KeyV") { 0b0000_0000_0101_0110 } else 
                if keystate.is_pressed("KeyW") { 0b0000_0000_0101_0111 } else 
                if keystate.is_pressed("KeyX") { 0b0000_0000_0101_1000 } else 
                if keystate.is_pressed("KeyY") { 0b0000_0000_0101_1001 } else 
                if keystate.is_pressed("KeyZ") { 0b0000_0000_0101_1010 } else
                { 0b0000_0000_0000_0000 }
            } else {
                if keystate.is_pressed("KeyA") { 0b0000_0000_0110_0001 } else 
                if keystate.is_pressed("KeyB") { 0b0000_0000_0110_0010 } else 
                if keystate.is_pressed("KeyC") { 0b0000_0000_0110_0011 } else 
                if keystate.is_pressed("KeyD") { 0b0000_0000_0110_0100 } else 
                if keystate.is_pressed("KeyE") { 0b0000_0000_0110_0101 } else 
                if keystate.is_pressed("KeyF") { 0b0000_0000_0110_0110 } else 
                if keystate.is_pressed("KeyG") { 0b0000_0000_0110_0111 } else 
                if keystate.is_pressed("KeyH") { 0b0000_0000_0110_1000 } else 
                if keystate.is_pressed("KeyI") { 0b0000_0000_0110_1001 } else 
                if keystate.is_pressed("KeyJ") { 0b0000_0000_0110_1010 } else 
                if keystate.is_pressed("KeyK") { 0b0000_0000_0110_1011 } else 
                if keystate.is_pressed("KeyL") { 0b0000_0000_0110_1100 } else 
                if keystate.is_pressed("KeyM") { 0b0000_0000_0110_1101 } else 
                if keystate.is_pressed("KeyN") { 0b0000_0000_0110_1110 } else 
                if keystate.is_pressed("KeyO") { 0b0000_0000_0110_1111 } else 
                if keystate.is_pressed("KeyP") { 0b0000_0000_0111_0000 } else 
                if keystate.is_pressed("KeyQ") { 0b0000_0000_0111_0001 } else 
                if keystate.is_pressed("KeyR") { 0b0000_0000_0111_0010 } else 
                if keystate.is_pressed("KeyS") { 0b0000_0000_0111_0011 } else 
                if keystate.is_pressed("KeyT") { 0b0000_0000_0111_0100 } else 
                if keystate.is_pressed("KeyU") { 0b0000_0000_0111_0101 } else 
                if keystate.is_pressed("KeyV") { 0b0000_0000_0111_0110 } else 
                if keystate.is_pressed("KeyW") { 0b0000_0000_0111_0111 } else 
                if keystate.is_pressed("KeyX") { 0b0000_0000_0111_1000 } else 
                if keystate.is_pressed("KeyY") { 0b0000_0000_0111_1001 } else 
                if keystate.is_pressed("KeyZ") { 0b0000_0000_0111_1010 } else 
                { 0b0000_0000_0000_0000 }
            }
        }
    }
}

#[async_trait(?Send)]
impl ComputerSystem for Nand2Tetris {
    async fn initialize(&mut self, keystate: &KeyState) -> Result<Box<dyn ComputerSystem>> {
        let mut nand2tetris = Box::new(Nand2Tetris::new());
        nand2tetris.computer.step(true, Nand2Tetris::get_keyboard_press_code(keystate));
        Ok(nand2tetris)
    }

    fn update(&mut self, keystate: &KeyState) {
        self.computer.step(false, Nand2Tetris::get_keyboard_press_code(keystate));
    }

    fn draw(&mut self, renderer: &Renderer) {
        // let screen = self.computer.get_screen();
        // for px in 0..screen.len() {
        //     let x = px % 512;
        //     let y = px / 512;
        //     let color = if screen[px] {"#000000"} else {"#FFFFFF"};
        //     renderer.draw_pixel(color, x.try_into().unwrap(), y.try_into().unwrap());
        // }
        let pixels = self.computer.get_update_screen_pixels();
        for (x, y, color) in pixels {
            let color = if color {"#000000"} else {"#FFFFFF"};
            renderer.draw_pixel(color, x, y);
        }
    }
}
