use std::env;

fn env(key: &str, value: &str) -> bool {
    env::var(key).as_deref() == Ok(value)
}

fn main() {
    if env("CARGO_CFG_TARGET_FAMILY", "unix") || env("CARGO_CFG_TARGET_OS", "wasi") {
        println!("cargo:rustc-cfg=os_str_ext");
    }
}
