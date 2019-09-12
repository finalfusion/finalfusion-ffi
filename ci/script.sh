#!/bin/bash

set -euxo pipefail

# On Rust 1.31.0, we only care about passing tests.
if (rustc --version | grep -v "^rustc 1.31.0") && [[ "$TRAVIS_OS_NAME" != "osx" ]] ; then
  ( cd finalfusion-ffi ; cargo fmt --all -- --check )
  ( cd finalfusion-ffi ; cargo clippy -- -D warnings )
fi

mkdir build
cd build
cmake ..
make

# First run unit tests normally, to see if any test fails.
make test

# If the tests succeed, run them once more to see if there
# are any memory errors or leaks.
if [[ "$TRAVIS_OS_NAME" != "osx" ]] ; then
    ctest \
      --overwrite MemoryCheckCommandOptions="--leak-check=full --error-exitcode=1" \
      -T memcheck
fi
