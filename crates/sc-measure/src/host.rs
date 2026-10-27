//! Actual measurement/Ease implementation of the readonly core metadata port.
use crate::{MeasurementError, MeasurementInputs, MeasurementInputsError};
use core::fmt;
use sc_core::{
    design::{DesignHostEntity, DesignHostEntityKind, DesignMetadataHost},
    ontology::EntityId,
    value::{CanonicalInputs, LengthDeclaration},
};

/// Original current inventory/context/binding refusal through the core metadata port.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeasurementHostError {
    /// Whole current measurement/Ease inventory refusal.
    Inputs(Box<MeasurementInputsError>),
    /// No actual owned measurement metadata has this identity.
    MissingMeasurement {
        /// Exact requested identity.
        measurement: EntityId,
    },
    /// Actual borrowed target context refused construction.
    RecordContext(Box<MeasurementError>),
    /// Actual measurement's explicit canonical binding refused.
    Measurement {
        /// Metadata carrying the binding.
        measurement: EntityId,
        /// Original typed cause.
        issue: Box<MeasurementError>,
    },
}
impl fmt::Display for MeasurementHostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inputs(issue) => write!(f, "measurement host inputs: {issue}"),
            Self::MissingMeasurement { measurement } => {
                write!(f, "measurement host has no measurement {measurement}")
            }
            Self::RecordContext(issue) => write!(f, "measurement host context: {issue}"),
            Self::Measurement { measurement, issue } => {
                write!(f, "measurement host binding {measurement}: {issue}")
            }
        }
    }
}
impl std::error::Error for MeasurementHostError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::MissingMeasurement { .. } => None,
            Self::RecordContext(issue) | Self::Measurement { issue, .. } => Some(issue.as_ref()),
        }
    }
}

impl DesignMetadataHost for MeasurementInputs {
    type Error = MeasurementHostError;

    fn entities(&self) -> Vec<DesignHostEntity> {
        let records = self.measurements().definition();
        let inputs = self.definition();
        let mut entities = Vec::new();
        for (index, record) in records.landmarks.iter().enumerate() {
            entities.push(DesignHostEntity {
                id: record.id(),
                kind: DesignHostEntityKind::Landmark,
                index,
            });
        }
        for (index, record) in records.procedures.iter().enumerate() {
            entities.push(DesignHostEntity {
                id: record.id(),
                kind: DesignHostEntityKind::Procedure,
                index,
            });
        }
        for (index, record) in records.measurements.iter().enumerate() {
            entities.push(DesignHostEntity {
                id: record.id(),
                kind: DesignHostEntityKind::Measurement,
                index,
            });
        }
        for (index, record) in records.tables.iter().enumerate() {
            entities.push(DesignHostEntity {
                id: record.id(),
                kind: DesignHostEntityKind::MeasurementTable,
                index,
            });
        }
        for (index, record) in inputs.eases.iter().enumerate() {
            entities.push(DesignHostEntity {
                id: record.id(),
                kind: DesignHostEntityKind::Ease,
                index,
            });
        }
        for (index, record) in inputs.ease_sets.iter().enumerate() {
            entities.push(DesignHostEntity {
                id: record.id(),
                kind: DesignHostEntityKind::EaseSet,
                index,
            });
        }
        entities
    }

    fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), Self::Error> {
        MeasurementInputs::validate_current(self, canonical)
            .map_err(|issue| MeasurementHostError::Inputs(Box::new(issue)))
    }

    fn measurement_declaration<'a>(
        &'a self,
        measurement: EntityId,
        canonical: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, Self::Error> {
        let record = self
            .measurements()
            .measurement(measurement)
            .ok_or(MeasurementHostError::MissingMeasurement { measurement })?;
        let context = self
            .measurements()
            .record_context(canonical)
            .map_err(|issue| MeasurementHostError::RecordContext(Box::new(issue)))?;
        record
            .declaration(&context)
            .map_err(|issue| MeasurementHostError::Measurement {
                measurement,
                issue: Box::new(issue),
            })
    }
}
