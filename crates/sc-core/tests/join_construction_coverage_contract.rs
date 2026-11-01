//! Exact ordered whole-field source partitions,independent of target presence or omission evidence.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{id, with_partitioned_sources, with_source_actions};
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
        group(A, 0, &[4, 5]),
        group(A, 1, &[5, 4]),
        group(B, 0, &[11, 12]),
        group(B, 1, &[12, 11]),
    ]
}
fn admit<'a>(
    raw: &'a JoinConstructionGroupManifest,
    sources: &'a DeclaredJoinConstructionSources<'a>,
) -> DeclaredJoinConstructionGroups<'a> {
    let l = raw.groups().iter().map(|g| g.outputs().len()).sum();
    raw.admit(sources, JoinConstructionGroupLimits::new(4, 4, l, 8 + l))
        .expect("authentic literal source groups")
}
#[test]
fn exact_fragments_cover_original_and_reversed_repeated_fields_without_target_presence() {
    with_partitioned_sources(vec![], |sources| {
        let raw = JoinConstructionGroupManifest::new(id(1000), groups());
        let snapshot = raw.clone();
        let declared = admit(&raw, &sources);
        let limits = JoinConstructionCoverageLimits::new(4, 8);
        let covered = declared
            .require_source_coverage(limits)
            .expect("four whole ordered source fields");
        assert!(std::ptr::eq(covered.groups(), &declared));
        assert_eq!(covered.limits(), limits);
        assert_eq!((limits.groups(), limits.references()), (4, 8));
        assert_eq!((covered.group_count(), covered.reference_count()), (4, 8));
        assert_eq!(covered.groups().sources().uses().use_count(), 0);
        for (actual, original) in covered.groups().groups().zip(raw.groups()) {
            assert!(std::ptr::eq(actual.definition(), original));
            let field = actual.source_action().source_field();
            assert_eq!(
                field.direction,
                if original.source_index() == 0 {
                    Direction::Original
                } else {
                    Direction::Reversed
                }
            );
            assert_eq!(
                actual.outputs().map(|o| *o.tag()).collect::<Vec<_>>(),
                original.outputs()
            );
        }
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn whole_group_and_reference_quotas_precede_first_invalid_interval_without_field_loans() {
    with_partitioned_sources(vec![], |sources| {
        let mut values = groups();
        *values.first_mut().expect("first group") = group(SeamSideId::A, 0, &[5, 4]);
        let raw = JoinConstructionGroupManifest::new(id(1000), values);
        let declared = admit(&raw, &sources);
        use JoinConstructionCoverageRefusal as R;
        for (limits, expected) in [
            (
                JoinConstructionCoverageLimits::new(3, 0),
                R::Groups {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                JoinConstructionCoverageLimits::new(4, 7),
                R::References {
                    required: 8,
                    allowed: 7,
                },
            ),
        ] {
            let error = declared
                .require_source_coverage(limits)
                .expect_err("whole quotas before bad first range");
            assert_eq!(error.refusal(), expected);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.groups(), &declared));
            assert!(error.refused_group().is_none());
            assert!(error.refused_output().is_none());
        }
    });
}
#[test]
fn gaps_overlap_and_wrong_order_keep_original_first_group_and_actual_reference_witnesses() {
    with_partitioned_sources(vec![], |sources| {
        use JoinConstructionCoverageRefusal as R;
        let half = Param::new(Rational::new(1, 2).expect("half")).expect("unit half");
        for (g, tags, expected) in [
            (
                0,
                vec![5],
                R::Gap {
                    group: 0,
                    index: Some(0),
                    from: Param::START,
                    to: half,
                },
            ),
            (
                0,
                vec![4],
                R::Gap {
                    group: 0,
                    index: None,
                    from: half,
                    to: Param::END,
                },
            ),
            (
                1,
                vec![4],
                R::Gap {
                    group: 1,
                    index: Some(0),
                    from: half,
                    to: Param::END,
                },
            ),
            (
                1,
                vec![5],
                R::Gap {
                    group: 1,
                    index: None,
                    from: Param::START,
                    to: half,
                },
            ),
            (
                0,
                vec![4, 4],
                R::Overlap {
                    group: 0,
                    index: 1,
                    cursor: half,
                },
            ),
            (
                1,
                vec![5, 5],
                R::Overlap {
                    group: 1,
                    index: 1,
                    cursor: half,
                },
            ),
            (
                0,
                vec![5, 4],
                R::Gap {
                    group: 0,
                    index: Some(0),
                    from: Param::START,
                    to: half,
                },
            ),
            (
                1,
                vec![4, 5],
                R::Gap {
                    group: 1,
                    index: Some(0),
                    from: half,
                    to: Param::END,
                },
            ),
        ] {
            let mut values = groups();
            let original = values.get(g).expect("original group");
            let replacement = group(original.side(), original.source_index(), &tags);
            *values.get_mut(g).expect("changed group") = replacement;
            let raw = JoinConstructionGroupManifest::new(id(1000), values);
            let snapshot = raw.clone();
            let declared = admit(&raw, &sources);
            let error = declared
                .require_source_coverage(JoinConstructionCoverageLimits::new(4, 8))
                .expect_err("exact original order required");
            assert_eq!(error.refusal(), expected);
            let actual = error.refused_group().expect("actual interval group");
            assert!(std::ptr::eq(
                actual.definition(),
                raw.groups().get(g).expect("raw original group")
            ));
            assert!(std::ptr::eq(
                actual.source_action().source_field(),
                sources
                    .assignment(
                        actual.definition().side(),
                        actual.definition().source_index()
                    )
                    .expect("actual source action")
                    .source_field()
            ));
            let index = match expected {
                R::Gap { index, .. } => index,
                R::Overlap { index, .. } => Some(index),
                _ => None,
            };
            if let Some(i) = index {
                let output = error.refused_output().expect("actual offending reference");
                let expected_output = actual.outputs().nth(i).expect("actual original reference");
                assert!(std::ptr::eq(
                    output.tag(),
                    actual.definition().outputs().get(i).expect("raw tag")
                ));
                assert!(std::ptr::eq(output.source(), expected_output.source()));
                assert_eq!((output.group_index(), output.index()), (g, i));
            } else {
                assert!(error.refused_output().is_none());
            }
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn whole_ranges_and_explicit_no_retain_scope_do_not_accept_omission_or_target_effects() {
    for retain in [true, false] {
        with_source_actions(
            true,
            |_, _| {
                if retain {
                    JoinConstructionSourceIntent::Retain
                } else {
                    JoinConstructionSourceIntent::Omit {
                        evidence_source: id(0),
                    }
                }
            },
            |sources| {
                use SeamSideId::{A, B};
                let values = if retain {
                    vec![
                        group(A, 0, &[4]),
                        group(A, 1, &[4]),
                        group(B, 0, &[11]),
                        group(B, 1, &[11]),
                    ]
                } else {
                    vec![]
                };
                let n = values.len();
                let raw = JoinConstructionGroupManifest::new(id(1000), values);
                let declared = raw
                    .admit(&sources, JoinConstructionGroupLimits::new(4, n, n, 3 * n))
                    .expect("explicit complete Retain scopes");
                let covered = declared
                    .require_source_coverage(JoinConstructionCoverageLimits::new(n, n))
                    .expect("whole source intervals or explicit no-Retain scope");
                assert_eq!(covered.group_count(), n);
                assert_eq!(covered.groups().sources().source_count(), 4);
                assert_eq!(covered.groups().sources().uses().use_count(), 0);
                if !retain {
                    assert!(covered
                        .groups()
                        .sources()
                        .assignments()
                        .all(|a| a.evidence_source() == Some(&id(0))));
                }
            },
        );
    }
}
