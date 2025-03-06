#!/usr/bin/env bash

cargo build --target=wasm32-unknown-unknown
RUST_BACKTRACE=1 wasm-bindgen --target=web --out-dir=target/wasm-bindgen/debug --out-name=rust-bindgen-prost-wkt-reproducer target/wasm32-unknown-unknown/debug/rust-bindgen-prost-wkt-reproducer.wasm --no-typescript
