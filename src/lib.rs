use byteorder::{ByteOrder, LittleEndian};

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::window;
extern crate console_error_panic_hook;
use std::panic;

mod geometry;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn get_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub struct Graph {
    node_targets: Vec<Vec<usize>>,
    node_sources: Vec<Vec<usize>>,
    node_locations: Vec<Vec<f64>>,
    loading_node_index: usize,
}

#[wasm_bindgen]
impl Graph {
    pub fn new(
        node_count: usize,
        display_width: f64,
        display_height: f64,
        spawn_scale: f64,
    ) -> Graph {
        let node_targets = (0..node_count).map(|_| Vec::new()).collect();
        let node_sources = (0..node_count).map(|_| Vec::new()).collect();
        let spawn_height = display_height * spawn_scale;
        let spawn_width = display_width * spawn_scale;
        let node_locations = (0..node_count)
            .map(|_| geometry::random_location(spawn_width, spawn_height))
            .collect();
        Graph {
            node_targets,
            node_sources,
            node_locations,
            loading_node_index: 0,
        }
    }

    pub fn node_targets_count(&self, node_id: usize) -> usize {
        self.node_targets.get(node_id).unwrap().len()
    }

    pub fn node_targets_ptr(&self, node_id: usize) -> *const usize {
        self.node_targets.get(node_id).unwrap().as_ptr()
    }

    pub fn node_location_ptr(&self, node_id: usize) -> *const f64 {
        self.node_locations.get(node_id).unwrap().as_ptr()
    }

    pub async fn load_edges(&mut self, chunk_array: Uint8Array) {
        let chunk_buffer = chunk_array.to_vec();
        console_log!("Converted the stream data to a u8 vector!");
        let mut numbers = vec![0; chunk_buffer.len() / 2];
        console_log!("Instantiated a new u16 vector!");
        LittleEndian::read_u16_into(&chunk_buffer, &mut numbers);
        console_log!("Filled u16 vector from the buffer!");
        console_log!("Getting targets for node {}...", self.loading_node_index);
        for &num in numbers.iter() {
            // The MAX acts as a delimiter
            if num == u16::MAX {
                self.loading_node_index += 1;
            } else {
                let target_index = num as usize;
                self.node_targets
                    .get_mut(self.loading_node_index)
                    .unwrap()
                    .push(target_index);
                self.node_sources
                    .get_mut(target_index)
                    .unwrap()
                    .push(self.loading_node_index);
            }
        }
    }

    pub fn node_ids_to_render(&mut self, rect: geometry::Rect) -> Vec<usize> {
        let perf = window().unwrap().performance().unwrap();
        let start = perf.now();

        let mut indices = rustc_hash::FxHashSet::default();

        for (idx, loc) in self.node_locations.iter().enumerate() {
            if rect.contains(loc) {
                indices.insert(idx);
                for source_idx in self.node_sources.get(idx).unwrap().into_iter() {
                    indices.insert(*source_idx);
                }
                for target_idx in self.node_targets.get(idx).unwrap().into_iter() {
                    indices.insert(*target_idx);
                }
            }
        }
        let result = indices.into_iter().collect::<Vec<usize>>();

        let elapsed = perf.now() - start;
        console_log!("node_ids_to_render took {} ms", elapsed);

        result
    }
}
