# dairy

A more compact, user friendly clone-on-write smart pointer.

```rust
use dairy::Cow;

let borrowed: Cow<str> = Cow::borrowed("Hello World!");
let owned: Cow<str> = Cow::owned(String::from("Hello World!"));
```

`dairy::Cow` is an improved version of the standard library `std::borrow::Cow`.
It is just 2 words wide, storing the length, capacity, and the ownership tag all
in one word. `dairy::Cow` is able to provide many more `From` implementations;
some which are not possible for the standard library to provide due to the
`alloc`, `std` split. Most notably `Cow<Path>` has the useful `From<&str>`
implementation.

Unfortunately these benefits come with some caveats:

- Only `str`, `[T]`, `CStr`, `OsStr`, and `Path` types are supported. And
  `OsStr` and `Path` are only supported on Unix (`unix` feature).
- On 32-bit operating systems the maximum length is `u16::MAX` which might not
  be sufficient for all use cases.

## Getting started

Add the following to your Cargo manifest.

```toml
[dependencies]
dairy = { version = "0.1", features = ["unix", "serde"] }
```

`no_std` is also supported by disabling the default `std` feature.

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
