//! Actual complete truing source and copy/Piece context;no refit or provider evidence.
use sc_core::{construction::*, ontology::*};
use sc_units::{Length, Tolerance, ToleranceClass};
pub fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
#[path = "seam_edit_sources.rs"]
pub mod sources;
pub fn with_copies(shared: bool, run: impl FnOnce(BoundTruingCopyInventory<'_>)) {
    with_materials(shared, None, run);
}
pub fn with_materials(
    shared: bool,
    materials: Option<[EntityId; 2]>,
    run: impl FnOnce(BoundTruingCopyInventory<'_>),
) {
    let f = sources::fixture(shared);
    assert_eq!(f.marks.len(), 1);
    let fit = Tolerance::new(
        ToleranceClass::GeometricApproximation,
        Length::from_micrometres(7).expect("length"),
        "explicit fixture length-fit source;not provider evidence",
    )
    .expect("requirement");
    let edit = ConstrainedSeamTruing::new(id(1000), id(301), SeamSideId::A, id(100), id(9000))
        .expect("inputs");
    let source = edit
        .bind_source(
            &f.graph,
            &f.plan,
            TruingFitProvider {
                source: id(9000),
                tolerance: &fit,
            },
        )
        .expect("actual source");
    assert!(std::ptr::eq(source.span(), sources::first_span(&f.graph)));
    let inventory = PieceInventory::new(
        f.pieces
            .iter()
            .enumerate()
            .map(|(index, p)| {
                let mut definition = p.definition().clone();
                if let Some(material) = materials.and_then(|ids| ids.get(index).copied()) {
                    definition.material = MaterialAssignment::Assigned(material);
                }
                Piece::new(definition, &f.ledger).expect("actual independently born Piece")
            })
            .collect(),
    )
    .expect("whole inventory");
    let limits =
        TruingCopyInventoryLimits::new(2, inventory.pieces().len(), inventory.pieces().len() + 6);
    let copies = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("actual both-side copy/Piece association");
    run(copies);
}
