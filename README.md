# `val`

[API docs](https://kodraus.github.io/val/val/index.html)

A lightweight serialization API for structured values sort of like [`ser`](https://github.com/KodrAus/ser) without baking in `serde` at all.

It's object-safe, but wraps that object-safe API up in a set of concrete structures that abstract over storage for the trait objects passed as arguments.

Producers of structured values use the `value` module. Consumers of structured values use the `visit` module.
