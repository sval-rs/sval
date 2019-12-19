#!/bin/bash

set -o errexit -o nounset

rustup target add thumbv6m-none-eabi

# `sval` builds
printf "\n\n---- sval ----\n\n"
cargo build --target=thumbv6m-none-eabi

printf "\n\n---- sval with arbitrary-depth ----\n\n"
cargo build --target=thumbv6m-none-eabi --features arbitrary-depth

printf "\n\n---- sval with alloc ----\n\n"
cargo build --target=thumbv6m-none-eabi --features alloc

printf "\n\n---- sval with fmt ----\n\n"
cargo build --target=thumbv6m-none-eabi --features fmt

printf "\n\n---- sval with serde ----\n\n"
cargo build --target=thumbv6m-none-eabi --features serde

printf "\n\n---- sval with serde and alloc ----\n\n"
cargo build --target=thumbv6m-none-eabi --features "serde alloc"

# sval_json builds
pushd json
printf "\n\n---- sval_json ----\n\n"
cargo build --target=thumbv6m-none-eabi
popd
