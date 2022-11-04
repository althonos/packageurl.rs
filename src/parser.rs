use super::errors::Error;
use super::errors::Result;
use super::utils;
use super::utils::PercentCodec;
use super::utils::QuickFind;
use super::validation;

pub fn parse_scheme<'a>(input: &str) -> Result<(&str, String)> {
    if let Some(i) = input.quickfind(b':') {
        if &input[..i] == "pkg" {
            let mut j = i + 1;
            let mut it = input[i + 1..].chars();
            while let Some('/') = it.next() {
                j += 1;
            }

            Ok((&input[j..], input[..i].to_string()))
        } else {
            Err(Error::InvalidScheme(input[..i].to_string()))
        }
    } else {
        Err(Error::MissingScheme)
    }
}

pub fn parse_subpath<'a>(input: &str) -> Result<(&str, Option<String>)> {
    if let Some(i) = input.quickrfind(b'#') {
        let mut subpath = String::with_capacity(i + 1);
        let mut components = input[i + 1..]
            .trim_matches('/')
            .split('/')
            .filter(|&c| !(c.is_empty() || c == "." || c == ".."));
        if let Some(c) = components.next() {
            let decoded = c.decode().decode_utf8()?;
            if validation::is_subpath_segment_valid(&decoded) {
                subpath.push_str(&decoded);
            } else {
                return Err(Error::InvalidSubpathSegment(decoded.to_string()));
            }
        }
        while let Some(c) = components.next() {
            let decoded = c.decode().decode_utf8()?;
            if validation::is_subpath_segment_valid(&decoded) {
                subpath.push('/');
                subpath.push_str(&decoded);
            } else {
                return Err(Error::InvalidSubpathSegment(decoded.to_string()));
            }
        }
        Ok((&input[..i], Some(subpath)))
    } else {
        Ok((input, None))
    }
}

pub fn parse_qualifiers<'a>(input: &str) -> Result<(&str, Vec<(String, String)>)> {
    if let Some(i) = input.quickrfind(b'?') {
        let mut qualifiers = Vec::new();
        let pairs = input[i + 1..]
            .split('&')
            .map(|ref pair| utils::cut(pair, b'='))
            .filter(|ref pair| !pair.1.is_empty());
        for (key, value) in pairs {
            if validation::is_qualifier_key_valid(key) {
                qualifiers.push((
                    key.to_lowercase(),
                    value.decode().decode_utf8()?.to_string(),
                ))
            } else {
                return Err(Error::InvalidKey(key.to_string()));
            }
        }
        Ok((&input[..i], qualifiers))
    } else {
        Ok((input, Vec::new()))
    }
}

pub fn parse_version<'a>(input: &str) -> Result<(&str, Option<String>)> {
    if let Some(i) = input.quickrfind(b'@') {
        Ok((
            &input[..i],
            Some(input[i + 1..].decode().decode_utf8()?.into()),
        ))
    } else {
        Ok((input, None))
    }
}

pub fn parse_type<'a>(input: &str) -> Result<(&str, String)> {
    match input.quickfind(b'/') {
        Some(i) if validation::is_type_valid(&input[..i]) => {
            Ok((&input[i + 1..], input[..i].to_lowercase().into()))
        }
        Some(i) => Err(Error::InvalidType(input[..i].to_string())),
        None => Err(Error::MissingType),
    }
}

pub fn parse_name<'a>(input: &str) -> Result<(&str, String)> {
    let (rem, name) = utils::rcut(input.trim_matches('/'), b'/');
    if name.is_empty() {
        Err(Error::MissingName)
    } else {
        let canonical_name = name.decode().decode_utf8()?.to_string();
        Ok((rem, canonical_name))
    }
}

pub fn parse_namespace<'a>(input: &'a str, pkg_type: &str) -> Result<(&'a str, Option<String>)> {
    if !input.is_empty() {
        let mut namespace = String::with_capacity(input.len());
        let mut components = input
            .trim_matches('/')
            .split('/')
            .filter(|&c| !(c.is_empty() || c == "." || c == ".."));
        if let Some(c) = components.next() {
            let decoded = c.decode().decode_utf8()?;
            if validation::is_namespace_component_valid(&decoded) {
                namespace.push_str(&decoded);
            } else {
                return Err(Error::InvalidNamespaceComponent(decoded.to_string()));
            }
        }
        while let Some(c) = components.next() {
            let decoded = c.decode().decode_utf8()?;
            if validation::is_namespace_component_valid(&decoded) {
                namespace.push('/');
                namespace.push_str(&decoded);
            } else {
                return Err(Error::InvalidNamespaceComponent(decoded.to_string()));
            }
        }
        Ok(("", Some(namespace)))
    } else {
        //Some ecosystems require non-empty namespaces
        match pkg_type {
            "maven" => Err(Error::InvalidNamespaceComponent(input.to_string())),
            _ => Ok(("", None))
        }
    }
}
