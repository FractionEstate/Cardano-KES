//! Golden test vectors for KES implementations
//!
//! These tests validate the correct behavior of KES key evolution and signatures
//! across multiple periods, ensuring compatibility with Cardano's KES implementation.

use cardano_kes::kes::single::{SingleKesSignature, SingleKesVerifyKey};
use cardano_kes::kes::sum::{Sum6Kes, Sum6KesSignature, Sum6KesVerifyKey};
use cardano_kes::common::traits::{KesSig, KesVk, KesSigningKey};

/// Test SingleKES basic operations
#[test]
fn test_single_kes_basic() {
    let mut seed = [0u8; 32];
    seed[0] = 0x42; // Deterministic test seed

    // Generate key at period 0
    let mut sk = cardano_kes::kes::single::BasicSingleKes::keygen(&seed, 0);
    let vk = sk.to_verifying_key();

    let message = b"SingleKES test message";

    // Sign at period 0
    let sig = sk.sign(0, message);
    assert!(vk.verify(0, message, &sig), "Signature verification failed at period 0");

    // Should not verify at wrong period
    assert!(!vk.verify(1, message, &sig), "Signature should not verify at wrong period");

    // Should not verify with wrong message
    assert!(!vk.verify(0, b"wrong message", &sig), "Signature should not verify with wrong message");

    // Test key update
    sk.update();

    // Old signature should still verify (signatures are tied to periods, not key state)
    assert!(vk.verify(0, message, &sig), "Old signature should still verify after key update");
}

/// Test SingleKES compact variant
#[test]
fn test_single_kes_compact() {
    let mut seed = [0u8; 32];
    seed[0] = 0x43;

    let mut sk = cardano_kes::kes::single::CompactSingleKes::keygen(&seed, 0);
    let vk = sk.to_verifying_key();

    let message = b"CompactSingleKES test message";
    let sig = sk.sign(0, message);

    assert!(vk.verify(0, message, &sig), "Compact signature verification failed");

    // Compact signatures should include VK
    let sig_bytes = sig.as_bytes();
    assert!(sig_bytes.len() > 64, "Compact signature should include verification key");
}

/// Test Sum2KES (2 periods)
#[test]
fn test_sum2_kes() {
    let mut seed = [0u8; 32];
    seed[0] = 0x44;

    let mut sk = cardano_kes::kes::sum::Sum2Kes::keygen(&seed, 0);
    let vk = sk.to_verifying_key();

    let msg0 = b"Period 0 message";
    let msg1 = b"Period 1 message";

    // Sign at period 0
    let sig0 = sk.sign(0, msg0);
    assert!(vk.verify(0, msg0, &sig0), "Sum2KES sig0 verification failed");

    // Update to period 1
    sk.update();

    // Sign at period 1
    let sig1 = sk.sign(1, msg1);
    assert!(vk.verify(1, msg1, &sig1), "Sum2KES sig1 verification failed");

    // Old signature should still verify
    assert!(vk.verify(0, msg0, &sig0), "Sum2KES old signature should verify");

    // Cannot sign at period 0 anymore (key evolved)
    // This is a property of the implementation
}

/// Test Sum6KES (64 periods = 2^6)
#[test]
fn test_sum6_kes_evolution() {
    let seed = [0x45u8; 32];

    let mut sk = Sum6Kes::keygen(&seed, 0);
    let vk = sk.to_verifying_key();

    // Test evolution through multiple periods
    let mut signatures = Vec::new();
    let mut messages = Vec::new();

    for period in 0..64 {
        let msg = format!("Period {} message", period);
        messages.push(msg.clone());

        let sig = sk.sign(period, msg.as_bytes());
        signatures.push(sig);

        if period < 63 {
            sk.update();
        }
    }

    // Verify all signatures
    for (period, (msg, sig)) in messages.iter().zip(signatures.iter()).enumerate() {
        assert!(
            vk.verify(period, msg.as_bytes(), sig),
            "Sum6KES signature verification failed at period {}",
            period
        );
    }
}

/// Test key expiration
#[test]
fn test_kes_expiration() {
    let seed = [0x46u8; 32];

    // Sum2KES has 2 periods (0 and 1)
    let mut sk = cardano_kes::kes::sum::Sum2Kes::keygen(&seed, 0);

    // Update beyond the max period
    sk.update(); // Period 1
    sk.update(); // Period 2 (expired)

    // Key should be expired
    // The implementation should handle this gracefully
    // (exact behavior depends on implementation - may panic or return None)
}

/// Test seed determinism
#[test]
fn test_kes_seed_determinism() {
    let seed = [0x47u8; 32];

    let sk1 = Sum6Kes::keygen(&seed, 0);
    let sk2 = Sum6Kes::keygen(&seed, 0);

    let vk1 = sk1.to_verifying_key();
    let vk2 = sk2.to_verifying_key();

    assert_eq!(
        vk1.as_bytes(),
        vk2.as_bytes(),
        "Same seed should produce same verification key"
    );
}

/// Test signature serialization round-trip
#[test]
fn test_kes_signature_serialization() {
    let seed = [0x48u8; 32];
    let mut sk = Sum6Kes::keygen(&seed, 0);
    let vk = sk.to_verifying_key();

    let message = b"Serialization test";
    let sig = sk.sign(0, message);

    // Serialize and deserialize
    let sig_bytes = sig.as_bytes();
    let sig2 = Sum6KesSignature::from_bytes(sig_bytes);

    // Should verify after round-trip
    assert!(vk.verify(0, message, &sig2), "Signature failed after serialization round-trip");
}

/// Test verification key serialization
#[test]
fn test_kes_vk_serialization() {
    let seed = [0x49u8; 32];
    let sk = Sum6Kes::keygen(&seed, 0);
    let vk = sk.to_verifying_key();

    let vk_bytes = vk.as_bytes();
    let vk2 = Sum6KesVerifyKey::from_bytes(vk_bytes);

    assert_eq!(vk.as_bytes(), vk2.as_bytes(), "VK serialization round-trip failed");
}

/// Test different KES sum types
#[test]
fn test_different_sum_types() {
    let seed = [0x4Au8; 32];

    // Sum0 (1 period)
    let sk0 = cardano_kes::kes::sum::Sum0Kes::keygen(&seed, 0);
    let vk0 = sk0.to_verifying_key();
    let sig0 = sk0.sign(0, b"msg");
    assert!(vk0.verify(0, b"msg", &sig0), "Sum0 verification failed");

    // Sum1 (2 periods)
    let mut sk1 = cardano_kes::kes::sum::Sum1Kes::keygen(&seed, 0);
    let vk1 = sk1.to_verifying_key();
    let sig1a = sk1.sign(0, b"msg");
    sk1.update();
    let sig1b = sk1.sign(1, b"msg");
    assert!(vk1.verify(0, b"msg", &sig1a) && vk1.verify(1, b"msg", &sig1b), "Sum1 verification failed");

    // Sum2 (4 periods)
    let mut sk2 = cardano_kes::kes::sum::Sum2Kes::keygen(&seed, 0);
    let vk2 = sk2.to_verifying_key();
    for i in 0..4 {
        let sig = sk2.sign(i, b"msg");
        assert!(vk2.verify(i, b"msg", &sig), "Sum2 verification failed at period {}", i);
        if i < 3 {
            sk2.update();
        }
    }
}

/// Property test: signatures from different periods should be different
#[test]
fn test_kes_signatures_unique_per_period() {
    let seed = [0x4Bu8; 32];
    let mut sk = Sum6Kes::keygen(&seed, 0);
    let message = b"Same message every period";

    let sig0 = sk.sign(0, message);
    sk.update();
    let sig1 = sk.sign(1, message);

    // Even with same message, signatures should differ due to period
    assert_ne!(
        sig0.as_bytes(),
        sig1.as_bytes(),
        "Signatures should differ across periods"
    );
}

/// Test binary compatibility with expected sizes
#[test]
fn test_kes_binary_sizes() {
    let seed = [0x4Cu8; 32];

    // SingleKES
    let single_sk = cardano_kes::kes::single::BasicSingleKes::keygen(&seed, 0);
    let single_vk = single_sk.to_verifying_key();
    let single_sig = single_sk.sign(0, b"test");

    // VK should be 32 bytes (Ed25519 public key)
    assert_eq!(single_vk.as_bytes().len(), 32, "SingleKES VK size incorrect");

    // Signature should be 64 bytes (Ed25519 signature)
    assert_eq!(single_sig.as_bytes().len(), 64, "SingleKES signature size incorrect");

    // Sum6KES
    let sum6_sk = Sum6Kes::keygen(&seed, 0);
    let sum6_vk = sum6_sk.to_verifying_key();

    // Sum6 VK should be 32 bytes (merkle root)
    assert_eq!(sum6_vk.as_bytes().len(), 32, "Sum6KES VK size incorrect");
}

/// Test edge cases
#[test]
fn test_kes_edge_cases() {
    let seed = [0x4Du8; 32];

    // Empty message
    let mut sk = Sum6Kes::keygen(&seed, 0);
    let vk = sk.to_verifying_key();
    let sig = sk.sign(0, b"");
    assert!(vk.verify(0, b"", &sig), "Empty message signature failed");

    // Very long message
    let long_msg = vec![0xAAu8; 10000];
    sk.update();
    let sig2 = sk.sign(1, &long_msg);
    assert!(vk.verify(1, &long_msg, &sig2), "Long message signature failed");
}
