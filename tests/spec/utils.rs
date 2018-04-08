use std::borrow::Cow;
use std::collections::HashMap;


pub fn find_test_case(raw_name: &str) -> Option<::serde_json::Value> {
    if let Ok(::serde_json::Value::Array(v)) = ::serde_json::from_slice(super::TEST_DATA) {
        v.into_iter().find(|x| {
            x["description"]
                .as_str()
                .unwrap_or("")
                .replace("/", "")
                .replace("  ", " ")
                .eq(raw_name)
        })
    } else {
        None
    }
}


pub fn assert_map_eq<'a>(
    actual: &HashMap<Cow<'a, str>, Cow<'a, str>>,
    expected: &::serde_json::Value,
) {

    let empty = ::serde_json::Map::new();
    let a = actual;
    let e = expected.as_object().unwrap_or(&empty);

    let keys_a: Vec<&str> = a.keys().map(|x| x.as_ref()).collect();
    let keys_e: Vec<&str> = e.keys().map(|x| x.as_ref()).collect();

    for k in keys_a.iter() {
        assert!(keys_e.contains(k), "unexpected key: '{}'", k);
    }

    for ref k in keys_e.iter() {
        assert!(keys_a.contains(k), "missing key: '{}'", k);
    }

    for k in keys_a.iter().map(|k| k.to_owned()) {
        let val_a = a.get(&k[..]).unwrap().as_ref();
        let val_e = e.get(&k[..]).unwrap().as_str().unwrap();
        assert_eq!(val_a, val_e, "values for key '{}' differ", &k);
    }

}
