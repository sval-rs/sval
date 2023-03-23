# `sval`: Streaming, structured values

`sval` is a lightweight serialization-only framework that treats values like a flat stream of tokens.
It's well suited to self-describing text formats like JSON.

## How is this different from `serde`?

`serde` is the de-facto serialization framework for Rust and is well suited to the majority of
use cases. `sval` is like a light blend of `serde::ser` and `serde::de` that is smaller in scope.
It makes a few key different design decisions than `serde` that make it effective for working with
self-describing formats:

1. The API is flat rather than using recursion to stream nested datastructures.
2. All values with dynamic sizes, including text strings, can be streamed in multiple calls.
3. Borrowing is an optional optimization.
4. The data model isn't a one-to-one mapping of Rust's own.

## Current status

This project has a complete and stable API, but isn't well documented yet.
