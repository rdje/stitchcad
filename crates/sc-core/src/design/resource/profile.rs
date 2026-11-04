//! Exact original selected-pin text;no profile lookup,version parsing or physical resolution.
use super::{
    DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
};
use crate::ontology::{EntityId, SelectedProfilePin, SelectedProfilePinDefinition};

/// Append all three actual authored pin strings in profile/schema/content order. Caller supplies
/// the original containing scope,owner and optional authored collection position;no entity/index
/// is manufactured from the opaque profile label. All field tokens remain producer-owned statics.
/// Original text is neither trimmed,normalized,cloned nor interpreted as a version/default policy.
///
/// This composes with complete canonical/body/metadata/cache accounting by the actual source
/// owner. It grants no pin/source association,Profile schema,evidence,currentness,value or whole
/// resource allowance. Earlier pin construction provides no content-budget credit.
/// # Errors
/// Original first text-field refusal or earlier sink latch,with actual caller scope/owner/index.
pub fn charge_selected_profile_pin_resources(
    pin: &SelectedProfilePin,
    sink: &mut dyn DesignResourceSink,
    scope: DesignResourceScope,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> Result<(), DesignResourceLimitError> {
    let SelectedProfilePinDefinition {
        profile_id,
        schema_version,
        content_version,
    } = pin.resource_definition();
    for (field, text) in [
        ("selected_profile.profile_id", profile_id),
        ("selected_profile.schema_version", schema_version),
        ("selected_profile.content_version", content_version),
    ] {
        sink.charge_text(
            text,
            DesignResourceLocation {
                scope,
                owner,
                field,
                index,
            },
        )?;
    }
    Ok(())
}
