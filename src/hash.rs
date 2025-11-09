//! Hash algorithms for KES constructions

use alloc::vec::Vec;

/// Trait for hash algorithms used in KES schemes
pub trait KesHashAlgorithm: Clone + Send + Sync + 'static {
    /// Output size in bytes
    const OUTPUT_SIZE: usize;

    /// Hash a single input
    fn hash(data: &[u8]) -> Vec<u8>;

    /// Hash two inputs concatenated
    fn hash_concat(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut combined = Vec::with_capacity(left.len() + right.len());
        combined.extend_from_slice(left);
        combined.extend_from_slice(right);
        Self::hash(&combined)
    }

    /// Expand seed into two seeds (for binary tree construction)
    ///
    /// Uses prefixes to match Haskell implementation:
    /// - r0 = hash(1 || seed)
    /// - r1 = hash(2 || seed)
    #[must_use]
    fn expand_seed(seed: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut left_input = Vec::with_capacity(1 + seed.len());
        left_input.push(1);
        left_input.extend_from_slice(seed);

        let mut right_input = Vec::with_capacity(1 + seed.len());
        right_input.push(2);
        right_input.extend_from_slice(seed);

        (Self::hash(&left_input), Self::hash(&right_input))
    }
}

/// Blake2b-224 (28-byte output)
#[derive(Clone, Debug)]
pub struct Blake2b224;

impl KesHashAlgorithm for Blake2b224 {
    const OUTPUT_SIZE: usize = 28;

    fn hash(data: &[u8]) -> Vec<u8> {
        use blake2::{Blake2b, Digest};
        use blake2::digest::consts::U28;

        let mut hasher = Blake2b::<U28>::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

/// Blake2b-256 (32-byte output) - Standard for Cardano KES
#[derive(Clone, Debug)]
pub struct Blake2b256;

impl KesHashAlgorithm for Blake2b256 {
    const OUTPUT_SIZE: usize = 32;

    fn hash(data: &[u8]) -> Vec<u8> {
        use blake2::{Blake2b256 as Blake2b256Hasher, Digest};

        let mut hasher = Blake2b256Hasher::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

/// Blake2b-512 (64-byte output)
#[derive(Clone, Debug)]
pub struct Blake2b512;

impl KesHashAlgorithm for Blake2b512 {
    const OUTPUT_SIZE: usize = 64;

    fn hash(data: &[u8]) -> Vec<u8> {
        use blake2::{Blake2b512 as Blake2b512Hasher, Digest};

        let mut hasher = Blake2b512Hasher::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake2b256_output_size() {
        let hash = Blake2b256::hash(b"test");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_seed_expansion() {
        let seed = b"test_seed";
        let (r0, r1) = Blake2b256::expand_seed(seed);

        assert_eq!(r0.len(), 32);
        assert_eq!(r1.len(), 32);
        assert_ne!(r0, r1); // Different hashes
    }
}
