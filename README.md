# `packageurl-rs` [![Star me](https://img.shields.io/github/stars/althonos/packageurl-rs.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/packageurl-rs/stargazers)

*Read and generate Package URLs in Rust.*

[![Actions](https://img.shields.io/github/checks-status/althonos/packageurl-rs/master?maxAge=600&style=flat-square)](https://github.com/althonos/packageurl.rs/actions)
[![Codecov](https://img.shields.io/codecov/c/github/althonos/packageurl.rs.svg?maxAge=600&style=flat-square)](https://codecov.io/github/althonos/packageurl.rs)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/althonos/packageurl.rs)
[![Crate](https://img.shields.io/crates/v/packageurl.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/packageurl)
[![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=86400&style=flat-square)](https://docs.rs/packageurl)
[![GitHub issues](https://img.shields.io/github/issues/althonos/packageurl-rs.svg?style=flat-square&maxAge=600)](https://github.com/althonos/packageurl-rs/issues)

## About

This crate is an implementation of the [Package URL](https://github.com/package-url/purl-spec)
specification for the [Rust](http://rust-lang.org/) programming language.

## 🔌 Usage

Add this crate to the `Cargo.toml`:

```toml
[dependencies]
packageurl = "0.4.1"
```

Parse a string containing a raw PURL using the
[`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait:

```rust
extern crate packageurl;

use std::str::FromStr;
use packageurl::PackageUrl;

fn example() {
    let purl = PackageUrl::from_str("pkg:cargo/packageurl@0.3.0");
}
```

Creating a PURL from Rust and then generating a string representation using:
the [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) trait:

```rust
extern crate packageurl;

use std::string::ToString;

fn example() {
    let canonical = package::PackageUrl::new("cargo", "packageurl")
        .expect("only fails if type is invalid")
        .with_version("0.3.0")
        .to_string();
}
```

`serde` serialization and deserialization is also supported, provided the
`serde` feature is enabled:

```toml
[dependencies]
packageurl = { version = "0.4.1", features = ["serde"] }
```

See more in the [online documentation](https://docs.rs/packageurl/).

## 📝 Features

- `memchr`: use the [`memchr`](https://docs.rs/memchr/) crate to locate
  separator when parsing.
- `serde`: enable serialization and deserialization support with the
  [`serde`](https://docs.rs/serde) crate.

## 📋 Changelog

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html) and provides
a [changelog](https://github.com/althonos/packageurl-rs/blob/master/CHANGELOG.md) as part of
the [GitHub releases](https://github.com/althonos/packageurl.rs/releases).

## 💭 Feedback

Found a bug? Have an enhancement request? Head over to the
[GitHub issue tracker](https://github.com/althonos/packageurl-rs/issues) of the project if
you need to report or ask something. If you are filling in on a bug, please include as much
information as you can about the issue, and try to recreate the same bug in a simple, easily
reproducible situation.

## ⚖️ License

This library is provided under the open-source [MIT license](https://choosealicense.com/licenses/mit/).
