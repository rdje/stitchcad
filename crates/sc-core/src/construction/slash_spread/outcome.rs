//! Explicit structural outcome requirements,not achieved geometry/fullness or publication.
use super::{RequiredSlashSpreadEffect, SlashSpreadContourEdge, SlashSpreadFullnessIntent};
use crate::ontology::DirectedEdge;
use core::fmt;

/// Required structural outcome;zero candidates never selects a mode implicitly.
/// ```compile_fail
/// let intent:sc_core::construction::SlashSpreadOutcomeIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadOutcomeIntent {
    /// Require exact original contours/metadata/copies with N0 and original Unchanged fullness intent.
    /// Actual supported domain,geometry/effect,current ownership and history remain evidence.
    PreserveSource,
    /// Reconcile the actual declared contours,whether candidates are present or absent.
    /// This does not infer physical change or no-change from population or metadata.
    ReconcileContours,
}
/// First source-preservation contradiction in population,fullness and original contour order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadOutcomeRefusal {
    /// Preserving the original source cannot introduce candidate edge identities.
    NewEdges,
    /// Source preservation requires the original authored Unchanged fullness request.
    Fullness,
    /// Boundary differs in identity,occurrence order or traversal.
    Boundary,
    /// Hole-group population differs from the original source.
    HoleGroups,
    /// First ordered hole differs in identity,occurrence order or traversal.
    Hole {
        /// Actual original/proposed hole position,not a persistent identity.
        hole: usize,
    },
    /// Construction occurrences differ in identity,order,traversal or repetitions.
    Construction,
}
/// Actual requested intent and complete effect/copy/source context survive refusal.
pub struct SlashSpreadOutcomeError<'a> {
    effect: &'a RequiredSlashSpreadEffect<'a>,
    intent: SlashSpreadOutcomeIntent,
    refusal: SlashSpreadOutcomeRefusal,
}
impl<'a> SlashSpreadOutcomeError<'a> {
    /// Actual complete original effect/copy/metadata/boundary/source context.
    #[must_use]
    pub const fn effect(&self) -> &'a RequiredSlashSpreadEffect<'a> {
        self.effect
    }
    /// Original explicit requested mode,owned unchanged by this error.
    #[must_use]
    pub const fn intent(&self) -> &SlashSpreadOutcomeIntent {
        &self.intent
    }
    /// First original structural contradiction.
    #[must_use]
    pub const fn refusal(&self) -> SlashSpreadOutcomeRefusal {
        self.refusal
    }
}
impl fmt::Debug for SlashSpreadOutcomeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadOutcomeError")
            .field("intent", &self.intent)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadOutcomeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread outcome: {:?}", self.refusal)
    }
}
impl std::error::Error for SlashSpreadOutcomeError<'_> {}
fn same_held_edges(proposed: &[SlashSpreadContourEdge], source: &[DirectedEdge]) -> bool {
    proposed.len() == source.len()
        && proposed.iter().zip(source).all(|(proposed, source)| {
            matches!(
                proposed, SlashSpreadContourEdge::Retained(edge) if edge == source
            )
        })
}
impl<'s> RequiredSlashSpreadEffect<'s> {
    /// Require explicit source preservation or reconciliation of the actual admitted contour plan.
    /// Source preservation checks N0,original Unchanged intent,then exact original boundary,hole
    /// groups/holes and construction occurrences. Already admitted whole N/R/M bounds apply to
    /// these scans;no allocation,evaluation,ledger edit or evidence acceptance occurs.
    /// # Errors
    /// PreserveSource contradicts candidate population,fullness,boundary,hole groups,first hole
    /// or construction,in that order. ReconcileContours adds no physical-effect conclusion.
    pub fn require_outcome<'a>(
        &'a self,
        intent: SlashSpreadOutcomeIntent,
    ) -> Result<RequiredSlashSpreadOutcome<'a>, SlashSpreadOutcomeError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| SlashSpreadOutcomeError {
            effect: self,
            intent,
            refusal,
        };
        let contours = self
            .copies()
            .metadata()
            .boundary()
            .consumers()
            .registration_plan()
            .sources()
            .contours();
        if intent == SlashSpreadOutcomeIntent::PreserveSource {
            if !contours.edges().manifest().candidates().is_empty() {
                return Err(refuse(SlashSpreadOutcomeRefusal::NewEdges));
            }
            if self.requested_fullness() != &SlashSpreadFullnessIntent::Unchanged {
                return Err(refuse(SlashSpreadOutcomeRefusal::Fullness));
            }
            let proposed = contours.contours();
            let source = contours.source_piece().definition();
            if !same_held_edges(proposed.boundary(), &source.boundary) {
                return Err(refuse(SlashSpreadOutcomeRefusal::Boundary));
            }
            if proposed.holes().len() != source.holes.len() {
                return Err(refuse(SlashSpreadOutcomeRefusal::HoleGroups));
            }
            for (hole, (proposed, source)) in proposed.holes().iter().zip(&source.holes).enumerate()
            {
                if !same_held_edges(proposed, source) {
                    return Err(refuse(SlashSpreadOutcomeRefusal::Hole { hole }));
                }
            }
            if !same_held_edges(proposed.construction_lines(), &source.construction_lines) {
                return Err(refuse(SlashSpreadOutcomeRefusal::Construction));
            }
        }
        Ok(RequiredSlashSpreadOutcome {
            effect: self,
            intent,
        })
    }
}
/// Authentic explicit structural outcome requirement;no achieved no-op/effect/readiness verdict.
/// ```compile_fail
/// fn forge<'a>(mut outcome:sc_core::construction::RequiredSlashSpreadOutcome<'a>,other:&'a sc_core::construction::RequiredSlashSpreadEffect<'a>) {
///     outcome.effect=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredSlashSpreadOutcome<'a> {
    effect: &'a RequiredSlashSpreadEffect<'a>,
    intent: SlashSpreadOutcomeIntent,
}
impl<'a> RequiredSlashSpreadOutcome<'a> {
    /// Actual complete original effect/copy/metadata/boundary/source context.
    #[must_use]
    pub const fn effect(&self) -> &'a RequiredSlashSpreadEffect<'a> {
        self.effect
    }
    /// Original explicit selected mode,not inferred from population or geometry.
    #[must_use]
    pub const fn intent(&self) -> &SlashSpreadOutcomeIntent {
        &self.intent
    }
    /// Actual complete candidate population from the authentic original manifest.
    #[must_use]
    pub fn new_edge_count(&self) -> usize {
        self.effect
            .copies()
            .metadata()
            .boundary()
            .consumers()
            .registration_plan()
            .sources()
            .contours()
            .edges()
            .manifest()
            .candidates()
            .len()
    }
}
