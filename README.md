# Exolvl

A library for reading and writing Exoracer level files.

## Todo

-   [ ] More documentation (see <https://github.com/skycloudd/exolvl/issues/1>)
-   [ ] Implement the `Default` trait for `Exolvl` (?)
-   [x] Use the `image` crate for images
-   [ ] Use the `glam` crate for `Vec2`
-   [ ] Use a `Duration` type for things like medal times
-   [ ] Use `Uuid` for UUIDs instead of `String`
-   [ ] Newtype pattern for ids
-   [ ] Make `theme` an enum, not a `String`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
exolvl = "0.6"
```

## License

Licensed under either of

-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
