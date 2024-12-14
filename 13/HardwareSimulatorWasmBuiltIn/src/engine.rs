use crate::browser::{self, LoopClosure};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::{
    mpsc::{unbounded, UnboundedReceiver},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{JsCast};
use web_sys::{CanvasRenderingContext2d};

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn draw_pixel(
        &self,
        color: &str,
        x: i32, // 点の位置（x座標）
        y: i32  // 点の位置（y座標）
    ) {
        let w = 2.0;    // 点の横幅
        let h = 2.0;    // 点の高さ
        self.context.set_fill_style(&color.into());
        self.context.fill_rect(f64::from(x*2), f64::from(y*2), w, h);
    }
}

#[async_trait(?Send)]
pub trait ComputerSystem {
    async fn initialize(&mut self, renderer: &Renderer, keystate: &KeyState) -> Result<Box<dyn ComputerSystem>>;
    fn update(&mut self, keystate: &KeyState);
    fn draw(&self, renderer: &Renderer);
}

// Sixty Frames per second, converted to a frame length in milliseconds
const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;
pub struct ComputerSystemLoop {
    last_frame: f64,
    accumulated_delta: f32,
}
type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl ComputerSystemLoop {
    pub async fn start(mut computer_system: impl ComputerSystem + 'static) -> Result<()> {
        let mut keyevent_receiver = prepare_input()?;

        let mut computer_system_loop = ComputerSystemLoop {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };

        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut keystate = KeyState::new();

        let mut computer_system = computer_system.initialize(&renderer, &keystate).await?;
        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            process_input(&mut keystate, &mut keyevent_receiver);

            computer_system_loop.accumulated_delta += (perf - computer_system_loop.last_frame) as f32;
            //log!("start {}", computer_system_loop.accumulated_delta.to_string());
            while computer_system_loop.accumulated_delta > FRAME_SIZE {
                //log!("in {}", computer_system_loop.accumulated_delta.to_string());
                computer_system.update(&keystate);
                //log!("in2 {}", computer_system_loop.accumulated_delta.to_string());
                computer_system_loop.accumulated_delta -= FRAME_SIZE;
            }
            //log!("end {}", computer_system_loop.accumulated_delta.to_string());
            computer_system_loop.last_frame = perf;
            computer_system.draw(&renderer);

            browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("ComputerSystemLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}

impl KeyState {
    fn new() -> Self {
        return KeyState {
            pressed_keys: HashMap::new(),
        };
    }

    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);
    }

    fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code.into());
    }
}

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
    loop {
        match keyevent_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
                KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt),
            },
        };
    }
}

fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);
    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::canvas()?.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    browser::canvas()?.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_receiver)
}
