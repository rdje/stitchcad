//! Original preservation requires exact fields,not merely canonical candidate membership/resources.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_candidate.rs"]
mod common;
use common::{
    baseline_inventory, baseline_plan, id, ledger, with_bound_plan, with_companion_fields,
};
use sc_core::{construction::*, ontology::*};

#[test]
fn whole_original_scope_lends_actual_consumed_and_unchanged_members_in_original_order() {
    with_bound_plan(|plan| {
        let fields = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 0, 24, 114))
            .expect("whole original/candidate resources");
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
        let repairs = raw
            .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
            .expect("explicit no companion exceptions");
        let matched = repairs
            .match_original_pieces()
            .expect("retained unrelated original exact;two source members consumed");
        assert!(std::ptr::eq(matched.repairs(), &repairs));
        assert_eq!(
            (
                matched.original_piece_count(),
                matched.retained_piece_count(),
                matched.consumed_piece_count()
            ),
            (3, 1, 2)
        );
        assert_eq!(
            matched
                .originals()
                .map(|p| p.original().id())
                .collect::<Vec<_>>(),
            [id(100), id(101), id(102)]
        );
        assert_eq!(
            matched
                .originals()
                .rev()
                .map(|p| p.original().id())
                .collect::<Vec<_>>(),
            [id(102), id(101), id(100)]
        );
        for scope in matched.originals() {
            assert!(std::ptr::eq(scope.context(), &matched));
            let original = plan
                .correspondence()
                .sources()
                .results()
                .source_piece(scope.original().id())
                .expect("actual original");
            assert!(std::ptr::eq(scope.original(), original));
            assert!(scope.companion_repair().is_none());
            if scope.original().id() == id(102) {
                let candidate = scope.candidate().expect("actual retained original");
                assert!(std::ptr::eq(
                    candidate,
                    plan.piece(id(102)).expect("same supplied candidate")
                ));
                assert_eq!(scope.retained_quantity(), 1);
                assert_eq!(candidate.definition(), original.definition());
            } else {
                assert!(scope.candidate().is_none());
                assert_eq!(scope.retained_quantity(), 0);
            }
        }
        assert!(std::ptr::eq(
            matched
                .original(id(102))
                .expect("actual indexed original")
                .original(),
            plan.original_inventory()
                .piece(id(102))
                .expect("same original")
        ));
        assert!(matched.original(id(5000)).is_none());
        assert!(matched.original(id(999)).is_none());
    });
}

#[test]
fn retained_source_quantity_is_derived_from_actual_copy_actions_without_changing_any_other_field() {
    common::pairing::with_partial_correspondence(|pairs| {
        let baseline = common::inventory(&pairs, 1, Mirroring::Single, 102, false);
        let ledger = ledger();
        let mut pieces = baseline.pieces().to_vec();
        for bits in [101, 100] {
            let mut d = pairs
                .sources()
                .results()
                .source_piece(id(bits))
                .expect("actual source with two authored copies")
                .definition()
                .clone();
            assert_eq!(d.quantity, 2);
            d.quantity = 1;
            pieces.push(Piece::new(d, &ledger).expect("one original authored copy remains"));
        }
        let inventory = PieceInventory::new(pieces).expect("actual full candidate inventory");
        let results = pairs.sources().results();
        let definitions = [
            *results
                .source_copy(id(204))
                .expect("retained B")
                .definition(),
            *results
                .copy(id(6000))
                .expect("explicit result")
                .definition(),
            *results
                .source_copy(id(202))
                .expect("retained unrelated")
                .definition(),
            *results
                .source_copy(id(203))
                .expect("retained A")
                .definition(),
        ];
        let plan = CutPlan::new(definitions.to_vec(), inventory.pieces())
            .expect("canonical exact retained/result population");
        let plan = pairs
            .bind_candidate_plan(
                &inventory,
                &plan,
                JoinCandidatePlanLimits::new(5, 3, 1, 4, 4, 16),
            )
            .expect("complete original actions bind actual candidate copies");
        let fields = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 4, 0, 34, 1000))
            .expect("whole resources before any field equality");
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
        let repairs = raw
            .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
            .expect("no companion exceptions");
        let matched = repairs
            .match_original_pieces()
            .expect("derived retained source quantity is the sole field difference");
        assert_eq!(
            (
                matched.retained_piece_count(),
                matched.consumed_piece_count()
            ),
            (3, 0)
        );
        assert_eq!(fields.occurrence_count(), 34);
        for bits in [100, 101] {
            let scope = matched
                .original(id(bits))
                .expect("actual partially consumed source");
            assert_eq!(plan.original_retained_count(id(bits)), Some(1));
            assert_eq!(scope.original().definition().quantity, 2);
            assert_eq!(scope.retained_quantity(), 1);
            let mut expected = scope.original().definition().clone();
            expected.quantity = 1;
            assert_eq!(
                &expected,
                scope
                    .candidate()
                    .expect("retained source is present")
                    .definition()
            );
        }
        assert_eq!(plan.copy_count(), 4);
        assert_eq!(plan.joined_piece().id(), id(5000)); // Joined-result matching remains separate.
    });
}

#[derive(Clone, Copy)]
enum Change {
    Boundary,
    Holes,
    Material,
    Label,
}
fn with_change(change: Change, run: impl FnOnce(BoundJoinInventoryFields<'_>)) {
    common::pairing::with_correspondence(|pairs| {
        let baseline = baseline_inventory(&pairs);
        let ledger = ledger();
        let pieces = baseline
            .pieces()
            .iter()
            .map(|p| {
                let mut d = p.definition().clone();
                if d.id == id(102) {
                    match change {
                        Change::Boundary => {
                            d.boundary.swap(0, 1);
                            d.label.name = "also changed".into();
                        }
                        Change::Holes => {
                            d.holes.push(vec![DirectedEdge {
                                edge: EdgeRef::new(id(2), LocalTag::new(1)),
                                direction: Direction::Original,
                            }]);
                            d.label.name = "also changed".into();
                        }
                        Change::Material => {
                            d.material =
                                MaterialAssignment::Unresolved("explicit different reason".into())
                        }
                        Change::Label => d.label.fabric = "ééé".into(),
                    }
                }
                Piece::new(d, &ledger).expect("structurally born actual changed candidate")
            })
            .collect();
        let inventory = PieceInventory::new(pieces)
            .expect("canonical candidate membership does not certify original fields");
        let plan = baseline_plan(&pairs, &inventory);
        let plan = pairs
            .bind_candidate_plan(
                &inventory,
                &plan,
                JoinCandidatePlanLimits::new(5, 3, 2, 2, 3, 11),
            )
            .expect("exact copy plan admits changed body fields");
        let fields = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 1, 25, 1000))
            .expect("whole changed fields admitted before comparison");
        run(fields);
    });
}
#[test]
fn canonical_plan_and_resource_presence_cannot_authorize_changed_original_geometry_metadata_or_utf8(
) {
    for (change, field) in [
        (Change::Boundary, JoinOriginalPieceField::Boundary),
        (Change::Holes, JoinOriginalPieceField::Holes),
        (Change::Material, JoinOriginalPieceField::Material),
        (
            Change::Label,
            JoinOriginalPieceField::Label(LabelField::Fabric),
        ),
    ] {
        with_change(change, |fields| {
            let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
            let repairs = raw
                .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
                .expect("no exceptions requested");
            let error = repairs.match_original_pieces().expect_err(
                "actual original fields must match despite canonical population/resources",
            );
            assert_eq!(
                error.refusal(),
                JoinOriginalPieceRefusal::Field {
                    piece: id(102),
                    field
                }
            );
            assert!(std::ptr::eq(error.repairs(), &repairs));
            let original = fields
                .plan()
                .correspondence()
                .sources()
                .results()
                .source_piece(id(102))
                .expect("actual original");
            let candidate = fields
                .plan()
                .piece(id(102))
                .expect("actual changed candidate");
            assert!(std::ptr::eq(
                error.original_piece().expect("actual diagnostic source"),
                original
            ));
            assert!(std::ptr::eq(
                error
                    .candidate_piece()
                    .expect("actual diagnostic candidate"),
                candidate
            ));
            assert_ne!(original.definition(), candidate.definition());
            if matches!(change, Change::Label) {
                assert_eq!(candidate.definition().label.fabric.len(), 6);
            }
        });
    }
}

#[test]
fn only_the_explicit_guarded_companion_requirement_covers_a_changed_original_mirroring_field() {
    let mode = Mirroring::PairMember {
        handedness: Handedness::Left,
        companion: id(102),
    };
    with_companion_fields(mode, true, |fields| {
        let empty = JoinCompanionRepairManifest::new(id(1000), vec![]);
        let repairs = empty
            .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
            .expect("empty explicit exceptions");
        let error = repairs
            .match_original_pieces()
            .expect_err("candidate reciprocal structure cannot silently reassign a companion");
        assert_eq!(
            error.refusal(),
            JoinOriginalPieceRefusal::Field {
                piece: id(102),
                field: JoinOriginalPieceField::Mirroring
            }
        );
        let request = JoinCompanionRepairRequest::new(
            id(102),
            Mirroring::PairMember {
                handedness: Handedness::Right,
                companion: id(100),
            },
            id(5000),
            id(0),
        );
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![request]);
        let snapshot = raw.clone();
        let repairs = raw
            .admit(&fields, JoinCompanionRepairLimits::new(1, 2))
            .expect("same actual original/candidate guard and explicit new companion");
        let matched = repairs
            .match_original_pieces()
            .expect("all other original fields preserved exactly");
        let scope = matched
            .original(id(102))
            .expect("actual retained separate member");
        let repair = scope
            .companion_repair()
            .expect("actual outstanding explicit repair request");
        assert!(std::ptr::eq(repair.context(), &repairs));
        assert!(std::ptr::eq(repair.original(), scope.original()));
        assert!(std::ptr::eq(
            repair.candidate(),
            scope.candidate().expect("same actual candidate")
        ));
        assert_eq!(*repair.evidence_source(), id(0)); // Remains an unresolved evidence request.
        let mut expected = scope.original().definition().clone();
        expected.mirroring = *repair.candidate_mirroring();
        assert_eq!(
            &expected,
            scope
                .candidate()
                .expect("actual retained member")
                .definition()
        );
        assert_eq!(raw, snapshot);
        assert_eq!(
            (
                matched.retained_piece_count(),
                matched.consumed_piece_count()
            ),
            (1, 2)
        );
    });
}
