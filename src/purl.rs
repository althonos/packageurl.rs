use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use percent_encoding::AsciiSet;

use super::errors::Error;
use super::errors::Result;
use super::parser;
use super::validation;
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
    // .add(b'/')
    // .add(b':')
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
    ty: Cow<'a, str>,
    /// The optional namespace
    namespace: Option<Cow<'a, str>>,
    /// The package name.
    name: Cow<'a, str>,
    /// The optional package version.
    version: Option<Cow<'a, str>>,
    /// The package qualifiers.
    qualifiers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    /// The package subpath.
    subpath: Option<Cow<'a, str>>,
}

impl<'a> PackageUrl<'a> {
    /// Create a new Package URL with the provided type and name.
    ///
    /// The Package URL type must be valid, otherwise an error will be returned.
    /// The type can only be composed of ASCII letters and numbers, '.', '+'
    /// and '-' (period, plus and dash). It cannot start with a number and
    /// cannot contain spaces.
    ///
    /// # Example
    /// ```rust
    /// # extern crate packageurl;
    /// assert!( packageurl::PackageUrl::new("cargo", "packageurl").is_ok() );
    /// assert!( packageurl::PackageUrl::new("bad type", "packageurl").is_err() );
    /// ```
    pub fn new<T, N>(ty: T, name: N) -> Result<Self>
    where
        T: Into<Cow<'a, str>>,
        N: Into<Cow<'a, str>>,
    {
        let t = ty.into();
        if validation::is_type_valid(&t) {
            Ok(Self::new_unchecked(t, name))
        } else {
            Err(Error::InvalidType(t.to_string()))
        }
    }

    /// Create a new Package URL without checking the type.
    fn new_unchecked<T, N>(ty: T, name: N) -> Self
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

    /// Get the Package URL type.
    pub fn ty(&self) -> &str {
        self.ty.as_ref()
    }

    /// Get the optional namespace.
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_ref().map(Cow::as_ref)
    }

    /// Get the package name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get the optional package version.
    pub fn version(&self) -> Option<&str> {
        self.version.as_ref().map(Cow::as_ref)
    }

    /// Get the package qualifiers
    pub fn qualifiers(&self) -> &HashMap<Cow<'a, str>, Cow<'a, str>> {
        &self.qualifiers
    }

    /// Get the optional package subpath.
    pub fn subpath(&self) -> Option<&str> {
        self.subpath.as_ref().map(Cow::as_ref)
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
    ///
    /// Subpaths must not contain empty, local ('.') or parent ('..') segments,
    /// otherwise an error will be returned.
    pub fn with_subpath<S>(&mut self, subpath: S) -> Result<&mut Self>
    where
        S: Into<Cow<'a, str>>,
    {
        let s = subpath.into();
        for component in s.split('/') {
            if !validation::is_subpath_segment_valid(component) {
                return Err(Error::InvalidSubpathSegment(component.into()));
            }
        }
        self.subpath = Some(s);
        Ok(self)
    }

    /// Add a qualifier to the package.
    pub fn add_qualifier<K, V>(&mut self, key: K, value: V) -> Result<&mut Self>
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        let k = key.into();
        if !validation::is_qualifier_key_valid(&k) {
            Err(Error::InvalidKey(k.into()))
        } else {
            self.qualifiers.insert(k, value.into());
            Ok(self)
        }
    }
}

impl FromStr for PackageUrl<'static> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Parse all components into strings (since we don't know infer from `s` lifetime)
        let (s, _) = parser::parse_scheme(s)?;
        let (s, subpath) = parser::parse_subpath(s)?;
        let (s, ql) = parser::parse_qualifiers(s)?;
        let (s, version) = parser::parse_version(s)?;
        let (s, ty) = parser::parse_type(s)?;
        let (s, mut name) = parser::parse_name(s)?;
        let (_, mut namespace) = parser::parse_namespace(s)?;

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

        let mut purl = Self::new_unchecked(ty, name);
        if let Some(ns) = namespace {
            purl.namespace = Some(ns.into());
        }
        if let Some(v) = version {
            purl.version = Some(v.into());
        }
        if let Some(sp) = subpath {
            purl.subpath = Some(sp.into());
        }
        for (k, v) in ql.into_iter() {
            purl.qualifiers.insert(k.into(), v.into());
        }

        // The obtained package url
        Ok(purl)
    }
}

impl fmt::Display for PackageUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Scheme: constant
        f.write_str("pkg:")?;

        // Type: no encoding needed
        self.ty.fmt(f).and(f.write_str("/"))?;

        // Namespace: percent-encode each component
        if let Some(ref ns) = self.namespace {
            for component in ns
                .split('/')
                .map(|s| s.encode(ENCODE_SET))
            {
                component.fmt(f).and(f.write_str("/"))?;
            }
        }

        // Name: percent-encode the name
        self.name.encode(ENCODE_SET).fmt(f)?;

        // Version: percent-encode the version
        if let Some(ref v) = self.version {
            f.write_str("@").and(v.encode(ENCODE_SET).fmt(f))?;
        }

        // Qualifiers: percent-encode the values
        if !self.qualifiers.is_empty() {
            f.write_str("?")?;

            let mut items = self.qualifiers.iter().collect::<Vec<_>>();
            items.sort();

            let mut iter = items.into_iter();
            if let Some((k, v)) = iter.next() {
                k.fmt(f)
                    .and(f.write_str("="))
                    .and(v.encode(ENCODE_SET).fmt(f))?;
            }
            while let Some((k, v)) = iter.next() {
                f.write_str("&")
                    .and(k.fmt(f))
                    .and(f.write_str("="))
                    .and(v.encode(ENCODE_SET).fmt(f))?;
            }
        }

        // Subpath: percent-encode the components
        if let Some(ref sp) = self.subpath {
            f.write_str("#")?;
            let mut components = sp
                .split('/')
                .filter(|&s| !(s == "" || s == "." || s == ".."));
            if let Some(component) = components.next() {
                component.encode(ENCODE_SET).fmt(f)?;
            }
            while let Some(component) = components.next() {
                f.write_str("/")?;
                component.encode(ENCODE_SET).fmt(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_str() {
        let raw_purl = "pkg:type/name/space/name@version?k1=v1&k2=v2#sub/path";
        let purl = PackageUrl::from_str(raw_purl).unwrap();
        assert_eq!(purl.ty(), "type");
        assert_eq!(purl.namespace(), Some("name/space"));
        assert_eq!(purl.name(), "name");
        assert_eq!(purl.version(), Some("version"));
        assert_eq!(purl.qualifiers().get("k1"), Some(&Cow::Borrowed("v1")));
        assert_eq!(purl.qualifiers().get("k2"), Some(&Cow::Borrowed("v2")));
        assert_eq!(purl.subpath(), Some("sub/path"));
    }

    #[test]
    fn test_to_str() {
        let canonical = "pkg:type/name/space/name@version?k1=v1&k2=v2#sub/path";
        let purl_string = PackageUrl::new("type", "name")
            .unwrap()
            .with_namespace("name/space")
            .with_version("version")
            .with_subpath("sub/path").unwrap()
            .add_qualifier("k1", "v1").unwrap()
            .add_qualifier("k2", "v2").unwrap()
            .to_string();
        assert_eq!(&purl_string, canonical);
    }
}
