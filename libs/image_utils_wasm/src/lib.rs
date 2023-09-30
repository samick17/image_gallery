use wasm_bindgen::prelude::*;
// use image_utils::{resize_image};

#[wasm_bindgen]
extern {
    pub fn write_file(buffer: Vec<u8>, file_path: &str);
}

#[wasm_bindgen]
extern {
    pub fn read_file(file_path: &str) -> Vec<u8>;
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn my_rust_fn() {
    // TODO
    log("my_rust_fn called!");
    alert("Hello!!");
}
#[wasm_bindgen]
pub fn resize_image(file_path: &str, dest_file_path: &str, new_width: u32, new_height: u32) {
    let bytes = read_file(file_path);
    let out_bytes = image_utils::resize_image(bytes, dest_file_path, new_width, new_height);
    write_file(out_bytes, dest_file_path);
}
