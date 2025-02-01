//https://qiita.com/k-yaina60/items/19ee87d1eb740519c11a
//https://github.com/Rust-SDL2/rust-sdl2
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode, KeyboardState};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use std::time::{Duration};
use sdl2::render::WindowCanvas;

use hardware_simulator::Computer;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Nand2Tetris HardwareSimulator", 1024, 512)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

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
    let mut computer = Computer::new();
    computer.load_program(instructions);
    let mut reset = true;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let state = event_pump.keyboard_state();
        computer.step(reset, get_keyboard_press_code(&state));
        reset = false;

        display_screen(&mut canvas, &computer);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn display_screen(canvas: &mut WindowCanvas, computer: &Computer) {
    let screen = computer.get_screen();
    for px in 0..screen.len() {
        let x = px % 512;
        let y = px / 512;
        let color = if screen[px] { Color::RGB(0, 0, 0) } else { Color::RGB(255, 255, 255) };
        canvas.set_draw_color(color);
        let w = 2;
        canvas.fill_rect(Rect::new((x * w).try_into().unwrap(), (y * w).try_into().unwrap(), w as u32, w as u32)).unwrap();
    }
}

fn get_keyboard_press_code(keystate: &KeyboardState) -> u16 {
    if keystate.is_scancode_pressed(Scancode::Num0)  { 0b0000_0000_0011_0000 } else
    if keystate.is_scancode_pressed(Scancode::Num1)  { 0b0000_0000_0011_0001 } else
    if keystate.is_scancode_pressed(Scancode::Num2)  { 0b0000_0000_0011_0010 } else
    if keystate.is_scancode_pressed(Scancode::Num3)  { 0b0000_0000_0011_0011 } else
    if keystate.is_scancode_pressed(Scancode::Num4)  { 0b0000_0000_0011_0100 } else
    if keystate.is_scancode_pressed(Scancode::Num5)  { 0b0000_0000_0011_0101 } else
    if keystate.is_scancode_pressed(Scancode::Num6)  { 0b0000_0000_0011_0110 } else
    if keystate.is_scancode_pressed(Scancode::Num7)  { 0b0000_0000_0011_0111 } else
    if keystate.is_scancode_pressed(Scancode::Num8)  { 0b0000_0000_0011_1000 } else
    if keystate.is_scancode_pressed(Scancode::Num9)  { 0b0000_0000_0011_1001 } else
    if keystate.is_scancode_pressed(Scancode::Left)  { 0b0000_0000_1000_0010 } else
    if keystate.is_scancode_pressed(Scancode::Up)    { 0b0000_0000_1000_0011 } else
    if keystate.is_scancode_pressed(Scancode::Right) { 0b0000_0000_1000_0100 } else
    if keystate.is_scancode_pressed(Scancode::Down)  { 0b0000_0000_1000_0101 } else {
        let is_shift = keystate.is_scancode_pressed(Scancode::LShift) || keystate.is_scancode_pressed(Scancode::RShift);
        if is_shift {
            if keystate.is_scancode_pressed(Scancode::A) { 0b0000_0000_0100_0001 } else
            if keystate.is_scancode_pressed(Scancode::B) { 0b0000_0000_0100_0010 } else
            if keystate.is_scancode_pressed(Scancode::C) { 0b0000_0000_0100_0011 } else
            if keystate.is_scancode_pressed(Scancode::D) { 0b0000_0000_0100_0100 } else
            if keystate.is_scancode_pressed(Scancode::E) { 0b0000_0000_0100_0101 } else
            if keystate.is_scancode_pressed(Scancode::F) { 0b0000_0000_0100_0110 } else
            if keystate.is_scancode_pressed(Scancode::G) { 0b0000_0000_0100_0111 } else
            if keystate.is_scancode_pressed(Scancode::H) { 0b0000_0000_0100_1000 } else
            if keystate.is_scancode_pressed(Scancode::I) { 0b0000_0000_0100_1001 } else
            if keystate.is_scancode_pressed(Scancode::J) { 0b0000_0000_0100_1010 } else
            if keystate.is_scancode_pressed(Scancode::K) { 0b0000_0000_0100_1011 } else
            if keystate.is_scancode_pressed(Scancode::L) { 0b0000_0000_0100_1100 } else
            if keystate.is_scancode_pressed(Scancode::M) { 0b0000_0000_0100_1101 } else
            if keystate.is_scancode_pressed(Scancode::N) { 0b0000_0000_0100_1110 } else
            if keystate.is_scancode_pressed(Scancode::O) { 0b0000_0000_0100_1111 } else
            if keystate.is_scancode_pressed(Scancode::P) { 0b0000_0000_0101_0000 } else
            if keystate.is_scancode_pressed(Scancode::Q) { 0b0000_0000_0101_0001 } else
            if keystate.is_scancode_pressed(Scancode::R) { 0b0000_0000_0101_0010 } else
            if keystate.is_scancode_pressed(Scancode::S) { 0b0000_0000_0101_0011 } else
            if keystate.is_scancode_pressed(Scancode::T) { 0b0000_0000_0101_0100 } else
            if keystate.is_scancode_pressed(Scancode::U) { 0b0000_0000_0101_0101 } else
            if keystate.is_scancode_pressed(Scancode::V) { 0b0000_0000_0101_0110 } else
            if keystate.is_scancode_pressed(Scancode::W) { 0b0000_0000_0101_0111 } else
            if keystate.is_scancode_pressed(Scancode::X) { 0b0000_0000_0101_1000 } else
            if keystate.is_scancode_pressed(Scancode::Y) { 0b0000_0000_0101_1001 } else
            if keystate.is_scancode_pressed(Scancode::Z) { 0b0000_0000_0101_1010 } else
            { 0b0000_0000_0000_0000 }
        } else {
            if keystate.is_scancode_pressed(Scancode::A) { 0b0000_0000_0110_0001 } else
            if keystate.is_scancode_pressed(Scancode::B) { 0b0000_0000_0110_0010 } else
            if keystate.is_scancode_pressed(Scancode::C) { 0b0000_0000_0110_0011 } else
            if keystate.is_scancode_pressed(Scancode::D) { 0b0000_0000_0110_0100 } else
            if keystate.is_scancode_pressed(Scancode::E) { 0b0000_0000_0110_0101 } else
            if keystate.is_scancode_pressed(Scancode::F) { 0b0000_0000_0110_0110 } else
            if keystate.is_scancode_pressed(Scancode::G) { 0b0000_0000_0110_0111 } else
            if keystate.is_scancode_pressed(Scancode::H) { 0b0000_0000_0110_1000 } else
            if keystate.is_scancode_pressed(Scancode::I) { 0b0000_0000_0110_1001 } else
            if keystate.is_scancode_pressed(Scancode::K) { 0b0000_0000_0110_1011 } else
            if keystate.is_scancode_pressed(Scancode::J) { 0b0000_0000_0110_1010 } else
            if keystate.is_scancode_pressed(Scancode::L) { 0b0000_0000_0110_1100 } else
            if keystate.is_scancode_pressed(Scancode::M) { 0b0000_0000_0110_1101 } else
            if keystate.is_scancode_pressed(Scancode::N) { 0b0000_0000_0110_1110 } else
            if keystate.is_scancode_pressed(Scancode::O) { 0b0000_0000_0110_1111 } else
            if keystate.is_scancode_pressed(Scancode::P) { 0b0000_0000_0111_0000 } else
            if keystate.is_scancode_pressed(Scancode::Q) { 0b0000_0000_0111_0001 } else
            if keystate.is_scancode_pressed(Scancode::R) { 0b0000_0000_0111_0010 } else
            if keystate.is_scancode_pressed(Scancode::S) { 0b0000_0000_0111_0011 } else
            if keystate.is_scancode_pressed(Scancode::T) { 0b0000_0000_0111_0100 } else
            if keystate.is_scancode_pressed(Scancode::U) { 0b0000_0000_0111_0101 } else
            if keystate.is_scancode_pressed(Scancode::V) { 0b0000_0000_0111_0110 } else
            if keystate.is_scancode_pressed(Scancode::W) { 0b0000_0000_0111_0111 } else
            if keystate.is_scancode_pressed(Scancode::X) { 0b0000_0000_0111_1000 } else
            if keystate.is_scancode_pressed(Scancode::Y) { 0b0000_0000_0111_1001 } else
            if keystate.is_scancode_pressed(Scancode::Z) { 0b0000_0000_0111_1010 } else
            { 0b0000_0000_0000_0000 }
        }
    }
}
