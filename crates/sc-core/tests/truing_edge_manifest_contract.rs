//! Original role/source-range candidates only;coverage,curve choice and fitting remain mandatory.
#![allow(clippy::expect_used)]
#[path = "support/truing_roles.rs"]
mod common;
use common::{copies::id, with_roles};
use sc_core::{construction::*, ontology::*};
fn range(creator: u128) -> EdgeRange {
    EdgeRange::whole(EdgeRef::new(id(creator), LocalTag::FIRST))
}
fn candidate(tag: u32, source: EdgeRange) -> TruingEdgeCandidate {
    TruingEdgeCandidate::new(LocalTag::new(tag), source)
}
#[test]
fn ordered_distinct_roles_preserve_equal_partial_ranges_actual_parents_and_original_field_borrows()
{
    with_roles(false, |roles| {
        let original = roles.copies().source().source_side().range;
        let q = |n| Param::new(Rational::new(n, 4).expect("quarter")).expect("unit domain");
        let partial = EdgeRange::new(original.edge(), q(1), q(3)).expect("positive original range");
        let raw =
            TruingEdgeManifest::new(id(1000), vec![candidate(9, partial), candidate(2, partial)]);
        let snapshot = raw.clone();
        let limits = TruingEdgeManifestLimits::new(2, 4);
        let admitted = raw.admit(&roles, limits).expect("local candidate metadata");
        assert!(std::ptr::eq(admitted.manifest(), &raw));
        assert!(std::ptr::eq(admitted.roles(), &roles));
        assert_eq!(admitted.limits(), limits);
        assert_eq!(
            (
                limits.candidates(),
                limits.workspace_entries(),
                admitted.workspace_entries(),
                admitted.edge_count()
            ),
            (2, 4, 4, 2)
        );
        assert_eq!(raw.operation(), id(1000));
        for (edge, index) in admitted.edges().zip(0..) {
            assert!(std::ptr::eq(edge.owner(), &admitted));
            assert_eq!(edge.index(), index);
            assert!(std::ptr::eq(
                edge.candidate(),
                raw.candidates().get(index).expect("original candidate")
            ));
            assert!(std::ptr::eq(edge.source(), edge.candidate().source()));
            assert_eq!(*edge.source(), partial);
            assert!(std::ptr::eq(edge.role().catalog(), roles.catalog()));
            assert_eq!(*edge.role().tag(), edge.candidate().tag());
            assert_eq!(
                edge.logical_output(),
                EdgeRef::new(id(1000), edge.candidate().tag())
            );
        }
        assert_eq!(
            admitted
                .edges()
                .rev()
                .map(|e| e.candidate().tag())
                .collect::<Vec<_>>(),
            vec![LocalTag::new(2), LocalTag::new(9)]
        );
        assert_eq!(
            admitted
                .edge(LocalTag::new(9))
                .expect("actual role")
                .index(),
            0
        );
        assert!(admitted.edge(LocalTag::new(7)).is_none());
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn complete_n_and_two_n_quotas_precede_wrong_operation_and_invalid_original_fields() {
    use TruingEdgeManifestRefusal as R;
    with_roles(false, |roles| {
        let raw = TruingEdgeManifest::new(
            id(999),
            vec![candidate(7, range(1000)), candidate(7, range(1000))],
        );
        let snapshot = raw.clone();
        for (limits, cause) in [
            (
                TruingEdgeManifestLimits::new(1, 0),
                R::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                TruingEdgeManifestLimits::new(2, 3),
                R::Workspace {
                    required: 4,
                    allowed: 3,
                },
            ),
            (TruingEdgeManifestLimits::new(2, 4), R::Operation),
        ] {
            let error = raw
                .admit(&roles, limits)
                .expect_err("whole resource then owner");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.roles(), &roles));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error
                .to_string()
                .starts_with("truing edge candidate manifest:"));
            assert!(format!("{error:?}").contains("TruingEdgeManifestError"));
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn original_duplicate_undefined_inactive_self_and_actual_piece_material_creator_aliases_have_exact_causes(
) {
    use TruingEdgeManifestRefusal as R;
    with_roles(true, |roles| {
        let actual = roles.copies().source().source_side().range;
        let mut rows = vec![
            (
                vec![candidate(9, actual), candidate(9, range(1000))],
                R::DuplicateTag {
                    first: 0,
                    second: 1,
                    tag: LocalTag::new(9),
                },
            ),
            (
                vec![candidate(99, range(1000))],
                R::UndefinedRole {
                    index: 0,
                    tag: LocalTag::new(99),
                },
            ),
            (
                vec![candidate(7, range(1000))],
                R::InactiveRole {
                    index: 0,
                    tag: LocalTag::new(7),
                },
            ),
            (
                vec![candidate(9, range(1000))],
                R::SelfSource {
                    index: 0,
                    source: range(1000).edge(),
                },
            ),
        ];
        for piece in [100, 101] {
            rows.push((
                vec![candidate(9, range(piece))],
                R::PieceSource {
                    index: 0,
                    source: range(piece).edge(),
                    piece: id(piece),
                },
            ));
        }
        for material in [777, 888] {
            rows.push((
                vec![candidate(9, range(material))],
                R::MaterialSource {
                    index: 0,
                    source: range(material).edge(),
                    material: id(material),
                },
            ));
        }
        for (entries, cause) in rows {
            let raw = TruingEdgeManifest::new(id(1000), entries);
            let snapshot = raw.clone();
            assert_eq!(
                raw.admit(&roles, TruingEdgeManifestLimits::new(2, 4))
                    .expect_err("first original cause")
                    .refusal(),
                cause
            );
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn empty_and_unowned_range_presence_remain_explicit_metadata_not_coverage_or_no_op_certificates() {
    with_roles(false, |roles| {
        let empty = TruingEdgeManifest::new(id(1000), vec![]);
        let admitted = empty
            .admit(&roles, TruingEdgeManifestLimits::new(0, 0))
            .expect("explicit empty candidate metadata");
        assert_eq!(admitted.edge_count(), 0);
        assert_eq!(admitted.workspace_entries(), 0);
        assert_eq!(admitted.edges().len(), 0);
        assert!(admitted.edge(LocalTag::new(9)).is_none());
        assert!(admitted
            .roles()
            .catalog()
            .active_role(LocalTag::new(9))
            .is_some());
        assert_eq!(
            admitted
                .roles()
                .copies()
                .source()
                .span()
                .definition()
                .stops
                .len(),
            1
        );
        let raw = TruingEdgeManifest::new(id(1000), vec![candidate(2, range(555))]);
        let admitted = raw
            .admit(&roles, TruingEdgeManifestLimits::new(1, 2))
            .expect("attribution does not certify source existence or selected domain");
        assert_eq!(
            *admitted
                .edge(LocalTag::new(2))
                .expect("original field")
                .source(),
            range(555)
        );
        assert_eq!(admitted.roles().copies().plan().copies().len(), 2);
    });
}
