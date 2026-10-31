//! Preserve actual complete physical-copy identities;do not infer copies from quantity or ordinals.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
#[path = "support/slash_sources.rs"]
mod sources;
use sources::*;
#[path = "support/slash_metadata.rs"]
mod metadata_context;
use metadata_context::with_boundary;

fn set_piece(f: &mut Fixture, quantity: u32, mirroring: Mirroring) {
    let mut definition = f.piece.definition().clone();
    definition.quantity = quantity;
    definition.mirroring = mirroring;
    f.piece = Piece::new(definition, &f.ledger).expect("born-valid canonical cut metadata");
}
fn row(copy: u128, piece: u128, orientation: CopyOrientation) -> CutCopyDefinition {
    CutCopyDefinition {
        id: id(copy),
        piece: id(piece),
        orientation,
    }
}
fn with_metadata<T>(f: &Fixture, run: impl FnOnce(&BoundSlashSpreadPieceMetadata<'_>) -> T) -> T {
    with_boundary(f, &contours(f), &manifest(f), |boundary| {
        let request = SlashSpreadPieceMetadataRequirement::preserve_source(
            SlashSpreadFoldIntent::NoFold,
            id(930),
            id(931),
        );
        let metadata = boundary
            .preserve_piece_metadata(&request)
            .expect("canonical source metadata");
        run(&metadata)
    })
}

#[test]
fn every_actual_copy_keeps_its_identity_orientation_and_authored_order_through_reordering() {
    use CopyOrientation::{Authored as A, Reflected as R};
    let mut f = fixture();
    set_piece(&mut f, 4, Mirroring::MirroredPairs);
    let rows = vec![
        row(410, 100, A),
        row(207, 100, R),
        row(900, 100, A),
        row(805, 100, R),
    ];
    let plan = CutPlan::new(rows.clone(), core::slice::from_ref(&f.piece))
        .expect("actual complete paired inventory");
    let mut reversed = rows.clone();
    reversed.reverse();
    let reordered = CutPlan::new(reversed.clone(), core::slice::from_ref(&f.piece))
        .expect("same actual ids in changed order");
    let journal = f.ledger.journal().len();
    with_metadata(&f, |metadata| {
        let policy = SlashSpreadCopyLimits::new(4);
        let bound = metadata
            .preserve_physical_copies(&plan, &policy)
            .expect("complete actual population");
        assert!(std::ptr::eq(bound.metadata(), metadata));
        assert!(std::ptr::eq(bound.plan(), &plan));
        assert!(std::ptr::eq(bound.limits(), &policy));
        assert_eq!(policy.copies(), 4);
        assert_eq!(bound.copy_count(), 4);
        assert_eq!(
            bound
                .copies()
                .map(|copy| *copy.definition())
                .collect::<Vec<_>>(),
            rows
        );
        for (actual, original) in bound.copies().zip(plan.copies()) {
            assert!(std::ptr::eq(actual, original));
        }
        assert_eq!(
            bound
                .copies()
                .rev()
                .map(|copy| *copy.definition())
                .collect::<Vec<_>>(),
            reversed
        );
        let after = metadata
            .preserve_physical_copies(&reordered, &policy)
            .expect("reordered identities preserved");
        assert_eq!(
            after
                .copies()
                .map(|copy| *copy.definition())
                .collect::<Vec<_>>(),
            reversed
        );
        for copy in after.copies() {
            assert_eq!(Some(copy), plan.copy(copy.id()));
        }
        assert_eq!(bound.metadata().requirement().all_copy_effects(), &id(931));
        let request = SlashSpreadEffectRequirement::new(id(940), id(941), id(942));
        let effect = bound.require_attributed_effect(&request);
        assert!(std::ptr::eq(effect.copies(), &bound));
        assert!(std::ptr::eq(effect.requirement(), &request));
        let original_intent = bound
            .metadata()
            .boundary()
            .consumers()
            .registration_plan()
            .sources()
            .contours()
            .edges()
            .bound_roles()
            .intent();
        assert!(std::ptr::eq(effect.output_intent(), original_intent));
        assert!(std::ptr::eq(
            effect.requested_fullness(),
            original_intent.fullness()
        ));
        assert!(std::ptr::eq(effect.input(), original_intent.input()));
        assert!(std::ptr::eq(
            effect.opening(),
            original_intent.input().opening()
        ));
        assert!(std::ptr::eq(
            effect.all_copy_effects_source(),
            bound.metadata().requirement().all_copy_effects()
        ));
        assert_eq!(effect.requirement().derivation(), &id(940));
        assert_eq!(effect.requirement().attribution(), &id(941));
        assert_eq!(effect.requirement().canonical_intakes(), &id(942));
        assert_eq!(
            effect.requested_fullness(),
            &SlashSpreadFullnessIntent::Added
        );
        assert_eq!(
            effect
                .copies()
                .copies()
                .map(|copy| *copy.definition())
                .collect::<Vec<_>>(),
            rows
        );
    });
    assert_eq!(f.ledger.journal().len(), journal);
    assert_eq!(
        plan.copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect::<Vec<_>>(),
        rows
    );
}

#[test]
fn whole_plan_quota_counts_unrelated_copies_before_source_quantity_association() {
    use CopyOrientation::Authored as A;
    use SlashSpreadCopyRefusal as R;
    let f = fixture();
    let mut other_definition = f.piece.definition().clone();
    other_definition.id = id(300);
    other_definition.quantity = 2;
    let other = Piece::new(other_definition, &f.ledger).expect("other actual Piece");
    let plan = CutPlan::new(
        vec![row(411, 300, A), row(410, 100, A), row(412, 300, A)],
        &[f.piece.clone(), other],
    )
    .expect("complete multi-Piece cut plan");
    let mut foreign_definition = f.piece.definition().clone();
    foreign_definition.quantity = 2;
    let foreign = Piece::new(foreign_definition, &f.ledger)
        .expect("different quantity,same declared Piece id");
    let wrong = CutPlan::new(vec![row(410, 100, A), row(411, 100, A)], &[foreign])
        .expect("valid against its own different Piece metadata");
    with_metadata(&f, |metadata| {
        let short = SlashSpreadCopyLimits::new(2);
        let error = metadata
            .preserve_physical_copies(&plan, &short)
            .expect_err("unrelated copies consume whole quota");
        assert_eq!(
            error.refusal(),
            R::Copies {
                required: 3,
                allowed: 2
            }
        );
        assert!(std::ptr::eq(error.metadata(), metadata));
        assert!(std::ptr::eq(error.plan(), &plan));
        assert!(std::ptr::eq(error.limits(), &short));
        let policy = SlashSpreadCopyLimits::new(3);
        let bound = metadata
            .preserve_physical_copies(&plan, &policy)
            .expect("one actual target copy");
        assert_eq!(bound.copy_count(), 1);
        assert_eq!(
            bound.copies().map(|copy| copy.id()).collect::<Vec<_>>(),
            vec![id(410)]
        );
        let zero = SlashSpreadCopyLimits::new(0);
        assert_eq!(
            metadata
                .preserve_physical_copies(&wrong, &zero)
                .expect_err("whole quota before different quantity")
                .refusal(),
            R::Copies {
                required: 2,
                allowed: 0
            }
        );
        assert_eq!(
            metadata
                .preserve_physical_copies(&wrong, &policy)
                .expect_err("do not infer target quantity from foreign inventory")
                .refusal(),
            R::Quantity {
                expected: 1,
                actual: 2
            }
        );
    });
}

#[test]
fn foreign_cut_metadata_cannot_supply_unbalanced_or_unexpected_reflections() {
    use CopyOrientation::{Authored as A, Reflected as R};
    let mut f = fixture();
    set_piece(&mut f, 4, Mirroring::MirroredPairs);
    let mut definition = f.piece.definition().clone();
    definition.mirroring = Mirroring::Single;
    let foreign = Piece::new(definition, &f.ledger).expect("other metadata with same id/quantity");
    let wrong = CutPlan::new(
        vec![
            row(400, 100, A),
            row(401, 100, A),
            row(402, 100, A),
            row(403, 100, A),
        ],
        &[foreign],
    )
    .expect("valid authored-only inventory for its own metadata");
    with_metadata(&f, |metadata| {
        let policy = SlashSpreadCopyLimits::new(4);
        assert_eq!(
            metadata
                .preserve_physical_copies(&wrong, &policy)
                .expect_err("source requires paired orientations")
                .refusal(),
            SlashSpreadCopyRefusal::UnbalancedPairs {
                authored: 4,
                reflected: 0
            }
        );
    });
    for mode in [
        Mirroring::Single,
        Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: id(300),
        },
    ] {
        let mut f = fixture();
        set_piece(&mut f, 1, mode);
        let mut definition = f.piece.definition().clone();
        definition.quantity = 2;
        definition.mirroring = Mirroring::MirroredPairs;
        let foreign = Piece::new(definition, &f.ledger)
            .expect("different mode/quantity for same declared id");
        let wrong = CutPlan::new(vec![row(400, 100, A), row(401, 100, R)], &[foreign])
            .expect("foreign paired inventory");
        with_metadata(&f, |metadata| {
            let policy = SlashSpreadCopyLimits::new(2);
            assert_eq!(
                metadata
                    .preserve_physical_copies(&wrong, &policy)
                    .expect_err("first reflection before quantity mismatch")
                    .refusal(),
                SlashSpreadCopyRefusal::UnexpectedReflection { copy: id(401) }
            );
        });
    }
}
