//! parser.rs

pub mod owned {

    use ::std::collections::HashMap;
    use ::percent_encoding::percent_decode;

    use super::super::errors;
    use super::super::utils;
    use utils::{QuickFind, JoinableIterator};

    pub fn parse_subpath<'a>(input: &str) -> errors::Result<(&str, Option<String>)> {
        if let Some(i) = input.quickrfind(b'#') {
            let subpath = input[i + 1..]
                .trim_matches('/')
                .split('/')
                .filter(|&c| !(c.is_empty() || c == "." || c == ".."))
                .map(|c| percent_decode(c.as_bytes()).decode_utf8_lossy())
                .join("/");
            Ok((&input[..i], Some(subpath)))
        } else {
            Ok((input, None))
        }
    }

    pub fn parse_qualifiers<'a>(input: &str) -> errors::Result<(&str, HashMap<String, String>)> {
        if let Some(i) = input.quickrfind(b'?') {
            let qualifiers = input[i + 1..]
                .split('&')
                .map(|ref pair| utils::cut(pair, b'='))
                .filter(|ref pair| !pair.1.is_empty())
                .map(|(key, value)| (key.to_lowercase(), value.to_string()))
                .collect::<HashMap<_, _>>();
            Ok((&input[..i], qualifiers))
        } else {
            Ok((input, HashMap::new()))
        }
    }

    pub fn parse_version<'a>(input: &str) -> errors::Result<(&str, Option<String>)> {
        if let Some(i) = input.quickrfind(b'@') {
            Ok((&input[..i], Some(input[i + 1..].to_string().into())))
        } else {
            Ok((input, None))
        }
    }

    pub fn parse_scheme<'a>(input: &str) -> errors::Result<(&str, String)> {
        if let Some(i) = input.quickfind(b':') {
            Ok((&input[i + 1..], input[..i].to_lowercase().into()))
        } else {
            bail!(errors::ErrorKind::MissingScheme)
        }
    }

    pub fn parse_name<'a>(input: &str) -> errors::Result<(&str, String)> {
        let (rem, name) = utils::rcut(input.trim_matches('/'), b'/');

        let canonical_name = if name.is_empty() {
            bail!(errors::ErrorKind::MissingName)
        } else {
            percent_decode(name.as_bytes())
                .decode_utf8_lossy()
                .to_string()
        };

        Ok((rem, canonical_name))
    }

    pub fn parse_namespace<'a>(input: &str) -> errors::Result<(&str, Option<String>)> {
        if !input.is_empty() {
            let namespace = input
                .trim_matches('/')
                .split('/')
                .filter(|&c| !(c.is_empty() || c == "." || c == ".."))
                .map(|c| percent_decode(c.as_bytes()).decode_utf8_lossy())
                .join("/");
            Ok(("", Some(namespace)))
        } else {
            Ok(("", None))
        }
    }

}
