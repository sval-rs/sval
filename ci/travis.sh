#!/bin/bash

set -o errexit -o nounset

# `sval` builds
printf "\n\n---- sval ----\n\n"
cargo test

printf "\n\n---- sval with std ----\n\n"
cargo test --features std

printf "\n\n---- sval with serde ----\n\n"
cargo test --features serde

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
printf "\n\n---- integration tests ----\n\n"
cargo test --all --exclude sval_json_benches
