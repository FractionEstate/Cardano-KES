//! CompactSingleKES - Single-period KES with embedded verification key
//!
//! Used as base case for CompactSumKES composition

// TODO: Extract from cardano-base-rust/cardano-crypto-class/src/kes/compact_single.rs

/// Trait for signatures that embed verification keys
pub trait OptimizedKesSignature {
    /// Verification key type
    type VerificationKey;

    /// Extract the embedded verification key from the signature
    fn embedded_verification_key(&self) -> &Self::VerificationKey;
}

/// CompactSingleKES signature structure
pub struct CompactSingleSig<D> {
    _phantom: core::marker::PhantomData<D>,
}

/// CompactSingleKES structure
pub struct CompactSingleKes<D> {
    _phantom: core::marker::PhantomData<D>,
}

// Implementation will be extracted from cardano-base-rust
