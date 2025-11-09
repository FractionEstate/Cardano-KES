# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure
- Core `KesAlgorithm` trait definition
- Error types (`KesError`, `KesMError`)
- Blake2b hash algorithms (224, 256, 512-bit)
- Metrics module (feature-gated)
- Comprehensive documentation
- README with examples
- Contributing guidelines

### TODO
- Extract SingleKES implementation from cardano-base-rust
- Extract CompactSingleKES implementation
- Extract SumKES implementation
- Extract CompactSumKES implementation
- Add Ed25519/DSIGN dependency
- Add test vectors
- Add benchmarks
- Publish to crates.io

## [0.1.0] - TBD

### Added
- Initial release (pending extraction completion)
- Full KES implementation compatible with Cardano
- `no_std` support
- Comprehensive test coverage
- Binary compatibility with Haskell implementation

[Unreleased]: https://github.com/FractionEstate/Cardano-KES/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/FractionEstate/Cardano-KES/releases/tag/v0.1.0
