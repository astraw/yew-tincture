# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0](https://github.com/astraw/yew-tincture/compare/0.3.0...0.4.0) - 2026-11-14

### Added
- implement Debug for public types and document the public API
- *(typed-input)* expose RawAndParsed contents and accept FnOnce in modify

### Changed
- [**breaking**] move to Rust 2024 edition and require Rust 1.100

### Dependencies
- *(deps)* [**breaking**] update yew to 0.23
- *(deps)* require current uuid and web-sys releases

### Documentation
- describe the components, yew compatibility and MSRV in the README

### Fixed
- *(typed-input)* update the text in modify() even when no input is mounted
- *(checkbox-label)* honor the updating_checked property again

## [0.3.0] - 2025-12-21

### Dependencies
- update yew to 0.22

[0.3.0]: https://github.com/astraw/yew-tincture/releases/tag/0.3.0
