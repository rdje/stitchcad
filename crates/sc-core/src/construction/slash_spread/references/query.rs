//! Exact declared-source eligibility;retained/range/bridge endpoints remain distinct,not mapped points.
use super::super::{
    AdmittedSlashSpreadContours, SlashSpreadBridgeDirection, SlashSpreadContourUse,
    SlashSpreadCutEnd, SlashSpreadEdge, SlashSpreadEdgeOrigin, SlashSpreadSourceMotion,
};
use crate::ontology::{DirectedRange, EdgeAnchor, EdgeRef};
use core::fmt;

/// Complete candidate/source/use allowances for each metadata query,without a default.
/// ```compile_fail
/// let limits:sc_core::construction::SlashSpreadReferenceQueryLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadReferenceQueryLimits {
    candidates: usize,
    source_edges: usize,
    uses: usize,
}
impl SlashSpreadReferenceQueryLimits {
    /// Select complete N candidates,R original source occurrences and M contour occurrences.
    #[must_use]
    pub const fn new(candidates: usize, source_edges: usize, uses: usize) -> Self {
        Self {
            candidates,
            source_edges,
            uses,
        }
    }
    /// Complete candidate allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete original source occurrence allowance,including repetitions.
    #[must_use]
    pub const fn source_edges(self) -> usize {
        self.source_edges
    }
    /// Complete contour occurrence allowance,including repetitions.
    #[must_use]
    pub const fn uses(self) -> usize {
        self.uses
    }
}
/// Complete count/quota or exact original source-frame/scope refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadReferenceQueryRefusal {
    /// Conservative 2N+1 match slots cannot be counted.
    CountOverflow,
    /// Complete candidate population exceeds policy.
    Candidates {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original source occurrences exceed policy.
    SourceEdges {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete contour occurrences exceed policy.
    Uses {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Identity is in no declared original source frame;aliases are not silently followed.
    SourceEdge,
    /// An external cut/provenance frame is known,but this parameter is outside its declared domains.
    SourcePosition,
}
/// Actual whole contour proof,owned original anchor and borrowed explicit policy survive refusal.
pub struct SlashSpreadReferenceQueryError<'a> {
    contours: &'a AdmittedSlashSpreadContours<'a>,
    anchor: EdgeAnchor,
    limits: &'a SlashSpreadReferenceQueryLimits,
    refusal: SlashSpreadReferenceQueryRefusal,
}
impl<'a> SlashSpreadReferenceQueryError<'a> {
    /// Actual whole admitted contour/source context.
    #[must_use]
    pub const fn contours(&self) -> &'a AdmittedSlashSpreadContours<'a> {
        self.contours
    }
    /// Exact original anchor value owned unchanged by this error.
    #[must_use]
    pub const fn anchor(&self) -> &EdgeAnchor {
        &self.anchor
    }
    /// Actual original caller-owned query policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadReferenceQueryLimits {
        self.limits
    }
    /// First whole-resource or exact source/scope cause.
    #[must_use]
    pub const fn refusal(&self) -> SlashSpreadReferenceQueryRefusal {
        self.refusal
    }
}
impl fmt::Debug for SlashSpreadReferenceQueryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadReferenceQueryError")
            .field("anchor", &self.anchor)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadReferenceQueryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread source eligibility: {:?}", self.refusal)
    }
}
impl std::error::Error for SlashSpreadReferenceQueryError<'_> {}

fn contains(anchor: &EdgeAnchor, range: &DirectedRange) -> bool {
    anchor.edge == range.range.edge()
        && anchor.param >= range.range.from()
        && anchor.param <= range.range.to()
}
impl<'s> AdmittedSlashSpreadContours<'s> {
    /// Query exact declared source frames under complete N/R/M quotas before scanning fields.
    /// Original Piece frames cover their full unit interval;external cut/provenance frames cover
    /// only their declared domains. A gap within that scope is a valid metadata outcome.
    /// No allocation,ledger fold,alias substitution,output Param or physical point resolution occurs.
    /// # Errors
    /// Count overflow,N/R/M quota,unknown source edge,then external source position. No partial list.
    pub fn query_source_reference<'a>(
        &'a self,
        anchor: EdgeAnchor,
        limits: &'a SlashSpreadReferenceQueryLimits,
    ) -> Result<SlashSpreadReferenceQuery<'a>, SlashSpreadReferenceQueryError<'a>>
    where
        's: 'a,
    {
        let contours: &AdmittedSlashSpreadContours<'a> = self;
        let refuse = |refusal| SlashSpreadReferenceQueryError {
            contours,
            anchor,
            limits,
            refusal,
        };
        let n = contours.edges().manifest().candidates().len();
        let r = contours.source_edge_count();
        let m = contours.uses().len();
        let maximum_match_slots = n
            .checked_mul(2)
            .and_then(|n| n.checked_add(1))
            .ok_or_else(|| refuse(SlashSpreadReferenceQueryRefusal::CountOverflow))?;
        if n > limits.candidates {
            return Err(refuse(SlashSpreadReferenceQueryRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if r > limits.source_edges {
            return Err(refuse(SlashSpreadReferenceQueryRefusal::SourceEdges {
                required: r,
                allowed: limits.source_edges,
            }));
        }
        if m > limits.uses {
            return Err(refuse(SlashSpreadReferenceQueryRefusal::Uses {
                required: m,
                allowed: limits.uses,
            }));
        }
        let in_piece = contours
            .source_piece()
            .edges()
            .any(|edge| edge.edge == anchor.edge);
        let cut = contours.edges().bound_roles().intent().input().cut();
        let mut known = in_piece || anchor.edge == cut.range.edge();
        let mut in_scope = in_piece || contains(&anchor, cut);
        for output in contours.edges().outputs() {
            if let SlashSpreadEdgeOrigin::Source { range, .. } = output.candidate().origin() {
                known |= range.range.edge() == anchor.edge;
                in_scope |= contains(&anchor, range);
            }
        }
        if !known {
            return Err(refuse(SlashSpreadReferenceQueryRefusal::SourceEdge));
        }
        if !in_scope {
            return Err(refuse(SlashSpreadReferenceQueryRefusal::SourcePosition));
        }
        let retained = contours.uses().any(|usage| {
            usage
                .retained()
                .is_some_and(|edge| edge.edge == anchor.edge)
        });
        Ok(SlashSpreadReferenceQuery {
            contours,
            anchor,
            limits,
            retained,
            maximum_match_slots,
        })
    }
}
/// Complete declared eligibility classification,not unique geometric/world-space correspondence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadReferenceMatchKind {
    /// Exactly one declared eligibility match,without geometric or current-reference proof.
    Unique,
    /// Multiple source/retained/bridge endpoint matches;none is silently selected.
    Ambiguous,
    /// No match within a known declared source scope;no repair is silently discharged.
    Gap,
}
/// Declared bridge endpoint in the candidate edge's own frame,not a computed output Param.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadBridgeEndpoint {
    /// Start of the bridge as explicitly directed by its provenance.
    Start,
    /// End of the bridge as explicitly directed by its provenance.
    End,
}
/// Inspection data;only the private match wrapper certifies association with a particular query.
#[derive(Clone, Copy, Debug)]
pub enum SlashSpreadReferenceOrigin<'a> {
    /// The exact original identity is retained;all original use evidence remains separately available.
    RetainedOriginal,
    /// An actual candidate's original positive source domain contains the query.
    Source(SlashSpreadEdge<'a>),
    /// An authentic bridge candidate declares a bank endpoint at the exact original queried cut bound.
    BridgeEndpoint {
        /// Authentic original bridge candidate view.
        output: SlashSpreadEdge<'a>,
        /// Actual stationary/moving bank occurrence.
        bank: SlashSpreadSourceMotion,
        /// Declared bridge start/end occurrence.
        endpoint: SlashSpreadBridgeEndpoint,
    },
}
/// Original query/context with all metadata matches available without query-path allocation.
/// ```compile_fail
/// fn escape(contours:sc_core::construction::AdmittedSlashSpreadContours<'_>,anchor:sc_core::ontology::EdgeAnchor)
///     ->sc_core::construction::SlashSpreadReferenceQuery<'static> {
///     contours.query_source_reference(anchor,&sc_core::construction::SlashSpreadReferenceQueryLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SlashSpreadReferenceQuery<'a> {
    contours: &'a AdmittedSlashSpreadContours<'a>,
    anchor: EdgeAnchor,
    limits: &'a SlashSpreadReferenceQueryLimits,
    retained: bool,
    maximum_match_slots: usize,
}
impl<'a> SlashSpreadReferenceQuery<'a> {
    /// Actual complete admitted contour/source context.
    #[must_use]
    pub const fn contours(&self) -> &'a AdmittedSlashSpreadContours<'a> {
        self.contours
    }
    /// Exact original anchor value owned unchanged by this query.
    #[must_use]
    pub const fn anchor(&self) -> &EdgeAnchor {
        &self.anchor
    }
    /// Actual borrowed original query policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadReferenceQueryLimits {
        self.limits
    }
    /// Conservative 2N+1 potential match slots,not actual matches or allocation bytes.
    #[must_use]
    pub const fn maximum_match_slots(&self) -> usize {
        self.maximum_match_slots
    }
    /// Count every declared match,without a prefix or physical point equivalence assumption.
    #[must_use]
    pub fn kind(&self) -> SlashSpreadReferenceMatchKind {
        match self.matches().count() {
            0 => SlashSpreadReferenceMatchKind::Gap,
            1 => SlashSpreadReferenceMatchKind::Unique,
            _ => SlashSpreadReferenceMatchKind::Ambiguous,
        }
    }
    /// Retained original once,then candidates in original order. Each bridge emits both endpoint
    /// bank occurrences in declared output-direction order;source candidates emit once despite
    /// repeated contour uses. No coordinate/name/amount-based coalescing or mapped Param exists here.
    pub fn matches(&self) -> impl DoubleEndedIterator<Item = SlashSpreadReferenceMatch<'_>> {
        let retained = self.retained.then_some(SlashSpreadReferenceMatch {
            query: self,
            origin: SlashSpreadReferenceOrigin::RetainedOriginal,
        });
        retained
            .into_iter()
            .chain(self.contours.edges().outputs().flat_map(move |output| {
                let make = |origin| {
                    Some(SlashSpreadReferenceMatch {
                        query: self,
                        origin,
                    })
                };
                match output.candidate().origin() {
                    SlashSpreadEdgeOrigin::Source { range, .. }
                        if contains(&self.anchor, range) =>
                    {
                        [make(SlashSpreadReferenceOrigin::Source(output)), None]
                    }
                    SlashSpreadEdgeOrigin::StraightBridge { end, direction } => {
                        let cut = self
                            .contours
                            .edges()
                            .bound_roles()
                            .intent()
                            .input()
                            .cut()
                            .range;
                        let at = match end {
                            SlashSpreadCutEnd::Lower => cut.from(),
                            SlashSpreadCutEnd::Upper => cut.to(),
                        };
                        if self.anchor.edge != cut.edge() || self.anchor.param != at {
                            [None, None]
                        } else {
                            let (first, second) = match direction {
                                SlashSpreadBridgeDirection::StationaryToMoving => (
                                    SlashSpreadSourceMotion::Stationary,
                                    SlashSpreadSourceMotion::Moving,
                                ),
                                SlashSpreadBridgeDirection::MovingToStationary => (
                                    SlashSpreadSourceMotion::Moving,
                                    SlashSpreadSourceMotion::Stationary,
                                ),
                            };
                            [
                                make(SlashSpreadReferenceOrigin::BridgeEndpoint {
                                    output,
                                    bank: first,
                                    endpoint: SlashSpreadBridgeEndpoint::Start,
                                }),
                                make(SlashSpreadReferenceOrigin::BridgeEndpoint {
                                    output,
                                    bank: second,
                                    endpoint: SlashSpreadBridgeEndpoint::End,
                                }),
                            ]
                        }
                    }
                    _ => [None, None],
                }
                .into_iter()
                .flatten()
            }))
    }
}
/// Authentic declared-source match with original context and candidate fields;not a resolved point.
/// ```compile_fail
/// fn reparent<'a>(mut matched:sc_core::construction::SlashSpreadReferenceMatch<'a>,other:&'a sc_core::construction::SlashSpreadReferenceQuery<'a>) {
///     matched.query=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SlashSpreadReferenceMatch<'a> {
    query: &'a SlashSpreadReferenceQuery<'a>,
    origin: SlashSpreadReferenceOrigin<'a>,
}
impl<'a> SlashSpreadReferenceMatch<'a> {
    /// Actual whole original query and complete contour/source context.
    #[must_use]
    pub const fn query(self) -> &'a SlashSpreadReferenceQuery<'a> {
        self.query
    }
    /// Actual eligibility source and authentic underlying candidate when present.
    #[must_use]
    pub const fn origin(self) -> SlashSpreadReferenceOrigin<'a> {
        self.origin
    }
    /// Original retained identity or explicit logical unissued candidate reference.
    #[must_use]
    pub fn edge(self) -> EdgeRef {
        match self.origin {
            SlashSpreadReferenceOrigin::RetainedOriginal => self.query.anchor.edge,
            SlashSpreadReferenceOrigin::Source(output)
            | SlashSpreadReferenceOrigin::BridgeEndpoint { output, .. } => output.logical_output(),
        }
    }
    /// All actual original retained occurrences for a retained-identity match;other kinds return none.
    /// No first occurrence is selected as a substitute for the complete evidence.
    pub fn retained_uses(self) -> impl DoubleEndedIterator<Item = SlashSpreadContourUse<'a>> {
        let retained = matches!(self.origin, SlashSpreadReferenceOrigin::RetainedOriginal);
        self.query.contours.uses().filter(move |usage| {
            retained
                && usage
                    .retained()
                    .is_some_and(|edge| edge.edge == self.query.anchor.edge)
        })
    }
}
