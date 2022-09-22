wasm_bindgen(
    chrome.runtime.getURL('idp-theming/idp_theming_bg.wasm')
).then(module => module.start())
.catch(console.error);
