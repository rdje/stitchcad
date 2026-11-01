//! Exact complete source-relative orientation requests;no contour or geometric-map certificate.
#![allow(clippy::expect_used)]
#[path = "support/truing_outside.rs"]
mod common;
use common::{
    domain::{q, roles::copies::id},
    with_outside,
};
use sc_core::{construction::*, ontology::*};
fn outside() -> TruingOutsideRoles {
    TruingOutsideRoles::new(
        TruingOutsideIntent::Role(LocalTag::new(2)),
        TruingOutsideIntent::Role(LocalTag::new(5)),
    )
}
fn raw(operation: u128, items: &[(u32, TruingParameterDirection)]) -> TruingDirectionManifest {
    TruingDirectionManifest::new(
        id(operation),
        items
            .iter()
            .map(|(tag, direction)| TruingDirectionAssignment::new(LocalTag::new(*tag), *direction))
            .collect(),
    )
}
#[test]
fn arbitrary_original_order_and_mixed_explicit_directions_bind_every_actual_selected_and_outside_source(
) {
    use TruingParameterDirection as D;
    for side in [SeamSideId::A, SeamSideId::B] {
        with_outside(
            side,
            q(1, 4),
            q(3, 4),
            TruingOutcomeIntent::Refit,
            outside(),
            |outside| {
                let raw = raw(
                    1000,
                    &[
                        (5, D::ReverseSource),
                        (9, D::SameAsSource),
                        (2, D::ReverseSource),
                    ],
                );
                let snapshot = raw.clone();
                let limits = TruingDirectionLimits::new(3, 3, 6);
                let admitted = raw
                    .admit(&outside, limits)
                    .expect("complete exact actual output set");
                assert!(std::ptr::eq(admitted.manifest(), &raw));
                assert!(std::ptr::eq(admitted.outside(), &outside));
                assert_eq!(admitted.limits(), limits);
                assert_eq!(
                    (
                        limits.outputs(),
                        limits.assignments(),
                        limits.workspace_entries()
                    ),
                    (3, 3, 6)
                );
                assert_eq!(admitted.workspace_entries(), 6);
                assert_eq!(raw.operation(), id(1000));
                assert_eq!(admitted.assignments().len(), 3);
                for (member, index) in admitted.assignments().zip(0..) {
                    let original = raw.assignments().get(index).expect("original assignment");
                    assert!(std::ptr::eq(member.context(), &admitted));
                    assert_eq!(member.index(), index);
                    assert!(std::ptr::eq(member.assignment(), original));
                    assert_eq!(member.parameter_direction(), *original.direction());
                    assert_eq!(
                        member.source().logical_output(),
                        EdgeRef::new(id(1000), *original.tag())
                    );
                    match member.source() {
                        TruingDirectionSource::Selected(source) => {
                            assert_eq!(*original.tag(), LocalTag::new(9));
                            assert!(std::ptr::eq(
                                source.context(),
                                outside.evidence().locks().outcome().source_domain()
                            ));
                            assert!(std::ptr::eq(member.source().range(), source.source()));
                        }
                        TruingDirectionSource::Outside(source) => {
                            assert_ne!(*original.tag(), LocalTag::new(9));
                            assert!(std::ptr::eq(source.context(), &outside));
                            assert_eq!(*source.role().tag(), *original.tag());
                            assert!(std::ptr::eq(member.source().range(), source.source()));
                        }
                    }
                    assert_eq!(
                        admitted
                            .assignment(*original.tag())
                            .expect("actual tag query")
                            .index(),
                        index
                    );
                }
                assert_eq!(
                    admitted
                        .assignments()
                        .rev()
                        .map(|a| *a.assignment().tag())
                        .collect::<Vec<_>>(),
                    vec![LocalTag::new(2), LocalTag::new(9), LocalTag::new(5)]
                );
                assert!(admitted.assignment(LocalTag::new(7)).is_none());
                assert!(admitted.assignment(LocalTag::new(99)).is_none());
                assert_eq!(raw, snapshot);
                assert!(format!("{admitted:?}").contains("AdmittedTruingDirections"));
            },
        );
    }
}
#[test]
fn whole_resources_then_operation_cardinality_and_original_membership_refuse_without_prefix_or_default(
) {
    use TruingDirectionRefusal as R;
    use TruingParameterDirection as D;
    with_outside(
        SeamSideId::B,
        q(1, 4),
        q(3, 4),
        TruingOutcomeIntent::Refit,
        outside(),
        |outside| {
            let wrong = raw(
                999,
                &[
                    (9, D::SameAsSource),
                    (9, D::ReverseSource),
                    (99, D::SameAsSource),
                ],
            );
            for (limits, cause) in [
                (
                    TruingDirectionLimits::new(2, 2, 5),
                    R::Outputs {
                        required: 3,
                        allowed: 2,
                    },
                ),
                (
                    TruingDirectionLimits::new(3, 2, 5),
                    R::Assignments {
                        required: 3,
                        allowed: 2,
                    },
                ),
                (
                    TruingDirectionLimits::new(3, 3, 5),
                    R::Workspace {
                        required: 6,
                        allowed: 5,
                    },
                ),
                (TruingDirectionLimits::new(3, 3, 6), R::Operation),
            ] {
                let error = wrong
                    .admit(&outside, limits)
                    .expect_err("whole resource then owner cause");
                assert!(std::ptr::eq(error.manifest(), &wrong));
                assert!(std::ptr::eq(error.outside(), &outside));
                assert_eq!(error.limits(), limits);
                assert_eq!(error.refusal(), cause);
                assert!(error
                    .to_string()
                    .starts_with("truing output parameter directions:"));
                assert!(format!("{error:?}").contains("TruingDirectionError"));
            }
            for (items, cause) in [
                (
                    vec![(9, D::SameAsSource), (2, D::SameAsSource)],
                    R::Count {
                        expected: 3,
                        actual: 2,
                    },
                ),
                (
                    vec![
                        (9, D::SameAsSource),
                        (2, D::SameAsSource),
                        (2, D::ReverseSource),
                    ],
                    R::Duplicate {
                        first: 1,
                        second: 2,
                        tag: LocalTag::new(2),
                    },
                ),
                (
                    vec![
                        (9, D::SameAsSource),
                        (2, D::SameAsSource),
                        (99, D::ReverseSource),
                    ],
                    R::UnknownOutput {
                        index: 2,
                        tag: LocalTag::new(99),
                    },
                ),
            ] {
                let raw = raw(1000, &items);
                assert_eq!(
                    raw.admit(&outside, TruingDirectionLimits::new(3, 3, 6))
                        .expect_err("exact original coverage cause")
                        .refusal(),
                    cause
                );
            }
        },
    );
}
#[test]
fn a_catalogue_active_role_is_not_an_output_and_empty_preservation_never_creates_direction_defaults(
) {
    use TruingParameterDirection as D;
    let absent = TruingOutsideRoles::new(TruingOutsideIntent::Absent, TruingOutsideIntent::Absent);
    with_outside(
        SeamSideId::A,
        Param::START,
        Param::END,
        TruingOutcomeIntent::Refit,
        absent,
        |outside| {
            assert!(outside
                .evidence()
                .locks()
                .outcome()
                .source_domain()
                .candidates()
                .roles()
                .catalog()
                .is_active(LocalTag::new(2)));
            let wrong = raw(1000, &[(2, D::SameAsSource)]);
            assert_eq!(
                wrong
                    .admit(&outside, TruingDirectionLimits::new(1, 1, 2))
                    .expect_err("active but not an actual output")
                    .refusal(),
                TruingDirectionRefusal::UnknownOutput {
                    index: 0,
                    tag: LocalTag::new(2)
                }
            );
            let raw = raw(1000, &[(9, D::ReverseSource)]);
            let bound = raw
                .admit(&outside, TruingDirectionLimits::new(1, 1, 2))
                .expect("explicit single source direction");
            assert_eq!(
                bound
                    .assignment(LocalTag::new(9))
                    .expect("actual only output")
                    .parameter_direction(),
                D::ReverseSource
            );
        },
    );
    with_outside(
        SeamSideId::B,
        q(1, 4),
        q(3, 4),
        TruingOutcomeIntent::PreserveSource,
        absent,
        |outside| {
            let raw = raw(1000, &[]);
            let bound = raw
                .admit(&outside, TruingDirectionLimits::new(0, 0, 0))
                .expect("explicit complete empty new-output set");
            assert_eq!(bound.workspace_entries(), 0);
            assert_eq!(bound.assignments().len(), 0);
            assert!(bound.assignment(LocalTag::new(9)).is_none());
            assert!(outside
                .evidence()
                .locks()
                .outcome()
                .source_domain()
                .candidates()
                .roles()
                .catalog()
                .is_active(LocalTag::new(9)));
        },
    );
}
