# `val`

- [API docs for `val`](https://kodraus.github.io/val/val/index.html)
- [API docs for `val_serde`](https://kodraus.github.io/val/val_serde/index.html)

A proof-of-concept lightweight serialization API for structured log values. `val` is:

- serialization only, like `serde::ser`.
- no-std by default.
- object-safe.

The object-safe API is wrapped up in a set of concrete structures that abstract over storage for the trait objects passed as arguments and provides a nicer API to work with.

Producers of structured values use the `value` module. Consumers of structured values use the `visit` module.

Integration between `val` and `serde` is provided by the `val_serde` crate.

# `sval`

- [API docs for `sval`](https://kodraus.github.io/val/sval/index.html)

A proof-of-concept lightweight serialization API for structured log values.

`sval` is similar to `val` but doesn't require keys or values are known upfront, and so doesn't use the callstack to track where serialization is up to. Instead, it has a tiny, fixed-size stack internally that tracks maps and sequences, and ensures serialization is valid.

Producers of structured values use the `value` module. Consumers of structured values use the `stream` module.
