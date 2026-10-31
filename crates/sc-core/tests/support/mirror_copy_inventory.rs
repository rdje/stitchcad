//! Shared actual candidate inventory fixture;explicit canonical registration is not issuance.
#[path = "mirror_result_copies.rs"]
pub mod copies;
use copies::context::edges::metadata::source as sources;
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
pub fn inventory(
    copies: &AdmittedMirrorPieceResultCopies<'_>,
    source_change: fn(&mut PieceDefinition),
    result_change: fn(&mut PieceDefinition),
) -> PieceInventory {
    let (mut ledger, _) = sources::fixture();
    let contours = copies.identities().source().context().contours();
    let identity = contours.coverage().edges().identity();
    let metadata = identity.roles().metadata();
    let cut = metadata.cut();
    let tags = contours
        .coverage()
        .edges()
        .edges()
        .map(|edge| edge.candidate().tag())
        .collect::<Vec<_>>();
    ledger
        .declare_tagged_edges(id(10), &tags)
        .expect("explicit fixture registration,not global issuance");
    ledger
        .declare_tagged_edges(id(70), &[LocalTag::FIRST])
        .expect("extra structural fixture edge");
    let mut source = cut.source().piece().definition().clone();
    source_change(&mut source);
    let source = Piece::new(source, &ledger).expect("actual structural candidate source");
    let mut result = PieceDefinition {
        id: *identity.piece_id(),
        boundary: contours.boundary().to_vec(),
        holes: contours.holes().to_vec(),
        construction_lines: contours.construction_lines().to_vec(),
        quantity: *cut.quantity(),
        mirroring: *cut.mirroring(),
        cut_on_fold: cut.cut_on_fold(),
        fold_edges: contours.fold_edges().to_vec(),
        cutting_side: *cut.cutting_side(),
        material: metadata.material().clone(),
        layer_index: *metadata.layer_index(),
        label: metadata.label().clone(),
    };
    result_change(&mut result);
    let mut other = cut.source().piece().definition().clone();
    other.id = id(300);
    other.quantity = 1;
    other.mirroring = Mirroring::Single;
    if let Mirroring::PairMember {
        handedness,
        companion,
    } = result.mirroring
    {
        assert_eq!(companion, id(300));
        other.quantity = result.quantity;
        other.mirroring = Mirroring::PairMember {
            handedness: match handedness {
                Handedness::Left => Handedness::Right,
                Handedness::Right => Handedness::Left,
            },
            companion: result.id,
        };
    }
    PieceInventory::new(vec![
        Piece::new(result, &ledger).expect("structural candidate result"),
        Piece::new(other, &ledger).expect("actual other/companion"),
        source,
    ])
    .expect("whole pair-consistent candidate inventory")
}
