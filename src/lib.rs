use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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
    let reader_obj = resp.body().unwrap().get_reader();
    let stream_reader: ReadableStreamDefaultReader = reader_obj.dyn_into().unwrap();
    loop {
        let chunk_obj = JsFuture::from(stream_reader.read()).await?;
        let chunk_bytes: Uint8Array = chunk_obj.dyn_into().unwrap();
        log(&"Cast chunk obj to uint8Array");
        log(&format!(
            "Got a chunk of length {} from the promise!",
            chunk_bytes.length()
        ));
        //let bytes = serde_wasm_bindgen::from_value(js_bytes)?;
    }

    Ok(JsValue::TRUE)
    //iErr(JsValue::FALSE)
}
