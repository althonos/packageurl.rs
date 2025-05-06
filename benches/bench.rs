use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub(crate) mod benches {
    use criterion::Criterion;
    use packageurl::PackageUrl;
    use std::str::FromStr;

    pub fn bench_from_str(b: &mut Criterion) {
        let raw_purl = "pkg:type/name/space/name@version?k1=v1&k2=v2#sub/path";
        b.bench_function("from_str", |_| {
            let _ = PackageUrl::from_str(raw_purl).unwrap();
        });
    }

    pub fn bench_from_url(b: &mut Criterion) {
        let raw_purl = "pkg:type/name/space/name@version?k1=v1&k2=v2#sub/path";
        b.bench_function("parse", |_| {
            let _ = url::Url::parse(raw_purl).unwrap();
        });
    }
}

criterion_group! {
  name = benches;
  // since insertion takes so long, we need to reduce the sample
  // size and increase the time so that we can get a few iterations
  // in between db resets.
  config = Criterion::default()
    .measurement_time(Duration::from_secs(15))
    .sample_size(10);
  targets = benches::bench_from_str, benches::bench_from_url
}
criterion_main!(benches);
