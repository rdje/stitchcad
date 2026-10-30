//! Bounded iterative source demand;all buffers reserve before values and only real roots fill outputs.
use super::{
    machine::{Machine, Progress},
    source_read::{source_index, Completed, GraphSources},
    FormulaExecutedScalarSourceGraph, FormulaScalarExecutionContext as Context,
    FormulaScalarExecutionWorkspace, FormulaScalarSourceGraphExecutionCause as Cause,
    FormulaScalarSourceGraphExecutionError as Error, FormulaScalarSourceGraphPlan as Plan,
    FormulaScalarSourceGraphWorkspace as Workspace,
};
use crate::ontology::EntityId;
use std::collections::TryReserveError;

impl<'p> Plan<'p> {
    /// Execute one exact original source and only its demanded suppliers over the actual acyclic
    /// relation. Every source's N task/N value buffers,all S controls/call frames and S output slots
    /// reserve fallibly before the first value. Existing D slots retain only successful reads.
    /// All roots remain unbound;only actual recipe lets may invoke their existing storage boundary.
    /// ```
    /// use sc_core::{ontology::EntityId,recipe::*,value::*};
    /// let formula=EntityId::from_bits(9001);
    /// let expression=FormulaExpressionDocument::new("1 mm/3".into())?;
    /// let n=expression.node_count();
    /// let limits=FormulaSourceLimits::new(1,expression.source().len(),expression.canonical().as_str().len());
    /// let sources=FormulaSources::new(vec![FormulaSource::new(formula,expression)],limits)?;
    /// let inputs=CanonicalInputs::new(vec![LengthDeclaration::new(LengthDeclarationDefinition {
    ///     id:EntityId::from_bits(10),source:EntityId::from_bits(20),
    ///     state:LengthState::Derived {formula},
    /// })?],vec![])?;
    /// let normalized=FormulaSourceInputs::new(&inputs,&sources).expect("actual join")
    ///     .normalize_sources(FormulaSourceNormalizationLimits::new(1,n,1+2*n)).expect("original arenas");
    /// let checked=normalized.check_sources(&FormulaNamespace::new([])?,
    ///     FormulaSourceCheckLimits::new(1,n,1,1+2*n)).expect("actual proof");
    /// let deps=checked.prepare_dependencies(FormulaSourceDependencyLimits::new(1,0,2)).expect("actual edges");
    /// let graph=deps.admit_acyclic(FormulaSourceCycleLimits::new(1,2)).expect("actual acyclic proof");
    /// let plan=graph.prepare_scalar_sources(FormulaSourceGraphExecutionLimits::new(1,n,0,2+2*n,1))
    ///     .expect("complete source preparation");
    /// let result=plan.evaluate_source(formula,FormulaScalarExecutionContext::new(
    ///     FormulaToleranceContext::empty(),FormulaSizeContext::empty())).expect("actual source root");
    /// assert_eq!(result.entry().value().numeric().expect("Length").rational(),FormulaRational::new(1000,3,false)?);
    /// assert_eq!(result.sources().count(),1);
    /// assert!(inputs.length(EntityId::from_bits(10)).unwrap().authored_value().is_err());
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Missing exact source selection precedes runtime allocation. Preparation preserves inline
    /// allocation causes;execution preserves actual source position,node and original scalar cause.
    /// Any failure drops the entire private outcome/cache prefix and returns no accepted result.
    pub fn evaluate_source<'a>(
        self,
        source: EntityId,
        context: Context<'a>,
    ) -> Result<FormulaExecutedScalarSourceGraph<'a>, Error<'a>>
    where
        'p: 'a,
    {
        let mut plan: Plan<'a> = self;
        let graph = plan.graph();
        let limits = plan.limits();
        let checked = graph.dependencies().checked();
        let fail = |cause| Error {
            graph,
            limits,
            source,
            cause,
        };
        let entry = source_index(checked, source).ok_or_else(|| fail(Cause::MissingSource))?;
        let s = plan.sources().len();
        let allocation = |workspace, source_index, requested, error: TryReserveError| {
            fail(Cause::Allocation {
                workspace,
                source_index,
                requested,
                error,
            })
        };
        let mut machines = Vec::new();
        machines
            .try_reserve_exact(s)
            .map_err(|e| allocation(Workspace::Sources, None, s, e))?;
        let mut frames = Vec::new();
        frames
            .try_reserve_exact(s)
            .map_err(|e| allocation(Workspace::CallFrames, None, s, e))?;
        let mut completed = Vec::new();
        completed
            .try_reserve_exact(s)
            .map_err(|e| allocation(Workspace::Outputs, None, s, e))?;
        completed.resize(s, None);
        for (index, source) in plan.sources().iter().enumerate() {
            let machine = Machine::new(source.plan().admission().node_count()).map_err(|e| {
                allocation(
                    match e.workspace {
                        FormulaScalarExecutionWorkspace::Tasks => Workspace::Tasks,
                        FormulaScalarExecutionWorkspace::Values => Workspace::Values,
                    },
                    Some(index),
                    e.requested,
                    e.error,
                )
            })?;
            machines.push(machine);
        }
        let evaluation = |source_index, error| {
            fail(Cause::Evaluation {
                source_index,
                error: Box::new(error),
            })
        };
        let start = |index: usize, machines: &mut [Machine<'a>], plan: &Plan<'a>| {
            let Some(source) = plan.sources().get(index) else {
                unreachable!("actual exact source index");
            };
            let Some(machine) = machines.get_mut(index) else {
                unreachable!("complete reserved source controls");
            };
            machine
                .start(source.plan().admission().root(), context)
                .map_err(|e| evaluation(index, e))
        };
        start(entry, &mut machines, &plan)?;
        frames.push(entry); // Exact selection implies S>0;the complete S frame buffer is reserved.
        let mut ordinal = 0;
        while let Some(index) = frames.last().copied() {
            let Some(current) = plan.sources.get_mut(index) else {
                unreachable!("original active source");
            };
            let Some(machine) = machines.get_mut(index) else {
                unreachable!("original active source machine");
            };
            let progress = machine
                .resume(&mut GraphSources {
                    plan: &mut current.plan,
                    checked,
                    completed: &completed,
                })
                .map_err(|e| evaluation(index, e))?;
            match progress {
                Progress::Complete(value) => {
                    let Some(slot) = completed.get_mut(index) else {
                        unreachable!("complete output reservation");
                    };
                    if slot.is_some() {
                        unreachable!("only an unexecuted source may enter the acyclic walk");
                    }
                    ordinal += 1;
                    *slot = Some(Completed { value, ordinal });
                    frames.pop();
                }
                Progress::AwaitDerived { node, input } => {
                    let Some(supplier) = input
                        .derived_formula()
                        .and_then(|id| source_index(checked, id))
                    else {
                        unreachable!("same immutable complete source/input/edge proofs retain actual supplier");
                    };
                    if frames.len() >= s {
                        return Err(fail(Cause::FrameReservation {
                            source_index: index,
                            span: node.span(),
                            admitted: s,
                            requested: frames.len() + 1,
                        }));
                    }
                    // The actual acyclic relation excludes an active supplier;only completed roots
                    // are cached and GraphSources returns those immediately,without another frame.
                    start(supplier, &mut machines, &plan)?;
                    frames.push(supplier);
                }
            }
        }
        Ok(FormulaExecutedScalarSourceGraph {
            plan,
            context,
            entry,
            completed,
        })
    }
}
