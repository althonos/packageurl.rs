//! lib.rs

#[cfg(feature = "memchr")]
extern crate memchr;
#[macro_use]
extern crate error_chain;
extern crate percent_encoding;
extern crate indexmap;

mod parser;
mod utils;
pub mod errors;
pub mod purl;

pub use purl::PackageUrl;
