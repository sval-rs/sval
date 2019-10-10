#!/bin/bash

set -o errexit -o nounset

./ci/test.sh

printf "\n\n---- benchmarks ----\n\n"
cargo bench --all --no-run
