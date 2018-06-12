use super::errors;
use super::utils;

pub mod owned {

    use super::errors;
    use super::utils;
    use utils::{JoinableIterator, PercentCodec, QuickFind};

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
                bail!(errors::ErrorKind::InvalidScheme(input[..i].to_string()))
            }
        } else {
            bail!(errors::ErrorKind::MissingScheme)
        }
    }

    pub fn parse_subpath<'a>(input: &str) -> errors::Result<(&str, Option<String>)> {
        if let Some(i) = input.quickrfind(b'#') {
            let subpath = input[i + 1..]
                .trim_matches('/')
                .split('/')
                .filter(|&c| !(c.is_empty() || c == "." || c == ".."))
                .map(|c| c.as_bytes().decode().decode_utf8_lossy())
                .join("/");
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
            bail!(errors::ErrorKind::MissingType)
        }
    }

    pub fn parse_name<'a>(input: &str) -> errors::Result<(&str, String)> {
        let (rem, name) = utils::rcut(input.trim_matches('/'), b'/');

        let canonical_name = if name.is_empty() {
            bail!(errors::ErrorKind::MissingName)
        } else {
            name.decode().decode_utf8_lossy().to_string()
        };

        Ok((rem, canonical_name))
    }

    pub fn parse_namespace<'a>(input: &str) -> errors::Result<(&str, Option<String>)> {
        if !input.is_empty() {
            let namespace = input
                .trim_matches('/')
                .split('/')
                .filter(|&c| !(c.is_empty() || c == "." || c == ".."))
                .map(|c| c.decode().decode_utf8_lossy())
                .join("/");
            Ok(("", Some(namespace)))
        } else {
            Ok(("", None))
        }
    }

}
