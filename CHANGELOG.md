# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-05

### Added

- Initial release of tuplities, a collection of utilities for working with tuples in Rust.
- Support for various tuple operations: clone, copy, debug, default, eq, hash, index, insert, len, mut, option, ord, partial_eq, partial_ord, pop_back, pop_front, push_back, push_front, ref, remove.
- Procedural macro for generating trait implementations for tuples up to size 128.
- Feature flags for controlling maximum supported tuple size (default: 8, up to 128).
- No-std compatible.
