//! Explicit ordered axis metadata over borrowed canonical input values.
use crate::{SizeLabel, SizeSetReference};
use core::fmt;
use sc_core::{ontology::EntityId, value::ScalarInputKind};
use std::collections::BTreeSet;

mod source;
pub use source::{SizeAxisSourceContext, SizeAxisValueContent};

/// Authored coordinate kind; neither labels nor magnitudes choose it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SizeAxisKind {
    /// Explicit human designations without numeric interpretation.
    Label,
    /// Canonical scalar coordinates of exactly this declared kind.
    Scalar(ScalarInputKind),
}

/// Explicit canonical-record role, separate from the scalar's quantity kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeAxisInputRole {
    /// Dedicated canonical length declaration.
    Length,
    /// General canonical scalar declaration.
    Scalar,
}

/// Authored axis value, with no cached numeric result or inferred designation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeAxisValueSource {
    /// Exact nonblank human coordinate designation.
    Label(SizeLabel),
    /// Actual dedicated length declaration identity.
    Length(EntityId),
    /// Actual general scalar declaration identity.
    Scalar(EntityId),
}

/// Stable coordinate identity, distinct from its axis, set and canonical numeric declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeAxisValueDefinition {
    /// Stable authored coordinate identity.
    pub id: EntityId,
    /// Exact authored designation or canonical-record reference.
    pub source: SizeAxisValueSource,
}

/// Editable axis declaration; ordering is authored data, not numeric or label sorting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeAxisDefinition {
    /// Owning set and authored revision; global currentness remains a registry obligation.
    pub reference: SizeSetReference,
    /// Stable axis identity.
    pub id: EntityId,
    /// Explicit nonblank human dimension name, retaining original Unicode/case/spacing.
    pub name: String,
    /// Explicit coordinate kind.
    pub kind: SizeAxisKind,
    /// Nonempty authored order of distinct coordinate identities.
    pub values: Vec<SizeAxisValueDefinition>,
}

/// Immutable locally validated axis; complete member coordinates and minimum axis count follow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeAxis {
    definition: SizeAxisDefinition,
}

/// Actual axis/source refusal, without substituting a name, ordinal or numeric value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeAxisError {
    /// A canonical input identity was supplied twice across the two source inventories.
    DuplicateSource(EntityId),
    /// An explicit dimension name was blank.
    BlankName {
        /// Requested axis.
        axis: EntityId,
        /// Exact refused name.
        name: String,
    },
    /// An axis declared no coordinate values.
    EmptyValues(EntityId),
    /// A local metadata identity repeats, aliases the set, or aliases a canonical source record.
    DuplicateIdentity {
        /// Requested axis.
        axis: EntityId,
        /// Actual conflicting identity.
        identity: EntityId,
    },
    /// Labelled coordinate values repeat the same exact designation.
    DuplicateLabel {
        /// Requested axis.
        axis: EntityId,
        /// Actual repeated designation.
        label: SizeLabel,
    },
    /// Queried coordinate does not belong to this axis.
    MissingValue {
        /// Actual axis.
        axis: EntityId,
        /// Requested coordinate.
        value: EntityId,
    },
    /// A declared canonical source is absent from the supplied current inventory.
    MissingSource {
        /// Actual axis.
        axis: EntityId,
        /// Actual coordinate.
        value: EntityId,
        /// Missing canonical declaration.
        declaration: EntityId,
        /// Required canonical-record role.
        expected: SizeAxisInputRole,
    },
    /// The actual canonical record has the wrong declaration role, even if quantity kinds agree.
    SourceRole {
        /// Actual axis.
        axis: EntityId,
        /// Actual coordinate.
        value: EntityId,
        /// Actual declaration.
        declaration: EntityId,
        /// Declared source role.
        expected: SizeAxisInputRole,
        /// Actual source role.
        actual: SizeAxisInputRole,
    },
    /// An actual coordinate kind differs from the explicit axis kind.
    Kind {
        /// Actual axis.
        axis: EntityId,
        /// Actual coordinate.
        value: EntityId,
        /// Declared axis kind.
        wanted: SizeAxisKind,
        /// Actual current coordinate kind.
        actual: SizeAxisKind,
    },
}

impl fmt::Display for SizeAxisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size axis: {self:?}")
    }
}
impl std::error::Error for SizeAxisError {}

/// Borrowed coordinate metadata and the actual current canonical content that satisfied its kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizeAxisValueView<'a> {
    axis: EntityId,
    definition: &'a SizeAxisValueDefinition,
    content: SizeAxisValueContent<'a>,
}

impl<'a> SizeAxisValueView<'a> {
    /// Original owning axis identity.
    #[must_use]
    pub const fn axis(self) -> EntityId {
        self.axis
    }

    /// Original coordinate metadata, including its stable identity and exact source reference.
    #[must_use]
    pub const fn definition(self) -> &'a SizeAxisValueDefinition {
        self.definition
    }

    /// Actual borrowed canonical declaration or human designation; no value/state is duplicated.
    #[must_use]
    pub const fn content(self) -> SizeAxisValueContent<'a> {
        self.content
    }

    /// Actual kind established from the current borrowed content.
    #[must_use]
    pub const fn kind(self) -> SizeAxisKind {
        self.content.kind()
    }
}

impl SizeAxis {
    pub(crate) fn resource_definition(&self) -> &SizeAxisDefinition {
        let Self { definition } = self;
        definition
    }
    /// Validate explicit name/order/identities and every actual current source/kind.
    /// This constructs one axis, not a complete SizeSet or member-coordinate grid.
    /// # Errors
    /// Returns the first structural or current source/kind refusal; no missing data is inferred.
    pub fn new(
        definition: SizeAxisDefinition,
        context: &SizeAxisSourceContext<'_>,
    ) -> Result<Self, SizeAxisError> {
        validate(&definition, context)?;
        Ok(Self { definition })
    }

    /// Borrow the exact authored metadata without unchecked mutation.
    #[must_use]
    pub const fn definition(&self) -> &SizeAxisDefinition {
        &self.definition
    }

    /// Revalidate all references/kinds against supplied current canonical records.
    /// # Errors
    /// Returns actual missing/retargeted/conflicting source evidence, without global registry proof.
    pub fn validate_current(
        &self,
        context: &SizeAxisSourceContext<'_>,
    ) -> Result<(), SizeAxisError> {
        validate(&self.definition, context)
    }

    /// Borrow a coordinate after revalidating the complete axis against current source records.
    /// This does not read or evaluate an Unknown/Derived numeric source.
    /// # Errors
    /// Returns the first current-axis refusal, then MissingValue for an absent coordinate identity.
    pub fn value<'a>(
        &'a self,
        value: EntityId,
        context: &SizeAxisSourceContext<'a>,
    ) -> Result<SizeAxisValueView<'a>, SizeAxisError> {
        self.validate_current(context)?;
        let definition = self
            .definition
            .values
            .iter()
            .find(|entry| entry.id == value)
            .ok_or(SizeAxisError::MissingValue {
                axis: self.definition.id,
                value,
            })?;
        let content = context.resolve(self.definition.id, definition)?;
        Ok(SizeAxisValueView {
            axis: self.definition.id,
            definition,
            content,
        })
    }
}

fn validate(
    definition: &SizeAxisDefinition,
    context: &SizeAxisSourceContext<'_>,
) -> Result<(), SizeAxisError> {
    let axis = definition.id;
    if definition.name.trim().is_empty() {
        return Err(SizeAxisError::BlankName {
            axis,
            name: definition.name.clone(),
        });
    }
    if definition.values.is_empty() {
        return Err(SizeAxisError::EmptyValues(axis));
    }
    let mut identities = BTreeSet::new();
    for identity in [definition.reference.id, axis] {
        if !identities.insert(identity) || context.contains(identity) {
            return Err(SizeAxisError::DuplicateIdentity { axis, identity });
        }
    }
    let mut labels = BTreeSet::new();
    // Complete local identity/label inventory precedes source lookups, avoiding ambiguous records.
    for value in &definition.values {
        if !identities.insert(value.id) || context.contains(value.id) {
            return Err(SizeAxisError::DuplicateIdentity {
                axis,
                identity: value.id,
            });
        }
        if let SizeAxisValueSource::Label(label) = &value.source {
            if !labels.insert(label.as_str()) {
                return Err(SizeAxisError::DuplicateLabel {
                    axis,
                    label: label.clone(),
                });
            }
        }
    }
    for value in &definition.values {
        let actual = context.resolve(axis, value)?.kind();
        if actual != definition.kind {
            return Err(SizeAxisError::Kind {
                axis,
                value: value.id,
                wanted: definition.kind,
                actual,
            });
        }
    }
    Ok(())
}
