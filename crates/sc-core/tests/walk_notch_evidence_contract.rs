//! Mandatory distinct original evidence requests;none is a provider verdict or execution authority.
#![allow(clippy::expect_used)]
#[path = "support/walk_fields.rs"]
mod common;
use common::{known::existing::assignments::roles::copies::input::id, with_fields};
use sc_core::construction::*;
const EXPECTED: [WalkNotchEvidenceKind; 10] = [
    WalkNotchEvidenceKind::CurrentOwnership,
    WalkNotchEvidenceKind::IdentityHistory,
    WalkNotchEvidenceKind::CanonicalNotches,
    WalkNotchEvidenceKind::VisibleRepairs,
    WalkNotchEvidenceKind::Correspondence,
    WalkNotchEvidenceKind::Differential,
    WalkNotchEvidenceKind::EaseAndStops,
    WalkNotchEvidenceKind::Profiles,
    WalkNotchEvidenceKind::AllCopyEffects,
    WalkNotchEvidenceKind::AuthoringAuthority,
];
#[test]
fn all_ten_distinct_scopes_lend_original_source_fields_and_complete_actual_field_context() {
    with_fields(1, |fields| {
        let context = WalkNotchContextSources::new(id(10000), id(10001), id(10002), id(10003));
        let effects = WalkNotchEffectSources::new(
            id(10004),
            id(10005),
            id(10006),
            id(10007),
            id(10008),
            id(10009),
        );
        let policy = WalkNotchEvidencePolicy::new(context, effects);
        let snapshot = policy;
        assert_eq!(*policy.context_sources(), context);
        assert_eq!(*policy.effect_sources(), effects);
        let required = fields.require_evidence(&policy);
        assert!(std::ptr::eq(required.fields(), &fields));
        assert!(std::ptr::eq(required.policy(), &policy));
        assert_eq!(required.requirements().len(), 10);
        for ((item, expected), index) in required.requirements().zip(EXPECTED).zip(0u128..) {
            assert!(std::ptr::eq(item.context(), &required));
            assert_eq!(item.kind(), expected);
            assert!(std::ptr::eq(item.source(), policy.source(expected)));
            assert_eq!(*item.source(), id(10000 + index));
        }
        assert_eq!(
            required
                .requirements()
                .rev()
                .map(|r| r.kind())
                .collect::<Vec<_>>(),
            EXPECTED.into_iter().rev().collect::<Vec<_>>()
        );
        assert_eq!(policy, snapshot);
        assert_eq!(fields.identities().retention_count(), 1);
        assert_eq!(fields.identities().proposal_count(), 1);
        assert!(format!("{required:?}").contains("RequiredWalkNotchEvidence"));
    });
}
#[test]
fn shared_zero_source_values_are_ordinary_requests_and_never_remove_or_merge_obligations() {
    with_fields(1, |fields| {
        let zero = id(0);
        let policy = WalkNotchEvidencePolicy::new(
            WalkNotchContextSources::new(zero, zero, zero, zero),
            WalkNotchEffectSources::new(zero, zero, zero, zero, zero, zero),
        );
        let required = fields.require_evidence(&policy);
        let requests = required.requirements().collect::<Vec<_>>();
        assert_eq!(requests.len(), 10);
        for (item, kind) in requests.iter().zip(EXPECTED) {
            assert_eq!(item.kind(), kind);
            assert_eq!(*item.source(), zero);
            assert!(std::ptr::eq(item.source(), policy.source(kind)));
        }
        for (index, first) in requests.iter().enumerate() {
            for second in requests.iter().skip(index + 1) {
                assert_ne!(first.kind(), second.kind());
                assert!(!std::ptr::eq(first.source(), second.source()));
            }
        }
    });
}
#[test]
fn four_explicit_original_occurrences_and_two_shared_groups_keep_every_evidence_scope() {
    with_fields(2, |fields| {
        let p = id(90000);
        let policy = WalkNotchEvidencePolicy::new(
            WalkNotchContextSources::new(p, p, p, p),
            WalkNotchEffectSources::new(p, p, p, p, p, p),
        );
        let required = fields.require_evidence(&policy);
        assert_eq!(
            required
                .requirements()
                .map(|r| r.kind())
                .collect::<Vec<_>>(),
            EXPECTED
        );
        let actual = required.fields();
        assert_eq!(actual.group_count(), 2);
        assert!(actual.groups().all(|g| g.occurrence_count() == 2));
        assert_eq!(
            actual
                .identities()
                .known()
                .existing()
                .assignments()
                .request_count(),
            4
        );
        assert_eq!(actual.identities().known().existing().notches().len(), 2);
        assert!(std::ptr::eq(actual.manifest(), fields.manifest()));
        assert!(std::ptr::eq(actual.identities(), fields.identities()));
    });
}
