//! Bounded actual local source validation;global Design and original-coordinate evidence stay separate.
use super::NotchSourceAdmission;
use crate::ontology::{
    AnchorError, EntityId, OrphaningEdit, RangeCoverage, RangeCoverageError, RangeIssue,
    RangePortion, SeamSideId, SewingError, UniquePointOutcome, UniquePointResolution,
};
use core::fmt;
use sc_units::UnitError;
mod current;

/// Actual range-query location;indices describe authored traversal,never substitute for edge IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotchSourceRange {
    /// Complete original source side.
    Side(SeamSideId),
    /// Whole held Piece edge in boundary,hole,construction order.
    PieceEdge {
        /// Actual Piece owner.
        piece: EntityId,
        /// Flattened original Piece edge index;the query error retains the actual held range.
        index: usize,
    },
}

/// Concrete local source refusal;bounded evidence is retained without constructing legacy trees.
#[derive(Debug, PartialEq, Eq)]
pub enum NotchSourceValidationIssue {
    /// Original CutPlan,identity,copy,ownership,self-overlap,stop or distribution cause.
    Structural(Box<SewingError>),
    /// Original landmark owner is missing from the supplied actual Piece registry.
    MissingLandmarkPiece {
        /// Actual landmark identity.
        landmark: EntityId,
        /// Actual held Piece owner.
        piece: EntityId,
    },
    /// Actual landmark has no unique current position;first choice/repair/input cause stays fixed.
    LandmarkPoint {
        /// Actual landmark identity,including an unused entry.
        landmark: EntityId,
        /// Actual held reference and bounded current outcome.
        resolution: Box<UniquePointResolution>,
    },
    /// Actual unique landmark resolves outside its Piece's surviving intervals.
    LandmarkOutsidePiece {
        /// Actual landmark identity.
        landmark: EntityId,
        /// Original held anchor and owner cause.
        issue: Box<AnchorError>,
    },
    /// Complete original source endpoint requires choice/repair or has an unknown source.
    Endpoint {
        /// Actual source side.
        side: SeamSideId,
        /// True for the original upper endpoint;false for lower.
        upper: bool,
        /// Actual original reference and bounded current outcome,not a folded choice tree.
        resolution: Box<UniquePointResolution>,
    },
    /// Complete positive source interval has actual unknown/deleted/gapped/numeric repair evidence.
    Coverage {
        /// Actual original source side.
        side: SeamSideId,
        /// Complete admitted positive-interval evidence;no lost portion is sampled away.
        evidence: RangeCoverage,
    },
    /// Original per-range admission refusal,with actual owner/query location.
    CoverageLimit {
        /// Actual source side or Piece edge.
        reference: NotchSourceRange,
        /// Original held range and quota/count cause.
        issue: RangeCoverageError,
    },
    /// Conservative interval-cache or metadata-index cardinality cannot be represented.
    DerivedBoundOverflow {
        /// Actual complete admitted dependency count.
        dependency_items: usize,
        /// Caller-selected peak portions per range query.
        range_portions: usize,
    },
    /// Appending a metadata-index row would exceed its pre-admitted bound or count representation.
    DerivedIndexItems {
        /// Actual identity whose row would be added.
        owner: EntityId,
        /// Already retained local index rows.
        retained: usize,
        /// One next row,checked before allocation.
        requested: usize,
        /// Conservative complete metadata-row bound.
        limit: usize,
    },
    /// Appending a resolved ownership interval would exceed the pre-admitted cache bound.
    DerivedItems {
        /// Actual Piece edge being cached.
        reference: NotchSourceRange,
        /// Actual retained ownership-interval population.
        retained: usize,
        /// One requested interval.
        requested: usize,
        /// Checked complete interval-cache bound.
        limit: usize,
    },
}
#[derive(Debug, PartialEq, Eq)]
struct Failure {
    source_span: EntityId,
    issue: NotchSourceValidationIssue,
    numeric: Option<UnitError>,
}
/// Original source identity and actual local validation cause,with no accepted partial source proof.
#[derive(Debug, PartialEq, Eq)]
pub struct NotchSourceValidationError(Box<Failure>);
impl NotchSourceValidationError {
    /// Actual original source,not a fabricated graph identity.
    #[must_use]
    pub const fn source_span(&self) -> EntityId {
        self.0.source_span
    }
    /// Complete actual refusal,evidence and diagnostic location.
    #[must_use]
    pub const fn issue(&self) -> &NotchSourceValidationIssue {
        &self.0.issue
    }
}
impl fmt::Display for NotchSourceValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "notch ease source {} current validation: {:?}",
            self.source_span(),
            self.issue()
        )
    }
}
impl std::error::Error for NotchSourceValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.issue() {
            NotchSourceValidationIssue::Structural(issue) => Some(issue.as_ref()),
            NotchSourceValidationIssue::LandmarkOutsidePiece { landmark: _, issue } => {
                Some(issue.as_ref())
            }
            NotchSourceValidationIssue::CoverageLimit {
                reference: _,
                issue,
            } => Some(issue),
            NotchSourceValidationIssue::MissingLandmarkPiece {
                landmark: _,
                piece: _,
            }
            | NotchSourceValidationIssue::LandmarkPoint {
                landmark: _,
                resolution: _,
            }
            | NotchSourceValidationIssue::Endpoint {
                side: _,
                upper: _,
                resolution: _,
            }
            | NotchSourceValidationIssue::Coverage {
                side: _,
                evidence: _,
            }
            | NotchSourceValidationIssue::DerivedBoundOverflow {
                dependency_items: _,
                range_portions: _,
            }
            | NotchSourceValidationIssue::DerivedIndexItems {
                owner: _,
                retained: _,
                requested: _,
                limit: _,
            }
            | NotchSourceValidationIssue::DerivedItems {
                reference: _,
                retained: _,
                requested: _,
                limit: _,
            } => self
                .0
                .numeric
                .as_ref()
                .map(|cause| cause as &dyn std::error::Error),
        }
    }
}
fn numeric_edit(edit: &OrphaningEdit) -> Option<UnitError> {
    match edit {
        OrphaningEdit::RecomputationFailed {
            operation: _,
            edge: _,
            cause,
        } => Some(*cause),
        OrphaningEdit::Deleted {
            operation: _,
            edge: _,
        }
        | OrphaningEdit::OffsetAmbiguous {
            operation: _,
            source: _,
        }
        | OrphaningEdit::OffsetUnmapped {
            operation: _,
            source: _,
        } => None,
    }
}
fn numeric_point(resolution: &UniquePointResolution) -> Option<UnitError> {
    match resolution.outcome() {
        UniquePointOutcome::Repair {
            cause,
            candidates: _,
        } => numeric_edit(cause),
        UniquePointOutcome::Resolved(_)
        | UniquePointOutcome::UnknownSource
        | UniquePointOutcome::JournalEntries {
            required: _,
            limit: _,
        }
        | UniquePointOutcome::NestedItems {
            operation: _,
            collection: _,
            consumed: _,
            requested: _,
            limit: _,
        }
        | UniquePointOutcome::CountOverflow {
            operation: _,
            collection: _,
            consumed: _,
            requested: _,
        }
        | UniquePointOutcome::SplitChoice {
            operation: _,
            source: _,
            at: _,
            first: _,
            second: _,
            direction: _,
        } => None,
    }
}
fn failure(source_span: EntityId, issue: NotchSourceValidationIssue) -> NotchSourceValidationError {
    let numeric = match &issue {
        NotchSourceValidationIssue::LandmarkPoint {
            landmark: _,
            resolution,
        }
        | NotchSourceValidationIssue::Endpoint {
            side: _,
            upper: _,
            resolution,
        } => numeric_point(resolution),
        NotchSourceValidationIssue::Coverage { side: _, evidence } => {
            evidence.portions().iter().find_map(|part| {
                let RangePortion::Unresolved(task) = part else {
                    return None;
                };
                match task.issue() {
                    RangeIssue::Orphaned(edit) => numeric_edit(&edit),
                    RangeIssue::UnknownSource => None,
                }
            })
        }
        NotchSourceValidationIssue::Structural(_)
        | NotchSourceValidationIssue::MissingLandmarkPiece {
            landmark: _,
            piece: _,
        }
        | NotchSourceValidationIssue::LandmarkOutsidePiece {
            landmark: _,
            issue: _,
        }
        | NotchSourceValidationIssue::CoverageLimit {
            reference: _,
            issue: _,
        }
        | NotchSourceValidationIssue::DerivedBoundOverflow {
            dependency_items: _,
            range_portions: _,
        }
        | NotchSourceValidationIssue::DerivedIndexItems {
            owner: _,
            retained: _,
            requested: _,
            limit: _,
        }
        | NotchSourceValidationIssue::DerivedItems {
            reference: _,
            retained: _,
            requested: _,
            limit: _,
        } => None,
    };
    NotchSourceValidationError(Box::new(Failure {
        source_span,
        issue,
        numeric,
    }))
}

/// Sealed local original-source structural proof,borrowing the exact immutable admitted inputs.
/// Current copy/Piece/landmark/source checks passed with bounded queries. Original-coordinate
/// uniqueness,amount evaluation,global Design/other graph members and physical readiness are separate.
/// Interval-cache/index bounds count logical entries,not allocator overhead,RSS or CPU time.
///
/// ```compile_fail
/// use sc_core::ontology::{NotchSourceAdmission, NotchSourceValidation};
/// fn forge<'a>(admission: NotchSourceAdmission<'a>) -> NotchSourceValidation<'a> {
///     NotchSourceValidation { admission, interval_item_bound: 1, index_item_bound: 1,
///         interval_item_count: 1, index_item_count: 1 }
/// }
/// ```
#[derive(Clone, Copy)]
pub struct NotchSourceValidation<'a> {
    admission: NotchSourceAdmission<'a>,
    interval_item_bound: usize,
    index_item_bound: usize,
    interval_item_count: usize,
    index_item_count: usize,
}
impl fmt::Debug for NotchSourceValidation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            admission,
            interval_item_bound,
            index_item_bound,
            interval_item_count,
            index_item_count,
        } = self;
        f.debug_struct("NotchSourceValidation")
            .field("source_span", &admission.source().span())
            .field("interval_item_bound", interval_item_bound)
            .field("index_item_bound", index_item_bound)
            .field("interval_item_count", interval_item_count)
            .field("index_item_count", index_item_count)
            .finish()
    }
}
impl<'a> NotchSourceValidation<'a> {
    /// Actual complete borrowed inputs and explicit selected limits.
    #[must_use]
    pub const fn admission(self) -> NotchSourceAdmission<'a> {
        self.admission
    }
    /// Conservative complete ownership-interval cache bound N*B,admitted before indexes/queries.
    #[must_use]
    pub const fn interval_item_bound(self) -> usize {
        self.interval_item_bound
    }
    /// Conservative metadata-row bound 3N+1,including fixed CutPlan validation arrays and local indexes.
    #[must_use]
    pub const fn index_item_bound(self) -> usize {
        self.index_item_bound
    }
    /// Completed validation's local index row count;indexes are discarded before returning.
    #[must_use]
    pub const fn index_item_count(self) -> usize {
        self.index_item_count
    }
    /// Actual resolved ownership intervals cached across all supplied Pieces during validation.
    #[must_use]
    pub const fn interval_item_count(self) -> usize {
        self.interval_item_count
    }
}
impl<'a> NotchSourceAdmission<'a> {
    /// Validate the actual complete local original source using bounded current/coverage queries.
    ///
    /// Checks derived cardinality,actual current CutPlan and local identities,all Piece ownership
    /// intervals,every supplied registry landmark,then source sides/stops/distribution. Piece gaps elsewhere
    /// are not silently turned into whole-Piece rejection:local membership uses surviving intervals.
    /// No graph identity,formula/profile value or original notch coordinate is invented.
    ///
    /// # Errors
    /// Returns first actual bound/plan/identity/range/landmark/source cause,without a partial proof.
    pub fn validate_sources(self) -> Result<NotchSourceValidation<'a>, NotchSourceValidationError> {
        current::validate(self)
    }
}
