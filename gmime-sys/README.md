gmime-sys
==========

Rust ffi-bindings for [GMime 3](http://spruce.sourceforge.net/gmime/).

[![Crate version](https://img.shields.io/crates/v/gmime-sys.svg)](https://crates.io/crates/gmime-sys)
[![Download statistics](https://img.shields.io/crates/d/gmime-sys.svg)](https://crates.io/crates/gmime-sys)
[![License](https://img.shields.io/crates/l/gmime-sys.svg)](https://crates.io/crates/gmime-sys)

[Documentation](https://vhdirk.github.io/gmime-rs/gmime-sys/)

## Building
**gmime-sys** expects libgmime development files to be installed on your system.


## Using

Add this to your `Cargo.toml`:

```toml
[dependencies]
gmime-sys = "*"
```

and this to your crate root:

```rust
extern crate gmime_sys;
```
