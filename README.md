# dairy

A more compact, user friendly clone-on-write smart pointer.

```rust
use dairy::Cow;
let borrowed: Cow<str> = Cow::borrowed("Hello World!");
let owned: Cow<str> = Cow::owned(String::from("Hello World!"));
```

`dairy::Cow` is an improved version of the standard library `std::borrow::Cow`.
On 64-bit Unix platforms it is just 2 words wide, storing the length, capacity,
and the ownership tag all in one word! On 32-bit Unix platforms it is 3 words
wide, storing the capacity and the ownership tag in the same word. On non-Unix
platforms it falls back to the standard library implementation which is 4 words
wide.

`dairy::Cow` is also able to provide many more `From` implementations; some
which are not possible for the standard library to provide due to the `core`,
`alloc`, and `std` split. Most notably `Cow<Path>` has the useful `From<&str>`
implementation.

## Getting started

Add the following to your Cargo manifest.

```toml
[dependencies]
dairy = "0.1"
```

`no_std` is also supported by disabling the default `std` feature. An allocator
is required.

```toml
[dependencies]
dairy = { version = "0.1", default-features = false }
```

## Acknowledgements

Some implementation details taken from the excellent
[beef](https://github.com/maciejhirsz/beef) crate.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
