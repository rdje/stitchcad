//! Stable ASCII machine identifiers shared by metadata and recipe namespaces.
use core::fmt;

/// Validated lower-snake identifier; a machine token is not a localized display label.
/// Built-in names remain valid references. Their rebinding is a separate namespace error.
/// ```compile_fail
/// fn rename(token: &mut sc_core::name::MachineToken) {
///     token.0 = "unvalidated name".to_owned();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MachineToken(String);

/// An identifier cannot enter the shared machine namespace with this spelling.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MachineTokenError {
    /// Requires ASCII lowercase start and nonempty lowercase/digit segments separated by underscores.
    InvalidSyntax(String),
    /// A grammar keyword cannot be an identifier.
    ReservedKeyword(String),
}

impl fmt::Display for MachineTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSyntax(token) => write!(f, "invalid lower-snake machine token {token:?}"),
            Self::ReservedKeyword(token) => {
                write!(f, "machine token {token:?} is a grammar keyword")
            }
        }
    }
}
impl std::error::Error for MachineTokenError {}

impl MachineToken {
    /// Validate without trimming, normalizing or automatically renaming the input.
    /// # Errors
    /// Returns [`MachineTokenError`] for malformed syntax or the keywords `let`, `assert`, `if`.
    pub fn new(input: impl Into<String>) -> Result<Self, MachineTokenError> {
        let input = input.into();
        let valid_start = input.bytes().next().is_some_and(|c| c.is_ascii_lowercase());
        let valid_segments = input.split('_').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        });
        if !valid_start || !valid_segments {
            return Err(MachineTokenError::InvalidSyntax(input));
        }
        if matches!(input.as_str(), "let" | "assert" | "if") {
            return Err(MachineTokenError::ReservedKeyword(input));
        }
        Ok(Self(input))
    }

    /// Exact authored machine spelling; no display-label transformation is implied.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether this is a built-in parameter name that authored inputs may reference but not rebind.
    #[must_use]
    pub fn is_reserved_input_name(&self) -> bool {
        matches!(
            self.as_str(),
            "eps_num"
                | "eps_geo"
                | "eps_fmt"
                | "eps_imp"
                | "eps_phys"
                | "size_index"
                | "size_count"
                | "is_base_size"
        )
    }
}

impl fmt::Display for MachineToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
