//! Read-only actual canonical-record inventories; no source-truth or global currentness claim.
use super::{
    SizeAxisError, SizeAxisInputRole as Role, SizeAxisKind, SizeAxisValueDefinition,
    SizeAxisValueSource,
};
use crate::SizeLabel;
use sc_core::{
    ontology::EntityId,
    value::{LengthDeclaration, ScalarInputDeclaration, ScalarInputKind},
};
use std::collections::BTreeMap;

/// Actual borrowed coordinate content, retaining the canonical source/state/provenance APIs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeAxisValueContent<'a> {
    /// Authored exact designation, with no numeric interpretation.
    Label(&'a SizeLabel),
    /// Original dedicated length record; Unknown/Derived retain their normal read refusals.
    Length(&'a LengthDeclaration),
    /// Original general scalar record, including its declared kind and complete authored state.
    Scalar(&'a ScalarInputDeclaration),
}

impl SizeAxisValueContent<'_> {
    /// Actual content kind; the original canonical record supplies scalar kind information.
    #[must_use]
    pub const fn kind(self) -> SizeAxisKind {
        match self {
            Self::Label(_) => SizeAxisKind::Label,
            Self::Length(_) => SizeAxisKind::Scalar(ScalarInputKind::Length),
            Self::Scalar(value) => SizeAxisKind::Scalar(value.kind()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Record<'a> {
    Length(&'a LengthDeclaration),
    Scalar(&'a ScalarInputDeclaration),
}
impl Record<'_> {
    const fn id(self) -> EntityId {
        match self {
            Self::Length(value) => value.id(),
            Self::Scalar(value) => value.id(),
        }
    }
    const fn role(self) -> Role {
        match self {
            Self::Length(_) => Role::Length,
            Self::Scalar(_) => Role::Scalar,
        }
    }
}

/// Unambiguous caller-supplied current canonical records, borrowed without numeric caches.
/// This is a local inventory, not a Design-revision, evidence-truth or approval certificate.
#[derive(Debug)]
pub struct SizeAxisSourceContext<'a> {
    records: BTreeMap<EntityId, Record<'a>>,
}

impl<'a> SizeAxisSourceContext<'a> {
    /// Validate identities across both inventories before exposing any axis lookup.
    /// # Errors
    /// Returns DuplicateSource for the first repeated canonical identity, including cross-role aliases.
    pub fn new(
        lengths: &[&'a LengthDeclaration],
        scalars: &[&'a ScalarInputDeclaration],
    ) -> Result<Self, SizeAxisError> {
        let mut records = BTreeMap::new();
        for record in lengths
            .iter()
            .copied()
            .map(Record::Length)
            .chain(scalars.iter().copied().map(Record::Scalar))
        {
            if records.insert(record.id(), record).is_some() {
                return Err(SizeAxisError::DuplicateSource(record.id()));
            }
        }
        Ok(Self { records })
    }

    pub(crate) fn contains(&self, identity: EntityId) -> bool {
        self.records.contains_key(&identity)
    }

    pub(crate) fn length(&self, identity: EntityId) -> Option<&'a LengthDeclaration> {
        match self.records.get(&identity)? {
            Record::Length(value) => Some(*value),
            Record::Scalar(_) => None,
        }
    }

    pub(crate) fn conflicts_with(
        &self,
        context: &crate::MeasurementTableContext<'_>,
    ) -> Option<EntityId> {
        self.records.iter().find_map(|(id, record)| {
            let conflict = context.current_measurement(*id).is_some()
                || (context.records().contains(*id)
                    && match record {
                        Record::Length(value) => {
                            context.records().current_declaration(*id) != Some(*value)
                        }
                        Record::Scalar(_) => true,
                    });
            conflict.then_some(*id)
        })
    }

    pub(super) fn resolve<'v>(
        &self,
        axis: EntityId,
        value: &'v SizeAxisValueDefinition,
    ) -> Result<SizeAxisValueContent<'v>, SizeAxisError>
    where
        'a: 'v,
    {
        let (declaration, expected) = match &value.source {
            SizeAxisValueSource::Label(label) => return Ok(SizeAxisValueContent::Label(label)),
            SizeAxisValueSource::Length(id) => (*id, Role::Length),
            SizeAxisValueSource::Scalar(id) => (*id, Role::Scalar),
        };
        let record =
            self.records
                .get(&declaration)
                .copied()
                .ok_or(SizeAxisError::MissingSource {
                    axis,
                    value: value.id,
                    declaration,
                    expected,
                })?;
        let actual = record.role();
        if actual != expected {
            return Err(SizeAxisError::SourceRole {
                axis,
                value: value.id,
                declaration,
                expected,
                actual,
            });
        }
        Ok(match record {
            Record::Length(record) => SizeAxisValueContent::Length(record),
            Record::Scalar(record) => SizeAxisValueContent::Scalar(record),
        })
    }
}
