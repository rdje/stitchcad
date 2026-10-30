//! Actual scalar capability scope and complete canonical source binding, before any value read.
use super::FormulaExecutionAdmission;
use crate::{
    recipe::{
        FormulaBuiltin, FormulaCanonicalInput, FormulaCanonicalInputError, FormulaCheckedNode,
        FormulaCheckedNodeForm as Form, FormulaDeclarationSource, FormulaReservedName,
        FormulaTangentLimits,
    },
    value::CanonicalInputs,
};
use core::fmt;
use std::collections::TryReserveError;

/// Actual source selected for one original name occurrence; no value has been read.
#[derive(Clone, Copy, Debug)]
pub enum FormulaScalarExecutionSourceKind<'a> {
    /// Exact source/family/owner/kind join in this plan's actual canonical inventory.
    Canonical(FormulaCanonicalInput<'a>),
    /// Actual reserved role; optional provider availability remains a lazy execution concern.
    Reserved(FormulaReservedName),
}

/// One original certified name occurrence and its actual canonical or reserved source.
/// Private fields prevent attaching another node or source to this view.
#[derive(Clone, Copy, Debug)]
pub struct FormulaScalarExecutionSource<'a> {
    node: FormulaCheckedNode<'a, 'a>,
    kind: FormulaScalarExecutionSourceKind<'a>,
}
impl<'a> FormulaScalarExecutionSource<'a> {
    /// Actual checked name node, retaining the original authored span and declaration.
    #[must_use]
    pub const fn node(self) -> FormulaCheckedNode<'a, 'a> {
        self.node
    }
    /// Real inventory join or original reserved role; neither is an executed read.
    #[must_use]
    pub const fn kind(self) -> FormulaScalarExecutionSourceKind<'a> {
        self.kind
    }
}

/// Preparation scope or source failure, separate from canonical formula-language diagnostics.
/// An unavailable implementation is never a language refusal or a computed result.
#[derive(Debug)]
pub enum FormulaScalarExecutionPlanRefusal<'a> {
    /// The built-in has no implementation in this selected scalar execution scope.
    UnavailableBuiltin(FormulaBuiltin),
    /// Tan requires an explicitly selected per-call execution policy before any values.
    MissingTangentPolicy,
    /// Geometry or a prior-let source requires its real integration owner.
    UnavailableSource(FormulaDeclarationSource<'a>),
    /// Original actual inventory family, identity/kind or borrowed-owner refusal.
    CanonicalInput(FormulaCanonicalInputError<'a>),
    /// The admitted D-source workspace could not be reserved; inspect allocation_error for cause.
    Allocation {
        /// Exact admitted source-record population requested, not allocator bytes.
        requested: usize,
    },
}

#[derive(Debug)]
enum PlanIssue {
    UnavailableBuiltin(FormulaBuiltin),
    MissingTangentPolicy,
    UnavailableSource,
    CanonicalInput,
    Allocation {
        requested: usize,
        error: TryReserveError,
    },
}

/// Exact failed node/proof owner and immutable inventory, with no accepted partial source plan.
/// Debug omits authored names/content. Explicit refusal inspection reconstructs full witnesses.
#[derive(Debug)]
pub struct FormulaScalarExecutionPlanError<'a> {
    node: FormulaCheckedNode<'a, 'a>,
    inputs: &'a CanonicalInputs,
    issue: PlanIssue,
    tangent_limits: Option<FormulaTangentLimits>,
}
impl<'a> FormulaScalarExecutionPlanError<'a> {
    /// Actual unavailable or refused name/call node; allocation errors retain the actual root.
    #[must_use]
    pub const fn node(&self) -> FormulaCheckedNode<'a, 'a> {
        self.node
    }
    /// Actual supplied immutable canonical inventory.
    #[must_use]
    pub const fn inputs(&self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Original selected per-call tangent policy,including for capability/source/allocation failure.
    #[must_use]
    pub const fn tangent_limits(&self) -> Option<FormulaTangentLimits> {
        self.tangent_limits
    }
    /// Detailed original capability/source cause, without reading a value or consulting a provider.
    #[must_use]
    pub fn refusal(&self) -> FormulaScalarExecutionPlanRefusal<'a> {
        use FormulaScalarExecutionPlanRefusal as Refusal;
        match self.issue {
            PlanIssue::UnavailableBuiltin(builtin) => Refusal::UnavailableBuiltin(builtin),
            PlanIssue::MissingTangentPolicy => Refusal::MissingTangentPolicy,
            PlanIssue::UnavailableSource => {
                let Form::Name { declaration, .. } = self.node.form() else {
                    unreachable!("private source issue retains its actual checked name");
                };
                Refusal::UnavailableSource(declaration.source())
            }
            PlanIssue::CanonicalInput => {
                let Form::Name { declaration, .. } = self.node.form() else {
                    unreachable!("private inventory issue retains its actual checked name");
                };
                let Err(error) = FormulaCanonicalInput::resolve(declaration, self.inputs) else {
                    unreachable!("same immutable source/inventory reproduces the join refusal");
                };
                Refusal::CanonicalInput(error)
            }
            PlanIssue::Allocation { requested, .. } => Refusal::Allocation { requested },
        }
    }
    /// Actual allocator refusal, not a substituted quota or language diagnostic.
    #[must_use]
    pub const fn allocation_error(&self) -> Option<&TryReserveError> {
        match &self.issue {
            PlanIssue::Allocation { error, .. } => Some(error),
            _ => None,
        }
    }
    // Graph preparation preserves a nested reservation failure inline, without allocating a box
    // while already handling allocation failure. Other original semantic errors remain intact.
    pub(super) fn into_allocation_error(self) -> Result<TryReserveError, Self> {
        match self {
            Self {
                issue: PlanIssue::Allocation { error, .. },
                ..
            } => Ok(error),
            error => Err(error),
        }
    }
}
impl fmt::Display for FormulaScalarExecutionPlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "scalar execution preparation at {:?}: {:?}",
            self.node.span(),
            self.issue
        )
    }
}
impl std::error::Error for FormulaScalarExecutionPlanError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.allocation_error().map(|error| error as _)
    }
}

#[derive(Debug)]
pub(super) struct SourceSlot<'a> {
    pub(super) node: Option<FormulaCheckedNode<'a, 'a>>,
    pub(super) kind: Option<FormulaScalarExecutionSourceKind<'a>>,
    pub(super) read: Option<super::read::ReadData<'a>>,
}

/// Complete source-bound preparation for the implemented scalar execution subset, without values.
/// Tangent requires explicit per-call limits;geometry/prior-let integration remains separate.
/// Each original repeated/untaken occurrence is bound. Optional reserved providers stay unread.
/// Private fields preserve the original proof, exact inventory and complete admitted population:
/// ```compile_fail
/// fn substitute<'a>(plan: &mut sc_core::recipe::FormulaScalarExecutionPlan<'a>,
///     inputs: &'a sc_core::value::CanonicalInputs) {
///     plan.inputs = inputs;
/// }
/// ```
/// ```compile_fail
/// fn escape<'a>(admission: sc_core::recipe::FormulaExecutionAdmission<'a, 'a>)
///     -> sc_core::recipe::FormulaScalarExecutionPlan<'a> {
///     let inputs = sc_core::value::CanonicalInputs::new(vec![], vec![]).unwrap();
///     admission.prepare_scalar(&inputs).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct FormulaScalarExecutionPlan<'a> {
    pub(super) admission: FormulaExecutionAdmission<'a, 'a>,
    inputs: &'a CanonicalInputs,
    pub(super) sources: Vec<SourceSlot<'a>>,
    tangent_limits: Option<FormulaTangentLimits>,
}
impl<'a> FormulaScalarExecutionPlan<'a> {
    /// Original complete proof and explicit logical workspace reservation.
    #[must_use]
    pub const fn admission(&self) -> &FormulaExecutionAdmission<'a, 'a> {
        &self.admission
    }
    /// Exact actual canonical inventory; no replacement snapshot or source/state clone.
    #[must_use]
    pub const fn inputs(&self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Explicit per-call tangent policy,or None for the legacy scope without configured Tan.
    #[must_use]
    pub const fn tangent_limits(&self) -> Option<FormulaTangentLimits> {
        self.tangent_limits
    }
    /// Every original name occurrence in static dependency order, including repetitions/branches.
    /// These are prospective sources, not a trace of executed reads.
    pub fn sources(
        &self,
    ) -> impl ExactSizeIterator<Item = FormulaScalarExecutionSource<'a>> + DoubleEndedIterator + '_
    {
        self.sources.iter().map(|slot| {
            let (Some(node), Some(kind)) = (slot.node, slot.kind) else {
                unreachable!("a complete private plan binds every original dependency slot");
            };
            FormulaScalarExecutionSource { node, kind }
        })
    }
}

impl<'p, 'd> FormulaExecutionAdmission<'p, 'd> {
    /// Bind every actual scalar source after complete capability checking, before any value read.
    /// Capability order is certified arena order, separate from later source-order execution.
    /// One D-source vector is fallibly reserved within the existing logical workspace allowance;
    /// The separate evaluate call reserves N task/value buffers and performs actual execution.
    /// ```
    /// use sc_core::{recipe::*, value::CanonicalInputs};
    /// let inputs = CanonicalInputs::new(vec![], vec![])?;
    /// let expression = FormulaExpression::parse("if(1 mm<2 mm,1 mm,eps_phys)")?
    ///     .normalize_literals()?;
    /// let proof = expression.check_kinds(&FormulaNamespace::new([])?).expect("static fixture");
    /// let plan = proof.admit_execution(FormulaExecutionLimits::new(6, 13))
    ///     .expect("exact allowance").prepare_scalar(&inputs).expect("available scalar scope");
    /// assert_eq!(plan.sources().len(), 1); // No physical provider or value has been read.
    /// assert!(std::ptr::eq(plan.admission().root().expression(), &expression));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Missing tangent policy or unavailable geometry/prior-let precedes source allocation. Actual
    /// canonical joins then run in original dependency order. No partial plan or value escapes.
    pub fn prepare_scalar<'a>(
        self,
        inputs: &'a CanonicalInputs,
    ) -> Result<FormulaScalarExecutionPlan<'a>, FormulaScalarExecutionPlanError<'a>>
    where
        'p: 'a,
        'd: 'a,
    {
        self.prepare_scalar_with_policy(inputs, None)
    }

    /// Prepare actual scalar execution with an explicit policy for each executed tangent call.
    /// Whole capability admission still precedes source allocation/joins;untaken calls require
    /// a selected policy but consume no numerical resources or contributions. Zero quotas are
    /// explicit policies and may refuse only when the actual call is executed.
    ///
    /// ```
    /// use sc_core::{recipe::*,value::CanonicalInputs};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let inputs=CanonicalInputs::new(vec![],vec![])?;
    /// let expression=FormulaExpression::parse("tan(30 deg)")?.normalize_literals()?;
    /// let namespace=FormulaNamespace::new([])?;
    /// let proof=expression.check_kinds(&namespace).expect("actual checked fixture");
    /// let limits=FormulaTangentLimits::new(2073,512);
    /// let plan=proof.admit_execution(FormulaExecutionLimits::new(256,768)).expect("explicit workspace")
    ///     .prepare_scalar_with_tangent_limits(&inputs,limits).expect("actual policy-aware plan");
    /// assert_eq!(plan.tangent_limits(),Some(limits));
    /// let result=plan.evaluate(FormulaScalarExecutionContext::new(
    ///     FormulaToleranceContext::empty(),FormulaSizeContext::empty())).expect("actual native execution");
    /// assert_eq!(result.value().numeric().map(|v|v.rational()),
    ///     Some(FormulaRational::from_signed_integer(577350)));
    /// # Ok(())
    /// # }
    /// ```
    /// # Errors
    /// Original unavailable scope,canonical source or allocator refusal with the selected policy.
    pub fn prepare_scalar_with_tangent_limits<'a>(
        self,
        inputs: &'a CanonicalInputs,
        tangent_limits: FormulaTangentLimits,
    ) -> Result<FormulaScalarExecutionPlan<'a>, FormulaScalarExecutionPlanError<'a>>
    where
        'p: 'a,
        'd: 'a,
    {
        self.prepare_scalar_with_policy(inputs, Some(tangent_limits))
    }

    fn prepare_scalar_with_policy<'a>(
        self,
        inputs: &'a CanonicalInputs,
        tangent_limits: Option<FormulaTangentLimits>,
    ) -> Result<FormulaScalarExecutionPlan<'a>, FormulaScalarExecutionPlanError<'a>>
    where
        'p: 'a,
        'd: 'a,
    {
        let admission: FormulaExecutionAdmission<'a, 'a> = self;
        let root = admission.root();
        let refuse = |node, issue| FormulaScalarExecutionPlanError {
            node,
            inputs,
            issue,
            tangent_limits,
        };
        for node in root.all_nodes() {
            match node.form() {
                Form::Call { builtin, .. } => {
                    if builtin == FormulaBuiltin::Tan {
                        if tangent_limits.is_none() {
                            return Err(refuse(node, PlanIssue::MissingTangentPolicy));
                        }
                    } else if !scalar_builtin_available(builtin) {
                        return Err(refuse(node, PlanIssue::UnavailableBuiltin(builtin)));
                    }
                }
                Form::Name { declaration, .. } => match declaration.source() {
                    FormulaDeclarationSource::Point(_)
                    | FormulaDeclarationSource::Edge(_)
                    | FormulaDeclarationSource::Recipe { .. } => {
                        return Err(refuse(node, PlanIssue::UnavailableSource));
                    }
                    FormulaDeclarationSource::Input { .. }
                    | FormulaDeclarationSource::LengthInput { .. }
                    | FormulaDeclarationSource::Reserved(_) => {}
                },
                Form::Literal(_)
                | Form::Negate(_)
                | Form::Square(_)
                | Form::Binary { .. }
                | Form::Conditional { .. } => {}
            }
        }
        let mut sources = Vec::new();
        sources
            .try_reserve_exact(admission.name_use_count())
            .map_err(|error| {
                refuse(
                    root,
                    PlanIssue::Allocation {
                        requested: admission.name_use_count(),
                        error,
                    },
                )
            })?;
        sources.resize_with(admission.name_use_count(), || SourceSlot {
            node: None,
            kind: None,
            read: None,
        });
        for node in root.all_nodes() {
            if let Some(index) = node.dependency_index() {
                let Some(slot) = sources.get_mut(index) else {
                    unreachable!(
                        "successful same-proof name certificates retain bounded dependency indices"
                    );
                };
                slot.node = Some(node);
            }
        }
        for slot in &mut sources {
            let Some(node) = slot.node else {
                unreachable!("complete proof retains a name certificate for every dependency");
            };
            let Form::Name { declaration, .. } = node.form() else {
                unreachable!("only actual name certificates have dependency indices");
            };
            slot.kind = Some(match declaration.source() {
                FormulaDeclarationSource::Reserved(name) => {
                    FormulaScalarExecutionSourceKind::Reserved(name)
                }
                _ => FormulaScalarExecutionSourceKind::Canonical(
                    FormulaCanonicalInput::resolve(declaration, inputs)
                        .map_err(|_| refuse(node, PlanIssue::CanonicalInput))?,
                ),
            });
        }
        Ok(FormulaScalarExecutionPlan {
            admission,
            inputs,
            sources,
            tangent_limits,
        })
    }
}

// Scope without selected tangent policy,shared with source/recipe factories until policy adoption.
pub(super) fn scalar_builtin_available(builtin: FormulaBuiltin) -> bool {
    use FormulaBuiltin as B;
    match builtin {
        B::Tan | B::X | B::Y | B::Dist | B::Dir | B::Len | B::ParamAt | B::PointAt => false,
        B::Sqrt
        | B::Hypot
        | B::Abs
        | B::Min
        | B::Max
        | B::Clamp
        | B::RoundTo
        | B::Sin
        | B::Cos
        | B::Atan
        | B::Atan2
        | B::ArcLength
        | B::If
        | B::Within => true,
    }
}
