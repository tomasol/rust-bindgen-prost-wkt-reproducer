# Reproducer for wasm-bindgen failing with rustc 1.85

The issue is reproducable on the provided GitHub Action.

To reproduce locally, run
```sh
./repro.sh
```
Make sure you have rust 1.85 installed as well as `wasm-bindgen` version 0.2.100 .

With nix:
```sh
nix develop --command ./repro.sh
```
