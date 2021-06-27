# dairy

A more compact, user friendly implementation of `Cow`.

- Optimized for use on 64-bit Unix systems (behind `unix` feature).
- Much better support for `Cow<Path>` and `Cow<OsStr>`.

```rust
use std::path::Path;
use dairy::Cow;

let name: Cow<str> = "Hello World!".into();
let path: Cow<Path> = "./path/to/file.txt".into();
```

**Caveats**

- Only `str`, `OsStr`, `CStr` and `Path` types are supported.
- `OsStr`, `CStr` and `Path` are only supported on Unix.
- On 32-bit operating systems the maximum length of types is (2¹⁶ - 1).

## Acknowledgements

Some implementation details taken from the excellent
[beef](https://github.com/maciejhirsz/beef) crate.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
