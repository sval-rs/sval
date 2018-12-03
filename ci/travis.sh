#!/bin/bash

set -o errexit -o nounset

# `sval` builds
cargo test
cargo test --features std
cargo test --features serde
cargo test --all-features --release

# `sval_json` builds
pushd json
cargo build
cargo build --features std
popd

# other builds
cargo test --all --exclude sval_json_benches
