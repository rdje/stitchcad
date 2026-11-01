//! Explicit result physical-copy requests,not issued identities,complete CutPlan or source pairing.
use super::DeclaredJoinFoldSources;
use crate::ontology::{CopyOrientation, CutCopy, CutCopyDefinition, EntityId, Piece};
use core::fmt;
use std::collections::BTreeMap;

/// Whole original copy C,original Piece I,result request Q and C+I+Q index allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinResultCopyLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinResultCopyLimits {
    copies: usize,
    pieces: usize,
    results: usize,
    workspace_entries: usize,
}
impl JoinResultCopyLimits {
    /// Admit every actual original inventory member and every raw result request before fields.
    /// Workspace counts three complete namespace indexes,not bytes or earlier immutable contexts.
    #[must_use]
    pub const fn new(
        copies: usize,
        pieces: usize,
        results: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            copies,
            pieces,
            results,
            workspace_entries,
        }
    }
    /// Every original physical copy,including unrelated and unselected source copies.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Every original Piece,including unrelated and companion members.
    #[must_use]
    pub const fn pieces(self) -> usize {
        self.pieces
    }
    /// Every raw requested result copy.
    #[must_use]
    pub const fn results(self) -> usize {
        self.results
    }
    /// Complete C+I+Q logical index entries,not allocator bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete arithmetic/resource,owner/population or original result request identity cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinResultCopyRefusal {
    /// Complete C+I+Q cannot be represented.
    WorkspaceOverflow,
    /// Complete actual original copy population exceeds policy.
    Copies {
        /// Actual whole C.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete actual original Piece population exceeds policy.
    Pieces {
        /// Actual whole I.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete raw requested result population exceeds policy.
    Results {
        /// Actual whole Q.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete C+I+Q index storage exceeds policy.
    Workspace {
        /// Required logical entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw operation differs from the authentic original join operation.
    Operation,
    /// Canonical authored result quantity cannot be represented as a request-list count.
    QuantityRange {
        /// Actual original quantity.
        quantity: u32,
    },
    /// Exactly the canonical requested quantity of result-copy declarations is required.
    Count {
        /// Actual canonical result quantity.
        expected: usize,
        /// Complete raw result requests.
        actual: usize,
    },
    /// Raw requested copy names another pattern Piece.
    Piece {
        /// Original request ordinal.
        index: usize,
    },
    /// New result copy ids must be distinct from each other.
    Duplicate {
        /// First original request.
        first: usize,
        /// Repeated original request.
        second: usize,
    },
    /// New copy id equals the actual join operation.
    OperationIdentity {
        /// Original request.
        index: usize,
    },
    /// New copy id equals the actual declared result Piece.
    ResultPieceIdentity {
        /// Original request.
        index: usize,
    },
    /// New copy id equals an actual original Piece,including unrelated members.
    OriginalPieceIdentity {
        /// Original request.
        index: usize,
    },
    /// New copy id equals an actual original physical copy,including consumed candidates.
    OriginalCopyIdentity {
        /// Original request.
        index: usize,
    },
}
/// Whole original proposal/fold/construction/inventory context,policy and actual first cause.
pub struct JoinResultCopyError<'a> {
    manifest: &'a JoinResultCopyManifest,
    folds: &'a DeclaredJoinFoldSources<'a>,
    limits: JoinResultCopyLimits,
    refusal: JoinResultCopyRefusal,
    original_piece: Option<&'a Piece>,
    original_copy: Option<&'a CutCopy>,
}
impl<'a> JoinResultCopyError<'a> {
    /// Actual complete original result requests,including invalid suffixes.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinResultCopyManifest {
        self.manifest
    }
    /// Actual whole original fold/construction/source/identity/inventory parents.
    #[must_use]
    pub const fn folds(&self) -> &'a DeclaredJoinFoldSources<'a> {
        self.folds
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinResultCopyLimits {
        self.limits
    }
    /// First whole resource,owner/population or original request cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinResultCopyRefusal {
        self.refusal
    }
    /// Actual original refused request for field causes;whole resource/owner/count causes None.
    #[must_use]
    pub fn refused_copy(&self) -> Option<&'a CutCopyDefinition> {
        use JoinResultCopyRefusal as R;
        let index = match self.refusal {
            R::Duplicate { second, .. } => second,
            R::Piece { index }
            | R::OperationIdentity { index }
            | R::ResultPieceIdentity { index }
            | R::OriginalPieceIdentity { index }
            | R::OriginalCopyIdentity { index } => index,
            _ => return None,
        };
        self.manifest.copies.get(index)
    }
    /// Actual original Piece only for OriginalPieceIdentity;no reconstructed replacement.
    #[must_use]
    pub const fn original_piece(&self) -> Option<&'a Piece> {
        self.original_piece
    }
    /// Actual original copy only for OriginalCopyIdentity;no recreated equal-content copy.
    #[must_use]
    pub const fn original_copy(&self) -> Option<&'a CutCopy> {
        self.original_copy
    }
}
impl fmt::Debug for JoinResultCopyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinResultCopyError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinResultCopyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join result copy requests: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinResultCopyError<'_> {}
/// Original operation and explicitly identified result-copy requests;no expansion or id synthesis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinResultCopyManifest {
    operation: EntityId,
    copies: Vec<CutCopyDefinition>,
}
impl JoinResultCopyManifest {
    /// Retain every original id/Piece/orientation field and order,including invalid/repeated requests.
    #[must_use]
    pub const fn new(operation: EntityId, copies: Vec<CutCopyDefinition>) -> Self {
        Self { operation, copies }
    }
    /// Actual original operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original result-copy declarations,not a born/validated result CutPlan.
    #[must_use]
    pub fn copies(&self) -> &[CutCopyDefinition] {
        &self.copies
    }
    /// Admit complete original C/I,raw Q and checked C+I+Q before fields/allocation. Operation and
    /// exact canonical authored quantity precede original request Piece/duplicate/local-id checks.
    /// Each requested copy explicitly names the declared result Piece and its orientation. New ids
    /// cannot reuse actual operation/result Piece/original Piece/original copy ids;unrelated members
    /// count and remain actual borrowed loans. These are scoped exclusions,not global namespace,
    /// kind,currentness,freshness or issuance proofs. All other id/evidence/geometry obligations remain.
    /// Work O(C log(C+2)+I log(I+2)+Q log(Q+C+I+2)+1),C+I+Q new index entries. Earlier immutable
    /// source/fold/construction/population policies remain. Original-copy actions,exact no-fork A+B
    /// pairing,orientation/mirroring and full candidate inventory/CutPlan checks remain mandatory.
    /// There is no provider,evidence,geometry,copy transform,allocation of persistent ids or effect.
    /// # Errors
    /// Whole C/I/Q,workspace arithmetic/quota,operation,quantity range/count;first original Piece,
    /// duplicate or scoped namespace collision. No accepted prefix or implicit copies are returned.
    pub fn admit<'a>(
        &'a self,
        folds: &'a DeclaredJoinFoldSources<'a>,
        limits: JoinResultCopyLimits,
    ) -> Result<DeclaredJoinResultCopies<'a>, JoinResultCopyError<'a>> {
        use JoinResultCopyRefusal as R;
        let refuse = |refusal| JoinResultCopyError {
            manifest: self,
            folds,
            limits,
            refusal,
            original_piece: None,
            original_copy: None,
        };
        let edges = folds
            .fold()
            .construction()
            .targets()
            .coverage()
            .groups()
            .sources()
            .uses()
            .connectivity()
            .contours()
            .directions()
            .coverage()
            .edges();
        let identity = edges.identity();
        let sources = identity.roles().banks().sources();
        let population = sources.population();
        let inventory = population.metadata().splice().inventory();
        let plan = inventory.source().plan();
        let original_pieces = inventory.inventory().pieces();
        let c = plan.copies().len();
        let i = original_pieces.len();
        let q = self.copies.len();
        if c > limits.copies {
            return Err(refuse(R::Copies {
                required: c,
                allowed: limits.copies,
            }));
        }
        if i > limits.pieces {
            return Err(refuse(R::Pieces {
                required: i,
                allowed: limits.pieces,
            }));
        }
        if q > limits.results {
            return Err(refuse(R::Results {
                required: q,
                allowed: limits.results,
            }));
        }
        let workspace = c
            .checked_add(i)
            .and_then(|n| n.checked_add(q))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation != folds.fold().requirement().operation() {
            return Err(refuse(R::Operation));
        }
        let quantity = *population.quantity();
        let expected =
            usize::try_from(quantity).map_err(|_| refuse(R::QuantityRange { quantity }))?;
        if q != expected {
            return Err(refuse(R::Count {
                expected,
                actual: q,
            }));
        }
        let by_original_copy: BTreeMap<_, _> =
            plan.copies().iter().map(|copy| (copy.id(), copy)).collect();
        let by_original_piece: BTreeMap<_, _> = original_pieces
            .iter()
            .map(|piece| (piece.id(), piece))
            .collect();
        let mut by_result = BTreeMap::new();
        for (index, copy) in self.copies.iter().enumerate() {
            if copy.piece != *identity.piece_id() {
                return Err(refuse(R::Piece { index }));
            }
            if let Some((first, _)) = by_result.insert(copy.id, (index, copy)) {
                return Err(refuse(R::Duplicate {
                    first,
                    second: index,
                }));
            }
            if copy.id == self.operation {
                return Err(refuse(R::OperationIdentity { index }));
            }
            if copy.id == *identity.piece_id() {
                return Err(refuse(R::ResultPieceIdentity { index }));
            }
            if let Some(original) = by_original_piece.get(&copy.id) {
                let mut error = refuse(R::OriginalPieceIdentity { index });
                error.original_piece = Some(*original);
                return Err(error);
            }
            if let Some(original) = by_original_copy.get(&copy.id) {
                let mut error = refuse(R::OriginalCopyIdentity { index });
                error.original_copy = Some(*original);
                return Err(error);
            }
        }
        Ok(DeclaredJoinResultCopies {
            manifest: self,
            folds,
            limits,
            workspace,
            by_original_copy,
            by_original_piece,
            by_result,
        })
    }
}
/// Authentic complete explicit result copy requests;no source consumption,pairing or canonical plan.
/// ```compile_fail
/// fn forge<'a>(mut copies:sc_core::construction::DeclaredJoinResultCopies<'a>,other:&'a sc_core::construction::JoinResultCopyManifest) {copies.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinResultCopyManifest,folds:sc_core::construction::DeclaredJoinFoldSources<'_>)->sc_core::construction::DeclaredJoinResultCopies<'static> {
///     raw.admit(&folds,sc_core::construction::JoinResultCopyLimits::new(0,0,0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct DeclaredJoinResultCopies<'a> {
    manifest: &'a JoinResultCopyManifest,
    folds: &'a DeclaredJoinFoldSources<'a>,
    limits: JoinResultCopyLimits,
    workspace: usize,
    by_original_copy: BTreeMap<EntityId, &'a CutCopy>,
    by_original_piece: BTreeMap<EntityId, &'a Piece>,
    by_result: BTreeMap<EntityId, (usize, &'a CutCopyDefinition)>,
}
impl<'a> DeclaredJoinResultCopies<'a> {
    /// Actual whole original result-copy requests.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinResultCopyManifest {
        self.manifest
    }
    /// Actual complete source fold/construction/identity/inventory parents.
    #[must_use]
    pub const fn folds(&self) -> &'a DeclaredJoinFoldSources<'a> {
        self.folds
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinResultCopyLimits {
        self.limits
    }
    /// Complete C+I+Q new index entries,not bytes or earlier immutable contexts.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete actual original physical-copy population.
    #[must_use]
    pub fn source_copy_count(&self) -> usize {
        self.by_original_copy.len()
    }
    /// Complete actual original Piece population.
    #[must_use]
    pub fn source_piece_count(&self) -> usize {
        self.by_original_piece.len()
    }
    /// Complete explicitly requested result-copy population,matching canonical authored quantity only.
    #[must_use]
    pub fn copy_count(&self) -> usize {
        self.manifest.copies.len()
    }
    /// Actual original copy by stable identity,including unrelated/unselected copies.
    #[must_use]
    pub fn source_copy(&self, id: EntityId) -> Option<&'a CutCopy> {
        self.by_original_copy.get(&id).copied()
    }
    /// Actual original Piece by stable identity,including unrelated/companion members.
    #[must_use]
    pub fn source_piece(&self, id: EntityId) -> Option<&'a Piece> {
        self.by_original_piece.get(&id).copied()
    }
    /// Authentic requested copies in original order,and reverse;no implicit expansion or transform.
    pub fn copies(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinResultCopyRequest<'_>> + DoubleEndedIterator {
        self.manifest
            .copies
            .iter()
            .enumerate()
            .map(move |(index, definition)| JoinResultCopyRequest {
                context: self,
                index,
                definition,
            })
    }
    /// Authentic explicit requested copy by persistent identity,not list position.
    #[must_use]
    pub fn copy(&self, id: EntityId) -> Option<JoinResultCopyRequest<'_>> {
        self.by_result
            .get(&id)
            .map(|(index, definition)| JoinResultCopyRequest {
                context: self,
                index: *index,
                definition,
            })
    }
}
/// One authentic explicit result physical-copy request,not a born current CutCopy.
#[derive(Clone, Copy, Debug)]
pub struct JoinResultCopyRequest<'a> {
    context: &'a DeclaredJoinResultCopies<'a>,
    index: usize,
    definition: &'a CutCopyDefinition,
}
impl<'a> JoinResultCopyRequest<'a> {
    /// Actual complete original requested-copy owner.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinResultCopies<'a> {
        self.context
    }
    /// Original declaration ordinal,not a synthesized copy identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original id/Piece/orientation fields.
    #[must_use]
    pub const fn definition(self) -> &'a CutCopyDefinition {
        self.definition
    }
    /// Actual caller-supplied stable result-copy identity,still unissued.
    #[must_use]
    pub const fn id(self) -> EntityId {
        self.definition.id
    }
    /// Actual original declared result Piece identity field.
    #[must_use]
    pub const fn piece_id(self) -> &'a EntityId {
        &self.definition.piece
    }
    /// Actual original authored/reflected request;no achieved physical transform is inferred.
    #[must_use]
    pub const fn orientation(self) -> &'a CopyOrientation {
        &self.definition.orientation
    }
}
