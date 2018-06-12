# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Update to the new specification with enforces the `pkg` scheme at the start
  of every package.

### Changed
- Replaced `scheme` field on `PackageUrl` with a field named `ty` (for type)
  as scheme now has a constant value of `pkg`.

## 0.1.0 - 2017-04-13
### Added
- The `PackageUrl` struct with parsing and serializing methods
- Test cases based on the PURL specification test suite.
- A small README file showing usecases and installation instructions.
- This CHANGELOG file.
