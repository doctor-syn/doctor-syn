#!/bin/bash
set -e -o pipefail

DEST=$1

mkdir -p core_simd

clear
cargo build --release
../target/release/libmgen --num-bits 32 -f trig -o core_simd/trig32.rs
