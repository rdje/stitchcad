//! Explicit selected metadata,whole text admission and one canonical print authority.
#![allow(clippy::expect_used)]
#[path = "support/mirror_source.rs"]
mod common;
use common::{axis, fixture, id, input};
use sc_core::{
    construction::{
        MirrorPieceCutPolicy, MirrorPieceFieldIntent as Field, MirrorPieceFoldIntent,
        MirrorPieceMetadataLimits as Limits, MirrorPieceMetadataPolicy as Policy,
        MirrorPieceMetadataRefusal as Refusal, MirrorPieceSourceLimits,
    },
    ontology::*,
};

fn label() -> LabelText {
    LabelText {
        name: "n".into(),
        size: "s".into(),
        fabric: "f".into(),
        colorway: "c".into(),
    }
}

#[test]
fn preserve_borrows_actual_fields_but_prints_the_one_explicit_result_cut_request() {
    let (_, piece) = fixture();
    let original = piece.clone();
    let input = input(10, 100, axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source
        .bind_cut_policy(&cut_policy)
        .expect("explicit new cut request");
    let policy = Policy::new(
        Field::PreserveSource,
        Field::PreserveSource,
        Field::PreserveSource,
    );
    let limits = Limits::new(55);
    let metadata = cut
        .bind_metadata(&policy, &limits)
        .expect("all55 resolved UTF-8 bytes");
    assert!(std::ptr::eq(metadata.cut(), &cut));
    assert!(std::ptr::eq(metadata.policy(), &policy));
    assert!(std::ptr::eq(metadata.limits(), &limits));
    assert!(std::ptr::eq(
        metadata.material(),
        &piece.definition().material
    ));
    assert!(std::ptr::eq(
        metadata.layer_index(),
        &piece.definition().layer_index
    ));
    assert!(std::ptr::eq(metadata.label(), &piece.definition().label));
    assert_eq!(metadata.text_bytes(), 55);
    assert_eq!(limits.text_bytes(), 55);
    assert_eq!(
        metadata.material(),
        &MaterialAssignment::Unresolved("  original material reason  ".into())
    );
    let printed = metadata.printed_label();
    assert!(std::ptr::eq(printed.text, metadata.label()));
    assert_eq!(printed.quantity, *cut.quantity());
    assert_eq!(printed.mirroring, *cut.mirroring());
    assert_eq!(printed.cut_on_fold, cut.cut_on_fold());
    assert_eq!(printed.quantity, 1);
    assert_eq!(printed.mirroring, Mirroring::Single);
    assert!(!printed.cut_on_fold);
    assert_eq!(piece.printed_label().quantity, 4);
    assert!(piece.printed_label().cut_on_fold);
    assert_eq!(piece, original);
}

#[test]
fn replacements_borrow_exact_request_values_and_count_utf8_bytes_without_normalization() {
    let (_, piece) = fixture();
    let input = input(10, 100, axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(
        2,
        Mirroring::MirroredPairs,
        MirrorPieceFoldIntent::ReflectedSource,
        CuttingSide::WrongSide,
    );
    let cut = source
        .bind_cut_policy(&cut_policy)
        .expect("explicit cut request");
    let policy = Policy::new(
        Field::Replace(MaterialAssignment::Unresolved(" 待定 ".into())),
        Field::Replace(i32::MIN),
        Field::Replace(LabelText {
            name: " 左 ".into(),
            size: "L".into(),
            fabric: "绸".into(),
            colorway: "赤".into(),
        }),
    );
    let limits = Limits::new(20);
    let metadata = cut
        .bind_metadata(&policy, &limits)
        .expect("12 label+8 reason bytes");
    if let Field::Replace(material) = policy.material() {
        assert!(std::ptr::eq(metadata.material(), material));
    }
    if let Field::Replace(layer) = policy.layer_index() {
        assert!(std::ptr::eq(metadata.layer_index(), layer));
    }
    if let Field::Replace(label) = policy.label() {
        assert!(std::ptr::eq(metadata.label(), label));
    }
    assert_eq!(metadata.text_bytes(), 20);
    assert_eq!(metadata.label().name, " 左 ");
    assert_eq!(
        metadata.material(),
        &MaterialAssignment::Unresolved(" 待定 ".into())
    );
    assert_eq!(*metadata.layer_index(), i32::MIN);
    assert!(metadata.printed_label().cut_on_fold);
    assert_eq!(metadata.printed_label().quantity, 2);
}

#[test]
fn mixed_fields_charge_only_resolved_text_and_do_not_resolve_an_assigned_material_id() {
    let (_, piece) = fixture();
    let input = input(10, 100, axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source
        .bind_cut_policy(&cut_policy)
        .expect("explicit cut request");
    let policy = Policy::new(
        Field::Replace(MaterialAssignment::Assigned(id(700))),
        Field::PreserveSource,
        Field::Replace(label()),
    );
    let limits = Limits::new(4);
    let metadata = cut
        .bind_metadata(&policy, &limits)
        .expect("four selected label bytes only");
    assert_eq!(metadata.material(), &MaterialAssignment::Assigned(id(700)));
    assert!(std::ptr::eq(
        metadata.layer_index(),
        &piece.definition().layer_index
    ));
    assert_eq!(metadata.label().name, "n");
    assert_eq!(metadata.text_bytes(), 4);
}

#[test]
fn whole_byte_quota_precedes_blank_label_and_material_with_all_original_refusal_context() {
    let (_, piece) = fixture();
    let input = input(10, 100, axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source
        .bind_cut_policy(&cut_policy)
        .expect("explicit cut request");
    let policy = Policy::new(
        Field::Replace(MaterialAssignment::Unresolved("\u{2003}".into())),
        Field::PreserveSource,
        Field::Replace(LabelText {
            name: "  ".into(),
            size: "\t".into(),
            fabric: "\u{2003}".into(),
            colorway: "z".into(),
        }),
    );
    let limits = Limits::new(9);
    let error = cut
        .bind_metadata(&policy, &limits)
        .expect_err("all10 bytes before any blank scan");
    assert_eq!(
        error.refusal(),
        Refusal::TextBytes {
            required: 10,
            allowed: 9
        }
    );
    assert!(std::ptr::eq(error.cut(), &cut));
    assert!(std::ptr::eq(error.policy(), &policy));
    assert!(std::ptr::eq(error.limits(), &limits));
    let exact = Limits::new(10);
    assert_eq!(
        cut.bind_metadata(&policy, &exact)
            .expect_err("name before other blank fields/material")
            .refusal(),
        Refusal::IncompleteLabel(LabelField::Name)
    );
}

#[test]
fn canonical_label_field_checks_precede_material_reason_and_preserve_unicode_blank_causes() {
    let (_, piece) = fixture();
    let input = input(10, 100, axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::Single,
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source
        .bind_cut_policy(&cut_policy)
        .expect("explicit cut request");
    let limits = Limits::new(9);
    for field in [
        LabelField::Name,
        LabelField::Size,
        LabelField::Fabric,
        LabelField::Colorway,
    ] {
        let mut text = label();
        let selected = match field {
            LabelField::Name => &mut text.name,
            LabelField::Size => &mut text.size,
            LabelField::Fabric => &mut text.fabric,
            LabelField::Colorway => &mut text.colorway,
        };
        *selected = "\u{2003}".into();
        let policy = Policy::new(
            Field::Replace(MaterialAssignment::Unresolved("\u{2003}".into())),
            Field::PreserveSource,
            Field::Replace(text),
        );
        assert_eq!(
            cut.bind_metadata(&policy, &limits)
                .expect_err("exact9 bytes;label before material")
                .refusal(),
            Refusal::IncompleteLabel(field)
        );
    }
    let policy = Policy::new(
        Field::Replace(MaterialAssignment::Unresolved("\u{2003}".into())),
        Field::PreserveSource,
        Field::Replace(label()),
    );
    assert_eq!(
        cut.bind_metadata(&policy, &limits)
            .expect_err("complete label but unexplained material")
            .refusal(),
        Refusal::UnexplainedMaterial
    );
}
