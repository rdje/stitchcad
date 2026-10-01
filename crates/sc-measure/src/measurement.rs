//! Canonical metadata references and current, unambiguous record inventories.
use core::fmt;
use sc_core::{name::MachineToken, ontology::EntityId, value::LengthDeclaration};
use sc_units::Unit;
use std::collections::{BTreeMap, BTreeSet};

/// Body measurement and garment point of measure are distinct semantic domains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeasurementKind {
    /// Anatomical measurement, with an authored landmark/procedure source.
    Body,
    /// Garment point of measure; never silently interchanged with a body quantity.
    Garment,
}

/// Authored landmark metadata, not an invented standard definition or geometric certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LandmarkDefinition {
    /// Stable landmark identity.
    pub id: EntityId,
    /// Human-readable name, separate from machine identifiers.
    pub name: String,
    /// Body or garment domain.
    pub kind: MeasurementKind,
    /// Source record; Design/G4 must establish its validity and provenance.
    pub source: EntityId,
}

/// Immutable named landmark. Global source/geometry validation remains Design/G2/G4.
/// ```compile_fail
/// fn retarget(landmark: &mut sc_measure::Landmark) {
///     landmark.definition.kind = sc_measure::MeasurementKind::Garment;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Landmark {
    definition: LandmarkDefinition,
}
impl Landmark {
    /// Validate required named content, without claiming a standard or physical source is true.
    /// # Errors
    /// Returns [`MeasurementError::EmptyName`] for a blank name.
    pub fn new(definition: LandmarkDefinition) -> Result<Self, MeasurementError> {
        require_name(definition.id, &definition.name)?;
        Ok(Self { definition })
    }
    /// Stable landmark identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Canonical authored content, borrowed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &LandmarkDefinition {
        &self.definition
    }
}

/// Canonical documented procedure; measurements reference its id instead of copying its prose.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementProcedureDefinition {
    /// Stable procedure identity.
    pub id: EntityId,
    /// Human-readable procedure name.
    pub name: String,
    /// Body or garment domain.
    pub kind: MeasurementKind,
    /// Required procedure documentation, retained exactly as authored.
    pub documentation: String,
    /// Source record; its provenance/truth and actual repeatability remain separately unproved.
    pub source: EntityId,
}

/// Immutable documented procedure; content presence is not proof of repeatability or source truth.
/// ```compile_fail
/// fn erase(procedure: &mut sc_measure::MeasurementProcedure) {
///     procedure.definition.documentation.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementProcedure {
    definition: MeasurementProcedureDefinition,
}
impl MeasurementProcedure {
    /// Validate named, documented content, without inferring physical or standards conformance.
    /// # Errors
    /// Returns [`MeasurementError`] for blank name or documentation.
    pub fn new(definition: MeasurementProcedureDefinition) -> Result<Self, MeasurementError> {
        require_name(definition.id, &definition.name)?;
        if definition.documentation.trim().is_empty() {
            return Err(MeasurementError::EmptyProcedureDocumentation(definition.id));
        }
        Ok(Self { definition })
    }
    /// Stable procedure identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Canonical documented content, borrowed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &MeasurementProcedureDefinition {
        &self.definition
    }
}

/// Authored measurement metadata. Value, source and state live only in its core declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementDefinition {
    /// Stable measurement identity, distinct from its referenced records.
    pub id: EntityId,
    /// Human-readable name; machine tokens are never derived from it.
    pub name: String,
    /// Stable machine identifier for API and recipe use.
    pub token: MachineToken,
    /// Original entered unit; changing metadata does not reconvert the canonical length.
    pub entered_unit: Unit,
    /// Body measurement or garment POM.
    pub kind: MeasurementKind,
    /// Two required landmark references; a girth-level intent may use one landmark twice.
    pub landmarks: [EntityId; 2],
    /// Required reference to a canonical documented procedure.
    pub procedure: EntityId,
    /// Sole value/state/source declaration; no duplicate cache is held in metadata.
    pub declaration: EntityId,
}

/// Immutable measurement metadata with explicit, current-reference validation.
/// ```compile_fail
/// fn change_kind(measurement: &mut sc_measure::Measurement) {
///     measurement.definition.kind = sc_measure::MeasurementKind::Body;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Measurement {
    definition: MeasurementDefinition,
}

/// Unambiguous borrowed record inventories supplied by the current caller.
/// This is not a Design-revision certificate; the command layer must supply current records.
#[derive(Debug)]
pub struct MeasurementContext<'a> {
    declarations: BTreeMap<EntityId, &'a LengthDeclaration>,
    landmarks: BTreeMap<EntityId, &'a Landmark>,
    procedures: BTreeMap<EntityId, &'a MeasurementProcedure>,
}
impl<'a> MeasurementContext<'a> {
    /// Validate unique identities across all supplied inventories before any lookup.
    /// # Errors
    /// Returns [`MeasurementError::DuplicateIdentity`] for repeated within- or cross-kind identity.
    pub fn new(
        declarations: &'a [LengthDeclaration],
        landmarks: &'a [Landmark],
        procedures: &'a [MeasurementProcedure],
    ) -> Result<Self, MeasurementError> {
        let mut seen = BTreeSet::new();
        let mut context = Self {
            declarations: BTreeMap::new(),
            landmarks: BTreeMap::new(),
            procedures: BTreeMap::new(),
        };
        for declaration in declarations {
            require_unique(declaration.id(), &mut seen)?;
            context.declarations.insert(declaration.id(), declaration);
        }
        for landmark in landmarks {
            require_unique(landmark.id(), &mut seen)?;
            context.landmarks.insert(landmark.id(), landmark);
        }
        for procedure in procedures {
            require_unique(procedure.id(), &mut seen)?;
            context.procedures.insert(procedure.id(), procedure);
        }
        Ok(context)
    }
    pub(crate) fn contains(&self, id: EntityId) -> bool {
        self.declarations.contains_key(&id)
            || self.landmarks.contains_key(&id)
            || self.procedures.contains_key(&id)
    }
}

/// Structural metadata or current-reference refusal; no physical/evidence certification is implied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeasurementError {
    /// A canonical metadata record needs a human-readable name.
    EmptyName(EntityId),
    /// A referenced procedure must carry actual nonblank documentation.
    EmptyProcedureDocumentation(EntityId),
    /// Metadata attempted to rebind a reserved formula input.
    ReservedInputToken {
        /// Authored measurement.
        measurement: EntityId,
        /// Built-in name, still legal as a reference.
        token: MachineToken,
    },
    /// An identity is repeated in context inventories or reused by the measurement itself.
    DuplicateIdentity(EntityId),
    /// The canonical declaration is absent from this context.
    MissingDeclaration(EntityId),
    /// A required landmark is absent from this context.
    MissingLandmark(EntityId),
    /// The documented procedure is absent from this context.
    MissingProcedure(EntityId),
    /// A landmark belongs to the other measurement domain.
    LandmarkKindMismatch {
        /// Measurement whose reference fails.
        measurement: EntityId,
        /// Incompatible landmark.
        landmark: EntityId,
        /// Required domain.
        expected: MeasurementKind,
        /// Current landmark domain.
        actual: MeasurementKind,
    },
    /// A procedure belongs to the other measurement domain.
    ProcedureKindMismatch {
        /// Measurement whose reference fails.
        measurement: EntityId,
        /// Incompatible procedure.
        procedure: EntityId,
        /// Required domain.
        expected: MeasurementKind,
        /// Current procedure domain.
        actual: MeasurementKind,
    },
}
impl fmt::Display for MeasurementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName(id) => write!(f, "measurement metadata {id} needs a name"),
            Self::EmptyProcedureDocumentation(id) => write!(f, "measurement procedure {id} needs documentation"),
            Self::ReservedInputToken { measurement, token } => write!(f, "measurement {measurement} cannot rebind reserved input {token}"),
            Self::DuplicateIdentity(id) => write!(f, "measurement metadata reuses identity {id}"),
            Self::MissingDeclaration(id) => write!(f, "measurement declaration {id} is missing"),
            Self::MissingLandmark(id) => write!(f, "measurement landmark {id} is missing"),
            Self::MissingProcedure(id) => write!(f, "measurement procedure {id} is missing"),
            Self::LandmarkKindMismatch { measurement, landmark, expected, actual } => write!(f, "measurement {measurement} expects {expected:?} landmark {landmark}, found {actual:?}"),
            Self::ProcedureKindMismatch { measurement, procedure, expected, actual } => write!(f, "measurement {measurement} expects {expected:?} procedure {procedure}, found {actual:?}"),
        }
    }
}
impl std::error::Error for MeasurementError {}

impl Measurement {
    /// Validate required metadata, unambiguous identity and current body/garment targets.
    /// # Errors
    /// Returns [`MeasurementError`] for blank name, reserved binding or missing/incompatible targets.
    pub fn new(
        definition: MeasurementDefinition,
        context: &MeasurementContext<'_>,
    ) -> Result<Self, MeasurementError> {
        require_name(definition.id, &definition.name)?;
        if definition.token.is_reserved_input_name() {
            return Err(MeasurementError::ReservedInputToken {
                measurement: definition.id,
                token: definition.token,
            });
        }
        let measurement = Self { definition };
        measurement.validate_current(context)?;
        Ok(measurement)
    }
    /// Stable measurement identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Authored metadata, borrowed without a second value or state representation.
    #[must_use]
    pub const fn definition(&self) -> &MeasurementDefinition {
        &self.definition
    }
    /// Check all current reference targets and identity ownership, without source-truth proof.
    /// # Errors
    /// Returns [`MeasurementError`] for identity collision or removed/incompatible targets.
    pub fn validate_current(
        &self,
        context: &MeasurementContext<'_>,
    ) -> Result<(), MeasurementError> {
        if context.contains(self.id()) {
            return Err(MeasurementError::DuplicateIdentity(self.id()));
        }
        self.declaration(context)?;
        self.landmarks(context)?;
        self.procedure(context)?;
        Ok(())
    }
    /// Borrow the current canonical value/state/source; validates only this declaration target.
    /// # Errors
    /// Returns [`MeasurementError::MissingDeclaration`] when its identity is absent.
    pub fn declaration<'a>(
        &self,
        context: &MeasurementContext<'a>,
    ) -> Result<&'a LengthDeclaration, MeasurementError> {
        context
            .declarations
            .get(&self.definition.declaration)
            .copied()
            .ok_or(MeasurementError::MissingDeclaration(
                self.definition.declaration,
            ))
    }
    /// Borrow both current landmarks, preserving authored order and repeated girth-level references.
    /// Validates only these landmark targets and their body/garment domains.
    /// # Errors
    /// Returns [`MeasurementError`] for missing or incompatible landmark metadata.
    pub fn landmarks<'a>(
        &self,
        context: &MeasurementContext<'a>,
    ) -> Result<[&'a Landmark; 2], MeasurementError> {
        let [a, b] = self.definition.landmarks;
        Ok([self.landmark(a, context)?, self.landmark(b, context)?])
    }
    fn landmark<'a>(
        &self,
        id: EntityId,
        context: &MeasurementContext<'a>,
    ) -> Result<&'a Landmark, MeasurementError> {
        let landmark = context
            .landmarks
            .get(&id)
            .copied()
            .ok_or(MeasurementError::MissingLandmark(id))?;
        if landmark.definition.kind != self.definition.kind {
            return Err(MeasurementError::LandmarkKindMismatch {
                measurement: self.id(),
                landmark: id,
                expected: self.definition.kind,
                actual: landmark.definition.kind,
            });
        }
        Ok(landmark)
    }
    /// Borrow current canonical documented content; validates only this procedure target/domain.
    /// # Errors
    /// Returns [`MeasurementError`] for missing or incompatible procedure metadata.
    pub fn procedure<'a>(
        &self,
        context: &MeasurementContext<'a>,
    ) -> Result<&'a MeasurementProcedure, MeasurementError> {
        let id = self.definition.procedure;
        let procedure = context
            .procedures
            .get(&id)
            .copied()
            .ok_or(MeasurementError::MissingProcedure(id))?;
        if procedure.definition.kind != self.definition.kind {
            return Err(MeasurementError::ProcedureKindMismatch {
                measurement: self.id(),
                procedure: id,
                expected: self.definition.kind,
                actual: procedure.definition.kind,
            });
        }
        Ok(procedure)
    }
}

fn require_name(id: EntityId, name: &str) -> Result<(), MeasurementError> {
    if name.trim().is_empty() {
        return Err(MeasurementError::EmptyName(id));
    }
    Ok(())
}
fn require_unique(id: EntityId, seen: &mut BTreeSet<EntityId>) -> Result<(), MeasurementError> {
    if !seen.insert(id) {
        return Err(MeasurementError::DuplicateIdentity(id));
    }
    Ok(())
}
