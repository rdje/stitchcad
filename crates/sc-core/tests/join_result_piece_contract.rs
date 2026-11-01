//! Complete explicit result matching with real ordered holes,construction repetitions and fold intent.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_result.rs"]
mod topology;
use common::id;
use sc_core::{construction::*, ontology::*};
use topology::candidate as common;
type FieldCase = (fn(&mut PieceDefinition), JoinResultPieceField);

#[test]
fn complete_result_matches_actual_requested_holes_repeated_construction_fold_and_selected_metadata()
{
    topology::with_result(
        |_| {},
        |fields| {
            let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
            let repairs = raw
                .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
                .expect("explicit no companion exceptions");
            let originals = repairs
                .match_original_pieces()
                .expect("both old sources consumed");
            let matched = originals
                .match_result_piece()
                .expect("complete explicit candidate fields");
            assert!(std::ptr::eq(matched.originals(), &originals));
            assert!(std::ptr::eq(matched.plan(), fields.plan()));
            assert!(std::ptr::eq(matched.result(), fields.plan().joined_piece()));
            let d = matched.result().definition();
            assert_eq!(
                (
                    d.boundary.len(),
                    d.holes.len(),
                    d.construction_lines.len(),
                    d.fold_edges.len()
                ),
                (2, 2, 8, 1)
            );
            assert_eq!(
                (d.quantity, d.mirroring, d.cut_on_fold),
                (1, Mirroring::Single, true)
            );
            assert_eq!(
                d.construction_lines
                    .iter()
                    .map(|e| e.edge.tag())
                    .collect::<Vec<_>>(),
                [11, 5, 4, 12, 4, 11, 5, 12]
                    .into_iter()
                    .map(LocalTag::new)
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                (
                    originals.retained_piece_count(),
                    originals.consumed_piece_count()
                ),
                (0, 2)
            );
            assert_eq!((fields.hole_count(), fields.occurrence_count()), (4, 23));
            assert_eq!(
                matched.result().geometric_validation(),
                GeometricValidation::DeferredToG2
            );
            let folds = matched.plan().correspondence().sources().results().folds();
            assert_eq!(folds.source_count(), 2);
            assert_eq!(folds.assignments().filter(|action| matches!(action.intent(),JoinFoldSourceIntent::Omit{evidence_source} if *evidence_source==id(0))).count(),2);
            // Original omissions remain unresolved requests.
        },
    );
}

#[test]
fn actual_order_direction_fold_and_construction_repetitions_must_match_after_canonical_plan_and_resources(
) {
    let cases: [FieldCase; 5] = [
        (|d| d.boundary.reverse(), JoinResultPieceField::Boundary),
        (|d| d.holes.reverse(), JoinResultPieceField::Holes),
        (
            |d| d.construction_lines.swap(0, 1),
            JoinResultPieceField::Construction,
        ),
        (
            |d| {
                d.cut_on_fold = false;
                d.fold_edges.clear();
            },
            JoinResultPieceField::CutOnFold,
        ),
        (
            |d| d.fold_edges = vec![EdgeRef::new(id(1000), LocalTag::new(8))],
            JoinResultPieceField::FoldEdges,
        ),
    ];
    for (change, field) in cases {
        topology::with_result(change, |fields| {
            let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
            let repairs = raw
                .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
                .expect("explicit no companion exceptions");
            let originals = repairs
                .match_original_pieces()
                .expect("canonical exact copies/consumed originals do not certify result content");
            let error = originals
                .match_result_piece()
                .expect_err("actual requested order/direction/fold/content must match");
            assert_eq!(error.field(), field);
            assert!(std::ptr::eq(error.originals(), &originals));
            assert!(std::ptr::eq(
                error.candidate_piece(),
                fields.plan().joined_piece()
            ));
        });
    }
}

#[test]
fn independently_selected_material_face_layer_and_utf8_labels_remain_exact_without_normalization() {
    let cases: [FieldCase; 4] = [
        (
            |d| {
                d.cutting_side = match d.cutting_side {
                    CuttingSide::Face => CuttingSide::WrongSide,
                    CuttingSide::WrongSide => CuttingSide::Face,
                }
            },
            JoinResultPieceField::CuttingSide,
        ),
        (
            |d| d.material = MaterialAssignment::Unresolved("explicit alternate reason".into()),
            JoinResultPieceField::Material,
        ),
        (|d| d.layer_index += 1, JoinResultPieceField::Layer),
        (
            |d| d.label.colorway = "ééé".into(),
            JoinResultPieceField::Label(LabelField::Colorway),
        ),
    ];
    for (change, field) in cases {
        topology::with_result(change, |fields| {
            let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
            let repairs = raw
                .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
                .expect("explicit no exceptions");
            let originals = repairs
                .match_original_pieces()
                .expect("complete original preservation");
            let error = originals
                .match_result_piece()
                .expect_err("canonical nonblank selected fields are not interchangeable");
            assert_eq!(error.field(), field);
            assert!(std::ptr::eq(
                error.candidate_piece(),
                fields.plan().joined_piece()
            ));
        });
    }
}

#[test]
fn complete_inventory_match_retains_explicit_companion_and_evidence_requirements_without_accepting_them(
) {
    common::with_companion_fields(
        Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: id(102),
        },
        true,
        |fields| {
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
                .expect("explicit guarded retained mate requirement");
            let originals = repairs
                .match_original_pieces()
                .expect("original exact except admitted explicit companion field");
            let matched = originals
                .match_result_piece()
                .expect("actual requested reciprocal result fields");
            assert!(std::ptr::eq(matched.originals(), &originals));
            let repair = matched
                .originals()
                .original(id(102))
                .expect("actual retained scope")
                .companion_repair()
                .expect("still visible explicit requirement");
            assert_eq!(*repair.evidence_source(), id(0));
            assert_eq!(repair.retired_companion().id(), id(100));
            assert_eq!(
                matched.result().definition().mirroring,
                Mirroring::PairMember {
                    handedness: Handedness::Left,
                    companion: id(102)
                }
            );
            assert_eq!(raw, snapshot);
        },
    );
}
