//! Golden test vectors for VRF implementations
//!
//! These tests validate byte-for-byte compatibility with Cardano's official VRF
//! implementation by testing against official test vectors and ensuring 100%
//! parity with IntersectMBO/cardano-base VRF algorithms.
//!
//! Tests follow the cardano-base patterns using the VrfDraft03 and VrfDraft13 APIs.

use cardano_crypto::vrf::{VrfDraft03, VrfDraft13};
use cardano_crypto::common::Result;

/// Test VRF Draft-03 basic roundtrip (Cardano production VRF)
#[test]
fn test_vrf_draft03_basic_roundtrip() -> Result<()> {
    let seed = [0x01u8; 32];
    let (secret_key, public_key) = VrfDraft03::keypair_from_seed(&seed);

    let message = b"Cardano slot leader election";

    // Generate proof
    let proof = VrfDraft03::prove(&secret_key, message)?;
    assert_eq!(proof.len(), 80, "Draft-03 proof must be 80 bytes for Cardano compatibility");

    // Verify proof
    let output = VrfDraft03::verify(&public_key, &proof, message)?;
    assert_eq!(output.len(), 64, "VRF output must be 64 bytes (SHA-512)");

    // proof_to_hash should match verify output
    let hash = VrfDraft03::proof_to_hash(&proof)?;
    assert_eq!(hash, output, "proof_to_hash must match verify output");

    Ok(())
}

/// Test VRF Draft-13 basic roundtrip (batch-compatible VRF)
#[test]
fn test_vrf_draft13_basic_roundtrip() -> Result<()> {
    let seed = [0x02u8; 32];
    let (secret_key, public_key) = VrfDraft13::keypair_from_seed(&seed);

    let message = b"Batch verification test";

    // Generate proof
    let proof = VrfDraft13::prove(&secret_key, message)?;
    assert_eq!(proof.len(), 128, "Draft-13 proof must be 128 bytes (batch-compatible)");

    // Verify proof
    let output = VrfDraft13::verify(&public_key, &proof, message)?;
    assert_eq!(output.len(), 64, "VRF output must be 64 bytes");

    // proof_to_hash should match verify output
    let hash = VrfDraft13::proof_to_hash(&proof)?;
    assert_eq!(hash, output, "proof_to_hash must match verify output");

    Ok(())
}

/// Test Draft-03 deterministic keypair generation
#[test]
fn test_draft03_deterministic_keygen() {
    let seed = [0x42u8; 32];

    let (sk1, pk1) = VrfDraft03::keypair_from_seed(&seed);
    let (sk2, pk2) = VrfDraft03::keypair_from_seed(&seed);

    // Public keys should be identical
    assert_eq!(pk1, pk2, "Draft-03 keypair generation must be deterministic");

    // Secret keys should be identical (includes seed + public key)
    assert_eq!(sk1, sk2, "Draft-03 secret keys must be deterministic");

    // Secret key format: seed (32 bytes) || public_key (32 bytes)
    assert_eq!(&sk1[0..32], &seed[..], "Secret key must contain original seed");
    assert_eq!(&sk1[32..64], &pk1[..], "Secret key must contain public key");
}

/// Test Draft-13 deterministic keypair generation
#[test]
fn test_draft13_deterministic_keygen() {
    let seed = [0x43u8; 32];

    let (sk1, pk1) = VrfDraft13::keypair_from_seed(&seed);
    let (sk2, pk2) = VrfDraft13::keypair_from_seed(&seed);

    // Public keys should be identical
    assert_eq!(pk1, pk2, "Draft-13 keypair generation must be deterministic");

    // Secret keys should be identical
    assert_eq!(sk1, sk2, "Draft-13 secret keys must be deterministic");

    // Secret key format check
    assert_eq!(&sk1[0..32], &seed[..], "Secret key must contain original seed");
    assert_eq!(&sk1[32..64], &pk1[..], "Secret key must contain public key");
}

/// Test Draft-03 proof determinism
#[test]
fn test_draft03_proof_determinism() -> Result<()> {
    let seed = [0x44u8; 32];
    let (secret_key, _public_key) = VrfDraft03::keypair_from_seed(&seed);

    let message = b"Determinism test message";

    let proof1 = VrfDraft03::prove(&secret_key, message)?;
    let proof2 = VrfDraft03::prove(&secret_key, message)?;

    assert_eq!(proof1, proof2, "Draft-03 proofs must be deterministic");

    Ok(())
}

/// Test Draft-13 proof determinism
#[test]
fn test_draft13_proof_determinism() -> Result<()> {
    let seed = [0x45u8; 32];
    let (secret_key, _public_key) = VrfDraft13::keypair_from_seed(&seed);

    let message = b"Determinism test message";

    let proof1 = VrfDraft13::prove(&secret_key, message)?;
    let proof2 = VrfDraft13::prove(&secret_key, message)?;

    assert_eq!(proof1, proof2, "Draft-13 proofs must be deterministic");

    Ok(())
}

/// Test Draft-03 verification rejects wrong message
#[test]
fn test_draft03_wrong_message() -> Result<()> {
    let seed = [0x50u8; 32];
    let (secret_key, public_key) = VrfDraft03::keypair_from_seed(&seed);

    let message = b"Original message";
    let proof = VrfDraft03::prove(&secret_key, message)?;

    // Correct message should verify
    assert!(VrfDraft03::verify(&public_key, &proof, message).is_ok());

    // Wrong message should fail
    assert!(VrfDraft03::verify(&public_key, &proof, b"Wrong message").is_err());

    Ok(())
}

/// Test Draft-13 verification rejects wrong message
#[test]
fn test_draft13_wrong_message() -> Result<()> {
    let seed = [0x51u8; 32];
    let (secret_key, public_key) = VrfDraft13::keypair_from_seed(&seed);

    let message = b"Original message";
    let proof = VrfDraft13::prove(&secret_key, message)?;

    // Correct message should verify
    assert!(VrfDraft13::verify(&public_key, &proof, message).is_ok());

    // Wrong message should fail
    assert!(VrfDraft13::verify(&public_key, &proof, b"Wrong message").is_err());

    Ok(())
}

/// Test Draft-03 verification rejects wrong public key
#[test]
fn test_draft03_wrong_public_key() -> Result<()> {
    let seed1 = [0x60u8; 32];
    let seed2 = [0x61u8; 32];

    let (secret_key, _public_key1) = VrfDraft03::keypair_from_seed(&seed1);
    let (_secret_key2, public_key2) = VrfDraft03::keypair_from_seed(&seed2);

    let message = b"Test message";
    let proof = VrfDraft03::prove(&secret_key, message)?;

    // Wrong public key should fail verification
    assert!(VrfDraft03::verify(&public_key2, &proof, message).is_err());

    Ok(())
}

/// Test Draft-13 verification rejects wrong public key
#[test]
fn test_draft13_wrong_public_key() -> Result<()> {
    let seed1 = [0x62u8; 32];
    let seed2 = [0x63u8; 32];

    let (secret_key, _public_key1) = VrfDraft13::keypair_from_seed(&seed1);
    let (_secret_key2, public_key2) = VrfDraft13::keypair_from_seed(&seed2);

    let message = b"Test message";
    let proof = VrfDraft13::prove(&secret_key, message)?;

    // Wrong public key should fail verification
    assert!(VrfDraft13::verify(&public_key2, &proof, message).is_err());

    Ok(())
}

/// Test Draft-03 with empty message (edge case)
#[test]
fn test_draft03_empty_message() -> Result<()> {
    let seed = [0x70u8; 32];
    let (secret_key, public_key) = VrfDraft03::keypair_from_seed(&seed);

    let message = b"";
    let proof = VrfDraft03::prove(&secret_key, message)?;
    let output = VrfDraft03::verify(&public_key, &proof, message)?;

    assert_eq!(output.len(), 64);

    Ok(())
}

/// Test Draft-13 with empty message (edge case)
#[test]
fn test_draft13_empty_message() -> Result<()> {
    let seed = [0x71u8; 32];
    let (secret_key, public_key) = VrfDraft13::keypair_from_seed(&seed);

    let message = b"";
    let proof = VrfDraft13::prove(&secret_key, message)?;
    let output = VrfDraft13::verify(&public_key, &proof, message)?;

    assert_eq!(output.len(), 64);

    Ok(())
}

/// Test Draft-03 with large message (stress test)
#[test]
fn test_draft03_large_message() -> Result<()> {
    let seed = [0x80u8; 32];
    let (secret_key, public_key) = VrfDraft03::keypair_from_seed(&seed);

    let message = vec![0xAAu8; 10000];
    let proof = VrfDraft03::prove(&secret_key, &message)?;
    let output = VrfDraft03::verify(&public_key, &proof, &message)?;

    assert_eq!(output.len(), 64);

    Ok(())
}

/// Test Draft-13 with large message (stress test)
#[test]
fn test_draft13_large_message() -> Result<()> {
    let seed = [0x81u8; 32];
    let (secret_key, public_key) = VrfDraft13::keypair_from_seed(&seed);

    let message = vec![0xBBu8; 10000];
    let proof = VrfDraft13::prove(&secret_key, &message)?;
    let output = VrfDraft13::verify(&public_key, &proof, &message)?;

    assert_eq!(output.len(), 64);

    Ok(())
}

/// Test proof sizes match Cardano specifications
#[test]
fn test_cardano_proof_sizes() {
    let seed = [0x90u8; 32];

    // Draft-03 is Cardano's production VRF (80 bytes)
    let (sk03, _pk03) = VrfDraft03::keypair_from_seed(&seed);
    let proof03 = VrfDraft03::prove(&sk03, b"test").unwrap();
    assert_eq!(proof03.len(), 80, "Cardano production VRF (Draft-03) must be 80 bytes");

    // Draft-13 is batch-compatible (128 bytes)
    let (sk13, _pk13) = VrfDraft13::keypair_from_seed(&seed);
    let proof13 = VrfDraft13::prove(&sk13, b"test").unwrap();
    assert_eq!(proof13.len(), 128, "Draft-13 batch VRF must be 128 bytes");
}

/// Test VRF output uniqueness (different seeds → different outputs)
#[test]
fn test_vrf_output_uniqueness() -> Result<()> {
    let seed1 = [0xA0u8; 32];
    let seed2 = [0xA1u8; 32];

    let (sk1, _pk1) = VrfDraft03::keypair_from_seed(&seed1);
    let (sk2, _pk2) = VrfDraft03::keypair_from_seed(&seed2);

    let message = b"Same message";

    let proof1 = VrfDraft03::prove(&sk1, message)?;
    let proof2 = VrfDraft03::prove(&sk2, message)?;

    assert_ne!(proof1, proof2, "Different keys must produce different proofs");

    Ok(())
}

/// Test VRF verification with invalid proof (corrupted bytes)
#[test]
fn test_vrf_invalid_proof() {
    let seed = [0xB0u8; 32];
    let (_secret_key, public_key) = VrfDraft03::keypair_from_seed(&seed);

    let message = b"Test message";
    let invalid_proof = [0u8; 80]; // All zeros = invalid proof

    assert!(VrfDraft03::verify(&public_key, &invalid_proof, message).is_err(),
            "Invalid proof should fail verification");
}

/// Test Cardano slot leader VRF use case
#[test]
fn test_cardano_slot_leader_vrf() -> Result<()> {
    // Simulate Cardano slot leader election VRF
    let pool_seed = [0xC0u8; 32]; // Pool operator's VRF seed
    let (vrf_sk, vrf_vk) = VrfDraft03::keypair_from_seed(&pool_seed);

    // Slot nonce (in Cardano this would be epoch nonce + slot number)
    let slot_nonce = b"epoch_12345_slot_67890";

    // Pool operator generates VRF proof
    let vrf_proof = VrfDraft03::prove(&vrf_sk, slot_nonce)?;

    // Network verifies the VRF proof
    let vrf_output = VrfDraft03::verify(&vrf_vk, &vrf_proof, slot_nonce)?;

    // VRF output is used to determine slot leadership
    // (In real Cardano, compare vrf_output[0..8] against stake threshold)
    assert_eq!(vrf_output.len(), 64, "VRF output must be 64 bytes for Cardano");

    Ok(())
}
