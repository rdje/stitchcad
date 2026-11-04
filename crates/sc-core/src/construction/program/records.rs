//! Original program step/native fields without reentering Root or provider source accounting.
use super::{DraftingProgramInputs, DraftingProgramStep};
use crate::{
    construction::DraftingContextSource,
    design::{
        fixed_value, DesignMetadataHost, DesignResourceLimitError, DesignResourceLocation,
        DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
};
pub(super) fn location(owner: EntityId) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(owner),
        field: "drafting.program.steps",
        index: None,
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> DraftingProgramInputs<'_, H, S> {
    /// Append only the actual original mixed step vector and complete native input fields to the
    /// caller's cumulative sink. Precharge the whole vector before traversal;retain original order,
    /// references,locations and first causes. Exhaustive owner/variant views and fixed context/ordinal
    /// guards expose new fields. No normalization,Root/provider/currentness/member/value callback,
    /// body clone,supplied totals or result allocation occurs. Original recipe ordinals are data,
    /// not checked for coverage/order here;native constructor bounds grant no content credit.
    ///
    /// This is record-only composition for a provider holding an original recipe. The integration
    /// must separately account its actual Root,context frame/snapshot,source payloads,copies/caches
    /// and counters through their real owners,without recursively reentering itself. This method
    /// alone grants no whole-program/source admission or query,allocator,CPU or execution allowance.
    /// Use `charge_resources` for the existing complete Root/provider/record program walk.
    /// ```
    /// use sc_core::{construction::{DraftingContextSource,DraftingProgramInputs},
    ///     design::{DesignMetadataHost,DesignResourceSink,DesignResourceLimitError}};
    /// fn records<H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(
    ///     program:&DraftingProgramInputs<'_,H,S>,sink:&mut dyn DesignResourceSink)
    ///     ->Result<(),DesignResourceLimitError> {program.charge_record_resources(sink)}
    /// ```
    /// # Errors
    /// Original first cumulative step/native-field cause,including earlier latched sink refusal
    /// for an empty vector. The caller owns/finishes its meter.
    pub fn charge_record_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self { context, steps } = self;
        fixed_value(context);
        sink.charge_entries(steps.len(), location(context.scope().root().id()))?;
        for step in steps {
            match step {
                DraftingProgramStep::Scalar { ordinal }
                | DraftingProgramStep::Semantic { ordinal } => fixed_value(ordinal),
                DraftingProgramStep::Drafting(input) => input.charge_resources(sink)?,
            }
        }
        Ok(())
    }
}
