//! Exact actual candidate differences, with closed coverage of every implemented owner definition.
use super::CommandMaterialChange;
use crate::{
    command::{AddNotch, AssignMaterial},
    design::{
        DesignGarment, DesignGarmentDefinition, DesignInputsDefinition, DesignReferenceHost,
        DesignSewingDefinition, DesignStructureDefinition,
    },
    ontology::{EntityId, Notch, Piece, SewingLandmark},
};
use core::fmt;

/// A private candidate changed content outside its implemented semantic operation.
/// This is an implementation-integrity refusal, never a partially applied command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDifferenceError {
    /// Original garment metadata families differ.
    GarmentContext,
    /// Original sewing graph differs.
    SewingGraph,
    /// Original canonical recipe, registry, higher sources or references differ.
    InputContext,
    /// Original Design identity/revision, canonical values, cut plan or ledger differs.
    StructuralContext,
    /// Original Piece count, identity order or an unpermitted Piece field differs.
    Pieces,
    /// Original semantic landmark order/content or append-only Notch shape differs.
    Landmarks,
    /// Actual selected Material assignment differs from explicit intent.
    MaterialIntent {
        /// Actual selected Piece identity.
        piece: EntityId,
    },
    /// Actual appended Notch differs from its intended owner/anchor/bindings/staged identity.
    NotchIntent,
}
impl fmt::Display for CommandDifferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "private candidate difference refused: {self:?}")
    }
}
impl std::error::Error for CommandDifferenceError {}

// Each pair of exhaustive patterns deliberately has no `..`: adding any owned definition field
// requires an explicit comparison decision here. Only Pieces/landmarks are compared separately.
pub(super) fn unchanged_context<H: DesignReferenceHost + PartialEq>(
    before: &DesignGarment<H>,
    after: &DesignGarment<H>,
) -> Result<(), CommandDifferenceError> {
    let DesignGarmentDefinition {
        sewing: _,
        pockets,
        notions,
        closures,
        layers,
        hems,
        constructions,
    } = before.definition();
    let original = (pockets, notions, closures, layers, hems, constructions);
    let DesignGarmentDefinition {
        sewing: _,
        pockets,
        notions,
        closures,
        layers,
        hems,
        constructions,
    } = after.definition();
    if original != (pockets, notions, closures, layers, hems, constructions) {
        return Err(CommandDifferenceError::GarmentContext);
    }
    let DesignSewingDefinition {
        inputs: _,
        graph,
        landmarks: _,
    } = before.sewing().definition();
    let original = graph;
    let DesignSewingDefinition {
        inputs: _,
        graph,
        landmarks: _,
    } = after.sewing().definition();
    if original != graph {
        return Err(CommandDifferenceError::SewingGraph);
    }
    let DesignInputsDefinition {
        structure: _,
        parameters,
        recipe,
        materials,
        sources,
        references,
    } = before.inputs().definition();
    let original = (parameters, recipe, materials, sources, references);
    let DesignInputsDefinition {
        structure: _,
        parameters,
        recipe,
        materials,
        sources,
        references,
    } = after.inputs().definition();
    if original != (parameters, recipe, materials, sources, references) {
        return Err(CommandDifferenceError::InputContext);
    }
    let DesignStructureDefinition {
        id,
        revision,
        inputs,
        pieces: _,
        cut_plan,
        topology,
    } = before.inputs().structure().definition();
    let original = (id, revision, inputs, cut_plan, topology);
    let DesignStructureDefinition {
        id,
        revision,
        inputs,
        pieces: _,
        cut_plan,
        topology,
    } = after.inputs().structure().definition();
    if original != (id, revision, inputs, cut_plan, topology) {
        return Err(CommandDifferenceError::StructuralContext);
    }
    Ok(())
}

pub(super) fn material_changes(
    before: &[Piece],
    after: &[Piece],
    intent: &AssignMaterial,
) -> Result<Vec<CommandMaterialChange>, CommandDifferenceError> {
    if before.len() != after.len() {
        return Err(CommandDifferenceError::Pieces);
    }
    for (original, candidate) in before.iter().zip(after) {
        if original.id() != candidate.id() {
            return Err(CommandDifferenceError::Pieces);
        }
        if intent.pieces().pieces().contains(&original.id()) {
            if &candidate.definition().material != intent.material() {
                return Err(CommandDifferenceError::MaterialIntent {
                    piece: original.id(),
                });
            }
            // Compare complete held Piece value, allowing exactly the actual assignment field.
            if original
                .with_material_assignment(candidate.definition().material.clone())
                .as_ref()
                != Ok(candidate)
            {
                return Err(CommandDifferenceError::Pieces);
            }
        } else if original != candidate {
            return Err(CommandDifferenceError::Pieces);
        }
    }
    let mut changes = Vec::new();
    for identity in intent.pieces().pieces() {
        let original = before
            .iter()
            .find(|piece| piece.id() == *identity)
            .ok_or(CommandDifferenceError::Pieces)?;
        let candidate = after
            .iter()
            .find(|piece| piece.id() == *identity)
            .ok_or(CommandDifferenceError::Pieces)?;
        if original.definition().material != candidate.definition().material {
            changes.push(CommandMaterialChange {
                piece: *identity,
                before: original.definition().material.clone(),
                after: candidate.definition().material.clone(),
            });
        }
    }
    Ok(changes)
}

pub(super) fn added_notch(
    before: &[SewingLandmark],
    after: &[SewingLandmark],
    intent: AddNotch,
    identity: EntityId,
) -> Result<Notch, CommandDifferenceError> {
    let Some((SewingLandmark::Notch(notch), prefix)) = after.split_last() else {
        return Err(CommandDifferenceError::Landmarks);
    };
    if prefix != before {
        return Err(CommandDifferenceError::Landmarks);
    }
    if notch.piece() != intent.piece() || notch.definition() != &intent.definition_with_id(identity)
    {
        return Err(CommandDifferenceError::NotchIntent);
    }
    Ok(notch.clone())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use super::*;
    use crate::{
        command::PieceSelection,
        ontology::{
            CuttingSide, DeterministicIdGenerator, DirectedEdge, Direction, EdgeAnchor,
            IdentityLedger, LabelText, MaterialAssignment, Mirroring, NotchDimensionBindings,
            NotchProfileBindings, Param, PieceDefinition, ProfileParameterRef,
        },
    };

    fn fixture() -> (Vec<Piece>, IdentityLedger) {
        let mut ledger = IdentityLedger::new();
        let edges = ledger
            .declare_edges(&mut DeterministicIdGenerator::new(), 2)
            .expect("actual declared topology");
        let pieces = edges
            .into_iter()
            .enumerate()
            .map(|(index, edge)| {
                Piece::new(
                    PieceDefinition {
                        id: EntityId::from_bits(100 + index as u128),
                        boundary: vec![DirectedEdge {
                            edge,
                            direction: Direction::Original,
                        }],
                        holes: vec![],
                        construction_lines: vec![],
                        quantity: 1,
                        mirroring: Mirroring::Single,
                        cut_on_fold: false,
                        fold_edges: vec![],
                        cutting_side: CuttingSide::Face,
                        material: MaterialAssignment::Unresolved("original exact intent".into()),
                        layer_index: 0,
                        label: LabelText {
                            name: "actual Piece".into(),
                            size: "M".into(),
                            fabric: "printed text".into(),
                            colorway: "original".into(),
                        },
                    },
                    &ledger,
                )
                .expect("actual valid immutable Piece")
            })
            .collect();
        (pieces, ledger)
    }
    fn assignment(piece: EntityId, material: MaterialAssignment) -> AssignMaterial {
        AssignMaterial::new(
            PieceSelection::new(vec![piece]).expect("actual selection"),
            material,
        )
        .expect("nonblank actual intent")
    }

    #[test]
    fn actual_material_diff_rejects_unrelated_piece_fields_order_and_unselected_changes() {
        let (before, ledger) = fixture();
        let first = before.first().expect("actual first Piece");
        let intent = assignment(
            first.id(),
            MaterialAssignment::Assigned(EntityId::from_bits(500)),
        );
        let mut after = before.clone();
        *after.first_mut().expect("candidate target") = first
            .with_material_assignment(intent.material().clone())
            .expect("exact replacement");
        let changes =
            material_changes(&before, &after, &intent).expect("actual changed assignment");
        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes.first().expect("actual change").before(),
            &first.definition().material
        );
        assert_eq!(
            changes.first().expect("actual change").after(),
            intent.material()
        );
        let mut changed_label = after.clone();
        let mut definition = changed_label
            .first()
            .expect("actual changed Piece")
            .definition()
            .clone();
        definition.label.fabric.push_str(" unintended");
        *changed_label.first_mut().expect("actual target") =
            Piece::new(definition, &ledger).expect("valid unrelated label mutation");
        assert_eq!(
            material_changes(&before, &changed_label, &intent),
            Err(CommandDifferenceError::Pieces)
        );
        let mut reversed = after.clone();
        reversed.reverse();
        assert_eq!(
            material_changes(&before, &reversed, &intent),
            Err(CommandDifferenceError::Pieces)
        );
        let last = after.last_mut().expect("unselected Piece");
        *last = last
            .with_material_assignment(intent.material().clone())
            .expect("valid unselected mutation");
        assert_eq!(
            material_changes(&before, &after, &intent),
            Err(CommandDifferenceError::Pieces)
        );
    }

    #[test]
    fn actual_material_diff_omits_noops_and_refuses_mismatched_requested_assignment() {
        let (before, _) = fixture();
        let first = before.first().expect("actual target");
        let noop = assignment(first.id(), first.definition().material.clone());
        assert!(material_changes(&before, &before, &noop)
            .expect("exact noop")
            .is_empty());
        let intent = assignment(
            first.id(),
            MaterialAssignment::Unresolved("  new reason\n".into()),
        );
        assert_eq!(
            material_changes(&before, &before, &intent),
            Err(CommandDifferenceError::MaterialIntent { piece: first.id() })
        );
        let mut after = before.clone();
        *after.first_mut().expect("candidate target") = first
            .with_material_assignment(intent.material().clone())
            .expect("exact reason replacement");
        assert_eq!(
            material_changes(&before, &after, &intent)
                .expect("actual reason change")
                .first()
                .expect("one exact change")
                .after(),
            intent.material()
        );
    }

    #[test]
    fn actual_notch_diff_checks_original_prefix_and_exact_appended_identity_and_bindings() {
        let (pieces, ledger) = fixture();
        let piece = pieces.first().expect("actual owner");
        let reference = ProfileParameterRef::new(EntityId::from_bits(600));
        let bindings = NotchProfileBindings {
            style: reference,
            sample: NotchDimensionBindings {
                depth: reference,
                width: reference,
            },
            production: NotchDimensionBindings {
                depth: reference,
                width: reference,
            },
            encoding: reference,
        };
        let intent = AddNotch::new(
            piece.id(),
            EdgeAnchor {
                edge: piece.edges().next().expect("actual edge").edge,
                param: Param::START,
            },
            bindings,
        );
        let original = Notch::new(
            intent.definition_with_id(EntityId::from_bits(700)),
            piece,
            &ledger,
        )
        .expect("original actual landmark");
        let identity = EntityId::from_bits(701);
        let added = Notch::new(intent.definition_with_id(identity), piece, &ledger)
            .expect("actual appended landmark");
        let before = vec![SewingLandmark::Notch(original.clone())];
        let after = vec![
            SewingLandmark::Notch(original),
            SewingLandmark::Notch(added.clone()),
        ];
        assert_eq!(added_notch(&before, &after, intent, identity), Ok(added));
        assert_eq!(
            added_notch(&before, &after, intent, EntityId::from_bits(702)),
            Err(CommandDifferenceError::NotchIntent)
        );
        let mut reversed = after;
        reversed.reverse();
        assert_eq!(
            added_notch(&before, &reversed, intent, identity),
            Err(CommandDifferenceError::Landmarks)
        );
    }
}
