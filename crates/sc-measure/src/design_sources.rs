//! Actual immutable measurement/external-set host; core owns the interface and canonical values.
use crate::{
    EaseError, GarmentSizeSetPosition, MeasurementHostError, MeasurementInputs, MtmSizeSetPosition,
    SizeSet, SizeSetInventory, SizeSetInventoryError, SizeSetMetadataPosition, SizeSetPosition,
};
use core::fmt;
use sc_core::{
    design::{
        DesignFormulaHost, DesignHostEntity, DesignHostEntityKind as Kind, DesignMetadataHost,
        DesignReferenceHost, DesignSizeSetHost, DesignSizeSetReference, DesignSourceReferences,
    },
    ontology::EntityId,
    recipe::{FormulaDeclaration, FormulaInputOrigin},
    value::{CanonicalInputs, LengthDeclaration},
};

/// Actual owned source objects; canonical declarations stay in the supplied core inventory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignSourcesDefinition {
    /// Exact actual measurement/Ease source owner.
    pub measurements: MeasurementInputs,
    /// Exact actual external set inventory, with explicit owned garment/MTM families.
    pub size_sets: SizeSetInventory,
}

/// Original current source/refusal through the core readonly host contracts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignSourcesError {
    /// Original complete current inventory or exact selected reference refusal.
    SizeSets(Box<SizeSetInventoryError>),
    /// Original actual measurement binding/context refusal.
    Measurement(Box<MeasurementHostError>),
    /// Original actual selected Ease amount declaration refusal.
    FormulaEase(Box<EaseError>),
    /// A provided logical table identity has no actual metadata record.
    MissingMeasurementTable {
        /// Exact provided table identity.
        table: EntityId,
    },
    /// A provided Ease-set identity has no actual metadata record.
    MissingEaseSet {
        /// Exact provided set identity.
        set: EntityId,
    },
    /// The selected Ease set names a different garment table than the provided Design table.
    EaseTableMismatch {
        /// Actual selected Ease set.
        ease_set: EntityId,
        /// Exact provided Design table.
        selected_table: EntityId,
        /// Actual table named by the Ease set.
        actual_table: EntityId,
    },
    /// The selected external SizeSet names a different logical Design table.
    SizeTableMismatch {
        /// Actual selected external reference.
        size_set: DesignSizeSetReference,
        /// Exact provided Design table.
        selected_table: EntityId,
        /// Actual table named by the set metadata.
        actual_table: EntityId,
    },
    /// The selected MTM chart names a different exact Ease set, even if content/labels match.
    MtmEaseSetMismatch {
        /// Actual selected external reference.
        size_set: DesignSizeSetReference,
        /// Exact provided Ease set.
        selected_set: EntityId,
        /// Actual Ease set named by the MTM chart.
        actual_set: EntityId,
    },
}
impl fmt::Display for DesignSourcesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SizeSets(issue) => write!(f, "Design source sets: {issue}"),
            Self::Measurement(issue) => write!(f, "Design source measurement: {issue}"),
            Self::FormulaEase(issue) => write!(f, "Design formula Ease: {issue}"),
            Self::MissingMeasurementTable { table } => {
                write!(f, "Design sources have no measurement table {table}")
            }
            Self::MissingEaseSet { set } => write!(f, "Design sources have no Ease set {set}"),
            Self::EaseTableMismatch {
                ease_set,
                selected_table,
                actual_table,
            } => {
                write!(f, "selected Ease set {ease_set} names table {actual_table}, not provided table {selected_table}")
            }
            Self::SizeTableMismatch {
                size_set,
                selected_table,
                actual_table,
            } => {
                write!(f, "selected SizeSet {size_set:?} names table {actual_table}, not provided table {selected_table}")
            }
            Self::MtmEaseSetMismatch {
                size_set,
                selected_set,
                actual_set,
            } => {
                write!(f, "selected MTM set {size_set:?} names Ease set {actual_set}, not provided set {selected_set}")
            }
        }
    }
}
impl std::error::Error for DesignSourcesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SizeSets(issue) => Some(issue.as_ref()),
            Self::Measurement(issue) => Some(issue.as_ref()),
            Self::FormulaEase(issue) => Some(issue.as_ref()),
            _ => None,
        }
    }
}

/// Private immutable actual higher source owner implementing the one-way core host contracts.
///
/// No canonical value/state or borrowed context is stored. Complete current validation includes
/// every actual source. Direct measurement binding is targeted; DesignMetadata adds whole-scope
/// current validation and canonical object membership. Profile resolution/readiness remains separate.
///
/// ```compile_fail
/// fn edit(sources: &mut sc_measure::DesignSources) { sources.definition.size_sets = sources.definition.size_sets.clone(); }
/// ```
/// ```compile_fail
/// fn edit(sources: &mut sc_measure::DesignSources) { sources.definition().measurements = sources.definition().measurements.clone(); }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct DesignSources {
    definition: DesignSourcesDefinition,
}
impl fmt::Debug for DesignSources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignSources")
            .field("size_set_count", &self.definition.size_sets.sets().len())
            .finish_non_exhaustive()
    }
}
impl DesignSources {
    /// Own exact source objects after complete current identity/reference validation.
    ///
    /// # Errors
    /// Returns original shared-input or external-source inventory refusal without partial success.
    pub fn new(
        definition: DesignSourcesDefinition,
        canonical: &CanonicalInputs,
    ) -> Result<Self, DesignSourcesError> {
        let sources = Self { definition };
        sources.validate_current(canonical)?;
        Ok(sources)
    }
    /// Exact original owned metadata definition; shared access supplies no mutator.
    #[must_use]
    pub const fn definition(&self) -> &DesignSourcesDefinition {
        &self.definition
    }
    /// Actual original measurement/Ease source owner.
    #[must_use]
    pub const fn measurements(&self) -> &MeasurementInputs {
        &self.definition.measurements
    }
    /// Actual original external set inventory, without reconstructing labels or family.
    #[must_use]
    pub const fn size_sets(&self) -> &SizeSetInventory {
        &self.definition.size_sets
    }
    /// Recheck actual complete source ownership/current semantics against explicit canonical inputs.
    ///
    /// # Errors
    /// Preserves original shared-input or actual set cause, including unselected source failures.
    pub fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), DesignSourcesError> {
        self.size_sets()
            .validate_current(canonical, self.measurements())
            .map_err(|issue| DesignSourcesError::SizeSets(Box::new(issue)))
    }
}
// Shared input positions are already catalogued by MeasurementInputs, not owned again by a set.
fn set_kind(position: SizeSetPosition) -> Option<Kind> {
    match position {
        SizeSetPosition::Garment(GarmentSizeSetPosition::Common(position))
        | SizeSetPosition::Mtm(MtmSizeSetPosition::Common(position)) => match position {
            SizeSetMetadataPosition::Inputs(_) => None,
            SizeSetMetadataPosition::Set => Some(Kind::SizeSet),
            SizeSetMetadataPosition::Member { .. } => Some(Kind::SizeMember),
            SizeSetMetadataPosition::Axis { .. } => Some(Kind::SizeAxis),
            SizeSetMetadataPosition::AxisValue { .. } => Some(Kind::SizeAxisValue),
            SizeSetMetadataPosition::Break { .. } => Some(Kind::SizeBreak),
        },
        SizeSetPosition::Garment(GarmentSizeSetPosition::Observation { .. }) => {
            Some(Kind::SizeChartObservation)
        }
        SizeSetPosition::Garment(GarmentSizeSetPosition::Chart)
        | SizeSetPosition::Mtm(MtmSizeSetPosition::Chart) => Some(Kind::SizeChart),
    }
}
impl DesignMetadataHost for DesignSources {
    type Error = DesignSourcesError;
    fn entities(&self) -> Vec<DesignHostEntity> {
        let mut entities = self.measurements().entities();
        // One counter per represented set-source category; at most seven categories, independent
        // of the number of source records. No semantic ordering of kinds or unchecked index needed.
        let mut counts: Vec<(Kind, usize)> = Vec::new();
        for set in self.size_sets().sets() {
            for (id, position) in set.identities() {
                if let Some(kind) = set_kind(position) {
                    let index = if let Some((_, count)) =
                        counts.iter_mut().find(|(category, _)| *category == kind)
                    {
                        let index = *count;
                        *count += 1;
                        index
                    } else {
                        counts.push((kind, 1));
                        0
                    };
                    entities.push(DesignHostEntity { id, kind, index });
                }
            }
        }
        entities
    }
    fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), Self::Error> {
        DesignSources::validate_current(self, canonical)
    }
    fn measurement_declaration<'a>(
        &'a self,
        measurement: EntityId,
        canonical: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, Self::Error> {
        self.measurements()
            .measurement_declaration(measurement, canonical)
            .map_err(|issue| DesignSourcesError::Measurement(Box::new(issue)))
    }
}
impl DesignSizeSetHost for DesignSources {
    fn validate_size_set(
        &self,
        reference: DesignSizeSetReference,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.size_sets()
            .validate_reference(reference, canonical, self.measurements())
            .map(|_| ())
            .map_err(|issue| DesignSourcesError::SizeSets(Box::new(issue)))
    }
}

impl DesignReferenceHost for DesignSources {
    fn validate_references(
        &self,
        references: DesignSourceReferences,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.validate_current(canonical)?;
        let table = references
            .measurement_table
            .map(|identity| {
                self.measurements()
                    .measurements()
                    .table(identity)
                    .ok_or(DesignSourcesError::MissingMeasurementTable { table: identity })
            })
            .transpose()?;
        let ease = references
            .ease_set
            .map(|identity| {
                self.measurements()
                    .ease_set(identity)
                    .ok_or(DesignSourcesError::MissingEaseSet { set: identity })
            })
            .transpose()?;
        let size = references
            .size_set
            .map(|reference| {
                self.size_sets()
                    .validate_reference(reference, canonical, self.measurements())
                    .map_err(|issue| DesignSourcesError::SizeSets(Box::new(issue)))
            })
            .transpose()?;
        if let (Some(table), Some(ease)) = (table, ease) {
            if table.id() != ease.definition().garment_table {
                return Err(DesignSourcesError::EaseTableMismatch {
                    ease_set: ease.id(),
                    selected_table: table.id(),
                    actual_table: ease.definition().garment_table,
                });
            }
        }
        if let (Some(table), Some(size)) = (table, size) {
            if table.id() != size.metadata().definition().design_table {
                return Err(DesignSourcesError::SizeTableMismatch {
                    size_set: size.reference().into(),
                    selected_table: table.id(),
                    actual_table: size.metadata().definition().design_table,
                });
            }
        }
        if let (Some(ease), Some(SizeSet::Mtm(size))) = (ease, size) {
            let actual_set = size.definition().chart.definition().ease_set.id;
            if ease.id() != actual_set {
                return Err(DesignSourcesError::MtmEaseSetMismatch {
                    size_set: size.reference().into(),
                    selected_set: ease.id(),
                    actual_set,
                });
            }
        }
        Ok(())
    }
}

impl DesignFormulaHost for DesignSources {
    fn formula_declarations<'a>(
        &'a self,
        references: DesignSourceReferences,
        canonical: &'a CanonicalInputs,
    ) -> Result<Vec<FormulaDeclaration<'a>>, Self::Error> {
        self.validate_references(references, canonical)?;
        let inventory = self.measurements().measurements();
        let mut declarations = Vec::new();
        if let Some(identity) = references.measurement_table {
            let table = inventory
                .table(identity)
                .ok_or(DesignSourcesError::MissingMeasurementTable { table: identity })?;
            for entry in &table.definition().entries {
                let declaration = self.measurement_declaration(entry.measurement, canonical)?;
                declarations.push(FormulaDeclaration::length_input(
                    &entry.token,
                    FormulaInputOrigin::Measurement,
                    entry.measurement,
                    declaration,
                ));
            }
        }
        if let Some(identity) = references.ease_set {
            let set = self
                .measurements()
                .ease_set(identity)
                .ok_or(DesignSourcesError::MissingEaseSet { set: identity })?;
            for entry in &set.definition().entries {
                // Complete current validation above has checked this exact saved binding against
                // the actual Ease metadata. Borrow directly from the canonical owner so no
                // temporary table/mapping context can become part of the returned lifetime.
                let declaration = canonical.length(entry.declaration).ok_or_else(|| {
                    DesignSourcesError::FormulaEase(Box::new(EaseError::MissingDeclaration {
                        ease: entry.ease,
                        declaration: entry.declaration,
                    }))
                })?;
                declarations.push(FormulaDeclaration::length_input(
                    &entry.token,
                    FormulaInputOrigin::Ease,
                    entry.ease,
                    declaration,
                ));
            }
        }
        Ok(declarations)
    }
}
