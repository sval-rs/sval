#!/bin/bash

set -o errexit -o nounset

# `sval` builds
printf "\n\n---- sval ----\n\n"
cargo test

printf "\n\n---- sval with std ----\n\n"
cargo test --features std
cargo test --features "std fmt"
cargo test --features "std serde1"

printf "\n\n---- sval with alloc ----\n\n"
cargo test --lib --features alloc
cargo test --features "alloc fmt"
cargo test --features "alloc serde1"

printf "\n\n---- sval with fmt ----\n\n"
cargo test --features fmt

printf "\n\n---- sval with serde1 ----\n\n"
cargo test --features serde1

printf "\n\n---- sval with all features in release mode ----\n\n"
cargo test --all-features --release

# sval_json builds
pushd json
printf "\n\n---- sval_json ----\n\n"
cargo test

printf "\n\n---- sval_json with std ----\n\n"
cargo test --features std
popd

# other builds
# Benches are checked in the `nightly` build
# Format consistency is checked in the `beta` build
printf "\n\n---- integration tests ----\n\n"
cargo test --all --exclude sval_json_benches --exclude sval_fmt_tests

cargo test -p sval_serde_no_alloc_tests
