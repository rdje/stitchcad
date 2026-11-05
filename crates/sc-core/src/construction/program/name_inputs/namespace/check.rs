//! Whole original mixed kind/name order;the final metadata cursor never escapes this controller.
use super::{
    DraftingProgramInitialNamespace, DraftingProgramOrderLimits, DraftingProgramOrderShape,
    DraftingProgramOrderWorkRefusal,
};
use crate::{
    construction::{
        BoundDraftingProgramInitials, CenterArcCheckError, CheckedCenterArcConstruction,
        CheckedPointConstruction, CheckedSlashSpreadOpening, DraftingInitialInputProvider,
        DraftingOperationInput, DraftingProgramStepKind, NormalizedDraftingOperation,
        PointConstructionCheckError, SlashSpreadCheckError,
    },
    design::DesignMetadataHost,
    recipe::{
        FormulaCheckedStatement, FormulaGeometryDeclaration, FormulaNameCursor,
        FormulaNamespaceError, FormulaStatementCheckError, SemanticOperation,
    },
};
use core::fmt;
use std::collections::TryReserveError;

/// Inspect complete original scalar/native argument proofs;this enum alone is no programme proof.
#[derive(Debug)]
pub enum DraftingProgramOrderProofKind<'q> {
    /// Actual Root semantic member in its original order. Its typed ID/rule dependencies remain
    /// separate; it contributes no scalar declaration, formula kind or Point/Edge name.
    Semantic {
        /// Original one-based semantic recipe ordinal.
        ordinal: usize,
        /// Actual immutable member, without a value, source/rule or execution grant.
        operation: &'q SemanticOperation,
    },
    /// Whole actual scalar statement,including both assertion operands and original ordinal.
    Scalar(FormulaCheckedStatement<'q>),
    /// Both original Length coordinate arguments.
    Point(CheckedPointConstruction<'q>),
    /// All original Radius/StartAngle/SweepAngle arguments.
    CenterArc(CheckedCenterArcConstruction<'q>),
    /// Actual explicit opening mode's complete scalar argument.
    SlashSpread(CheckedSlashSpreadOpening<'q>),
    /// Original native family without scalar argument documents;dependencies remain separate.
    Other(DraftingOperationInput<'q>),
}
/// Privately numbered genuine mixed step and its complete original argument proof.
#[derive(Debug)]
pub struct DraftingProgramOrderStep<'q> {
    position: usize,
    kind: DraftingProgramOrderProofKind<'q>,
}
impl<'q> DraftingProgramOrderStep<'q> {
    /// Genuine one-based mixed position,separate from scalar statement ordinal.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }
    /// Original complete scalar/native components;no replacement or mutable proof slot.
    #[must_use]
    pub const fn kind(&self) -> &DraftingProgramOrderProofKind<'q> {
        &self.kind
    }
}
/// Original first full-work,reservation,header or child-checking refusal;no accepted prefix.
pub enum DraftingProgramOrderRefusal<'q> {
    /// Complete actual work refused before reservation/header/child work.
    Work(DraftingProgramOrderWorkRefusal),
    /// Original complete outer-slot reservation cause,retained inline.
    Allocation(TryReserveError),
    /// Original namespace/header source pair before this step's operands become visible.
    Header {
        /// Genuine one-based mixed position.
        position: usize,
        /// Original named-output occurrence,or None for an actual scalar header.
        output_index: Option<usize>,
        /// Existing original namespace diagnostic and source witnesses.
        cause: Box<FormulaNamespaceError<'q>>,
    },
    /// Original scalar statement/ordinal/operand cause.
    Scalar {
        /// Genuine one-based mixed position.
        position: usize,
        /// Original statement cause;boxing compacts semantic diagnostics.
        cause: Box<FormulaStatementCheckError<'q>>,
    },
    /// Original point coordinate/arena/name/kind cause.
    Point {
        /// Genuine one-based mixed position.
        position: usize,
        /// Original typed cause.
        cause: Box<PointConstructionCheckError<'q>>,
    },
    /// Original center-arc argument/arena/name/kind cause.
    CenterArc {
        /// Genuine one-based mixed position.
        position: usize,
        /// Original typed cause.
        cause: Box<CenterArcCheckError<'q>>,
    },
    /// Original opening mode/arena/name/kind cause.
    SlashSpread {
        /// Genuine one-based mixed position.
        position: usize,
        /// Original typed cause.
        cause: Box<SlashSpreadCheckError<'q>>,
    },
}
impl fmt::Debug for DraftingProgramOrderRefusal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Work(cause) => f.debug_tuple("Work").field(cause).finish(),
            Self::Allocation(cause) => f.debug_tuple("Allocation").field(cause).finish(),
            Self::Header {
                position,
                output_index,
                ..
            } => f
                .debug_struct("Header")
                .field("position", position)
                .field("output_index", output_index)
                .finish_non_exhaustive(),
            Self::Scalar { position, cause } => f
                .debug_struct("Scalar")
                .field("position", position)
                .field("cause", cause)
                .finish(),
            Self::Point { position, cause } => f
                .debug_struct("Point")
                .field("position", position)
                .field("cause", cause)
                .finish(),
            Self::CenterArc { position, cause } => f
                .debug_struct("CenterArc")
                .field("position", position)
                .field("cause", cause)
                .finish(),
            Self::SlashSpread { position, cause } => f
                .debug_struct("SlashSpread")
                .field("position", position)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Original complete bound sources and selected policy survive;the local map/proof prefix does not.
pub struct DraftingProgramOrderError<
    'q,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'q BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    limits: &'q DraftingProgramOrderLimits,
    refusal: DraftingProgramOrderRefusal<'q>,
}
impl<
        'q,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramOrderError<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original actual associations,roles,positions,Root/programme and resource policies.
    #[must_use]
    pub const fn inputs(&self) -> &'q BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Original additional checker policy.
    #[must_use]
    pub const fn limits(&self) -> &'q DraftingProgramOrderLimits {
        self.limits
    }
    /// Original first refusal and complete original source/statement/argument witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramOrderRefusal<'q> {
        &self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramOrderError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.refusal.fmt(f)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingProgramOrderError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting mixed order: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingProgramOrderError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            DraftingProgramOrderRefusal::Allocation(cause) => Some(cause),
            DraftingProgramOrderRefusal::Scalar { cause, .. } => cause.source(),
            DraftingProgramOrderRefusal::Point { cause, .. } => cause.source(),
            DraftingProgramOrderRefusal::CenterArc { cause, .. } => cause.source(),
            DraftingProgramOrderRefusal::SlashSpread { cause, .. } => cause.source(),
            DraftingProgramOrderRefusal::Work(_) | DraftingProgramOrderRefusal::Header { .. } => {
                None
            }
        }
    }
}
/// Complete original mixed name/header and scalar/native argument-kind proof.
/// The original initial map is consumed;the final cursor map is dropped without cloning/extraction.
/// Root formula sources,canonical/geometry/semantic dependencies,values and whole programme
/// acceptance remain separate. Actual scalar ordinals/spans and native owners stay unchanged.
/// ```compile_fail,E0616
/// use sc_core::{construction::{CheckedDraftingProgramOrder,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     proof:&mut CheckedDraftingProgramOrder<'_, '_, '_, '_, '_, '_, '_,H,S>) {proof.steps.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::{CheckedDraftingProgramOrder,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn escape<'q,'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     proof:CheckedDraftingProgramOrder<'q,'i,'r,'b,'n,'p,'a,H,S>)
///     ->CheckedDraftingProgramOrder<'static,'i,'r,'b,'n,'p,'a,H,S> {proof}
/// ```
pub struct CheckedDraftingProgramOrder<
    'q,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'q BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    limits: &'q DraftingProgramOrderLimits,
    shape: DraftingProgramOrderShape,
    steps: Vec<DraftingProgramOrderStep<'q>>,
}
impl<
        'q,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > CheckedDraftingProgramOrder<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact original complete initial/output source associations and actual programme owners.
    #[must_use]
    pub const fn inputs(&self) -> &'q BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Original explicit additional checker allowance.
    #[must_use]
    pub const fn limits(&self) -> &'q DraftingProgramOrderLimits {
        self.limits
    }
    /// Actual complete checked cohort and admitted prospective proof envelope.
    #[must_use]
    pub const fn shape(&self) -> DraftingProgramOrderShape {
        self.shape
    }
    /// Complete original mixed order;individual proof components cannot be replaced.
    #[must_use]
    pub fn steps(&self) -> &[DraftingProgramOrderStep<'q>] {
        &self.steps
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for CheckedDraftingProgramOrder<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CheckedDraftingProgramOrder")
            .field("step_count", &self.steps.len())
            .finish_non_exhaustive()
    }
}
impl<
        's,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Consume the actual initial map and check every original mixed step under complete work
    /// bounds. Reserve all proof slots first. Headers precede operands;native outputs become visible
    /// only after all original scalar arguments pass. Same-step outputs are absent from their RHS.
    /// Existing actual statement/native checkers retain all branches,causes,ordinals and spans.
    /// No provider/value/geometry callback,source extraction or cloned final namespace occurs.
    ///
    /// ```
    /// use sc_core::{construction::{DraftingProgramInitialNamespace,DraftingInitialInputProvider,
    ///     DraftingProgramOrderLimits},design::DesignMetadataHost};
    /// fn checked_steps<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     names:DraftingProgramInitialNamespace<'_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     policy:DraftingProgramOrderLimits)->Result<usize,String> {
    ///     names.check_order(&policy).map(|proof|proof.steps().len()).map_err(|error|error.to_string())
    /// }
    /// ```
    /// # Errors
    /// Complete work/original allocation cause,first original header or scalar/native argument cause.
    /// The complete original source owners/policy survive;no accepted local proof/map prefix escapes.
    pub fn check_order<'q>(
        self,
        limits: &'q DraftingProgramOrderLimits,
    ) -> Result<
        CheckedDraftingProgramOrder<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramOrderError<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    >
    where
        's: 'q,
    {
        let inputs = self.inputs;
        let refuse = |refusal| DraftingProgramOrderError {
            inputs,
            limits,
            refusal,
        };
        let shape = self
            .admit_order_work(limits)
            .map(|work| work.shape())
            .map_err(|error| refuse(DraftingProgramOrderRefusal::Work(error.refusal())))?;
        let mut steps = Vec::new();
        steps
            .try_reserve_exact(shape.step_count())
            .map_err(|cause| refuse(DraftingProgramOrderRefusal::Allocation(cause)))?;
        let positions = inputs.roles().positions();
        let program = positions.work().inputs().program();
        let mut cursor = FormulaNameCursor::new(self.namespace, program.recipe().normalized());
        for step in program.steps() {
            let position = step.position();
            let kind = match step.kind() {
                DraftingProgramStepKind::Semantic { ordinal, operation } => {
                    DraftingProgramOrderProofKind::Semantic { ordinal, operation }
                }
                DraftingProgramStepKind::Scalar { ordinal, statement } => {
                    let current = cursor
                        .current()
                        .map_err(|cause| {
                            refuse(DraftingProgramOrderRefusal::Header {
                                position,
                                output_index: None,
                                cause: Box::new(cause),
                            })
                        })?
                        .unwrap_or_else(|| {
                            unreachable!("private original scalar coverage/cursor invariant")
                        });
                    if current.statement_index() != ordinal
                        || !core::ptr::eq(current.statement(), statement)
                    {
                        unreachable!("private normalized programme/cursor retain the same actual recipe order");
                    }
                    let proof = current.check_kinds().map_err(|cause| {
                        refuse(DraftingProgramOrderRefusal::Scalar {
                            position,
                            cause: Box::new(cause),
                        })
                    })?;
                    let advanced = cursor.advance_metadata().map_err(|cause| {
                        refuse(DraftingProgramOrderRefusal::Header {
                            position,
                            output_index: None,
                            cause: Box::new(cause),
                        })
                    })?;
                    if !advanced {
                        unreachable!("same actual scalar statement advances exactly once");
                    }
                    DraftingProgramOrderProofKind::Scalar(proof)
                }
                DraftingProgramStepKind::Drafting(operation) => {
                    // Reuse existing map collision rules without inserting self/output names.
                    for request in positions
                        .outputs()
                        .filter(|request| request.step().position() == position)
                    {
                        let declaration = request.output().declaration();
                        let geometry = FormulaGeometryDeclaration::try_from(declaration)
                            .unwrap_or_else(|_| {
                                unreachable!("private named native output is Point/Edge")
                            });
                        let header = |cause| {
                            refuse(DraftingProgramOrderRefusal::Header {
                                position,
                                output_index: Some(request.output_index()),
                                cause: Box::new(cause),
                            })
                        };
                        cursor
                            .visible_names()
                            .check_geometry(geometry)
                            .map_err(header)?;
                        // Earlier same-step names are not in the map yet;retain their exact pair.
                        for prior in positions
                            .outputs()
                            .take(request.output_index())
                            .filter(|prior| prior.step().position() == position)
                        {
                            if prior.output().name() == request.output().name() {
                                return Err(header(FormulaNamespaceError::AmbiguousName {
                                    sources: Box::new([prior.output().declaration(), declaration]),
                                }));
                            }
                        }
                    }
                    let kind = match operation {
                        NormalizedDraftingOperation::Point(point) => {
                            DraftingProgramOrderProofKind::Point(
                                point
                                    .check_arguments_in_scope(cursor.visible_names())
                                    .map_err(|cause| {
                                        refuse(DraftingProgramOrderRefusal::Point {
                                            position,
                                            cause: Box::new(cause),
                                        })
                                    })?,
                            )
                        }
                        NormalizedDraftingOperation::CenterArc(arc) => {
                            DraftingProgramOrderProofKind::CenterArc(
                                arc.check_arguments_in_scope(cursor.visible_names())
                                    .map_err(|cause| {
                                        refuse(DraftingProgramOrderRefusal::CenterArc {
                                            position,
                                            cause: Box::new(cause),
                                        })
                                    })?,
                            )
                        }
                        NormalizedDraftingOperation::SlashSpread { input, opening } => {
                            crate::design::fixed_value(input);
                            DraftingProgramOrderProofKind::SlashSpread(
                                opening
                                    .check_argument_in_scope(cursor.visible_names())
                                    .map_err(|cause| {
                                        refuse(DraftingProgramOrderRefusal::SlashSpread {
                                            position,
                                            cause: Box::new(cause),
                                        })
                                    })?,
                            )
                        }
                        NormalizedDraftingOperation::Other(input) => {
                            DraftingProgramOrderProofKind::Other(*input)
                        }
                    };
                    for request in positions
                        .outputs()
                        .filter(|request| request.step().position() == position)
                    {
                        let geometry =
                            FormulaGeometryDeclaration::try_from(request.output().declaration())
                                .unwrap_or_else(|_| {
                                    unreachable!("private named native output is Point/Edge")
                                });
                        cursor.declare_geometry(geometry).map_err(|cause| {
                            refuse(DraftingProgramOrderRefusal::Header {
                                position,
                                output_index: Some(request.output_index()),
                                cause: Box::new(cause),
                            })
                        })?;
                    }
                    kind
                }
            };
            steps.push(DraftingProgramOrderStep { position, kind });
        }
        Ok(CheckedDraftingProgramOrder {
            inputs,
            limits,
            shape,
            steps,
        })
    }
}

mod retained_resources;
