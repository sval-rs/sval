#!/bin/bash

set -o errexit -o nounset

cargo test --verbose
cargo test --all-features
cargo test --all --exclude sval_json_benches
