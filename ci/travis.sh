#!/bin/bash

set -o errexit -o nounset

cargo test --verbose
cargo test --features std
cargo test --features serde
cargo test --all-features

pushd json
cargo build --verbose
cargo build --features std
popd

cargo test --all --exclude sval_json_benches
