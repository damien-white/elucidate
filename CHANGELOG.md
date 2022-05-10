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
- Cargo configuration file with Rust compiler flags.
    - Includes platform-specific linker configurations.
- Assets directory containing a project logo in SVG and PNG format.
- Reference to the full JSON grammar to consolidate the syntax rules and enhance documentation.

### Changed

- Modified `Value` enum members to reflect the currently supported JSON data types.
- Use `recognize_float` to greatly simplify the `float` parser logic.
- Crate-level documentation to accurately reflect the purpose of the library.
- The README is more succinct in some places and more expansive in others.
    - The project README should be clear and readable.
- Functions are now private instead of public within their crate where possible.

### Fixed

- The `unsigned_integer` parser is working as intended.
    - Source: https://docs.rs/nom/latest/nom/character/streaming/fn.digit0.html
- Inaccurate documentation for the non-zero integer function.

### Removed

- Nightly toolchain docs generation; `cargo doc` now uses the latest stable toolchain.
- "assets" directory containing artifacts from a different project.
- Attribution section from README until we find a better place for it.

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
