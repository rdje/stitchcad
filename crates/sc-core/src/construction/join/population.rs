//! Explicit canonical result cut population;actual source/result copy actions remain separate.
use super::BoundJoinPieceMetadata;
use crate::ontology::Mirroring;
use core::fmt;

/// One caller-authored quantity/mirroring declaration,without inherited source quantities.
/// Raw construction retains invalid values for contextual admission;no copy is allocated.
/// ```compile_fail
/// let population:sc_core::construction::JoinCutPopulation=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCutPopulation {
    quantity: u32,
    mirroring: Mirroring,
}
impl JoinCutPopulation {
    /// Require both canonical result fields explicitly,without counting or pairing source copies.
    #[must_use]
    pub const fn new(quantity: u32, mirroring: Mirroring) -> Self {
        Self {
            quantity,
            mirroring,
        }
    }
    /// Original declared total physical-cut quantity,including both hands for MirroredPairs.
    #[must_use]
    pub const fn quantity(&self) -> &u32 {
        &self.quantity
    }
    /// Original cut mode,including explicit separate-member hand/companion where requested.
    #[must_use]
    pub const fn mirroring(&self) -> &Mirroring {
        &self.mirroring
    }
}
/// First local canonical population contradiction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCutPopulationRefusal {
    /// Canonical Piece quantity must be positive.
    ZeroQuantity,
    /// MirroredPairs requires an even total,not an entered per-hand amount.
    UnpairedQuantity {
        /// Original odd requested total.
        quantity: u32,
    },
}
/// Actual complete metadata/source/splice and whole original population survive refusal.
pub struct JoinCutPopulationError<'a> {
    metadata: &'a BoundJoinPieceMetadata<'a>,
    population: &'a JoinCutPopulation,
    refusal: JoinCutPopulationRefusal,
}
impl<'a> JoinCutPopulationError<'a> {
    /// Actual complete original selected metadata/splice/source context.
    #[must_use]
    pub const fn metadata(&self) -> &'a BoundJoinPieceMetadata<'a> {
        self.metadata
    }
    /// Actual whole raw population,including all refused values.
    #[must_use]
    pub const fn population(&self) -> &'a JoinCutPopulation {
        self.population
    }
    /// First canonical zero/odd-total cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinCutPopulationRefusal {
        self.refusal
    }
}
impl fmt::Debug for JoinCutPopulationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCutPopulationError")
            .field("population", &self.population)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCutPopulationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join cut population: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCutPopulationError<'_> {}
impl<'s> BoundJoinPieceMetadata<'s> {
    /// Bind the single explicit result population under existing canonical positive/even-pair rules.
    /// Constant work/space,no scan,allocation,identity issuance or source/candidate mutation.
    /// Companion identity/reciprocity,every original-copy action,result membership/orientation,
    /// actual full inventory/CutPlan,fold/print indicators and physical/authority proofs stay required.
    /// # Errors
    /// Zero quantity,then odd MirroredPairs total. Values are never rounded or doubled.
    pub fn bind_cut_population<'a>(
        &'a self,
        population: &'a JoinCutPopulation,
    ) -> Result<BoundJoinCutPopulation<'a>, JoinCutPopulationError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| JoinCutPopulationError {
            metadata: self,
            population,
            refusal,
        };
        if population.quantity == 0 {
            return Err(refuse(JoinCutPopulationRefusal::ZeroQuantity));
        }
        if population.mirroring == Mirroring::MirroredPairs
            && !population.quantity.is_multiple_of(2)
        {
            return Err(refuse(JoinCutPopulationRefusal::UnpairedQuantity {
                quantity: population.quantity,
            }));
        }
        Ok(BoundJoinCutPopulation {
            metadata: self,
            population,
        })
    }
}
/// Authentic locally valid requested population;not a born Piece,copy plan or print view.
/// ```compile_fail
/// fn forge<'a>(mut bound:sc_core::construction::BoundJoinCutPopulation<'a>,
///     other:&'a sc_core::construction::JoinCutPopulation) {bound.population=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(metadata:&'a sc_core::construction::BoundJoinPieceMetadata<'a>)
///     ->sc_core::construction::BoundJoinCutPopulation<'a> {
///     use sc_core::{construction::*,ontology::Mirroring};
///     let population=JoinCutPopulation::new(1,Mirroring::Single);
///     metadata.bind_cut_population(&population).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundJoinCutPopulation<'a> {
    metadata: &'a BoundJoinPieceMetadata<'a>,
    population: &'a JoinCutPopulation,
}
impl<'a> BoundJoinCutPopulation<'a> {
    /// Actual complete original metadata/splice/source parents.
    #[must_use]
    pub const fn metadata(&self) -> &'a BoundJoinPieceMetadata<'a> {
        self.metadata
    }
    /// Actual original complete canonical population declaration.
    #[must_use]
    pub const fn population(&self) -> &'a JoinCutPopulation {
        self.population
    }
    /// Actual original authoritative total-quantity field;no derived second quantity.
    #[must_use]
    pub const fn quantity(&self) -> &'a u32 {
        self.population.quantity()
    }
    /// Actual original mode/hand/companion field,without companion validity or copy matching proof.
    #[must_use]
    pub const fn mirroring(&self) -> &'a Mirroring {
        self.population.mirroring()
    }
}
