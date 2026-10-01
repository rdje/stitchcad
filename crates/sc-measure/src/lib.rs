//! Referenced measurement metadata, with body measurements and garment POMs kept distinct.
//!
//! G1-SLICE.4a.2b implements immutable metadata and typed current-reference checks. Value, state
//! and source are borrowed from canonical sc-core length declarations. Caller-authored landmarks
//! and documented procedures do not certify source truth, physical repeatability or release approval.
//! MeasurementTable pins stable bindings over borrowed current records. Ease and SizeSet follow
//! in their separate owned G1 slices.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod measurement;
pub use measurement::{
    Landmark, LandmarkDefinition, Measurement, MeasurementContext, MeasurementDefinition,
    MeasurementError, MeasurementKind, MeasurementProcedure, MeasurementProcedureDefinition,
};

mod table;
pub use table::{
    MeasurementBinding, MeasurementTable, MeasurementTableContext, MeasurementTableDefinition,
    MeasurementTableError,
};
