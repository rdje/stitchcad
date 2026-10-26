//! Exact selected profile labels; G4 owns profile grammar, registry, constraints and physical resolution.
use core::fmt;
use sc_core::ontology::ProfileBindingValidation;

/// Editable selected target pin, keeping roadmap8 schema and content versions distinct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetProfilePinDefinition {
    /// Exact authored profile key, including roadmap keys such as acme-cmt-pt.
    pub profile_id: String,
    /// Exact profile schema-version label, not its content revision.
    pub schema_version: String,
    /// Exact profile content-version label; no latest-version lookup or default.
    pub content_version: String,
}
/// Which explicit pin field lacks content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeSetProfilePinField {
    /// Selected profile key.
    ProfileId,
    /// Schema-version label.
    SchemaVersion,
    /// Content-version label.
    ContentVersion,
}
/// Actual blank pin field, retaining the refused spelling without normalization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetProfilePinError {
    /// Exact field that needs an authored value.
    pub field: SizeSetProfilePinField,
    /// Original refused text, including whitespace.
    pub value: String,
}
impl fmt::Display for SizeSetProfilePinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "size set profile pin {:?} is blank: {:?}",
            self.field, self.value
        )
    }
}
impl std::error::Error for SizeSetProfilePinError {}

/// Immutable explicit selected-profile labels, not a profile-validation or existence certificate.
/// Labels are opaque here; G4 owns profile/schema/content-version grammar and immutable registry binding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetProfilePin {
    definition: SizeSetProfilePinDefinition,
}
impl SizeSetProfilePin {
    /// Require all three authored fields without trimming, parsing versions or selecting a profile.
    /// # Errors
    /// Returns the first blank profile/schema/content field, with its original text.
    pub fn new(definition: SizeSetProfilePinDefinition) -> Result<Self, SizeSetProfilePinError> {
        for (field, value) in [
            (SizeSetProfilePinField::ProfileId, &definition.profile_id),
            (
                SizeSetProfilePinField::SchemaVersion,
                &definition.schema_version,
            ),
            (
                SizeSetProfilePinField::ContentVersion,
                &definition.content_version,
            ),
        ] {
            if value.trim().is_empty() {
                return Err(SizeSetProfilePinError {
                    field,
                    value: value.clone(),
                });
            }
        }
        Ok(Self { definition })
    }
    /// Original profile/schema/content labels, with no unchecked mutation.
    #[must_use]
    pub const fn definition(&self) -> &SizeSetProfilePinDefinition {
        &self.definition
    }
    /// Explicit profile validation/resolution boundary; this pin supplies no physical values or authority.
    #[must_use]
    pub const fn validation(&self) -> ProfileBindingValidation {
        ProfileBindingValidation::DeferredToG4
    }
}
