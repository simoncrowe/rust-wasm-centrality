use super::geometry;
use js_sys;
use wasm_bindgen::prelude::*;

struct Touch {
    loc: geometry::Vector2,
    id: u32,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TouchSet {
    loc_data: Vec<f32>,
    id_data: Vec<i32>,
}

#[wasm_bindgen]
impl TouchSet {
    pub fn new(locs: js_sys::Float32Array, ids: js_sys::Int32Array) -> TouchSet {
        let loc_data = locs.to_vec();
        let id_data = ids.to_vec();
        TouchSet { loc_data, id_data }
    }
}

pub fn touch_scale(pair: &[TouchSet]) -> f32 {
    0.0
}

pub fn touch_offset(pair: &[TouchSet]) -> geometry::Vector2 {
    geometry::Vector2 { x: 0.0, y: 0.0 }
}
