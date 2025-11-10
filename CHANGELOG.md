# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-11-10

### Added
- **Complete VRF Implementation**
  - IETF VRF Draft-03 (ECVRF-ED25519-SHA512-Elligator2)
  - IETF VRF Draft-13 (ECVRF-ED25519-SHA512-TAI)
  - Cardano-compatible hash-to-curve using Elligator2
  - Full prove/verify/proof_to_hash APIs
  - XMD expansion (expand_message_xmd) for Draft-13
  - 8 VRF golden tests passing

- **Complete KES Implementation**
  - SingleKES (1 period - base case)
  - Sum2KES (4 periods - binary tree composition)
  - Sum6KES (64 periods - Cardano standard for stake pools)
  - CompactSingleKES, CompactSum2KES, CompactSum6KES (optimized signatures)
  - Trait-based KesAlgorithm API with proper error handling
  - Forward-secure key evolution with move semantics
  - Type-safe period management
  - 9 KES golden tests passing

- **Digital Signatures (DSIGN)**
  - Ed25519 signature scheme with full trait implementation
  - Deterministic key generation from seed
  - Full sign/verify/keygen API

- **Hash Functions**
  - Blake2b (224, 256, 512-bit variants)
  - SHA-256, SHA-512, SHA-256d
  - RIPEMD-160
  - Keccak-256

- **CBOR Serialization**
  - Full CBOR encoding/decoding for all cryptographic types
  - Cardano-compatible binary format

- **no_std Support**
  - Works in embedded and WebAssembly environments
  - Optional `alloc` feature for heap allocation
  - No standard library required

- **Comprehensive Testing**
  - 95 library unit tests
  - 9 KES golden tests
  - 8 VRF golden tests
  - 4 documentation tests
  - 100% test pass rate (112/112 tests)
  - Zero compiler warnings
  - Zero clippy warnings

- **Documentation & Examples**
  - Complete API documentation with examples
  - `kes_lifecycle`: Complete KES demonstration
  - `dsign_sign_verify`: Digital signature examples
  - `vrf_basic`: VRF usage patterns

### Features
- `default`: Enables std, thiserror, vrf, kes, dsign, hash
- `std`: Standard library support
- `alloc`: Heap allocation support (for no_std)
- `vrf`: Verifiable Random Functions
- `kes`: Key Evolving Signatures
- `dsign`: Digital Signatures
- `hash`: Hash functions
- `cbor`: CBOR serialization support
- `serde`: Serde serialization support
- Component-specific features for selective compilation

### Security
- Zero unsafe code - Pure safe Rust implementation
- Forward-secure key evolution for KES
- Constant-time operations where applicable
- Comprehensive test coverage
- Security audit ready

### Compatibility
- Rust 1.91.0 or later (MSRV)
- 100% compatible with cardano-node cryptographic primitives
- Binary-compatible with Haskell cardano-crypto-class
- Sum6KES matches Cardano stake pool requirements (64 periods, ~90 days)
- VRF algorithms match IntersectMBO/cardano-base

[Unreleased]: https://github.com/FractionEstate/Cardano-KES/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/FractionEstate/Cardano-KES/releases/tag/v0.1.0

