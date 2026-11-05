//! Original complete coverage witnesses and first cause,with no accepted association prefix.
use super::{DraftingProgramSemanticWorkError, DraftingProgramSemanticWorkLimits, Key, Location};
use crate::{
    construction::{DraftingProgramSemanticPopulationShape, DraftingProgramSemanticWorkShape},
    design::DesignResourceDemand,
};
use core::fmt;
use std::collections::TryReserveError;
/// Original first fresh work,reservation or actual selection-key refusal.
pub enum DraftingProgramSelectionCoverageRefusal<'w, I: ?Sized> {
    /// Original typed complete aggregate cause and stage witnesses.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// A complete bounded association/row vector could not be reserved.
    Reservation {
        /// Original requested logical record count.
        records: usize,
        /// Original allocator reservation cause.
        cause: TryReserveError,
    },
    /// First original row matching neither a direct requirement nor an actual native target.
    UnexpectedSelection {
        /// Original zero-based row index.
        row: usize,
        /// Original complete requested structural key.
        key: Key,
    },
    /// First original required direct field with no explicit row;no default origin or pin exists.
    MissingSelection {
        /// Actual original field coordinates under the retained owner.
        location: Location,
        /// Actual original required structural key.
        key: Key,
    },
}
impl<I: ?Sized> fmt::Debug for DraftingProgramSelectionCoverageRefusal<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Work(cause) => f.debug_tuple("Work").field(cause).finish(),
            Self::Reservation { records, cause } => f
                .debug_struct("Reservation")
                .field("records", records)
                .field("cause", cause)
                .finish(),
            Self::UnexpectedSelection { row, key } => f
                .debug_struct("UnexpectedSelection")
                .field("row", row)
                .field("key", key)
                .finish(),
            Self::MissingSelection { location, key } => f
                .debug_struct("MissingSelection")
                .field("location", location)
                .field("key", key)
                .finish(),
        }
    }
}
/// Original exact owner/policy,only completed fresh stage metadata,and first original cause.
pub struct DraftingProgramSelectionCoverageError<'w, I: ?Sized> {
    details: Box<Failure<'w, I>>,
}
struct Failure<'w, I: ?Sized> {
    inputs: &'w I,
    limits: &'w DraftingProgramSemanticWorkLimits,
    demand: Option<DesignResourceDemand>,
    population: Option<DraftingProgramSemanticPopulationShape>,
    work: Option<DraftingProgramSemanticWorkShape>,
    refusal: DraftingProgramSelectionCoverageRefusal<'w, I>,
}
impl<'w, I: ?Sized> DraftingProgramSelectionCoverageError<'w, I> {
    pub(super) fn new(
        inputs: &'w I,
        limits: &'w DraftingProgramSemanticWorkLimits,
        demand: Option<DesignResourceDemand>,
        population: Option<DraftingProgramSemanticPopulationShape>,
        work: Option<DraftingProgramSemanticWorkShape>,
        refusal: DraftingProgramSelectionCoverageRefusal<'w, I>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                inputs,
                limits,
                demand,
                population,
                work,
                refusal,
            }),
        }
    }
    /// Exact original joined owner and all its actual source/programme/selection loans.
    #[must_use]
    pub const fn inputs(&self) -> &'w I {
        self.details.inputs
    }
    /// Exact original full selected aggregate policy.
    #[must_use]
    pub const fn limits(&self) -> &'w DraftingProgramSemanticWorkLimits {
        self.details.limits
    }
    /// Complete fresh original content,only after that stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Complete original population shape,only after its arithmetic passed.
    #[must_use]
    pub const fn population_shape(&self) -> Option<DraftingProgramSemanticPopulationShape> {
        self.details.population
    }
    /// Complete selected child-work shape,only after its arithmetic passed.
    #[must_use]
    pub const fn work_shape(&self) -> Option<DraftingProgramSemanticWorkShape> {
        self.details.work
    }
    /// Original first typed cause,including borrowed fresh work witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramSelectionCoverageRefusal<'w, I> {
        &self.details.refusal
    }
}
impl<I: ?Sized> fmt::Debug for DraftingProgramSelectionCoverageError<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramSelectionCoverageError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<I: ?Sized> fmt::Display for DraftingProgramSelectionCoverageError<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting direct selection coverage: {:?}",
            self.details.refusal
        )
    }
}
impl<I: ?Sized> std::error::Error for DraftingProgramSelectionCoverageError<'_, I> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramSelectionCoverageRefusal::Work(cause) => {
                std::error::Error::source(cause)
            }
            DraftingProgramSelectionCoverageRefusal::Reservation { records: _, cause } => {
                Some(cause)
            }
            _ => None,
        }
    }
}
