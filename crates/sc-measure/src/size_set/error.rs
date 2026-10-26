//! Original chart/break evidence and cross-view snapshot/identity refusals.
use super::SizeSetSnapshot;
use crate::{MtmChartError, SizeBreaksError, SizeChartCollectionError};
use core::fmt;
use sc_core::ontology::EntityId;

/// Composite current SizeSet input refusal, without cached values or an inferred snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetInputError {
    /// Original current chart/reference/coverage failure.
    Chart(Box<SizeChartCollectionError>),
    /// Original current MTM body/Ease/reference/coverage or grading-path refusal.
    Mtm(Box<MtmChartError>),
    /// Original current break requirement/declaration/coverage failure.
    Breaks(Box<SizeBreaksError>),
    /// Views do not borrow the same actual snapshot object; matching ids alone do not unify them.
    SharedSnapshot(SizeSetSnapshot),
    /// Cross-view entities reuse an identity in another current inventory.
    DuplicateIdentity(EntityId),
}
impl fmt::Display for SizeSetInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size set inputs: {self:?}")
    }
}
impl std::error::Error for SizeSetInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Chart(issue) => Some(issue),
            Self::Mtm(issue) => Some(issue),
            Self::Breaks(issue) => Some(issue),
            _ => None,
        }
    }
}
