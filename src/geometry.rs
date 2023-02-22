use log::debug;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub const VALUES_PER_SQUARE: usize = 12;
pub const VALUES_PER_LINE: usize = 4;

/// Translates a point in graph layout space to display space
///
/// (Display space is clip space in terms of the graphics library.)
pub fn layout_to_clipspace(
    layout_location: Vector2,
    display_offset: Vector2,
    display_scale: f32,
    aspect_ratio: f32,
) -> Vector2 {
    let x = ((layout_location.x - display_offset.x) * display_scale) / aspect_ratio;
    let y = (layout_location.y - display_offset.y) * display_scale;
    Vector2::new(x, y)
}

pub struct Rect {
    bottom_left: Vector2,
    top_right: Vector2,
}

impl Rect {
    pub fn new(bottom_left: Vector2, top_right: Vector2) -> Rect {
        Rect {
            bottom_left,
            top_right,
        }
    }

    pub fn contains(&self, point: Vector2) -> bool {
        point.x > self.bottom_left.x
            && point.x < self.top_right.x
            && point.y > self.bottom_left.y
            && point.y < self.top_right.y
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

    pub fn get_data(&self) -> Vec<f32> {
        self.data.clone()
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
