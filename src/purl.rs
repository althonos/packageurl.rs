use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

use parser;

#[derive(Debug, Clone)]
pub struct PackageUrl<'a> {
    pub scheme: Cow<'a, str>,
    pub namespace: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
    pub version: Option<Cow<'a, str>>,
    pub qualifiers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub subpath: Option<Cow<'a, str>>,
}

impl FromStr for PackageUrl<'static> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        use nom;

        // Parse all components
        let (s, sp) = try_parse!(s, parser::owned::parse_subpath);
        let (s, ql) = try_parse!(s, parser::owned::parse_qualifiers);
        let (s, vr) = try_parse!(s, parser::owned::parse_version);
        let (s, sc) = try_parse!(s, parser::owned::parse_scheme);
        let (s, mut nm) = try_parse!(s, parser::owned::parse_name);
        let (_, mut ns) = try_parse!(s, parser::owned::parse_namespace);

        // Special rules for some schemes
        match sc.as_ref() {
            "bitbucket" | "github" => {
                nm = nm.to_lowercase().into();
                ns = ns.map(|namespace| namespace.to_lowercase().into());
            }
            "pypi" => {
                nm = nm.replace('_', "-").to_lowercase().into();
            }
            _ => {}
        };

        // The obtained package url
        Ok(PackageUrl {
            scheme: sc,
            namespace: ns,
            name: nm,
            version: vr,
            qualifiers: ql,
            subpath: sp,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[test]
    fn test_from_str() {
        let raw_purl = "type:name/space/name@version?k1=v1&k2=v2#sub/path";
        let purl = PackageUrl::from_str(raw_purl).unwrap();
        assert_eq!(purl.scheme, "type");
        assert_eq!(purl.namespace, Some(Cow::Borrowed("name/space")));
        assert_eq!(purl.name, "name");
        assert_eq!(purl.version, Some(Cow::Borrowed("version")));
        //assert_eq!(purl.qualifiers.get("k1"), Some("v1")));
        //assert_eq!(purl.qualifiers.get("k2"), Some("v2")));
        assert_eq!(purl.subpath, Some(Cow::Borrowed("sub/path")));
    }

    #[bench]
    fn bench_from_str(b: &mut Bencher) {
        let raw_purl = "type:name/space/name@version?k1=v1&k2=v2#sub/path";
        b.iter(|| PackageUrl::from_str(raw_purl));
    }

}
