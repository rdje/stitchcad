//! Referenced measurement metadata, with body measurements and garment POMs kept distinct.
//!
//! G1-SLICE.4a.2b implements immutable metadata and typed current-reference checks. Value, state
//! and source are borrowed from canonical sc-core length declarations. Caller-authored landmarks
//! and documented procedures do not certify source truth, physical repeatability or release approval.
//! MeasurementInventory owns exact metadata/table vectors; MeasurementInputs composes actual Ease
//! mappings/sets and reuses complete current checks over singly owned core values.
//! MeasurementTable and individual Ease mappings pin stable bindings over borrowed current records.
//! Ease sets validate current per-POM mappings/table membership. Size membership is implemented;
//! garment chart observations borrow current member/POM inputs. Chart collections validate exact
//! Design/member coverage. MTM charts retain custom single-member body/Ease inputs. Individual size
//! axes have explicit ordered values over borrowed canonical inputs. Complete member points require
//! at least one authored axis. Individual signed breaks bind explicit axes/POMs/adjacent pairs;
//! explicit required dimensions and current adjacent coverage are implemented. SizeSet composition
//! and immutable profile transformation intent preserve original sources. Garment inputs compose current
//! snapshots with typed structural path requirements. MTM views preserve canonical body/Ease inputs
//! and explicit singleton grading refusal; numeric execution and persistence remain separate.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod resource;
pub use resource::{
    charge_garment_size_set_chart_resources, charge_measurement_input_resources,
    charge_mtm_size_set_chart_resources, charge_size_set_inventory_resources,
    charge_size_set_metadata_resources,
};

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

mod inventory;
pub use inventory::{
    MeasurementInventory, MeasurementInventoryDefinition, MeasurementInventoryError,
    MeasurementInventoryPosition,
};

mod inputs;
pub use inputs::{
    MeasurementInputs, MeasurementInputsDefinition, MeasurementInputsError,
    MeasurementInputsPosition,
};

mod host;
pub use host::MeasurementHostError;

mod size_set_metadata;
pub use size_set_metadata::{
    SizeSetMetadata, SizeSetMetadataDefinition, SizeSetMetadataError, SizeSetMetadataPosition,
};

mod garment_size_set;
pub use garment_size_set::{
    GarmentSizeSet, GarmentSizeSetDefinition, GarmentSizeSetError, GarmentSizeSetPosition,
};

mod mtm_size_set;
pub use mtm_size_set::{MtmSizeSet, MtmSizeSetDefinition, MtmSizeSetError, MtmSizeSetPosition};

mod size_set_owned;
pub use size_set_owned::{SizeSet, SizeSetError, SizeSetPosition};
mod size_set_inventory;
pub use size_set_inventory::{SizeSetInventory, SizeSetInventoryError, SizeSetInventoryPosition};

mod design_sources;
pub use design_sources::{DesignSources, DesignSourcesDefinition, DesignSourcesError};

mod ease;
pub use ease::{CompressionPermission, Ease, EaseDefinition, EaseError, EaseSide, FitIntent};

mod ease_set;
pub use ease_set::{EaseBinding, EaseSet, EaseSetContext, EaseSetDefinition, EaseSetError};

mod size_membership;
pub use size_membership::{
    SizeFormulaContext, SizeFormulaContextError, SizeLabel, SizeMember, SizeMembership,
    SizeMembershipDefinition, SizeMembershipError, SizeSetReference, SizeSystem,
};

mod size_axis;
pub use size_axis::{
    SizeAxis, SizeAxisDefinition, SizeAxisError, SizeAxisInputRole, SizeAxisKind,
    SizeAxisSourceContext, SizeAxisValueContent, SizeAxisValueDefinition, SizeAxisValueSource,
    SizeAxisValueView,
};

mod size_axes;
pub use size_axes::{
    SizeAxes, SizeAxesDefinition, SizeAxesError, SizeAxisCoordinate, SizeMemberPoint,
    SizeMemberPointView,
};

mod size_break;
pub use size_break::{SizeBreak, SizeBreakContext, SizeBreakDefinition, SizeBreakError};

mod size_breaks;
pub use size_breaks::{
    SizeBreakCell, SizeBreakDimension, SizeBreaks, SizeBreaksDefinition, SizeBreaksError,
};

mod size_set;
pub use size_set::{
    GarmentSizeSetInputs, MtmSizeSetInputs, SizeInstantiationPath, SizeSetInputError,
    SizeSetInputs, SizeSetSnapshot,
};

mod size_set_resolution;
pub use size_set_resolution::{
    SizeBreakScaling, SizeBreakScalingInputs, SizeLabelMapping, SizeSetChartReplacement,
    SizeSetChartReplacementContext, SizeSetProfilePin, SizeSetProfilePinDefinition,
    SizeSetProfilePinError, SizeSetProfilePinField, SizeSetReplacementCharts,
    SizeSetResolutionIntent, SizeSetResolutionIntentDefinition, SizeSetResolutionIntentError,
    SizeSetResolutionOrigin, SizeSetResolutionOriginDefinition, SizeSetResolutionOriginError,
    SizeSetTransformation, SizeSetTransformationDefinition, SizeSetTransformationError,
    SizeSetTransformationKind, SizeSetTransformationReadError, SizeSetTransformationState,
    SizeSetTransformationValue,
};

mod size_chart;
pub use size_chart::{
    SizeChartContext, SizeChartError, SizeChartObservation, SizeChartObservationDefinition,
    SizeChartRole,
};

mod size_chart_collection;
pub use size_chart_collection::{
    SizeChart, SizeChartBinding, SizeChartCollectionContext, SizeChartCollectionError,
    SizeChartDefinition,
};

mod mtm_chart;
pub use mtm_chart::{MtmChart, MtmChartContext, MtmChartDefinition, MtmChartError};
