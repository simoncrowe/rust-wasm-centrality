use wasm_bindgen::prelude::*;

pub fn random_location(scale_x: f32, scale_y: f32) -> Vec<f32> {
    let x_loc = js_sys::Math::random() as f32 * scale_x;
    let y_loc = js_sys::Math::random() as f32 * scale_y;
    vec![x_loc, y_loc]
}

pub fn layout_to_display(
    layout_location: &Vec<f32>,
    display_offset: &Vec<f32>,
    display_scale: &f32,
    aspect_ratio: &f32,
) -> Vec<f32> {
    vec![0.0, 0.0]
}

#[wasm_bindgen]
pub struct Rect {
    bottom_left: Vec<f32>,
    top_right: Vec<f32>,
}

#[wasm_bindgen]
impl Rect {
    pub fn new(bottom_left: Vec<f32>, top_right: Vec<f32>) -> Rect {
        Rect {
            bottom_left,
            top_right,
        }
    }
}

impl Rect {
    pub fn contains(&self, point: &Vec<f32>) -> bool {
        point[0] > self.bottom_left[0]
            && point[0] < self.top_right[0]
            && point[1] > self.bottom_left[1]
            && point[1] < self.top_right[1]
    }
}

#[cfg(test)]
mod tests;
