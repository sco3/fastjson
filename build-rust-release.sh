#/usr/bin/env -S bash 

cargo build --release 

ln -sf target/debug/rust-serde-json ./rust-serde-json-release 
ln -sf target/debug/rust-simd-json ./rust-simd-json-release
