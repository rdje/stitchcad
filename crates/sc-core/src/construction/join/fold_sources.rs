//! Complete original fold-field actions,not accepted source omission or physical fold correspondence.
use super::{JoinSourceOccurrence, JoinSourceOccurrenceLocation, RequiredJoinFold};
use crate::ontology::{EdgeRef, EntityId, SeamSideId};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit action for one guarded original fold declaration;no default or inferred source discard.
/// ```compile_fail
/// let intent:sc_core::construction::JoinFoldSourceIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinFoldSourceIntent {
    /// Attribute the requested result fold to this exact original side/edge frame.
    /// Actual whole-axis/state/copy correspondence still requires the parent evidence request.
    Retain,
    /// Explicitly request omission of this original declaration,without accepting its justification.
    Omit {
        /// Actual authored source/consumer justification request,no implied evidence truth or approval.
        evidence_source: EntityId,
    },
}
/// Original A/B fold snapshot selector,expected persistent field and explicit Retain/Omit request.
/// ```compile_fail
/// use sc_core::{construction::*,ontology::*};
/// JoinFoldSourceAssignment::new(SeamSideId::A,0,PointRef::new(EntityId::from_bits(1),LocalTag::FIRST),JoinFoldSourceIntent::Retain);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinFoldSourceAssignment {
    side: SeamSideId,
    index: usize,
    expected: EdgeRef,
    intent: JoinFoldSourceIntent,
}
impl JoinFoldSourceAssignment {
    /// Retain all original fields;the ordinal is a guarded snapshot selector,not a geometry identity.
    #[must_use]
    pub const fn new(
        side: SeamSideId,
        index: usize,
        expected: EdgeRef,
        intent: JoinFoldSourceIntent,
    ) -> Self {
        Self {
            side,
            index,
            expected,
            intent,
        }
    }
    /// Explicit original A/B source role.
    #[must_use]
    pub const fn side(&self) -> SeamSideId {
        self.side
    }
    /// Original fold field snapshot selector.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }
    /// Actual expected original persistent fold field,not a replacement geometry reference.
    #[must_use]
    pub const fn expected(&self) -> &EdgeRef {
        &self.expected
    }
    /// Actual authored retention/omission request.
    #[must_use]
    pub const fn intent(&self) -> &JoinFoldSourceIntent {
        &self.intent
    }
    /// Actual authored omission evidence id field,or None for Retain.
    #[must_use]
    pub const fn evidence_source(&self) -> Option<&EntityId> {
        match &self.intent {
            JoinFoldSourceIntent::Retain => None,
            JoinFoldSourceIntent::Omit { evidence_source } => Some(evidence_source),
        }
    }
}
/// Whole original source R,raw action M and checked2M workspace allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinFoldSourceLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinFoldSourceLimits {
    sources: usize,
    assignments: usize,
    workspace_entries: usize,
}
impl JoinFoldSourceLimits {
    /// Admit all original source fields before fold counts,then all rows/checked2M before fields.
    #[must_use]
    pub const fn new(sources: usize, assignments: usize, workspace_entries: usize) -> Self {
        Self {
            sources,
            assignments,
            workspace_entries,
        }
    }
    /// Every actual original A/B field,including separate fold declarations.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Every original raw fold action.
    #[must_use]
    pub const fn assignments(self) -> usize {
        self.assignments
    }
    /// Complete2M source-index/action associations,not bytes or earlier immutable contexts.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole arithmetic/resource,owner/count or original source/action/result attribution cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinFoldSourceRefusal {
    /// Complete original fold count cannot be represented.
    SourceCountOverflow,
    /// Complete2M cannot be represented.
    WorkspaceOverflow,
    /// Whole original field population exceeds policy.
    Sources {
        /// Complete R.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole raw action population exceeds policy.
    Assignments {
        /// Complete M.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete2M new entries exceed policy.
    Workspace {
        /// Complete required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw manifest names another operation.
    Operation,
    /// One unique actual source action is required per original fold declaration.
    Count {
        /// Complete actual original fold fields.
        expected: usize,
        /// Raw action population.
        actual: usize,
    },
    /// An original A/B fold selector repeats.
    DuplicateSource {
        /// First original row.
        first: usize,
        /// Repeated original row.
        second: usize,
    },
    /// Selector names no actual original fold field.
    MissingSource {
        /// Original row.
        index: usize,
    },
    /// Expected persistent field differs from the actual original fold field.
    SourceField {
        /// Original row.
        index: usize,
    },
    /// Retain requires an actual explicitly requested result fold,not NoFold.
    ResultNoFold {
        /// Original row.
        index: usize,
    },
    /// Actual result output belongs to another original A/B source frame.
    SourceSide {
        /// Original row.
        index: usize,
    },
    /// Actual result output is attributed to another persistent source edge.
    SourceEdge {
        /// Original row.
        index: usize,
    },
}
/// Whole original manifest,result/source parents,policy and first cause remain borrowed.
pub struct JoinFoldSourceError<'a> {
    manifest: &'a JoinFoldSourceManifest,
    fold: &'a RequiredJoinFold<'a>,
    limits: JoinFoldSourceLimits,
    refusal: JoinFoldSourceRefusal,
}
impl<'a> JoinFoldSourceError<'a> {
    /// Actual complete original source actions,including refused suffixes.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinFoldSourceManifest {
        self.manifest
    }
    /// Actual whole desired fold/construction/cut/source parents.
    #[must_use]
    pub const fn fold(&self) -> &'a RequiredJoinFold<'a> {
        self.fold
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinFoldSourceLimits {
        self.limits
    }
    /// First whole resource,owner/count or original row cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinFoldSourceRefusal {
        self.refusal
    }
    /// Actual refused original action row;whole resource/owner/count causes None.
    #[must_use]
    pub fn refused_assignment(&self) -> Option<&'a JoinFoldSourceAssignment> {
        use JoinFoldSourceRefusal as R;
        let index = match self.refusal {
            R::DuplicateSource { second, .. } => second,
            R::MissingSource { index }
            | R::SourceField { index }
            | R::ResultNoFold { index }
            | R::SourceSide { index }
            | R::SourceEdge { index } => index,
            _ => return None,
        };
        self.manifest.assignments.get(index)
    }
    /// Actual original fold occurrence for a row cause,if that snapshot selector exists.
    #[must_use]
    pub fn refused_source(&self) -> Option<JoinSourceOccurrence<'a>> {
        let row = self.refused_assignment()?;
        source_occurrence(self.fold, row.side, row.index)
    }
}
impl fmt::Debug for JoinFoldSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinFoldSourceError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinFoldSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join original fold actions: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinFoldSourceError<'_> {}
fn sources<'a>(fold: &'a RequiredJoinFold<'a>) -> &'a super::BoundJoinSourcePieces<'a> {
    fold.construction()
        .targets()
        .coverage()
        .groups()
        .sources()
        .uses()
        .connectivity()
        .contours()
        .directions()
        .coverage()
        .edges()
        .identity()
        .roles()
        .banks()
        .sources()
}
fn source_occurrence<'a>(
    fold: &'a RequiredJoinFold<'a>,
    side: SeamSideId,
    index: usize,
) -> Option<JoinSourceOccurrence<'a>> {
    sources(fold).occurrence(side, JoinSourceOccurrenceLocation::Fold(index))
}
/// Explicit operation and complete ordered guarded original fold actions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinFoldSourceManifest {
    operation: EntityId,
    assignments: Vec<JoinFoldSourceAssignment>,
}
impl JoinFoldSourceManifest {
    /// Retain every original action/guard/order,without defaults,deduplication or source replacement.
    #[must_use]
    pub const fn new(operation: EntityId, assignments: Vec<JoinFoldSourceAssignment>) -> Self {
        Self {
            operation,
            assignments,
        }
    }
    /// Original explicit operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete raw original actions.
    #[must_use]
    pub fn assignments(&self) -> &[JoinFoldSourceAssignment] {
        &self.assignments
    }
    /// Admit full R before original fold counts,then full M/checked2M before fields/allocation.
    /// Operation/exact complete count precede original duplicate/missing/expected-field guards.
    /// Retain requires actual result fold attribution to that same original A/B and persistent edge;
    /// Omit retains its actual independent evidence request,including zero/reused ids,not truth.
    /// Unique actual field membership plus exact count covers every original A/B fold declaration.
    /// Work O(M log(M+2)+1),2M new index/association entries,no provider/geometry/evidence call.
    /// Earlier immutable contexts/storage remain. Neither a source subrange nor Retain proves whole
    /// physical fold correspondence;NoFold/Omit cannot grant deletion/state/copy/consumer authority.
    /// # Errors
    /// Whole R/M,count/2M arithmetic,workspace,operation,count;first original selector/guard/action/
    /// result frame cause. No accepted prefix,source substitute or unrequested fold is returned.
    pub fn admit<'a>(
        &'a self,
        fold: &'a RequiredJoinFold<'a>,
        limits: JoinFoldSourceLimits,
    ) -> Result<DeclaredJoinFoldSources<'a>, JoinFoldSourceError<'a>> {
        use JoinFoldSourceRefusal as R;
        let refuse = |refusal| JoinFoldSourceError {
            manifest: self,
            fold,
            limits,
            refusal,
        };
        let source = sources(fold);
        let r = source.occurrence_count();
        let m = self.assignments.len();
        if r > limits.sources {
            return Err(refuse(R::Sources {
                required: r,
                allowed: limits.sources,
            }));
        }
        if m > limits.assignments {
            return Err(refuse(R::Assignments {
                required: m,
                allowed: limits.assignments,
            }));
        }
        let k = source
            .piece(SeamSideId::A)
            .definition()
            .fold_edges
            .len()
            .checked_add(source.piece(SeamSideId::B).definition().fold_edges.len())
            .ok_or_else(|| refuse(R::SourceCountOverflow))?;
        let workspace = m
            .checked_mul(2)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation != fold.requirement().operation() {
            return Err(refuse(R::Operation));
        }
        if m != k {
            return Err(refuse(R::Count {
                expected: k,
                actual: m,
            }));
        }
        let mut by_source = BTreeMap::new();
        let mut actions = Vec::with_capacity(m);
        for (index, row) in self.assignments.iter().enumerate() {
            let key = (row.side == SeamSideId::B, row.index);
            if let Some(first) = by_source.insert(key, index) {
                return Err(refuse(R::DuplicateSource {
                    first,
                    second: index,
                }));
            }
            let original = source_occurrence(fold, row.side, row.index)
                .ok_or_else(|| refuse(R::MissingSource { index }))?;
            if row.expected != *original.field().edge() {
                return Err(refuse(R::SourceField { index }));
            }
            if matches!(row.intent, JoinFoldSourceIntent::Retain) {
                let boundary = fold
                    .boundary()
                    .ok_or_else(|| refuse(R::ResultNoFold { index }))?;
                let output = boundary.output().source();
                if output.candidate().side() != row.side {
                    return Err(refuse(R::SourceSide { index }));
                }
                if output.source().edge() != *original.field().edge() {
                    return Err(refuse(R::SourceEdge { index }));
                }
            }
            actions.push(FoldAssociation {
                index,
                row,
                original,
            });
        }
        Ok(DeclaredJoinFoldSources {
            manifest: self,
            fold,
            limits,
            workspace,
            by_source,
            actions,
        })
    }
}
#[derive(Debug)]
struct FoldAssociation<'a> {
    index: usize,
    row: &'a JoinFoldSourceAssignment,
    original: JoinSourceOccurrence<'a>,
}
/// Complete native original fold actions;omission/correspondence/state/copy/consumer truth outstanding.
/// ```compile_fail
/// fn forge<'a>(mut proof:sc_core::construction::DeclaredJoinFoldSources<'a>,other:&'a sc_core::construction::JoinFoldSourceManifest) {proof.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinFoldSourceManifest,fold:sc_core::construction::RequiredJoinFold<'_>)->sc_core::construction::DeclaredJoinFoldSources<'static> {
///     raw.admit(&fold,sc_core::construction::JoinFoldSourceLimits::new(0,0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct DeclaredJoinFoldSources<'a> {
    manifest: &'a JoinFoldSourceManifest,
    fold: &'a RequiredJoinFold<'a>,
    limits: JoinFoldSourceLimits,
    workspace: usize,
    by_source: BTreeMap<(bool, usize), usize>,
    actions: Vec<FoldAssociation<'a>>,
}
impl<'a> DeclaredJoinFoldSources<'a> {
    /// Actual whole original source-action manifest.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinFoldSourceManifest {
        self.manifest
    }
    /// Actual whole explicit desired fold and construction/source/cut parents.
    #[must_use]
    pub const fn fold(&self) -> &'a RequiredJoinFold<'a> {
        self.fold
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinFoldSourceLimits {
        self.limits
    }
    /// Complete2M new index/action entries,not bytes or earlier immutable contexts.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete original fold action count,including independent Omit requests.
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.actions.len()
    }
    /// Authentic original actions in authored order,and reverse.
    pub fn assignments(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinFoldSourceAction<'_>> + DoubleEndedIterator {
        self.actions
            .iter()
            .map(move |association| JoinFoldSourceAction {
                context: self,
                association,
            })
    }
    /// Authentic original action by actual original A/B fold selector.
    #[must_use]
    pub fn assignment(&self, side: SeamSideId, index: usize) -> Option<JoinFoldSourceAction<'_>> {
        let i = self.by_source.get(&(side == SeamSideId::B, index))?;
        self.actions
            .get(*i)
            .map(|association| JoinFoldSourceAction {
                context: self,
                association,
            })
    }
}
/// One authentic original action,actual fold occurrence/Piece/copy and unchanged evidence field.
#[derive(Clone, Copy, Debug)]
pub struct JoinFoldSourceAction<'a> {
    context: &'a DeclaredJoinFoldSources<'a>,
    association: &'a FoldAssociation<'a>,
}
impl<'a> JoinFoldSourceAction<'a> {
    /// Actual whole native fold/source/action context.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinFoldSources<'a> {
        self.context
    }
    /// Original raw action row ordinal,not a geometry identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original raw guarded source/intent fields.
    #[must_use]
    pub const fn assignment(self) -> &'a JoinFoldSourceAssignment {
        self.association.row
    }
    /// Authentic actual original fold occurrence,including Piece/copy/source fields.
    #[must_use]
    pub const fn source(self) -> JoinSourceOccurrence<'a> {
        self.association.original
    }
    /// Actual original persistent fold field,not the authored expected-field copy.
    #[must_use]
    pub const fn source_field(self) -> &'a EdgeRef {
        self.association.original.field().edge()
    }
    /// Actual authored Retain/Omit request.
    #[must_use]
    pub const fn intent(self) -> &'a JoinFoldSourceIntent {
        self.association.row.intent()
    }
    /// Actual authored Omit evidence id field,or None for Retain;no accepted evidence implied.
    #[must_use]
    pub const fn evidence_source(self) -> Option<&'a EntityId> {
        self.association.row.evidence_source()
    }
}
