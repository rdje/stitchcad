//! Composite current SizeSet input views; path checks are structural, not numeric execution or approval.
mod error;
mod garment;
mod mtm;
mod shared;
pub use error::SizeSetInputError;
pub use garment::GarmentSizeSetInputs;
pub use mtm::MtmSizeSetInputs;

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

/// Explicit family of already validated current SizeSet input views, without guessing from labels or count.
/// Common access/path checks retain the original borrowed records and original family-specific errors.
/// This is a Rust input interface, not a persisted or transport representation.
#[derive(Clone, Copy, Debug)]
pub enum SizeSetInputs<'a> {
    /// Authored garment-chart inputs, including a possible custom one-member range.
    Garment(GarmentSizeSetInputs<'a>),
    /// Authored custom singleton body/Ease inputs with explicit grading refusal.
    Mtm(MtmSizeSetInputs<'a>),
}
impl<'a> From<GarmentSizeSetInputs<'a>> for SizeSetInputs<'a> {
    fn from(value: GarmentSizeSetInputs<'a>) -> Self {
        Self::Garment(value)
    }
}
impl<'a> From<MtmSizeSetInputs<'a>> for SizeSetInputs<'a> {
    fn from(value: MtmSizeSetInputs<'a>) -> Self {
        Self::Mtm(value)
    }
}
impl<'a> SizeSetInputs<'a> {
    /// Exact set/revision from the original current membership.
    #[must_use]
    pub fn reference(self) -> crate::SizeSetReference {
        match self {
            Self::Garment(value) => value.reference(),
            Self::Mtm(value) => value.reference(),
        }
    }
    /// Original canonical membership, without label/base/order reconstruction.
    #[must_use]
    pub const fn membership(self) -> &'a crate::SizeMembership {
        match self {
            Self::Garment(value) => value.membership(),
            Self::Mtm(value) => value.membership(),
        }
    }
    /// Original authored axes/member coordinates; every family requires at least one axis.
    #[must_use]
    pub const fn axes(self) -> &'a crate::SizeAxes {
        match self {
            Self::Garment(value) => value.axes(),
            Self::Mtm(value) => value.axes(),
        }
    }
    /// Original ordered required/available break records, with no missing-cell defaults.
    #[must_use]
    pub const fn breaks(self) -> &'a crate::SizeBreaks {
        match self {
            Self::Garment(value) => value.breaks(),
            Self::Mtm(value) => value.breaks(),
        }
    }
    /// Dispatch the original current metadata/snapshot/identity validation for the explicitly selected family.
    /// # Errors
    /// Returns the original chart/break/snapshot error without retagging the chart family.
    pub fn validate_current(self) -> Result<(), SizeSetInputError> {
        match self {
            Self::Garment(value) => value.validate_current(),
            Self::Mtm(value) => value.validate_current(),
        }
    }
    pub(crate) fn contains_entity(self, identity: sc_core::ontology::EntityId) -> bool {
        match self {
            Self::Garment(value) => value.contains_entity(identity),
            Self::Mtm(value) => value.contains_entity(identity),
        }
    }

    /// Require the selected family's original structural inputs for the explicit path.
    /// Numeric availability, actual instantiation, global currentness and approval remain separate.
    /// # Errors
    /// Preserves exact missing cells, mapping errors and the original MTM grading refusal.
    pub fn validate_inputs_for(self, path: SizeInstantiationPath) -> Result<(), SizeSetInputError> {
        match self {
            Self::Garment(value) => value.validate_inputs_for(path),
            Self::Mtm(value) => value.validate_inputs_for(path),
        }
    }
}
