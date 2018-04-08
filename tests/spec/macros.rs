macro_rules! spec_tests {
    ($name:ident, $desc: expr) => {

        mod $name {

            use ::std::str::FromStr;
            use ::packageurl::PackageUrl;

            #[test]
            fn purl_to_components() {
                let test_case = super::utils::find_test_case($desc).unwrap();
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

                        super::utils::assert_map_eq(&purl.qualifiers, &test_case["qualifiers"]);

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

            // #[test]
            // fn components_to_canonical() {
            //
            // }
            //
            // #[test]
            // fn canonical_to_canonical() {
            //
            // }
            //
            // #[test]
            // fn purl_to_canonical() {
            //
            // }

        }

    }
}
