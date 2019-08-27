#!/bin/bash

set -euxo pipefail

# On Rust 1.31.0, we only care about passing tests.
if rustc --version | grep -v "^rustc 1.31.0"; then
  ( cd finalfusion-ffi ; cargo fmt --all -- --check )
  ( cd finalfusion-ffi ; cargo clippy -- -D warnings )
fi

mkdir build
cd build
cmake ..
make
make test
