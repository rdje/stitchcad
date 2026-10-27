//! Actual higher source schemas compose with core input ownership without a dependency cycle.
#![allow(clippy::expect_used)]
#[path = "support/owned_size_set_fixture.rs"]
mod support;
use sc_core::{
    command::DesignRevision,
    design::{
        DesignHostEntity, DesignHostEntityKind, DesignInputs, DesignInputsDefinition,
        DesignInputsError, DesignInputsPosition, DesignMetadata, DesignMetadataError,
        DesignParameterDefinition, DesignParameterTarget, DesignParameters, DesignSizeSetReference,
        DesignSourceReferences, DesignStructure, DesignStructureDefinition,
        DesignStructurePosition,
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
        references: DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
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

fn references(
    table: Option<u128>,
    ease: Option<u128>,
    size: Option<(u128, u32)>,
) -> DesignSourceReferences {
    DesignSourceReferences {
        measurement_table: table.map(id),
        ease_set: ease.map(id),
        size_set: size.map(|(identity, revision)| {
            DesignSizeSetReference::new(id(identity), sc_units::Count::new(revision))
        }),
    }
}
#[test]
fn provided_and_unprovided_reference_fields_are_owned_exactly_without_inferred_selection() {
    for authored in [
        references(None, None, None),
        references(None, None, Some((40, 2))),
        references(Some(51), Some(60), Some((40, 2))),
        references(Some(51), Some(60), Some((1000, 0))),
    ] {
        let mut definition = fixture();
        definition.references = authored;
        let state = DesignInputs::new(definition)
            .expect("actual supplied references with valid joint scope");
        assert_eq!(state.references(), authored);
        assert_eq!(state.validate_current(), Ok(()));
    }
}
#[test]
fn numeric_presence_and_wrong_revision_cannot_replace_provided_source_metadata() {
    let expected = DesignSizeSetReference::new(id(40), sc_units::Count::new(3));
    let actual = DesignSizeSetReference::new(id(40), sc_units::Count::new(2));
    for (authored, issue) in [
        (
            references(Some(2000), None, None),
            DesignSourcesError::MissingMeasurementTable { table: id(2000) },
        ),
        (
            references(None, Some(2000), None),
            DesignSourcesError::MissingEaseSet { set: id(2000) },
        ),
        (
            references(None, None, Some((2000, 0))),
            DesignSourcesError::SizeSets(Box::new(SizeSetInventoryError::MissingSet {
                expected: DesignSizeSetReference::new(id(2000), sc_units::Count::new(0)),
            })),
        ),
        (
            references(None, None, Some((40, 3))),
            DesignSourcesError::SizeSets(Box::new(SizeSetInventoryError::RevisionMismatch {
                expected,
                actual,
            })),
        ),
    ] {
        let mut definition = fixture();
        assert!(definition.structure.inputs().scalar(id(2000)).is_some());
        definition.references = authored;
        assert_eq!(
            DesignInputs::new(definition).expect_err("provided metadata must actually match"),
            DesignInputsError::References(Box::new(issue))
        );
    }
}
#[test]
fn provided_table_must_match_actual_selected_ease_and_size_sources() {
    for (authored, issue) in [
        (
            references(Some(50), Some(60), None),
            DesignSourcesError::EaseTableMismatch {
                ease_set: id(60),
                selected_table: id(50),
                actual_table: id(51),
            },
        ),
        (
            references(Some(50), None, Some((40, 2))),
            DesignSourcesError::SizeTableMismatch {
                size_set: DesignSizeSetReference::new(id(40), sc_units::Count::new(2)),
                selected_table: id(50),
                actual_table: id(51),
            },
        ),
    ] {
        let mut definition = fixture();
        definition.references = authored;
        assert_eq!(
            DesignInputs::new(definition).expect_err("individually present references disagree"),
            DesignInputsError::References(Box::new(issue))
        );
    }
}
#[test]
fn mtm_chart_keeps_its_exact_ease_set_even_when_another_set_has_matching_content() {
    let mut definition = fixture();
    let canonical = definition.structure.inputs();
    let original = definition.sources.host();
    let mut inputs = original.measurements().definition().clone();
    let peer = {
        let records = inputs
            .measurements
            .record_context(canonical)
            .expect("actual records");
        let tables =
            MeasurementTableContext::new(&inputs.measurements.definition().measurements, &records)
                .expect("actual metadata");
        let context = EaseSetContext::new(
            &inputs.measurements.definition().tables,
            &inputs.eases,
            &tables,
        )
        .expect("actual mapping context");
        let mut peer = inputs
            .ease_sets
            .first()
            .expect("actual selected set")
            .definition()
            .clone();
        peer.id = id(61);
        EaseSet::new(peer, &context).expect("distinct actual set with matching entries and tables")
    };
    inputs.ease_sets.push(peer);
    let measurements =
        MeasurementInputs::new(inputs, canonical).expect("actual complete replacement metadata");
    let sources = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets: original.size_sets().clone(),
        },
        canonical,
    )
    .expect("both actual sets valid in current source scope");
    definition.sources = DesignMetadata::new(sources, canonical).expect("actual current provider");
    definition.references = references(Some(51), Some(61), Some((40, 2)));
    assert_eq!(
        DesignInputs::new(definition).expect_err("peer cannot replace the chart's saved identity"),
        DesignInputsError::References(Box::new(DesignSourcesError::MtmEaseSetMismatch {
            size_set: DesignSizeSetReference::new(id(40), sc_units::Count::new(2)),
            selected_set: id(61),
            actual_set: id(60)
        }))
    );
}
