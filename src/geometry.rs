use log::debug;
use wasm_bindgen::prelude::*;

pub fn random_location(scale_x: f32, scale_y: f32) -> Vec<f32> {
    let x_loc = js_sys::Math::random() as f32 * scale_x;
    let y_loc = js_sys::Math::random() as f32 * scale_y;
    vec![x_loc, y_loc]
}

/// Translates a point in graph layout space to display space
///
/// (Display space is clip space in terms of the graphics library.)
pub fn layout_to_display(
    layout_location: &Vec<f32>,
    display_offset: &Vec<f32>,
    display_scale: &f32,
    aspect_ratio: &f32,
) -> Vec<f32> {
    let x = ((layout_location[0] - display_offset[0]) * display_scale) / aspect_ratio;
    let y = (layout_location[1] - display_offset[1]) * display_scale;
    vec![x, y]
}

/// Get vertices for drawing a square centred on a point
///
/// The vertices are packed flat for the graphics library,
/// which draws the square using two triangles in clip space.
pub fn square_vertices(location: &Vec<f32>, aspect_ratio: &f32, edge_offset: &f32) -> Vec<f32> {
    let mut flat_vertices = vec![0.0; 12];

    let left_x = location[0] - (edge_offset / aspect_ratio);
    let right_x = location[0] + (edge_offset / aspect_ratio);
    let top_y = location[1] + edge_offset;
    let bottom_y = location[1] - edge_offset;

    flat_vertices[0] = left_x;
    flat_vertices[1] = top_y;

    flat_vertices[2] = right_x;
    flat_vertices[3] = top_y;

    flat_vertices[4] = right_x;
    flat_vertices[5] = bottom_y;

    flat_vertices[6] = right_x;
    flat_vertices[7] = bottom_y;

    flat_vertices[8] = left_x;
    flat_vertices[9] = bottom_y;

    flat_vertices[10] = left_x;
    flat_vertices[11] = top_y;

    flat_vertices
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
