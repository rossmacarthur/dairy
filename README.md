# dairy

[![Crates.io Version](https://img.shields.io/crates/v/dairy.svg)](https://crates.io/crates/dairy)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/dairy)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/dairy/build/trunk)](https://github.com/rossmacarthur/dairy/actions?query=workflow%3Abuild)

A more compact, user friendly clone-on-write smart pointer.

```rust
use dairy::Cow;
let borrowed: Cow<str> = Cow::borrowed("Hello World!");
let owned: Cow<str> = Cow::owned(String::from("Hello World!"));
```

## 🚀 Getting started

Add the following to your Cargo manifest.

```toml
[dependencies]
dairy = "0.2"
```

`no_std` is also supported by disabling the default `std` feature. An allocator
is required.

```toml
[dependencies]
dairy = { version = "0.2", default-features = false }
```

[Serde](https://serde.rs) is supported behind the `serde` feature.

```toml
[dependencies]
dairy = { version = "0.2", features = ["serde"] }
```

## 🤔 Description

`dairy::Cow` is an improved version of the standard library `std::borrow::Cow`.
Depending on the platform and type this crate transparently provides a better
underlying implementation which will be more compact. This crate currently
supports the following types: `str`, `[T]` `CStr`, `OsStr`, and `Path`.

`dairy::Cow` is also able to provide many more `From` implementations; some
which are not possible for the standard library to provide due to the `alloc`,
`std` split. For example `Cow<Path>` now has the useful `From<&str>`
implementation.

### Underlying implementation

- On 64-bit platforms the compact implementation of `Cow` is two words wide,
  storing the length, capacity, and the ownership tag in the same word.
- On 32-bit platforms the compact implementation of `Cow` is three words wide,
  storing the capacity and the ownership tag in the same word.
- The default implementation simply used the the standard library implementation
  which is four words wide. This is typically required in cases where the
  standard library does not provide a `.into_raw_parts()` or equivalent method
  for types. The following table documents how `Cow<T>` is implemented for each
  type on Unix and Windows.

| `T`     | cfg(unix) | cfg(windows) |
| ------- | --------- | ------------ |
| `str`   | *compact* | *compact*    |
| `[T]`   | *compact* | *compact*    |
| `CStr`  | *compact* | *compact*    |
| `OsStr` | *compact* | **default**  |
| `Path`  | *compact* | **default**  |

## Acknowledgements

Some implementation details taken from the excellent
[beef](https://github.com/maciejhirsz/beef) crate.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
