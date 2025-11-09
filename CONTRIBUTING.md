# Contributing to Cardano KES

Thank you for your interest in contributing to Cardano KES! This document provides guidelines for contributing to this project.

## Development Status

**Note:** This crate is currently being extracted from [`cardano-base-rust`](https://github.com/FractionEstate/cardano-base-rust). The extraction process is ongoing.

### Extraction Checklist

- [x] Project structure created
- [x] Core traits defined (`KesAlgorithm`)
- [x] Error types defined
- [x] Hash algorithms implemented (Blake2b)
- [ ] Extract SingleKES implementation
- [ ] Extract CompactSingleKES implementation
- [ ] Extract SumKES implementation
- [ ] Extract CompactSumKES implementation
- [ ] Add Ed25519/DSIGN dependency (or extract as separate crate)
- [ ] Extract test vectors
- [ ] Extract benchmarks
- [ ] Documentation review
- [ ] Publish to crates.io

## How to Contribute

### Reporting Issues

If you find a bug or have a feature request:

1. Check if an issue already exists
2. If not, create a new issue with:
   - Clear description of the problem/feature
   - Steps to reproduce (for bugs)
   - Expected vs actual behavior
   - Rust version and platform information

### Development Setup

```bash
# Clone the repository
git clone https://github.com/FractionEstate/Cardano-KES.git
cd Cardano-KES

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run with all features
cargo test --all-features

# Format code
cargo fmt

# Run clippy
cargo clippy --all-features
```

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Pass all clippy lints (`cargo clippy`)
- Write documentation for public APIs
- Add tests for new functionality
- Maintain `no_std` compatibility where possible

### Testing Requirements

- **Unit tests** for individual components
- **Integration tests** for end-to-end workflows
- **Test vectors** from Cardano Haskell implementation
- **Documentation examples** that compile and run

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting
5. Commit with clear messages
6. Push to your fork
7. Open a pull request with:
   - Clear description of changes
   - Reference to related issues
   - Test results

## Extraction Process

If you're helping with the extraction from `cardano-base-rust`:

### Priority Order

1. **Core implementations**
   - `single.rs` - SingleKES
   - `compact_single.rs` - CompactSingleKES
   - `sum.rs` - SumKES
   - `compact_sum.rs` - CompactSumKES

2. **Dependencies**
   - Ed25519 implementation (or extract DSIGN)
   - Seed generation utilities
   - MLocked memory handling (if needed)

3. **Testing**
   - Test vectors from `cardano-test-vectors`
   - Integration tests from `cardano-crypto-class/tests/kes_*.rs`
   - Benchmarks from `cardano-crypto-class/benches/kes_bench.rs`

### Extraction Guidelines

When extracting code from `cardano-base-rust`:

1. **Preserve functionality** - Keep the implementation identical
2. **Update imports** - Adjust module paths for new structure
3. **Maintain compatibility** - Keep binary compatibility with Haskell
4. **Add tests** - Include relevant test cases
5. **Document changes** - Update CHANGELOG.md

## Code Review Process

All contributions will be reviewed for:

- **Correctness** - Does it work as intended?
- **Compatibility** - Binary compatible with Haskell implementation?
- **Testing** - Adequate test coverage?
- **Documentation** - Clear API documentation?
- **Style** - Follows Rust conventions?

## License

By contributing, you agree that your contributions will be licensed under both:

- MIT License
- Apache License 2.0

## Getting Help

- Open an issue for questions
- Check existing documentation
- Review the Haskell implementation: [`cardano-crypto-class`](https://github.com/input-output-hk/cardano-base)

## Related Projects

- [`cardano-vrf`](https://github.com/FractionEstate/Cardano-VRF) - Verifiable Random Functions
- [`cardano-base-rust`](https://github.com/FractionEstate/cardano-base-rust) - Source repository

## Acknowledgments

This project is based on the Haskell `cardano-crypto-class` library and follows the academic paper:

> "Composition and Efficiency Tradeoffs for Forward-Secure Digital Signatures"
> by Tal Malkin, Daniele Micciancio, and Sara Miner
> https://eprint.iacr.org/2001/034
