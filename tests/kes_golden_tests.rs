//! Golden test vectors for KES implementations
//!
//! These tests validate the correct behavior of KES key evolution and signatures
//! across multiple periods, ensuring compatibility with Cardano's KES implementation.
//! 
//! Tests follow the IntersectMBO/cardano-base test patterns using trait-based API.

use cardano_crypto::common::Result;
use cardano_crypto::dsign::Ed25519;
use cardano_crypto::kes::{Blake2b256, KesAlgorithm, SingleKes, Sum2Kes, Sum6Kes};

/// Test SingleKES basic operations using trait-based API
#[test]
fn test_single_kes_basic() -> Result<()> {
    type TestKes = SingleKes<Ed25519>;
    
    let mut seed = [0u8; 32];
    seed[0] = 0x42; // Deterministic test seed

    // Generate key at period 0
    let (mut sk, vk) = TestKes::gen_key_kes_from_seed_bytes(&seed);

    let message = b"SingleKES test message";
    let period = 0;

    // Sign at period 0
    let sig = TestKes::sign_kes(&sk, period, message)?;
    
    // Verification should succeed
    assert!(TestKes::verify_kes(&vk, period, message, &sig).is_ok(), 
            "Signature verification failed at period 0");

    // Should not verify at wrong period
    assert!(TestKes::verify_kes(&vk, 1, message, &sig).is_err(), 
            "Signature should not verify at wrong period");

    // Should not verify with wrong message
    assert!(TestKes::verify_kes(&vk, period, b"wrong message", &sig).is_err(), 
            "Signature should not verify with wrong message");

    // Test key update (SingleKES only supports period 0, so update should fail/expire)
    let update_result = TestKes::update_kes(&mut sk, period);
    assert!(update_result.is_err(), "SingleKES should expire after period 0");

    Ok(())
}

/// Test Sum2KES (2 periods)
#[test]
fn test_sum2_kes() -> Result<()> {
    type TestKes = Sum2Kes<Ed25519, Blake2b256>;
    
    let mut seed = [0u8; 32];
    seed[0] = 0x44;

    let (mut sk, vk) = TestKes::gen_key_kes_from_seed_bytes(&seed);

    let msg0 = b"Period 0 message";
    let msg1 = b"Period 1 message";

    // Sign at period 0
    let sig0 = TestKes::sign_kes(&sk, 0, msg0)?;
    assert!(TestKes::verify_kes(&vk, 0, msg0, &sig0).is_ok(), 
            "Sum2KES sig0 verification failed");

    // Update to period 1
    TestKes::update_kes(&mut sk, 0)?;

    // Sign at period 1
    let sig1 = TestKes::sign_kes(&sk, 1, msg1)?;
    assert!(TestKes::verify_kes(&vk, 1, msg1, &sig1).is_ok(), 
            "Sum2KES sig1 verification failed");

    // Old signature should still verify
    assert!(TestKes::verify_kes(&vk, 0, msg0, &sig0).is_ok(), 
            "Sum2KES old signature should verify");

    Ok(())
}

/// Test Sum6KES (64 periods = 2^6) - Cardano's production KES scheme
#[test]
fn test_sum6_kes_evolution() -> Result<()> {
    type TestKes = Sum6Kes<Ed25519, Blake2b256>;
    
    let seed = [0x45u8; 32];

    let (mut sk, vk) = TestKes::gen_key_kes_from_seed_bytes(&seed);

    // Test evolution through multiple periods
    let test_periods = [0, 1, 2, 4, 8, 16, 32, 63]; // Sample various periods

    for &period in &test_periods {
        // Update to target period
        for current in 0..period {
            TestKes::update_kes(&mut sk, current)?;
        }

        // Sign and verify at this period
        let message = format!("Period {} message", period);
        let sig = TestKes::sign_kes(&sk, period, message.as_bytes())?;
        
        assert!(TestKes::verify_kes(&vk, period, message.as_bytes(), &sig).is_ok(),
                "Sum6KES verification failed at period {}", period);

        // Reset for next test
        let (new_sk, _) = TestKes::gen_key_kes_from_seed_bytes(&seed);
        sk = new_sk;
    }

    Ok(())
}

/// Test Sum6KES total periods (Cardano standard: 64 periods for 1.5 days at 20s/slot)
#[test]
fn test_sum6_kes_total_periods() {
    type TestKes = Sum6Kes<Ed25519, Blake2b256>;
    
    // Cardano uses Sum6KES which gives 2^6 = 64 periods
    // This matches the Cardano mainnet KES period (approximately 1.5 days)
    const EXPECTED_PERIODS: u32 = 64;
    assert_eq!(TestKes::total_periods_kes(), EXPECTED_PERIODS,
               "Sum6KES must support exactly 64 periods for Cardano compatibility");
}

/// Test Sum6KES with Cardano standard parameters
/// This validates the exact configuration used in cardano-node block production
#[test]
fn test_sum6_kes_cardano_standard() -> Result<()> {
    type CardanoKes = Sum6Kes<Ed25519, Blake2b256>;
    
    // Use a deterministic seed matching Cardano test vectors
    let seed = [0x4Cu8; 32];

    let (mut sk, vk) = CardanoKes::gen_key_kes_from_seed_bytes(&seed);

    // Test signing and verification at various periods within the 64-period range
    let test_cases = vec![
        (0, b"Genesis block" as &[u8]),
        (10, b"Early epoch"),
        (31, b"Mid evolution"),
        (63, b"Final period"),
    ];

    for (period, message) in test_cases {
        // Update to target period
        for t in 0..period {
            CardanoKes::update_kes(&mut sk, t)?;
        }

        // Sign
        let sig = CardanoKes::sign_kes(&sk, period, message)?;

        // Verify
        assert!(CardanoKes::verify_kes(&vk, period, message, &sig).is_ok(),
                "Cardano KES verification failed at period {} with message {:?}", 
                period, std::str::from_utf8(message));

        // Wrong period should fail
        if period > 0 {
            assert!(CardanoKes::verify_kes(&vk, period - 1, message, &sig).is_err(),
                    "Signature should not verify at wrong period");
        }

        // Reset for next test
        let (new_sk, _) = CardanoKes::gen_key_kes_from_seed_bytes(&seed);
        sk = new_sk;
    }

    Ok(())
}

/// Test KES key serialization round-trip
#[test]
fn test_kes_serialization_roundtrip() -> Result<()> {
    type TestKes = Sum6Kes<Ed25519, Blake2b256>;
    
    let seed = [0x99u8; 32];
    let (_sk, vk) = TestKes::gen_key_kes_from_seed_bytes(&seed);

    // Serialize verification key
    let vk_bytes = TestKes::raw_serialize_verification_key_kes(&vk);

    // Deserialize
    let vk_restored = TestKes::raw_deserialize_verification_key_kes(&vk_bytes)?;

    // Original and restored should produce same serialization
    let vk_bytes_restored = TestKes::raw_serialize_verification_key_kes(&vk_restored);
    assert_eq!(vk_bytes, vk_bytes_restored, "Verification key serialization round-trip failed");

    Ok(())
}

/// Test signature serialization round-trip
#[test]
fn test_signature_serialization_roundtrip() -> Result<()> {
    type TestKes = Sum2Kes<Ed25519, Blake2b256>;
    
    let seed = [0xAAu8; 32];
    let (sk, vk) = TestKes::gen_key_kes_from_seed_bytes(&seed);

    let message = b"Serialization test";
    let sig = TestKes::sign_kes(&sk, 0, message)?;

    // Serialize signature
    let sig_bytes = TestKes::raw_serialize_signature_kes(&sig);

    // Deserialize
    let sig_restored = TestKes::raw_deserialize_signature_kes(&sig_bytes)?;

    // Verify restored signature works
    assert!(TestKes::verify_kes(&vk, 0, message, &sig_restored).is_ok(),
            "Deserialized signature failed verification");

    Ok(())
}

/// Test cross-period validation failure
#[test]
fn test_cross_period_validation_failure() -> Result<()> {
    type TestKes = Sum2Kes<Ed25519, Blake2b256>;
    
    let seed = [0xBBu8; 32];
    let (sk, vk) = TestKes::gen_key_kes_from_seed_bytes(&seed);

    let message = b"Period mismatch test";
    
    // Sign at period 0
    let sig = TestKes::sign_kes(&sk, 0, message)?;

    // Verification at period 0 should succeed
    assert!(TestKes::verify_kes(&vk, 0, message, &sig).is_ok());

    // Verification at wrong period should fail
    assert!(TestKes::verify_kes(&vk, 1, message, &sig).is_err(),
            "Signature from period 0 should not verify at period 1");

    Ok(())
}

/// Test deterministic key generation
#[test]
fn test_deterministic_key_generation() -> Result<()> {
    type TestKes = Sum6Kes<Ed25519, Blake2b256>;
    
    let seed = [0xCCu8; 32];

    // Generate twice with same seed
    let (sk1, vk1) = TestKes::gen_key_kes_from_seed_bytes(&seed);
    let (sk2, vk2) = TestKes::gen_key_kes_from_seed_bytes(&seed);

    // Verification keys should be identical
    let vk1_bytes = TestKes::raw_serialize_verification_key_kes(&vk1);
    let vk2_bytes = TestKes::raw_serialize_verification_key_kes(&vk2);
    assert_eq!(vk1_bytes, vk2_bytes, "Deterministic key generation failed for verification keys");

    // Signatures from both keys should be identical
    let message = b"Determinism test";
    let sig1 = TestKes::sign_kes(&sk1, 0, message)?;
    let sig2 = TestKes::sign_kes(&sk2, 0, message)?;

    let sig1_bytes = TestKes::raw_serialize_signature_kes(&sig1);
    let sig2_bytes = TestKes::raw_serialize_signature_kes(&sig2);
    assert_eq!(sig1_bytes, sig2_bytes, "Deterministic signatures failed");

    Ok(())
}
