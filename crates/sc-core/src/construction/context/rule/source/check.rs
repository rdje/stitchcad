//! Exhaustive original source headers before exact rule identity/family;no signature/value matching.
use super::{
    AdmittedDraftingRuleWork, DraftingRuleProvider, DraftingRuleResponse,
    DraftingRuleSourceIssue as I, Profile,
};
use crate::{
    construction::{DraftingProfileSource, DraftingRuleOrigin, DraftingRuleSource},
    design::{fixed_value, DesignMetadataHost},
};
pub(super) fn issue<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>(
    work: &AdmittedDraftingRuleWork<'_, H, S>,
    expected: Option<Profile<'_, H, S>>,
    source: DraftingRuleResponse<'_, H, S>,
) -> Option<I> {
    let DraftingRuleSource {
        declaration,
        body,
        origin,
    } = source;
    fixed_value(&body); // Original opaque body association is the honest source contract.
    if !core::ptr::eq(origin.root(), work.context().scope().root()) {
        return Some(I::Root);
    }
    if !origin.snapshot().is_same_record(work.context().snapshot()) {
        return Some(I::Snapshot);
    }
    match (expected, origin) {
        (None, DraftingRuleOrigin::Design { root, snapshot }) => {
            fixed_value(&root);
            fixed_value(&snapshot);
        }
        (Some(expected), DraftingRuleOrigin::Profile(actual)) => {
            let DraftingProfileSource {
                root,
                selected,
                inputs,
                profile,
                snapshot,
            } = actual;
            let DraftingProfileSource {
                root: expected_root,
                selected: expected_pin,
                inputs: expected_inputs,
                profile: expected_profile,
                snapshot: expected_snapshot,
            } = expected;
            // Root/snapshot already matched the same actual context through both admissions.
            fixed_value(&root);
            fixed_value(&snapshot);
            fixed_value(&expected_root);
            fixed_value(&expected_snapshot);
            if !core::ptr::eq(selected, expected_pin) {
                return Some(I::SelectedPin);
            }
            if !core::ptr::eq(inputs, expected_inputs) {
                return Some(I::Inventory);
            }
            if !profile.is_same_record(expected_profile) {
                return Some(I::Profile);
            }
        }
        _ => return Some(I::Origin),
    }
    let requested = work.request();
    if declaration.id() != requested.rule {
        return Some(I::Rule);
    }
    if declaration.kind() != requested.kind {
        return Some(I::Kind);
    }
    None
}
