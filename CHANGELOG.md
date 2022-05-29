# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added top-level `json_value` function to parse arbitrary JSON values into Rust types.
- Unified `number` data type and parser that includes integer and floating point values.
    - JSON makes no distinction between signed, unsigned and float values.
- Full test coverage for all existing parser functions.
- Helper function (currently within the test module) for creating `nom` errors.
- Cargo configuration file with Rust compiler flags.
    - Includes platform-specific linker configurations.
- Assets directory containing a project logo in SVG and PNG format.
- Reference to the full JSON grammar to consolidate the syntax rules and enhance documentation.
- `string` type parsing utilities such as those found in `src/parser/util.rs`
    - The specialized `whitespace0` combinator

### Changed

- `Value` enum members to reflect the currently supported JSON data types.
- Use `recognize_float` to greatly simplify the `float` parser logic.
- Crate-level documentation to accurately reflect the purpose of the library.
- The README is more succinct in some places and more expansive in others.
    - The project README should be clear and readable.
- Functions are now private instead of public within their crate where possible.
- Project structure changed to give `parser` its own submodule.
    - Top-level parsers for handling JSON data types.
    - Helper functions, modified and new parser combinators for creating domain-specific parsers.

### Fixed

- Added missing test coverage for the `json_value` parser.
- Numbers are now parsed correctly as per the official JSON specification (RFC 8259).

### Removed

- Nightly toolchain docs generation; `cargo doc` now uses the latest stable toolchain.
- "assets" directory containing artifacts from a different project.
- Attribution section from README until we find a better place for it.
- Support for `Integer` and `Float` values; Support now matches JSON's `Number` type.
    - The `Number` type is represented as a 64-bit floating point.
- The `unsigned_integer`, and `integer` parsers.
- Support for experimental `mold` linker for Linux (macOS support is buggy)

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
