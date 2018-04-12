//! `packageurl` is an implementation of the [Package URL] specification for the [Rust] programming language.
//!
//! [Rust]: http://rust-lang.org/
//! [Package URL]: https://github.com/package-url/purl-spec
//!
//!
//! # Parsing
//!
//! Parse a package url after bringing the [`FromStr`] trait in scope:
//! ```rust
//! use std::str::FromStr;
//! use packageurl::PackageUrl;
//!
//! let purl = PackageUrl::from_str("npm:%40angular/animation@12.3.1").unwrap();
//! assert!(purl.name == "animation");
//! assert!(purl.namespace.unwrap() == "@angular");
//! ```
//!
//! Parsing a purl may fail, in which case an error kind from the [`errors`] module
//! is returned:
//! ```rust
//! use std::str::FromStr;
//! use packageurl::PackageUrl;
//!
//! let err = PackageUrl::from_str("package@0.1.0").unwrap_err();
//! assert!(err.description() == "missing scheme");
//! ```
//!
//! The parsed [`PackageUrl`] will have a [`'static`] lifetime, so that the parsed string can be
//! safely discarded.
//!
//! [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
//! [`PackageUrl`]: example_generated/struct.PackageUrl.html
//! [`'static`]: https://doc.rust-lang.org/reference/items/static-items.html#static-lifetime-elision
#![doc(issue_tracker_base_url = "https://github.com/althonos/packageurl-rs/issues/")]

#[cfg(feature = "memchr")]
extern crate memchr;
#[macro_use]
extern crate error_chain;
extern crate percent_encoding;

mod parser;
mod utils;
mod purl;
pub mod errors;

pub use purl::PackageUrl;
