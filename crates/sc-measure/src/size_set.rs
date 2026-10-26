//! Composite current SizeSet input views; path checks are structural, not numeric execution or approval.
mod error;
mod garment;
pub use error::SizeSetInputError;
pub use garment::GarmentSizeSetInputs;

/// Explicit instantiation path whose current authored inputs are requested.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeInstantiationPath {
    /// Regenerate from the current member chart or body/Ease inputs.
    Regeneration,
    /// Grade from an actual base instance/rules and declared adjacent breaks.
    GradeRules,
}

/// Actual borrowed snapshot that must be shared across separately validated input views.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeSetSnapshot {
    /// One actual canonical membership record, including labels/order/base.
    Membership,
    /// One actual measurement metadata/canonical-record context.
    Measurements,
    /// One actual logical Design measurement table.
    DesignTable,
}
