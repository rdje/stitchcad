//! Owned common SizeSet metadata; contexts are rebuilt from explicit current canonical sources.
use crate::{
    MeasurementError, MeasurementInputs, MeasurementInputsError, MeasurementInputsPosition,
    MeasurementTableContext, MeasurementTableError, SizeAxes, SizeAxisError, SizeAxisSourceContext,
    SizeBreakContext, SizeBreakError, SizeBreaks, SizeBreaksError, SizeMembership,
    SizeSetReference,
};
use core::fmt;
use sc_core::{ontology::EntityId, value::CanonicalInputs};
use std::collections::BTreeMap;

/// Exact owned common source objects; charts and canonical scalar values live separately.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetMetadataDefinition {
    /// Actual canonical membership, with explicit labels/order/base/revision.
    pub membership: SizeMembership,
    /// Actual authored axes and complete member points.
    pub axes: SizeAxes,
    /// Actual required dimensions and available signed break declarations; drafts may be partial.
    pub breaks: SizeBreaks,
    /// Exact referenced logical Design measurement table in the supplied measurement inventory.
    pub design_table: EntityId,
}

/// Original ownership position in the complete inputs/common-SizeSet identity census.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeSetMetadataPosition {
    /// Inherited canonical/measurement/Ease ownership position.
    Inputs(MeasurementInputsPosition),
    /// The canonical SizeSet identity on membership.
    Set,
    /// Actual authored member.
    Member {
        /// Zero-based member position.
        index: usize,
    },
    /// Actual authored axis.
    Axis {
        /// Zero-based axis position.
        index: usize,
    },
    /// Actual authored coordinate value.
    AxisValue {
        /// Zero-based owning axis position.
        axis: usize,
        /// Zero-based value position within that axis.
        index: usize,
    },
    /// Actual available break declaration.
    Break {
        /// Zero-based authored break position.
        index: usize,
    },
}

/// Original current source refusal, without cached success or synthesized records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetMetadataError {
    /// Two owned records claim one identity; references are not additional owners.
    DuplicateIdentity {
        /// Repeated identity.
        identity: EntityId,
        /// First actual source position.
        first: SizeSetMetadataPosition,
        /// First repeated source position.
        duplicate: SizeSetMetadataPosition,
    },
    /// Actual measurement/Ease inventory failed its whole current check.
    Inputs(Box<MeasurementInputsError>),
    /// The explicitly referenced Design table is absent, even if another table has equal content.
    MissingDesignTable {
        /// Exact missing table identity.
        table: EntityId,
    },
    /// Original canonical measurement target-context refusal.
    RecordContext(Box<MeasurementError>),
    /// Original measurement binding-context refusal.
    TableContext(Box<MeasurementTableError>),
    /// Original axis-source identity refusal.
    Sources(Box<SizeAxisError>),
    /// Original current axes/table/canonical consistency refusal.
    BreakContext(Box<SizeBreakError>),
    /// Original current required/available break metadata refusal, without requiring coverage.
    Breaks(Box<SizeBreaksError>),
}
impl fmt::Display for SizeSetMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "SizeSet metadata identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::Inputs(issue) => write!(f, "SizeSet metadata inputs: {issue}"),
            Self::MissingDesignTable { table } => write!(f, "SizeSet metadata has no Design table {table}"),
            Self::RecordContext(issue) => write!(f, "SizeSet metadata record context: {issue}"),
            Self::TableContext(issue) => write!(f, "SizeSet metadata table context: {issue}"),
            Self::Sources(issue) => write!(f, "SizeSet metadata sources: {issue}"),
            Self::BreakContext(issue) => write!(f, "SizeSet metadata break context: {issue}"),
            Self::Breaks(issue) => write!(f, "SizeSet metadata breaks: {issue}"),
        }
    }
}
impl std::error::Error for SizeSetMetadataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } | Self::MissingDesignTable { .. } => None,
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::RecordContext(issue) => Some(issue.as_ref()),
            Self::TableContext(issue) => Some(issue.as_ref()),
            Self::Sources(issue) => Some(issue.as_ref()),
            Self::BreakContext(issue) => Some(issue.as_ref()),
            Self::Breaks(issue) => Some(issue.as_ref()),
        }
    }
}

/// Private common metadata owner; a complete chart-family SizeSet remains a separate composition.
///
/// No canonical declaration or borrowed context is stored. Current checks always use explicit
/// actual measurement sources; Unknown/Derived values and incomplete break drafts remain data.
///
/// ```compile_fail
/// fn edit(metadata: &mut sc_measure::SizeSetMetadata) { metadata.definition.design_table = sc_core::ontology::EntityId::from_bits(1); }
/// ```
/// ```compile_fail
/// fn edit(metadata: &mut sc_measure::SizeSetMetadata) { metadata.definition().breaks = metadata.definition().breaks.clone(); }
/// ```
/// ```compile_fail
/// fn escape(metadata: &sc_measure::SizeSetMetadata, canonical: &sc_core::value::CanonicalInputs,
///     inputs: &sc_measure::MeasurementInputs) {
///     let _context = metadata.with_context(canonical, inputs, |context| context);
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct SizeSetMetadata {
    definition: SizeSetMetadataDefinition,
}
impl fmt::Debug for SizeSetMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SizeSetMetadata")
            .field(
                "member_count",
                &self.definition.membership.definition().members.len(),
            )
            .field("axis_count", &self.definition.axes.definition().axes.len())
            .field(
                "break_count",
                &self.definition.breaks.definition().breaks.len(),
            )
            .finish_non_exhaustive()
    }
}
impl SizeSetMetadata {
    pub(crate) fn resource_definition(&self) -> &SizeSetMetadataDefinition {
        let Self { definition } = self;
        definition
    }
    /// Own exact common metadata after whole identity/current-source validation, allowing drafts.
    ///
    /// # Errors
    /// Returns the same original current refusal as validate_current, without partially owning data.
    pub fn new(
        definition: SizeSetMetadataDefinition,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<Self, SizeSetMetadataError> {
        let metadata = Self { definition };
        metadata.validate_current(canonical, inputs)?;
        Ok(metadata)
    }
    /// Actual owned common metadata; shared access supplies no mutator.
    #[must_use]
    pub const fn definition(&self) -> &SizeSetMetadataDefinition {
        &self.definition
    }
    /// Exact external object identity/revision from actual membership.
    #[must_use]
    pub const fn reference(&self) -> SizeSetReference {
        self.definition.membership.reference()
    }

    /// Actual common owned identities: set, members, each axis and its values, then breaks.
    /// References to these objects and canonical inputs are uses, never additional declarations.
    #[must_use]
    pub fn identities(&self) -> Vec<(EntityId, SizeSetMetadataPosition)> {
        let mut positions = vec![(self.reference().id, SizeSetMetadataPosition::Set)];
        positions.extend(
            self.definition
                .membership
                .definition()
                .members
                .iter()
                .enumerate()
                .map(|(index, member)| (member.id, SizeSetMetadataPosition::Member { index })),
        );
        for (index, axis) in self.definition.axes.definition().axes.iter().enumerate() {
            positions.push((
                axis.definition().id,
                SizeSetMetadataPosition::Axis { index },
            ));
            positions.extend(
                axis.definition()
                    .values
                    .iter()
                    .enumerate()
                    .map(|(value, record)| {
                        (
                            record.id,
                            SizeSetMetadataPosition::AxisValue {
                                axis: index,
                                index: value,
                            },
                        )
                    }),
            );
        }
        positions.extend(
            self.definition
                .breaks
                .definition()
                .breaks
                .iter()
                .enumerate()
                .map(|(index, record)| {
                    (
                        record.definition().id,
                        SizeSetMetadataPosition::Break { index },
                    )
                }),
        );
        positions
    }

    /// Recheck whole actual source identities, metadata and break bindings; coverage is separate.
    ///
    /// # Errors
    /// Returns the original current-source cause as described by with_context.
    pub fn validate_current(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<(), SizeSetMetadataError> {
        self.with_context(canonical, inputs, |_| ())
    }

    /// Validate the whole actual scope, then visit a temporary context that cannot escape.
    ///
    /// Census order is all canonical/measurement/Ease positions, then identities(). Current
    /// validators run before the callback. No complete chart/grade coverage or numeric evaluation
    /// is implied. Callback results may contain owned data, never a borrow of these local contexts.
    ///
    /// # Errors
    /// Returns first identity collision, current inputs, missing exact table or original context/
    /// break refusal. The callback is not invoked on failure, and its result is not interpreted.
    pub fn with_context<R>(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
        visit: impl for<'context> FnOnce(&SizeBreakContext<'context>) -> R,
    ) -> Result<R, SizeSetMetadataError> {
        let mut seen = BTreeMap::new();
        for (identity, position) in inputs
            .identity_positions(canonical)
            .map(|(id, position)| (id, SizeSetMetadataPosition::Inputs(position)))
            .chain(self.identities())
        {
            if let Some(first) = seen.insert(identity, position) {
                return Err(SizeSetMetadataError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        inputs
            .validate_current(canonical)
            .map_err(|issue| SizeSetMetadataError::Inputs(Box::new(issue)))?;
        let inventory = inputs.measurements();
        let table = inventory.table(self.definition.design_table).ok_or(
            SizeSetMetadataError::MissingDesignTable {
                table: self.definition.design_table,
            },
        )?;
        let records = inventory
            .record_context(canonical)
            .map_err(|issue| SizeSetMetadataError::RecordContext(Box::new(issue)))?;
        let measurements =
            MeasurementTableContext::new(&inventory.definition().measurements, &records)
                .map_err(|issue| SizeSetMetadataError::TableContext(Box::new(issue)))?;
        let lengths = canonical.lengths().iter().collect::<Vec<_>>();
        let scalars = canonical.scalars().iter().collect::<Vec<_>>();
        let sources = SizeAxisSourceContext::new(&lengths, &scalars)
            .map_err(|issue| SizeSetMetadataError::Sources(Box::new(issue)))?;
        let context = SizeBreakContext::new(
            &self.definition.membership,
            &self.definition.axes,
            &sources,
            table,
            &measurements,
        )
        .map_err(|issue| SizeSetMetadataError::BreakContext(Box::new(issue)))?;
        self.definition
            .breaks
            .validate_current(&context)
            .map_err(|issue| SizeSetMetadataError::Breaks(Box::new(issue)))?;
        Ok(visit(&context))
    }
}
