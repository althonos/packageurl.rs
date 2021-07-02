use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use percent_encoding::AsciiSet;

use super::errors;
use super::parser;
use super::utils::PercentCodec;

const ENCODE_SET: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'\\')
    .add(b'[')
    .add(b']')
    .add(b'^')
    .add(b'|');

/// A Package URL.
#[derive(Debug, Clone)]
pub struct PackageUrl<'a> {
    /// The package URL type.
    pub ty: Cow<'a, str>,
    /// The optional namespace
    pub namespace: Option<Cow<'a, str>>,
    /// The package name.
    pub name: Cow<'a, str>,
    /// The optional package version.
    pub version: Option<Cow<'a, str>>,
    /// The package qualifiers.
    pub qualifiers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    /// The package subpath.
    pub subpath: Option<Cow<'a, str>>,
}

impl<'a> PackageUrl<'a> {
    /// Create a new Package URL with the provided type and name.
    pub fn new<T, N>(ty: T, name: N) -> Self
    where
        T: Into<Cow<'a, str>>,
        N: Into<Cow<'a, str>>,
    {
        Self {
            ty: ty.into(),
            namespace: None,
            name: name.into(),
            version: None,
            qualifiers: HashMap::new(),
            subpath: None,
        }
    }

    /// Assign a namespace to the package.
    pub fn with_namespace<N>(&mut self, namespace: N) -> &mut Self
    where
        N: Into<Cow<'a, str>>,
    {
        self.namespace = Some(namespace.into());
        self
    }

    /// Assign a version to the package.
    pub fn with_version<V>(&mut self, version: V) -> &mut Self
    where
        V: Into<Cow<'a, str>>,
    {
        self.version = Some(version.into());
        self
    }

    /// Assign a subpath to the package.
    pub fn with_subpath<S>(&mut self, subpath: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
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

    fn from_str(s: &str) -> errors::Result<Self> {
        // Parse all components into strings (since we don't know infer from `s` lifetime)
        let (s, _) = parser::owned::parse_scheme(s)?;
        let (s, subpath) = parser::owned::parse_subpath(s)?;
        let (s, ql) = parser::owned::parse_qualifiers(s)?;
        let (s, version) = parser::owned::parse_version(s)?;
        let (s, ty) = parser::owned::parse_type(s)?;
        let (s, mut name) = parser::owned::parse_name(s)?;
        let (_, mut namespace) = parser::owned::parse_namespace(s)?;

        // Special rules for some types
        match ty.as_ref() {
            "bitbucket" | "github" => {
                name = name.to_lowercase().into();
                namespace = namespace.map(|ns| ns.to_lowercase().into());
            }
            "pypi" => {
                name = name.replace('_', "-").to_lowercase().into();
            }
            _ => {}
        };

        let mut purl = Self::new(ty, name);
        if let Some(ns) = namespace {
            purl.with_namespace(ns);
        }
        if let Some(v) = version {
            purl.with_version(v);
        }
        if let Some(sp) = subpath {
            purl.with_subpath(sp);
        }
        for (k, v) in ql.into_iter() {
            purl.add_qualifier(k, v);
        }

        // The obtained package url
        Ok(purl)
    }
}

impl fmt::Display for PackageUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Scheme: constant
        write!(f, "pkg:")?;

        // Type: no encoding needed
        write!(f, "{}/", self.ty)?;

        // Namespace: percent-encode each component
        if let Some(ref ns) = self.namespace {
            for component in ns
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.encode(ENCODE_SET))
            {
                write!(f, "{}/", component)?;
            }
        }

        // Name: percent-encode the name
        write!(f, "{}", self.name.encode(ENCODE_SET))?;

        // Version: percent-encode the version
        if let Some(ref v) = self.version {
            write!(f, "@{}", v.encode(ENCODE_SET))?;
        }

        // Qualifiers: percent-encode the values
        if !self.qualifiers.is_empty() {
            write!(f, "?")?;

            let mut items = self.qualifiers.iter().collect::<Vec<_>>();
            items.sort();

            fmt_delimited(
                items
                    .into_iter()
                    .map(|(k, v)| format!("{}={}", k, v.encode(ENCODE_SET))),
                "&",
                f,
            )?;
        }

        // Subpath: percent-encode the components
        if let Some(ref sp) = self.subpath {
            write!(f, "#")?;
            fmt_delimited(
                sp.split('/')
                    .filter(|&s| match s {
                        "" | "." | ".." => false,
                        _ => true,
                    })
                    .map(|s| s.encode(ENCODE_SET)),
                "/",
                f,
            )?;
        }

        Ok(())
    }
}

fn fmt_delimited<T: fmt::Display>(
    values: impl IntoIterator<Item = T>,
    delimiter: &str,
    formatter: &mut fmt::Formatter,
) -> fmt::Result {
    let mut iter = values.into_iter();
    if let Some(val) = iter.next() {
        val.fmt(formatter)?;
    }
    while let Some(val) = iter.next() {
        formatter.write_str(delimiter)?;
        val.fmt(formatter)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_str() {
        let raw_purl = "pkg:type/name/space/name@version?k1=v1&k2=v2#sub/path";
        let purl = PackageUrl::from_str(raw_purl).unwrap();
        assert_eq!(purl.ty, "type");
        assert_eq!(purl.namespace, Some(Cow::Borrowed("name/space")));
        assert_eq!(purl.name, "name");
        assert_eq!(purl.version, Some(Cow::Borrowed("version")));
        assert_eq!(purl.qualifiers.get("k1"), Some(&Cow::Borrowed("v1")));
        assert_eq!(purl.qualifiers.get("k2"), Some(&Cow::Borrowed("v2")));
        assert_eq!(purl.subpath, Some(Cow::Borrowed("sub/path")));
    }

    #[test]
    fn test_to_str() {
        let canonical = "pkg:type/name/space/name@version?k1=v1&k2=v2#sub/path";
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
