use super::errors;
use super::utils;

pub mod owned {

    use super::errors;
    use super::utils;
    use utils::{PercentCodec, QuickFind};

    pub fn parse_scheme<'a>(input: &str) -> errors::Result<(&str, String)> {
        if let Some(i) = input.quickfind(b':') {
            if &input[..i] == "pkg" {
                let mut j = i + 1;
                let mut it = input[i + 1..].chars();
                while let Some('/') = it.next() {
                    j += 1;
                }

                Ok((&input[j..], input[..i].to_string()))
            } else {
                Err(errors::Error::InvalidScheme(input[..i].to_string()))
            }
        } else {
            Err(errors::Error::MissingScheme)
        }
    }

    pub fn parse_subpath<'a>(input: &str) -> errors::Result<(&str, Option<String>)> {
        if let Some(i) = input.quickrfind(b'#') {
            let mut subpath = String::with_capacity(i + 1);
            let mut components = input[i + 1..]
                .trim_matches('/')
                .split('/')
                .filter(|&c| !(c.is_empty() || c == "." || c == ".."));
            if let Some(c) = components.next() {
                subpath.push_str(&c.decode().decode_utf8()?);
            }
            while let Some(c) = components.next() {
                subpath.push('/');
                subpath.push_str(&c.decode().decode_utf8()?);
            }
            Ok((&input[..i], Some(subpath)))
        } else {
            Ok((input, None))
        }
    }

    pub fn parse_qualifiers<'a>(input: &str) -> errors::Result<(&str, Vec<(String, String)>)> {
        if let Some(i) = input.quickrfind(b'?') {
            let qualifiers = input[i + 1..]
                .split('&')
                .map(|ref pair| utils::cut(pair, b'='))
                .filter(|ref pair| !pair.1.is_empty())
                .map(|(key, value)| (key.to_lowercase(), value.to_string()))
                .collect();
            Ok((&input[..i], qualifiers))
        } else {
            Ok((input, Vec::new()))
        }
    }

    pub fn parse_version<'a>(input: &str) -> errors::Result<(&str, Option<String>)> {
        if let Some(i) = input.quickrfind(b'@') {
            Ok((&input[..i], Some(input[i + 1..].to_string().into())))
        } else {
            Ok((input, None))
        }
    }

    pub fn parse_type<'a>(input: &str) -> errors::Result<(&str, String)> {
        if let Some(i) = input.quickfind(b'/') {
            Ok((&input[i + 1..], input[..i].to_lowercase().into()))
        } else {
            Err(errors::Error::MissingType)
        }
    }

    pub fn parse_name<'a>(input: &str) -> errors::Result<(&str, String)> {
        let (rem, name) = utils::rcut(input.trim_matches('/'), b'/');
        if name.is_empty() {
            Err(errors::Error::MissingName)
        } else {
            let canonical_name = name.decode().decode_utf8()?.to_string();
            Ok((rem, canonical_name))
        }
    }

    pub fn parse_namespace<'a>(input: &str) -> errors::Result<(&str, Option<String>)> {
        if !input.is_empty() {
            let mut namespace = String::with_capacity(input.len());
            let mut components = input
                .trim_matches('/')
                .split('/')
                .filter(|&c| !(c.is_empty() || c == "." || c == ".."));
            if let Some(c) = components.next() {
                namespace.push_str(&c.decode().decode_utf8()?);
            }
            while let Some(c) = components.next() {
                namespace.push('/');
                namespace.push_str(&c.decode().decode_utf8()?);
            }
            Ok(("", Some(namespace)))
        } else {
            Ok(("", None))
        }
    }
}
