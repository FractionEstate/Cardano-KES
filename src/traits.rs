//! Core KES algorithm trait

use crate::error::{KesError, KesMError};

/// The KES period type (0-indexed)
pub type Period = u64;

/// Core trait for Key Evolving Signature algorithms
///
/// KES provides forward security through irreversible key evolution.
/// Once a key evolves past period N, it cannot sign for periods < N.
///
/// # Example
///
/// ```rust
/// use cardano_kes::*;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let seed = vec![0u8; Sum2Kes::SEED_SIZE];
/// let mut sk = Sum2Kes::gen_key_kes_from_seed_bytes(&seed)?;
/// let vk = Sum2Kes::derive_verification_key(&sk)?;
///
/// // Sign for period 0
/// let sig0 = Sum2Kes::sign_kes(&(), 0, b"msg0", &sk)?;
/// Sum2Kes::verify_kes(&(), &vk, 0, b"msg0", &sig0)?;
///
/// // Evolve to period 1
/// sk = Sum2Kes::update_kes(&(), sk, 0)?.expect("key still valid");
///
/// // Can sign for period 1
/// let sig1 = Sum2Kes::sign_kes(&(), 1, b"msg1", &sk)?;
///
/// // Cannot sign for period 0 anymore
/// assert!(Sum2Kes::sign_kes(&(), 0, b"msg0", &sk).is_err());
/// # Ok(())
/// # }
/// ```
pub trait KesAlgorithm: Sized {
    /// Verification key type
    type VerificationKey;

    /// Signing key type
    type SigningKey;

    /// Signature type
    type Signature;

    /// Optional context parameter
    type Context;

    /// Algorithm name
    const ALGORITHM_NAME: &'static str;

    /// Required seed size in bytes
    const SEED_SIZE: usize;

    /// Verification key size in bytes
    const VERIFICATION_KEY_SIZE: usize;

    /// Signing key size in bytes
    const SIGNING_KEY_SIZE: usize;

    /// Signature size in bytes
    const SIGNATURE_SIZE: usize;

    /// Total number of periods this KES scheme supports
    fn total_periods() -> Period;

    /// Derive verification key from signing key
    ///
    /// # Errors
    ///
    /// Returns an error if the signing key is invalid
    fn derive_verification_key(
        signing_key: &Self::SigningKey,
    ) -> Result<Self::VerificationKey, KesMError>;

    /// Sign a message at a specific period
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The period is out of range
    /// - The key has expired
    /// - The key cannot sign for this period (already evolved past it)
    fn sign_kes(
        context: &Self::Context,
        period: Period,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> Result<Self::Signature, KesMError>;

    /// Verify a KES signature
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails
    fn verify_kes(
        context: &Self::Context,
        verification_key: &Self::VerificationKey,
        period: Period,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<(), KesError>;

    /// Evolve the signing key to the next period
    ///
    /// Returns `None` if the key has expired (reached max period).
    ///
    /// # Errors
    ///
    /// Returns an error if the key evolution fails
    fn update_kes(
        context: &Self::Context,
        signing_key: Self::SigningKey,
        period: Period,
    ) -> Result<Option<Self::SigningKey>, KesMError>;

    /// Generate a signing key from seed bytes
    ///
    /// # Errors
    ///
    /// Returns an error if the seed is invalid
    fn gen_key_kes_from_seed_bytes(seed: &[u8]) -> Result<Self::SigningKey, KesMError>;

    /// Serialize verification key
    fn raw_serialize_verification_key_kes(key: &Self::VerificationKey) -> alloc::vec::Vec<u8>;

    /// Deserialize verification key
    fn raw_deserialize_verification_key_kes(bytes: &[u8]) -> Option<Self::VerificationKey>;

    /// Serialize signature
    fn raw_serialize_signature_kes(signature: &Self::Signature) -> alloc::vec::Vec<u8>;

    /// Deserialize signature
    fn raw_deserialize_signature_kes(bytes: &[u8]) -> Option<Self::Signature>;

    /// Securely forget/zeroize a signing key
    fn forget_signing_key_kes(signing_key: Self::SigningKey);
}

/// Trait for unsound KES operations (testing/vector generation only)
///
/// Production code should never serialize signing keys!
pub trait UnsoundKesAlgorithm: KesAlgorithm {
    /// Serialize signing key (UNSAFE - for testing only!)
    fn raw_serialize_signing_key_kes(key: &Self::SigningKey) -> alloc::vec::Vec<u8>;

    /// Deserialize signing key (UNSAFE - for testing only!)
    fn raw_deserialize_signing_key_kes(bytes: &[u8]) -> Option<Self::SigningKey>;
}
