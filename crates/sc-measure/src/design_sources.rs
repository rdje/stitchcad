//! Actual immutable measurement/external-set host; core owns the interface and canonical values.
use crate::{
    GarmentSizeSetPosition, MeasurementHostError, MeasurementInputs, MtmSizeSetPosition,
    SizeSetInventory, SizeSetInventoryError, SizeSetMetadataPosition, SizeSetPosition,
};
use core::fmt;
use sc_core::{
    design::{
        DesignHostEntity, DesignHostEntityKind as Kind, DesignMetadataHost, DesignSizeSetHost,
        DesignSizeSetReference,
    },
    ontology::EntityId,
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
}
impl fmt::Display for DesignSourcesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SizeSets(issue) => write!(f, "Design source sets: {issue}"),
            Self::Measurement(issue) => write!(f, "Design source measurement: {issue}"),
        }
    }
}
impl std::error::Error for DesignSourcesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SizeSets(issue) => Some(issue.as_ref()),
            Self::Measurement(issue) => Some(issue.as_ref()),
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
