use std::{
    f32::consts::PI,
    ffi::c_void,
};

use log::{Level, info};
use memoffset::offset_of;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn wasm_start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("error initializing console_log");

    info!("WASM initialized.");
}

#[derive(Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn export() -> String {
        let mut out = String::new();
        out.push_str(r#"
        export class Point {
            private readonly _view: DataView;
            constructor(buffer: ArrayBuffer, arrayPtr: number, ix: number) {"#);
        out.push_str(&format!("\nconst size = {};\n", std::mem::size_of::<Point>()));
        out.push_str(r#"
                const offset = ix * size;
                this._view = new DataView(buffer, arrayPtr + offset, size);
            }
            get x(): number {"#);
        out.push_str(&format!("\nconst offset = {};\n", offset_of!(Point, x)));
        out.push_str(r#"
                return this._view.getFloat32(offset, true);
            }
            get y(): number {"#);
        out.push_str(&format!("\nconst offset = {};\n", offset_of!(Point, y)));
        out.push_str(r#"
                return this._view.getFloat32(offset, true);
            }
        }
        "#);
        out
    }
}

#[wasm_bindgen]
pub struct Data {
    points: Vec<Point>
}

const POINTS: usize = 100;
const RADIUS: f32 = 200.0;
const SPEED: f32 = 0.0001;

#[wasm_bindgen]
impl Data {
    #[wasm_bindgen(constructor)]
    pub fn new(time: f32) -> Self {
        let mut out = Data { points: vec![Point { x: 0.0, y: 0.0 }; POINTS] };
        out.update(time);
        out
    }

    pub fn count(&self) -> usize { self.points.len() }
    pub fn points(&self) -> *const c_void { self.points.as_ptr() as *const c_void }

    pub fn update(&mut self, time: f32) {
        let inc = (2.0*PI)/(POINTS as f32);
        for (ix, pt) in self.points.iter_mut().enumerate() {
            let angle = (ix as f32) * inc + (time*SPEED);
            pt.x = RADIUS * angle.cos();
            pt.y = RADIUS * angle.sin();
        }
    }
}