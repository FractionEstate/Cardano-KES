//! Error types for KES operations

use core::fmt;

#[cfg(feature = "std")]
use std::error::Error as StdError;

/// The KES period type
pub type Period = u64;

/// Errors that can occur during KES operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KesError {
    /// Signature verification failed
    VerificationFailed,

    /// Wrong data length
    WrongLength {
        /// Context for the error
        context: &'static str,
        /// Expected length in bytes
        expected: usize,
        /// Actual length in bytes
        actual: usize,
    },

    /// Generic error message
    Message(alloc::string::String),

    /// Key evolved beyond maximum period
    KeyExpired,

    /// Period out of valid range
    PeriodOutOfRange {
        /// The invalid period
        period: Period,
        /// Maximum valid period
        max_period: Period,
    },
}

impl KesError {
    /// Create a wrong length error
    #[must_use]
    pub fn wrong_length(context: &'static str, expected: usize, actual: usize) -> Self {
        KesError::WrongLength {
            context,
            expected,
            actual,
        }
    }
}

impl fmt::Display for KesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KesError::VerificationFailed => write!(f, "KES signature verification failed"),
            KesError::WrongLength { context, expected, actual } => {
                write!(f, "{}: wrong length, expected {} bytes but got {}", context, expected, actual)
            }
            KesError::Message(msg) => write!(f, "{}", msg),
            KesError::KeyExpired => write!(f, "KES key evolved beyond max period"),
            KesError::PeriodOutOfRange { period, max_period } => {
                write!(f, "period {} out of range [0, {})", period, max_period)
            }
        }
    }
}

#[cfg(feature = "std")]
impl StdError for KesError {}

/// Error type that can include allocation errors (for no_std compatibility)
#[derive(Debug)]
pub enum KesMError {
    /// KES-specific error
    Kes(KesError),

    /// Generic error message
    Message(alloc::string::String),
}

impl From<KesError> for KesMError {
    fn from(err: KesError) -> Self {
        KesMError::Kes(err)
    }
}

impl fmt::Display for KesMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KesMError::Kes(err) => write!(f, "{}", err),
            KesMError::Message(msg) => write!(f, "{}", msg),
        }
    }
}

#[cfg(feature = "std")]
impl StdError for KesMError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            KesMError::Kes(err) => Some(err),
            KesMError::Message(_) => None,
        }
    }
}
