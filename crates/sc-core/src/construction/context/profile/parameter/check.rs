//! Exhaustive exact original source header and parameter identity before canonical member lookup.
use super::{DraftingProfileLogicalIssue as I, DraftingProfileLogicalSource};
use crate::{
    construction::DraftingProfileSource,
    design::{fixed_value, DesignMetadataHost},
    ontology::ProfileParameterRef,
};
pub(super) fn issue<H: DesignMetadataHost, S: ?Sized, P: ?Sized, M: ?Sized>(
    expected: DraftingProfileSource<'_, H, S, P>,
    parameter: ProfileParameterRef,
    returned: DraftingProfileLogicalSource<'_, H, S, P, M>,
) -> Option<I> {
    let DraftingProfileLogicalSource {
        profile,
        parameter: actual,
        declaration,
        metadata,
    } = returned;
    // These original loans are retained;canonical kind/member check follows this header check.
    fixed_value(&declaration);
    fixed_value(&metadata);
    let DraftingProfileSource {
        root,
        selected,
        inputs,
        profile,
        snapshot,
    } = profile;
    if !core::ptr::eq(root, expected.root) {
        return Some(I::Root);
    }
    if !core::ptr::eq(selected, expected.selected) {
        return Some(I::SelectedPin);
    }
    if !core::ptr::eq(inputs, expected.inputs) {
        return Some(I::Inventory);
    }
    if !profile.is_same_record(expected.profile) {
        return Some(I::Profile);
    }
    if !snapshot.is_same_record(expected.snapshot) {
        return Some(I::Snapshot);
    }
    if actual.id() != parameter.id() {
        return Some(I::Parameter);
    }
    None
}
