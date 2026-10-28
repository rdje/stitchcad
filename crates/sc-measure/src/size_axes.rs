//! Complete explicit member points over one or more authored axes, without grid interpolation.
use crate::{
    SizeAxis, SizeAxisSourceContext, SizeAxisValueView, SizeMember, SizeMembership,
    SizeSetReference,
};
use sc_core::ontology::EntityId;

mod error;
mod validation;
pub use error::SizeAxesError;

/// Explicit coordinate reference; neither member label nor position supplies either identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizeAxisCoordinate {
    /// Authored owning axis identity.
    pub axis: EntityId,
    /// Authored value identity belonging to that axis.
    pub value: EntityId,
}

/// One member's complete authored point; coordinate order is preserved independently of axis order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeMemberPoint {
    /// Actual stable membership identity, never a label or ordinal.
    pub member: EntityId,
    /// Exactly one explicit coordinate for every axis, without default values.
    pub coordinates: Vec<SizeAxisCoordinate>,
}

/// Editable complete axis/member-point input. Charts, breaks and profile resolution are separate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeAxesDefinition {
    /// Exact set/revision shared with the supplied membership and every axis.
    pub reference: SizeSetReference,
    /// Nonempty authored order of named axes and their explicit value catalogs.
    pub axes: Vec<SizeAxis>,
    /// Exactly one complete distinct point per actual member; no Cartesian grid is manufactured.
    pub points: Vec<SizeMemberPoint>,
}

/// Immutable locally validated axis/member coverage, not a complete SizeSet or registry certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeAxes {
    definition: SizeAxesDefinition,
}

/// Original authored point and actual current member, borrowed without copied labels or coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizeMemberPointView<'a> {
    member: &'a SizeMember,
    definition: &'a SizeMemberPoint,
}
impl<'a> SizeMemberPointView<'a> {
    /// Actual current membership record.
    #[must_use]
    pub const fn member(self) -> &'a SizeMember {
        self.member
    }
    /// Original explicit coordinates and their authored order.
    #[must_use]
    pub const fn definition(self) -> &'a SizeMemberPoint {
        self.definition
    }
}

impl SizeAxes {
    pub(crate) fn resource_definition(&self) -> &SizeAxesDefinition {
        let Self { definition } = self;
        definition
    }
    /// Validate exact pins, nonempty axes and complete distinct member points against actual records.
    /// # Errors
    /// Returns structural, membership, coordinate-ownership or current canonical-source evidence.
    pub fn new(
        definition: SizeAxesDefinition,
        membership: &SizeMembership,
        sources: &SizeAxisSourceContext<'_>,
    ) -> Result<Self, SizeAxesError> {
        validation::validate(&definition, membership, sources)?;
        Ok(Self { definition })
    }

    /// Original authored vectors, without unchecked mutation or implicit reordering.
    #[must_use]
    pub const fn definition(&self) -> &SizeAxesDefinition {
        &self.definition
    }

    /// Recheck the entire aggregate against supplied current membership and canonical declarations.
    /// # Errors
    /// Returns the first current structural/source refusal; equal supplied pins do not prove registry truth.
    pub fn validate_current(
        &self,
        membership: &SizeMembership,
        sources: &SizeAxisSourceContext<'_>,
    ) -> Result<(), SizeAxesError> {
        validation::validate(&self.definition, membership, sources)
    }

    /// Borrow the actual member and original explicit point after complete current revalidation.
    /// # Errors
    /// Returns current aggregate refusals before an absent requested member.
    pub fn point<'a>(
        &'a self,
        member: EntityId,
        membership: &'a SizeMembership,
        sources: &SizeAxisSourceContext<'_>,
    ) -> Result<SizeMemberPointView<'a>, SizeAxesError> {
        self.validate_current(membership, sources)?;
        let actual = membership.member(member).map_err(SizeAxesError::Member)?;
        let definition = self
            .definition
            .points
            .iter()
            .find(|point| point.member == member)
            .ok_or(SizeAxesError::MissingMember(member))?;
        Ok(SizeMemberPointView {
            member: actual,
            definition,
        })
    }

    /// Borrow the member's explicit coordinate and its original current canonical content.
    /// # Errors
    /// Returns full current validation first, then actual requested member/axis lookup refusals.
    pub fn coordinate<'a>(
        &'a self,
        member: EntityId,
        axis: EntityId,
        membership: &SizeMembership,
        sources: &SizeAxisSourceContext<'a>,
    ) -> Result<SizeAxisValueView<'a>, SizeAxesError> {
        self.validate_current(membership, sources)?;
        membership.member(member).map_err(SizeAxesError::Member)?;
        let point = self
            .definition
            .points
            .iter()
            .find(|point| point.member == member)
            .ok_or(SizeAxesError::MissingMember(member))?;
        let coordinate = point
            .coordinates
            .iter()
            .find(|value| value.axis == axis)
            .ok_or(SizeAxesError::MissingAxis(axis))?;
        let actual = self
            .definition
            .axes
            .iter()
            .find(|value| value.definition().id == axis)
            .ok_or(SizeAxesError::MissingAxis(axis))?;
        actual
            .value(coordinate.value, sources)
            .map_err(SizeAxesError::Axis)
    }
}
