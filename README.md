# `val`

- [API docs for `val`](https://kodraus.github.io/val/val/index.html)
- [API docs for `val_serde`](https://kodraus.github.io/val/val_serde/index.html)

A lightweight serialization API for structured values with `serde` integration through a separate crate. `val` is no-std compatible and object-safe, but wraps that object-safe API up in a set of concrete structures that abstract over storage for the trait objects passed as arguments and provides a nicer API to work with.

Producers of structured values use the `value` module. Consumers of structured values use the `visit` module.
