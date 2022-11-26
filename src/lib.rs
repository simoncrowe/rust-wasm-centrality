use byteorder::{ByteOrder, LittleEndian};
use js_sys::Reflect::get;
use js_sys::{Object, Uint8Array};
use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{ReadableStreamDefaultReader, Response};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub struct Graph {
    node_targets: Vec<Vec<usize>>,
    node_locations: Vec<Vec<f64>>,
}

#[wasm_bindgen]
impl Graph {
    pub fn new() -> Graph {
        let node_targets = Vec::new();
        let node_locations = Vec::new();
        Graph {
            node_targets,
            node_locations,
        }
    }

    pub fn node_count(&self) -> usize {
        // TODO: delete if unused
        cmp::min(self.node_targets.len(), self.node_locations.len())
    }

    pub fn node_targets_count(&self, node_id: usize) -> usize {
        self.node_targets.get(node_id - 1).unwrap().len()
    }

    pub fn node_targets_ptr(&self, node_id: usize) -> *const usize {
        self.node_targets.get(node_id - 1).unwrap().as_ptr()
    }

    pub fn node_location_ptr(&self, node_id: usize) -> *const f64 {
        self.node_locations.get(node_id - 1).unwrap().as_ptr()
    }

    pub fn node_ids_to_render(&self, rect: Rect) -> Vec<usize> {
        let contained_indices = self
            .node_locations
            .iter()
            .enumerate()
            .filter(|(_idx, loc)| rect.contains(loc))
            .map(|(idx, _loc)| idx);

        let neighbouring_indices = contained_indices
            .clone()
            .map(|idx| self.neighbours(idx))
            .flatten();

        let mut result = contained_indices
            .chain(neighbouring_indices)
            .map(|idx| idx + 1)
            .collect::<Vec<usize>>();

        result.sort();
        result.dedup();
        result
    }

    pub async fn load_edges(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let resp_promise = window.fetch_with_str(&"./targets.bin");
        let resp_value = JsFuture::from(resp_promise).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        log(&format!("Response status code: {}", resp.status()));
        if resp.status() != 200 {
            let msg = format!(
                "Bad status code when getting targets binary: {}",
                resp.status()
            );
            return Err(JsValue::from(msg));
        }
        let reader_value = resp.body().unwrap().get_reader();
        let reader: ReadableStreamDefaultReader = reader_value.dyn_into().unwrap();

        self.node_locations.push(random_location(1024.0));
        self.node_targets.push(Vec::new());
        let mut current_node_index: usize = 0;
        log(&format!("Getting targets for node {}", current_node_index));
        loop {
            let result_value = JsFuture::from(reader.read()).await?;
            let result: Object = result_value.dyn_into().unwrap();
            log(&"Got an object from the stream!");
            let done_value = get(&result, &JsValue::from_str("done")).unwrap();
            log(&"Got a 'done' value from the object!");
            if done_value.as_bool().unwrap() {
                log(&format!(
                    "Done. Loaded targets for {} nodes",
                    current_node_index - 1
                ));
                break;
            }
            let chunk_value = get(&result, &JsValue::from_str("value")).unwrap();
            log(&"Got a value for the stream data!");
            let chunk_array: Uint8Array = chunk_value.dyn_into().unwrap();
            log(&"Cast the stream data as an array!");
            let chunk_buffer = chunk_array.to_vec();
            log(&"Converted the stream data to a u8 vector!");
            let mut numbers = vec![0; chunk_buffer.len() / 2];
            log(&"Instantiated a new u16 vector!");
            LittleEndian::read_u16_into(&chunk_buffer, &mut numbers);
            log(&"Filled u16 vector from the buffer!");
            log(&format!(
                "Getting targets for node {}...",
                current_node_index
            ));
            for &num in numbers.iter() {
                // The MAX acts as a delimiter
                if num == u16::MAX {
                    self.node_locations.push(random_location(1024.0));
                    self.node_targets.push(Vec::new());
                    current_node_index += 1;
                } else {
                    self.node_targets
                        .get_mut(current_node_index)
                        .unwrap()
                        .push(num as usize);
                }
            }
        }

        Ok(())
    }
}

impl Graph {
    fn neighbours(&self, node_index: usize) -> Vec<usize> {
        let mut neighbours: Vec<usize> = self
            .node_targets
            .iter()
            .enumerate()
            .filter(|(_idx, targets)| targets.contains(&node_index))
            .map(|(idx, _targets)| idx)
            .collect();
        neighbours.extend(self.node_targets.get(node_index).unwrap());
        neighbours
    }
}

fn random_location(scale: f64) -> Vec<f64> {
    let x_loc = js_sys::Math::random() * scale;
    let y_loc = js_sys::Math::random() * scale;
    vec![x_loc, y_loc]
}

#[wasm_bindgen]
pub struct Rect {
    bottom_left: Vec<f64>,
    top_right: Vec<f64>,
}

#[wasm_bindgen]
impl Rect {
    pub fn new(bottom_left: Vec<f64>, top_right: Vec<f64>) -> Rect {
        Rect {
            bottom_left,
            top_right,
        }
    }
}

impl Rect {
    fn contains(&self, point: &Vec<f64>) -> bool {
        point[0] > self.bottom_left[0]
            && point[0] < self.top_right[0]
            && point[1] > self.bottom_left[1]
            && point[1] < self.top_right[1]
    }
}
