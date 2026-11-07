//! Complete actual higher roots, preserving one cumulative sink and original component locations.
use super::{
    charge_closure_inventory_resources, charge_construction_inventory_resources,
    charge_formula_source_resources, charge_hem_inventory_resources,
    charge_layer_inventory_resources, charge_material_inventory_resources,
    charge_notion_placement_inventory_resources, charge_parameter_resources,
    charge_pocket_inventory_resources, charge_recipe_document_resources,
    charge_sewing_graph_resources, fixed_slice, fixed_value, DesignResourceLimitError,
    DesignResourceLocation, DesignResourceScope, DesignResourceSink, DesignResourceSource,
    ResourceProjection,
};
use crate::design::{
    DesignGarment, DesignGarmentDefinition, DesignInputs, DesignInputsDefinition, DesignSewing,
    DesignSewingDefinition,
};
use crate::value::LengthDeclaration;
/// Charge the actual complete input root, including the source host and separately retained census.
/// Exhaustive private views follow schema order. References are fixed Copy payloads; the captured
/// identity vector is precharged without invoking a semantic census or currentness callback.
/// Success is logical accounting under the source port's honest/exhaustive integration obligation,
/// not authorization, currentness, allocator memory/time or runtime admission.
/// # Errors
/// Returns the original first refusal, including the actual containing Design for sources.entities.
pub fn charge_design_input_resources<H: DesignResourceSource>(
    inputs: &DesignInputs<H>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_inputs_projected(inputs, ResourceProjection::Current, sink)
}
fn charge_inputs_projected<H: DesignResourceSource>(
    inputs: &DesignInputs<H>,
    projection: ResourceProjection<'_>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignInputsDefinition {
        structure,
        parameters,
        recipe,
        formula_sources,
        semantic_recipe,
        materials,
        sources,
        references,
    } = inputs.resource_definition();
    super::structure::charge_structure_projected(structure, projection, sink)?;
    charge_parameter_resources(parameters, sink)?;
    charge_recipe_document_resources(recipe, sink)?;
    charge_formula_source_resources(formula_sources, sink)?;
    super::charge_semantic_recipe_resources(semantic_recipe, sink)?;
    charge_material_inventory_resources(materials, sink)?;
    let (host, entities) = sources.resource_parts();
    host.charge_resources(sink)?;
    sink.charge_entries(
        entities.len(),
        DesignResourceLocation {
            scope: DesignResourceScope::Inputs,
            owner: Some(structure.id()),
            field: "sources.entities",
            index: None,
        },
    )?;
    fixed_slice(entities);
    fixed_value(references);
    Ok(())
}

/// Charge the actual complete sewing root: inputs, original graph, then all retained landmarks.
/// No clone, reconstruction, current validation, numeric lookup or topology replay occurs.
/// # Errors
/// Returns the original first component refusal under the same cumulative sink.
pub fn charge_design_sewing_resources<H: DesignResourceSource>(
    sewing: &DesignSewing<H>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_sewing_projected(sewing, ResourceProjection::Current, sink)
}
fn charge_sewing_projected<H: DesignResourceSource>(
    sewing: &DesignSewing<H>,
    projection: ResourceProjection<'_>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignSewingDefinition {
        inputs,
        graph,
        landmarks,
    } = sewing.resource_definition();
    charge_inputs_projected(inputs, projection, sink)?;
    charge_sewing_graph_resources(graph, sink)?;
    let appended = if let ResourceProjection::AppendedNotch(notch) = projection {
        Some(notch)
    } else {
        None
    };
    super::sewing::charge_landmarks_appended(landmarks, appended, sink)
}
/// Charge every actual implemented garment root field in schema order, without resetting the sink.
/// Shared logical references are fixed; owned snapshots and vectors count at their actual owners.
/// This is the complete represented model over an honest resource source, not a runtime gate,
/// retained command-journal budget, physical/readiness proof or allocator memory/time bound.
/// # Errors
/// Returns the original first refusal, preserving component scope, owner and authored index.
pub fn charge_design_garment_resources<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(garment, ResourceProjection::Current, sink)
}
fn charge_garment_projected<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    projection: ResourceProjection<'_>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignGarmentDefinition {
        sewing,
        pockets,
        notions,
        closures,
        layers,
        hems,
        constructions,
        drafting,
    } = garment.resource_definition();
    sink.charge_text(
        drafting,
        DesignResourceLocation {
            scope: DesignResourceScope::Garment,
            owner: None,
            field: "drafting",
            index: None,
        },
    )?;
    charge_sewing_projected(sewing, projection, sink)?;
    charge_pocket_inventory_resources(pockets, sink)?;
    charge_notion_placement_inventory_resources(notions, sink)?;
    charge_closure_inventory_resources(closures, sink)?;
    charge_layer_inventory_resources(layers, sink)?;
    charge_hem_inventory_resources(hems, sink)?;
    charge_construction_inventory_resources(constructions, sink)
}

// The caller has resolved a real Measurement binding and checked this exact canonical identity.
// This private logical projection neither installs data nor skips subsequent semantic validation.
pub(crate) fn charge_garment_length_replacement<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    replacement: &LengthDeclaration,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(garment, ResourceProjection::Length(replacement), sink)
}

// The command has checked every selected actual Piece before entering this borrowed projection.
pub(crate) fn charge_garment_material_assignment<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    pieces: &[crate::ontology::EntityId],
    assignment: &crate::ontology::MaterialAssignment,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(
        garment,
        ResourceProjection::Material { pieces, assignment },
        sink,
    )
}

// The command has peeked an actual reserved ID and validated this fixed semantic payload. No
// staged ID has been consumed and no model/registry vector has been copied or grown yet.
pub(crate) fn charge_garment_notch_append<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    notch: &crate::ontology::Notch,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(garment, ResourceProjection::AppendedNotch(notch), sink)
}

// This is the private journal's actual independent maximum-issued revision, never the restored
// model revision or an untrusted request claim. Only its next magnitude is forecast here.
pub(crate) fn charge_garment_successor_revision<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    maximum_issued: &crate::command::DesignRevision,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(
        garment,
        ResourceProjection::SuccessorRevision(maximum_issued),
        sink,
    )
}

#[cfg(test)]
mod semantic_projection_test {
    use super::*;
    use crate::{
        command::DesignRevision, design::*, name::MachineToken, ontology::*, recipe::*, value::*,
    };
    use core::num::{NonZeroU64, NonZeroUsize};
    struct EmptyHost;
    fn missing() -> std::io::Error {
        std::io::Error::other("no metadata sources")
    }
    impl DesignMetadataHost for EmptyHost {
        type Error = std::io::Error;
        fn entities(&self) -> Vec<DesignHostEntity> {
            vec![]
        }
        fn validate_current(&self, _: &CanonicalInputs) -> Result<(), Self::Error> {
            Ok(())
        }
        fn measurement_declaration<'a>(
            &'a self,
            _: EntityId,
            _: &'a CanonicalInputs,
        ) -> Result<&'a LengthDeclaration, Self::Error> {
            Err(missing())
        }
    }
    impl DesignSizeSetHost for EmptyHost {
        fn validate_size_set(
            &self,
            _: DesignSizeSetReference,
            _: &CanonicalInputs,
        ) -> Result<(), Self::Error> {
            Err(missing())
        }
    }
    impl DesignReferenceHost for EmptyHost {
        fn validate_references(
            &self,
            refs: DesignSourceReferences,
            _: &CanonicalInputs,
        ) -> Result<(), Self::Error> {
            if refs.measurement_table.is_none()
                && refs.ease_set.is_none()
                && refs.size_set.is_none()
            {
                Ok(())
            } else {
                Err(missing())
            }
        }
    }
    impl DesignResourceSource for EmptyHost {
        fn charge_resources(
            &self,
            _: &mut dyn DesignResourceSink,
        ) -> Result<(), DesignResourceLimitError> {
            Ok(())
        }
    }
    #[test]
    fn prospective_length_root_keeps_actual_semantic_fields_and_replacement_only_evidence(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = EntityId::from_bits;
        let record = |evidence| {
            LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(10),
                source: id(100),
                state: LengthState::Known {
                    value: sc_units::Length::ZERO,
                    evidence,
                },
            })
        };
        let canonical = CanonicalInputs::new(vec![record(vec![id(101)])?], vec![])?;
        let parameters = DesignParameters::new(vec![], &canonical)?;
        let materials = MaterialInventory::new(vec![], &canonical)?;
        let sources = DesignMetadata::new(EmptyHost, &canonical)?;
        let operation = ButtonholeOperation::new(
            ButtonholeOperationDefinition {
                id: id(70),
                button: NotionSize::Profile(ProfileParameterRef::new(id(900))),
                policy: id(901),
                inputs: vec![ButtonholePolicyInput {
                    role: MachineToken::new("policy_a")?,
                    target: CanonicalInputTarget::Length(id(10)),
                }],
            },
            ButtonholeOperationLimits::new(1, 8, 1),
        )?;
        let root = DesignInputs::new(DesignInputsDefinition {
            structure: DesignStructure::new(DesignStructureDefinition {
                id: id(1),
                revision: DesignRevision::from_counter(7),
                inputs: canonical,
                pieces: PieceInventory::new(vec![])?,
                cut_plan: CutPlan::new(vec![], &[])?,
                topology: IdentityLedger::new(),
            })?,
            parameters,
            materials,
            sources,
            recipe: FormulaRecipeDocument::new(String::new())?,
            formula_sources: FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))?,
            semantic_recipe: SemanticRecipe::new(
                vec![SemanticOperation::Buttonhole(operation)],
                SemanticRecipeLimits::new(1, 1),
            )?,
            references: DesignSourceReferences {
                measurement_table: None,
                ease_set: None,
                size_set: None,
            },
        })?;
        let limits = |entries| -> Result<DesignResourceLimits, Box<dyn std::error::Error>> {
            Ok(DesignResourceLimits::new(
                NonZeroUsize::new(entries).ok_or("positive entries")?,
                NonZeroUsize::new(16).ok_or("positive text")?,
                NonZeroU64::new(3).ok_or("positive bits")?,
            ))
        };
        let mut actual = DesignResourceMeter::new(limits(4)?);
        charge_design_input_resources(&root, &mut actual)?;
        let demand = actual.finish()?;
        assert_eq!(
            (
                demand.collection_entries(),
                demand.text_bytes(),
                demand.counter_bits()
            ),
            (4, 16, 3)
        );
        let replacement = record(vec![id(102), id(103)])?;
        let mut projected = DesignResourceMeter::new(limits(5)?);
        charge_inputs_projected(
            &root,
            ResourceProjection::Length(&replacement),
            &mut projected,
        )?;
        let demand = projected.finish()?;
        assert_eq!(
            (
                demand.collection_entries(),
                demand.text_bytes(),
                demand.counter_bits()
            ),
            (5, 16, 3)
        );
        let mut short = DesignResourceMeter::new(limits(4)?);
        let expected = DesignResourceLimitError::Count {
            unit: DesignResourceCount::CollectionEntries,
            maximum: 4,
            consumed: 4,
            requested: 1,
            location: DesignResourceLocation {
                scope: DesignResourceScope::Inputs,
                owner: Some(id(70)),
                field: "semantic.buttonhole.inputs",
                index: None,
            },
        };
        assert_eq!(
            charge_inputs_projected(&root, ResourceProjection::Length(&replacement), &mut short),
            Err(expected)
        );
        assert_eq!(short.finish(), Err(expected));
        assert!(
            matches!(root.inputs().length(id(10)).map(LengthDeclaration::state),Some(LengthState::Known {evidence,..}) if evidence==&[id(101)])
        );
        Ok(())
    }
}
