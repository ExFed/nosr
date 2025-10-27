# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-10-26

### Added

- Initial implementation of nosr parser in Rust
- Core modules: `span`, `error`, `lexer`, `node`, `parser`
- API functions: `document`, `table`, `vector`, `text`, `uint64`, `double`
- Support for tables, vectors, and scalars
- Line and block comment support
- String escape sequences
- Lazy parsing with zero-copy string handling
- Comprehensive test suite (33 tests)
- Documentation and examples
- README and usage guide

[Unreleased]: https://github.com/ExFed/nosr/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ExFed/nosr/releases/tag/v0.1.0
