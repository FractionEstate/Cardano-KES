# Cardano KES - Pure Rust Implementation

[![Crates.io](https://img.shields.io/crates/v/cardano-kes.svg)](https://crates.io/crates/cardano-kes)
[![Documentation](https://docs.rs/cardano-kes/badge.svg)](https://docs.rs/cardano-kes)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.91%2B-orange.svg)](https://www.rust-lang.org)

**Pure Rust implementation of Cardano Key Evolving Signatures (KES)**

This crate provides a complete, production-ready implementation of Key Evolving Signature schemes used in the Cardano blockchain, extracted from `cardano-base-rust` and optimized as a standalone library.

## Features

- ✅ **Complete KES Implementation** - SingleKES, SumKES, CompactSingleKES, CompactSumKES
- ✅ **Binary Compatible** - Matches Haskell `cardano-crypto-class` implementation
- ✅ **No Standard Library Required** - `no_std` compatible with `alloc`
- ✅ **Zero Unsafe Code** - Pure safe Rust implementation
- ✅ **Comprehensive Tests** - Full test vector coverage from Cardano
- ✅ **Well Documented** - Complete API documentation and examples

## What is KES?

Key Evolving Signatures (KES) provide **forward security** - once a key evolves to a new period, it cannot sign for previous periods, even if compromised. This is critical for blockchain consensus where stake pool operators must protect against key theft.

### KES Families

| Algorithm | Periods | Use Case |
|-----------|---------|----------|
| `SingleKES` | 1 | Base case (wraps Ed25519) |
| `SumKES` | 2^n | Standard composition with full VK storage |
| `CompactSingleKES` | 1 | Base with embedded VK in signature |
| `CompactSumKES` | 2^n | Optimized composition (smaller signatures) |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cardano-kes = "0.1"
```

### Feature Flags

```toml
[dependencies]
cardano-kes = { version = "0.1", features = ["serde"] }
```

Available features:
- `std` (default) - Standard library support
- `serde` - Serialization support for key types
- `kes-metrics` - Lightweight metrics for benchmarking

For `no_std` environments:
```toml
[dependencies]
cardano-kes = { version = "0.1", default-features = false }
```

## Quick Start

```rust
use cardano_kes::*;

// Generate a signing key from a seed
let seed = vec![0u8; Sum2Kes::SEED_SIZE];
let mut signing_key = Sum2Kes::gen_key_kes_from_seed_bytes(&seed)?;
let verification_key = Sum2Kes::derive_verification_key(&signing_key)?;

// Sign a message at period 0
let message = b"Hello, Cardano!";
let period = 0;
let signature = Sum2Kes::sign_kes(&(), period, message, &signing_key)?;

// Verify the signature
Sum2Kes::verify_kes(&(), &verification_key, period, message, &signature)?;

// Evolve key to next period
signing_key = Sum2Kes::update_kes(&(), signing_key, period)?
    .expect("key is still valid");

// Key can now sign for period 1, but NOT period 0 (forward security)
```

## KES Algorithm Details

### Single-period KES (SingleKES)

The simplest KES - wraps Ed25519 DSIGN for a single period:

```rust
use cardano_kes::*;

let seed = vec![0u8; SingleKes::SEED_SIZE];
let signing_key = SingleKes::gen_key_kes_from_seed_bytes(&seed)?;
let vk = SingleKes::derive_verification_key(&signing_key)?;

// Only period 0 is valid
let sig = SingleKes::sign_kes(&(), 0, b"message", &signing_key)?;
SingleKes::verify_kes(&(), &vk, 0, b"message", &sig)?;
```

### Multi-period Sum KES

Binary tree composition supporting 2^n periods:

```rust
use cardano_kes::*;

// Sum2Kes = 2^2 = 4 periods (0, 1, 2, 3)
let seed = vec![0u8; Sum2Kes::SEED_SIZE];
let mut sk = Sum2Kes::gen_key_kes_from_seed_bytes(&seed)?;
let vk = Sum2Kes::derive_verification_key(&sk)?;

// Sign and evolve through all periods
for period in 0..Sum2Kes::total_periods() {
    let message = format!("Period {}", period);
    let sig = Sum2Kes::sign_kes(&(), period, message.as_bytes(), &sk)?;
    Sum2Kes::verify_kes(&(), &vk, period, message.as_bytes(), &sig)?;

    // Update for next period
    if period + 1 < Sum2Kes::total_periods() {
        sk = Sum2Kes::update_kes(&(), sk, period)?
            .expect("key still valid");
    }
}
```

### Compact Sum KES (Optimized)

More efficient signatures by embedding off-path verification keys:

```rust
use cardano_kes::*;

// CompactSum3Kes = 2^3 = 8 periods with smaller signatures
let seed = vec![0u8; CompactSum3Kes::SEED_SIZE];
let mut sk = CompactSum3Kes::gen_key_kes_from_seed_bytes(&seed)?;
let vk = CompactSum3Kes::derive_verification_key(&sk)?;

let sig = CompactSum3Kes::sign_kes(&(), 0, b"message", &sk)?;
CompactSum3Kes::verify_kes(&(), &vk, 0, b"message", &sig)?;
```

## Type Aliases

Pre-configured KES algorithms for different period counts:

```rust
// Sum family (using Blake2b-256)
pub type Sum0Kes = SingleKes<Ed25519>;           // 1 period
pub type Sum1Kes = SumKes<Sum0Kes, Blake2b256>;  // 2 periods
pub type Sum2Kes = SumKes<Sum1Kes, Blake2b256>;  // 4 periods
pub type Sum3Kes = SumKes<Sum2Kes, Blake2b256>;  // 8 periods
pub type Sum4Kes = SumKes<Sum3Kes, Blake2b256>;  // 16 periods
pub type Sum5Kes = SumKes<Sum4Kes, Blake2b256>;  // 32 periods
pub type Sum6Kes = SumKes<Sum5Kes, Blake2b256>;  // 64 periods
pub type Sum7Kes = SumKes<Sum6Kes, Blake2b256>;  // 128 periods

// Compact sum family (optimized)
pub type CompactSum0Kes = CompactSingleKes<Ed25519>;
pub type CompactSum1Kes = CompactSumKes<CompactSum0Kes, Blake2b256>;
// ... up to CompactSum7Kes
```

## Architecture

This crate is extracted from the Cardano ecosystem's Rust cryptography implementation:

```
cardano-kes/
├── src/
│   ├── lib.rs              # Main module and exports
│   ├── error.rs            # Error types
│   ├── period.rs           # Period type and utilities
│   ├── traits.rs           # Core KesAlgorithm trait
│   ├── single.rs           # SingleKES implementation
│   ├── compact_single.rs   # CompactSingleKES
│   ├── sum.rs              # SumKES family
│   ├── compact_sum.rs      # CompactSumKES family
│   ├── hash.rs             # Blake2b hash algorithms
│   └── metrics.rs          # Optional metrics
├── examples/
│   └── basic_usage.rs
└── tests/
    └── integration_tests.rs
```

## Testing

```bash
# Run all tests
cargo test

# Run with metrics enabled
cargo test --features kes-metrics

# Run specific test
cargo test single_kes_basic

# Run examples
cargo run --example basic_usage
```

## Binary Compatibility

This implementation maintains binary compatibility with Haskell's `cardano-crypto-class`:

- ✅ Verification key serialization matches byte-for-byte
- ✅ Signature format identical to Haskell implementation
- ✅ All official Cardano test vectors pass
- ✅ Hash algorithm (Blake2b-256) matches Haskell exactly

## Security Considerations

### Forward Security

Once a key evolves past a period, it **cannot** sign for that period:

```rust
let mut sk = Sum2Kes::gen_key_kes_from_seed_bytes(&seed)?;

// Sign for period 0
let sig0 = Sum2Kes::sign_kes(&(), 0, b"msg0", &sk)?;

// Evolve to period 1
sk = Sum2Kes::update_kes(&(), sk, 0)?.unwrap();

// ❌ Cannot sign for period 0 anymore!
// This will return an error
let result = Sum2Kes::sign_kes(&(), 0, b"msg0", &sk);
assert!(result.is_err());
```

### Key Zeroization

Signing keys are automatically zeroized when dropped:

```rust
{
    let sk = Sum2Kes::gen_key_kes_from_seed_bytes(&seed)?;
    // Use sk...
} // sk is zeroized here
```

### No Unsafe Code

This crate uses **zero unsafe code** - all operations are safe Rust.

## Related Crates

Part of the Cardano Rust ecosystem:

- [`cardano-vrf`](https://crates.io/crates/cardano-vrf) - Verifiable Random Functions
- `cardano-dsign` - Digital signatures (coming soon)
- `cardano-cbor` - CBOR encoding (coming soon)

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Build docs
cargo doc --open

# Run benches
cargo bench
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

This implementation is based on the Haskell `cardano-crypto-class` library and the academic paper:

> "Composition and Efficiency Tradeoffs for Forward-Secure Digital Signatures"
> by Tal Malkin, Daniele Micciancio, and Sara Miner
> https://eprint.iacr.org/2001/034

Special thanks to:
- The Cardano Foundation
- IOHK/Input Output
- The Rust Cardano community
