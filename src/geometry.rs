use log::debug;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub const VALUES_PER_SQUARE: usize = 12;
pub const VALUES_PER_LINE: usize = 4;

// Optimised imperative code for computing clipspace vertices
//
// Returns the flattened triangles and lines for the graphics library.
pub fn populate_clipspace_vertices(
    flat_vertices: &mut Vec<f32>,
    node_locations: Points,
    node_target_ids: &Vec<Vec<usize>>,
    aspect_ratio: f32,
    square_edge_offset: f32,
) {
    let perf = window().unwrap().performance().unwrap();
    let node_count = node_locations.len();

    let mut start = perf.now();
    let mut node_index = 0;
    let mut square_index = 0;
    while node_index < node_count {
        let location = node_locations.get_point(node_index);

        let left_x = location.x - (square_edge_offset / aspect_ratio);
        let right_x = location.x + (square_edge_offset / aspect_ratio);
        let top_y = location.y + square_edge_offset;
        let bottom_y = location.y - square_edge_offset;

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

        square_index += VALUES_PER_SQUARE;
        node_index += 1;
    }
    let mut elapsed = perf.now() - start;
    debug!(
        "Setting note square triangles took {} ms. ({} ms per square)",
        elapsed,
        elapsed / node_count as f64
    );

    start = perf.now();
    let mut source_index = 0;
    let mut line_index = node_count * VALUES_PER_SQUARE;
    while source_index < node_count {
        let source_location = node_locations.get_point(source_index);
        let target_indices: &Vec<usize> = &node_target_ids[source_index];
        for target_index in target_indices {
            let target_location = node_locations.get_point(*target_index);

            flat_vertices[line_index] = source_location.x;
            flat_vertices[line_index + 1] = source_location.y;

            flat_vertices[line_index + 2] = target_location.x;
            flat_vertices[line_index + 3] = target_location.y;

            line_index += VALUES_PER_LINE;
        }
        source_index += 1;
    }
    elapsed = perf.now() - start;
    debug!(
        "Setting note edge lines took {} ms. ({} ms per node)",
        elapsed,
        elapsed / node_count as f64
    );
}

pub struct Rect {
    bottom_left: Vec<f32>,
    top_right: Vec<f32>,
}

impl Rect {
    pub fn new(bottom_left: Vec<f32>, top_right: Vec<f32>) -> Rect {
        Rect {
            bottom_left,
            top_right,
        }
    }

    pub fn contains(&self, point: &Vec<f32>) -> bool {
        point[0] > self.bottom_left[0]
            && point[0] < self.top_right[0]
            && point[1] > self.bottom_left[1]
            && point[1] < self.top_right[1]
    }
}

pub struct Points {
    data: Vec<f32>,
}

impl Points {
    pub fn new_random(count: usize, spawn_scale: f32) -> Points {
        let data: Vec<f32> = (0..(count * 2))
            .map(|_| js_sys::Math::random() as f32 * spawn_scale)
            .collect();
        Points { data }
    }
    pub fn len(&self) -> usize {
        self.data.len() / 2
    }

    pub fn get_point(&self, index: usize) -> Vector2 {
        let start_index = index * 2;
        Vector2 {
            x: self.data[start_index],
            y: self.data[start_index + 1],
        }
    }

    pub fn to_clipspace(
        &self,
        display_offset: Vector2,
        display_scale: &f32,
        aspect_ratio: &f32,
    ) -> Points {
        let mut clipspace_data: Vec<f32> = vec![0.0; self.data.len()];
        for index in (0..self.data.len()).step_by(2) {
            clipspace_data[index] =
                ((self.data[index] - display_offset.x) * display_scale) / aspect_ratio;
            clipspace_data[index + 1] = (self.data[index + 1] - display_offset.y) * display_scale;
        }
        Points {
            data: clipspace_data,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
}

#[cfg(test)]
mod tests;
