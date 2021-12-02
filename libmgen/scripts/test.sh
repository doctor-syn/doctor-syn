#!/bin/bash
set -e -o pipefail

TEMPDIR=`mktemp -d`
clear
# cargo clean
cargo build --release
cargo new $TEMPDIR/test_libmgen
cp ../target/release/libmgen $TEMPDIR/test_libmgen

cd $TEMPDIR/test_libmgen
mkdir tests
# ./libmgen --generate-tests --num-bits 64 -f trig -o tests/trig64.rs
# ./libmgen --generate-tests --num-bits 32 -f trig -o tests/trig32.rs
./libmgen --generate-tests --num-bits 64 -f logexp -o tests/logexp64.rs
# ./libmgen --generate-tests --num-bits 32 -f logexp -o tests/logexp32.rs
echo $TEMPDIR/test_libmgen
cargo test
echo $TEMPDIR/test_libmgen

