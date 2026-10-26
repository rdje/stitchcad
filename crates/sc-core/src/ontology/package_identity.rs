//! Exact content-identity claims, not a computed digest or certificate of a stored candidate.
use core::fmt;

/// Opaque algorithm label and exact manifest-digest bytes; actual canonicalizer validation is separate.
/// Equality retains the label's case/spacing and every byte, including leading or trailing zeroes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackageIdentity {
    algorithm: String,
    digest: Vec<u8>,
}
/// Actual missing identity content, without normalizing or replacing it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageIdentityError {
    /// A hash algorithm must be named explicitly; the original spelling is retained.
    BlankAlgorithm(String),
    /// No digest bytes were supplied for this exact algorithm label.
    EmptyDigest {
        /// Original explicit algorithm spelling.
        algorithm: String,
    },
}
impl fmt::Display for PackageIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankAlgorithm(value) => {
                write!(f, "package identity algorithm is blank: {value:?}")
            }
            Self::EmptyDigest { algorithm } => {
                write!(f, "package identity digest is empty for {algorithm:?}")
            }
        }
    }
}
impl std::error::Error for PackageIdentityError {}
impl PackageIdentity {
    /// Preserve explicit identity claims without hashing, parsing aliases or guessing an algorithm.
    /// # Errors
    /// Refuses a blank algorithm first, then empty digest bytes. Execution must check the actual
    /// supported algorithm, digest width and exact stored canonical manifest before accepting a target.
    pub fn new(algorithm: String, digest: Vec<u8>) -> Result<Self, PackageIdentityError> {
        if algorithm.trim().is_empty() {
            return Err(PackageIdentityError::BlankAlgorithm(algorithm));
        }
        if digest.is_empty() {
            return Err(PackageIdentityError::EmptyDigest { algorithm });
        }
        Ok(Self { algorithm, digest })
    }
    /// Exact algorithm label; its presence proves no registry support.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }
    /// Exact original digest bytes, not a newly computed or truncated value.
    #[must_use]
    pub fn digest(&self) -> &[u8] {
        &self.digest
    }
}
