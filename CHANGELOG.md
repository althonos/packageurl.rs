# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
[Unreleased]: https://github.com/althonos/packageurl.rs/compare/0.3.0...HEAD


## [0.3.0] - 2021-07-02
[0.3.0] https://github.com/althonos/packageurl.rs/compare/0.2.0...0.3.0

### Added
- Optional `Serialize` and `Deserialize` implementations when the `serde` feature is enabled. Closes [#3](https://github.com/althonos/packageurl.rs/issues/3).
- `Display` implementation for `PackageUrl`. Closes [#2](https://github.com/althonos/packageurl.rs/issues/2).
- `PartialEq` implementation for `PackageUrl`.
- `std::error::Error` trait implementation for the `packageurl::Error` type.

### Changed
- `PackageUrl` does not provide direct access to its fields anymore, only through read-only getters.
- Some `PackageUrl` methods now return a result in case the argument fails to pass validation.

### Fixed
- Qualifier keys are now properly validated and rejected when they contain invalid characters.


## [0.2.0] - 2018-06-14
[0.2.0]: https://github.com/althonos/packageurl.rs/compare/0.1.0...0.2.0

### Fixed
- Update to the new specification with enforces the `pkg` scheme at the start of every package.

### Changed
- Replaced `scheme` field on `PackageUrl` with a field named `ty` (for type) as scheme now has a constant value of `pkg`.

## [0.1.0] - 2017-04-13
[0.1.0]: https://github.com/althonos/packageurl.rs/compare/f61ab5c...0.1.0

### Added
- The `PackageUrl` struct with parsing and serializing methods
- Test cases based on the PURL specification test suite.
- A small README file showing usecases and installation instructions.
- This CHANGELOG file.
