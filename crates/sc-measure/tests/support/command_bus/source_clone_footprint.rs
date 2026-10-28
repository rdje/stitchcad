//! D293 scoped diagnostic: honest real source/cache charging, semantic Clone equality, no fake totals.
use super::group_commit::{capacity, group, material, mutation};
use super::resource_admission::control;
use super::*;
use sc_core::{
    command::{CommandControlError as Control, CommandOperationLimits},
    design::{
        charge_design_garment_resources, DesignGarmentDefinition, DesignHostEntity, DesignInputs,
        DesignInputsDefinition, DesignMetadata, DesignMetadataHost, DesignReferenceHost,
        DesignResourceLimitError, DesignResourceLocation, DesignResourceMeter, DesignResourceScope,
        DesignResourceSink, DesignResourceSource, DesignSewing, DesignSewingDefinition,
        DesignSizeSetHost, DesignSizeSetReference, DesignSourceReferences,
    },
    value::{CanonicalInputs, LengthDeclaration},
};
use sc_measure::DesignSourcesError;
use std::num::NonZeroUsize;
#[derive(Debug)]
struct CacheHost {
    actual: DesignSources,
    cache: String,
}
impl Clone for CacheHost {
    fn clone(&self) -> Self {
        let mut cache = self.cache.clone();
        cache.push('x');
        Self {
            actual: self.actual.clone(),
            cache,
        }
    }
}
impl PartialEq for CacheHost {
    fn eq(&self, other: &Self) -> bool {
        self.actual == other.actual
    }
}
impl Eq for CacheHost {}
impl DesignMetadataHost for CacheHost {
    type Error = DesignSourcesError;
    fn entities(&self) -> Vec<DesignHostEntity> {
        self.actual.entities()
    }
    fn validate_current(&self, c: &CanonicalInputs) -> Result<(), Self::Error> {
        self.actual.validate_current(c)
    }
    fn measurement_declaration<'a>(
        &'a self,
        id: EntityId,
        c: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, Self::Error> {
        self.actual.measurement_declaration(id, c)
    }
}
impl DesignSizeSetHost for CacheHost {
    fn validate_size_set(
        &self,
        r: DesignSizeSetReference,
        c: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.actual.validate_size_set(r, c)
    }
}
impl DesignReferenceHost for CacheHost {
    fn validate_references(
        &self,
        r: DesignSourceReferences,
        c: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.actual.validate_references(r, c)
    }
}
impl DesignResourceSource for CacheHost {
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.actual.charge_resources(sink)?;
        sink.charge_text(
            &self.cache,
            DesignResourceLocation {
                scope: DesignResourceScope::SourceHost,
                owner: None,
                field: "source.clone_cache",
                index: None,
            },
        )
    }
}
fn owner() -> DesignGarment<CacheHost> {
    let original = super::notch_preview::owner();
    let inputs = original.inputs().definition();
    let host = CacheHost {
        actual: inputs.sources.host().clone(),
        cache: "x".into(),
    };
    assert_eq!(
        host.clone(),
        host,
        "semantic Eq/Clone law holds despite cache-size difference"
    );
    let sources = DesignMetadata::new(host, inputs.structure.inputs())
        .expect("actual complete source owner plus honestly charged cache");
    let inputs = DesignInputs::new(DesignInputsDefinition {
        structure: inputs.structure.clone(),
        parameters: inputs.parameters.clone(),
        recipe: inputs.recipe.clone(),
        materials: inputs.materials.clone(),
        sources,
        references: inputs.references,
    })
    .expect("actual same inputs");
    let old = original.sewing().definition();
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph: old.graph.clone(),
        landmarks: old.landmarks.clone(),
    })
    .expect("actual same sewing");
    let old = original.definition();
    let owner = DesignGarment::new(DesignGarmentDefinition {
        sewing,
        pockets: old.pockets.clone(),
        notions: old.notions.clone(),
        closures: old.closures.clone(),
        layers: old.layers.clone(),
        hems: old.hems.clone(),
        constructions: old.constructions.clone(),
    })
    .expect("actual complete model");
    owner
        .validate_current()
        .expect("actual immutable source semantics");
    owner
}
#[test]
fn honest_clone_cache_growth_reproduces_generic_publication_gap_d293() {
    let owner = owner();
    assert_eq!(owner.inputs().sources().host().cache.len(), 1);
    let design = owner.inputs().id();
    let piece = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual Piece")
        .id();
    let mut meter = DesignResourceMeter::new(model_policy());
    charge_design_garment_resources(&owner, &mut meter).expect("actual full source/cache count");
    let base = meter.finish().expect("actual initial resource demand");
    let mut bus = CommandBus::with_edit_capacity(owner, Host::new(design), vec![], capacity(2))
        .expect("actual finite no-history owner");
    bus.install_operation_limits(CommandOperationLimits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
    ))
    .expect("input policy");
    bus.install_model_limits(model_policy())
        .expect("model policy");
    let limits =
        super::retained_current::policy(base.collection_entries() + 4, base.text_bytes() + 27, 5);
    bus.install_retained_limits(limits)
        .expect("forecast cache2 plus26-byte original no-op request");
    let accepted = group(
        816000,
        vec![mutation(
            design,
            DesignRevision::initial(),
            816000,
            816001,
            material(piece, "explicit selection pending"),
        )],
    );
    bus.commit_group(&accepted)
        .expect("D293 reproduction: forecast accepts cache2, final clone grows to3");
    let next = group(
        816010,
        vec![mutation(
            design,
            DesignRevision::from_counter(1),
            816010,
            816011,
            material(piece, "explicit selection pending"),
        )],
    );
    let error = bus
        .preview_group(&next)
        .expect_err("D293 reproduction: accepted owner is already one byte over policy");
    let expected = DesignResourceLimitError::Count {
        unit: sc_core::design::DesignResourceCount::TextBytes,
        maximum: base.text_bytes() + 27,
        consumed: base.text_bytes() + 2,
        requested: 26,
        location: DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(accepted.id().entity_id()),
            field: "requests.mutations.material.reason",
            index: Some(0),
        },
    };
    assert_eq!(control(&error), Some(&Control::Retained(expected)));
    eprintln!("D293 reproduced: immutable cache1, equal candidate cache2, equal final cache3; accepted projection bytes{} but actual retained bytes{}; next current meter refuses original {}+26",base.text_bytes()+27,base.text_bytes()+28,base.text_bytes()+2);
}
