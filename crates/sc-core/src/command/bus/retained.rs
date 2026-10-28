//! Complete actual private retained artifact state, under explicit immutable host policy.
use super::{operation::checkpoint, CommandBus, CommandControlError};
use crate::{
    command::{resource::fixed, CommandProgressObserver, CommandProgressPhase as Phase},
    design::{
        charge_design_garment_resources, DesignResourceLimitError, DesignResourceLocation,
        DesignResourceMeter, DesignResourceScope, DesignResourceSink, DesignResourceSource,
    },
    ontology::EntityId,
};

pub(super) fn location(owner: EntityId, field: &'static str) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(owner),
        field,
        index: None,
    }
}
impl<H: DesignResourceSource, A> CommandBus<H, A> {
    // Internal only: callers have already authenticated and bound the exact requested snapshot.
    // A is the trusted external authority service, excluded from retained artifact content. It is
    // never inspected or called here. H's actual owned source data belongs to each represented model.
    fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            state,
            authority: _,
            identities,
            journal,
            history,
            operation_limits,
            model_limits,
            retained_limits,
        } = self;
        fixed(operation_limits);
        fixed(model_limits);
        fixed(retained_limits);
        let owner = state.inputs().id();
        charge_design_garment_resources(state, sink)?;
        identities.charge_resources(owner, sink)?;
        if let Some(journal) = journal {
            journal.charge_resources(owner, sink)?;
        }
        if let Some(history) = history {
            history.charge_resources(owner, sink)?;
        }
        Ok(())
    }
    pub(super) fn admit_retained_current(
        &self,
        observer: &dyn CommandProgressObserver,
    ) -> Result<(), CommandControlError> {
        checkpoint(observer, Phase::RetainedValidation, 0, 1, None)?;
        let mut meter = DesignResourceMeter::new(self.require_retained_limits()?);
        self.charge_retained_resources(&mut meter)
            .map_err(CommandControlError::Retained)?;
        meter.finish().map_err(CommandControlError::Retained)?;
        checkpoint(observer, Phase::RetainedValidation, 1, 1, None)
    }
}
