# dairy

A more compact, user friendly clone-on-write smart pointer.

```rust
use std::path::Path;
use dairy::Cow;

let borrowed: Cow<str> = Cow::borrowed("Hello World!");
let owned: Cow<str> = Cow::owned(String::from("Hello World!"));
```

## Introduction

`dairy::Cow` is an improved version of the standard library `std::borrow::Cow`.
It is just 2 words wide, storing the length, capacity, and the ownership tag all
in one word. See [tests/size.rs](tests/size.rs).

`dairy::Cow` has many more `From` and `PartialEq` implementations. Most notably
for `Cow<Path>` making `Into<Cow<Path>>` just as nice to use as `Cow<str>`.

Unfortunately these benefits come with some caveats:

- Only `str`, `[T]`, `OsStr`, `CStr` and `Path` types are supported.
- Additionally, `OsStr` and `Path` are only supported on Unix.
- On 32-bit operating systems the maximum length is `u16::MAX` which is
  not sufficient for all use cases.

## Getting started

Add the following to your Cargo manifest.

```toml
[dependencies]
dairy = { version = "0.1", features = ["unix"] }
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
