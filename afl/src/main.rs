extern crate afl;
extern crate packageurl;

use std::str::FromStr;

fn main() {
    afl::read_stdio_string(|string| {
        let _ = packageurl::PackageUrl::from_str(&string);
    })
}
