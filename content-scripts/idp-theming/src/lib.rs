use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    web_sys::console::log_1(&JsValue::from_str("Hello World!"));
}
