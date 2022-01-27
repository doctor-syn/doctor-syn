#!/bin/bash
set -e -o pipefail

# Note this is a work in progress.

clear
cargo build --release
../target/release/libmgen --num-bits 32 --language portable-simd -f trig -x select -o ~/atomicincrement/stdsimd/crates/std_float/src/libm32.rs
rustfmt ~/atomicincrement/stdsimd/crates/std_float/src/libm32.rs
