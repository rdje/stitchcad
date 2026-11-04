//! Original occurrences and complete creator classification, before any geometry source lookup.
use super::{
    BoundDraftingProgramCanonicalInputs, DraftingProgramFormulaGeometryConsumer,
    DraftingProgramFormulaGeometryUse, DraftingProgramNativeGeometryUse,
};
use crate::{
    construction::{
        DraftingGeometryReference as R, DraftingInitialInputProvider, DraftingOperationInput as I,
        DraftingProgramStepKind, DraftingProgramStepView, NormalizedDraftingProgram,
    },
    design::DesignMetadataHost,
};
use core::{fmt, iter::FusedIterator};

/// Original formula or native geometry occurrence. This enum alone grants no source association.
#[derive(Clone, Copy, Debug)]
pub enum DraftingProgramGeometryOccurrence<'v, 'j> {
    /// Actual original checked Root source or complete mixed canonical occurrence.
    Formula(DraftingProgramFormulaGeometryUse<'v, 'j>),
    /// Actual original direct native field and its checked mixed consumer.
    Native(DraftingProgramNativeGeometryUse<'v, 'j>),
}
impl DraftingProgramGeometryOccurrence<'_, '_> {
    /// Actual one-based mixed consumer position; Root sources have no invented mixed position.
    #[must_use]
    pub fn position(self) -> Option<usize> {
        match self {
            Self::Formula(u) => match u.consumer() {
                DraftingProgramFormulaGeometryConsumer::Source { .. } => None,
                DraftingProgramFormulaGeometryConsumer::Mixed(slot) => {
                    Some(slot.occurrence().position())
                }
            },
            Self::Native(u) => Some(u.position()),
        }
    }
    /// Original persistent reference, without source lookup or selector evaluation.
    #[must_use]
    pub fn reference(self) -> R {
        match self {
            Self::Formula(u) => u.reference(),
            Self::Native(u) => u.reference(),
        }
    }
}
/// Actual strictly earlier native creating step and original requested reference, including unnamed
/// outputs. Private construction certifies creator classification only, not a valid geometry source.
/// ```compile_fail,E0616
/// fn alter(r:&mut sc_core::construction::DraftingGeometryStepRequest<'_, '_>) {r.reference=todo!();}
/// ```
/// ```compile_fail
/// fn escape<'v,'a>(r:sc_core::construction::DraftingGeometryStepRequest<'v,'a>)
///     ->sc_core::construction::DraftingGeometryStepRequest<'static,'a> {r}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct DraftingGeometryStepRequest<'v, 'a> {
    step: DraftingProgramStepView<'v, 'a>,
    reference: R,
    variable: bool,
}
impl<'v, 'a> DraftingGeometryStepRequest<'v, 'a> {
    /// Genuine original creating position and actual normalized native owner.
    #[must_use]
    pub const fn step(self) -> DraftingProgramStepView<'v, 'a> {
        self.step
    }
    /// Original requested PointRef or EdgeRef; no global name is fabricated.
    #[must_use]
    pub const fn reference(self) -> R {
        self.reference
    }
    /// Actual original native creating input, not a matching-id replacement.
    #[must_use]
    pub fn input(self) -> I<'a> {
        match self.step.kind() {
            DraftingProgramStepKind::Drafting(input) => input.input(),
            DraftingProgramStepKind::Scalar { .. } | DraftingProgramStepKind::Semantic { .. } => {
                unreachable!("private native creator request")
            }
        }
    }
    /// Variable output requires an authentic original family field; fixed output is schema-checked.
    #[must_use]
    pub const fn requires_role_binding(self) -> bool {
        self.variable
    }
}
/// Creator classification, not accepted geometry/frame/currentness or complete programme proof.
#[derive(Clone, Copy, Debug)]
pub enum DraftingProgramGeometryTarget<'v, 'a> {
    /// No native/semantic operation with this creator ID; existing-source binding is still owed.
    Existing,
    /// Actual strictly earlier creating step, with a fixed output or unresolved authentic role.
    Earlier(DraftingGeometryStepRequest<'v, 'a>),
}
/// Original first creator refusal with genuine original step witnesses; no source was queried.
#[derive(Clone, Copy, Debug)]
pub enum DraftingProgramGeometryCreatorRefusal<'v, 'a> {
    /// Original semantic operation has no native Point/Edge output; its ID cannot fall through
    /// to an existing geometry provider or be reinterpreted as a native family.
    SemanticCreator {
        /// Actual Root member and original semantic/mixed positions.
        step: DraftingProgramStepView<'v, 'a>,
    },
    /// Two actual native/semantic steps carry this identity;no earlier-prefix selection is allowed.
    Ambiguous {
        /// First original candidate.
        first: DraftingProgramStepView<'v, 'a>,
        /// Second original candidate, including one later than the consumer.
        second: DraftingProgramStepView<'v, 'a>,
    },
    /// A Root source cannot depend on any programme-created output.
    RootSource {
        /// Actual programme candidate.
        step: DraftingProgramStepView<'v, 'a>,
    },
    /// Actual candidate is the consumer itself.
    SelfCreator {
        /// Actual same-position candidate.
        step: DraftingProgramStepView<'v, 'a>,
    },
    /// Actual candidate is later than this consumer.
    FutureCreator {
        /// Actual future candidate.
        step: DraftingProgramStepView<'v, 'a>,
    },
    /// Fixed schema has another exact Point/Edge reference or tag.
    FixedOutput {
        /// Actual creating step.
        step: DraftingProgramStepView<'v, 'a>,
        /// Actual fixed output, including its kind/tag.
        actual: R,
    },
    /// This native family declares no direct Point/Edge output for this request.
    Unsupported {
        /// Actual family owner.
        step: DraftingProgramStepView<'v, 'a>,
    },
    /// This variable family supports Edge outputs only; a Point was requested.
    OutputKind {
        /// Actual variable family owner.
        step: DraftingProgramStepView<'v, 'a>,
    },
}
/// One original occurrence and its complete creator classification. A refusal stays beside its
/// original consumer/reference; iteration may continue, without certifying an accepted prefix.
/// No diagnostic allocation or source/provider/value callback occurs.
/// ```compile_fail,E0616
/// fn alter(r:&mut sc_core::construction::DraftingProgramGeometryRequest<'_, '_>) {r.target=todo!();}
/// ```
#[derive(Clone, Copy)]
pub struct DraftingProgramGeometryRequest<'v, 'j> {
    occurrence: DraftingProgramGeometryOccurrence<'v, 'j>,
    target: Result<
        DraftingProgramGeometryTarget<'v, 'j>,
        DraftingProgramGeometryCreatorRefusal<'v, 'j>,
    >,
}
impl<'v, 'j> DraftingProgramGeometryRequest<'v, 'j> {
    /// Exact original formula/native consumer and original typed selector/name metadata.
    #[must_use]
    pub const fn occurrence(self) -> DraftingProgramGeometryOccurrence<'v, 'j> {
        self.occurrence
    }
    /// Creator classification or first original refusal. No live or planned supplier is accepted.
    /// # Errors
    /// Complete creator ambiguity, Root-source, self/future, fixed-reference or family-kind refusal.
    pub const fn target(
        self,
    ) -> Result<DraftingProgramGeometryTarget<'v, 'j>, DraftingProgramGeometryCreatorRefusal<'v, 'j>>
    {
        self.target
    }
}
impl fmt::Debug for DraftingProgramGeometryRequest<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGeometryRequest")
            .field("occurrence", &self.occurrence)
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}
fn classify<'v, 'j, H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized>(
    program: &'v NormalizedDraftingProgram<'_, 'j, H, S>,
    occurrence: DraftingProgramGeometryOccurrence<'v, 'j>,
) -> Result<DraftingProgramGeometryTarget<'v, 'j>, DraftingProgramGeometryCreatorRefusal<'v, 'j>> {
    use DraftingProgramGeometryCreatorRefusal as E;
    let reference = occurrence.reference();
    let creator = match reference {
        R::Point(p) => p.creator(),
        R::Edge(e) => e.creator(),
    };
    let mut candidates = program.steps().filter(|step| match step.kind() {
        DraftingProgramStepKind::Drafting(input) => input.input().operation() == creator,
        DraftingProgramStepKind::Semantic { operation, .. } => operation.id() == creator,
        DraftingProgramStepKind::Scalar { .. } => false,
    });
    let Some(step) = candidates.next() else {
        return Ok(DraftingProgramGeometryTarget::Existing);
    };
    if let Some(second) = candidates.next() {
        return Err(E::Ambiguous {
            first: step,
            second,
        });
    }
    if matches!(step.kind(), DraftingProgramStepKind::Semantic { .. }) {
        return Err(E::SemanticCreator { step });
    }
    let Some(position) = occurrence.position() else {
        return Err(E::RootSource { step });
    };
    if step.position() == position {
        return Err(E::SelfCreator { step });
    }
    if step.position() > position {
        return Err(E::FutureCreator { step });
    }
    let DraftingProgramStepKind::Drafting(operation) = step.kind() else {
        unreachable!("private native candidate filter");
    };
    let actual = match operation.input() {
        I::ConstructPoint(p) => Some(R::Point(p.output())),
        I::DrawLine(l) => Some(R::Edge(l.output())),
        I::DrawArc(a) => Some(R::Edge(a.output())),
        I::DrawBezier(b) => Some(R::Edge(b.output())),
        I::OffsetEdge(_) | I::SlashSpread(_) | I::TrueSeam(_) | I::JoinPieces(_) => {
            if matches!(reference, R::Point(_)) {
                return Err(E::OutputKind { step });
            }
            None
        }
        I::MirrorPiece(_) => None,
        I::CloseDart { .. } | I::AddNotch { .. } | I::PlaceGrainline(_) | I::WalkSeam(_) => {
            return Err(E::Unsupported { step })
        }
    };
    if let Some(actual) = actual {
        if actual != reference {
            return Err(E::FixedOutput { step, actual });
        }
    }
    Ok(DraftingProgramGeometryTarget::Earlier(
        DraftingGeometryStepRequest {
            step,
            reference,
            variable: actual.is_none(),
        },
    ))
}
impl<
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Classify every actual original geometry occurrence without allocation or callbacks. Root
    /// source uses come first; mixed formula/native uses merge by genuine position, formula uses
    /// before direct native fields at the same position. Each original producer retains its order.
    /// Every occurrence searches the complete actual programme, including later duplicate creators.
    /// This supplies no complete work allowance or supplier/frame/currentness/semantic acceptance.
    ///
    /// ```
    /// use sc_core::{construction::*,design::DesignMetadataHost};
    /// fn requests<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     inputs:&BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_,H,S>) -> usize {
    ///     inputs.geometry_requests().filter(|r|r.target().is_ok()).count()
    /// }
    /// ```
    pub fn geometry_requests(
        &self,
    ) -> impl FusedIterator<Item = DraftingProgramGeometryRequest<'_, 'j>> + '_ {
        let program = self
            .inputs()
            .order()
            .inputs()
            .roles()
            .positions()
            .work()
            .inputs()
            .program();
        let mut formulas = self
            .formula_geometry_dependencies()
            .map(DraftingProgramGeometryOccurrence::Formula)
            .peekable();
        let mut native = self
            .native_geometry_dependencies()
            .map(DraftingProgramGeometryOccurrence::Native)
            .peekable();
        core::iter::from_fn(move || {
            let next = match (formulas.peek(), native.peek()) {
                (Some(f), Some(n)) if f.position() <= n.position() => formulas.next(),
                (Some(_), Some(_)) | (None, Some(_)) => native.next(),
                (Some(_), None) => formulas.next(),
                (None, None) => None,
            }?;
            Some(DraftingProgramGeometryRequest {
                occurrence: next,
                target: classify(program, next),
            })
        })
        .fuse()
    }
}
