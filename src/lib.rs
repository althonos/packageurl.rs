//! lib.rs
#![feature(test)]

#[cfg(test)]
extern crate test;

#[cfg(feature = "memchr")]
extern crate memchr;

extern crate nom;
extern crate percent_encoding;

#[macro_use]
mod macros;
mod parser;
mod purl;
mod utils;

pub use purl::PackageUrl;
