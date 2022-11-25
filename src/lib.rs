use byteorder::{ByteOrder, LittleEndian};
use js_sys::Reflect::get;
use js_sys::{Object, Uint8Array};
use std::collections::HashMap;
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
pub struct Graph {
    node_targets: HashMap<u16, Vec<u16>>,
}

#[wasm_bindgen]
impl Graph {
    pub fn new() -> Graph {
        let node_targets = HashMap::new();
        Graph { node_targets }
    }
    pub async fn load_edges(&mut self) -> Result<JsValue, JsValue> {
        let window = web_sys::window().unwrap();
        let resp_promise = window.fetch_with_str(&"./edges.bin");
        let resp_value = JsFuture::from(resp_promise).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        log(&format!("Response status code: {}", resp.status()));
        if resp.status() != 200 {
            return Err(JsValue::FALSE);
        }
        let reader_value = resp.body().unwrap().get_reader();
        let reader: ReadableStreamDefaultReader = reader_value.dyn_into().unwrap();

        let mut current_node_id: u16 = 1;
        self.node_targets.insert(current_node_id, Vec::new());
        log(&format!("Getting targets for node {}", current_node_id));
        loop {
            let result_value = JsFuture::from(reader.read()).await?;
            let result: Object = result_value.dyn_into().unwrap();
            log(&"Got an object from the stream!");
            let done_value = get(&result, &JsValue::from_str("done")).unwrap();
            log(&"Got a 'done' value from the object!");
            if done_value.as_bool().unwrap() {
                log(&"Done. Got all the chunks!");
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
            for &num in numbers.iter() {
                if num == u16::MAX {
                    // The MAX acts as a delimiter
                    current_node_id += 1;
                    self.node_targets.insert(current_node_id, Vec::new());
                    log(&format!("Getting targets for node {}", current_node_id));
                } else {
                    self.node_targets
                        .get_mut(&current_node_id)
                        .unwrap()
                        .push(num);
                }
            }
        }

        for (node_id, targets) in self.node_targets.iter() {
            log(&format!("Node {} has {} targets", node_id, targets.len()));
        }

        Ok(JsValue::TRUE)
    }
}
