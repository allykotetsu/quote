cargo build --target wasm32-unknown-unknown --release
mv target/wasm32-unknown-unknown/release/quote.wasm plugin.wasm
zip -q "quote.ocpkg" plugin.wasm plugin.manifest.json icon.png INSTRUCTIONS.md public public/*/*