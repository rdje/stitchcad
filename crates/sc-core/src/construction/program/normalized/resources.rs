//! Fresh complete raw input charges and actual held normalized vectors,without prospective totals.
use super::{NormalizedDraftingOperation, NormalizedDraftingProgram};
use crate::{
    construction::DraftingContextSource,
    design::{
        fixed_value, DesignMetadataHost, DesignResourceLimitError, DesignResourceLocation,
        DesignResourceScope, DesignResourceSink, DesignResourceSource,
    },
    ontology::EntityId,
    recipe::FormulaNormalizedExpression,
};
fn location(owner: EntityId, field: &'static str, index: Option<usize>) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(owner),
        field,
        index,
    }
}
fn expressions<const N: usize>(
    docs: [&FormulaNormalizedExpression<'_>; N],
    fields: [(&'static str, &'static str); N],
    owner: EntityId,
    index: usize,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    for (expression, (nodes, arguments)) in docs.into_iter().zip(fields) {
        expression.charge_retained_resources(
            sink,
            location(owner, nodes, Some(index)),
            location(owner, arguments, Some(index)),
        )?;
    }
    Ok(())
}
impl<H: DesignResourceSource, S: DraftingContextSource<H> + ?Sized>
    NormalizedDraftingProgram<'_, '_, H, S>
{
    /// Freshly charge all actual raw Root/context/program fields,then complete retained source,
    /// scalar statement and native-slot arenas. Actual node and call-index vector lengths count;
    /// conservative normalization bounds or old demands never replace this held-field walk.
    /// Borrowed names/source and fixed Copy literals own no additional text/counter buffers.
    ///
    /// Every schema is exhaustively classified before traversal;no parsing,normalization,lookup,
    /// values,cloning or extra arena/index allocation occurs. Additional name fields compose through
    /// their separate component producer. This grants no namespace,source truth or program approval.
    ///
    /// ```
    /// use sc_core::{construction::{DraftingContextSource, NormalizedDraftingProgram},
    ///     design::{DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
    ///     DesignResourceMeter, DesignResourceSource}};
    /// fn program_demand<H:DesignResourceSource,S:DraftingContextSource<H> + ?Sized>(
    ///     program:&NormalizedDraftingProgram<'_, '_,H,S>,policy:DesignResourceLimits)
    ///     -> Result<DesignResourceDemand,DesignResourceLimitError> {
    ///     let mut meter=DesignResourceMeter::new(policy);
    ///     program.charge_resources(&mut meter)?;
    ///     meter.finish()
    /// }
    /// ```
    /// # Errors
    /// Original first actual raw/retained field excess with original source/operation/role/index.
    /// Empty owners retain their actual Root/context content and any earlier sink failure.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.admitted.resource_inputs().charge_resources(sink)?;
        self.charge_normalized_resources(sink)
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    NormalizedDraftingProgram<'_, '_, H, S>
{
    /// Append original raw step/native records and every actual retained normalized arena to one
    /// cumulative sink, without reentering Root or provider accounting. The actual containing
    /// garment/Root/provider must compose separately. Both raw and normalized complete vectors
    /// are precharged;unused sources and repeated native inputs remain logical occurrences.
    /// No lookup,currentness,value,normalization,clone,result allocation or prior-demand credit
    /// occurs. The caller owns and finishes the meter. This supplies content,not work/admission.
    /// # Errors
    /// Original first raw/normalized field refusal or an earlier cumulative sink latch.
    /// ```
    /// use sc_core::{construction::*,design::*};
    /// fn normalized_records<H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(
    ///     program:&NormalizedDraftingProgram<'_, '_,H,S>,sink:&mut dyn DesignResourceSink)
    ///     ->Result<(),DesignResourceLimitError> {program.charge_record_resources(sink)}
    /// ```
    pub fn charge_record_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.admitted
            .resource_inputs()
            .charge_record_resources(sink)?;
        self.charge_normalized_resources(sink)
    }
    fn charge_normalized_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            admitted,
            sources,
            recipe,
            operations,
        } = self;
        fixed_value(admitted);
        let inputs = admitted.resource_inputs();
        let root = inputs.context().scope().root().id();
        sources
            .resource_normalized()
            .charge_retained_resources(sink, location(root, "drafting.normalized.sources", None))?;
        recipe.resource_normalized().charge_retained_resources(
            sink,
            location(root, "drafting.normalized.recipe.statements", None),
        )?;
        sink.charge_entries(
            operations.len(),
            location(root, "drafting.normalized.operations", None),
        )?;
        for (index, slot) in operations.iter().enumerate() {
            let Some(operation) = slot else {
                continue;
            };
            let owner = operation.input().operation();
            match operation {
                NormalizedDraftingOperation::Point(p) => expressions(
                    p.resource_expressions(),
                    [
                        (
                            "drafting.normalized.point.x.nodes",
                            "drafting.normalized.point.x.arguments",
                        ),
                        (
                            "drafting.normalized.point.y.nodes",
                            "drafting.normalized.point.y.arguments",
                        ),
                    ],
                    owner,
                    index,
                    sink,
                )?,
                NormalizedDraftingOperation::CenterArc(a) => expressions(
                    a.resource_expressions(),
                    [
                        (
                            "drafting.normalized.arc.radius.nodes",
                            "drafting.normalized.arc.radius.arguments",
                        ),
                        (
                            "drafting.normalized.arc.start_angle.nodes",
                            "drafting.normalized.arc.start_angle.arguments",
                        ),
                        (
                            "drafting.normalized.arc.sweep_angle.nodes",
                            "drafting.normalized.arc.sweep_angle.arguments",
                        ),
                    ],
                    owner,
                    index,
                    sink,
                )?,
                NormalizedDraftingOperation::SlashSpread { input, opening } => {
                    fixed_value(input);
                    expressions(
                        opening.resource_expressions(),
                        [(
                            "drafting.normalized.slash.opening.nodes",
                            "drafting.normalized.slash.opening.arguments",
                        )],
                        owner,
                        index,
                        sink,
                    )?;
                }
                NormalizedDraftingOperation::Other(input) => fixed_value(input),
            }
        }
        Ok(())
    }
}
