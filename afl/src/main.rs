#[macro_use]
extern crate afl;
extern crate packageurl;

use std::str::FromStr;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = packageurl::PackageUrl::from_str(&s);
        }
    });
}
