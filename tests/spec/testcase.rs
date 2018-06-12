use std::borrow::Cow;
use std::collections::HashMap;

// Jun 12, 2018
static TEST_SUITE: &[u8] = include_bytes!("test-suite-data.json");

#[derive(Deserialize)]
pub struct SpecTestCase<'a> {
    pub description: Cow<'a, str>,
    pub purl: Cow<'a, str>,
    pub canonical_purl: Cow<'a, str>,
    pub ty: Cow<'a, str>,
    pub namespace: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
    pub version: Option<Cow<'a, str>>,
    pub qualifiers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub subpath: Option<Cow<'a, str>>,
    pub is_invalid: bool,
}

impl<'a> SpecTestCase<'a> {
    pub fn new(desc: &'a str) -> Self {
        if let Ok(::serde_json::Value::Array(v)) = ::serde_json::from_slice(TEST_SUITE) {
            let json = v
                .into_iter()
                .find(|x| x["description"].as_str().unwrap().eq(desc))
                .unwrap();
            ::serde_json::from_value(json).unwrap()
        } else {
            unreachable!()
        }
    }
}
