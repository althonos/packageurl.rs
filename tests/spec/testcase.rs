use std::borrow::Cow;
use std::collections::HashMap;

// May 20, 2019
static TEST_SUITE: &[u8] = include_bytes!("test-suite-data.json");

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SpecTestCase<'a> {
    pub description: Cow<'a, str>,
    pub purl: Cow<'a, str>,
    pub canonical_purl: Option<Cow<'a, str>>,
    #[serde(rename = "type")]
    pub ty: Option<Cow<'a, str>>,
    pub namespace: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub version: Option<Cow<'a, str>>,
    pub qualifiers: Option<HashMap<Cow<'a, str>, Cow<'a, str>>>,
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
            unreachable!("invalid json file")
        }
    }
}
