use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn make_download(name: &str, bytes: &[u8], mime: &str);
}
