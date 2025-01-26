#[macro_use]
mod browser;
mod engine;
mod nand2tetris;

use engine::ComputerSystemLoop;
use nand2tetris::Nand2Tetris;
use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    browser::spawn_local(async move {
        let nand2tetris = Nand2Tetris::new();

        ComputerSystemLoop::start(nand2tetris)
            .await
            .expect("Could not start computer system loop");
    });

    Ok(())
}
