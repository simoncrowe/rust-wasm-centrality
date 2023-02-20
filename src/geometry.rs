use log::debug;
use wasm_bindgen::prelude::*;

const NUMBERS_PER_SQUARE: usize = 12;
const NUMBERS_PER_LINE: usize = 4;

pub fn random_location(scale_x: f32, scale_y: f32) -> Vec<f32> {
    let x_loc = js_sys::Math::random() as f32 * scale_x;
    let y_loc = js_sys::Math::random() as f32 * scale_y;
    vec![x_loc, y_loc]
}

/// Translates a point in graph layout space to display space
///
/// (Display space is clip space in terms of the graphics library.)
pub fn layout_to_clipspace(
    layout_location: &Vec<f32>,
    display_offset: &Vec<f32>,
    display_scale: &f32,
    aspect_ratio: &f32,
) -> Vec<f32> {
    let x = ((layout_location[0] - display_offset[0]) * display_scale) / aspect_ratio;
    let y = (layout_location[1] - display_offset[1]) * display_scale;
    vec![x, y]
}

// Optimised imperative code for computing clipspace vertices
//
// Returns the flattened triangles and lines for the graphics library.
pub fn build_clipspace_vertices(
    node_locations: Vec<Vec<f32>>,
    node_target_ids: &Vec<Vec<usize>>,
    aspect_ratio: f32,
    square_edge_offset: f32,
) -> Vec<f32> {
    let node_count = node_locations.len();
    let edge_count: usize = node_target_ids.iter().map(|ids| ids.len()).sum();
    let number_count = (node_count * NUMBERS_PER_SQUARE) + (edge_count * NUMBERS_PER_LINE);
    let mut flat_vertices: Vec<f32> = vec![0.0; number_count];

    let mut node_index = 0;
    let mut square_index = 0;
    while node_index < node_count {
        let location: &Vec<f32> = &node_locations[node_index];

        let left_x = location[0] - (square_edge_offset / aspect_ratio);
        let right_x = location[0] + (square_edge_offset / aspect_ratio);
        let top_y = location[1] + square_edge_offset;
        let bottom_y = location[1] - square_edge_offset;

        flat_vertices[square_index] = left_x;
        flat_vertices[square_index + 1] = top_y;

        flat_vertices[square_index + 2] = right_x;
        flat_vertices[square_index + 3] = top_y;

        flat_vertices[square_index + 4] = right_x;
        flat_vertices[square_index + 5] = bottom_y;

        flat_vertices[square_index + 6] = right_x;
        flat_vertices[square_index + 7] = bottom_y;

        flat_vertices[square_index + 8] = left_x;
        flat_vertices[square_index + 9] = bottom_y;

        flat_vertices[square_index + 10] = left_x;
        flat_vertices[square_index + 11] = top_y;

        square_index += NUMBERS_PER_SQUARE;
        node_index += 1;
    }

    let mut source_index = 0;
    let mut line_index = node_count * NUMBERS_PER_SQUARE;
    while source_index < node_count {
        let source_location: &Vec<f32> = &node_locations[source_index];
        let target_indices: &Vec<usize> = &node_target_ids[source_index];
        for target_index in target_indices {
            let target_location: &Vec<f32> = &node_locations[*target_index];
            flat_vertices[line_index] = source_location[0];
            flat_vertices[line_index + 1] = source_location[1];
            flat_vertices[line_index + 2] = target_location[0];
            flat_vertices[line_index + 3] = target_location[1];
            line_index += NUMBERS_PER_LINE;
        }
        source_index += 1;
    }
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
