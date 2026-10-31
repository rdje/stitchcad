//! Exact bounded candidate source/result fields and actual pair-consistent inventory membership.
#![allow(clippy::expect_used)]
#[path = "support/mirror_copy_inventory.rs"]
mod common;
use common::copies::{with_cut, with_ids};
use common::inventory;
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn proposal() -> MirrorPieceResultCopyManifest {
    MirrorPieceResultCopyManifest::new(
        id(10),
        vec![MirrorPieceResultCopyIntent::Issue(CutCopyDefinition {
            id: id(500),
            piece: id(200),
            orientation: CopyOrientation::Authored,
        })],
    )
}
fn keep(_: &mut PieceDefinition) {}

#[test]
fn actual_candidate_members_keep_complete_fields_order_and_inspected_counts_for_both_fold_intents()
{
    for fold in [
        MirrorPieceFoldIntent::NoFold,
        MirrorPieceFoldIntent::ReflectedSource,
    ] {
        with_cut(1, Mirroring::Single, fold, false, |ids| {
            let raw = proposal();
            let copies = raw
                .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
                .expect("explicit native proposal");
            let inventory = inventory(&copies, keep, keep);
            let original = inventory.clone();
            let r = if fold == MirrorPieceFoldIntent::NoFold {
                30
            } else {
                32
            };
            let limits = MirrorPieceCopyInventoryLimits::new(3, 8, r, 220);
            let bound = copies
                .bind_piece_inventory(&inventory, &limits)
                .expect("exact actual candidate fields");
            assert!(std::ptr::eq(bound.copies(), &copies));
            assert!(std::ptr::eq(bound.inventory(), &inventory));
            assert!(std::ptr::eq(bound.limits(), &limits));
            assert_eq!(
                (
                    limits.pieces(),
                    limits.holes(),
                    limits.edges(),
                    limits.text_bytes()
                ),
                (3, 8, r, 220)
            );
            assert_eq!(
                (bound.holes(), bound.edges(), bound.text_bytes()),
                (8, r, 220)
            );
            assert!(std::ptr::eq(
                bound.source_piece(),
                inventory.piece(id(100)).expect("actual source")
            ));
            assert!(std::ptr::eq(
                bound.result_piece(),
                inventory.piece(id(200)).expect("actual result")
            ));
            assert_eq!(
                bound
                    .inventory()
                    .pieces()
                    .iter()
                    .map(Piece::id)
                    .collect::<Vec<_>>(),
                vec![id(200), id(300), id(100)]
            );
            assert!(bound.source_companion().is_none());
            assert!(bound.result_companion().is_none());
            assert_eq!(inventory, original);
        });
    }
}

#[test]
fn whole_inventory_and_four_form_resources_precede_member_and_content_checks() {
    use MirrorPieceCopyInventoryRefusal as R;
    with_ids(1, Mirroring::Single, false, |ids| {
        let raw = proposal();
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
            .expect("proposal");
        let changed = inventory(
            &copies,
            |source| {
                source.holes.push(vec![DirectedEdge {
                    edge: EdgeRef::new(id(70), LocalTag::FIRST),
                    direction: Direction::Original,
                }])
            },
            keep,
        );
        for (limits, refusal) in [
            (
                MirrorPieceCopyInventoryLimits::new(2, 0, 0, 0),
                R::Pieces {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                MirrorPieceCopyInventoryLimits::new(3, 8, 0, 0),
                R::Holes {
                    required: 9,
                    allowed: 8,
                },
            ),
            (
                MirrorPieceCopyInventoryLimits::new(3, 9, 30, 0),
                R::Edges {
                    required: 31,
                    allowed: 30,
                },
            ),
            (
                MirrorPieceCopyInventoryLimits::new(3, 9, 31, 219),
                R::TextBytes {
                    required: 220,
                    allowed: 219,
                },
            ),
            (
                MirrorPieceCopyInventoryLimits::new(3, 9, 31, 220),
                R::SourceChanged,
            ),
        ] {
            let error = copies
                .bind_piece_inventory(&changed, &limits)
                .expect_err("whole refusal without partial association");
            assert!(std::ptr::eq(error.copies(), &copies));
            assert!(std::ptr::eq(error.inventory(), &changed));
            assert!(std::ptr::eq(error.limits(), &limits));
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror candidate Piece inventory"));
        }
        let empty = PieceInventory::new(vec![]).expect("empty metadata");
        assert_eq!(
            copies
                .bind_piece_inventory(&empty, &MirrorPieceCopyInventoryLimits::new(0, 0, 0, 0))
                .expect_err("source absent")
                .refusal(),
            R::MissingSource { id: id(100) }
        );
        let source = ids
            .source()
            .context()
            .contours()
            .coverage()
            .edges()
            .identity()
            .roles()
            .metadata()
            .cut()
            .source()
            .piece()
            .clone();
        let source_only = PieceInventory::new(vec![source]).expect("source alone");
        assert_eq!(
            copies
                .bind_piece_inventory(
                    &source_only,
                    &MirrorPieceCopyInventoryLimits::new(1, 0, 0, 0)
                )
                .expect_err("result absent")
                .refusal(),
            R::MissingResult { id: id(200) }
        );
        let text = inventory(&copies, |source| source.label.name.push_str(" extra"), keep);
        assert_eq!(
            copies
                .bind_piece_inventory(&text, &MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220))
                .expect_err("candidate source text also counted before comparison")
                .refusal(),
            R::TextBytes {
                required: 226,
                allowed: 220
            }
        );
    });
}

#[test]
fn every_actual_result_field_matches_native_canonical_metadata_and_contours() {
    use MirrorPieceCopyInventoryField as F;
    use MirrorPieceCopyInventoryRefusal as R;
    with_ids(1, Mirroring::Single, false, |ids| {
        let raw = proposal();
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
            .expect("proposal");
        type Mutation = (F, fn(&mut PieceDefinition));
        let mutations: [Mutation; 9] = [
            (F::Quantity, |d| d.quantity = 2),
            (F::CutOnFold, |d| {
                d.cut_on_fold = true;
                d.fold_edges = vec![d.boundary.first().expect("actual boundary").edge];
            }),
            (F::CuttingSide, |d| d.cutting_side = CuttingSide::WrongSide),
            (F::Material, |d| {
                d.material = MaterialAssignment::Unresolved("different reason".into())
            }),
            (F::Layer, |d| d.layer_index += 1),
            (F::Label, |d| d.label.name.push('!')),
            (F::Boundary, |d| d.boundary.reverse()),
            (F::Holes, |d| d.holes.reverse()),
            (F::Construction, |d| d.construction_lines.reverse()),
        ];
        for (field, mutation) in mutations {
            let actual = inventory(&copies, keep, mutation);
            assert_eq!(
                copies
                    .bind_piece_inventory(
                        &actual,
                        &MirrorPieceCopyInventoryLimits::new(3, 20, 100, 1000)
                    )
                    .expect_err("complete canonical result field equality")
                    .refusal(),
                R::ResultField { field }
            );
        }
        let changed_source = inventory(&copies, |d| d.layer_index += 1, |d| d.quantity = 2);
        assert_eq!(
            copies
                .bind_piece_inventory(
                    &changed_source,
                    &MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220)
                )
                .expect_err("source preservation precedes result mismatch")
                .refusal(),
            R::SourceChanged
        );
    });
    with_cut(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::ReflectedSource,
        false,
        |ids| {
            let raw = proposal();
            let copies = raw
                .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
                .expect("fold proposal");
            let actual = inventory(&copies, keep, |d| {
                d.fold_edges = vec![d.boundary.first().expect("other actual boundary edge").edge]
            });
            assert_eq!(
                copies
                    .bind_piece_inventory(
                        &actual,
                        &MirrorPieceCopyInventoryLimits::new(3, 8, 32, 220)
                    )
                    .expect_err("fold identity remains canonical")
                    .refusal(),
                R::ResultField {
                    field: F::FoldEdges
                }
            );
        },
    );
}

#[test]
fn actual_reciprocal_companion_is_selected_by_identity_and_result_mirroring_must_match() {
    let member = Mirroring::PairMember {
        handedness: Handedness::Left,
        companion: id(300),
    };
    with_ids(1, member, false, |ids| {
        let raw = proposal();
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
            .expect("explicit member copy");
        let inventory = inventory(&copies, keep, keep);
        let limits = MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220);
        let bound = copies
            .bind_piece_inventory(&inventory, &limits)
            .expect("actual reciprocal candidate relation");
        let companion = bound.result_companion().expect("separate member");
        assert!(std::ptr::eq(
            companion,
            inventory.piece(id(300)).expect("actual canonical member")
        ));
        assert_eq!(
            companion.definition().mirroring,
            Mirroring::PairMember {
                handedness: Handedness::Right,
                companion: id(200)
            }
        );
        assert_eq!(
            companion.definition().quantity,
            bound.result_piece().definition().quantity
        );
        assert!(bound.source_companion().is_none());
    });
    with_ids(1, Mirroring::Single, false, |ids| {
        let raw = proposal();
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
            .expect("single proposal");
        let inventory = inventory(&copies, keep, |d| {
            d.mirroring = Mirroring::PairMember {
                handedness: Handedness::Left,
                companion: id(300),
            }
        });
        assert_eq!(
            copies
                .bind_piece_inventory(
                    &inventory,
                    &MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220)
                )
                .expect_err("birth-valid inventory is not this native result policy")
                .refusal(),
            MirrorPieceCopyInventoryRefusal::ResultField {
                field: MirrorPieceCopyInventoryField::Mirroring
            }
        );
    });
}
