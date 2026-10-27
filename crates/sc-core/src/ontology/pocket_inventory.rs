//! Owned Pockets and complete current component references, without physical opening proof.
use super::{
    CutPlan, CutPlanError, Direction, EntityId, IdentityLedger, Param, Piece, Pocket, PocketError,
    RangeResolution, Resolution,
};
use core::fmt;
use std::collections::BTreeMap;

/// Original collection/current source refusal with actual positions and complete raw evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PocketInventoryError {
    /// Two authored Pocket objects claim the same owned identity.
    DuplicateIdentity {
        /// Exact repeated identity.
        identity: EntityId,
        /// Zero-based first authored position.
        first: usize,
        /// Zero-based first repeated authored position.
        duplicate: usize,
    },
    /// Original complete current physical-copy plan refusal.
    CutPlan(Box<CutPlanError>),
    /// Original actual Pocket target/placement refusal.
    Pocket {
        /// Zero-based authored Pocket position.
        index: usize,
        /// Actual Pocket identity.
        pocket: EntityId,
        /// Original typed target/placement cause.
        issue: Box<PocketError>,
    },
    /// A component's held interval/endpoints need repair or an explicit choice.
    ComponentRange {
        /// Actual holding Pocket identity.
        pocket: EntityId,
        /// Zero-based authored component position.
        component: usize,
        /// Exact authored physical component copy.
        copy: EntityId,
        /// Actual component source Piece.
        piece: EntityId,
        /// Zero-based held edge position across boundary, holes and construction lines.
        edge: usize,
        /// Original traversal; evidence stays in the stored forward frame.
        direction: Direction,
        /// Full raw interval and point-endpoint evidence, not a nested Error.
        evidence: Box<RangeResolution>,
        /// Choice-aware lower/upper answers in the held forward frame; raw evidence stays intact.
        endpoints: Box<[Resolution; 2]>,
    },
}
impl fmt::Display for PocketInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "Pocket identity {identity} repeats at {duplicate}, first supplied at {first}"),
            Self::CutPlan(issue) => write!(f, "Pocket inventory current copies: {issue}"),
            Self::Pocket { index, pocket, issue } => write!(f, "Pocket {pocket} at {index}: {issue}"),
            Self::ComponentRange { pocket, component, copy, piece, edge, .. } => write!(f, "Pocket {pocket} component {component}/copy {copy}/Piece {piece} edge {edge} needs interval or endpoint repair/choice"),
        }
    }
}
impl std::error::Error for PocketInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CutPlan(issue) => Some(issue.as_ref()),
            Self::Pocket { issue, .. } => Some(issue.as_ref()),
            Self::DuplicateIdentity { .. } | Self::ComponentRange { .. } => None,
        }
    }
}

/// Private immutable authored Pocket collection; contexts are borrowed only for current queries.
///
/// Construction checks collection ids and preserves exact objects, including held references after
/// edits. Current validation checks complete copies, placements and all component intervals.
/// Global Design id joins, logical opening declarations and G2/G3/G4 physical scope stay separate.
/// An explicit empty collection makes no whole Design or assembly readiness claim.
///
/// ```compile_fail
/// fn edit(pockets: &mut sc_core::ontology::PocketInventory) { pockets.pockets.clear(); }
/// ```
/// ```compile_fail
/// fn edit(pockets: &mut sc_core::ontology::PocketInventory) { pockets.pockets().get_mut(0); }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct PocketInventory {
    pockets: Vec<Pocket>,
}
impl fmt::Debug for PocketInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PocketInventory")
            .field("pocket_count", &self.pockets.len())
            .finish_non_exhaustive()
    }
}
impl PocketInventory {
    /// Own exact authored order without sorting, selecting or repairing references.
    ///
    /// # Errors
    /// Returns first duplicate Pocket identity with both original positions.
    pub fn new(pockets: Vec<Pocket>) -> Result<Self, PocketInventoryError> {
        let mut seen = BTreeMap::new();
        for (duplicate, pocket) in pockets.iter().enumerate() {
            if let Some(first) = seen.insert(pocket.id(), duplicate) {
                return Err(PocketInventoryError::DuplicateIdentity {
                    identity: pocket.id(),
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self { pockets })
    }
    /// Exact original authored objects; shared access grants no mutator.
    #[must_use]
    pub fn pockets(&self) -> &[Pocket] {
        &self.pockets
    }
    /// Inspect a held object by exact id; presence alone supplies no current source proof.
    #[must_use]
    pub fn pocket(&self, identity: EntityId) -> Option<&Pocket> {
        self.pockets.iter().find(|pocket| pocket.id() == identity)
    }
    /// Check current complete copies, every placement and every component's full held interval.
    ///
    /// Boundary, hole and construction references are checked, not only attachment endpoints.
    /// Raw point choices/repairs stay explicit; data is never rewritten. This does not resolve
    /// opening domain/state/dependencies or certify physical geometry.
    ///
    /// # Errors
    /// Returns first complete-plan cause, then first Pocket/component failure in authored order.
    pub fn validate_current(
        &self,
        plan: &CutPlan,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<(), PocketInventoryError> {
        CutPlan::new(
            plan.copies()
                .iter()
                .map(|copy| *copy.definition())
                .collect(),
            pieces,
        )
        .map_err(|issue| PocketInventoryError::CutPlan(Box::new(issue)))?;
        let choices = ledger
            .registrations()
            .map(|registration| {
                (
                    (
                        registration.owner(),
                        registration.edge(),
                        registration.param(),
                    ),
                    registration.split_choice(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for (index, pocket) in self.pockets().iter().enumerate() {
            let current_issue = |issue| PocketInventoryError::Pocket {
                index,
                pocket: pocket.id(),
                issue: Box::new(issue),
            };
            pocket
                .validate_current(plan, pieces, ledger)
                .map_err(current_issue)?;
            for (component, reference) in pocket.definition().components.iter().enumerate() {
                let piece = pocket
                    .component(reference.copy, plan, pieces)
                    .map_err(current_issue)?;
                for (edge, (held, evidence)) in piece.range_resolutions(ledger).enumerate() {
                    let endpoint = |param, raw: &Resolution| {
                        choices
                            .get(&(piece.id(), held.edge, param))
                            .copied()
                            .flatten()
                            .map_or_else(
                                || raw.clone(),
                                |side| ledger.resolve_with_choice(held.edge, param, Some(side)),
                            )
                    };
                    let start = endpoint(Param::START, evidence.start());
                    let end = endpoint(Param::END, evidence.end());
                    if !evidence.has_full_coverage()
                        || start.resolved().is_none()
                        || end.resolved().is_none()
                    {
                        return Err(PocketInventoryError::ComponentRange {
                            pocket: pocket.id(),
                            component,
                            copy: reference.copy,
                            piece: piece.id(),
                            edge,
                            direction: held.direction,
                            evidence: Box::new(evidence),
                            endpoints: Box::new([start, end]),
                        });
                    }
                }
            }
        }
        Ok(())
    }
}
