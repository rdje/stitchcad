//! Retained original ease declarations and source intervals; graph installation is separate.
use super::{
    DeclaredEase, EdgeAnchor, EdgeRef, EntityId, RangeSplitError, SeamDirection, SeamSide,
    SeamSideId, SeamSpan,
};
use core::fmt;
use std::sync::Arc;

/// Immutable original ease source captured from an actual structurally validated span.
/// Original physical copies, source domains, direction and symbolic/distribution intent survive
/// subsequent interval partitions. This snapshot does not assert currentness or realized ease.
///
/// Its provenance cannot be forged through public fields:
/// ```compile_fail
/// use sc_core::ontology::{DeclaredEase, EntityId, SeamDirection, SeamEaseSource, SeamSide};
/// fn forge(span: EntityId, a: SeamSide, b: SeamSide, direction: SeamDirection,
///     ease: DeclaredEase) -> SeamEaseSource {
///     SeamEaseSource { span, a, b, direction, ease }
/// }
/// ```
#[derive(PartialEq, Eq)]
pub struct SeamEaseSource {
    span: EntityId,
    a: SeamSide,
    b: SeamSide,
    direction: SeamDirection,
    ease: DeclaredEase,
}

impl fmt::Debug for SeamEaseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            span,
            a,
            b,
            direction,
            ease: _,
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
    /// Clones the actual declaration once,including any weighted regions. Subsequent portion
    /// clones/splits share that same source;callers must admit this capture cost before bus use.
    /// This snapshot preserves held references without fresh topology or source-value validation.
    /// Canonical graph ownership/resource integration and quantity queries remain separate stages.
    ///
    /// The retained source may outlive the caller's original graph:
    /// ```
    /// use sc_core::ontology::{SeamEasePortion, SeamSpan};
    /// fn retain(actual: &SeamSpan) -> SeamEasePortion {
    ///     let portion = actual.retain_ease_source();
    ///     assert_eq!(portion.source().span(), actual.id());
    ///     assert_eq!(portion.source().declaration(), &actual.definition().ease);
    ///     portion
    /// }
    /// ```
    #[must_use]
    pub fn retain_ease_source(&self) -> SeamEasePortion {
        let original = self.definition();
        SeamEasePortion {
            source: Arc::new(SeamEaseSource {
                span: original.id,
                a: original.a,
                b: original.b,
                direction: original.direction,
                ease: original.ease.clone(),
            }),
            a: original.a,
            b: original.b,
        }
    }
}
