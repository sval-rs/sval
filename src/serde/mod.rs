/*!
Integration between `sval` and `serde`.

Add the `serde` feature to your `Cargo.toml` to enable this module:

```toml,no_run
[dependencies.sval]
features = ["serde"]
```

See the [`v1`] module for more details.
*/

#[cfg(feature = "serde1_lib")]
pub mod v1;
