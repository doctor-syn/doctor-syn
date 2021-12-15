#!/bin/bash
set -e -o pipefail

#TEMPDIR=`mktemp -d`
TEMPDIR=/tmp

clear
# cargo clean
cargo build --release
rm -rf $TEMPDIR/test_libmgen
cargo new $TEMPDIR/test_libmgen
cp ../target/release/libmgen $TEMPDIR/test_libmgen

cd $TEMPDIR/test_libmgen
mkdir tests
./libmgen --num-bits 32 -f exp -o tests/exp32.rs --language "rust"
./libmgen --num-bits 32 -f exp -o tests/exp32.c --language "c"
# ./libmgen --generate-tests --num-bits 64 -f trig -o tests/trig64.rs
# ./libmgen --generate-tests --num-bits 32 -f trig -o tests/trig32.rs
# ./libmgen --generate-tests --num-bits 64 -f logexp -o tests/logexp64.rs
# ./libmgen --generate-tests --num-bits 32 -f logexp -o tests/logexp32.rs
# ./libmgen --generate-tests --num-bits 64 -f normal -o tests/normal64.rs
# ./libmgen --generate-tests --num-bits 64 --language c -f normal -o tests/normal.c
echo $TEMPDIR/test_libmgen
cargo test
echo $TEMPDIR/test_libmgen

