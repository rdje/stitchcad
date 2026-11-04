//! Complete initial canonical/metadata and existing-geometry associations before namespace work.
use super::BoundDraftingProgramOutputRoles;
use crate::{
    construction::{
        DraftingGeometryBinding, DraftingGeometryError, DraftingGeometryReference,
        DraftingInitialInputProvider, DraftingInitialInputSource, DraftingProgramStepKind,
    },
    design::DesignMetadataHost,
    ontology::EntityId,
    recipe::{FormulaCanonicalInput, FormulaCanonicalInputError, FormulaDeclarationSource as D},
};
use core::fmt;
use std::collections::TryReserveError;

/// First field preventing an actual initial metadata response from matching its containing input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingInitialBindingIssue {
    /// Returned canonical join belongs to a different actual inventory,even with equal ids.
    Inventory,
    /// Returned declaration does not borrow the original metadata name field.
    Name,
    /// Source family,origin,metadata identity,canonical target/kind or actual Length owner differs.
    Declaration,
    /// Returned snapshot record differs from the actual containing context record.
    Snapshot,
}

/// Original complete initial occurrence after actual scalar/source or existing-geometry binding.
/// No scalar state or geometry value is read;metadata records remain under the immutable port.
/// This inspectable enum alone is no whole association proof;only the private complete owner is.
pub enum DraftingInitialBinding<
    'i,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    /// Actual containing inventory,original declaration,metadata and snapshot record loans.
    Input(DraftingInitialInputSource<'i, S::Metadata, S::Snapshot>),
    /// Exact existing output under the original context;no programme creator can enter here.
    Geometry(DraftingGeometryBinding<'i, H, S>),
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingInitialBinding<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(source) => source.fmt(f),
            Self::Geometry(source) => source.fmt(f),
        }
    }
}

/// First original refusal;allocation causes stay inline and source witnesses are never copied.
pub enum DraftingInitialRefusal<
    'i,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    /// Complete outer-vector reservation failed before any initial source query.
    Allocation(TryReserveError),
    /// Original actual inventory/kind/Length-owner join refusal,before the provider query.
    Canonical {
        /// Original zero-based extra-initial occurrence.
        index: usize,
        /// Existing join cause with original declaration and actual inventory;boxing compacts the diagnostic.
        cause: Box<FormulaCanonicalInputError<'i>>,
    },
    /// Original typed initial metadata-source cause.
    Source {
        /// Original zero-based extra-initial occurrence.
        index: usize,
        /// Original provider cause;boxing only compacts this semantic diagnostic.
        cause: Box<S::Error>,
    },
    /// Complete original response and first mismatch against its containing original input.
    Mismatch {
        /// Original zero-based extra-initial occurrence.
        index: usize,
        /// Complete original response,including foreign inventory/metadata/snapshot loans.
        source: Box<DraftingInitialInputSource<'i, S::Metadata, S::Snapshot>>,
        /// First mismatch in inventory,name,declaration,snapshot order.
        issue: DraftingInitialBindingIssue,
    },
    /// A programme native creator cannot be an always-visible initial geometry source.
    PlannedCreator {
        /// Original zero-based extra-initial occurrence.
        index: usize,
        /// Actual matching native operation identity.
        creator: EntityId,
        /// First genuine one-based mixed position with this creator,before any live lookup.
        position: usize,
    },
    /// Original existing-output cause or complete reference/frame/snapshot mismatch.
    Geometry {
        /// Original zero-based extra-initial occurrence.
        index: usize,
        /// Original context geometry refusal and its complete source witnesses.
        cause: Box<DraftingGeometryError<'i, H, S>>,
    },
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingInitialRefusal<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation(cause) => f.debug_tuple("Allocation").field(cause).finish(),
            Self::Canonical { index, cause } => f
                .debug_struct("Canonical")
                .field("initial_index", index)
                .field("cause", cause)
                .finish(),
            Self::Source { index, .. } => f
                .debug_struct("Source")
                .field("initial_index", index)
                .finish_non_exhaustive(),
            Self::Mismatch { index, issue, .. } => f
                .debug_struct("Mismatch")
                .field("initial_index", index)
                .field("issue", issue)
                .finish_non_exhaustive(),
            Self::PlannedCreator {
                index, position, ..
            } => f
                .debug_struct("PlannedCreator")
                .field("initial_index", index)
                .field("position", position)
                .finish_non_exhaustive(),
            Self::Geometry { index, .. } => f
                .debug_struct("Geometry")
                .field("initial_index", index)
                .finish_non_exhaustive(),
        }
    }
}

/// Complete initial refusal pins original role bindings,all original inputs and selected policy.
pub struct DraftingInitialError<
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    roles: &'i BoundDraftingProgramOutputRoles<'r, 'b, 'n, 'p, 'a, H, S>,
    refusal: DraftingInitialRefusal<'i, H, S>,
}
impl<
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingInitialError<'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original complete roles,positions,bounded work,input and policy;no accepted initial prefix.
    #[must_use]
    pub const fn roles(&self) -> &'i BoundDraftingProgramOutputRoles<'r, 'b, 'n, 'p, 'a, H, S> {
        self.roles
    }
    /// Original first cause and complete original source witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingInitialRefusal<'i, H, S> {
        &self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingInitialError<'_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.refusal.fmt(f)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingInitialError<'_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting initial binding: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingInitialError<'_, '_, '_, '_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            DraftingInitialRefusal::Allocation(cause) => Some(cause),
            DraftingInitialRefusal::Source { cause, .. } => Some(cause.as_ref()),
            DraftingInitialRefusal::Geometry { cause, .. } => cause.source(),
            DraftingInitialRefusal::Canonical { .. }
            | DraftingInitialRefusal::Mismatch { .. }
            | DraftingInitialRefusal::PlannedCreator { .. } => None,
        }
    }
}

/// Whole original initial-vector association,borrowing the complete prior role proof.
/// Values,namespace collisions,ordered visibility and whole dependency acceptance remain separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::{BoundDraftingProgramInitials,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     bound:&mut BoundDraftingProgramInitials<'_, '_, '_, '_, '_, '_,H,S>) {bound.sources.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::{BoundDraftingProgramInitials,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn escape<'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     bound:BoundDraftingProgramInitials<'i,'r,'b,'n,'p,'a,H,S>)
///     ->BoundDraftingProgramInitials<'static,'r,'b,'n,'p,'a,H,S> {bound}
/// ```
pub struct BoundDraftingProgramInitials<
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    roles: &'i BoundDraftingProgramOutputRoles<'r, 'b, 'n, 'p, 'a, H, S>,
    sources: Vec<DraftingInitialBinding<'i, H, S>>,
}
impl<
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original complete roles/positions/input and explicit resource policy.
    #[must_use]
    pub const fn roles(&self) -> &'i BoundDraftingProgramOutputRoles<'r, 'b, 'n, 'p, 'a, H, S> {
        self.roles
    }
    /// Complete original initial-vector order;repeated occurrences retain separate bindings.
    #[must_use]
    pub fn sources(&self) -> &[DraftingInitialBinding<'i, H, S>] {
        &self.sources
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramInitials<'_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramInitials")
            .field("initial_count", &self.sources.len())
            .finish_non_exhaustive()
    }
}
fn same_declaration(expected: D<'_>, actual: D<'_>) -> bool {
    match (expected, actual) {
        (
            D::Input {
                origin: e,
                input: ei,
                declaration: ed,
                kind: ek,
            },
            D::Input {
                origin: a,
                input: ai,
                declaration: ad,
                kind: ak,
            },
        ) => e == a && ei == ai && ed == ad && ek == ak,
        (
            D::LengthInput {
                origin: e,
                input: ei,
                declaration: ed,
            },
            D::LengthInput {
                origin: a,
                input: ai,
                declaration: ad,
            },
        ) => e == a && ei == ai && core::ptr::eq(ed, ad),
        _ => false,
    }
}
impl<'r, 'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized>
    BoundDraftingProgramOutputRoles<'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Bind all original extra initials after complete held-content/work/output association.
    /// Reserve every initial slot before callbacks. Canonical inputs first join the actual Root
    /// inventory,then compare returned inventory,name owner,declaration and snapshot in that order.
    /// For each geometry occurrence,scan every actual native/semantic operation identity before
    /// existing-output lookup. A semantic member cannot supply native Point/Edge geometry.
    /// Earlier borrowed sources are discarded on any later refusal;no accepted prefix escapes.
    ///
    /// ```
    /// use sc_core::{construction::{BoundDraftingProgramOutputRoles,DraftingInitialInputProvider},
    ///     design::DesignMetadataHost};
    /// fn bind_initials<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     roles:&BoundDraftingProgramOutputRoles<'_, '_, '_, '_, '_,H,S>)->Result<usize,String> {
    ///     let bound=roles.bind_initial_sources().map_err(|error|error.to_string())?;
    ///     Ok(bound.sources().len())
    /// }
    /// ```
    /// # Errors
    /// Original inline reservation cause,canonical/provider/geometry refusal,complete metadata
    /// mismatch or actual programme creator witness. No values or namespace factory is invoked.
    pub fn bind_initial_sources<'i>(
        &'i self,
    ) -> Result<
        BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingInitialError<'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    > {
        let refuse = |refusal| DraftingInitialError {
            roles: self,
            refusal,
        };
        let names = self.positions().work().inputs();
        let program = names.program();
        let context = program.admitted().inputs().context();
        let mut sources = Vec::new();
        sources
            .try_reserve_exact(names.initial().len())
            .map_err(|cause| refuse(DraftingInitialRefusal::Allocation(cause)))?;
        for (index, initial) in names.initial().iter().enumerate() {
            let declaration = initial.declaration();
            let requested = match declaration.source() {
                D::Input { .. } | D::LengthInput { .. } => {
                    let input = FormulaCanonicalInput::resolve(
                        declaration,
                        context.scope().root().inputs(),
                    )
                    .map_err(|cause| {
                        refuse(DraftingInitialRefusal::Canonical {
                            index,
                            cause: Box::new(cause),
                        })
                    })?;
                    let source = context
                        .source()
                        .initial_input(index, input)
                        .map_err(|cause| {
                            refuse(DraftingInitialRefusal::Source {
                                index,
                                cause: Box::new(cause),
                            })
                        })?;
                    let actual = source.input.metadata();
                    let issue = if !core::ptr::eq(input.inputs(), source.input.inputs()) {
                        Some(DraftingInitialBindingIssue::Inventory)
                    } else if !core::ptr::eq(declaration.name(), actual.name()) {
                        Some(DraftingInitialBindingIssue::Name)
                    } else if !same_declaration(declaration.source(), actual.source()) {
                        Some(DraftingInitialBindingIssue::Declaration)
                    } else if !context.snapshot().is_same_record(source.snapshot) {
                        Some(DraftingInitialBindingIssue::Snapshot)
                    } else {
                        None
                    };
                    if let Some(issue) = issue {
                        return Err(refuse(DraftingInitialRefusal::Mismatch {
                            index,
                            source: Box::new(source),
                            issue,
                        }));
                    }
                    sources.push(DraftingInitialBinding::Input(source));
                    continue;
                }
                D::Point(point) => DraftingGeometryReference::Point(point),
                D::Edge(edge) => DraftingGeometryReference::Edge(edge),
                D::Recipe { .. } | D::Reserved(_) => {
                    unreachable!("private initial declaration excludes recipe/reserved sources")
                }
            };
            let creator = match requested {
                DraftingGeometryReference::Point(point) => point.creator(),
                DraftingGeometryReference::Edge(edge) => edge.creator(),
            };
            for step in program.steps() {
                let is_planned = match step.kind() {
                    DraftingProgramStepKind::Drafting(operation) => {
                        operation.input().operation() == creator
                    }
                    DraftingProgramStepKind::Semantic { operation, .. } => {
                        operation.id() == creator
                    }
                    DraftingProgramStepKind::Scalar { .. } => false,
                };
                if is_planned {
                    return Err(refuse(DraftingInitialRefusal::PlannedCreator {
                        index,
                        creator,
                        position: step.position(),
                    }));
                }
            }
            let binding = context.bind_existing_geometry(requested).map_err(|cause| {
                refuse(DraftingInitialRefusal::Geometry {
                    index,
                    cause: Box::new(cause),
                })
            })?;
            sources.push(DraftingInitialBinding::Geometry(binding));
        }
        Ok(BoundDraftingProgramInitials {
            roles: self,
            sources,
        })
    }
}
