set -ex
# cargo +nightly build --release
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target no-modules --out-dir ./target ./target/wasm32-unknown-unknown/release/learn_worker.wasm