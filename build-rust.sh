#/usr/bin/env -S bash

cargo build 

ln -sf target/debug/rust-serde-json ./rust-serde-json-debug
ln -sf target/debug/rust-simd-json ./rust-simd-json
