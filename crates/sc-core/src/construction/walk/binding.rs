//! Exact original-side anchor association;currentness and geometric correspondence remain separate.
use super::{AdmittedWalkSeamEdit, WalkBalancePair};
use crate::ontology::{EdgeAnchor, EdgeRange, SeamSide, SeamSideId, SeamSpan, SewingGraph};
use core::fmt;

fn span_side(span: &SeamSpan, side: SeamSideId) -> &SeamSide {
    match side {
        SeamSideId::A => &span.definition().a,
        SeamSideId::B => &span.definition().b,
    }
}
/// Actual graph lookup or first original pair/side association cause;no alias is silently followed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkEditBindingRefusal<'a> {
    /// The original required span id is absent from the provided graph's spans() collection.
    MissingSpan,
    /// Mark edge differs from that actual side's original source frame.
    Edge {
        /// Actual zero-based authored pair index.
        pair: usize,
        /// Actual A/B request field.
        side: SeamSideId,
        /// Original requested anchor.
        anchor: &'a EdgeAnchor,
        /// Actual side's original source range.
        source: &'a EdgeRange,
    },
    /// Exact mark position is outside that side's inclusive original bounds.
    Position {
        /// Actual zero-based authored pair index.
        pair: usize,
        /// Actual A/B request field.
        side: SeamSideId,
        /// Original requested anchor.
        anchor: &'a EdgeAnchor,
        /// Actual side's original source range.
        source: &'a EdgeRange,
    },
}
/// Actual admitted owner,provided graph and typed cause;no accepted bound prefix is returned.
pub struct WalkEditBindingError<'a> {
    admitted: &'a AdmittedWalkSeamEdit<'a>,
    graph: &'a SewingGraph,
    refusal: WalkEditBindingRefusal<'a>,
}
impl<'a> WalkEditBindingError<'a> {
    /// Original admitted input and its explicit policy,not a cloned request.
    #[must_use]
    pub const fn admitted(&self) -> &'a AdmittedWalkSeamEdit<'a> {
        self.admitted
    }
    /// Actual provided immutable graph,including its original canonical span records.
    #[must_use]
    pub const fn graph(&self) -> &'a SewingGraph {
        self.graph
    }
    /// Original missing-span or first actual pair/side edge/position cause.
    #[must_use]
    pub const fn refusal(&self) -> WalkEditBindingRefusal<'a> {
        self.refusal
    }
}
impl fmt::Debug for WalkEditBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkEditBindingError")
            .field("operation", &self.admitted.edit().operation())
            .field("span", &self.admitted.edit().span_id())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkEditBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.refusal {
            WalkEditBindingRefusal::MissingSpan => {
                f.write_str("walk editing span is absent from the provided graph span collection")
            }
            WalkEditBindingRefusal::Edge { pair, side, .. } => write!(
                f,
                "walk edit pair {pair} {side:?} edge differs from its original span source"
            ),
            WalkEditBindingRefusal::Position { pair, side, .. } => write!(
                f,
                "walk edit pair {pair} {side:?} position lies outside its original span range"
            ),
        }
    }
}
impl std::error::Error for WalkEditBindingError<'_> {}

/// Complete association with the actual provided graph/span;no notch or geometry is published.
/// ```compile_fail
/// fn escape(admitted:sc_core::construction::AdmittedWalkSeamEdit<'_>,graph:sc_core::ontology::SewingGraph)
///     ->sc_core::construction::BoundWalkSeamEdit<'static> {
///     admitted.bind_source(&graph).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundWalkSeamEdit<'a> {
    admitted: &'a AdmittedWalkSeamEdit<'a>,
    graph: &'a SewingGraph,
    span: &'a SeamSpan,
}
impl<'a> BoundWalkSeamEdit<'a> {
    /// Actual admitted input and original complete request population.
    #[must_use]
    pub const fn admitted(self) -> &'a AdmittedWalkSeamEdit<'a> {
        self.admitted
    }
    /// Actual provided canonical graph;membership in a current containing Design is separate.
    #[must_use]
    pub const fn graph(self) -> &'a SewingGraph {
        self.graph
    }
    /// Actual span record from that graph,including direction,ease,stops and physical-copy sides.
    #[must_use]
    pub const fn span(self) -> &'a SeamSpan {
        self.span
    }
    /// All original pairs in authored order,including repeated requests. Private views borrow
    /// this actual admitted edit,graph/span and original pair field;no output is generated.
    pub fn pairs(
        self,
    ) -> impl ExactSizeIterator<Item = BoundWalkBalancePair<'a>> + DoubleEndedIterator + 'a {
        self.admitted
            .edit()
            .pairs()
            .iter()
            .enumerate()
            .map(move |(index, pair)| BoundWalkBalancePair {
                admitted: self.admitted,
                graph: self.graph,
                span: self.span,
                index,
                pair,
            })
    }
}
/// One authentic original pair/source association;requested anchors do not prove sewing correspondence.
/// ```compile_fail
/// fn forge<'a>(admitted:&'a sc_core::construction::AdmittedWalkSeamEdit<'a>,
///     graph:&'a sc_core::ontology::SewingGraph,span:&'a sc_core::ontology::SeamSpan,
///     pair:&'a sc_core::construction::WalkBalancePair)->sc_core::construction::BoundWalkBalancePair<'a> {
///     sc_core::construction::BoundWalkBalancePair { admitted,graph,span,index:0,pair }
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundWalkBalancePair<'a> {
    admitted: &'a AdmittedWalkSeamEdit<'a>,
    graph: &'a SewingGraph,
    span: &'a SeamSpan,
    index: usize,
    pair: &'a WalkBalancePair,
}
impl<'a> BoundWalkBalancePair<'a> {
    /// Actual original admitted edit supplying this pair.
    #[must_use]
    pub const fn admitted(self) -> &'a AdmittedWalkSeamEdit<'a> {
        self.admitted
    }
    /// Actual graph supplying this pair's canonical correspondence.
    #[must_use]
    pub const fn graph(self) -> &'a SewingGraph {
        self.graph
    }
    /// Actual span,not a detached equal-looking replacement.
    #[must_use]
    pub const fn span(self) -> &'a SeamSpan {
        self.span
    }
    /// Actual zero-based authored pair position;not a formula/operation statement ordinal.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original pair field,retaining both anchors and all profile contexts.
    #[must_use]
    pub const fn pair(self) -> &'a WalkBalancePair {
        self.pair
    }
    /// Actual original span-side field;copy is a physical-copy identity,not a pattern Piece id.
    #[must_use]
    pub fn source_side(self, side: SeamSideId) -> &'a SeamSide {
        span_side(self.span, side)
    }
}
impl<'s> AdmittedWalkSeamEdit<'s> {
    /// Resolve the actual named span in the provided graph,then check all original pairs A/B.
    /// Edge identities must match the actual side's original frame;inclusive range endpoints
    /// are retained without remapping. No current fragment alias or matching counterpart is guessed.
    /// This proves association/bounds only. Current Design/graph/ledger ownership,arclength
    /// correspondence,ease/stops,profile truth,semantic output policy and geometry remain owned.
    /// # Errors
    /// Actual missing target,then first actual pair/side edge or position refusal. No prefix escapes.
    pub fn bind_source<'a>(
        &'a self,
        graph: &'a SewingGraph,
    ) -> Result<BoundWalkSeamEdit<'a>, WalkEditBindingError<'a>>
    where
        's: 'a,
    {
        let admitted: &AdmittedWalkSeamEdit<'a> = self;
        let refuse = |refusal| WalkEditBindingError {
            admitted,
            graph,
            refusal,
        };
        let span = graph
            .spans()
            .iter()
            .find(|span| span.id() == admitted.edit().span_id())
            .ok_or_else(|| refuse(WalkEditBindingRefusal::MissingSpan))?;
        for (index, pair) in admitted.edit().pairs().iter().enumerate() {
            for side in [SeamSideId::A, SeamSideId::B] {
                let anchor = pair.mark(side).anchor();
                let source = &span_side(span, side).range;
                if anchor.edge != source.edge() {
                    return Err(refuse(WalkEditBindingRefusal::Edge {
                        pair: index,
                        side,
                        anchor,
                        source,
                    }));
                }
                if anchor.param < source.from() || anchor.param > source.to() {
                    return Err(refuse(WalkEditBindingRefusal::Position {
                        pair: index,
                        side,
                        anchor,
                        source,
                    }));
                }
            }
        }
        Ok(BoundWalkSeamEdit {
            admitted,
            graph,
            span,
        })
    }
}
