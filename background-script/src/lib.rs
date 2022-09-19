use js_sys::Function;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_extensions_sys::*;

use common::Message::GetTabIdRequest;

#[wasm_bindgen]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let on_message = Closure::<dyn Fn(JsValue, MessageSender, Function)>::new(
        move |raw_message: JsValue, sender: MessageSender, send_response: Function| {
            let message: common::Message = serde_wasm_bindgen::from_value(raw_message).unwrap();
            match message {
                GetTabIdRequest => {
                    send_response.call1(&JsValue::null(), &JsValue::from(sender.tab().unwrap().id().unwrap()));
                }
            }
        }
    );
    chrome.runtime().on_message().add_listener(on_message.as_ref().unchecked_ref());
    on_message.forget();
}
