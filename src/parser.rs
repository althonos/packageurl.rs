//! parser.rs

use std::borrow::Cow;
use std::collections::HashMap;

type CowStr<'a> = Cow<'a, str>;
type OptCowStr<'a> = Option<CowStr<'a>>;
type CowStrMap<'a> = HashMap<CowStr<'a>, CowStr<'a>>;

pub mod owned {

    use std::borrow::Cow;
    use std::collections::HashMap;

    use nom::IResult;
    use nom::ErrorKind;
    use percent_encoding::percent_decode;

    use utils;
    use utils::JoinableIterator;
    use utils::QuickFind;

    use super::{CowStr, CowStrMap, OptCowStr};

    pub fn parse_subpath<'a>(input: &str) -> IResult<&str, OptCowStr<'a>> {
        if let Some(i) = input.quickrfind(b'#') {
            let subpath = input[i + 1..]
                .trim_matches('/')
                .split('/')
                .filter(|&c| !(c.is_empty() || c == "." || c == ".."))
                .map(|c| percent_decode(c.as_bytes()).decode_utf8_lossy())
                .join("/");
            IResult::Done(&input[..i], Some(Cow::Owned(subpath)))
        } else {
            IResult::Done(input, None)
        }
    }

    pub fn parse_qualifiers<'a>(input: &str) -> IResult<&str, CowStrMap<'a>> {
        if let Some(i) = input.quickrfind(b'?') {
            let qualifiers = input[i + 1..]
                .split('&')
                .map(|ref pair| utils::cut(pair, b'='))
                .filter(|ref pair| !pair.1.is_empty())
                .map(|(key, value)| (key.to_lowercase().into(), value.to_string().into()))
                .collect::<CowStrMap<'a>>();
            IResult::Done(&input[..i], qualifiers)
        } else {
            IResult::Done(input, HashMap::new())
        }
    }

    pub fn parse_version<'a>(input: &str) -> IResult<&str, OptCowStr<'a>> {
        if let Some(i) = input.quickrfind(b'@') {
            IResult::Done(&input[..i], Some(input[i + 1..].to_string().into()))
        } else {
            IResult::Done(input, None)
        }
    }

    pub fn parse_scheme<'a>(input: &str) -> IResult<&str, CowStr<'a>> {
        if let Some(i) = input.quickfind(b':') {
            IResult::Done(&input[i + 1..], input[..i].to_lowercase().into())
        } else {
            // FIXME: true error management
            IResult::Error(ErrorKind::Custom(1))
        }
    }

    pub fn parse_name<'a>(input: &str) -> IResult<&str, CowStr<'a>> {
        let (rem, name) = utils::rcut(input.trim_matches('/'), b'/');

        if name.is_empty() {
            return IResult::Error(ErrorKind::Custom(0));
        }

        IResult::Done(
            rem,
            percent_decode(name.as_bytes())
                .decode_utf8_lossy()
                .to_string()
                .into(),
        )
    }

    pub fn parse_namespace<'a>(input: &str) -> IResult<&str, OptCowStr<'a>> {
        if !input.is_empty() {
            let namespace = input
                .trim_matches('/')
                .split('/')
                .filter(|&c| !(c.is_empty() || c == "." || c == ".."))
                .map(|c| percent_decode(c.as_bytes()).decode_utf8_lossy())
                .join("/");
            IResult::Done("", Some(namespace.into()))
        } else {
            IResult::Done("", None)
        }
    }

}
