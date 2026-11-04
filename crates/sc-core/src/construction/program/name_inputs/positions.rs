//! Complete original output positions;variable-role catalogue candidates still need family proofs.
use super::{AdmittedDraftingProgramNameWork, DraftingGeometryNameSource as G, DraftingOutputName};
use crate::{
    construction::{
        DraftingContextSource, DraftingOperationFamily, DraftingOperationInput as I,
        DraftingProgramStep, DraftingProgramStepView,
    },
    design::DesignMetadataHost,
    ontology::EntityId,
};
use core::{fmt, iter::FusedIterator};

/// Original requested output position or actual fixed-source mismatch;no manufactured span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingOutputPositionRefusal {
    /// Zero or a position outside the complete original mixed sequence.
    MissingStep,
    /// Requested position is an actual scalar statement,not a creating native input.
    ScalarStep {
        /// Genuine ordinal in the whole original scalar recipe.
        ordinal: usize,
    },
    /// Requested position is a Root semantic operation, with no native Point/Edge output.
    SemanticStep {
        /// Genuine original semantic recipe ordinal, independent of mixed/scalar positions.
        ordinal: usize,
    },
    /// Actual fixed source family differs from the native input at the requested position.
    Family {
        /// Family required by the original fixed source variant.
        expected: DraftingOperationFamily,
        /// Actual family at the original mixed position.
        actual: DraftingOperationFamily,
    },
    /// Matching family but a different actual schema owner,even if its fields/ids compare equal.
    FixedOwner,
    /// Variable-role catalogue names another authored creator;matching ids alone prove no manifest.
    RoleCreator {
        /// Actual catalogue's original operation id.
        requested: EntityId,
        /// Actual native input's authored operation id.
        actual: EntityId,
    },
}
/// Complete refusal retains the actual bounded work and original named-output occurrence.
pub struct DraftingOutputPositionError<
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    work: AdmittedDraftingProgramNameWork<'b, 'n, 'p, 'a, H, S>,
    index: usize,
    refusal: Box<DraftingOutputPositionRefusal>,
}
impl<'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingOutputPositionError<'b, 'n, 'p, 'a, H, S>
{
    /// Actual original work/input/policy and complete admitted populations.
    #[must_use]
    pub const fn work(&self) -> &AdmittedDraftingProgramNameWork<'b, 'n, 'p, 'a, H, S> {
        &self.work
    }
    /// Zero-based original output-name position,not a program or scalar ordinal.
    #[must_use]
    pub const fn output_index(&self) -> usize {
        self.index
    }
    /// Actual original authored output name,requested position and source handle.
    #[must_use]
    pub fn output(&self) -> &DraftingOutputName<'n> {
        self.work
            .inputs()
            .outputs()
            .get(self.index)
            .unwrap_or_else(|| unreachable!("private original output occurrence invariant"))
    }
    /// Exact first position/fixed-owner/role-creator refusal.
    #[must_use]
    pub fn refusal(&self) -> DraftingOutputPositionRefusal {
        *self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingOutputPositionError<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingOutputPositionError")
            .field("output_index", &self.index)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingOutputPositionError<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting output position: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingOutputPositionError<'_, '_, '_, '_, H, S>
{
}

/// All original output-name occurrences tied to their actual mixed native steps.
/// Fixed schemas match actual owner identity. Variable-role catalogues retain only matching
/// authored creator candidates;actual family proof parents/manifests remain required next.
/// This grants no role issuance,source/currentness,namespace,kind,geometry or program approval.
/// ```compile_fail,E0616
/// use sc_core::{construction::{DraftingProgramOutputPositions,DraftingContextSource},design::DesignMetadataHost};
/// fn rewrite<H:DesignMetadataHost,S:DraftingContextSource<H>>(positions:&mut DraftingProgramOutputPositions<'_, '_, '_, '_,H,S>) {
///     let _=&positions.work;
/// }
/// ```
pub struct DraftingProgramOutputPositions<
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    work: AdmittedDraftingProgramNameWork<'b, 'n, 'p, 'a, H, S>,
}
impl<'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramOutputPositions<'b, 'n, 'p, 'a, H, S>
{
    /// Actual complete bounded work that preceded every position/source comparison.
    #[must_use]
    pub const fn work(&self) -> &AdmittedDraftingProgramNameWork<'b, 'n, 'p, 'a, H, S> {
        &self.work
    }
    /// Borrow every original output occurrence in authored output-vector order. Direct lookup
    /// retains the genuine combined position and actual normalized step arena;no copied index,
    /// sorting,deduplication,renumbering or scalar cursor is used. No accepted prefix is exposed.
    pub fn outputs(
        &self,
    ) -> impl ExactSizeIterator<Item = DraftingOutputPosition<'_, 'n, 'a>>
           + DoubleEndedIterator
           + FusedIterator {
        self.work
            .inputs()
            .outputs()
            .iter()
            .enumerate()
            .map(|(index, output)| {
                let position = output
                    .position()
                    .checked_sub(1)
                    .unwrap_or_else(|| unreachable!("private nonzero output-position invariant"));
                let step = self
                    .work
                    .inputs()
                    .program()
                    .steps()
                    .nth(position)
                    .unwrap_or_else(|| {
                        unreachable!("private complete immutable output-position invariant")
                    });
                DraftingOutputPosition {
                    index,
                    output,
                    step,
                }
            })
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramOutputPositions<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramOutputPositions")
            .field("output_count", &self.work.inputs().outputs().len())
            .finish_non_exhaustive()
    }
}
/// One genuine original name/native-step loan from a complete position association.
/// Fields are private;this view cannot survive its containing position owner's loan.
/// ```compile_fail
/// use sc_core::{construction::{DraftingContextSource,DraftingOutputPosition,DraftingProgramOutputPositions},design::DesignMetadataHost};
/// fn escape<'v,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     positions:&'v DraftingProgramOutputPositions<'b,'n,'p,'a,H,S>)
///     ->Option<DraftingOutputPosition<'static,'n,'a>> { positions.outputs().next() }
/// ```
#[derive(Clone, Copy)]
pub struct DraftingOutputPosition<'v, 'n, 'a> {
    index: usize,
    output: &'v DraftingOutputName<'n>,
    step: DraftingProgramStepView<'v, 'a>,
}
impl<'v, 'n, 'a> DraftingOutputPosition<'v, 'n, 'a> {
    /// Original zero-based output-name occurrence,independent of mixed/scalar positions.
    #[must_use]
    pub const fn output_index(self) -> usize {
        self.index
    }
    /// Original authored name,requested position and actual source handle.
    #[must_use]
    pub const fn output(self) -> &'v DraftingOutputName<'n> {
        self.output
    }
    /// Genuine original mixed position and actual normalized native arena loan.
    #[must_use]
    pub const fn step(self) -> DraftingProgramStepView<'v, 'a> {
        self.step
    }
    /// True for catalogue candidates that still require actual published family proof parents.
    /// False identifies an actual fixed-schema owner match,not current/live geometry.
    #[must_use]
    pub const fn requires_role_binding(self) -> bool {
        matches!(self.output.source(), G::PointRole(_) | G::EdgeRole(_))
    }
}
impl fmt::Debug for DraftingOutputPosition<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingOutputPosition")
            .field("output_index", &self.index)
            .field("step", &self.step)
            .finish_non_exhaustive()
    }
}
fn fixed(
    expected: DraftingOperationFamily,
    input: I<'_>,
    same: Option<bool>,
) -> Option<DraftingOutputPositionRefusal> {
    match same {
        Some(true) => None,
        Some(false) => Some(DraftingOutputPositionRefusal::FixedOwner),
        None => Some(DraftingOutputPositionRefusal::Family {
            expected,
            actual: input.family(),
        }),
    }
}
fn source(source: G<'_>, input: I<'_>) -> Option<DraftingOutputPositionRefusal> {
    match source {
        G::Point(p) => fixed(
            DraftingOperationFamily::ConstructPoint,
            input,
            if let I::ConstructPoint(actual) = input {
                Some(std::ptr::eq(p, actual))
            } else {
                None
            },
        ),
        G::Line(l) => fixed(
            DraftingOperationFamily::DrawLine,
            input,
            if let I::DrawLine(actual) = input {
                Some(std::ptr::eq(l, actual))
            } else {
                None
            },
        ),
        G::CenterArc(a) => fixed(
            DraftingOperationFamily::DrawArc,
            input,
            if let I::DrawArc(actual) = input {
                Some(std::ptr::eq(a, actual))
            } else {
                None
            },
        ),
        G::Bezier(b) => fixed(
            DraftingOperationFamily::DrawBezier,
            input,
            if let I::DrawBezier(actual) = input {
                Some(std::ptr::eq(b, actual))
            } else {
                None
            },
        ),
        G::PointRole(_) | G::EdgeRole(_) => {
            let requested = match source.reference() {
                crate::construction::DraftingGeometryReference::Point(p) => p.creator(),
                crate::construction::DraftingGeometryReference::Edge(e) => e.creator(),
            };
            (requested != input.operation()).then_some(DraftingOutputPositionRefusal::RoleCreator {
                requested,
                actual: input.operation(),
            })
        }
    }
}
impl<'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    AdmittedDraftingProgramNameWork<'b, 'n, 'p, 'a, H, S>
{
    /// Bind the complete original output vector to actual original mixed creating positions.
    /// Fixed schemas require exact actual owners,not equal ids/fields. Variable-role metadata
    /// retains matching-creator candidates only;published family proofs/manifests are still required.
    /// Success allocates no index or namespace and performs no callbacks,kind checks or values.
    ///
    /// ```
    /// use sc_core::{construction::{AdmittedDraftingProgramNameWork, DraftingContextSource},
    ///     design::DesignMetadataHost};
    /// fn inspect_positions<H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    ///     work:AdmittedDraftingProgramNameWork<'_, '_, '_, '_,H,S>)->Result<usize,String> {
    ///     let positions=work.bind_output_positions().map_err(|error|error.to_string())?;
    ///     for output in positions.outputs() {
    ///         assert_eq!(output.step().position(),output.output().position());
    ///     }
    ///     let count=positions.outputs().len();
    ///     Ok(count)
    /// }
    /// ```
    /// # Errors
    /// First authored output occurrence with zero/missing/scalar position,wrong fixed family or
    /// actual owner,or different role creator. Original bounded work/name/source remains intact;
    /// a failure returns no accepted prefix. Collisions/source truth are separate later checks.
    pub fn bind_output_positions(
        self,
    ) -> Result<
        DraftingProgramOutputPositions<'b, 'n, 'p, 'a, H, S>,
        DraftingOutputPositionError<'b, 'n, 'p, 'a, H, S>,
    > {
        for (index, output) in self.inputs().outputs().iter().enumerate() {
            let step = output.position().checked_sub(1).and_then(|position| {
                self.inputs()
                    .program()
                    .admitted()
                    .inputs()
                    .steps()
                    .get(position)
            });
            let refusal = match step {
                None => Some(DraftingOutputPositionRefusal::MissingStep),
                Some(DraftingProgramStep::Scalar { ordinal }) => {
                    Some(DraftingOutputPositionRefusal::ScalarStep { ordinal: *ordinal })
                }
                Some(DraftingProgramStep::Drafting(input)) => source(output.source(), *input),
                Some(DraftingProgramStep::Semantic { ordinal }) => {
                    Some(DraftingOutputPositionRefusal::SemanticStep { ordinal: *ordinal })
                }
            };
            if let Some(refusal) = refusal {
                return Err(DraftingOutputPositionError {
                    work: self,
                    index,
                    refusal: Box::new(refusal),
                });
            }
        }
        Ok(DraftingProgramOutputPositions { work: self })
    }
}
