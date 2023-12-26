use super::geometry;
use js_sys;
use log::debug;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TouchSet {
    data: HashMap<i32, geometry::Vector2>,
}

#[wasm_bindgen]
impl TouchSet {
    pub fn new(locs: js_sys::Float32Array, ids: js_sys::Int32Array) -> TouchSet {
        let loc_data = locs.to_vec();
        let id_data = ids.to_vec();
        let mut data: HashMap<i32, geometry::Vector2> = HashMap::new();
        for (idx, loc) in loc_data.as_slice().chunks(2).enumerate() {
            let id = id_data[idx];
            let x = loc[0];
            let y = loc[1];
            data.insert(id, geometry::Vector2 { x, y });
        }
        TouchSet { data }
    }
}

fn id_intersection(first: &TouchSet, second: &TouchSet) -> HashSet<i32> {
    let first_keys: HashSet<i32> = first.data.keys().cloned().collect();
    let second_keys: HashSet<i32> = second.data.keys().cloned().collect();
    first_keys.intersection(&second_keys).copied().collect()
}

pub fn touch_scale(pair: &[TouchSet]) -> f32 {
    0.0
}

pub fn touch_offset(pair: &[TouchSet]) -> geometry::Vector2 {
    let first_set = &pair[0];
    let second_set = &pair[1];
    let common_ids = id_intersection(first_set, second_set);
    let mut offset = geometry::Vector2 { x: 0.0, y: 0.0 };
    let mut count: f32 = 0.0;
    for id in common_ids.iter() {
        count += 1.0;
        let first = first_set
            .data
            .get(id)
            .expect("First key set should contain key in intersection");
        let second = second_set
            .data
            .get(id)
            .expect("Second key set should contain key in intersection");
        offset += *first - *second
    }
    offset /= count;
    offset
}
