//! Actual aggregate checks must keep complete topology coverage separate from garment metadata.
#![allow(clippy::expect_used)]
use super::{garment_ownership, id};
use sc_core::{
    design::{DesignGarment, DesignInputs, DesignSewing, DesignStructure},
    ontology::*,
};
use sc_measure::DesignSources;

fn with_unused_piece(damage: bool) -> DesignGarment<DesignSources> {
    let mut garment = garment_ownership::definition();
    garment.layers = LayerInventory::new(
        garment.layers.facings().to_vec(),
        vec![],
        garment.layers.interfacings().to_vec(),
    )
    .expect("supported explicit layer scope");
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let mut ids = DeterministicIdGenerator::from_start(240000);
    let held = structure
        .topology
        .declare_edges(&mut ids, 1)
        .expect("actual unused source")
        .into_iter()
        .next()
        .expect("one authored source");
    let mut definition = structure
        .pieces
        .pieces()
        .first()
        .expect("actual Piece template")
        .definition()
        .clone();
    definition.id = id(240010);
    definition.boundary = vec![DirectedEdge {
        edge: held,
        direction: Direction::Original,
    }];
    let piece = Piece::new(definition, &structure.topology).expect("born actual unused Piece");
    for (edge, param) in piece.endpoint_references() {
        structure
            .topology
            .register(piece.id(), edge, param)
            .expect("explicit unused Piece endpoint registration");
    }
    if damage {
        let third = Param::new(Rational::new(1, 3).expect("third")).expect("valid bound");
        let half = Param::new(Rational::new(1, 2).expect("half")).expect("valid bound");
        let first = structure
            .topology
            .split(&mut ids, held, third)
            .expect("first split");
        let second = structure
            .topology
            .split(&mut ids, first.second(), half)
            .expect("second split");
        structure
            .topology
            .delete(&mut ids, second.first())
            .expect("delete only the middle");
    }
    let mut pieces = structure.pieces.pieces().to_vec();
    pieces.push(piece); // actual born object survives later history unchanged
    structure.pieces = PieceInventory::new(pieces).expect("complete original ownership");
    let mut copies = structure
        .cut_plan
        .copies()
        .iter()
        .map(|copy| *copy.definition())
        .collect::<Vec<_>>();
    copies.push(CutCopyDefinition {
        id: id(240011),
        piece: id(240010),
        orientation: CopyOrientation::Authored,
    });
    structure.cut_plan =
        CutPlan::new(copies, structure.pieces.pieces()).expect("complete actual quantities");
    inputs.structure = DesignStructure::new(structure).expect("held topology is inspectable");
    sewing.inputs = DesignInputs::new(inputs).expect("actual current canonical input root");
    garment.sewing = DesignSewing::new(sewing).expect("same actual sewing over expanded root");
    DesignGarment::new(garment).expect("actual coupled immutable aggregate")
}

#[test]
fn supported_current_aggregate_borrows_the_same_owner_for_topology_and_every_declaration() {
    let garment = with_unused_piece(false);
    let saved = garment.definition().clone();
    garment
        .require_in_scope()
        .expect("implemented explicit envelope");
    garment
        .validate_current()
        .expect("all current garment sources");
    let declarations = garment.declarations().expect("all current metadata roles");
    assert!(!declarations.is_empty());
    let structure = garment.inputs().structure();
    let topology = structure
        .topology_readiness()
        .expect("all registered points and held intervals");
    assert!(core::ptr::eq(topology.structure(), structure));
    assert!(topology.is_ready());
    assert_eq!(topology.piece_edges().len(), 3);
    assert_eq!(topology.registrations().len(), 2);
    assert!(declarations
        .iter()
        .all(|view| core::ptr::eq(view.canonical_inputs(), structure.inputs())));
    assert_eq!(garment.definition(), &saved);
}

#[test]
fn current_sources_and_live_points_cannot_certify_an_unused_pieces_deleted_interior() {
    let garment = with_unused_piece(true);
    let saved = garment.definition().clone();
    garment
        .require_in_scope()
        .expect("explicit scope does not inspect topology");
    garment
        .validate_current()
        .expect("current garment references do not use the third Piece");
    garment
        .declarations()
        .expect("complete current roles do not replace topology coverage");
    let structure = garment.inputs().structure();
    assert_eq!(
        structure.topology().release_readiness(),
        ReleaseReadiness::Releasable
    );
    let report = structure
        .topology_readiness()
        .expect("original registered point answers");
    assert!(core::ptr::eq(report.structure(), structure));
    assert!(report
        .registrations()
        .iter()
        .all(|(_, answer)| answer.resolved().is_some()));
    let unused = report
        .piece_edges()
        .iter()
        .find(|edge| edge.piece() == id(240010))
        .expect("unused Piece included");
    assert!(unused.start().resolved().is_some() && unused.end().resolved().is_some());
    assert_eq!(unused.range().portions().len(), 3);
    assert_eq!(unused.range().repairs().count(), 1);
    assert!(!unused.is_ready() && !report.is_ready());
    assert!(report
        .piece_edges()
        .iter()
        .filter(|edge| edge.piece() != id(240010))
        .all(sc_core::design::DesignPieceTopology::is_ready));
    assert_eq!(garment.definition(), &saved);
}
