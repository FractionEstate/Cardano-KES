//! # Cardano KES - Key Evolving Signatures
//!
//! Pure Rust implementation of Key Evolving Signature schemes used in the Cardano blockchain.
//!
//! ## What is KES?
//!
//! Key Evolving Signatures (KES) provide **forward security** through irreversible key evolution.
//! Once a signing key evolves to period N, it cannot sign for any period < N, even if compromised.
//!
//! This is critical for blockchain consensus where validators must:
//! - Sign blocks across many periods (epochs/slots)
//! - Protect against future key compromise affecting historical signatures
//! - Minimize damage from key theft
//!
//! ## Quick Example
//!
//! ```rust
//! use cardano_kes::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Generate a Sum2KES key (supports 4 periods: 0, 1, 2, 3)
//! let seed = vec![0u8; Sum2Kes::SEED_SIZE];
//! let mut signing_key = Sum2Kes::gen_key_kes_from_seed_bytes(&seed)?;
//! let verification_key = Sum2Kes::derive_verification_key(&signing_key)?;
//!
//! // Sign for period 0
//! let message = b"Block at period 0";
//! let signature = Sum2Kes::sign_kes(&(), 0, message, &signing_key)?;
//!
//! // Verify signature
//! Sum2Kes::verify_kes(&(), &verification_key, 0, message, &signature)?;
//!
//! // Evolve key to period 1
//! signing_key = Sum2Kes::update_kes(&(), signing_key, 0)?
//!     .expect("key is still valid");
//!
//! // ✓ Can sign for period 1
//! let sig1 = Sum2Kes::sign_kes(&(), 1, b"Block at period 1", &signing_key)?;
//!
//! // ✗ Cannot sign for period 0 anymore (forward security!)
//! assert!(Sum2Kes::sign_kes(&(), 0, message, &signing_key).is_err());
//! # Ok(())
//! # }
//! ```
//!
//! ## KES Algorithm Families
//!
//! ### SingleKES
//!
//! The simplest KES - a single-period wrapper around Ed25519:
//!
//! ```rust
//! use cardano_kes::*;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let seed = vec![0u8; SingleKes::SEED_SIZE];
//! let sk = SingleKes::gen_key_kes_from_seed_bytes(&seed)?;
//! let vk = SingleKes::derive_verification_key(&sk)?;
//!
//! // Only period 0 is valid
//! let sig = SingleKes::sign_kes(&(), 0, b"message", &sk)?;
//! SingleKes::verify_kes(&(), &vk, 0, b"message", &sig)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### SumKES
//!
//! Binary tree composition supporting 2^n periods:
//!
//! ```rust
//! use cardano_kes::*;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Sum3Kes = 2^3 = 8 periods
//! let seed = vec![0u8; Sum3Kes::SEED_SIZE];
//! let mut sk = Sum3Kes::gen_key_kes_from_seed_bytes(&seed)?;
//! let vk = Sum3Kes::derive_verification_key(&sk)?;
//!
//! // Evolve through periods
//! for period in 0..Sum3Kes::total_periods() {
//!     let message = format!("Period {}", period);
//!     let sig = Sum3Kes::sign_kes(&(), period, message.as_bytes(), &sk)?;
//!     Sum3Kes::verify_kes(&(), &vk, period, message.as_bytes(), &sig)?;
//!
//!     if period + 1 < Sum3Kes::total_periods() {
//!         sk = Sum3Kes::update_kes(&(), sk, period)?.expect("key valid");
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### CompactSumKES
//!
//! Optimized variant with smaller signatures (embeds verification keys):
//!
//! ```rust
//! use cardano_kes::*;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let seed = vec![0u8; CompactSum3Kes::SEED_SIZE];
//! let sk = CompactSum3Kes::gen_key_kes_from_seed_bytes(&seed)?;
//! let vk = CompactSum3Kes::derive_verification_key(&sk)?;
//!
//! let sig = CompactSum3Kes::sign_kes(&(), 0, b"message", &sk)?;
//! CompactSum3Kes::verify_kes(&(), &vk, 0, b"message", &sig)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **`std`** (default) - Standard library support
//! - **`serde`** - Serialization for keys and signatures
//! - **`kes-metrics`** - Lightweight performance metrics
//!
//! ## Binary Compatibility
//!
//! This implementation maintains byte-level compatibility with Haskell's `cardano-crypto-class`:
//!
//! - Verification keys serialize identically
//! - Signatures are binary-compatible
//! - Hash algorithm (Blake2b-256) matches exactly
//! - All official Cardano test vectors pass
//!
//! ## Security
//!
//! - ✅ Zero unsafe code
//! - ✅ Automatic key zeroization on drop
//! - ✅ Forward security guarantees
//! - ✅ Constant-time operations where applicable
//!
//! ## Module Organization
//!
//! ```text
//! cardano_kes
//! ├── error           Error types (KesError, KesMError)
//! ├── traits          Core KesAlgorithm trait
//! ├── single          SingleKES implementation
//! ├── compact_single  CompactSingleKES
//! ├── sum             SumKES family
//! ├── compact_sum     CompactSumKES family
//! ├── hash            Blake2b hash algorithms
//! └── metrics         Optional performance metrics
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

extern crate alloc;

// Core modules
pub mod error;
pub mod hash;
pub mod metrics;
pub mod traits;

// KES implementations
pub mod compact_single;
pub mod compact_sum;
pub mod single;
pub mod sum;

// Re-exports for convenience
pub use error::{KesError, KesMError};
pub use hash::{Blake2b224, Blake2b256, Blake2b512, KesHashAlgorithm};
pub use traits::{KesAlgorithm, Period, UnsoundKesAlgorithm};

// Re-export SingleKes
pub use single::SingleKes;

// Re-export CompactSingleKes
pub use compact_single::{CompactSingleKes, CompactSingleSig, OptimizedKesSignature};

// Re-export Sum type aliases (using Blake2b256)
pub use sum::{Sum0Kes, Sum1Kes, Sum2Kes, Sum3Kes, Sum4Kes, Sum5Kes, Sum6Kes, Sum7Kes};

// Re-export CompactSum type aliases (using Blake2b256)
pub use compact_sum::{
    CompactSum0Kes, CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, CompactSum4Kes, CompactSum5Kes,
    CompactSum6Kes, CompactSum7Kes,
};
