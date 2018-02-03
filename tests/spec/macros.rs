macro_rules! assert_map_eq {
    ($actual: expr, $expected: expr) => ({

        let empty = serde_json::Map::new();

        let a = $actual;
        let e = $expected.as_object().unwrap_or(&empty);

        let keys_a: Vec<&str> = a.keys().map(|x| x.as_ref()).collect();
        let keys_e: Vec<&str> = e.keys().map(|x| x.as_ref()).collect();

        for k in keys_a.iter() {
            assert!(keys_e.contains(k), "unexpected key: '{}'", k);
        }

        for ref k in keys_e.iter() {
            assert!(keys_a.contains(k), "missing key: '{}", k);
        }

        for k in keys_a.iter().map(|k| k.to_string()) {
            let val_a = a.get(&k[..]).unwrap().as_ref();
            let val_e = e.get(&k).unwrap().as_str().unwrap();
            assert_eq!(val_a, val_e, "values for key '{}' differ", &k);
        }


    })
}

macro_rules! spec_parse_test {
    ($name:ident) => {

        #[test]
        fn $name() {
            use std::str::FromStr;
            use packageurl::PackageUrl;

            let data_name = stringify!($name).replace("_", " ");
            let test_case = find_test_case(&data_name).unwrap();

            if let Some(u) = test_case["purl"].as_str() {
                if let Ok(purl) = PackageUrl::from_str(u) {

                    assert!(!test_case["is_invalid"].as_bool().unwrap());

                    assert_eq!(purl.scheme, test_case["type"].as_str().unwrap());
                    match purl.namespace {
                        Some(s) => assert_eq!(s, test_case["namespace"].as_str().unwrap()),
                        None => assert!(test_case["namespace"].is_null()),
                    };
                    assert_eq!(purl.name, test_case["name"].as_str().unwrap());
                    match purl.version {
                        Some(s) => assert_eq!(s, test_case["version"].as_str().unwrap()),
                        None => assert!(test_case["version"].is_null()),
                    };
                    assert_map_eq!(purl.qualifiers, test_case["qualifiers"]);
                    match purl.subpath {
                        Some(s) => assert_eq!(s, test_case["subpath"].as_str().unwrap()),
                        None => assert!(test_case["subpath"].is_null()),
                    };
                } else {
                    assert!(test_case["is_invalid"].as_bool().unwrap());
                }
            } else {
                assert!(false, "parsing error in {}", stringify!($name))
            }
        }

    }
}
