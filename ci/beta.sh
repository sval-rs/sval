#!/bin/bash

set -o errexit -o nounset

./ci/test.sh

printf "\n\n---- format consistency ----\n\n"
cargo test -p sval_fmt_tests
