//! Whole actual held-content admission, then complete shape and both original recipe orders.
use super::{
    DraftingProgramInputs, DraftingProgramLimits, DraftingProgramShape, DraftingProgramStep,
};
use crate::{
    construction::{DraftingContextSource, DraftingOperationInput},
    design::{
        charge_design_input_resources, DesignMetadataHost, DesignResourceDemand,
        DesignResourceLimitError, DesignResourceLocation, DesignResourceMeter, DesignResourceScope,
        DesignResourceSink, DesignResourceSource,
    },
    recipe::FormulaExpressionDocument,
};
use core::fmt;

/// Original whole-program refusal; the error owner retains all original inputs and policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramAdmissionRefusal {
    /// Original first actual held-field excess, before metadata arithmetic or scalar order.
    Resources(DesignResourceLimitError),
    /// Checked complete population or prospective record arithmetic is unrepresentable.
    PopulationOverflow,
    /// Complete expression population exceeds its explicit allowance.
    Expressions {
        /// Explicit selected complete allowance.
        allowed: usize,
        /// Complete actual required population, without clipping or deduplication.
        required: usize,
    },
    /// Complete node population exceeds its explicit allowance.
    Nodes {
        /// Explicit selected complete allowance.
        allowed: usize,
        /// Complete actual required population, without clipping or deduplication.
        required: usize,
    },
    /// Complete conservative prospective record population exceeds its explicit allowance.
    RetainedRecords {
        /// Explicit selected complete allowance.
        allowed: usize,
        /// Complete actual required population, without clipping or deduplication.
        required: usize,
    },
    /// First original scalar request disagrees with required original recipe order/coverage.
    ScalarOrder {
        /// Genuine one-based combined-program position, separate from the requested ordinal.
        position: usize,
        /// Next original recipe ordinal, or None when all original statements already occurred.
        expected: Option<usize>,
        /// Exact original requested scalar ordinal, including zero or out-of-range input.
        requested: usize,
    },
    /// The sequence ended before all original scalar statements/assertions were included.
    MissingScalars {
        /// Complete original scalar statement population, including assertions.
        required: usize,
        /// Original scalar statements present in correct relative order before sequence end.
        provided: usize,
    },
    /// First semantic request disagrees with actual Root semantic recipe order/coverage.
    SemanticOrder {
        /// Genuine one-based mixed position, independent of either recipe's ordinal.
        position: usize,
        /// Next original semantic ordinal, or None after the complete actual cohort.
        expected: Option<usize>,
        /// Exact original request, including zero or out-of-range input.
        requested: usize,
    },
    /// Sequence ended before the complete actual Root semantic cohort was included.
    MissingSemantics {
        /// Complete actual semantic recipe population.
        required: usize,
        /// Semantic operations present in correct relative order before sequence end.
        provided: usize,
    },
}
/// Failed whole admission, borrowing complete original data and explicit policy.
/// A box keeps uncommon semantic/content diagnostics compact; no normalized allocation error is
/// wrapped here and no child factory or accepted prefix is created by a refusal.
pub struct DraftingProgramAdmissionError<
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    inputs: &'p DraftingProgramInputs<'a, H, S>,
    limits: &'p DraftingProgramLimits,
    refusal: Box<DraftingProgramAdmissionRefusal>,
}
impl<'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramAdmissionError<'p, 'a, H, S>
{
    /// Complete original mixed sequence, actual context and original Root recipe.
    #[must_use]
    pub const fn inputs(&self) -> &'p DraftingProgramInputs<'a, H, S> {
        self.inputs
    }
    /// Original explicit caller-owned complete policy.
    #[must_use]
    pub const fn limits(&self) -> &'p DraftingProgramLimits {
        self.limits
    }
    /// Original first whole-content/population/order cause; no admitted prefix is returned.
    #[must_use]
    pub fn refusal(&self) -> DraftingProgramAdmissionRefusal {
        *self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramAdmissionError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramAdmissionError")
            .field("limits", self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingProgramAdmissionError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "whole drafting input admission: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingProgramAdmissionError<'_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal.as_ref() {
            DraftingProgramAdmissionRefusal::Resources(cause) => Some(cause),
            _ => None,
        }
    }
}

/// Complete held-content, prospective shape and both recipe coverage/order admission.
/// No names, kinds, output dependencies, geometry values, runtime or release approval are proved.
/// ```compile_fail,E0616
/// use sc_core::{construction::{AdmittedDraftingProgramInputs,DraftingContextSource},
///     design::DesignMetadataHost};
/// fn substitute<H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     admitted:&mut AdmittedDraftingProgramInputs<'_, '_,H,S>) { admitted.inputs=todo!(); }
/// ```
pub struct AdmittedDraftingProgramInputs<
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    inputs: &'p DraftingProgramInputs<'a, H, S>,
    limits: &'p DraftingProgramLimits,
    demand: DesignResourceDemand,
    shape: DraftingProgramShape,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for AdmittedDraftingProgramInputs<'_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for AdmittedDraftingProgramInputs<'_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    AdmittedDraftingProgramInputs<'p, 'a, H, S>
{
    pub(super) fn resource_inputs(&self) -> &'p DraftingProgramInputs<'a, H, S> {
        use crate::design::fixed_value;
        let Self {
            inputs,
            limits,
            demand,
            shape,
        } = self;
        fixed_value(*limits);
        fixed_value(demand);
        fixed_value(shape);
        inputs
    }

    /// Actual original complete mixed source owner; no replacement owner is accepted.
    #[must_use]
    pub const fn inputs(&self) -> &'p DraftingProgramInputs<'a, H, S> {
        self.inputs
    }
    /// Original explicit whole-program policy, not a component demand reused as authority.
    #[must_use]
    pub const fn limits(&self) -> &'p DraftingProgramLimits {
        self.limits
    }
    /// Fresh cumulative complete held-content demand under exhaustive source contracts.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Complete actual prospective shape admitted before scalar order checks or child factories.
    #[must_use]
    pub const fn shape(&self) -> DraftingProgramShape {
        self.shape
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for AdmittedDraftingProgramInputs<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingProgramInputs")
            .field("shape", &self.shape)
            .field("demand", &self.demand)
            .finish_non_exhaustive()
    }
}

impl<'a, H: DesignResourceSource, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramInputs<'a, H, S>
{
    /// Freshly charge the actual Root, complete context source, mixed Vec and all native fields;
    /// then bound all source/recipe/native expression nodes and conservative prospective records;
    /// only then require every original scalar statement and semantic operation exactly once in
    /// each actual recipe's original relative order. The first wrong mixed request wins;at sequence
    /// end,missing scalar coverage precedes missing semantic coverage.
    /// No reparse, normalization, value, geometry lookup, identity set or accepted prefix is created.
    /// Existing Root/context demands are not trusted as a replacement for this cumulative walk.
    /// # Errors
    /// Complete content,checked populations/quotas,then original recipe order/coverage refusal.
    /// Every refusal borrows the complete original source owner and explicit policy.
    pub fn admit<'p>(
        &'p self,
        limits: &'p DraftingProgramLimits,
    ) -> Result<
        AdmittedDraftingProgramInputs<'p, 'a, H, S>,
        DraftingProgramAdmissionError<'p, 'a, H, S>,
    > {
        let refuse = |refusal| DraftingProgramAdmissionError {
            inputs: self,
            limits,
            refusal: Box::new(refusal),
        };
        let demand = charge(self, limits)
            .map_err(|e| refuse(DraftingProgramAdmissionRefusal::Resources(e)))?;
        let shape = measure(self, limits).map_err(refuse)?;
        recipe_order(self, shape.statements, shape.semantics).map_err(refuse)?;
        Ok(AdmittedDraftingProgramInputs {
            inputs: self,
            limits,
            demand,
            shape,
        })
    }
}
fn charge<H: DesignResourceSource, S: DraftingContextSource<H> + ?Sized>(
    inputs: &DraftingProgramInputs<'_, H, S>,
    limits: &DraftingProgramLimits,
) -> Result<DesignResourceDemand, DesignResourceLimitError> {
    let mut meter = DesignResourceMeter::new(limits.content());
    inputs.charge_resources(&mut meter)?;
    meter.finish()
}
impl<H: DesignResourceSource, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramInputs<'_, H, S>
{
    /// Append the complete actual Root,context source,mixed vector and native held fields to a
    /// cumulative restricted sink. This reuses the original admission walk without trusting old
    /// demands,normalizing,looking up geometry or reading values. Normalized arenas compose later.
    /// # Errors
    /// Original first field refusal,including an ignored source-sink failure and empty input.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        // Exhaustive held owner; future fields must receive accounting before this compiles.
        let DraftingProgramInputs { context, steps } = self;
        let root = context.scope().root();
        charge_design_input_resources(root, sink)?;
        let source_result = context.source().charge_resources(sink);
        let location = DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(root.id()),
            field: "drafting.program.steps",
            index: None,
        };
        // Catch ignored provider failures before a later returned cause, even for an empty sequence.
        sink.charge_entries(0, location)?;
        source_result?;
        sink.charge_entries(steps.len(), location)?;
        for step in steps {
            match step {
                DraftingProgramStep::Scalar { ordinal }
                | DraftingProgramStep::Semantic { ordinal } => {
                    crate::design::fixed_value(ordinal);
                }
                DraftingProgramStep::Drafting(input) => input.charge_resources(sink)?,
            }
        }
        Ok(())
    }
}
fn add(left: usize, right: usize) -> Result<usize, DraftingProgramAdmissionRefusal> {
    left.checked_add(right)
        .ok_or(DraftingProgramAdmissionRefusal::PopulationOverflow)
}
fn documents<const N: usize>(
    docs: [&FormulaExpressionDocument; N],
) -> Result<(usize, usize), DraftingProgramAdmissionRefusal> {
    Ok((
        N,
        docs.into_iter()
            .try_fold(0, |n, doc| add(n, doc.node_count()))?,
    ))
}
fn native(
    input: DraftingOperationInput<'_>,
) -> Result<(usize, usize), DraftingProgramAdmissionRefusal> {
    use DraftingOperationInput as I;
    match input {
        I::ConstructPoint(p) => documents(p.resource_documents()),
        I::DrawArc(a) => documents(a.resource_documents()),
        I::SlashSpread(s) => documents([s.resource_opening().resource_document()]),
        I::DrawLine(_)
        | I::DrawBezier(_)
        | I::OffsetEdge(_)
        | I::CloseDart { .. }
        | I::AddNotch { .. }
        | I::PlaceGrainline(_)
        | I::WalkSeam(_)
        | I::TrueSeam(_)
        | I::MirrorPiece(_)
        | I::JoinPieces(_) => Ok((0, 0)),
    }
}
fn measure<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>(
    inputs: &DraftingProgramInputs<'_, H, S>,
    limits: &DraftingProgramLimits,
) -> Result<DraftingProgramShape, DraftingProgramAdmissionRefusal> {
    use DraftingProgramAdmissionRefusal as R;
    let recipe = inputs.recipe().shape();
    let sources = inputs.context().scope().root().formula_sources().sources();
    let mut expressions = add(sources.len(), recipe.expression_count())?;
    let mut nodes = recipe.node_count();
    for source in sources {
        nodes = add(nodes, source.expression().node_count())?;
    }
    let mut arguments = 0;
    for step in inputs.steps() {
        if let DraftingProgramStep::Drafting(input) = step {
            let (count, n) = native(*input)?;
            arguments = add(arguments, count)?;
            expressions = add(expressions, count)?;
            nodes = add(nodes, n)?;
        }
    }
    let retained_records = nodes
        .checked_mul(2)
        .and_then(|n| n.checked_add(expressions))
        .and_then(|n| n.checked_add(recipe.statement_count()))
        .and_then(|n| n.checked_add(inputs.steps().len()))
        .ok_or(R::PopulationOverflow)?;
    if expressions > limits.expressions() {
        return Err(R::Expressions {
            allowed: limits.expressions(),
            required: expressions,
        });
    }
    if nodes > limits.nodes() {
        return Err(R::Nodes {
            allowed: limits.nodes(),
            required: nodes,
        });
    }
    if retained_records > limits.retained_records() {
        return Err(R::RetainedRecords {
            allowed: limits.retained_records(),
            required: retained_records,
        });
    }
    Ok(DraftingProgramShape {
        steps: inputs.steps().len(),
        statements: recipe.statement_count(),
        semantics: inputs.semantic_recipe().operations().len(),
        sources: sources.len(),
        arguments,
        expressions,
        nodes,
        retained_records,
    })
}
fn recipe_order<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>(
    inputs: &DraftingProgramInputs<'_, H, S>,
    statements: usize,
    semantics: usize,
) -> Result<(), DraftingProgramAdmissionRefusal> {
    use DraftingProgramAdmissionRefusal as R;
    let (mut scalars, mut semantic) = (0, 0);
    for (index, step) in inputs.steps().iter().enumerate() {
        let (ordinal, matched, count, is_semantic) = match step {
            DraftingProgramStep::Scalar { ordinal } => (ordinal, &mut scalars, statements, false),
            DraftingProgramStep::Semantic { ordinal } => (ordinal, &mut semantic, semantics, true),
            DraftingProgramStep::Drafting(_) => continue,
        };
        let next = add(*matched, 1)?;
        let expected = (*matched < count).then_some(next);
        if expected != Some(*ordinal) {
            let position = add(index, 1)?;
            return Err(if is_semantic {
                R::SemanticOrder {
                    position,
                    expected,
                    requested: *ordinal,
                }
            } else {
                R::ScalarOrder {
                    position,
                    expected,
                    requested: *ordinal,
                }
            });
        }
        *matched = next;
    }
    // End-of-sequence omissions use fixed schema order: scalar, then semantic.
    if scalars != statements {
        return Err(R::MissingScalars {
            required: statements,
            provided: scalars,
        });
    }
    if semantic != semantics {
        return Err(R::MissingSemantics {
            required: semantics,
            provided: semantic,
        });
    }
    Ok(())
}
