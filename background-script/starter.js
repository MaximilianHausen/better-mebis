import wasm_bindgen from './background_script.js';

wasm_bindgen(
    chrome.runtime.getURL('background-script/background_script_bg.wasm')
).then(module => module.start())
.catch(console.error);
