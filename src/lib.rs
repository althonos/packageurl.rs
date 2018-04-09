//! lib.rs
#[cfg(feature = "memchr")]
extern crate memchr;
#[macro_use]
extern crate error_chain;
extern crate percent_encoding;

mod errors;
mod parser;
mod purl;
mod utils;

pub use purl::PackageUrl;
