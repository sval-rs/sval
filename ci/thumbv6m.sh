#!/bin/bash

set -o errexit -o nounset

rustup target add thumbv6m-none-eabi

# `sval` builds
printf "\n\n---- sval ----\n\n"
cargo build --target=thumbv6m-none-eabi

printf "\n\n---- sval with fmt ----\n\n"
cargo build --target=thumbv6m-none-eabi --features fmt

# sval_json builds
pushd json
printf "\n\n---- sval_json ----\n\n"
cargo build --target=thumbv6m-none-eabi
popd
