//! Exact held-source cut binding; current-world validation and paired effects remain separate.
use super::SplitSeamSpan;
use crate::ontology::{
    EdgeAnchor, EdgeRange, EdgeRef, EntityId, RangeSplitError, SeamDirection, SeamSide, SeamSideId,
    SeamSpan, SewingGraph,
};
use core::fmt;

/// Original target, frame or strict-interior refusal, checked in target/A/B order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandSeamSplitBindingError {
    /// The requested correspondence is absent from the supplied graph.
    MissingSpan {
        /// Original requested correspondence identity.
        span: EntityId,
    },
    /// A cut names a different source edge from the actual named side.
    SourceFrame {
        /// Actual requested correspondence identity.
        span: EntityId,
        /// Original named side; the other side never substitutes for it.
        side: SeamSideId,
        /// Held source edge of the actual span side.
        expected: EdgeRef,
        /// Original requested edge and exact parameter.
        cut: EdgeAnchor,
    },
    /// A same-frame cut is not strictly inside the actual held side interval.
    Interior {
        /// Actual correspondence identity.
        span: EntityId,
        /// Original named side.
        side: SeamSideId,
        /// Original interval/cut refusal, also retained as the standard error source.
        issue: RangeSplitError,
    },
}

impl fmt::Display for CommandSeamSplitBindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSpan { span } => {
                write!(
                    f,
                    "seam split span {span} is absent from the supplied graph"
                )
            }
            Self::SourceFrame {
                span,
                side,
                expected,
                cut,
            } => write!(
                f,
                "seam split span {span} side {side:?} needs source {expected:?}, got {cut:?}"
            ),
            Self::Interior { span, side, issue } => {
                write!(f, "seam split span {span} side {side:?}: {issue}")
            }
        }
    }
}

impl std::error::Error for CommandSeamSplitBindingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Interior {
                span: _,
                side: _,
                issue,
            } => Some(issue),
            Self::MissingSpan { span: _ }
            | Self::SourceFrame {
                span: _,
                side: _,
                expected: _,
                cut: _,
            } => None,
        }
    }
}

/// One declared interval pairing projected from a source binding, without a new span identity.
/// Both physical-copy identities, source frames and endpoint direction remain original metadata.
/// Ease, stops, lineage, currentness and physical validity require their separate execution checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandSeamSplitPair {
    source_span: EntityId,
    a: SeamSide,
    b: SeamSide,
    direction: SeamDirection,
}

impl CommandSeamSplitPair {
    /// Actual original parent identity, not an identity allocated for a child span.
    #[must_use]
    pub const fn source_span(&self) -> EntityId {
        self.source_span
    }

    /// Exact named side's original physical copy and projected source interval.
    #[must_use]
    pub const fn side(&self, side: SeamSideId) -> SeamSide {
        match side {
            SeamSideId::A => self.a,
            SeamSideId::B => self.b,
        }
    }

    /// Original endpoint correspondence, independent of physical-copy reflection.
    #[must_use]
    pub const fn direction(&self) -> SeamDirection {
        self.direction
    }
}

/// Borrowed original request and actual held span, with each side's exact lower/upper intervals.
/// Copy identities, direction, ease and stops remain on the original span. This is source-frame
/// evidence, not current-world validation, physical correspondence or an execution capability.
///
/// A caller cannot forge the successful binding, even with correctly typed candidate intervals:
/// ```compile_fail
/// use sc_core::{command::{CommandSeamSplitBinding, SplitSeamSpan},
///     ontology::{EdgeRange, SeamSpan}};
/// fn forge<'a>(request: &'a SplitSeamSpan, span: &'a SeamSpan,
///     ranges: [EdgeRange; 2]) -> CommandSeamSplitBinding<'a> {
///     CommandSeamSplitBinding { request, span, side_a: ranges, side_b: ranges }
/// }
/// ```
/// Its original owners must outlive the binding:
/// ```compile_fail
/// use sc_core::{command::{CommandSeamSplitBinding, SplitSeamSpan}, ontology::SewingGraph};
/// fn escape<'a>(request: &'a SplitSeamSpan, graph: &'a SewingGraph)
///     -> CommandSeamSplitBinding<'static> {
///     request.bind_source_cuts(graph).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct CommandSeamSplitBinding<'a> {
    request: &'a SplitSeamSpan,
    span: &'a SeamSpan,
    side_a: [EdgeRange; 2],
    side_b: [EdgeRange; 2],
}

impl fmt::Debug for CommandSeamSplitBinding<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            request: _,
            span,
            side_a,
            side_b,
        } = self;
        f.debug_struct("CommandSeamSplitBinding")
            .field("span", &span.id())
            .field("side_a", side_a)
            .field("side_b", side_b)
            .finish_non_exhaustive()
    }
}

impl<'a> CommandSeamSplitBinding<'a> {
    /// Exact original request, without retagging or replacing its source anchors.
    #[must_use]
    pub const fn request(&self) -> &'a SplitSeamSpan {
        self.request
    }

    /// Actual original graph member; its copy, direction, ease and stop data are not cloned.
    #[must_use]
    pub const fn span(&self) -> &'a SeamSpan {
        self.span
    }

    /// Lower then upper source-frame intervals for this named side, independently of seam direction.
    /// In particular, opposite endpoint correspondence does not reorder these ascending bounds.
    #[must_use]
    pub const fn ranges(&self, side: SeamSideId) -> &[EdgeRange; 2] {
        match side {
            SeamSideId::A => &self.side_a,
            SeamSideId::B => &self.side_b,
        }
    }

    /// Pair intervals in side-A lower/upper order using the original endpoint correspondence.
    /// Same direction pairs lower A with lower B; opposite pairs lower A with upper B. Every
    /// interval retains its own ascending source bounds and original physical-copy identity.
    /// These fixed projections allocate no IDs and supply no child ease, stops or live graph edits.
    #[must_use]
    pub const fn paired_intervals(&self) -> [CommandSeamSplitPair; 2] {
        let source = self.span.definition();
        let [first_a, second_a] = self.side_a;
        let [lower_b, upper_b] = self.side_b;
        let (first_b, second_b) = match source.direction {
            SeamDirection::Same => (lower_b, upper_b),
            SeamDirection::Opposite => (upper_b, lower_b),
        };
        [
            CommandSeamSplitPair {
                source_span: self.span.id(),
                a: SeamSide {
                    copy: source.a.copy,
                    range: first_a,
                },
                b: SeamSide {
                    copy: source.b.copy,
                    range: first_b,
                },
                direction: source.direction,
            },
            CommandSeamSplitPair {
                source_span: self.span.id(),
                a: SeamSide {
                    copy: source.a.copy,
                    range: second_a,
                },
                b: SeamSide {
                    copy: source.b.copy,
                    range: second_b,
                },
                direction: source.direction,
            },
        ]
    }
}

impl SplitSeamSpan {
    /// Bind exact named cuts to the original correspondence in this supplied immutable graph.
    /// Lookup is in authored graph order; each cut must use its actual side's held edge frame.
    /// The operation retains borrows and fixed interval arrays without editing or allocating IDs.
    /// Current copy/range resolution, paired correspondence, ease/stop conservation, lineage,
    /// authority and transaction publication remain separate execution obligations.
    ///
    /// ```
    /// use sc_core::{command::{CommandSeamSplitBinding, CommandSeamSplitBindingError, SplitSeamSpan},
    ///     ontology::{SeamSideId, SewingGraph}};
    /// fn inspect_existing<'a>(request: &'a SplitSeamSpan, graph: &'a SewingGraph)
    ///     -> Result<CommandSeamSplitBinding<'a>, CommandSeamSplitBindingError> {
    ///     let binding = request.bind_source_cuts(graph)?;
    ///     let [lower, upper] = binding.ranges(SeamSideId::A);
    ///     assert_eq!((lower.to(), upper.from()), (request.cut(SeamSideId::A).param,
    ///         request.cut(SeamSideId::A).param));
    ///     let [first, second] = binding.paired_intervals();
    ///     assert_eq!(first.source_span(), second.source_span());
    ///     assert_eq!(first.direction(), binding.span().definition().direction);
    ///     Ok(binding)
    /// }
    /// ```
    ///
    /// # Errors
    /// Refuses a missing target first, then side A before B. On each side a wrong frame precedes
    /// strict-interior validation, retaining the original concrete range refusal in its source chain.
    pub fn bind_source_cuts<'a>(
        &'a self,
        graph: &'a SewingGraph,
    ) -> Result<CommandSeamSplitBinding<'a>, CommandSeamSplitBindingError> {
        let span = graph
            .spans()
            .iter()
            .find(|span| span.id() == self.span())
            .ok_or(CommandSeamSplitBindingError::MissingSpan { span: self.span() })?;
        let bind_side = |side| {
            let range = match side {
                SeamSideId::A => span.definition().a.range,
                SeamSideId::B => span.definition().b.range,
            };
            let cut = self.cut(side);
            if cut.edge != range.edge() {
                return Err(CommandSeamSplitBindingError::SourceFrame {
                    span: span.id(),
                    side,
                    expected: range.edge(),
                    cut,
                });
            }
            range
                .split_at(cut.param)
                .map_err(|issue| CommandSeamSplitBindingError::Interior {
                    span: span.id(),
                    side,
                    issue,
                })
        };
        let side_a = bind_side(SeamSideId::A)?;
        let side_b = bind_side(SeamSideId::B)?;
        Ok(CommandSeamSplitBinding {
            request: self,
            span,
            side_a,
            side_b,
        })
    }
}
