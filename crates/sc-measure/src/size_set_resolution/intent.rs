//! Immutable resolution intent preserves authored origin/claims; it never applies profile overrides.
use crate::{
    SizeSetChartReplacementContext, SizeSetInputs, SizeSetResolutionOrigin,
    SizeSetResolutionOriginError, SizeSetTransformation, SizeSetTransformationError,
};
use core::fmt;
use sc_core::ontology::EntityId;
use std::collections::BTreeSet;

/// Editable intent containing original/selected-profile/resolved metadata and ordered declarations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetResolutionIntentDefinition {
    /// Immutable exact source/profile/target/provenance metadata.
    pub origin: SizeSetResolutionOrigin,
    /// Authored declaration order, including unresolved states; empty means no declared override.
    pub transformations: Vec<SizeSetTransformation>,
}
/// Original origin/declaration failure or cross-record identity conflict.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetResolutionIntentError {
    /// Original actual source/reference/target-origin failure.
    Origin(Box<SizeSetResolutionOriginError>),
    /// A transformation repeats an identity or aliases the target/origin provenance.
    DuplicateIdentity(EntityId),
    /// Original declaration/current binding failure, preserving its authored identity and error chain.
    Transformation {
        /// Authored declaration whose current validation failed.
        transformation: EntityId,
        /// Original typed failure; no observation/formula is replaced by a fallback.
        issue: Box<SizeSetTransformationError>,
    },
}
impl fmt::Display for SizeSetResolutionIntentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size set resolution intent: {self:?}")
    }
}
impl std::error::Error for SizeSetResolutionIntentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Origin(issue) => Some(issue),
            Self::Transformation { issue, .. } => Some(issue),
            Self::DuplicateIdentity(_) => None,
        }
    }
}
/// Immutable locally checked intent, not an applied/resolved SizeSet or geometry result.
/// Declaration order/state stays authored. G4 owns composition, conflicts, evidence truth and execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetResolutionIntent {
    definition: SizeSetResolutionIntentDefinition,
}
impl SizeSetResolutionIntent {
    /// Validate origin and declarations against actual original source inputs, without applying them.
    /// # Errors
    /// Returns original origin/declaration failures or cross-record identity conflicts.
    /// Authored chart replacement requires the explicit-context constructor.
    pub fn new(
        definition: SizeSetResolutionIntentDefinition,
        inputs: SizeSetInputs<'_>,
    ) -> Result<Self, SizeSetResolutionIntentError> {
        let result = Self { definition };
        result.validate_current(inputs)?;
        Ok(result)
    }
    /// Validate using actual replacement candidates in their original source metadata context.
    /// # Errors
    /// Returns original origin/declaration/candidate failures or cross-record identity conflicts.
    pub fn new_with_context(
        definition: SizeSetResolutionIntentDefinition,
        context: &SizeSetChartReplacementContext<'_>,
    ) -> Result<Self, SizeSetResolutionIntentError> {
        let result = Self { definition };
        result.validate_current_with_context(context)?;
        Ok(result)
    }
    /// Original metadata and declaration vector, with no unchecked mutation or sorting.
    #[must_use]
    pub const fn definition(&self) -> &SizeSetResolutionIntentDefinition {
        &self.definition
    }
    /// Recheck against the actual original source snapshot; does not read unresolved values.
    /// # Errors
    /// Returns original failures or identity conflicts; authored chart replacement needs its context.
    pub fn validate_current(
        &self,
        inputs: SizeSetInputs<'_>,
    ) -> Result<(), SizeSetResolutionIntentError> {
        self.validate(inputs, None)
    }
    /// Recheck actual source and replacement chart targets, preserving all original evidence.
    /// # Errors
    /// Returns original origin/declaration/current candidate errors or identity conflicts.
    pub fn validate_current_with_context(
        &self,
        context: &SizeSetChartReplacementContext<'_>,
    ) -> Result<(), SizeSetResolutionIntentError> {
        self.validate(context.inputs(), Some(context))
    }
    fn validate(
        &self,
        inputs: SizeSetInputs<'_>,
        context: Option<&SizeSetChartReplacementContext<'_>>,
    ) -> Result<(), SizeSetResolutionIntentError> {
        use SizeSetResolutionIntentError as Error;
        self.definition
            .origin
            .validate_current(inputs)
            .map_err(|issue| Error::Origin(Box::new(issue)))?;
        let origin = self.definition.origin.definition();
        if context.is_some_and(|context| context.contains_entity(origin.resolved.id)) {
            return Err(Error::DuplicateIdentity(origin.resolved.id));
        }
        let mut identities = BTreeSet::from([origin.resolved.id, origin.provenance]);
        for transformation in &self.definition.transformations {
            let declaration = transformation.definition();
            if !identities.insert(declaration.id) {
                return Err(Error::DuplicateIdentity(declaration.id));
            }
            if declaration.source == origin.resolved.id {
                return Err(Error::DuplicateIdentity(origin.resolved.id));
            }
            let result = match context {
                Some(context) => transformation.validate_current_with_context(context),
                None => transformation.validate_current(inputs),
            };
            result.map_err(|issue| Error::Transformation {
                transformation: declaration.id,
                issue: Box::new(issue),
            })?;
        }
        Ok(())
    }
}
