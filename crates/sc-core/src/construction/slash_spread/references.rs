//! Declared-source eligibility and later explicit reference disposition;no geometric resolution.
mod query;
mod registrations;
pub use registrations::{
    BoundSlashSpreadRegistrationPlan, SlashSpreadRegisteredSourceRequest,
    SlashSpreadRegistrationDisposition, SlashSpreadRegistrationIntent, SlashSpreadRegistrationPlan,
    SlashSpreadRegistrationPlanError, SlashSpreadRegistrationPlanLimits,
    SlashSpreadRegistrationPlanRefusal,
};
mod sources;
pub use query::{
    SlashSpreadBridgeEndpoint, SlashSpreadReferenceMatch, SlashSpreadReferenceMatchKind,
    SlashSpreadReferenceOrigin, SlashSpreadReferenceQuery, SlashSpreadReferenceQueryError,
    SlashSpreadReferenceQueryLimits, SlashSpreadReferenceQueryRefusal,
};
pub use sources::{
    BoundSlashSpreadSourcePlan, SlashSpreadGlobalSourceIntent, SlashSpreadSourceDisposition,
    SlashSpreadSourcePlan, SlashSpreadSourcePlanError, SlashSpreadSourcePlanLimits,
    SlashSpreadSourcePlanRefusal, SlashSpreadSourceRequest,
};
