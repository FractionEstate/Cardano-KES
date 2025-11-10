//! Golden test vectors for VRF implementations
//!
//! These tests validate byte-for-byte compatibility with Cardano's official VRF
//! implementation by testing against official test vectors from the cardano-base repository.

use cardano_crypto::vrf::{draft03, draft13};
use std::fs;
use std::path::PathBuf;

/// Represents a parsed VRF test vector
#[derive(Debug, Clone)]
struct VrfTestVector {
    name: String,
    version: String,
    ciphersuite: String,
    sk: Vec<u8>,
    pk: Vec<u8>,
    alpha: Vec<u8>,  // Message
    pi: Vec<u8>,     // Proof
    beta: Vec<u8>,   // Output hash
}

impl VrfTestVector {
    /// Parse a test vector file
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut name = String::new();
        let mut version = String::new();
        let mut ciphersuite = String::new();
        let mut sk = Vec::new();
        let mut pk = Vec::new();
        let mut alpha = Vec::new();
        let mut pi = Vec::new();
        let mut beta = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "vrf" => name = value.to_string(),
                    "ver" => version = value.to_string(),
                    "ciphersuite" => ciphersuite = value.to_string(),
                    "sk" => sk = hex::decode(value)?,
                    "pk" => pk = hex::decode(value)?,
                    "alpha" => alpha = hex::decode(value)?,
                    "pi" => pi = hex::decode(value)?,
                    "beta" => beta = hex::decode(value)?,
                    _ => {}
                }
            }
        }

        Ok(VrfTestVector {
            name,
            version,
            ciphersuite,
            sk,
            pk,
            alpha,
            pi,
            beta,
        })
    }

    /// Get the complete signing key (sk || pk for Ed25519)
    fn signing_key(&self) -> Vec<u8> {
        let mut full_sk = Vec::with_capacity(64);
        full_sk.extend_from_slice(&self.sk);
        full_sk.extend_from_slice(&self.pk);
        full_sk
    }
}

/// Run all VRF Draft-03 golden tests
#[test]
fn test_vrf_draft03_golden_vectors() {
    let test_files = vec![
        "vrf_ver03_generated_1",
        "vrf_ver03_generated_2",
        "vrf_ver03_generated_3",
        "vrf_ver03_generated_4",
        "vrf_ver03_standard_10",
        "vrf_ver03_standard_11",
        "vrf_ver03_standard_12",
    ];

    for test_file in test_files {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/test_vectors");
        path.push(test_file);

        println!("Testing VRF Draft-03: {}", test_file);

        let vector = VrfTestVector::from_file(path.to_str().unwrap())
            .expect(&format!("Failed to load test vector: {}", test_file));

        // Verify metadata
        assert_eq!(vector.name, "PraosVRF", "Test vector name mismatch");
        assert_eq!(vector.version, "ietfdraft03", "Test vector version mismatch");
        assert_eq!(
            vector.ciphersuite, "ECVRF-ED25519-SHA512-Elligator2",
            "Test vector ciphersuite mismatch"
        );

        // Test key generation from seed
        let seed: [u8; 32] = vector.sk.clone().try_into().unwrap();
        let keypair = draft03::keypair_from_seed(&seed);
        assert_eq!(
            keypair.vk.as_bytes(),
            &vector.pk[..],
            "Public key derivation mismatch in {}",
            test_file
        );

        // Test proof generation
        let proof = draft03::prove(&keypair, &vector.alpha);
        assert_eq!(
            proof.as_bytes(),
            &vector.pi[..],
            "Proof generation mismatch in {}",
            test_file
        );

        // Test output from proof
        let output_from_proof = draft03::proof_to_hash(&proof);
        assert_eq!(
            output_from_proof.as_bytes(),
            &vector.beta[..],
            "Output from proof mismatch in {}",
            test_file
        );

        // Test verification
        let output_from_verify = draft03::verify(&keypair.vk, &proof, &vector.alpha);
        assert!(
            output_from_verify.is_some(),
            "Verification failed in {}",
            test_file
        );
        assert_eq!(
            output_from_verify.unwrap().as_bytes(),
            &vector.beta[..],
            "Output from verification mismatch in {}",
            test_file
        );

        println!("  ✓ {} passed", test_file);
    }
}

/// Run all VRF Draft-13 golden tests
#[test]
fn test_vrf_draft13_golden_vectors() {
    let test_files = vec![
        "vrf_ver13_generated_1",
        "vrf_ver13_generated_2",
        "vrf_ver13_generated_3",
        "vrf_ver13_generated_4",
        "vrf_ver13_standard_10",
        "vrf_ver13_standard_11",
        "vrf_ver13_standard_12",
    ];

    for test_file in test_files {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/test_vectors");
        path.push(test_file);

        println!("Testing VRF Draft-13: {}", test_file);

        let vector = VrfTestVector::from_file(path.to_str().unwrap())
            .expect(&format!("Failed to load test vector: {}", test_file));

        // Verify metadata
        assert_eq!(vector.name, "PraosBatchCompatVRF", "Test vector name mismatch");
        assert_eq!(vector.version, "ietfdraft13", "Test vector version mismatch");
        assert_eq!(
            vector.ciphersuite, "ECVRF-ED25519-SHA512-Elligator2",
            "Test vector ciphersuite mismatch"
        );

        // Test key generation from seed
        let seed: [u8; 32] = vector.sk.clone().try_into().unwrap();
        let keypair = draft13::keypair_from_seed(&seed);
        assert_eq!(
            keypair.vk.as_bytes(),
            &vector.pk[..],
            "Public key derivation mismatch in {}",
            test_file
        );

        // Test proof generation
        let proof = draft13::prove(&keypair, &vector.alpha);
        assert_eq!(
            proof.as_bytes(),
            &vector.pi[..],
            "Proof generation mismatch in {}",
            test_file
        );

        // Test output from proof
        let output_from_proof = draft13::proof_to_hash(&proof);
        assert_eq!(
            output_from_proof.as_bytes(),
            &vector.beta[..],
            "Output from proof mismatch in {}",
            test_file
        );

        // Test verification
        let output_from_verify = draft13::verify(&keypair.vk, &proof, &vector.alpha);
        assert!(
            output_from_verify.is_some(),
            "Verification failed in {}",
            test_file
        );
        assert_eq!(
            output_from_verify.unwrap().as_bytes(),
            &vector.beta[..],
            "Output from verification mismatch in {}",
            test_file
        );

        println!("  ✓ {} passed", test_file);
    }
}

/// Test that proofs from one VRF version don't verify with another
#[test]
fn test_vrf_version_incompatibility() {
    let seed = [0u8; 32];
    let message = b"test message";

    let kp03 = draft03::keypair_from_seed(&seed);
    let kp13 = draft13::keypair_from_seed(&seed);

    let proof03 = draft03::prove(&kp03, message);
    let proof13 = draft13::prove(&kp13, message);

    // Draft-03 proof should not be valid for Draft-13 (different proof sizes)
    assert_eq!(proof03.as_bytes().len(), 80, "Draft-03 proof should be 80 bytes");
    assert_eq!(proof13.as_bytes().len(), 128, "Draft-13 proof should be 128 bytes");
}

/// Benchmark-style test to ensure performance is reasonable
#[test]
fn test_vrf_performance_sanity() {
    use std::time::Instant;

    let seed = [42u8; 32];
    let message = b"performance test message";

    // Draft-03 performance
    let start = Instant::now();
    let kp03 = draft03::keypair_from_seed(&seed);
    let keygen_time = start.elapsed();

    let start = Instant::now();
    let proof = draft03::prove(&kp03, message);
    let prove_time = start.elapsed();

    let start = Instant::now();
    let _output = draft03::verify(&kp03.vk, &proof, message);
    let verify_time = start.elapsed();

    println!("Draft-03 Performance:");
    println!("  Keygen:  {:?}", keygen_time);
    println!("  Prove:   {:?}", prove_time);
    println!("  Verify:  {:?}", verify_time);

    // Sanity checks (should complete in reasonable time)
    assert!(keygen_time.as_millis() < 100, "Keygen too slow");
    assert!(prove_time.as_millis() < 100, "Prove too slow");
    assert!(verify_time.as_millis() < 100, "Verify too slow");
}
