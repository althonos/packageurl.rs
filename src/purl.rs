use std::borrow::Cow;
use std::collections::HashMap;

use std::str::FromStr;
use std::string::ToString;

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

impl<'a> FromStr for PackageUrl<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom;

        // Parse all components
        let (s, subpath) = try_parse!(s, parser::owned::parse_subpath);
        let (s, qualifiers) = try_parse!(s, parser::owned::parse_qualifiers);
        let (s, version) = try_parse!(s, parser::owned::parse_version);
        let (s, scheme) = try_parse!(s, parser::owned::parse_scheme);
        let (s, mut name) = try_parse!(s, parser::owned::parse_name);
        let (_, mut namespace) = try_parse!(s, parser::owned::parse_namespace);

        // Special rules for some schemes
        match scheme.as_ref() {
            "bitbucket" | "github" => {
                name = name.to_lowercase().into();
                namespace = namespace.map(|ns| ns.to_lowercase().into());
            }
            "pypi" => {
                name = name.replace('_', "-").to_lowercase().into();
            }
            _ => {}
        };

        // The obtained package url
        Ok(PackageUrl {
            scheme,
            namespace,
            name,
            version,
            qualifiers,
            subpath,
        })
    }
}

impl<'a> ToString for PackageUrl<'a> {
    fn to_string(&self) -> String {
        let mut url = String::new();

        url.push_str(&self.scheme);
        url.push(':');

        if let Some(ref ns) = self.namespace {
            url.push_str(ns);
            url.push('/');
        }

        url.push_str(&self.name);

        if let Some(ref v) = self.version {
            url.push('@');
            url.push_str(v);
        }

        if !self.qualifiers.is_empty() {
            url.push('?');

            let mut items = self.qualifiers.iter().collect::<Vec<_>>();
            items.sort();
            let ref mut it = items.iter().peekable();

            while let Some(&(k, v)) = it.next() {
                url.push_str(&k);
                url.push('=');
                url.push_str(&v);
                if let Some(_) = it.peek() {
                    url.push('&')
                };
            }
        }

        if let Some(ref sp) = self.subpath {
            url.push('#');
            url.push_str(sp);
        }

        url
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_str() {
        let raw_purl = "type:name/space/name@version?k1=v1&k2=v2#sub/path";
        let purl = PackageUrl::from_str(raw_purl).unwrap();
        assert_eq!(purl.scheme, "type");
        assert_eq!(purl.namespace, Some(Cow::Borrowed("name/space")));
        assert_eq!(purl.name, "name");
        assert_eq!(purl.version, Some(Cow::Borrowed("version")));
        assert_eq!(purl.qualifiers.get("k1"), Some(&Cow::Borrowed("v1")));
        assert_eq!(purl.qualifiers.get("k2"), Some(&Cow::Borrowed("v2")));
        assert_eq!(purl.subpath, Some(Cow::Borrowed("sub/path")));
    }

    #[test]
    fn test_to_str() {
        let raw_purl = "type:name/space/name@version?k1=v1&k2=v2#sub/path";
        let purl = PackageUrl::from_str(raw_purl).unwrap();
        assert_eq!(&purl.to_string(), raw_purl);
    }

}
