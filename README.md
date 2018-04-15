gmime-rs
==========

Rust bindings and wrappers for [GMime 3](http://spruce.sourceforge.net/gmime/).

[![Build Status](https://travis-ci.org/vhdirk/gmime-rs.svg?branch=master)](https://travis-ci.org/vhdirk/gmime-rs)
[![Crate version](https://img.shields.io/crates/v/gmime.svg)](https://crates.io/crates/gmime)
[![Download statistics](https://img.shields.io/crates/d/gmime.svg)](https://crates.io/crates/gmime)
[![License](https://img.shields.io/crates/l/gmime.svg)](https://crates.io/crates/gmime)

[Documentation](https://vhdirk.github.io/gmime-rs/gmime/)

## Building
**gmime-rs** expects libgmime development files to be installed on your system.


## Using

Add this to your `Cargo.toml`:

```toml
[dependencies]
gmime = "*"
```

and this to your crate root:

```rust
extern crate gmime;
```
