#!/bin/bash
set -e -o pipefail

# Note this is a work in progress.

clear
cargo build
../target/debug/libmgen --num-bits 32 --language portable-simd -f trig,logexp,hyperbolic,hypot,cbrt -o ~/atomicincrement/stdsimd/crates/std_float/src/libm32.rs
rustfmt ~/atomicincrement/stdsimd/crates/std_float/src/libm32.rs
