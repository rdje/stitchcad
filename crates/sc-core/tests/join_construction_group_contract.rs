//! Explicit original retained-source groups and actual output attribution,not partition/target proof.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{id, with_source_actions};
use sc_core::{construction::*, ontology::*};
fn group(side: SeamSideId, index: usize, tags: &[u32]) -> JoinConstructionMappingGroup {
    JoinConstructionMappingGroup::new(
        side,
        index,
        tags.iter().copied().map(LocalTag::new).collect(),
    )
}
fn groups() -> Vec<JoinConstructionMappingGroup> {
    use SeamSideId::{A, B};
    vec![
        group(B, 1, &[13]),
        group(A, 0, &[4]),
        group(B, 0, &[11]),
        group(A, 1, &[4]),
    ]
}
#[test]
fn explicit_reordered_groups_repeat_geometry_across_distinct_source_scopes_and_lend_actual_fields()
{
    with_source_actions(
        true,
        |_, _| JoinConstructionSourceIntent::Retain,
        |sources| {
            let raw = JoinConstructionGroupManifest::new(id(1000), groups());
            let snapshot = raw.clone();
            let limits = JoinConstructionGroupLimits::new(4, 4, 4, 12);
            let declared = raw
                .admit(&sources, limits)
                .expect("all actual Retain scopes grouped");
            assert_eq!(raw.operation(), id(1000));
            assert!(std::ptr::eq(declared.manifest(), &raw));
            assert!(std::ptr::eq(declared.sources(), &sources));
            assert_eq!(declared.limits(), limits);
            assert_eq!(
                (
                    limits.sources(),
                    limits.groups(),
                    limits.references(),
                    limits.workspace_entries()
                ),
                (4, 4, 4, 12)
            );
            assert_eq!(declared.workspace_entries(), 12);
            assert_eq!(declared.group_count(), 4);
            assert_eq!(declared.reference_count(), 4);
            assert_eq!(
                declared.groups().map(|g| g.index()).collect::<Vec<_>>(),
                [0, 1, 2, 3]
            );
            assert_eq!(
                declared
                    .outputs()
                    .rev()
                    .map(|o| *o.tag())
                    .collect::<Vec<_>>(),
                [4, 11, 4, 13].map(LocalTag::new)
            );
            for bound in declared.groups() {
                let original = raw.groups().get(bound.index()).expect("original raw group");
                let source = sources
                    .assignment(original.side(), original.source_index())
                    .expect("actual source action");
                assert!(std::ptr::eq(bound.context(), &declared));
                assert!(std::ptr::eq(bound.definition(), original));
                assert!(std::ptr::eq(
                    bound.source_action().assignment(),
                    source.assignment()
                ));
                assert!(std::ptr::eq(
                    declared
                        .group(original.side(), original.source_index())
                        .expect("actual group")
                        .definition(),
                    original
                ));
                for output in bound.outputs() {
                    let tag = original
                        .outputs()
                        .get(output.index())
                        .expect("actual tag field");
                    assert!(std::ptr::eq(output.context(), &declared));
                    assert_eq!(output.group_index(), bound.index());
                    assert!(std::ptr::eq(output.group_definition(), original));
                    assert!(std::ptr::eq(output.tag(), tag));
                    assert!(std::ptr::eq(
                        output.source_action().source_field(),
                        source.source_field()
                    ));
                    let expected = sources
                        .uses()
                        .connectivity()
                        .contours()
                        .directions()
                        .assignment(*tag)
                        .expect("actual output");
                    assert!(std::ptr::eq(
                        output.output().assignment(),
                        expected.assignment()
                    ));
                    assert!(std::ptr::eq(output.source(), expected.source().source()));
                }
                assert_eq!(
                    bound
                        .outputs()
                        .next_back()
                        .expect("reverse member")
                        .group_index(),
                    bound.index()
                );
            }
            assert_eq!(
                declared
                    .outputs()
                    .map(|o| o.required_traversal())
                    .collect::<Vec<_>>(),
                [
                    Direction::Original,
                    Direction::Original,
                    Direction::Reversed,
                    Direction::Reversed
                ]
            );
            assert!(declared.group(SeamSideId::A, 99).is_none());
            assert_eq!(raw, snapshot);
            assert_eq!(declared.sources().uses().use_count(), 0);
            assert!(format!("{declared:?}").contains("DeclaredJoinConstructionGroups"));
        },
    );
}
#[test]
fn complete_sources_groups_references_and_workspace_precede_owner_and_all_source_tag_fields() {
    use JoinConstructionGroupRefusal as R;
    with_source_actions(
        true,
        |_, _| JoinConstructionSourceIntent::Retain,
        |sources| {
            let raw = JoinConstructionGroupManifest::new(
                id(1001),
                vec![group(SeamSideId::A, 99, &[99]); 4],
            );
            let snapshot = raw.clone();
            for (limits, cause) in [
                (
                    JoinConstructionGroupLimits::new(3, 0, 0, 0),
                    R::Sources {
                        required: 4,
                        allowed: 3,
                    },
                ),
                (
                    JoinConstructionGroupLimits::new(4, 3, 0, 0),
                    R::Groups {
                        required: 4,
                        allowed: 3,
                    },
                ),
                (
                    JoinConstructionGroupLimits::new(4, 4, 3, 0),
                    R::References {
                        required: 4,
                        allowed: 3,
                    },
                ),
                (
                    JoinConstructionGroupLimits::new(4, 4, 4, 11),
                    R::Workspace {
                        required: 12,
                        allowed: 11,
                    },
                ),
                (JoinConstructionGroupLimits::new(4, 4, 4, 12), R::Operation),
            ] {
                let error = raw
                    .admit(&sources, limits)
                    .expect_err("whole quotas/owner before fields");
                assert!(std::ptr::eq(error.manifest(), &raw));
                assert!(std::ptr::eq(error.sources(), &sources));
                assert_eq!(error.limits(), limits);
                assert_eq!(error.refusal(), cause);
                assert!(
                    error.refused_group().is_none()
                        && error.refused_tag().is_none()
                        && error.refused_source_action().is_none()
                        && error.refused_output().is_none()
                );
                assert!(error
                    .to_string()
                    .starts_with("join retained construction groups:"));
                assert!(format!("{error:?}").contains("JoinConstructionGroupError"));
            }
            assert_eq!(raw, snapshot);
        },
    );
}
#[test]
fn exact_count_and_first_original_group_source_and_frame_causes_preserve_actual_refs() {
    use JoinConstructionGroupRefusal as R;
    use SeamSideId::A;
    with_source_actions(
        true,
        |_, _| JoinConstructionSourceIntent::Retain,
        |sources| {
            for g in [3, 5] {
                let raw =
                    JoinConstructionGroupManifest::new(id(1000), vec![group(A, 99, &[99]); g]);
                assert_eq!(
                    raw.admit(&sources, JoinConstructionGroupLimits::new(4, g, g, 3 * g))
                        .expect_err("Retain count before fields")
                        .refusal(),
                    R::Count {
                        expected: 4,
                        actual: g
                    }
                );
            }
            for (replacement, cause, tag) in [
                (
                    group(A, 0, &[99]),
                    R::DuplicateSource {
                        first: 1,
                        second: 3,
                    },
                    None,
                ),
                (group(A, 99, &[4]), R::MissingSource { group: 3 }, None),
                (group(A, 1, &[]), R::EmptyGroup { group: 3 }, None),
                (
                    group(A, 1, &[99]),
                    R::UnknownOutput { group: 3, index: 0 },
                    Some(99),
                ),
                (
                    group(A, 1, &[7]),
                    R::UnknownOutput { group: 3, index: 0 },
                    Some(7),
                ),
                (
                    group(A, 1, &[11]),
                    R::SourceSide { group: 3, index: 0 },
                    Some(11),
                ),
                (
                    group(A, 1, &[2]),
                    R::SourceEdge { group: 3, index: 0 },
                    Some(2),
                ),
            ] {
                let mut values = groups();
                *values.get_mut(3).expect("last original group") = replacement;
                let raw = JoinConstructionGroupManifest::new(id(1000), values);
                let snapshot = raw.clone();
                let limits = JoinConstructionGroupLimits::new(4, 4, 5, 13);
                let error = raw
                    .admit(&sources, limits)
                    .expect_err("first original field contradiction");
                assert_eq!(error.refusal(), cause);
                assert_eq!(raw, snapshot);
                assert_eq!(error.limits(), limits);
                let actual = raw.groups().get(3).expect("actual refused group");
                assert!(std::ptr::eq(
                    error.refused_group().expect("group cause"),
                    actual
                ));
                if let Some(tag) = tag {
                    assert!(std::ptr::eq(
                        error.refused_tag().expect("original tag field"),
                        actual.outputs().first().expect("actual original tag")
                    ));
                    assert_eq!(
                        *error.refused_tag().expect("actual tag"),
                        LocalTag::new(tag)
                    );
                } else {
                    assert!(error.refused_tag().is_none());
                }
                if matches!(cause, R::SourceSide { .. } | R::SourceEdge { .. }) {
                    let output = error.refused_output().expect("actual mismatched output");
                    let actual_output = sources
                        .uses()
                        .connectivity()
                        .contours()
                        .directions()
                        .assignment(*actual.outputs().first().expect("actual tag"))
                        .expect("actual directional output");
                    assert!(std::ptr::eq(
                        output.assignment(),
                        actual_output.assignment()
                    ));
                } else {
                    assert!(error.refused_output().is_none());
                }
                if matches!(cause, R::MissingSource { .. }) {
                    assert!(error.refused_source_action().is_none());
                } else {
                    let expected = sources
                        .assignment(actual.side(), actual.source_index())
                        .expect("actual source action");
                    assert!(std::ptr::eq(
                        error
                            .refused_source_action()
                            .expect("actual guarded source")
                            .assignment(),
                        expected.assignment()
                    ));
                }
            }
        },
    );
}
#[test]
fn omitted_sources_cannot_have_retained_groups_and_raw_partial_or_repeated_refs_claim_no_coverage()
{
    use SeamSideId::{A, B};
    with_source_actions(
        true,
        |_, index| {
            if index == 0 {
                JoinConstructionSourceIntent::Retain
            } else {
                JoinConstructionSourceIntent::Omit {
                    evidence_source: id(0),
                }
            }
        },
        |sources| {
            let raw = JoinConstructionGroupManifest::new(
                id(1000),
                vec![group(A, 0, &[4]), group(A, 1, &[5])],
            );
            let error = raw
                .admit(&sources, JoinConstructionGroupLimits::new(4, 2, 2, 6))
                .expect_err("Omit cannot supply Retain group");
            assert_eq!(
                error.refusal(),
                JoinConstructionGroupRefusal::OmittedSource { group: 1 }
            );
            assert!(matches!(
                error
                    .refused_source_action()
                    .expect("actual omitted source")
                    .intent(),
                JoinConstructionSourceIntent::Omit { .. }
            ));
            let raw = JoinConstructionGroupManifest::new(
                id(1000),
                vec![group(A, 0, &[4]), group(B, 0, &[11])],
            );
            assert_eq!(
                raw.admit(&sources, JoinConstructionGroupLimits::new(4, 2, 2, 6))
                    .expect("every actual Retain source grouped")
                    .group_count(),
                2
            );
        },
    );
    with_source_actions(
        true,
        |_, _| JoinConstructionSourceIntent::Omit {
            evidence_source: id(0),
        },
        |sources| {
            let raw = JoinConstructionGroupManifest::new(id(1000), vec![]);
            let declared = raw
                .admit(&sources, JoinConstructionGroupLimits::new(4, 0, 0, 0))
                .expect("explicit no-Retain group scope");
            assert_eq!(declared.group_count(), 0);
            assert_eq!(declared.reference_count(), 0);
            assert_eq!(declared.workspace_entries(), 0);
            assert_eq!(declared.sources().source_count(), 4); // No accepted omission evidence follows.
        },
    );
    with_source_actions(
        true,
        |_, _| JoinConstructionSourceIntent::Retain,
        |sources| {
            for tags in [&[9][..], &[4, 4][..]] {
                let mut values = groups();
                *values.get_mut(1).expect("actual A0 group") = group(A, 0, tags);
                let raw = JoinConstructionGroupManifest::new(id(1000), values);
                let declared = raw
                    .admit(&sources, JoinConstructionGroupLimits::new(4, 4, 5, 13))
                    .expect("literal attribution only");
                let bound = declared.group(A, 0).expect("actual A0 group");
                assert_eq!(bound.outputs().len(), tags.len());
                assert_eq!(declared.sources().uses().use_count(), 0); // Target-use presence is separate.
                if tags.len() == 1 {
                    assert!(
                        bound
                            .outputs()
                            .next()
                            .expect("partial reference")
                            .source()
                            .to()
                            < Param::END
                    );
                } else {
                    assert_eq!(
                        bound.outputs().map(|o| *o.tag()).collect::<Vec<_>>(),
                        [LocalTag::new(4); 2]
                    );
                }
            }
        },
    );
}
