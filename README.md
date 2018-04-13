# `packageurl`

*Read and generate Package URLs in Rust.*

[![TravisCI](https://img.shields.io/travis/althonos/packageurl-rs/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/packageurl-rs/branches)
[![Codecov](https://img.shields.io/codecov/c/github/althonos/packageurl-rs.svg?maxAge=600&style=flat-square)](https://codecov.io/github/althonos/packageurl-rs)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/althonos/packageurl-rs)
[![Crate](https://img.shields.io/crates/v/packageurl.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/packageurl)
[![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=86400&style=flat-square)](https://docs.rs/packageurl)
[![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=86400&style=flat-square)](https://sagiegurari.github.io/cargo-make)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=86400&style=flat-square)](http://keepachangelog.com/)
[![SayThanks](https://img.shields.io/badge/say-thanks!-1EAEDB.svg?maxAge=86400&style=flat-square)](https://saythanks.io/to/althonos)

## About

This crate is an implementation of the [Package URL](https://github.com/package-url/purl-spec)
specification for the [Rust](http://rust-lang.org/) programming language.

## Usage

Add this crate to the `Cargo.toml`:

```toml
[dependencies]
packageurl = "^0.1.0"
```

Parse a string containing a raw PURL using the
[`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait:

```rust
extern crate packageurl
use std::str::FromStr;
use packageurl::PackageUrl;

let purl = PackageUrl::from_str("cargo:packageurl@0.1.0")
```

Generate a canonical PURL using the
[`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) trait:

```rust
extern crate packageurl;
use std::string::ToString;
use package::PackageUrl;

let canonical = PackageUrl::new("cargo", "packageurl")
    .with_version("0.1.0")
    .to_string();
```

See more in the [online documentation](https://docs.rs/packageurl/).


## Note

The specification is not completely enforced right now: in particular, it will
allow building Package URLs with invalid namespaces and the like. More validations
and errors are to be added in the future.
