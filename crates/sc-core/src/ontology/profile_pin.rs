//! Exact selected profile labels; G4 owns profile grammar, registry, constraints and physical resolution.
use super::ProfileBindingValidation;
use core::fmt;

/// Editable selected target pin, keeping roadmap8 schema and content versions distinct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedProfilePinDefinition {
    /// Exact authored profile key, including roadmap keys such as acme-cmt-pt.
    pub profile_id: String,
    /// Exact profile schema-version label, not its content revision.
    pub schema_version: String,
    /// Exact profile content-version label; no latest-version lookup or default.
    pub content_version: String,
}
/// Which explicit pin field lacks content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectedProfilePinField {
    /// Selected profile key.
    ProfileId,
    /// Schema-version label.
    SchemaVersion,
    /// Content-version label.
    ContentVersion,
}
/// Actual blank pin field, retaining the refused spelling without normalization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedProfilePinError {
    /// Exact field that needs an authored value.
    pub field: SelectedProfilePinField,
    /// Original refused text, including whitespace.
    pub value: String,
}
impl fmt::Display for SelectedProfilePinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "selected profile pin {:?} is blank: {:?}",
            self.field, self.value
        )
    }
}
impl std::error::Error for SelectedProfilePinError {}

/// Immutable explicit selected-profile labels, not a profile-validation or existence certificate.
/// Labels are opaque here; G4 owns profile/schema/content-version grammar and immutable registry binding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedProfilePin {
    definition: SelectedProfilePinDefinition,
}
impl SelectedProfilePin {
    /// Require all three authored fields without trimming, parsing versions or selecting a profile.
    /// # Errors
    /// Returns the first blank profile/schema/content field, with its original text.
    pub fn new(definition: SelectedProfilePinDefinition) -> Result<Self, SelectedProfilePinError> {
        for (field, value) in [
            (SelectedProfilePinField::ProfileId, &definition.profile_id),
            (
                SelectedProfilePinField::SchemaVersion,
                &definition.schema_version,
            ),
            (
                SelectedProfilePinField::ContentVersion,
                &definition.content_version,
            ),
        ] {
            if value.trim().is_empty() {
                return Err(SelectedProfilePinError {
                    field,
                    value: value.clone(),
                });
            }
        }
        Ok(Self { definition })
    }
    /// Original profile/schema/content labels, with no unchecked mutation.
    #[must_use]
    pub const fn definition(&self) -> &SelectedProfilePinDefinition {
        &self.definition
    }
    /// Explicit profile validation/resolution boundary; this pin supplies no physical values or authority.
    #[must_use]
    pub const fn validation(&self) -> ProfileBindingValidation {
        ProfileBindingValidation::DeferredToG4
    }
}
