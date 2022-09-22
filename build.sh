#!/bin/sh

rm -rf "./pkg"

wasm-pack build --out-dir '../../pkg/idp-theming' --target=no-modules content-scripts/idp-theming || exit 1
cp ./content-scripts/idp-theming/starter.js ./pkg/idp-theming/starter.js
cp ./content-scripts/idp-theming/style.css ./pkg/idp-theming/style.css

wasm-pack build --out-dir '../pkg/background-script' --target=web background-script || exit 1
cp ./background-script/starter.js ./pkg/background-script/starter.js

cp -r "./icons" "./pkg/icons"
cp manifest.json pkg/manifest.json
