//! Explicit actual target occurrences,original traversal and repeated source multiplicity.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_targets.rs"]
mod common;
use common::{id, proposed, row, rows, with_coverage};
use sc_core::{construction::*, ontology::*};
#[test]
fn explicit_interleaved_actual_occurrences_preserve_original_lists_and_borrow_all_fields() {
    with_coverage(proposed(), |coverage| {
        let mut values = rows();
        values.rotate_right(1);
        let raw = JoinConstructionTargetManifest::new(
            id(1000),
            values,
            JoinAdditionalConstructionIntent::SourceMappedOnly,
        );
        let snapshot = raw.clone();
        let limits = JoinConstructionTargetLimits::new(4, 8, 8, 20);
        let declared = raw
            .admit(&coverage, limits)
            .expect("distinct actual occurrence per retained reference");
        assert!(std::ptr::eq(declared.manifest(), &raw));
        assert!(std::ptr::eq(declared.coverage(), &coverage));
        assert_eq!(raw.operation(), id(1000));
        assert_eq!(declared.limits(), limits);
        assert_eq!(
            (
                limits.groups(),
                limits.references(),
                limits.uses(),
                limits.workspace_entries()
            ),
            (4, 8, 8, 20)
        );
        assert_eq!(
            (declared.mapping_count(), declared.workspace_entries()),
            (8, 20)
        );
        assert_eq!(
            declared
                .mappings()
                .map(|m| *m.use_index())
                .collect::<Vec<_>>(),
            [7, 0, 4, 1, 6, 2, 5, 3]
        );
        assert_eq!(
            declared
                .mappings()
                .rev()
                .map(|m| *m.use_index())
                .collect::<Vec<_>>(),
            [3, 5, 2, 6, 1, 4, 0, 7]
        );
        for m in declared.mappings() {
            assert!(std::ptr::eq(m.context(), &declared));
            let row = raw
                .assignments()
                .get(m.group_index())
                .expect("actual raw row");
            assert!(std::ptr::eq(m.assignment(), row));
            assert!(std::ptr::eq(
                m.use_index(),
                row.uses().get(m.index()).expect("actual ordinal field")
            ));
            assert!(std::ptr::eq(
                m.source_group().definition(),
                m.source().group_definition()
            ));
            let target = coverage
                .groups()
                .sources()
                .uses()
                .uses()
                .nth(*m.use_index())
                .expect("actual original target");
            assert!(std::ptr::eq(m.target().declared(), target.declared()));
            assert_eq!(m.target().declared().tag(), m.source().tag());
            assert_eq!(*m.target().direction(), m.source().required_traversal());
            assert!(std::ptr::eq(
                declared
                    .mapping_for_use(*m.use_index())
                    .expect("actual indexed occurrence")
                    .use_index(),
                m.use_index()
            ));
        }
        assert!(declared.mapping_for_use(8).is_none());
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn complete_resources_precede_owner_counts_and_first_invalid_raw_assignment() {
    let mut uses = proposed();
    uses.push(JoinConstructionUse::new(
        LocalTag::new(6),
        Direction::Original,
    ));
    with_coverage(uses, |coverage| {
        let mut values = rows();
        *values.first_mut().expect("first row") =
            row(SeamSideId::A, 99, vec![usize::MAX, usize::MAX]);
        let raw = JoinConstructionTargetManifest::new(
            id(9999),
            values,
            JoinAdditionalConstructionIntent::AllowAdditional {
                evidence_source: id(0),
            },
        );
        use JoinConstructionTargetRefusal as R;
        for (limits, cause) in [
            (
                JoinConstructionTargetLimits::new(3, 0, 0, 0),
                R::Groups {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                JoinConstructionTargetLimits::new(4, 7, 0, 0),
                R::References {
                    required: 8,
                    allowed: 7,
                },
            ),
            (
                JoinConstructionTargetLimits::new(4, 8, 8, 0),
                R::Uses {
                    required: 9,
                    allowed: 8,
                },
            ),
            (
                JoinConstructionTargetLimits::new(4, 8, 9, 19),
                R::Workspace {
                    required: 20,
                    allowed: 19,
                },
            ),
            (JoinConstructionTargetLimits::new(4, 8, 9, 20), R::Operation),
        ] {
            let error = raw
                .admit(&coverage, limits)
                .expect_err("whole resources before raw field errors");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.coverage(), &coverage));
            assert!(error.refused_assignment().is_none());
            assert!(error.refused_use_index().is_none());
            assert!(error.refused_source_group().is_none());
            assert!(error.refused_source_output().is_none());
            assert!(error.refused_use().is_none());
        }
        for (values, cause) in [
            (
                rows().into_iter().take(3).collect(),
                R::Count {
                    expected: 4,
                    actual: 3,
                },
            ),
            (
                {
                    let mut r = rows();
                    r.first_mut()
                        .expect("first row")
                        .clone_from(&row(SeamSideId::A, 0, vec![4]));
                    r
                },
                R::ReferenceCount {
                    expected: 8,
                    actual: 7,
                },
            ),
        ] {
            let raw = JoinConstructionTargetManifest::new(
                id(1000),
                values,
                JoinAdditionalConstructionIntent::SourceMappedOnly,
            );
            let error = raw
                .admit(&coverage, JoinConstructionTargetLimits::new(4, 8, 9, 20))
                .expect_err("exact whole counts");
            assert_eq!(error.refusal(), cause);
            assert!(error.refused_assignment().is_none());
        }
    });
}
#[test]
fn first_original_source_and_target_refusals_keep_actual_raw_and_authentic_field_loans() {
    with_coverage(proposed(), |coverage| {
        use JoinConstructionTargetRefusal as R;
        use SeamSideId::{A, B};
        for (values, cause, g, i) in [
            (
                {
                    let mut r = rows();
                    *r.get_mut(1).expect("row1") = row(A, 0, vec![4, 1]);
                    r
                },
                R::DuplicateSource {
                    first: 0,
                    second: 1,
                },
                1,
                None,
            ),
            (
                {
                    let mut r = rows();
                    *r.first_mut().expect("row0") = row(B, 99, vec![4, 1]);
                    r
                },
                R::MissingSource { group: 0 },
                0,
                None,
            ),
            (
                vec![
                    row(A, 0, vec![4]),
                    row(A, 1, vec![6, 2, 1]),
                    row(B, 0, vec![5, 3]),
                    row(B, 1, vec![7, 0]),
                ],
                R::GroupReferenceCount {
                    group: 0,
                    expected: 2,
                    actual: 1,
                },
                0,
                None,
            ),
            (
                {
                    let mut r = rows();
                    *r.get_mut(1).expect("row1") = row(A, 1, vec![4, 2]);
                    r
                },
                R::DuplicateUse {
                    first_group: 0,
                    first_index: 0,
                    group: 1,
                    index: 0,
                },
                1,
                Some(0),
            ),
            (
                {
                    let mut r = rows();
                    *r.get_mut(1).expect("row1") = row(A, 1, vec![6, usize::MAX]);
                    r
                },
                R::MissingUse { group: 1, index: 1 },
                1,
                Some(1),
            ),
            (
                {
                    let mut r = rows();
                    *r.get_mut(1).expect("row1") = row(A, 1, vec![3, 2]);
                    r
                },
                R::Output { group: 1, index: 0 },
                1,
                Some(0),
            ),
            (
                {
                    let mut r = rows();
                    *r.first_mut().expect("row0") = row(A, 0, vec![2, 1]);
                    r
                },
                R::Traversal { group: 0, index: 0 },
                0,
                Some(0),
            ),
        ] {
            let raw = JoinConstructionTargetManifest::new(
                id(1000),
                values,
                JoinAdditionalConstructionIntent::SourceMappedOnly,
            );
            let snapshot = raw.clone();
            let error = raw
                .admit(&coverage, JoinConstructionTargetLimits::new(4, 8, 8, 20))
                .expect_err("actual source/target contract");
            assert_eq!(error.refusal(), cause);
            let actual = error.refused_assignment().expect("original refused row");
            assert!(std::ptr::eq(
                actual,
                raw.assignments().get(g).expect("raw row")
            ));
            if matches!(cause, R::MissingSource { .. }) {
                assert!(error.refused_source_group().is_none());
            } else {
                let source = error.refused_source_group().expect("actual source group");
                assert_eq!(
                    (
                        source.definition().side(),
                        source.definition().source_index()
                    ),
                    (actual.side(), actual.source_index())
                );
            }
            if let Some(index) = i {
                assert!(std::ptr::eq(
                    error.refused_use_index().expect("actual ordinal"),
                    actual.uses().get(index).expect("raw field")
                ));
                let source = error.refused_source_output().expect("actual source output");
                assert_eq!(source.index(), index);
                if matches!(cause, R::MissingUse { .. }) {
                    assert!(error.refused_use().is_none());
                } else {
                    let target = error.refused_use().expect("actual proposed occurrence");
                    assert_eq!(
                        target.index(),
                        *actual.uses().get(index).expect("target ordinal")
                    );
                    let original = coverage
                        .groups()
                        .sources()
                        .uses()
                        .manifest()
                        .uses()
                        .get(target.index())
                        .expect("actual raw proposed field");
                    assert!(std::ptr::eq(target.declared(), original));
                }
            } else {
                assert!(error.refused_use_index().is_none());
                assert!(error.refused_source_output().is_none());
                assert!(error.refused_use().is_none());
            }
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn missing_targets_cannot_erase_retained_fields_and_additional_policy_is_only_retained_here() {
    with_coverage(vec![], |coverage| {
        let raw = JoinConstructionTargetManifest::new(
            id(1000),
            rows(),
            JoinAdditionalConstructionIntent::SourceMappedOnly,
        );
        let error = raw
            .admit(&coverage, JoinConstructionTargetLimits::new(4, 8, 0, 20))
            .expect_err("empty targets cannot carry eight retained references");
        assert_eq!(
            error.refusal(),
            JoinConstructionTargetRefusal::MissingUse { group: 0, index: 0 }
        );
        assert!(error.refused_use().is_none());
    });
    let mut uses = proposed();
    uses.push(JoinConstructionUse::new(
        LocalTag::new(6),
        Direction::Original,
    ));
    with_coverage(uses, |coverage| {
        for intent in [
            JoinAdditionalConstructionIntent::SourceMappedOnly,
            JoinAdditionalConstructionIntent::AllowAdditional {
                evidence_source: id(0),
            },
        ] {
            let raw = JoinConstructionTargetManifest::new(id(1000), rows(), intent);
            let declared = raw
                .admit(&coverage, JoinConstructionTargetLimits::new(4, 8, 9, 20))
                .expect("source assignment only;leftover check remains");
            assert!(std::ptr::eq(
                declared.manifest().additional(),
                raw.additional()
            ));
            assert_eq!(*raw.additional(), intent);
            assert_eq!(declared.mapping_count(), 8);
            assert!(declared.mapping_for_use(8).is_none());
            assert_eq!(declared.coverage().groups().sources().uses().use_count(), 9);
        }
    });
}
