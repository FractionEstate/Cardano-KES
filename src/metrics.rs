//! Lightweight metrics for KES operations (feature-gated)
//!
//! Enabled via the `kes-metrics` crate feature.

#![cfg(feature = "kes-metrics")]

use core::sync::atomic::{AtomicU64, Ordering};

/// Snapshot of KES metrics
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KesMetrics {
    /// Number of signing keys generated
    pub signing_keys: u64,
    /// Total bytes of signing key material
    pub signing_key_bytes: u64,
    /// Number of signatures created
    pub signatures: u64,
    /// Total bytes of signature data
    pub signature_bytes: u64,
    /// Number of key updates (evolutions)
    pub updates: u64,
}

static SIGNING_KEYS: AtomicU64 = AtomicU64::new(0);
static SIGNING_KEY_BYTES: AtomicU64 = AtomicU64::new(0);
static SIGNATURES: AtomicU64 = AtomicU64::new(0);
static SIGNATURE_BYTES: AtomicU64 = AtomicU64::new(0);
static UPDATES: AtomicU64 = AtomicU64::new(0);

/// Record a signing key generation
pub fn record_signing_key(bytes: usize) {
    SIGNING_KEYS.fetch_add(1, Ordering::Relaxed);
    SIGNING_KEY_BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
}

/// Record a signature creation
pub fn record_signature(bytes: usize) {
    SIGNATURES.fetch_add(1, Ordering::Relaxed);
    SIGNATURE_BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
}

/// Record a key update
pub fn record_update() {
    UPDATES.fetch_add(1, Ordering::Relaxed);
}

/// Get current metrics snapshot
#[must_use]
pub fn snapshot() -> KesMetrics {
    KesMetrics {
        signing_keys: SIGNING_KEYS.load(Ordering::Relaxed),
        signing_key_bytes: SIGNING_KEY_BYTES.load(Ordering::Relaxed),
        signatures: SIGNATURES.load(Ordering::Relaxed),
        signature_bytes: SIGNATURE_BYTES.load(Ordering::Relaxed),
        updates: UPDATES.load(Ordering::Relaxed),
    }
}

/// Reset all metrics to zero
pub fn reset() {
    SIGNING_KEYS.store(0, Ordering::Relaxed);
    SIGNING_KEY_BYTES.store(0, Ordering::Relaxed);
    SIGNATURES.store(0, Ordering::Relaxed);
    SIGNATURE_BYTES.store(0, Ordering::Relaxed);
    UPDATES.store(0, Ordering::Relaxed);
}

// No-op versions when feature is disabled
#[cfg(not(feature = "kes-metrics"))]
pub fn record_signing_key(_bytes: usize) {}

#[cfg(not(feature = "kes-metrics"))]
pub fn record_signature(_bytes: usize) {}

#[cfg(not(feature = "kes-metrics"))]
pub fn record_update() {}
