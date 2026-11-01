//! Complete explicit source-relative directions on actual original join output candidates.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{id, with_cut_coverage, with_identity, BankFixture};
use sc_core::{construction::*, ontology::*};
fn field(tag: u32, direction: JoinParameterDirection) -> JoinDirectionAssignment {
    JoinDirectionAssignment::new(LocalTag::new(tag), direction)
}
#[test]
fn complete_reordered_assignments_lend_original_fields_and_authentic_sources_without_inferred_maps()
{
    use JoinParameterDirection::{ReverseSource, SameAsSource};
    with_cut_coverage(false, |coverage| {
        let raw = JoinDirectionManifest::new(
            id(1000),
            vec![field(2, ReverseSource), field(9, SameAsSource)],
        );
        let snapshot = raw.clone();
        let limits = JoinDirectionLimits::new(2, 2, 4);
        let directions = raw
            .admit(&coverage, limits)
            .expect("complete actual output assignments");
        assert!(std::ptr::eq(directions.manifest(), &raw));
        assert!(std::ptr::eq(directions.coverage(), &coverage));
        assert_eq!(directions.limits(), limits);
        assert_eq!(directions.workspace_entries(), 4);
        assert_eq!(limits.outputs(), 2);
        assert_eq!(limits.assignments(), 2);
        assert_eq!(limits.workspace_entries(), 4);
        assert_eq!(raw.operation(), id(1000));
        assert_eq!(
            directions
                .assignments()
                .map(|a| a.source().logical_output())
                .collect::<Vec<_>>(),
            [2, 9].map(|t| EdgeRef::new(id(1000), LocalTag::new(t)))
        );
        assert_eq!(
            directions
                .assignments()
                .rev()
                .map(|a| a.index())
                .collect::<Vec<_>>(),
            [1, 0]
        );
        for (index, output) in directions.assignments().enumerate() {
            let original = raw.assignments().get(index).expect("original field");
            assert!(std::ptr::eq(output.context(), &directions));
            assert!(std::ptr::eq(output.assignment(), original));
            assert_eq!(output.index(), index);
            assert_eq!(output.parameter_direction(), *original.direction());
            assert_eq!(output.source().candidate().tag(), *original.tag());
            assert!(std::ptr::eq(output.source().owner(), coverage.edges()));
            assert!(std::ptr::eq(
                directions
                    .assignment(*original.tag())
                    .expect("explicit tag")
                    .assignment(),
                original
            ));
        }
        assert!(directions.assignment(LocalTag::new(13)).is_none());
        assert_eq!(raw, snapshot);
        assert!(format!("{directions:?}").contains("AdmittedJoinDirections"));
    });
}
#[test]
fn complete_n_m_2m_admission_precedes_owner_count_and_malformed_assignment_fields() {
    use JoinDirectionRefusal as R;
    use JoinParameterDirection::SameAsSource;
    with_cut_coverage(false, |coverage| {
        let raw = JoinDirectionManifest::new(
            id(1001),
            vec![field(99, SameAsSource), field(99, SameAsSource)],
        );
        let snapshot = raw.clone();
        for (limits, cause) in [
            (
                JoinDirectionLimits::new(1, 0, 0),
                R::Outputs {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinDirectionLimits::new(2, 1, 0),
                R::Assignments {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinDirectionLimits::new(2, 2, 3),
                R::Workspace {
                    required: 4,
                    allowed: 3,
                },
            ),
            (JoinDirectionLimits::new(2, 2, 4), R::Operation),
        ] {
            let error = raw
                .admit(&coverage, limits)
                .expect_err("complete quotas/owner first");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.coverage(), &coverage));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error.refused_assignment().is_none());
            assert!(error
                .to_string()
                .starts_with("join output parameter directions:"));
            assert!(format!("{error:?}").contains("JoinDirectionError"));
        }
        assert_eq!(raw, snapshot);
        for assignments in [
            vec![field(99, SameAsSource)],
            vec![field(99, SameAsSource); 3],
        ] {
            let m = assignments.len();
            let raw = JoinDirectionManifest::new(id(1000), assignments);
            let error = raw
                .admit(&coverage, JoinDirectionLimits::new(2, m, 2 * m))
                .expect_err("count before fields");
            assert_eq!(
                error.refusal(),
                R::Count {
                    expected: 2,
                    actual: m
                }
            );
            assert!(error.refused_assignment().is_none());
        }
    });
}
#[test]
fn equal_count_cannot_hide_duplicate_or_unknown_outputs_including_active_but_absent_roles() {
    use JoinDirectionRefusal as R;
    use JoinParameterDirection::{ReverseSource, SameAsSource};
    with_cut_coverage(false, |coverage| {
        assert!(coverage
            .edges()
            .identity()
            .roles()
            .catalog()
            .active_role(LocalTag::new(13))
            .is_some());
        for (second, cause) in [
            (
                9,
                R::Duplicate {
                    first: 0,
                    second: 1,
                    tag: LocalTag::new(9),
                },
            ),
            (
                13,
                R::UnknownOutput {
                    index: 1,
                    tag: LocalTag::new(13),
                },
            ),
            (
                7,
                R::UnknownOutput {
                    index: 1,
                    tag: LocalTag::new(7),
                },
            ),
            (
                99,
                R::UnknownOutput {
                    index: 1,
                    tag: LocalTag::new(99),
                },
            ),
        ] {
            let raw = JoinDirectionManifest::new(
                id(1000),
                vec![field(9, SameAsSource), field(second, ReverseSource)],
            );
            let snapshot = raw.clone();
            let error = raw
                .admit(&coverage, JoinDirectionLimits::new(2, 2, 4))
                .expect_err("first original membership cause");
            assert_eq!(error.refusal(), cause);
            assert_eq!(raw, snapshot);
            assert!(std::ptr::eq(
                error.refused_assignment().expect("actual refused field"),
                raw.assignments().get(1).expect("original second field")
            ));
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.coverage(), &coverage));
        }
    });
}
#[test]
fn complete_noncut_outputs_need_explicit_directions_and_empty_metadata_proves_no_result() {
    use JoinParameterDirection::{ReverseSource, SameAsSource};
    with_cut_coverage(true, |coverage| {
        assert_eq!(coverage.cut_edge_count(), 4);
        assert_eq!(coverage.edges().edge_count(), 10);
        let raw = JoinDirectionManifest::new(
            id(1000),
            coverage
                .edges()
                .edges()
                .rev()
                .enumerate()
                .map(|(i, e)| {
                    JoinDirectionAssignment::new(
                        e.candidate().tag(),
                        if i % 2 == 0 {
                            SameAsSource
                        } else {
                            ReverseSource
                        },
                    )
                })
                .collect(),
        );
        let error = raw
            .admit(&coverage, JoinDirectionLimits::new(4, 10, 20))
            .expect_err("whole output count includes noncut fields");
        assert_eq!(
            error.refusal(),
            JoinDirectionRefusal::Outputs {
                required: 10,
                allowed: 4
            }
        );
        let directions = raw
            .admit(&coverage, JoinDirectionLimits::new(10, 10, 20))
            .expect("all actual candidates explicitly directed");
        assert_eq!(directions.assignments().len(), 10);
        for output in directions.assignments() {
            assert_eq!(
                output.parameter_direction(),
                *output.assignment().direction()
            );
            let original = coverage
                .edges()
                .edge(*output.assignment().tag())
                .expect("actual same output");
            assert!(std::ptr::eq(
                output.source().candidate(),
                original.candidate()
            ));
            assert!(std::ptr::eq(
                output.source().occurrence().field().edge(),
                original.occurrence().field().edge()
            ));
        }
        let raw = JoinDirectionManifest::new(
            id(1000),
            coverage
                .cut_edges()
                .map(|e| JoinDirectionAssignment::new(e.candidate().tag(), SameAsSource))
                .collect(),
        );
        assert_eq!(
            raw.admit(&coverage, JoinDirectionLimits::new(10, 4, 8))
                .expect_err("cut-only assignments are incomplete")
                .refusal(),
            JoinDirectionRefusal::Count {
                expected: 10,
                actual: 4
            }
        );
    });
    with_identity(BankFixture::basic(), |identity| {
        let raw = JoinEdgeManifest::new(id(1000), vec![]);
        let edges = raw
            .admit(&identity, JoinEdgeManifestLimits::new(0, 0))
            .expect("empty presence metadata");
        let coverage = edges
            .require_cut_coverage(JoinCutCoverageLimits::new(0, 2, 0))
            .expect("no whole-bank outside intervals");
        let raw = JoinDirectionManifest::new(id(1000), vec![]);
        let directions = raw
            .admit(&coverage, JoinDirectionLimits::new(0, 0, 0))
            .expect("explicit empty direction metadata");
        assert_eq!(directions.assignments().len(), 0);
        assert_eq!(directions.workspace_entries(), 0);
        assert_eq!(
            *directions
                .coverage()
                .edges()
                .identity()
                .roles()
                .banks()
                .sources()
                .population()
                .quantity(),
            1
        );
    });
}
