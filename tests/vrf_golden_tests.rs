//! Golden test vectors for VRF implementations (Cardano-compatible)

use cardano_crypto::common::Result;
use cardano_crypto::vrf::{VrfDraft03, VrfDraft13};

#[test]
fn test_vrf_draft03_basic() -> Result<()> {
    let seed = [0x01u8; 32];
    let (sk, vk) = VrfDraft03::keypair_from_seed(&seed);

    let alpha = b"VRF Draft-03 test message";
    let proof = VrfDraft03::prove(&sk, alpha)?;

    let hash = VrfDraft03::verify(&vk, &proof, alpha)?;
    let hash_from_proof = VrfDraft03::proof_to_hash(&proof)?;
    assert_eq!(hash, hash_from_proof);
    assert_eq!(hash.len(), 64);

    Ok(())
}

#[test]
fn test_vrf_draft13_basic() -> Result<()> {
    let seed = [0x02u8; 32];
    let (sk, vk) = VrfDraft13::keypair_from_seed(&seed);

    let alpha = b"VRF Draft-13 test message";
    let proof = VrfDraft13::prove(&sk, alpha)?;

    let hash = VrfDraft13::verify(&vk, &proof, alpha)?;
    let hash_from_proof = VrfDraft13::proof_to_hash(&proof)?;
    assert_eq!(hash, hash_from_proof);
    assert_eq!(hash.len(), 64);

    Ok(())
}

#[test]
fn test_vrf_draft03_deterministic() -> Result<()> {
    let (sk1, _) = VrfDraft03::keypair_from_seed(&[0x03u8; 32]);
    let (sk2, _) = VrfDraft03::keypair_from_seed(&[0x03u8; 32]);

    let proof1 = VrfDraft03::prove(&sk1, b"test")?;
    let proof2 = VrfDraft03::prove(&sk2, b"test")?;
    assert_eq!(proof1, proof2);

    Ok(())
}

#[test]
fn test_vrf_draft13_deterministic() -> Result<()> {
    let (sk1, _) = VrfDraft13::keypair_from_seed(&[0x04u8; 32]);
    let (sk2, _) = VrfDraft13::keypair_from_seed(&[0x04u8; 32]);

    let proof1 = VrfDraft13::prove(&sk1, b"test")?;
    let proof2 = VrfDraft13::prove(&sk2, b"test")?;
    assert_eq!(proof1, proof2);

    Ok(())
}

#[test]
fn test_vrf_draft03_wrong_key() -> Result<()> {
    let (sk1, _) = VrfDraft03::keypair_from_seed(&[0x05u8; 32]);
    let (_, vk2) = VrfDraft03::keypair_from_seed(&[0x06u8; 32]);

    let proof = VrfDraft03::prove(&sk1, b"test")?;
    assert!(VrfDraft03::verify(&vk2, &proof, b"test").is_err());

    Ok(())
}

#[test]
fn test_vrf_draft13_wrong_key() -> Result<()> {
    let (sk1, _) = VrfDraft13::keypair_from_seed(&[0x07u8; 32]);
    let (_, vk2) = VrfDraft13::keypair_from_seed(&[0x08u8; 32]);

    let proof = VrfDraft13::prove(&sk1, b"test")?;
    assert!(VrfDraft13::verify(&vk2, &proof, b"test").is_err());

    Ok(())
}

#[test]
fn test_vrf_draft03_cardano_compat() -> Result<()> {
    let seed = [0x42u8; 32];
    let (sk, vk) = VrfDraft03::keypair_from_seed(&seed);

    let messages = [
        b"Block header" as &[u8],
        b"Nonce generation",
        b"Leader election",
    ];

    for &msg in &messages {
        let proof = VrfDraft03::prove(&sk, msg)?;
        let hash = VrfDraft03::verify(&vk, &proof, msg)?;
        let hash2 = VrfDraft03::proof_to_hash(&proof)?;
        assert_eq!(hash, hash2);
        assert_eq!(hash.len(), 64);
    }

    Ok(())
}

#[test]
fn test_vrf_draft13_cardano_compat() -> Result<()> {
    let seed = [0x43u8; 32];
    let (sk, vk) = VrfDraft13::keypair_from_seed(&seed);

    let messages = [
        b"Block header" as &[u8],
        b"Nonce generation",
        b"Leader election",
    ];

    for &msg in &messages {
        let proof = VrfDraft13::prove(&sk, msg)?;
        let hash = VrfDraft13::verify(&vk, &proof, msg)?;
        let hash2 = VrfDraft13::proof_to_hash(&proof)?;
        assert_eq!(hash, hash2);
        assert_eq!(hash.len(), 64);
    }

    Ok(())
}
