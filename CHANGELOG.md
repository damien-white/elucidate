# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Parsers for extracting the `number` data type into `Integer` and `Float` types.
    - JSON makes no distinction between signed, unsigned and float values.
    - Rust treats these types differently, so `Integer` and `Float` are handled separately.
- Full test coverage for all existing parser functions.
- Helper function (currently within the test module) for creating `nom` errors.
- Custom Rust compiler flags including platform-specific linker configurations.

### Changed

- Modified `Value` enum members to reflect the currently supported JSON data types.
- Crate-level documentation to accurately reflect the purpose of the library.
- Made the README more succinct, improving readability and clarity.

### Fixed

- The `unsigned_integer` parser is working as intended.
    - Source: https://docs.rs/nom/latest/nom/character/streaming/fn.digit0.html

### Removed

- Nightly toolchain docs generation; `cargo doc` now uses the latest stable toolchain.
- "assets" directory containing artifacts from a different project.

## [0.1.0] - 2021-05-06

### Added

- First release.
- Project template with basic CI/CD workflows and project README.
- Documentation targeted at end-users for interacting with repository.
    - Includes topics such as how to open new issues, request features, and report bugs.

<!-- Types of changes -->
<!--
- Added:        for new features
- Changed:      for changes in existing functionality
- Deprecated:   for soon-to-be removed features
- Removed:      for now removed features
- Fixed:        for any bug fixes
- Security:     in case of vulnerabilities
-->
