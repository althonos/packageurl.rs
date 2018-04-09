use ::std::borrow::Cow;
use ::std::str::FromStr;
use ::std::string::ToString;

use ::indexmap::IndexMap;

use super::parser;
use super::errors;

#[derive(Debug, Clone)]
pub struct PackageUrl<'a> {
    pub scheme: Cow<'a, str>,
    pub namespace: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
    pub version: Option<Cow<'a, str>>,
    pub qualifiers: IndexMap<Cow<'a, str>, Cow<'a, str>>,
    pub subpath: Option<Cow<'a, str>>,
}


impl<'a> PackageUrl<'a> {

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

    pub fn with_namespace<N>(&mut self, namespace: N) -> &mut Self
    where
        N: Into<Cow<'a, str>>
    {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_version<V>(&mut self, version: V) -> &mut Self
    where
        V: Into<Cow<'a, str>>
    {
        self.version = Some(version.into());
        self
    }

    pub fn with_subpath<S>(&mut self, subpath: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>
    {
        self.subpath = Some(subpath.into());
        self
    }

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

        // Turn qualifiers into a `HashMap<Cow, Cow>`
        // let qualifiers = ql.into_iter()
        //     .map(|(k, v)| (k.into(), v.into()))
        //     .collect::<HashMap<_, _>>();

        // The obtained package url
        Ok(purl)
        // Ok(PackageUrl {
        //     scheme: Cow::Owned(scheme),
        //     namespace: namespace.map(Cow::Owned),
        //     name: Cow::Owned(name),
        //     version: version.map(Cow::Owned),
        //     qualifiers,
        //     subpath: subpath.map(Cow::Owned),
        // })
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
