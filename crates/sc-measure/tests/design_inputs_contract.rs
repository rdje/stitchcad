//! Actual higher source schemas compose with core input ownership without a dependency cycle.
#![allow(clippy::expect_used)]
#[path = "support/owned_size_set_fixture.rs"]
mod support;
use sc_core::{
    command::DesignRevision,
    design::{
        DesignHostEntity, DesignHostEntityKind, DesignInputs, DesignInputsDefinition,
        DesignInputsError, DesignInputsPosition, DesignMetadata, DesignMetadataError,
        DesignParameterDefinition, DesignParameterTarget, DesignParameters, DesignStructure,
        DesignStructureDefinition, DesignStructurePosition,
    },
    name::MachineToken,
    ontology::{CutPlan, IdentityLedger, MaterialInventory, PieceInventory},
    recipe::FormulaRecipeDocument,
    value::CanonicalInputs,
};
use sc_measure::*;
use support::id;
fn fixture() -> DesignInputsDefinition<DesignSources> {
    let (canonical, measurements, mtm, garment) = support::inventory_fixture();
    let size_sets =
        SizeSetInventory::new(vec![mtm.into(), garment.into()], &canonical, &measurements)
            .expect("actual external sources");
    let host = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets,
        },
        &canonical,
    )
    .expect("actual metadata host");
    let sources = DesignMetadata::new(host, &canonical).expect("actual core host acceptance");
    let parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(5001),
            token: MachineToken::new("authored_body").expect("explicit name"),
            target: DesignParameterTarget::Length(id(10)),
        }],
        &canonical,
    )
    .expect("actual canonical parameter target");
    let materials =
        MaterialInventory::new(vec![], &canonical).expect("explicit empty material inventory");
    let structure = DesignStructure::new(DesignStructureDefinition {
        id: id(5000),
        revision: DesignRevision::initial(),
        inputs: canonical,
        pieces: PieceInventory::new(vec![]).expect("explicit empty Pieces"),
        cut_plan: CutPlan::new(vec![], &[]).expect("explicit empty plan"),
        topology: IdentityLedger::new(),
    })
    .expect("actual structural owner");
    DesignInputsDefinition {
        structure,
        parameters,
        materials,
        sources,
        recipe: FormulaRecipeDocument::new("let width:length=body_waist".to_owned())
            .expect("exact owned recipe syntax"),
    }
}
#[test]
fn actual_source_host_borrows_the_single_canonical_owner_inside_core_composition() {
    let state = DesignInputs::new(fixture()).expect("actual cross-crate composition");
    assert_eq!(state.id(), id(5000));
    assert!(core::ptr::eq(state.inputs(), state.structure().inputs()));
    let declaration = state
        .sources()
        .measurement_declaration(id(1), state.inputs())
        .expect("actual current body binding");
    assert!(core::ptr::eq(
        declaration,
        state
            .inputs()
            .length(id(10))
            .expect("single actual canonical record")
    ));
    assert_eq!(state.sources().host().size_sets().sets().len(), 2);
    assert_eq!(state.recipe().source(), "let width:length=body_waist");
}
#[test]
fn actual_external_set_metadata_cannot_reuse_the_containing_design_identity() {
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    structure.id = id(40);
    definition.structure = DesignStructure::new(structure).expect("valid standalone structural id");
    assert_eq!(
        DesignInputs::new(definition).expect_err("actual whole registry collision"),
        DesignInputsError::DuplicateIdentity {
            identity: id(40),
            first: DesignInputsPosition::Structure(DesignStructurePosition::Design),
            duplicate: DesignInputsPosition::Source(DesignHostEntity {
                id: id(40),
                kind: DesignHostEntityKind::SizeSet,
                index: 0
            })
        }
    );
}
#[test]
fn sources_are_revalidated_against_owned_values_after_standalone_construction() {
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), vec![])
        .expect("removed actual garment coordinate");
    definition.structure = DesignStructure::new(structure).expect("actual replacement structure");
    assert!(matches!(DesignInputs::new(definition),
        Err(DesignInputsError::Sources(issue)) if matches!(issue.as_ref(),DesignMetadataError::Host(
            DesignSourcesError::SizeSets(issue)) if matches!(issue.as_ref(),SizeSetInventoryError::InvalidSet {set,..} if set.id==id(1000)))));
}
