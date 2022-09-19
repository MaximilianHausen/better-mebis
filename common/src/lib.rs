use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "message_type")]
pub enum Message {
    GetTabIdRequest,
}

pub mod utils {
    #[cfg(feature = "content_script")]
    pub mod content_script {
        use crate::Message;
        use web_extensions_sys::chrome;

        pub async fn get_tab_id() -> i32 {
            serde_wasm_bindgen::from_value(
                chrome.runtime()
                    .send_message(
                        None,
                        &serde_wasm_bindgen::to_value(&Message::GetTabIdRequest).unwrap(),
                        None,
                    ).await.unwrap()
            ).unwrap()
        }
    }
}
