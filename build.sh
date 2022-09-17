#!/bin/sh

rm -rf "./pkg"

wasm-pack build --out-dir '../../pkg/idp-theming' --target=no-modules content-scripts/idp-theming || exit 1
printf "wasm_bindgen(chrome.runtime.getURL('idp-theming/idp_theming_bg.wasm')).then(module => module.main()).catch(console.error);" >> ./pkg/idp-theming/starter.js

cp -r "./icons" "./pkg/icons"
cp manifest.json pkg/manifest.json
