# `sval`

`sval` is a serialization API for Rust that streams the structure of data like a tokenizer.
`sval` is well suited to streaming self-describing text-based data formats like JSON.
`sval` is a lot like `serde::ser`, but with a few differences in design:

1. It uses a single `Stream` trait instead of `serde::Serializer` with its associated types.
2. All state is internal to the implementation of `Stream`, so providing an object-safe API doesn't
require an allocator.
3. The basic data-model is smaller than `serde`'s, but tags allow natural customization for things like
arbitrary-precision numbers and fixed-size arrays.

`sval` isn't intended as a successor project to `serde`. It fills a particular niche in the
landscape that can't be served by `serde` + `erased_serde`. In general, `sval`'s API is noisier
than `serde`s, since it's flat so the starts and ends of structural elements like map keys need
to be called out. Implementations of `Stream` need to do more work to track their state internally,
instead of relying on the callstack.

# Current status

This project has a complete and stable API, but isn't well documented yet.
