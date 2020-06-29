use std::f32::consts::PI;

use log::{Level, info};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn wasm_start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("error initializing console_log");

    info!("WASM initialized.");
}

#[wasm_bindgen]
pub fn greet() {
    info!("Hello.");
}

#[wasm_bindgen]
pub struct Data {
    xs: Vec<f32>,
    ys: Vec<f32>,
}

const POINTS: usize = 100;
const RADIUS: f32 = 50.0;

#[wasm_bindgen]
impl Data {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut xs = vec![];
        let mut ys = vec![];
        let inc = (2.0*PI)/(POINTS as f32);
        for ix in 0..POINTS {
            let angle = (ix as f32) * inc;
            xs.push(RADIUS * angle.cos());
            ys.push(RADIUS * angle.sin());
        }
        Data { xs, ys }
    }

    pub fn count(&self) -> usize { self.xs.len() }
    pub fn xs(&self) -> *const f32 { self.xs.as_ptr() }
    pub fn ys(&self) -> *const f32 { self.ys.as_ptr() }
}