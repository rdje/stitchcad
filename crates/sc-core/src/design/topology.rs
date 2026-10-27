//! Scoped current point and whole-interval evidence over an immutable structural owner.
use super::DesignStructure;
use crate::ontology::{
    DirectedEdge, Direction, EntityId, LedgerError, Param, RangeResolution, Registration,
    Resolution,
};
use core::fmt;
use std::collections::BTreeMap;

/// Every original held Piece edge, with raw range evidence and current authored endpoint answers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignPieceTopology {
    piece: EntityId,
    index: usize,
    held: DirectedEdge,
    range: RangeResolution,
    start: Resolution,
    end: Resolution,
}
impl DesignPieceTopology {
    /// Actual holding Piece identity.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Zero-based authored edge position across boundary, holes and construction lines.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }
    /// Original directed edge, never rewritten to a surviving fragment.
    #[must_use]
    pub const fn held(&self) -> DirectedEdge {
        self.held
    }
    /// Unmodified whole-interval evidence, including raw source-frame endpoint answers.
    #[must_use]
    pub const fn range(&self) -> &RangeResolution {
        &self.range
    }
    /// Authored traversal start, applying only this Piece's exact saved registration choice.
    #[must_use]
    pub const fn start(&self) -> &Resolution {
        &self.start
    }
    /// Authored traversal end, applying only this Piece's exact saved registration choice.
    #[must_use]
    pub const fn end(&self) -> &Resolution {
        &self.end
    }
    /// Complete interval coverage and uniquely resolved authored endpoints, without geometry proof.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.range.has_full_coverage()
            && self.start.resolved().is_some()
            && self.end.resolved().is_some()
    }
}

/// Derived structural topology readiness, borrowing the exact immutable owner it describes.
///
/// All registered points and held Piece intervals/endpoints are inspected. Unresolved points,
/// unstated split choices and lost interiors block this scope. Empty drafts are structurally ready
/// in this scope only. Sewing/garment references, geometry, execution and release remain separate.
///
/// ```compile_fail
/// fn forge(report: &mut sc_core::design::DesignTopologyReadiness<'_>) { report.piece_edges.clear(); }
/// ```
/// ```compile_fail
/// fn escape(state: sc_core::design::DesignStructure) -> sc_core::design::DesignTopologyReadiness<'static> {
///     state.topology_readiness().unwrap()
/// }
/// ```
#[derive(Clone)]
pub struct DesignTopologyReadiness<'a> {
    structure: &'a DesignStructure,
    registrations: Vec<(Registration, Resolution)>,
    piece_edges: Vec<DesignPieceTopology>,
}
impl fmt::Debug for DesignTopologyReadiness<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignTopologyReadiness")
            .field("registration_count", &self.registrations.len())
            .field("piece_edge_count", &self.piece_edges.len())
            .finish_non_exhaustive()
    }
}
impl<'a> DesignTopologyReadiness<'a> {
    /// Exact structural owner; no cached result can be transferred to a replacement state.
    #[must_use]
    pub const fn structure(&self) -> &'a DesignStructure {
        self.structure
    }
    /// Every actual registration in ledger order, with its original choice-aware current answer.
    #[must_use]
    pub fn registrations(&self) -> &[(Registration, Resolution)] {
        &self.registrations
    }
    /// Every held Piece edge in Piece/edge authored order, including unregistered endpoints.
    #[must_use]
    pub fn piece_edges(&self) -> &[DesignPieceTopology] {
        &self.piece_edges
    }
    /// No current point ambiguity/repair or uncovered Piece interval in this structural scope.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.registrations
            .iter()
            .all(|(_, answer)| answer.resolved().is_some())
            && self.piece_edges.iter().all(DesignPieceTopology::is_ready)
    }
}
impl DesignStructure {
    /// Inspect every actual registered point and complete held Piece interval without mutation.
    ///
    /// Explicit choices apply only to matching owner/edge/parameter registrations. Raw interval
    /// evidence remains unchanged; traversal endpoints also honor each held edge's direction.
    /// This does not call or widen the ledger's narrower release_readiness verdict.
    ///
    /// # Errors
    /// Propagates an original registered-point lookup refusal without omitting that point.
    pub fn topology_readiness(&self) -> Result<DesignTopologyReadiness<'_>, LedgerError> {
        let ledger = self.topology();
        let registrations = ledger
            .registrations()
            .map(|registration| {
                ledger
                    .resolve_registered(
                        registration.owner(),
                        registration.edge(),
                        registration.param(),
                    )
                    .map(|answer| (registration, answer))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let answers: BTreeMap<_, _> = registrations
            .iter()
            .map(|(registration, answer)| {
                (
                    (
                        registration.owner(),
                        registration.edge(),
                        registration.param(),
                    ),
                    answer,
                )
            })
            .collect();
        let mut piece_edges = Vec::new();
        for piece in self.pieces().pieces() {
            for (index, (held, range)) in piece.range_resolutions(ledger).enumerate() {
                let endpoint = |param| {
                    answers.get(&(piece.id(), held.edge, param)).map_or_else(
                        || ledger.resolve(held.edge, param),
                        |answer| (*answer).clone(),
                    )
                };
                let (start, end) = match held.direction {
                    Direction::Original => (Param::START, Param::END),
                    Direction::Reversed => (Param::END, Param::START),
                };
                piece_edges.push(DesignPieceTopology {
                    piece: piece.id(),
                    index,
                    held,
                    range,
                    start: endpoint(start),
                    end: endpoint(end),
                });
            }
        }
        Ok(DesignTopologyReadiness {
            structure: self,
            registrations,
            piece_edges,
        })
    }
}
