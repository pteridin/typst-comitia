#/bin/zsh
cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/typst_comitia.wasm ./typst-package/comitia.wasm
typst compile ./typst-package/example.typ
mv ./typst-package/example.pdf ./typst-package/examples/