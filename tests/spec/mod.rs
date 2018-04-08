#[macro_use]
mod macros;
mod utils;

static TEST_DATA: &[u8] = include_bytes!("test-suite-data.json");

spec_tests!(type_required, "type is required");
spec_tests!(name_required, "name is required");
spec_tests!(gem, "gem uses qualifiers");
spec_tests!(npm, "npm can be scoped");
spec_tests!(rpm, "rpm use qualifiers");
spec_tests!(nuget, "nuget names are case sensitive");
spec_tests!(pypi, "pypi names have special rules and not case sensitive");
spec_tests!(debian, "debian can use qualifiers");
spec_tests!(bitbucket, "bitbucket namespace and name should be lowercased");
spec_tests!(github, "github namespace and name should be lowercased");
spec_tests!(docker, "docker uses qualifiers and hash image id as versions");
spec_tests!(maven, "valid maven purl");
spec_tests!(maven_basic, "basic valid maven purl without version");
spec_tests!(go_subpath, "valid go purl without version and with subpath");
spec_tests!(go_version, "valid go purl with version and subpath");
spec_tests!(maven_qualifiers, "maven uses qualifiers");
spec_tests!(maven_pom, "maven pom reference");
spec_tests!(maven_type, "maven uses type");
spec_tests!(slash, "slash after type is not significant");
spec_tests!(double_slash, "double slash after type is not significant");
