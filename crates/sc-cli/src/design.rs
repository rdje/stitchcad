//! Builds a real Design from a session's inputs and recipe, through the model's own constructors.
use sc_core::{
    command::DesignRevision,
    design::{
        DesignGarment, DesignGarmentDefinition, DesignInputs, DesignInputsDefinition,
        DesignMetadata, DesignParameterDefinition, DesignParameterTarget, DesignParameters,
        DesignSewing, DesignSewingDefinition, DesignSourceReferences, DesignStructure,
        DesignStructureDefinition,
    },
    name::MachineToken,
    ontology::{
        ClosureInventory, ConstructionInventory, CutPlan, EntityId, HemInventory, IdentityLedger,
        LayerInventory, MaterialInventory, NotionPlacementInventory, PieceInventory,
        PocketInventory, SewingGraph, SewingGraphDefinition,
    },
    recipe::{FormulaRecipeDocument, FormulaSourceLimits, FormulaSources, SemanticRecipe},
    value::{CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use sc_measure::{
    DesignSources, DesignSourcesDefinition, Landmark, LandmarkDefinition, Measurement,
    MeasurementContext, MeasurementDefinition, MeasurementInputs, MeasurementInputsDefinition,
    MeasurementInventory, MeasurementInventoryDefinition, MeasurementKind, MeasurementProcedure,
    MeasurementProcedureDefinition, MeasurementTable, MeasurementTableContext,
    MeasurementTableDefinition, SizeSetInventory,
};
use sc_units::{Length, Unit};
use std::fmt::Display;

/// One measurement a session supplies: its name in the recipe and its value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub token: String,
    pub value: Length,
}

/// Identities come from the design's own and a counter, so two runs of a session are identical.
#[derive(Clone, Copy, Debug)]
pub struct Identities {
    base: u128,
}

impl Identities {
    pub const fn new(design: EntityId) -> Self {
        Self {
            base: design.as_bits(),
        }
    }

    /// The nth identity under this design.
    pub const fn nth(self, ordinal: u128) -> EntityId {
        EntityId::from_bits(self.base.wrapping_add(ordinal))
    }

    // Each input owns a fixed block of identities.
    const fn length(self, at: u128) -> EntityId {
        self.nth(1000 + at)
    }

    const fn source(self, at: u128) -> EntityId {
        self.nth(2000 + at)
    }

    const fn evidence(self, at: u128) -> EntityId {
        self.nth(3000 + at)
    }

    pub const fn measurement(self, at: u128) -> EntityId {
        self.nth(4000 + at)
    }

    const fn parameter(self, at: u128) -> EntityId {
        self.nth(5000 + at)
    }
}

/// A length a session states, held as a known value with its entry as the evidence.
pub fn declaration(
    identities: Identities,
    at: u128,
    value: Length,
) -> Result<LengthDeclaration, String> {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: identities.length(at),
        source: identities.source(at),
        state: LengthState::Known {
            value,
            evidence: vec![identities.evidence(at)],
        },
    })
    .map_err(text)
}

fn text(error: impl Display) -> String {
    error.to_string()
}

fn token(name: &str) -> Result<MachineToken, String> {
    MachineToken::new(name).map_err(|error| format!("{name}: {error}"))
}

/// A Design with these inputs and this recipe, and no Pieces yet.
pub fn build(
    design: EntityId,
    revision: DesignRevision,
    inputs: &[Input],
    recipe: &str,
    drafting: &str,
) -> Result<DesignGarment<DesignSources>, String> {
    let identities = Identities::new(design);
    let (landmark, procedure, table) = (identities.nth(10), identities.nth(11), identities.nth(12));
    let (graph, recorded) = (identities.nth(13), identities.nth(14));

    let lengths = inputs
        .iter()
        .zip(0u128..)
        .map(|(input, at)| declaration(identities, at, input.value))
        .collect::<Result<Vec<_>, _>>()?;
    let canonical = CanonicalInputs::new(lengths, vec![]).map_err(text)?;

    let landmarks = vec![Landmark::new(LandmarkDefinition {
        id: landmark,
        name: "session measurement".into(),
        kind: MeasurementKind::Body,
        source: recorded,
    })
    .map_err(text)?];
    let procedures = vec![MeasurementProcedure::new(MeasurementProcedureDefinition {
        id: procedure,
        name: "session entry".into(),
        kind: MeasurementKind::Body,
        documentation: "The value this session states. How it was measured is not recorded.".into(),
        source: recorded,
    })
    .map_err(text)?];
    let records =
        MeasurementContext::new(canonical.lengths(), &landmarks, &procedures).map_err(text)?;
    let measurements = inputs
        .iter()
        .zip(0u128..)
        .map(|(input, at)| {
            Measurement::new(
                MeasurementDefinition {
                    id: identities.measurement(at),
                    name: input.token.clone(),
                    token: token(&input.token)?,
                    entered_unit: Unit::Millimetre,
                    kind: MeasurementKind::Body,
                    landmarks: [landmark, landmark],
                    procedure,
                    declaration: identities.length(at),
                },
                &records,
            )
            .map_err(text)
        })
        .collect::<Result<Vec<_>, String>>()?;
    let context = MeasurementTableContext::new(&measurements, &records).map_err(text)?;
    let tables = vec![MeasurementTable::new(
        MeasurementTableDefinition {
            id: table,
            name: "session measurements".into(),
            entries: measurements.iter().map(Into::into).collect(),
        },
        &context,
    )
    .map_err(text)?];
    let inventory = MeasurementInventory::new(
        MeasurementInventoryDefinition {
            landmarks,
            procedures,
            measurements,
            tables,
        },
        &canonical,
    )
    .map_err(text)?;
    let measured = MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements: inventory,
            eases: vec![],
            ease_sets: vec![],
        },
        &canonical,
    )
    .map_err(text)?;
    let size_sets = SizeSetInventory::new(vec![], &canonical, &measured).map_err(text)?;
    let host = DesignSources::new(
        DesignSourcesDefinition {
            measurements: measured,
            size_sets,
        },
        &canonical,
    )
    .map_err(text)?;
    let sources = DesignMetadata::new(host, &canonical).map_err(text)?;
    let parameters = DesignParameters::new(
        inputs
            .iter()
            .zip(0u128..)
            .map(|(input, at)| {
                Ok(DesignParameterDefinition {
                    id: identities.parameter(at),
                    token: token(&input.token)?,
                    target: DesignParameterTarget::Length(identities.length(at)),
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        &canonical,
    )
    .map_err(text)?;
    let materials = MaterialInventory::new(vec![], &canonical).map_err(text)?;
    let structure = DesignStructure::new(DesignStructureDefinition {
        id: design,
        revision,
        inputs: canonical,
        pieces: PieceInventory::new(vec![]).map_err(text)?,
        cut_plan: CutPlan::new(vec![], &[]).map_err(text)?,
        topology: IdentityLedger::new(),
    })
    .map_err(text)?;
    let inputs_owner = DesignInputs::new(DesignInputsDefinition {
        semantic_recipe: SemanticRecipe::empty(),
        references: DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
        structure,
        parameters,
        materials,
        sources,
        formula_sources: FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))
            .map_err(text)?,
        recipe: FormulaRecipeDocument::new(recipe.to_owned()).map_err(text)?,
    })
    .map_err(text)?;
    let empty_graph = SewingGraph::new(
        SewingGraphDefinition {
            id: graph,
            spans: vec![],
        },
        inputs_owner.structure().cut_plan(),
        inputs_owner.structure().pieces().pieces(),
        &[],
        inputs_owner.structure().topology(),
    )
    .map_err(text)?;
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs: inputs_owner,
        graph: empty_graph,
        landmarks: vec![],
    })
    .map_err(text)?;
    DesignGarment::new(DesignGarmentDefinition {
        sewing,
        pockets: PocketInventory::new(vec![]).map_err(text)?,
        notions: NotionPlacementInventory::new(vec![]).map_err(text)?,
        closures: ClosureInventory::new(vec![]).map_err(text)?,
        layers: LayerInventory::new(vec![], vec![], vec![]).map_err(text)?,
        hems: HemInventory::new(vec![]).map_err(text)?,
        constructions: ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])
            .map_err(text)?,
        drafting: drafting.to_owned(),
    })
    .map_err(text)
}
