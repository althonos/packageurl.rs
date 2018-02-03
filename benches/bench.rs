#![feature(test)]

extern crate packageurl;
extern crate test;
extern crate url;

use std::str::FromStr;

use test::Bencher;
use packageurl::PackageUrl;

#[bench]
fn bench_from_str(b: &mut Bencher) {
    let raw_purl = "type:name/space/name@version?k1=v1&k2=v2#sub/path";
    b.iter(|| {
        let _ = PackageUrl::from_str(raw_purl).unwrap();
    });
}

#[bench]
fn bench_from_url(b: &mut Bencher) {
    let raw_purl = "type:name/space/name@version?k1=v1&k2=v2#sub/path";
    b.iter(|| {
        let _ = url::Url::parse(raw_purl).unwrap();
    });
}
