use serde_json;

#[macro_use]
mod macros;

static TEST_DATA: &[u8] = include_bytes!("test-suite-data.json");

fn find_test_case(raw_name: &str) -> Option<serde_json::Value> {
    if let Ok(serde_json::Value::Array(v)) = serde_json::from_slice(TEST_DATA) {
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

spec_parse_test!(valid_maven_purl);
spec_parse_test!(basic_valid_maven_purl_without_version);
spec_parse_test!(valid_go_purl_without_version_and_with_subpath);
spec_parse_test!(valid_go_purl_with_version_and_subpath);
spec_parse_test!(bitbucket_namespace_and_name_should_be_lowercased);
spec_parse_test!(github_namespace_and_name_should_be_lowercased);
spec_parse_test!(debian_can_use_qualifiers);
spec_parse_test!(docker_uses_qualifiers_and_hash_image_id_as_versions);
spec_parse_test!(gem_uses_qualifiers);
spec_parse_test!(maven_uses_qualifiers);
spec_parse_test!(maven_pom_reference);
spec_parse_test!(maven_uses_type);
spec_parse_test!(npm_can_be_scoped);
spec_parse_test!(nuget_names_are_case_sensitive);
spec_parse_test!(pypi_names_have_special_rules_and_not_case_sensitive);
spec_parse_test!(rpm_use_qualifiers);
spec_parse_test!(type_is_required);
spec_parse_test!(name_is_required);
spec_parse_test!(slash_after_type_is_not_significant);
spec_parse_test!(double_slash_after_type_is_not_significant);
