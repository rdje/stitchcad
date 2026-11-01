//! Whole declared curve alternatives,original priority and authentic attribution;no feasibility.
#![allow(clippy::expect_used)]
#[path = "support/truing_lineage.rs"]
mod fixture;
use fixture::{
    contours::{
        piece::common::{
            domain::{q, roles::copies::id},
            with_outside,
        },
        FoldFixture,
    },
    with_lineage,
};
use sc_core::{construction::*, ontology::*};
fn geometry(creator: u128, direction: Direction) -> DirectedRange {
    DirectedRange {
        range: EdgeRange::new(
            EdgeRef::new(id(creator), LocalTag::new(3)),
            q(1, 5),
            q(4, 5),
        )
        .expect("positive original curve frame"),
        direction,
    }
}
fn group(tag: u32, geometries: Vec<DirectedRange>) -> TruingCurveAlternatives {
    TruingCurveAlternatives::new(LocalTag::new(tag), geometries)
}
fn groups() -> Vec<TruingCurveAlternatives> {
    let repeated = geometry(2000, Direction::Reversed);
    vec![
        group(
            11,
            vec![
                geometry(2001, Direction::Original),
                geometry(2002, Direction::Reversed),
            ],
        ),
        group(
            9,
            vec![repeated, repeated, geometry(2003, Direction::Original)],
        ),
    ]
}
fn raw(operation: u128, groups: Vec<TruingCurveAlternatives>) -> TruingCurveManifest {
    TruingCurveManifest::new(
        id(operation),
        TruingCurveSelectionPolicy::FirstFeasibleCompleteAssignment,
        groups,
    )
}
#[test]
fn complete_groups_and_repeated_choices_keep_authored_priority_distinct_frames_and_actual_parent_fields(
) {
    for side in [SeamSideId::A, SeamSideId::B] {
        for variant in [
            FoldFixture::TargetBoundary,
            FoldFixture::TargetHole,
            FoldFixture::UnrelatedBoundary,
            FoldFixture::NoFold,
        ] {
            with_lineage(side, TruingOutcomeIntent::Refit, variant, |lineage| {
                let raw = raw(1000, groups());
                let snapshot = raw.clone();
                let limits = TruingCurveLimits::new(2, 2, 5, 6, 4);
                let admitted = raw
                    .admit(&lineage, limits)
                    .expect("complete unique selected-role coverage");
                assert!(std::ptr::eq(admitted.manifest(), &raw));
                assert!(std::ptr::eq(admitted.lineage(), &lineage));
                assert!(std::ptr::eq(admitted.policy(), raw.policy()));
                assert_eq!(
                    *raw.policy(),
                    TruingCurveSelectionPolicy::FirstFeasibleCompleteAssignment
                );
                assert_eq!(raw.operation(), id(1000));
                assert_eq!(admitted.limits(), limits);
                assert_eq!(
                    (
                        limits.selected_outputs(),
                        limits.groups(),
                        limits.choices(),
                        limits.combinations(),
                        limits.workspace_entries()
                    ),
                    (2, 2, 5, 6, 4)
                );
                assert_eq!(
                    (
                        admitted.choice_count(),
                        admitted.combination_count(),
                        admitted.workspace_entries()
                    ),
                    (5, 6, 4)
                );
                assert_eq!(admitted.groups().len(), 2);
                let directions = lineage.fold().contours().directions();
                for (index, group) in admitted.groups().enumerate() {
                    let original = raw.groups().get(index).expect("actual original group");
                    assert!(std::ptr::eq(group.context(), &admitted));
                    assert_eq!(group.index(), index);
                    assert!(std::ptr::eq(group.proposal(), original));
                    assert!(std::ptr::eq(group.output().context(), directions));
                    assert_eq!(group.output().assignment().tag(), original.tag());
                    assert!(matches!(
                        group.output().source(),
                        TruingDirectionSource::Selected(_)
                    ));
                    assert_eq!(group.alternatives().len(), original.geometries().len());
                    assert_eq!(
                        admitted
                            .group(*original.tag())
                            .expect("authentic tag query")
                            .index(),
                        index
                    );
                    for (choice, alternative) in group.alternatives().enumerate() {
                        assert_eq!(alternative.index(), choice);
                        assert_eq!(alternative.group().index(), index);
                        assert!(std::ptr::eq(alternative.group().context(), &admitted));
                        assert!(std::ptr::eq(
                            alternative.geometry(),
                            original
                                .geometries()
                                .get(choice)
                                .expect("actual original choice field")
                        ));
                        assert_eq!(
                            (
                                alternative.geometry().range.from(),
                                alternative.geometry().range.to()
                            ),
                            (q(1, 5), q(4, 5))
                        );
                    }
                    assert_eq!(
                        group
                            .alternatives()
                            .rev()
                            .map(|c| c.index())
                            .collect::<Vec<_>>(),
                        (0..original.geometries().len()).rev().collect::<Vec<_>>()
                    );
                }
                assert_eq!(
                    admitted
                        .groups()
                        .rev()
                        .map(|g| *g.proposal().tag())
                        .collect::<Vec<_>>(),
                    vec![LocalTag::new(9), LocalTag::new(11)]
                );
                let repeated = admitted.group(LocalTag::new(9)).expect("actual selected9");
                let first = repeated
                    .alternatives()
                    .next()
                    .expect("first original occurrence");
                let second = repeated
                    .alternatives()
                    .nth(1)
                    .expect("second equal original occurrence");
                assert_eq!(first.geometry(), second.geometry());
                assert!(!std::ptr::eq(first.geometry(), second.geometry()));
                for tag in [2, 5, 7, 99] {
                    assert!(admitted.group(LocalTag::new(tag)).is_none());
                }
                assert_eq!(raw, snapshot);
                assert!(format!("{admitted:?}").contains("AdmittedTruingCurveManifest"));
            });
        }
    }
}
#[test]
fn whole_resources_precede_wrong_owner_and_original_group_or_geometry_errors_without_a_prefix_result(
) {
    use TruingCurveRefusal as R;
    with_lineage(
        SeamSideId::B,
        TruingOutcomeIntent::Refit,
        FoldFixture::TargetBoundary,
        |lineage| {
            let g = geometry(999, Direction::Reversed);
            let wrong = raw(999, vec![group(9, vec![g, g, g]), group(9, vec![g, g])]);
            for (limits, cause) in [
                (
                    TruingCurveLimits::new(1, 1, 4, 5, 3),
                    R::SelectedOutputs {
                        required: 2,
                        allowed: 1,
                    },
                ),
                (
                    TruingCurveLimits::new(2, 1, 4, 5, 3),
                    R::Groups {
                        required: 2,
                        allowed: 1,
                    },
                ),
                (
                    TruingCurveLimits::new(2, 2, 4, 5, 3),
                    R::Choices {
                        required: 5,
                        allowed: 4,
                    },
                ),
                (
                    TruingCurveLimits::new(2, 2, 5, 5, 3),
                    R::Combinations {
                        required: 6,
                        allowed: 5,
                    },
                ),
                (
                    TruingCurveLimits::new(2, 2, 5, 6, 3),
                    R::Workspace {
                        required: 4,
                        allowed: 3,
                    },
                ),
                (TruingCurveLimits::new(2, 2, 5, 6, 4), R::Operation),
            ] {
                let error = wrong
                    .admit(&lineage, limits)
                    .expect_err("whole resources before owner/duplicate/self source");
                assert!(std::ptr::eq(error.manifest(), &wrong));
                assert!(std::ptr::eq(error.lineage(), &lineage));
                assert_eq!(error.limits(), limits);
                assert_eq!(error.refusal(), cause);
                assert!(error.to_string().contains("truing curve alternatives"));
            }
            let c = geometry(2000, Direction::Original);
            let own = geometry(1000, Direction::Reversed);
            for (proposal, cause) in [
                (
                    raw(1000, vec![]),
                    R::Count {
                        expected: 2,
                        actual: 0,
                    },
                ),
                (
                    raw(1000, vec![group(9, vec![c])]),
                    R::Count {
                        expected: 2,
                        actual: 1,
                    },
                ),
                (
                    raw(1000, vec![group(9, vec![c]), group(9, vec![c])]),
                    R::Duplicate {
                        first: 0,
                        second: 1,
                        tag: LocalTag::new(9),
                    },
                ),
                (
                    raw(1000, vec![group(9, vec![c]), group(2, vec![c])]),
                    R::UnknownSelectedOutput {
                        index: 1,
                        tag: LocalTag::new(2),
                    },
                ),
                (
                    raw(1000, vec![group(9, vec![c]), group(99, vec![c])]),
                    R::UnknownSelectedOutput {
                        index: 1,
                        tag: LocalTag::new(99),
                    },
                ),
                (
                    raw(1000, vec![group(9, vec![]), group(11, vec![c])]),
                    R::EmptyGroup {
                        index: 0,
                        tag: LocalTag::new(9),
                    },
                ),
                (
                    raw(1000, vec![group(9, vec![c]), group(11, vec![c, own])]),
                    R::SelfSource {
                        group: 1,
                        choice: 1,
                    },
                ),
            ] {
                let snapshot = proposal.clone();
                let error = proposal
                    .admit(&lineage, TruingCurveLimits::new(2, 2, 5, 6, 4))
                    .expect_err("complete local original cause");
                assert!(std::ptr::eq(error.manifest(), &proposal));
                assert!(std::ptr::eq(error.lineage(), &lineage));
                assert_eq!(error.refusal(), cause);
                if let R::SelfSource { group, choice } = cause {
                    let field = proposal
                        .groups()
                        .get(group)
                        .and_then(|g| g.geometries().get(choice))
                        .expect("actual original refused geometry");
                    assert!(std::ptr::eq(
                        error.refused_geometry().expect("original self source"),
                        field
                    ));
                    assert_eq!(field, &own);
                } else {
                    assert!(error.refused_geometry().is_none());
                }
                assert_eq!(proposal, snapshot);
            }
        },
    );
}
#[test]
fn complete_product_overflow_refuses_and_a_late_empty_group_is_not_a_spurious_prefix_overflow() {
    use TruingCurveRefusal as R;
    with_lineage(
        SeamSideId::A,
        TruingOutcomeIntent::Refit,
        FoldFixture::NoFold,
        |lineage| {
            let count = usize::try_from(usize::BITS).expect("platform bit count");
            let c = geometry(2000, Direction::Original);
            let overflowing = raw(1000, (0..count).map(|_| group(9, vec![c, c])).collect());
            let limits = TruingCurveLimits::new(2, count, 2 * count, usize::MAX, 2 * count);
            assert_eq!(
                overflowing
                    .admit(&lineage, limits)
                    .expect_err("complete declared product before logical cardinality")
                    .refusal(),
                R::CombinationOverflow
            );
            let zero = raw(
                1000,
                (0..count)
                    .map(|index| {
                        group(
                            9,
                            if index + 1 == count {
                                vec![]
                            } else {
                                vec![c, c]
                            },
                        )
                    })
                    .collect(),
            );
            assert_eq!(
                zero.admit(
                    &lineage,
                    TruingCurveLimits::new(2, count, 2 * count, 0, 2 * count)
                )
                .expect_err("full product zero;then actual cardinality failure")
                .refusal(),
                R::Count {
                    expected: 2,
                    actual: count
                }
            );
        },
    );
}
#[test]
fn explicit_empty_preservation_keeps_policy_and_original_lineage_and_cannot_declare_new_curve_groups(
) {
    with_outside(
        SeamSideId::A,
        Param::START,
        Param::END,
        TruingOutcomeIntent::PreserveSource,
        TruingOutsideRoles::new(TruingOutsideIntent::Absent, TruingOutsideIntent::Absent),
        |outside| {
            let directions = TruingDirectionManifest::new(id(1000), vec![]);
            let directions = directions
                .admit(&outside, TruingDirectionLimits::new(0, 0, 0))
                .expect("empty new outputs");
            let source = directions
                .admit_exact_piece_source(TruingPieceSourceLimits::new(0, 0, 1))
                .expect("actual original source");
            let contours = source
                .declare_contours(TruingContourLimits::new(0, 0, 1, 1, 1))
                .expect("original contour");
            let request = TruingFoldRequirement::new(TruingFoldIntent::NoFold, id(7001));
            let fold = contours
                .require_fold(&request, TruingFoldLimits::new(0, 1))
                .expect("no fold");
            let intent = TruingSourceIdentityIntent::RetainOriginal;
            let lineage = fold
                .require_lineage(&intent, TruingLineageLimits::new(0, 1))
                .expect("original identity request");
            let raw = raw(1000, vec![]);
            let admitted = raw
                .admit(&lineage, TruingCurveLimits::new(0, 0, 0, 0, 0))
                .expect("explicitly empty curve domain");
            assert!(std::ptr::eq(admitted.lineage(), &lineage));
            assert!(std::ptr::eq(admitted.policy(), raw.policy()));
            assert_eq!(
                (
                    admitted.groups().len(),
                    admitted.choice_count(),
                    admitted.combination_count(),
                    admitted.workspace_entries()
                ),
                (0, 0, 0, 0)
            );
            let extra = TruingCurveManifest::new(
                id(1000),
                TruingCurveSelectionPolicy::FirstFeasibleCompleteAssignment,
                vec![group(9, vec![geometry(2000, Direction::Original)])],
            );
            assert_eq!(
                extra
                    .admit(&lineage, TruingCurveLimits::new(0, 1, 1, 1, 2))
                    .expect_err("preservation cannot infer new selected outputs")
                    .refusal(),
                TruingCurveRefusal::Count {
                    expected: 0,
                    actual: 1
                }
            );
        },
    );
}
