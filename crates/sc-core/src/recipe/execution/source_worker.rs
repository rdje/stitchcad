//! Bounded iterative source demand;all buffers reserve before values and only real roots fill outputs.
use super::{
    source_read::source_index, source_runtime::SourceRuntime, FormulaExecutedScalarSourceGraph,
    FormulaScalarExecutionContext as Context, FormulaScalarSourceGraphExecutionCause as Cause,
    FormulaScalarSourceGraphExecutionError as Error, FormulaScalarSourceGraphPlan as Plan,
};

use crate::ontology::EntityId;

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
    // Preserve original allocation cause and selected policy inline on pre-value failure.
    #[allow(clippy::result_large_err)]
    pub fn evaluate_source<'a>(
        self,
        source: EntityId,
        context: Context<'a>,
    ) -> Result<FormulaExecutedScalarSourceGraph<'a>, Error<'a>>
    where
        'p: 'a,
    {
        let plan: Plan<'a> = self;
        let graph = plan.graph();
        let limits = plan.limits();
        let tangent_limits = plan.tangent_limits();
        let checked = graph.dependencies().checked();
        let fail = |cause| Error {
            graph,
            limits,
            tangent_limits,
            source,
            cause,
        };
        let entry = source_index(checked, source).ok_or_else(|| fail(Cause::MissingSource))?;
        let mut runtime = SourceRuntime::new(plan, context).map_err(fail)?;
        runtime.demand(entry).map_err(fail)?;
        let completed = runtime.finish();
        Ok(FormulaExecutedScalarSourceGraph {
            plan: completed.plan,
            context: completed.context,
            entry,
            completed: completed.completed,
        })
    }
}
