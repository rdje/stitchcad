//! Exact original Root/requested labels/snapshot;canonical/body semantics remain provider-owned.
use super::{DraftingProfileProvider, DraftingProfileSource, DraftingProfileSourceIssue as I};
use crate::{
    construction::DraftingContext,
    design::{fixed_value, DesignMetadataHost},
    ontology::{SelectedProfilePin, SelectedProfilePinDefinition, SelectedProfilePinField as F},
};
fn pin(expected: &SelectedProfilePin, actual: &SelectedProfilePin) -> Option<F> {
    let SelectedProfilePinDefinition {
        profile_id: a,
        schema_version: b,
        content_version: c,
    } = expected.resource_definition();
    let SelectedProfilePinDefinition {
        profile_id: x,
        schema_version: y,
        content_version: z,
    } = actual.resource_definition();
    [
        (F::ProfileId, a, x),
        (F::SchemaVersion, b, y),
        (F::ContentVersion, c, z),
    ]
    .into_iter()
    .find_map(|(field, a, b)| (a != b).then_some(field))
}
pub(super) fn issue<H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized>(
    context: DraftingContext<'_, H, S>,
    selected: &SelectedProfilePin,
    source: DraftingProfileSource<'_, H, S::Snapshot, S::Profile>,
) -> Option<I> {
    // Exhaustive response:added fields require explicit association/accounting decisions.
    let DraftingProfileSource {
        root,
        selected: actual,
        inputs,
        profile,
        snapshot,
    } = source;
    // Original borrowed provider associations;no copied content or G4 proof is granted here.
    fixed_value(&inputs);
    fixed_value(&profile);
    if !core::ptr::eq(root, context.scope().root()) {
        return Some(I::Root);
    }
    if let Some(field) = pin(selected, actual) {
        return Some(I::SelectedPin(field));
    }
    if !snapshot.is_same_record(context.snapshot()) {
        return Some(I::Snapshot);
    }
    None
}
