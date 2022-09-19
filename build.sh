#!/bin/sh

rm -rf "./pkg"

wasm-pack build --out-dir '../../pkg/idp-theming' --target=no-modules content-scripts/idp-theming || exit 1
printf "wasm_bindgen(chrome.runtime.getURL('idp-theming/idp_theming_bg.wasm')).then(module => module.start()).catch(console.error);" >> ./pkg/idp-theming/starter.js

wasm-pack build --out-dir '../pkg/background-script' --target=web background-script || exit 1
printf "import wasm_bindgen from './background_script.js';\nwasm_bindgen(chrome.runtime.getURL('background-script/background_script_bg.wasm')).then(module => module.start()).catch(console.error);" >> ./pkg/background-script/starter.js

cp -r "./icons" "./pkg/icons"
cp manifest.json pkg/manifest.json
