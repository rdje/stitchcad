//! Canonical declared or retained ease,with immutable original sources and source intervals.
use super::{
    DeclaredEase, EdgeAnchor, EdgeRange, EdgeRef, EntityId, RangeSplitError, SeamDirection,
    SeamSide, SeamSideId, SeamSpan, StopLandmark,
};
use core::fmt;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Canonical ease intent: an authored declaration or an interval of a retained original source.
/// A retained portion has no independently authored child amount or distribution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeamEase {
    /// Original signed/symbolic amount and allocation intent.
    Declared(DeclaredEase),
    /// Source intervals sharing their immutable original declaration.
    Portion(SeamEasePortion),
}
impl From<DeclaredEase> for SeamEase {
    fn from(value: DeclaredEase) -> Self {
        Self::Declared(value)
    }
}
impl From<SeamEasePortion> for SeamEase {
    fn from(value: SeamEasePortion) -> Self {
        Self::Portion(value)
    }
}
impl SeamEase {
    /// Original source declaration; for a portion this is the full parent,not a child quantity.
    #[must_use]
    pub fn declaration(&self) -> &DeclaredEase {
        match self {
            Self::Declared(value) => value,
            Self::Portion(value) => value.source().declaration(),
        }
    }
    /// Exact retained source intervals,when this span inherits its ease.
    #[must_use]
    pub const fn portion(&self) -> Option<&SeamEasePortion> {
        match self {
            Self::Declared(_) => None,
            Self::Portion(value) => Some(value),
        }
    }
    /// Edit an authored declaration in an editable input;retained source data stays immutable.
    pub fn declared_mut(&mut self) -> Option<&mut DeclaredEase> {
        match self {
            Self::Declared(value) => Some(value),
            Self::Portion(_) => None,
        }
    }
}

/// Actual side or direction mismatch between a span and its retained source interval portion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamEaseBindingIssue {
    /// Two consuming spans claim overlapping intervals of one original allocation source.
    Overlap {
        /// Named original side.
        side: SeamSideId,
        /// Previously admitted consuming span.
        first_span: EntityId,
        /// Actual previous original-frame interval.
        first: EdgeRange,
        /// Actual overlapping new original-frame interval.
        interval: EdgeRange,
    },
    /// A side differs from the portion's original physical copy or exact domain.
    Side {
        /// Named side.
        side: SeamSideId,
        /// Actual immutable portion side.
        expected: SeamSide,
        /// Actual supplied span side.
        actual: SeamSide,
    },
    /// The span changes its retained source's endpoint correspondence.
    Direction {
        /// Original source direction.
        expected: SeamDirection,
        /// Actual supplied span direction.
        actual: SeamDirection,
    },
}
impl fmt::Display for SeamEaseBindingIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overlap {
                side,
                first_span,
                first,
                interval,
            } => write!(
                f,
                "retained ease {side:?} interval {interval:?} overlaps span {first_span} interval {first:?}"
            ),
            Self::Side {
                side,
                expected,
                actual,
            } => write!(
                f,
                "retained ease {side:?} needs {expected:?}, got {actual:?}"
            ),
            Self::Direction { expected, actual } => write!(
                f,
                "retained ease direction needs {expected:?}, got {actual:?}"
            ),
        }
    }
}
impl std::error::Error for SeamEaseBindingIssue {}

/// Immutable original ease source captured from an actual structurally validated span.
/// Original physical copies, source domains, direction and symbolic/distribution intent survive
/// subsequent interval partitions. This snapshot does not assert currentness or realized ease.
///
/// Its provenance cannot be forged through public fields:
/// ```compile_fail
/// use sc_core::ontology::{DeclaredEase, EntityId, SeamDirection, SeamEaseSource, SeamSide, StopLandmark};
/// fn forge(span: EntityId, a: SeamSide, b: SeamSide, direction: SeamDirection,
///     ease: DeclaredEase, stops: Vec<StopLandmark>) -> SeamEaseSource {
///     SeamEaseSource { span, a, b, direction, ease, stops }
/// }
/// ```
#[derive(PartialEq, Eq)]
pub struct SeamEaseSource {
    span: EntityId,
    a: SeamSide,
    b: SeamSide,
    direction: SeamDirection,
    ease: DeclaredEase,
    stops: Vec<StopLandmark>,
}

impl fmt::Debug for SeamEaseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            span,
            a,
            b,
            direction,
            ease: _,
            stops: _,
        } = self;
        f.debug_struct("SeamEaseSource")
            .field("span", span)
            .field("a", a)
            .field("b", b)
            .field("direction", direction)
            .finish_non_exhaustive()
    }
}

impl SeamEaseSource {
    pub(crate) fn resource_parts(
        &self,
    ) -> (
        EntityId,
        SeamSide,
        SeamSide,
        SeamDirection,
        &DeclaredEase,
        &[StopLandmark],
    ) {
        let Self {
            span,
            a,
            b,
            direction,
            ease,
            stops,
        } = self;
        (*span, *a, *b, *direction, ease, stops)
    }
    /// Original semantic stops,including source notch-boundary membership,in authored order.
    #[must_use]
    pub fn stops(&self) -> &[StopLandmark] {
        &self.stops
    }

    /// Original correspondence identity; no new child or independently issued source identity.
    #[must_use]
    pub const fn span(&self) -> EntityId {
        self.span
    }

    /// Original physical copy and full source domain for the named side.
    #[must_use]
    pub const fn side(&self, side: SeamSideId) -> SeamSide {
        match side {
            SeamSideId::A => self.a,
            SeamSideId::B => self.b,
        }
    }

    /// Original endpoint correspondence, independent of copy reflection.
    #[must_use]
    pub const fn direction(&self) -> SeamDirection {
        self.direction
    }

    /// Original signed/symbolic amount and complete distribution, never a child declaration.
    #[must_use]
    pub const fn declaration(&self) -> &DeclaredEase {
        &self.ease
    }
}

/// Original source-frame or strict-interior refusal while partitioning retained intervals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamEasePortionError {
    /// The named cut uses a different source edge from that actual portion side.
    SourceFrame {
        /// Retained original correspondence identity.
        source_span: EntityId,
        /// Named side, checked A before B.
        side: SeamSideId,
        /// Actual side's held source edge.
        expected: EdgeRef,
        /// Original requested edge and position.
        cut: EdgeAnchor,
    },
    /// Same-frame cut lies outside the strict interior of that actual portion side.
    Interior {
        /// Retained original correspondence identity.
        source_span: EntityId,
        /// Named side.
        side: SeamSideId,
        /// Actual portion interval and requested cut; also the standard error source.
        issue: RangeSplitError,
    },
}

impl fmt::Display for SeamEasePortionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceFrame {
                source_span,
                side,
                expected,
                cut,
            } => write!(
                f,
                "ease source {source_span} portion {side:?} needs {expected:?}, got {cut:?}"
            ),
            Self::Interior {
                source_span,
                side,
                issue,
            } => write!(f, "ease source {source_span} portion {side:?}: {issue}"),
        }
    }
}

impl std::error::Error for SeamEasePortionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Interior {
                source_span: _,
                side: _,
                issue,
            } => Some(issue),
            Self::SourceFrame {
                source_span: _,
                side: _,
                expected: _,
                cut: _,
            } => None,
        }
    }
}

/// Private source intervals sharing one immutable original declaration through repeated splits.
/// Clone shares that same source allocation, without cloning weighted regions. A portion has no
/// child identity or independently authored amount. Partitioning does not validate physical cut
/// correspondence, evaluate a symbolic source or install a new graph span.
///
/// Caller-supplied domains cannot forge this relationship:
/// ```compile_fail
/// use sc_core::ontology::{SeamEasePortion, SeamEaseSource, SeamSide};
/// use std::sync::Arc;
/// fn forge(source: Arc<SeamEaseSource>, a: SeamSide, b: SeamSide) -> SeamEasePortion {
///     SeamEasePortion { source, a, b }
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct SeamEasePortion {
    source: Arc<SeamEaseSource>,
    a: SeamSide,
    b: SeamSide,
}

impl fmt::Debug for SeamEasePortion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { source, a, b } = self;
        f.debug_struct("SeamEasePortion")
            .field("source_span", &source.span())
            .field("a", a)
            .field("b", b)
            .finish_non_exhaustive()
    }
}

impl SeamEasePortion {
    // Graph construction calls this only after complete content/coherence validation. Equal
    // sources become one actual immutable body,not merely one counted identity.
    pub(crate) fn canonicalize_source(
        &mut self,
        sources: &mut BTreeMap<EntityId, Arc<SeamEaseSource>>,
        ordered: &mut Vec<Arc<SeamEaseSource>>,
    ) {
        let Self { source, a: _, b: _ } = self;
        let canonical = sources.entry(source.span()).or_insert_with(|| {
            let shared = Arc::clone(source);
            ordered.push(Arc::clone(&shared));
            shared
        });
        *source = Arc::clone(canonical);
    }
    pub(crate) const fn resource_sides(&self) -> (SeamSide, SeamSide) {
        let Self { source: _, a, b } = self;
        (*a, *b)
    }
    pub(crate) fn shared_source(&self) -> Arc<SeamEaseSource> {
        let Self { source, a: _, b: _ } = self;
        Arc::clone(source)
    }
    /// Original full source/domain/declaration, shared unchanged by every descendant portion.
    #[must_use]
    pub fn source(&self) -> &SeamEaseSource {
        &self.source
    }

    /// This portion's named physical copy and unchanged original-frame interval.
    #[must_use]
    pub const fn side(&self, side: SeamSideId) -> SeamSide {
        match side {
            SeamSideId::A => self.a,
            SeamSideId::B => self.b,
        }
    }

    /// Partition these actual intervals in A-lower/upper order using the retained direction.
    /// Only source-frame and interior checks occur; arbitrary declared cuts remain unevaluated
    /// correspondence inputs. The original ease declaration is not copied into child amounts.
    /// # Errors
    /// Checks A completely before B,frame before interior;retains the original portion refusal.
    pub fn split_at(
        &self,
        a: EdgeAnchor,
        b: EdgeAnchor,
    ) -> Result<[Self; 2], SeamEasePortionError> {
        let split = |side: SeamSideId, cut: EdgeAnchor| {
            let original = self.side(side);
            if cut.edge != original.range.edge() {
                return Err(SeamEasePortionError::SourceFrame {
                    source_span: self.source.span,
                    side,
                    expected: original.range.edge(),
                    cut,
                });
            }
            original
                .range
                .split_at(cut.param)
                .map_err(|issue| SeamEasePortionError::Interior {
                    source_span: self.source.span,
                    side,
                    issue,
                })
        };
        let [first_a, second_a] = split(SeamSideId::A, a)?;
        let [lower_b, upper_b] = split(SeamSideId::B, b)?;
        let (first_b, second_b) = match self.source.direction {
            SeamDirection::Same => (lower_b, upper_b),
            SeamDirection::Opposite => (upper_b, lower_b),
        };
        Ok([
            Self {
                source: Arc::clone(&self.source),
                a: SeamSide {
                    copy: self.a.copy,
                    range: first_a,
                },
                b: SeamSide {
                    copy: self.b.copy,
                    range: first_b,
                },
            },
            Self {
                source: Arc::clone(&self.source),
                a: SeamSide {
                    copy: self.a.copy,
                    range: second_a,
                },
                b: SeamSide {
                    copy: self.b.copy,
                    range: second_b,
                },
            },
        ])
    }
}

impl SeamSpan {
    /// Capture an owned immutable original ease source and whole-domain portion from this span.
    /// Clones an authored declaration and original semantic stops once. Subsequent portion
    /// clones/splits share that same source;callers must admit this capture cost before bus use.
    /// This snapshot preserves held references without fresh topology or source-value validation.
    /// Retained portions keep their existing source;quantity queries and paired effects remain separate stages.
    ///
    /// The retained source may outlive the caller's original graph:
    /// ```
    /// use sc_core::ontology::{SeamEasePortion, SeamSpan};
    /// fn retain(actual: &SeamSpan) -> SeamEasePortion {
    ///     let portion = actual.retain_ease_source();
    ///     assert_eq!(portion.source().span(), actual.id());
    ///     assert_eq!(portion.source().declaration(), actual.definition().ease.declaration());
    ///     portion
    /// }
    /// ```
    #[must_use]
    pub fn retain_ease_source(&self) -> SeamEasePortion {
        let original = self.definition();
        if let SeamEase::Portion(portion) = &original.ease {
            return portion.clone();
        }
        SeamEasePortion {
            source: Arc::new(SeamEaseSource {
                span: original.id,
                a: original.a,
                b: original.b,
                direction: original.direction,
                ease: original.ease.declaration().clone(),
                stops: original.stops.clone(),
            }),
            a: original.a,
            b: original.b,
        }
    }
}
