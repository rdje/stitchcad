#![doc = include_str!("../../../docs/book/src/spec/factory-profiles.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod exact;
mod registry;
mod schema;

pub use exact::{decimal_text, parse_decimal};
pub use registry::{Kind, ParameterSpec, ACCEPTS, NOTCH_CODES, PARAMETERS, PEN_FUNCTIONS};
pub use schema::{
    check_succession, ContentVersion, Effect, Layer, NotchSpec, ParamValue, Parameter, Profile,
    Refusal, Resolve, Stage, State, TargetSystem, SCHEMA_VERSION,
};
