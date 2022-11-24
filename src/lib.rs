use js_sys::Reflect::get;
use js_sys::{Object, Uint8Array};
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
pub async fn fetch_and_compute_graph() -> Result<JsValue, JsValue> {
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
    let result_value = JsFuture::from(reader.read()).await?;
    let result: Object = result_value.dyn_into().unwrap();
    let chunk_value = get(&result, &JsValue::from_str("value")).unwrap();
    let chunk_array: Uint8Array = chunk_value.dyn_into().unwrap();
    let chunk = chunk_array.to_vec();
    log(&format!("Got chunk of length: {}", chunk.len()));

    Ok(JsValue::TRUE)
}
