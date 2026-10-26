//! Selected profile/version pins and immutable SizeSet resolution-origin metadata, not execution.
mod origin;
mod profile;
pub use origin::{
    SizeSetResolutionOrigin, SizeSetResolutionOriginDefinition, SizeSetResolutionOriginError,
};
pub use profile::{
    SizeSetProfilePin, SizeSetProfilePinDefinition, SizeSetProfilePinError, SizeSetProfilePinField,
};
mod transformation;
pub use transformation::{
    SizeLabelMapping, SizeSetTransformation, SizeSetTransformationDefinition,
    SizeSetTransformationError, SizeSetTransformationKind, SizeSetTransformationReadError,
    SizeSetTransformationState, SizeSetTransformationValue,
};
mod break_scaling;
pub use break_scaling::{SizeBreakScaling, SizeBreakScalingInputs};
mod chart_replacement;
pub use chart_replacement::{
    SizeSetChartReplacement, SizeSetChartReplacementContext, SizeSetReplacementCharts,
};
