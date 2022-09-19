use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let tab_id = common::utils::content_script::get_tab_id().await;
    gloo_console::info!("Tab id: ", tab_id);
}
