//! Actual source/capture/stop caller join with original quantity ports and strict owner controls.
use super::{
    admitted, at, command_request, id, ownership_limits, DesignSewing, DesignSewingDefinition,
    EaseAmount, SeamSideId, SeamSpanDefinition, SewingGraph, SewingGraphDefinition,
};
use crate::notch_context_acceptance::{
    command_capture_limits, command_model_limits, command_weighted_state, limits, state,
};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    command::{
        CommandStopChild as Child, CommandStopSourceBinding, CommandStopSourceBindingError as Error,
    },
    ontology::{NotchEaseLimits, ProfileParameterRef, WeightedEaseLimits},
};
use sc_units::Length;

fn actual_children(
    state: &DesignSewing<sc_measure::DesignSources>,
    joined: &CommandStopSourceBinding<'_, sc_measure::DesignSources>,
) {
    let capture = joined.capture();
    let binding = capture.binding();
    let portions = capture
        .portion()
        .split_at(
            binding.request().cut(SeamSideId::A),
            binding.request().cut(SeamSideId::B),
        )
        .expect("actual captured original source portions");
    let spans = portions
        .into_iter()
        .zip([Child::First, Child::Second])
        .zip([id(99_001), id(99_002)])
        .map(|((portion, child), id)| {
            let stop_view = joined.child(child);
            assert_eq!(
                stop_view.pair().side(SeamSideId::A),
                portion.side(SeamSideId::A)
            );
            assert_eq!(
                stop_view.pair().side(SeamSideId::B),
                portion.side(SeamSideId::B)
            );
            SeamSpanDefinition {
                id,
                a: portion.side(SeamSideId::A),
                b: portion.side(SeamSideId::B),
                direction: portion.source().direction(),
                ease: portion.into(),
                stops: stop_view.rows().iter().map(|row| *row.stop()).collect(),
            }
        })
        .collect();
    let structure = state.inputs().structure();
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: state.graph().id(),
            spans,
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        state.landmarks(),
        structure.topology(),
    )
    .expect("actual child graph from sealed captured source and stops");
    assert_eq!(graph.ease_sources().count(), 1);
    assert!(core::ptr::eq(
        graph
            .ease_sources()
            .next()
            .expect("canonical original root"),
        capture.source()
    ));
    let candidate = DesignSewing::new(DesignSewingDefinition {
        inputs: state.inputs().clone(),
        graph,
        landmarks: state.landmarks().to_vec(),
    })
    .expect("real immutable owned candidate");
    assert_eq!(
        candidate
            .amount_declarations()
            .expect("actual child current source preflight")
            .len(),
        2
    );
    assert_eq!(candidate.landmarks(), state.landmarks());
}

#[test]
fn command_stop_source_binding_joins_actual_weighted_quantities_and_ordered_child_stops() {
    for side in [SeamSideId::A, SeamSideId::B] {
        for sign in [-1, 1] {
            let amount = Length::from_micrometres(sign).expect("actual signed literal");
            let state = command_weighted_state(EaseAmount::Explicit(amount), side);
            let saved = state.clone();
            let request = command_request(&state, 0, at(1, 2), at(2, 3));
            let (model, _) = command_model_limits(&state);
            let source = request
                .admit_sewing_source(&state, model, command_capture_limits(5))
                .expect("actual complete Design source admission");
            let capture = source.capture_admission().capture();
            let inputs = admitted(&state, &request, 1, 2, 2);
            let ownership = inputs
                .admit_ownership_workspace(ownership_limits(10))
                .expect("actual exact ownership cap");
            let context = ownership
                .validate_current_context()
                .expect("actual complete bounded current stop context");
            let memberships = context
                .derive_stop_memberships()
                .expect("actual complete authored membership");
            let partitions = memberships
                .partition_stops()
                .expect("actual ordered child stop views");
            let joined = partitions
                .bind_source_capture(&capture)
                .expect("same exact original request/target");
            assert!(core::ptr::eq(joined.capture(), &capture));
            assert!(core::ptr::eq(joined.partitions(), &partitions));
            assert!(core::ptr::eq(
                joined.capture().binding().span(),
                inputs.binding().span()
            ));
            let values = joined
                .capture()
                .weighted_explicit_ease(WeightedEaseLimits::new(
                    NonZeroUsize::new(2).expect("actual complete regions"),
                    NonZeroU64::new(598).expect("original weighted allowance"),
                ))
                .expect("existing original quantity port");
            assert_eq!(values.parent_amount(), amount);
            assert_eq!(
                values.first_amount(),
                if side == SeamSideId::A {
                    Length::ZERO
                } else {
                    amount
                }
            );
            assert_eq!(
                values.first_amount().checked_add(values.second_amount()),
                Ok(amount)
            );
            for child in [Child::First, Child::Second] {
                assert_eq!(joined.child(child).pair(), partitions.child(child).pair());
                for (actual, original) in joined
                    .child(child)
                    .rows()
                    .iter()
                    .zip(partitions.child(child).rows())
                {
                    assert!(core::ptr::eq(*actual, *original));
                }
            }
            actual_children(&state, &joined);
            assert_eq!(state, saved);
        }
    }
}

#[test]
fn command_stop_source_binding_joins_actual_retained_notch_quantities_without_root_stop_backfill() {
    for side in [SeamSideId::A, SeamSideId::B] {
        for sign in [-1, 1] {
            let amount = Length::from_micrometres(sign).expect("actual signed literal");
            let state = state(EaseAmount::Explicit(amount), side);
            let saved = state.clone();
            let request = command_request(&state, 0, at(3, 8), at(5, 8));
            let (model, _) = command_model_limits(&state);
            let source = request
                .admit_sewing_source(&state, model, command_capture_limits(3))
                .expect("actual complete Design source admission");
            let capture = source.capture_admission().capture();
            let inputs = admitted(&state, &request, 1, 2, 2);
            let ownership = inputs
                .admit_ownership_workspace(ownership_limits(10))
                .expect("actual exact ownership cap");
            let context = ownership
                .validate_current_context()
                .expect("actual N0 bounded context");
            let memberships = context
                .derive_stop_memberships()
                .expect("no authored target stops");
            let partitions = memberships
                .partition_stops()
                .expect("two empty actual lists");
            let joined = partitions
                .bind_source_capture(&capture)
                .expect("same exact original request/target");
            assert_eq!(joined.partitions().reference_count(), 0);
            assert_eq!(joined.capture().source().stops().len(), 2);
            assert!(core::ptr::eq(
                joined.capture().source(),
                state
                    .graph()
                    .ease_sources()
                    .next()
                    .expect("actual canonical source")
            ));
            let structure = state.inputs().structure();
            let original = joined
                .capture()
                .source()
                .admit_notch_context(
                    structure.cut_plan(),
                    structure.pieces().pieces(),
                    state.landmarks(),
                    structure.topology(),
                    limits(1, 2),
                )
                .expect("actual admitted full original dependencies")
                .validate_sources()
                .expect("full original source current context")
                .original_context()
                .expect("unique original Notches");
            let values = joined
                .capture()
                .notch_explicit_ease(
                    &original,
                    NotchEaseLimits::new(
                        NonZeroU64::new(473).expect("original notch quantity allowance"),
                    ),
                )
                .expect("existing exact original notch query port");
            assert_eq!(
                (
                    values.parent_amount(),
                    values.first_amount(),
                    values.second_amount()
                ),
                (amount, Length::ZERO, amount)
            );
            assert!(core::ptr::eq(values.original_context(), &original));
            for child in [Child::First, Child::Second] {
                assert!(joined.child(child).rows().is_empty());
            }
            actual_children(&state, &joined);
            assert_eq!(state, saved);
        }
    }
}

#[test]
fn command_stop_source_binding_preserves_actual_symbolic_metadata_without_value_providers() {
    for amount in [
        EaseAmount::Formula(id(5_001)),
        EaseAmount::Profile(ProfileParameterRef::new(id(98_010))),
    ] {
        let state = command_weighted_state(amount, SeamSideId::A);
        let saved = state.clone();
        let request = command_request(&state, 0, at(1, 2), at(2, 3));
        let (model, _) = command_model_limits(&state);
        let source = request
            .admit_sewing_source(&state, model, command_capture_limits(5))
            .expect("actual symbolic source metadata");
        let capture = source.capture_admission().capture();
        let inputs = admitted(&state, &request, 1, 2, 2);
        let ownership = inputs
            .admit_ownership_workspace(ownership_limits(10))
            .expect("exact actual cap");
        let context = ownership
            .validate_current_context()
            .expect("no numeric provider needed for current stops");
        let memberships = context.derive_stop_memberships().expect("actual stops");
        let partitions = memberships.partition_stops().expect("actual ordered views");
        let joined = partitions
            .bind_source_capture(&capture)
            .expect("actual source owners match");
        assert_eq!(
            joined
                .capture()
                .binding()
                .span()
                .definition()
                .ease
                .declaration()
                .amount,
            amount
        );
        assert!(core::ptr::eq(joined.capture(), &capture));
        actual_children(&state, &joined);
        assert_eq!(state, saved);
    }
}

#[test]
fn command_stop_source_binding_rejects_equal_request_and_same_id_equal_body_target_owners() {
    use std::error::Error as _;
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let saved = state.clone();
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let other_request = Box::new(request);
    let other_state = state.clone();
    let (model, _) = command_model_limits(&state);
    let inputs = admitted(&state, &request, 1, 2, 2);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(10))
        .expect("exact actual cap");
    let context = ownership
        .validate_current_context()
        .expect("original actual current context");
    let memberships = context
        .derive_stop_memberships()
        .expect("original actual stop rows");
    let partitions = memberships
        .partition_stops()
        .expect("original actual stop views");
    for (captured_request, captured_state, request_mismatch) in [
        (&*other_request, &state, true),
        (&request, &other_state, false),
        (&*other_request, &other_state, true),
    ] {
        let source = captured_request
            .admit_sewing_source(captured_state, model, command_capture_limits(5))
            .expect("other actual independently admitted source");
        let capture = source.capture_admission().capture();
        let error = partitions
            .bind_source_capture(&capture)
            .expect_err("IDs/equal bodies cannot substitute for exact actual owners");
        assert!(error.source().is_none());
        match error {
            Error::Request { expected, captured } => {
                assert!(request_mismatch);
                assert_eq!(expected, captured);
                assert!(core::ptr::eq(expected, &request));
                assert!(core::ptr::eq(captured, captured_request));
                assert!(!core::ptr::eq(expected, captured));
            }
            Error::Target { expected, captured } => {
                assert!(!request_mismatch);
                assert_eq!(expected.id(), captured.id());
                assert_eq!(expected.definition(), captured.definition());
                assert!(core::ptr::eq(expected, inputs.binding().span()));
                assert!(core::ptr::eq(captured, capture.binding().span()));
                assert!(!core::ptr::eq(expected, captured));
            }
        }
    }
    assert_eq!(state, saved);
    assert_eq!(other_state, saved);
}
