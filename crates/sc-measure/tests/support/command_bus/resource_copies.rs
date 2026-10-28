//! Observe foreign source Clone independently of model reconstruction; actual model owners and APIs.
use super::group_commit::{capacity, group, material, mutation};
use super::*;
use sc_core::{
    command::{AddNotch, ChangeMeasurement, CommandHistoryDirection, CommandHistoryRequest},
    design::{
        DesignGarmentDefinition, DesignHostEntity, DesignInputs, DesignInputsDefinition,
        DesignMetadata, DesignMetadataHost, DesignReferenceHost, DesignSewing,
        DesignSewingDefinition, DesignSizeSetHost, DesignSizeSetReference, DesignSourceReferences,
    },
    value::{CanonicalInputs, LengthDeclaration, LengthState},
};
use sc_measure::DesignSourcesError;

// Instrumentation delegates every census/reference/binding/validation to the actual immutable host.
// Telemetry is observational only; semantic source equality deliberately excludes these counters.
// `copies` counts foreign H::clone calls, never core model reconstruction or allocation work.
#[derive(Debug)]
pub(super) struct CountedSources {
    actual: DesignSources,
    copies: Rc<Cell<usize>>,
    walks: Rc<Cell<usize>>,
    callbacks: Rc<Cell<usize>>,
}
impl CountedSources {
    pub(super) fn telemetry(&self) -> [Rc<Cell<usize>>; 3] {
        [
            Rc::clone(&self.copies),
            Rc::clone(&self.walks),
            Rc::clone(&self.callbacks),
        ]
    }
}
impl Clone for CountedSources {
    fn clone(&self) -> Self {
        self.copies.set(self.copies.get() + 1);
        Self {
            actual: self.actual.clone(),
            copies: Rc::clone(&self.copies),
            walks: Rc::clone(&self.walks),
            callbacks: Rc::clone(&self.callbacks),
        }
    }
}
impl PartialEq for CountedSources {
    fn eq(&self, other: &Self) -> bool {
        self.actual == other.actual
    }
}
impl Eq for CountedSources {}
impl DesignMetadataHost for CountedSources {
    type Error = DesignSourcesError;
    fn entities(&self) -> Vec<DesignHostEntity> {
        self.callbacks.set(self.callbacks.get() + 1);
        self.actual.entities()
    }
    fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), Self::Error> {
        self.callbacks.set(self.callbacks.get() + 1);
        self.actual.validate_current(canonical)
    }
    fn measurement_declaration<'a>(
        &'a self,
        measurement: EntityId,
        canonical: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, Self::Error> {
        self.callbacks.set(self.callbacks.get() + 1);
        self.actual.measurement_declaration(measurement, canonical)
    }
}
impl DesignSizeSetHost for CountedSources {
    fn validate_size_set(
        &self,
        reference: DesignSizeSetReference,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.callbacks.set(self.callbacks.get() + 1);
        self.actual.validate_size_set(reference, canonical)
    }
}
impl sc_core::design::DesignResourceSource for CountedSources {
    fn charge_resources(
        &self,
        sink: &mut dyn sc_core::design::DesignResourceSink,
    ) -> Result<(), sc_core::design::DesignResourceLimitError> {
        self.walks.set(self.walks.get() + 1);
        self.actual.charge_resources(sink)
    }
}
impl DesignReferenceHost for CountedSources {
    fn validate_references(
        &self,
        references: DesignSourceReferences,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.callbacks.set(self.callbacks.get() + 1);
        self.actual.validate_references(references, canonical)
    }
}
pub(super) fn owner(copies: Rc<Cell<usize>>) -> (DesignGarment<CountedSources>, AddNotch) {
    let original = notch_preview::owner();
    let addition = notch_preview::addition(&original);
    let inputs = original.inputs().definition();
    let sources = DesignMetadata::new(
        CountedSources {
            actual: inputs.sources.host().clone(),
            copies,
            walks: Rc::new(Cell::new(0)),
            callbacks: Rc::new(Cell::new(0)),
        },
        inputs.structure.inputs(),
    )
    .expect("actual source host under a copy observer");
    let inputs = DesignInputs::new(DesignInputsDefinition {
        structure: inputs.structure.clone(),
        parameters: inputs.parameters.clone(),
        recipe: inputs.recipe.clone(),
        materials: inputs.materials.clone(),
        sources,
        references: inputs.references,
    })
    .expect("actual original input components");
    let sewing = original.sewing().definition();
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph: sewing.graph.clone(),
        landmarks: sewing.landmarks.clone(),
    })
    .expect("actual original sewing components");
    let garment = original.definition();
    let owner = DesignGarment::new(DesignGarmentDefinition {
        sewing,
        pockets: garment.pockets.clone(),
        notions: garment.notions.clone(),
        closures: garment.closures.clone(),
        layers: garment.layers.clone(),
        hems: garment.hems.clone(),
        constructions: garment.constructions.clone(),
    })
    .expect("actual original garment components");
    (owner, addition)
}

#[test]
fn actual_source_is_shared_across_candidate_revision_and_snapshot_traversal() {
    let copies = Rc::new(Cell::new(0));
    let (owner, addition) = owner(Rc::clone(&copies));
    let snapshot = owner.clone();
    assert!(core::ptr::eq(
        owner.inputs().sources().host(),
        snapshot.inputs().sources().host()
    ));
    assert_ne!(
        owner.inputs().sources().entities().as_ptr(),
        snapshot.inputs().sources().entities().as_ptr(),
        "captured entity vectors remain separate owned copies"
    );
    let detached = owner.inputs().sources().host().clone();
    assert_eq!(
        copies.get(),
        1,
        "independent foreign Clone control proves observer is live"
    );
    drop(detached);
    copies.set(0);
    let design = owner.inputs().id();
    let base = owner.inputs().revision().clone();
    let original_address = owner.inputs().structure().pieces().pieces().as_ptr();
    let piece = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual Piece")
        .id();
    let mut replacement = owner
        .inputs()
        .sources()
        .measurement_declaration(id(1), owner.inputs().inputs())
        .expect("actual original Measurement binding")
        .definition()
        .clone();
    replacement.state = LengthState::Unknown {
        observation: id(802010),
    };
    let replacement = LengthDeclaration::new(replacement).expect("actual unknown declaration");
    let members = vec![
        mutation(
            design,
            base.clone(),
            802000,
            802001,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), replacement)),
        ),
        mutation(
            design,
            base.clone(),
            802000,
            802002,
            material(piece, "copy-bounded intent"),
        ),
        mutation(
            design,
            base.clone(),
            802000,
            802003,
            MutationPayload::AddNotch(addition),
        ),
    ];
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            Host::new(design),
            notch_preview::reserve(),
            capacity(1),
            capacity(1),
            capacity(2),
        )
        .expect("explicit finite history"),
    );
    let mut observed = Vec::new();
    for member in &members {
        copies.set(0);
        bus.preview(&CommandRequest::Mutation(member.clone()))
            .expect("actual private metadata candidate");
        observed.push(copies.get());
    }
    assert_eq!(
        observed,
        vec![0, 0, 0],
        "D293: every metadata candidate shares its original source instance"
    );
    let alias = members.last().expect("actual Notch member").clone();
    let mut original_members = members;
    original_members.push(alias);
    let request = group(802000, original_members);
    copies.set(0);
    let preview = bus
        .preview_group(&request)
        .expect("three candidates and an exact original alias");
    assert_eq!(preview.steps().len(), 4);
    assert_eq!(
        preview.steps().last().expect("alias report").replay_of(),
        Some(2)
    );
    assert_eq!(copies.get(), 0, "shared source: no foreign H::clone");
    copies.set(0);
    let accepted = bus
        .commit_group(&request)
        .expect("one group and fresh revision");
    assert_eq!(copies.get(), 0, "shared source: no foreign H::clone");
    copies.set(0);
    assert!(bus
        .commit_group(&request)
        .expect("exact accepted replay")
        .is_replay());
    assert_eq!(copies.get(), 0, "replay reads the accepted record");
    let traversal = |revision, key, direction| {
        CommandHistoryRequest::new(
            CommandMutationContext::new(
                design,
                actor(),
                revision,
                CommandIdempotencyKey::from_entity_id(id(key)),
                Some(request.id()),
            ),
            request.clone(),
            direction,
        )
        .expect("current operator and original group")
    };
    let undo = traversal(
        accepted.entry().commit().revision().clone(),
        802004,
        CommandHistoryDirection::Undo,
    );
    bus.apply_history(&undo)
        .expect("actual predecessor snapshot");
    assert_eq!(copies.get(), 0, "Undo moves the actual source host");
    let CommandResult::PieceList(restored) = bus
        .execute(&query(design, base.clone(), QueryPayload::GetPieceList))
        .expect("restored original snapshot");
    assert_eq!(restored.pieces().as_ptr(), original_address);
    let redo = traversal(base, 802005, CommandHistoryDirection::Redo);
    bus.apply_history(&redo).expect("actual accepted snapshot");
    assert_eq!(copies.get(), 0, "Redo moves the actual source host");
}
