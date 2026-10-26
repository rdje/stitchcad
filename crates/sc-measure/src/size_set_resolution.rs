//! Selected profile/version pins and immutable SizeSet resolution-origin metadata, not execution.
mod origin;
mod profile;
pub use origin::{
    SizeSetResolutionOrigin, SizeSetResolutionOriginDefinition, SizeSetResolutionOriginError,
};
pub use profile::{
    SizeSetProfilePin, SizeSetProfilePinDefinition, SizeSetProfilePinError, SizeSetProfilePinField,
};
