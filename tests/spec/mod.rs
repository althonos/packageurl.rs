#![cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
mod macros;
mod testcase;

spec_tests!(type_required, "a type is always required");
spec_tests!(scheme_required, "a scheme is always required");
spec_tests!(name_required, "a name is required");
spec_tests!(invalid_qualifier_key, "checks for invalid qualifier keys");
spec_tests!(gem, "Java gem can use a qualifier");
spec_tests!(npm, "npm can be scoped");
spec_tests!(rpm, "rpm often use qualifiers");
spec_tests!(nuget, "nuget names are case sensitive");
spec_tests!(pypi, "pypi names have special rules and not case sensitive");
spec_tests!(debian, "debian can use qualifiers");
spec_tests!(bitbucket, "bitbucket namespace and name should be lowercased");
spec_tests!(github, "github namespace and name should be lowercased");
spec_tests!(docker, "docker uses qualifiers and hash image id as versions");
spec_tests!(maven, "valid maven purl");
spec_tests!(maven_basic, "basic valid maven purl without version");
spec_tests!(maven_case_sensitive, "valid maven purl with case sensitive namespace and name");
spec_tests!(maven_space, "valid maven purl containing a space in the version and qualifier");
spec_tests!(go_subpath, "valid go purl without version and with subpath");
spec_tests!(go_version, "valid go purl with version and subpath");
spec_tests!(maven_qualifiers, "maven often uses qualifiers");
spec_tests!(maven_pom, "maven pom reference");
spec_tests!(maven_type, "maven can come with a type qualifier");
spec_tests!(simple_slash, "slash / after scheme is not significant");
spec_tests!(double_slash, "double slash // after scheme is not significant");
spec_tests!(triple_slash, "slash /// after type  is not significant");
