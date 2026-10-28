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

// Borrowed actual bounded preparation only; no supplied totals, owned counter, model or grant token.
pub(super) struct AuthoringProjection<'a, H> {
    pub(super) candidate: &'a crate::design::DesignGarment<H>,
    pub(super) changes: &'a super::CommandGroupPreview,
    pub(super) request: &'a crate::command::CommandMutationGroup,
}
pub(super) fn charge_successor_revision(
    revision: &crate::command::DesignRevision,
    sink: &mut dyn DesignResourceSink,
    location: DesignResourceLocation,
) -> Result<(), DesignResourceLimitError> {
    crate::design::charge_revision_bits(revision.successor_resource_bits(), sink, location)
}
// Normal vectors precharge their whole prospective length. The representational boundary uses exact
// chunks at the same location, never wrapping/clipping; no finite platform-sized policy admits both.
pub(super) fn charge_appended_entries(
    original: usize,
    sink: &mut dyn DesignResourceSink,
    location: DesignResourceLocation,
) -> Result<(), DesignResourceLimitError> {
    if let Some(entries) = original.checked_add(1) {
        sink.charge_entries(entries, location)
    } else {
        sink.charge_entries(original, location)?;
        sink.charge_entries(1, location)
    }
}
impl<H: DesignResourceSource, A> CommandBus<H, A> {
    fn charge_retained_after_authoring(
        &self,
        projection: &AuthoringProjection<'_, H>,
        journal: &super::execution::CommandJournal,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            state,
            authority: _,
            identities,
            journal: _,
            history,
            operation_limits,
            model_limits,
            retained_limits,
        } = self;
        fixed(operation_limits);
        fixed(model_limits);
        fixed(retained_limits);
        let AuthoringProjection {
            candidate,
            changes,
            request,
        } = projection;
        let owner = state.inputs().id();
        crate::design::charge_garment_successor_revision(
            candidate,
            journal.resource_maximum_revision(),
            sink,
        )?;
        identities.charge_resources(owner, sink)?;
        journal.charge_after_authoring_resources(
            owner,
            request,
            changes,
            history.as_ref(),
            sink,
        )?;
        if let Some(history) = history {
            history.charge_after_authoring_resources(owner, state, sink)?;
        }
        Ok(())
    }
    pub(super) fn admit_retained_authoring(
        &self,
        projection: AuthoringProjection<'_, H>,
        observer: &dyn CommandProgressObserver,
    ) -> Result<(), super::CommandGroupCommitError<H::Error>> {
        let group = projection.request;
        super::operation::group_checkpoint::<H::Error>(
            group,
            observer,
            Phase::RetainedPublication,
            0,
            1,
            None,
        )?;
        let journal = self
            .journal
            .as_ref()
            .ok_or(super::CommandGroupCommitError::EditsNotConfigured { group: group.id() })?;
        let limits = self.require_retained_limits().map_err(|issue| {
            super::CommandGroupPreviewError::<H::Error>::control(group.id(), issue)
        })?;
        let mut meter = DesignResourceMeter::new(limits);
        self.charge_retained_after_authoring(&projection, journal, &mut meter)
            .map_err(|issue| {
                super::CommandGroupPreviewError::<H::Error>::control(
                    group.id(),
                    CommandControlError::Retained(issue),
                )
            })?;
        meter.finish().map_err(|issue| {
            super::CommandGroupPreviewError::<H::Error>::control(
                group.id(),
                CommandControlError::Retained(issue),
            )
        })?;
        super::operation::group_checkpoint::<H::Error>(
            group,
            observer,
            Phase::RetainedPublication,
            1,
            1,
            None,
        )?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use core::num::{NonZeroU64, NonZeroUsize};
    #[test]
    fn appended_collection_count_boundary_preserves_exact_original_first_error_without_wrap() {
        let original = location(EntityId::from_bits(1), "retained.journal.entries");
        let limits = crate::design::DesignResourceLimits::new(
            NonZeroUsize::MAX,
            NonZeroUsize::MAX,
            NonZeroU64::MAX,
        );
        let mut meter = DesignResourceMeter::new(limits);
        let expected = DesignResourceLimitError::Count {
            unit: crate::design::DesignResourceCount::CollectionEntries,
            maximum: usize::MAX,
            consumed: usize::MAX,
            requested: 1,
            location: original,
        };
        assert_eq!(
            charge_appended_entries(usize::MAX, &mut meter, original),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        let mut meter = DesignResourceMeter::new(limits);
        assert_eq!(charge_appended_entries(2, &mut meter, original), Ok(()));
        assert_eq!(meter.finish().map(|d| d.collection_entries()), Ok(3));
    }
}
