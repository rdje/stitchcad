//! Explicit proposed result construction uses,not complete original field reconciliation or effects.
use super::{JoinDirectionalOutput, RequiredJoinCutConnectivity};
use crate::ontology::{Direction, EdgeRef, EntityId, LocalTag};
use core::fmt;

/// One explicitly directed construction occurrence. Repeated output tags remain separate uses.
/// ```compile_fail
/// use sc_core::{construction::*,ontology::LocalTag};
/// JoinConstructionUse::new(LocalTag::FIRST,JoinParameterDirection::SameAsSource);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConstructionUse {
    tag: LocalTag,
    direction: Direction,
}
impl JoinConstructionUse {
    /// Retain explicit persistent tag/traversal;infer no direction or semantic ownership.
    #[must_use]
    pub const fn new(tag: LocalTag, direction: Direction) -> Self {
        Self { tag, direction }
    }
    /// Actual original explicit output tag field,not an occurrence ordinal.
    #[must_use]
    pub const fn tag(&self) -> &LocalTag {
        &self.tag
    }
    /// Actual original construction traversal,distinct from source-relative parameter orientation.
    #[must_use]
    pub const fn direction(&self) -> &Direction {
        &self.direction
    }
}
/// Complete actual candidate N,proposed use U and U new association allowances before fields.
/// ```compile_fail
/// let limits:sc_core::construction::JoinConstructionUseLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConstructionUseLimits {
    outputs: usize,
    uses: usize,
    workspace_entries: usize,
}
impl JoinConstructionUseLimits {
    /// Bound every actual candidate and every original repeated construction occurrence before
    /// fields/allocation. Workspace counts U associations,not bytes or earlier caller/context storage.
    #[must_use]
    pub const fn new(outputs: usize, uses: usize, workspace_entries: usize) -> Self {
        Self {
            outputs,
            uses,
            workspace_entries,
        }
    }
    /// Every actual cut/noncut candidate output.
    #[must_use]
    pub const fn outputs(self) -> usize {
        self.outputs
    }
    /// Every original proposed construction occurrence,including repetitions.
    #[must_use]
    pub const fn uses(self) -> usize {
        self.uses
    }
    /// Complete new U association entries,not allocator bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole quota,operation or original actual-output membership refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinConstructionUseRefusal {
    /// Complete actual candidate population exceeds policy.
    Outputs {
        /// Actual whole N.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original proposed use population exceeds policy.
    Uses {
        /// Actual whole U.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete U additional associations exceed policy.
    Workspace {
        /// Actual whole required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw operation differs from the actual original join operation.
    Operation,
    /// Tag names no output in the actual complete direction/candidate context.
    UnknownOutput {
        /// First original proposed construction occurrence.
        index: usize,
        /// Actual explicit tag.
        tag: LocalTag,
    },
}
/// Whole original manifest,actual cut-path context,policy and first cause survive refusal.
pub struct JoinConstructionUseError<'a> {
    manifest: &'a JoinConstructionManifest,
    connectivity: &'a RequiredJoinCutConnectivity<'a>,
    limits: JoinConstructionUseLimits,
    refusal: JoinConstructionUseRefusal,
}
impl<'a> JoinConstructionUseError<'a> {
    /// Actual complete original construction proposal,including unaccepted repeated suffixes.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinConstructionManifest {
        self.manifest
    }
    /// Actual complete required cut path and original cut-use/direction/source parents.
    #[must_use]
    pub const fn connectivity(&self) -> &'a RequiredJoinCutConnectivity<'a> {
        self.connectivity
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionUseLimits {
        self.limits
    }
    /// First complete quota,operation or original membership cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinConstructionUseRefusal {
        self.refusal
    }
    /// Actual original first refused use only for UnknownOutput;other causes return None.
    #[must_use]
    pub fn refused_use(&self) -> Option<&'a JoinConstructionUse> {
        let JoinConstructionUseRefusal::UnknownOutput { index, .. } = self.refusal else {
            return None;
        };
        self.manifest.uses.get(index)
    }
}
impl fmt::Debug for JoinConstructionUseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinConstructionUseError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinConstructionUseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join proposed construction uses: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinConstructionUseError<'_> {}
/// Explicit operation and complete proposed construction list. Empty/repeated lists are retained;
/// declaration presence is neither accepted original-source omission nor a no-op/result certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinConstructionManifest {
    operation: EntityId,
    uses: Vec<JoinConstructionUse>,
}
impl JoinConstructionManifest {
    /// Retain every original field and order,without filtering,sorting,deduplication or id issuance.
    #[must_use]
    pub const fn new(operation: EntityId, uses: Vec<JoinConstructionUse>) -> Self {
        Self { operation, uses }
    }
    /// Actual original declared operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original construction occurrences,including repeats/unknown tags before admission.
    #[must_use]
    pub fn uses(&self) -> &[JoinConstructionUse] {
        &self.uses
    }
    /// Admit complete N/U/U-entry resources before operation,fields or new association allocation.
    /// Each original tag must name an actual directional output,not merely an active catalogue role.
    /// Construction may reuse cut/noncut outputs and repeat geometry,as canonical Piece fields permit.
    /// Explicit traversal remains independent of source field/parameter orientation. Empty lists
    /// retain presence only;complete original construction/fold/consumer reconciliation remains
    /// mandatory separately,as do geometry,currentness,copy effects,identity/retirement and authority.
    /// Work O(U log(N+2)+1),U new associations,no source/key scan or registry/provider/geometry call.
    /// Earlier complete input/source/candidate/direction/cut-use/path policies and storage remain.
    /// # Errors
    /// Whole N,U,association quotas;operation;then first original unknown actual output tag.
    pub fn admit<'a>(
        &'a self,
        connectivity: &'a RequiredJoinCutConnectivity<'a>,
        limits: JoinConstructionUseLimits,
    ) -> Result<DeclaredJoinConstructionUses<'a>, JoinConstructionUseError<'a>> {
        use JoinConstructionUseRefusal as R;
        let refuse = |refusal| JoinConstructionUseError {
            manifest: self,
            connectivity,
            limits,
            refusal,
        };
        let directions = connectivity.contours().directions();
        let n = directions.coverage().edges().edge_count();
        let u = self.uses.len();
        if n > limits.outputs {
            return Err(refuse(R::Outputs {
                required: n,
                allowed: limits.outputs,
            }));
        }
        if u > limits.uses {
            return Err(refuse(R::Uses {
                required: u,
                allowed: limits.uses,
            }));
        }
        if u > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: u,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation
            != directions
                .coverage()
                .edges()
                .identity()
                .roles()
                .catalog()
                .catalog()
                .operation()
        {
            return Err(refuse(R::Operation));
        }
        let mut uses = Vec::with_capacity(u);
        for (index, declared) in self.uses.iter().enumerate() {
            let tag = declared.tag;
            let output = directions
                .assignment(tag)
                .ok_or_else(|| refuse(R::UnknownOutput { index, tag }))?;
            uses.push(ConstructionAssociation {
                index,
                declared,
                output,
            });
        }
        Ok(DeclaredJoinConstructionUses {
            manifest: self,
            connectivity,
            limits,
            uses,
        })
    }
}
#[derive(Debug)]
struct ConstructionAssociation<'a> {
    index: usize,
    declared: &'a JoinConstructionUse,
    output: JoinDirectionalOutput<'a>,
}
/// Authentic ordered proposed construction uses,not complete source-field reconciliation or geometry.
/// ```compile_fail
/// fn forge<'a>(mut uses:sc_core::construction::DeclaredJoinConstructionUses<'a>,other:&'a sc_core::construction::JoinConstructionManifest) {uses.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinConstructionManifest,path:sc_core::construction::RequiredJoinCutConnectivity<'_>)
///     ->sc_core::construction::DeclaredJoinConstructionUses<'static> {
///     raw.admit(&path,sc_core::construction::JoinConstructionUseLimits::new(0,0,0)).unwrap()
/// }
/// ```
pub struct DeclaredJoinConstructionUses<'a> {
    manifest: &'a JoinConstructionManifest,
    connectivity: &'a RequiredJoinCutConnectivity<'a>,
    limits: JoinConstructionUseLimits,
    uses: Vec<ConstructionAssociation<'a>>,
}
impl fmt::Debug for DeclaredJoinConstructionUses<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeclaredJoinConstructionUses")
            .field("operation", &self.manifest.operation)
            .field("uses", &self.uses.len())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl<'a> DeclaredJoinConstructionUses<'a> {
    /// Actual complete original proposal,without normalized or reconstructed fields.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinConstructionManifest {
        self.manifest
    }
    /// Actual required original cut-path and whole source/identity/candidate/direction parents.
    #[must_use]
    pub const fn connectivity(&self) -> &'a RequiredJoinCutConnectivity<'a> {
        self.connectivity
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionUseLimits {
        self.limits
    }
    /// Complete proposed construction occurrence count,including repetitions.
    #[must_use]
    pub fn use_count(&self) -> usize {
        self.uses.len()
    }
    /// Complete additional U associations,not allocator bytes or earlier input/context storage.
    #[must_use]
    pub fn workspace_entries(&self) -> usize {
        self.uses.len()
    }
    /// Every authentic proposed use in original order,and reverse;no deduplication or source copying.
    pub fn uses(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinPlacedConstructionUse<'_>> + DoubleEndedIterator {
        self.uses
            .iter()
            .map(move |association| JoinPlacedConstructionUse {
                context: self,
                association,
            })
    }
    /// All authentic occurrences of one explicit output tag,in original order and reverse.
    /// This scans U associations without allocating an index;repetitions remain independent.
    pub fn uses_for(
        &self,
        tag: LocalTag,
    ) -> impl DoubleEndedIterator<Item = JoinPlacedConstructionUse<'_>> {
        self.uses()
            .filter(move |use_| *use_.declared().tag() == tag)
    }
}
/// One authentic original construction use with its actual direction/candidate/source attribution.
/// ```compile_fail
/// fn forge<'a>(mut use_:sc_core::construction::JoinPlacedConstructionUse<'a>,other:&'a sc_core::construction::DeclaredJoinConstructionUses<'a>) {use_.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinPlacedConstructionUse<'a> {
    context: &'a DeclaredJoinConstructionUses<'a>,
    association: &'a ConstructionAssociation<'a>,
}
impl<'a> JoinPlacedConstructionUse<'a> {
    /// Actual complete proposed construction-use owner.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinConstructionUses<'a> {
        self.context
    }
    /// Original occurrence ordinal,not a persistent identity or deduplicated geometry index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original explicit tag/traversal field.
    #[must_use]
    pub const fn declared(self) -> &'a JoinConstructionUse {
        self.association.declared
    }
    /// Actual original explicit construction traversal field.
    #[must_use]
    pub const fn direction(self) -> &'a Direction {
        self.association.declared.direction()
    }
    /// Actual original directional output/candidate/source,without copied provider truth.
    #[must_use]
    pub const fn output(self) -> JoinDirectionalOutput<'a> {
        self.association.output
    }
    /// Declared actual operation/persistent-tag tuple,without issuing/publishing an edge.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        self.association.output.source().logical_output()
    }
}
