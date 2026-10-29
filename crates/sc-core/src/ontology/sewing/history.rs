//! Identity-only history binding; current sources and actual command derivation stay separate.
use super::{EntityId, SeamSpan, SeamSpanHistory, SewingGraph};
use crate::ontology::SeamSpanHistoryState;
use core::fmt;

/// Original graph/history identity inconsistency, before a bound graph can escape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamSpanHistoryBindingError {
    /// Existing immutable history cannot be replaced on this graph snapshot.
    AlreadyBound,
    /// An actual live graph member is absent from the supplied history.
    MissingLiveSpan {
        /// Actual graph member identity.
        span: EntityId,
        /// Original authored graph member position.
        index: usize,
    },
    /// An actual live graph member is retired in the supplied history.
    LiveSpanRetired {
        /// Actual graph member identity.
        span: EntityId,
        /// Original authored graph member position.
        index: usize,
        /// Actual history retirement record.
        split_entry: usize,
        /// Original immediate children, without automatic replacement.
        children: [EntityId; 2],
    },
    /// A retained original ease source has no supplied history identity.
    MissingSource {
        /// Exact canonical source identity.
        source_span: EntityId,
        /// First-consuming-source position.
        index: usize,
    },
    /// A retained original ease source is still live in the supplied history.
    SourceNotRetired {
        /// Exact canonical source identity.
        source_span: EntityId,
        /// First-consuming-source position.
        index: usize,
        /// Actual supplied birth record.
        birth_entry: usize,
    },
    /// A retained source identity belongs to a different history family from its consumer.
    UnrelatedSource {
        /// Actual consuming live graph member.
        span: EntityId,
        /// Exact original ease-source identity.
        source_span: EntityId,
        /// Actual authored consuming-span position.
        index: usize,
    },
    /// A history leaf has no actual live graph member.
    UnrepresentedLiveSpan {
        /// Original supplied leaf identity.
        span: EntityId,
        /// Actual supplied birth record.
        birth_entry: usize,
    },
    /// A supplied span identity collides with the containing graph itself.
    GraphIdentityCollision {
        /// Original repeated identity.
        span: EntityId,
        /// Actual supplied birth record.
        birth_entry: usize,
    },
}
impl fmt::Display for SeamSpanHistoryBindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sewing graph span history: {self:?}")
    }
}
impl std::error::Error for SeamSpanHistoryBindingError {}

/// Borrowed actual live graph member or explicit immediate retirement evidence.
/// This is held identity evidence, not current copy/range, physical or mutation authority.
#[derive(Clone, Copy)]
pub enum SewingSpanLookup<'a> {
    /// The exact immutable member of this graph; no equal-body replacement is manufactured.
    Live(&'a SeamSpan),
    /// The exact original history record; old references are not moved to a chosen child.
    RetiredSplit {
        /// Actual original birth record.
        birth_entry: usize,
        /// Actual original retirement record.
        split_entry: usize,
        /// Immediate A-ordered children, even if one has since been retired.
        children: &'a [EntityId; 2],
    },
}
impl fmt::Debug for SewingSpanLookup<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Live(span) => f.debug_tuple("Live").field(&span.id()).finish(),
            Self::RetiredSplit {
                birth_entry,
                split_entry,
                children,
            } => f
                .debug_struct("RetiredSplit")
                .field("birth_entry", birth_entry)
                .field("split_entry", split_entry)
                .field("children", children)
                .finish(),
        }
    }
}

impl SewingGraph {
    /// Bind supplied identity history to this exact immutable graph, without current-source work.
    /// Every graph span must be a history leaf; every history leaf must be an actual graph span.
    /// Every retained ease-source identity must be a retired ancestor of its consuming spans.
    /// Check graph members, sources, consumer ancestry, then supplied births in their original
    /// orders. No index, clone or descendant tree is allocated.
    /// Global Design ownership and current Piece/copy/landmark namespaces still require their
    /// existing validators. This does not prove actual cuts or execution of the supplied history.
    /// ```
    /// use sc_core::ontology::{SeamSpanHistory, SeamSpanHistoryBindingError, SewingGraph};
    /// fn bind(graph: SewingGraph, history: SeamSpanHistory)
    ///     -> Result<SewingGraph, SeamSpanHistoryBindingError> {
    ///     graph.with_span_history(history)
    /// }
    /// ```
    /// # Errors
    /// Returns the first original identity inconsistency or refusal to replace existing history.
    pub fn with_span_history(
        mut self,
        history: SeamSpanHistory,
    ) -> Result<Self, SeamSpanHistoryBindingError> {
        use SeamSpanHistoryBindingError as Error;
        if self.history.is_some() {
            return Err(Error::AlreadyBound);
        }
        for (index, span) in self.spans.iter().enumerate() {
            match history.state(span.id()) {
                None => {
                    return Err(Error::MissingLiveSpan {
                        span: span.id(),
                        index,
                    });
                }
                Some(SeamSpanHistoryState::RetiredSplit {
                    split_entry,
                    children,
                    ..
                }) => {
                    return Err(Error::LiveSpanRetired {
                        span: span.id(),
                        index,
                        split_entry,
                        children: *children,
                    });
                }
                Some(SeamSpanHistoryState::Live { .. }) => {}
            }
        }
        for (index, source) in self.ease_sources.iter().enumerate() {
            match history.state(source.span()) {
                None => {
                    return Err(Error::MissingSource {
                        source_span: source.span(),
                        index,
                    });
                }
                Some(SeamSpanHistoryState::Live { birth_entry }) => {
                    return Err(Error::SourceNotRetired {
                        source_span: source.span(),
                        index,
                        birth_entry,
                    });
                }
                Some(SeamSpanHistoryState::RetiredSplit { .. }) => {}
            }
        }
        for (index, span) in self.spans.iter().enumerate() {
            if let Some(portion) = span.definition().ease.portion() {
                let source_span = portion.source().span();
                if !history.is_ancestor(source_span, span.id()) {
                    return Err(Error::UnrelatedSource {
                        span: span.id(),
                        source_span,
                        index,
                    });
                }
            }
        }
        for (span, birth_entry, _) in history.births() {
            if span == self.id {
                return Err(Error::GraphIdentityCollision { span, birth_entry });
            }
            if matches!(history.state(span), Some(SeamSpanHistoryState::Live { .. }))
                && !self.spans.iter().any(|member| member.id() == span)
            {
                return Err(Error::UnrepresentedLiveSpan { span, birth_entry });
            }
        }
        self.history = Some(history);
        Ok(self)
    }

    /// Exact immutable attached history; legacy held graphs return none rather than invented edits.
    /// ```compile_fail
    /// fn replace(graph: &mut sc_core::ontology::SewingGraph) {
    ///     graph.history = None;
    /// }
    /// ```
    #[must_use]
    pub const fn span_history(&self) -> Option<&SeamSpanHistory> {
        self.history.as_ref()
    }

    /// Borrow an actual live member or both immediate children of an explicitly retired parent.
    /// Retained source provenance without attached history does not invent a retirement record.
    /// Lookup performs no current-source query, reference repair, child selection or allocation.
    /// ```compile_fail
    /// fn escape(graph: &sc_core::ontology::SewingGraph, id: sc_core::ontology::EntityId)
    ///     -> sc_core::ontology::SewingSpanLookup<'static> {
    ///     graph.lookup_span(id).unwrap()
    /// }
    /// ```
    #[must_use]
    pub fn lookup_span(&self, identity: EntityId) -> Option<SewingSpanLookup<'_>> {
        if let Some(span) = self.spans.iter().find(|span| span.id() == identity) {
            return Some(SewingSpanLookup::Live(span));
        }
        match self.history.as_ref()?.state(identity)? {
            SeamSpanHistoryState::RetiredSplit {
                birth_entry,
                split_entry,
                children,
            } => Some(SewingSpanLookup::RetiredSplit {
                birth_entry,
                split_entry,
                children,
            }),
            SeamSpanHistoryState::Live { .. } => None,
        }
    }
}
