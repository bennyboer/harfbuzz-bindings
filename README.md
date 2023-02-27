# HarfBuzz Bindings for Rust

Bindings for the [HarfBuzz](https://github.com/harfbuzz/harfbuzz) text shaping engine generated with [bindgen](https://github.com/rust-lang/rust-bindgen) to support the newest versions of HarfBuzz.

## Preconditions

- Install `vcpkg` (https://github.com/microsoft/vcpkg)
- Install `harfbuzz` with `vcpkg install harfbuzz --triplet x64-windows-static` (or another triplet matching your target OS) to get a statically compiled HarfBuzz version
- Set environment variable `RUSTFLAGS = "-Ctarget-feature=+crt-static"`
- Setup `bindgen` (https://rust-lang.github.io/rust-bindgen/requirements.html)

## Usage

Add this dependency to your `Cargo.toml`:

```toml
[dependencies]
harfbuzz-bindings = "0.1"
```

And fire up `cargo build`!

## Additional information

This crate has been implemented following the [bindgen tutorial](https://rust-lang.github.io/rust-bindgen/introduction.html).
