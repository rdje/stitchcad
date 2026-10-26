//! Immutable original/profile/resolved references for a proposed SizeSet transformation.
use crate::{SizeSetInputError, SizeSetInputs, SizeSetProfilePin, SizeSetReference};
use core::fmt;
use sc_core::ontology::EntityId;

/// Editable origin metadata for a separately identified resolved target, not a transformed output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetResolutionOriginDefinition {
    /// Exact SizeSet reference originally named by the Design.
    pub source: SizeSetReference,
    /// Explicit selected profile/schema/content labels.
    pub profile: SizeSetProfilePin,
    /// Explicit allocated target identity/revision, distinct from all known source objects.
    pub resolved: SizeSetReference,
    /// Authored correspondence/provenance record; truth/existence remain registry/evidence obligations.
    pub provenance: EntityId,
}
/// Actual input/current-reference or target-identity refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetResolutionOriginError {
    /// Original current input failure.
    Inputs(Box<SizeSetInputError>),
    /// Authored source reference differs from the actual supplied SizeSet input view.
    SourceMismatch {
        /// Authored source pin.
        expected: SizeSetReference,
        /// Actual current input pin.
        actual: SizeSetReference,
    },
    /// Resolved target reuses a known input/provenance identity, even at another revision.
    DuplicateIdentity(EntityId),
}
impl fmt::Display for SizeSetResolutionOriginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size set resolution origin: {self:?}")
    }
}
impl std::error::Error for SizeSetResolutionOriginError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Inputs(issue) => Some(issue),
            _ => None,
        }
    }
}
/// Immutable declared resolution origin; no profile, mapping, numeric source or input object is mutated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetResolutionOrigin {
    definition: SizeSetResolutionOriginDefinition,
}
impl SizeSetResolutionOrigin {
    /// Validate the source pin and supplied allocated target against actual current input identities.
    /// Does not allocate ids, certify global freshness or validate/execute the selected profile.
    /// # Errors
    /// Returns original current input evidence, a mismatched source pin or a reused target identity.
    pub fn new(
        definition: SizeSetResolutionOriginDefinition,
        inputs: SizeSetInputs<'_>,
    ) -> Result<Self, SizeSetResolutionOriginError> {
        let result = Self { definition };
        result.validate_current(inputs)?;
        Ok(result)
    }
    /// Original source/selected-profile/target/provenance metadata, without unchecked mutation.
    #[must_use]
    pub const fn definition(&self) -> &SizeSetResolutionOriginDefinition {
        &self.definition
    }
    /// Recheck source identity/revision and target nonaliasing against the supplied current view.
    /// Global registry/currentness, provenance truth, profile resolution and execution remain separate.
    /// # Errors
    /// Returns original input failures, SourceMismatch or DuplicateIdentity; never retargets a source.
    pub fn validate_current(
        &self,
        inputs: SizeSetInputs<'_>,
    ) -> Result<(), SizeSetResolutionOriginError> {
        inputs
            .validate_current()
            .map_err(|issue| SizeSetResolutionOriginError::Inputs(Box::new(issue)))?;
        let actual = inputs.reference();
        if self.definition.source != actual {
            return Err(SizeSetResolutionOriginError::SourceMismatch {
                expected: self.definition.source,
                actual,
            });
        }
        let identity = self.definition.resolved.id;
        if identity == self.definition.provenance || inputs.contains_entity(identity) {
            return Err(SizeSetResolutionOriginError::DuplicateIdentity(identity));
        }
        Ok(())
    }
}
