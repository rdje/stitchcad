//! Explicit joined fields and complete selected-text admission,without output/cut compatibility.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
#[allow(dead_code)]
#[path = "support/seam_edit_sources.rs"]
mod common;
fn label(name: &str, size: &str, fabric: &str, colorway: &str) -> LabelText {
    LabelText {
        name: name.into(),
        size: size.into(),
        fabric: fabric.into(),
        colorway: colorway.into(),
    }
}
fn with_splice(run: impl FnOnce(RequiredJoinExactSplice<'_>)) {
    let f = common::fixture(false);
    let inventory = PieceInventory::new(
        f.pieces
            .iter()
            .enumerate()
            .map(|(index, piece)| {
                let mut definition = piece.definition().clone();
                if index == 0 {
                    definition.label = label("front", "M", "cotton", "blue");
                    definition.material = MaterialAssignment::Unresolved("source A pending".into());
                    definition.layer_index = 7;
                    definition.cutting_side = CuttingSide::Face;
                } else {
                    definition.label = label("back", "L", "linen", "ivory");
                    definition.material = MaterialAssignment::Assigned(id(501));
                    definition.layer_index = -3;
                    definition.cutting_side = CuttingSide::WrongSide;
                }
                Piece::new(definition, &f.ledger).expect("actual source metadata")
            })
            .collect(),
    )
    .expect("whole inventory");
    let edit = JoinPiecesAtSeam::new(id(1000), id(301), id(100), id(101)).expect("join input");
    let source = edit
        .bind_source(&f.graph, &f.plan, inventory.pieces())
        .expect("actual source slice");
    let limits = JoinCopyInventoryLimits::new(2, 2, 8);
    let copies = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("whole source inventory");
    let request = JoinExactSpliceRequirement::new(JoinSplicePlacement::KeepOriginal);
    run(copies.require_exact_splice(&request));
}
fn policy(material: MaterialAssignment, text: LabelText) -> JoinPieceMetadataPolicy {
    JoinPieceMetadataPolicy::new(
        JoinPieceFieldIntent::Authored(material),
        JoinPieceFieldIntent::Authored(99),
        JoinPieceFieldIntent::Authored(text),
        JoinPieceFieldIntent::Authored(CuttingSide::WrongSide),
    )
}
fn authored<T>(intent: &JoinPieceFieldIntent<T>) -> Option<&T> {
    match intent {
        JoinPieceFieldIntent::Authored(value) => Some(value),
        JoinPieceFieldIntent::FromSide(_) => None,
    }
}
#[test]
fn either_source_keeps_actual_original_field_pointers_and_only_selected_text_counts() {
    with_splice(|splice| {
        for (side, bytes) in [(SeamSideId::A, 32), (SeamSideId::B, 15)] {
            let policy = JoinPieceMetadataPolicy::new(
                JoinPieceFieldIntent::FromSide(side),
                JoinPieceFieldIntent::FromSide(side),
                JoinPieceFieldIntent::FromSide(side),
                JoinPieceFieldIntent::FromSide(side),
            );
            let limits = JoinPieceMetadataLimits::new(bytes);
            let bound = splice
                .bind_metadata(&policy, &limits)
                .expect("complete original selected fields");
            let original = splice.source().side(side).piece().definition();
            assert!(std::ptr::eq(bound.splice(), &splice));
            assert!(std::ptr::eq(bound.policy(), &policy));
            assert!(std::ptr::eq(bound.limits(), &limits));
            assert!(std::ptr::eq(bound.material(), &original.material));
            assert!(std::ptr::eq(bound.layer_index(), &original.layer_index));
            assert!(std::ptr::eq(bound.label(), &original.label));
            assert!(std::ptr::eq(bound.cutting_side(), &original.cutting_side));
            assert_eq!((bound.text_bytes(), limits.text_bytes()), (bytes, bytes));
            assert!(format!("{bound:?}").contains("BoundJoinPieceMetadata"));
        }
    });
}
#[test]
fn authored_unicode_and_mixed_fields_keep_original_policy_values_without_face_inference() {
    with_splice(|splice| {
        let raw = policy(
            MaterialAssignment::Unresolved("new β".into()),
            label("合片", "M", "linen", "蓝"),
        );
        let snapshot = raw.clone();
        let limits = JoinPieceMetadataLimits::new(21);
        let bound = splice
            .bind_metadata(&raw, &limits)
            .expect("21 actual selected UTF-8 bytes");
        let material = authored(raw.material()).expect("authored material");
        let layer = authored(raw.layer_index()).expect("authored layer");
        let text = authored(raw.label()).expect("authored label");
        let face = authored(raw.cutting_side()).expect("authored face");
        assert!(std::ptr::eq(bound.material(), material));
        assert!(std::ptr::eq(bound.layer_index(), layer));
        assert!(std::ptr::eq(bound.label(), text));
        assert!(std::ptr::eq(bound.cutting_side(), face));
        assert_eq!(bound.text_bytes(), 21);
        assert_eq!(raw, snapshot);
        let mixed = JoinPieceMetadataPolicy::new(
            JoinPieceFieldIntent::FromSide(SeamSideId::B),
            JoinPieceFieldIntent::Authored(-11),
            JoinPieceFieldIntent::FromSide(SeamSideId::A),
            JoinPieceFieldIntent::FromSide(SeamSideId::A),
        );
        let limits = JoinPieceMetadataLimits::new(16);
        let bound = splice
            .bind_metadata(&mixed, &limits)
            .expect("mixed choices explicit");
        assert!(std::ptr::eq(
            bound.material(),
            &splice
                .source()
                .side(SeamSideId::B)
                .piece()
                .definition()
                .material
        ));
        assert!(std::ptr::eq(
            bound.label(),
            &splice
                .source()
                .side(SeamSideId::A)
                .piece()
                .definition()
                .label
        ));
        assert_eq!(*bound.layer_index(), -11);
        assert_eq!(*bound.cutting_side(), CuttingSide::Face);
    });
}
#[test]
fn complete_utf8_quota_precedes_all_canonical_blank_label_and_material_causes() {
    use JoinPieceMetadataRefusal as R;
    with_splice(|splice| {
        let raw = policy(
            MaterialAssignment::Unresolved("\u{2003}".into()),
            label(" \u{2003}", "\t", "\n", " "),
        );
        let limits = JoinPieceMetadataLimits::new(9);
        let error = splice
            .bind_metadata(&raw, &limits)
            .expect_err("full10 before blank name");
        assert!(std::ptr::eq(error.splice(), &splice));
        assert!(std::ptr::eq(error.policy(), &raw));
        assert!(std::ptr::eq(error.limits(), &limits));
        assert_eq!(
            error.refusal(),
            R::TextBytes {
                required: 10,
                allowed: 9
            }
        );
        assert!(error.to_string().starts_with("join piece metadata:"));
        assert!(format!("{error:?}").contains("JoinPieceMetadataError"));
        assert_eq!(
            splice
                .bind_metadata(&raw, &JoinPieceMetadataLimits::new(10))
                .expect_err("canonical name first")
                .refusal(),
            R::IncompleteLabel(LabelField::Name)
        );
        for field in [
            LabelField::Name,
            LabelField::Size,
            LabelField::Fabric,
            LabelField::Colorway,
        ] {
            let mut text = label("n", "s", "f", "c");
            *match field {
                LabelField::Name => &mut text.name,
                LabelField::Size => &mut text.size,
                LabelField::Fabric => &mut text.fabric,
                LabelField::Colorway => &mut text.colorway,
            } = " \t".into();
            let raw = policy(MaterialAssignment::Unresolved(" ".into()), text);
            assert_eq!(
                splice
                    .bind_metadata(&raw, &JoinPieceMetadataLimits::new(6))
                    .expect_err("label before unresolved reason")
                    .refusal(),
                R::IncompleteLabel(field)
            );
        }
        let raw = policy(
            MaterialAssignment::Unresolved("\u{2003}".into()),
            label("n", "s", "f", "c"),
        );
        assert_eq!(
            splice
                .bind_metadata(&raw, &JoinPieceMetadataLimits::new(7))
                .expect_err("actual blank material")
                .refusal(),
            R::UnexplainedMaterial
        );
    });
}
