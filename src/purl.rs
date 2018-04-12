use ::std::borrow::Cow;
use ::std::str::FromStr;
use ::std::string::ToString;

use ::indexmap::IndexMap;
use ::percent_encoding::USERINFO_ENCODE_SET;

use super::parser;
use super::errors;
use super::utils::PercentCodec;


/// A Package URL.
#[derive(Debug, Clone)]
pub struct PackageUrl<'a> {
    /// The package URL scheme (its *type*).
    pub scheme: Cow<'a, str>,
    /// The optional namespace
    pub namespace: Option<Cow<'a, str>>,
    /// The package name.
    pub name: Cow<'a, str>,
    /// The optional package version.
    pub version: Option<Cow<'a, str>>,
    /// The package qualifiers.
    pub qualifiers: IndexMap<Cow<'a, str>, Cow<'a, str>>,
    /// The package subpath.
    pub subpath: Option<Cow<'a, str>>,
}


impl<'a> PackageUrl<'a> {

    /// Create a new Package URL with the provided scheme and name.
    pub fn new<S, N>(scheme: S, name: N) -> Self
    where
        S: Into<Cow<'a, str>>,
        N: Into<Cow<'a, str>>,
    {
        Self {
            scheme: scheme.into(),
            namespace: None,
            name: name.into(),
            version: None,
            qualifiers: IndexMap::new(),
            subpath: None,
        }
    }

    /// Assign a namespace to the package.
    pub fn with_namespace<N>(&mut self, namespace: N) -> &mut Self
    where
        N: Into<Cow<'a, str>>
    {
        self.namespace = Some(namespace.into());
        self
    }

    /// Assign a version to the package.
    pub fn with_version<V>(&mut self, version: V) -> &mut Self
    where
        V: Into<Cow<'a, str>>
    {
        self.version = Some(version.into());
        self
    }

    /// Assign a subpath to the package.
    pub fn with_subpath<S>(&mut self, subpath: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>
    {
        self.subpath = Some(subpath.into());
        self
    }

    /// Add a qualifier to the package.
    pub fn add_qualifier<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.qualifiers.insert(key.into(), value.into());
        self
    }

}

impl FromStr for PackageUrl<'static> {
    type Err = errors::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // Parse all components into strings (since we don't know infer from `s` lifetime)
        let (s, subpath) = parser::owned::parse_subpath(s)?;
        let (s, ql) = parser::owned::parse_qualifiers(s)?;
        let (s, version) = parser::owned::parse_version(s)?;
        let (s, scheme) = parser::owned::parse_scheme(s)?;
        let (s, mut name) = parser::owned::parse_name(s)?;
        let (_, mut namespace) = parser::owned::parse_namespace(s)?;

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

        let mut purl = Self::new(scheme, name);
        if let Some(ns) = namespace { purl.with_namespace(ns); }
        if let Some(v) = version { purl.with_version(v); }
        if let Some(sp) = subpath { purl.with_subpath(sp); }
        for (k, v) in ql.into_iter() { purl.add_qualifier(k, v); }

        // The obtained package url
        Ok(purl)
    }
}

impl<'a> ToString for PackageUrl<'a> {
    fn to_string(&self) -> String {
        let mut url = String::new();

        // Scheme: no encoding needed
        url.push_str(&self.scheme);
        url.push(':');

        // Namespace: percent-encode each component
        if let Some(ref ns) = self.namespace {
            ns.split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.encode(USERINFO_ENCODE_SET))
                .for_each(|pe| {
                    pe.for_each(|s| url.push_str(s));
                    url.push('/')
                });
        }

        // Name: percent-encode the name
        self.name.encode(USERINFO_ENCODE_SET).for_each(|s| url.push_str(s));

        // Version: percent-encode the version
        if let Some(ref v) = self.version {
            url.push('@');
            v.encode(USERINFO_ENCODE_SET).for_each(|s| url.push_str(s));
        }

        // Qualifiers: percent-encode the values
        if !self.qualifiers.is_empty() {
            url.push('?');

            let mut items = self.qualifiers.iter().collect::<Vec<_>>();
            items.sort();
            let ref mut it = items.iter().peekable();

            while let Some(&(k, v)) = it.next() {
                url.push_str(&k);
                url.push('=');
                &v.encode(USERINFO_ENCODE_SET).for_each(|s| url.push_str(s));
                if it.peek().is_some() {
                    url.push('&')
                };
            }
        }

        // Subpath: percent-encode the components
        if let Some(ref sp) = self.subpath {
            url.push('#');
            let components = sp
                .split('/')
                .filter(|&s| match s {"" | "." | ".." => false, _ => true})
                .map(|s| s.encode(USERINFO_ENCODE_SET));
            let ref mut it = components.peekable();
            while let Some(component) = it.next() {
                component.for_each(|s| url.push_str(s));
                if it.peek().is_some() {
                    url.push('/')
                }
            }
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
        let canonical = "type:name/space/name@version?k1=v1&k2=v2#sub/path";
        let purl_string = PackageUrl::new("type", "name")
            .with_namespace("name/space")
            .with_version("version")
            .with_subpath("sub/path")
            .add_qualifier("k1", "v1")
            .add_qualifier("k2", "v2")
            .to_string();
        assert_eq!(&purl_string, canonical);
    }

}
