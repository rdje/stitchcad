//! Complete bounded planned-role sources;genuine input/catalogue/context records before return.
use super::{DraftingGeometryNameSource as G, DraftingProgramOutputPositions};
use crate::{
    construction::{
        DraftingContextSource, DraftingOperationInput as I, DraftingPlannedOutputProvider,
        DraftingPlannedOutputSource, DraftingProgramStepKind,
    },
    design::DesignMetadataHost,
};
use core::fmt;
use std::collections::TryReserveError;

/// First original field preventing a genuine planned family source from binding to its request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingOutputRoleBindingIssue {
    /// Actual creating schema/family owner differs,even when authored ids/fields compare equal.
    Input,
    /// Actual Point/Edge role domain differs from the original named-output source.
    RoleKind,
    /// Actual admitted catalogue owner differs,even for a second admission of the same raw data.
    Catalogue,
    /// Actual original definition field differs.
    Definition,
    /// Actual original active-list tag field differs.
    Tag,
    /// Actual family logical output differs from the original explicit role reference.
    Reference,
    /// Actual frame source record differs from the original containing context record.
    Frame,
    /// Actual snapshot source record differs from the original containing context record.
    Snapshot,
}
/// Original occurrence and complete raw source at its first field mismatch;no cloned payload.
pub struct DraftingOutputRoleMismatch<'r, F: ?Sized, S: ?Sized> {
    index: usize,
    source: DraftingPlannedOutputSource<'r, F, S>,
    issue: DraftingOutputRoleBindingIssue,
}
impl<'r, F: ?Sized, S: ?Sized> DraftingOutputRoleMismatch<'r, F, S> {
    /// Original zero-based named-output occurrence.
    #[must_use]
    pub const fn output_index(&self) -> usize {
        self.index
    }
    /// Complete original response,with actual input/role/frame/snapshot loans intact.
    #[must_use]
    pub const fn returned(&self) -> DraftingPlannedOutputSource<'r, F, S> {
        self.source
    }
    /// First actual field that failed binding.
    #[must_use]
    pub const fn issue(&self) -> DraftingOutputRoleBindingIssue {
        self.issue
    }
}
impl<F: ?Sized, S: ?Sized> fmt::Debug for DraftingOutputRoleMismatch<'_, F, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingOutputRoleMismatch")
            .field("output_index", &self.index)
            .field("issue", &self.issue)
            .finish_non_exhaustive()
    }
}
/// Original allocation/provider or complete source mismatch;allocator causes remain inline.
pub enum DraftingOutputRoleRefusal<'r, F: ?Sized, S: ?Sized, E> {
    /// Original complete outer-vector reservation cause,before any provider query or diagnostic box.
    Allocation(TryReserveError),
    /// Original typed planned-source cause at its original named-output occurrence.
    Source {
        /// Original zero-based named-output occurrence.
        index: usize,
        /// Original provider cause;boxing compacts this semantic diagnostic only.
        cause: Box<E>,
    },
    /// Complete original response and first mismatch,boxed without copying opaque records.
    Mismatch(Box<DraftingOutputRoleMismatch<'r, F, S>>),
}
impl<F: ?Sized, S: ?Sized, E> fmt::Debug for DraftingOutputRoleRefusal<'_, F, S, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation(error) => f.debug_tuple("Allocation").field(error).finish(),
            Self::Source { index, .. } => f
                .debug_struct("Source")
                .field("output_index", index)
                .finish_non_exhaustive(),
            Self::Mismatch(error) => error.fmt(f),
        }
    }
}
/// Complete refusal pins the original bounded positions,input owner and selected policy.
pub struct DraftingOutputRoleError<
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    positions: &'r DraftingProgramOutputPositions<'b, 'n, 'p, 'a, H, S>,
    refusal: DraftingOutputRoleRefusal<'r, S::Frame, S::Snapshot, S::Error>,
}
impl<'r, 'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingOutputRoleError<'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original complete positions/bounded work/input/policy;no accepted prefix is retained.
    #[must_use]
    pub const fn positions(&self) -> &'r DraftingProgramOutputPositions<'b, 'n, 'p, 'a, H, S> {
        self.positions
    }
    /// Original allocation/provider cause or complete actual source mismatch.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingOutputRoleRefusal<'r, S::Frame, S::Snapshot, S::Error> {
        &self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingOutputRoleError<'_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.refusal.fmt(f)
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingOutputRoleError<'_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting output role binding: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingOutputRoleError<'_, '_, '_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            DraftingOutputRoleRefusal::Allocation(error) => Some(error),
            DraftingOutputRoleRefusal::Source { cause, .. } => Some(cause.as_ref()),
            DraftingOutputRoleRefusal::Mismatch(_) => None,
        }
    }
}
/// Complete actual planned role/input/catalogue/context association;no namespace or geometry grant.
/// Sources borrow the original provider and position owner;there is no self-reference or copied
/// namespace. Fixed-schema names have None slots and perform no planned-output lookup.
/// ```compile_fail,E0616
/// use sc_core::{construction::{BoundDraftingProgramOutputRoles,DraftingContextSource},design::DesignMetadataHost};
/// fn inspect<H:DesignMetadataHost,S:DraftingContextSource<H>>(bound:&BoundDraftingProgramOutputRoles<'_, '_, '_, '_, '_,H,S>) {
///     let _=&bound.sources;
/// }
/// ```
/// ```compile_fail
/// use sc_core::{construction::{BoundDraftingProgramOutputRoles,DraftingContextSource},design::DesignMetadataHost};
/// fn escape<'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     bound:BoundDraftingProgramOutputRoles<'r,'b,'n,'p,'a,H,S>)
///     ->BoundDraftingProgramOutputRoles<'static,'b,'n,'p,'a,H,S> {bound}
/// ```
pub struct BoundDraftingProgramOutputRoles<
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    positions: &'r DraftingProgramOutputPositions<'b, 'n, 'p, 'a, H, S>,
    sources: Vec<Option<DraftingPlannedOutputSource<'r, S::Frame, S::Snapshot>>>,
}
impl<'r, 'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    BoundDraftingProgramOutputRoles<'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Actual original positions and complete bounded input/policy.
    #[must_use]
    pub const fn positions(&self) -> &'r DraftingProgramOutputPositions<'b, 'n, 'p, 'a, H, S> {
        self.positions
    }
    /// Complete original output-vector order:None for fixed schemas,Some for actual bound roles.
    #[must_use]
    pub fn sources(&self) -> &[Option<DraftingPlannedOutputSource<'r, S::Frame, S::Snapshot>>] {
        &self.sources
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramOutputRoles<'_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramOutputRoles")
            .field("output_count", &self.sources.len())
            .finish_non_exhaustive()
    }
}
fn same_input(expected: I<'_>, actual: I<'_>) -> bool {
    match expected {
        I::OffsetEdge(e) => matches!(actual,I::OffsetEdge(a) if std::ptr::eq(e,a)),
        I::SlashSpread(e) => matches!(actual,I::SlashSpread(a) if std::ptr::eq(e,a)),
        I::TrueSeam(e) => matches!(actual,I::TrueSeam(a) if std::ptr::eq(e,a)),
        I::MirrorPiece(e) => matches!(actual,I::MirrorPiece(a) if std::ptr::eq(e,a)),
        I::JoinPieces(e) => matches!(actual,I::JoinPieces(a) if std::ptr::eq(e,a)),
        I::ConstructPoint(_)
        | I::DrawLine(_)
        | I::DrawArc(_)
        | I::DrawBezier(_)
        | I::CloseDart { .. }
        | I::AddNotch { .. }
        | I::PlaceGrainline(_)
        | I::WalkSeam(_) => false,
    }
}
fn role(expected: G<'_>, actual: G<'_>) -> Option<DraftingOutputRoleBindingIssue> {
    use DraftingOutputRoleBindingIssue as R;
    match (expected, actual) {
        (G::PointRole(e), G::PointRole(a)) => {
            if !std::ptr::eq(e.catalog(), a.catalog()) {
                Some(R::Catalogue)
            } else if !std::ptr::eq(e.definition(), a.definition()) {
                Some(R::Definition)
            } else if !std::ptr::eq(e.tag(), a.tag()) {
                Some(R::Tag)
            } else {
                None
            }
        }
        (G::EdgeRole(e), G::EdgeRole(a)) => {
            if !std::ptr::eq(e.catalog(), a.catalog()) {
                Some(R::Catalogue)
            } else if !std::ptr::eq(e.definition(), a.definition()) {
                Some(R::Definition)
            } else if !std::ptr::eq(e.tag(), a.tag()) {
                Some(R::Tag)
            } else {
                None
            }
        }
        _ => Some(R::RoleKind),
    }
}
impl<'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingPlannedOutputProvider<H> + ?Sized>
    DraftingProgramOutputPositions<'b, 'n, 'p, 'a, H, S>
{
    /// Bind every original planned role after full content/work/position admission. Reserve all
    /// prospective output slots before any source query;fixed-schema names use None and no lookup.
    /// Authentic field input,role kind,catalogue,definition,tag,logical reference,frame and snapshot
    /// are compared in that order. Actual record identity uses SourceRecord,not payload addresses.
    /// No values,existing-geometry lookup,namespace insertion or accepted prefix occurs.
    ///
    /// ```
    /// use sc_core::{construction::{DraftingPlannedOutputProvider, DraftingProgramOutputPositions},
    ///     design::DesignMetadataHost};
    /// fn bind_sources<H:DesignMetadataHost,S:DraftingPlannedOutputProvider<H> + ?Sized>(
    ///     positions:&DraftingProgramOutputPositions<'_, '_, '_, '_,H,S>)->Result<usize,String> {
    ///     let bound=positions.bind_role_sources().map_err(|error|error.to_string())?;
    ///     Ok(bound.sources().len())
    /// }
    /// ```
    /// # Errors
    /// Original inline reservation cause,first original provider cause,or complete first mismatch.
    /// Original positions/input/policy and bad source witnesses survive;diagnostics hide names.
    pub fn bind_role_sources<'r>(
        &'r self,
    ) -> Result<
        BoundDraftingProgramOutputRoles<'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingOutputRoleError<'r, 'b, 'n, 'p, 'a, H, S>,
    > {
        let refuse = |refusal| DraftingOutputRoleError {
            positions: self,
            refusal,
        };
        let context = self.work().inputs().program().admitted().inputs().context();
        let mut sources = Vec::new();
        sources
            .try_reserve_exact(self.work().inputs().outputs().len())
            .map_err(|cause| refuse(DraftingOutputRoleRefusal::Allocation(cause)))?;
        for request in self.outputs() {
            if !request.requires_role_binding() {
                sources.push(None);
                continue;
            }
            let index = request.output_index();
            let source = context.source().planned_output(request).map_err(|cause| {
                refuse(DraftingOutputRoleRefusal::Source {
                    index,
                    cause: Box::new(cause),
                })
            })?;
            let DraftingProgramStepKind::Drafting(operation) = request.step().kind() else {
                unreachable!("private actual native output-position invariant");
            };
            let expected = request.output().source();
            let issue = if !same_input(operation.input(), source.output.input()) {
                Some(DraftingOutputRoleBindingIssue::Input)
            } else if let Some(issue) = role(expected, source.output.name_source()) {
                Some(issue)
            } else if source.output.reference() != expected.reference() {
                Some(DraftingOutputRoleBindingIssue::Reference)
            } else if !context.frame().is_same_record(source.frame) {
                Some(DraftingOutputRoleBindingIssue::Frame)
            } else if !context.snapshot().is_same_record(source.snapshot) {
                Some(DraftingOutputRoleBindingIssue::Snapshot)
            } else {
                None
            };
            if let Some(issue) = issue {
                return Err(refuse(DraftingOutputRoleRefusal::Mismatch(Box::new(
                    DraftingOutputRoleMismatch {
                        index,
                        source,
                        issue,
                    },
                ))));
            }
            sources.push(Some(source));
        }
        Ok(BoundDraftingProgramOutputRoles {
            positions: self,
            sources,
        })
    }
}
