#!/bin/bash
TEMPDIR=`mktemp -d`
# cargo clean
cargo build --release
cargo new $TEMPDIR/test_libmgen
cp ../target/release/libmgen $TEMPDIR/test_libmgen

cd $TEMPDIR/test_libmgen
mkdir tests
./libmgen --generate-tests --num-bits 64 -f trig -o tests/trig64.rs
./libmgen --generate-tests --num-bits 32 -f trig -o tests/trig32.rs
cargo test
