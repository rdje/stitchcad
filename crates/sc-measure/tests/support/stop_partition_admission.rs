//! Actual owned Design input admission;bounded current stop membership follows separately.
use super::{
    at, command_model_limits, command_request, command_weighted_state, id, state, CutPlan,
    DesignInputs, DesignSewing, DesignSewingDefinition, DesignSewingError, DesignStructure,
    EaseAmount, EdgeAnchor, ProfileParameterRef, SeamSideId, SewingError, SewingLandmark,
    UniquePointLimits,
};
use core::num::NonZeroUsize;

fn command_stop_limits(
    items: usize,
    journal: usize,
    nested: usize,
    portions: usize,
) -> sc_core::command::CommandStopPartitionLimits {
    sc_core::command::CommandStopPartitionLimits::new(
        NonZeroUsize::new(items).expect("workspace"),
        UniquePointLimits::new(
            NonZeroUsize::new(journal).expect("complete journal"),
            NonZeroUsize::new(nested).expect("complete nested input"),
        ),
        NonZeroUsize::new(portions).expect("per-range peak"),
    )
}
#[test]
fn command_stop_admission_borrows_actual_inputs_and_exact_independent_workspace_reservation() {
    use sc_core::command::CommandStopPartitionAdmissionError as Error;
    for amount in [
        EaseAmount::Explicit(sc_units::Length::ZERO),
        EaseAmount::Formula(id(5_001)),
        EaseAmount::Profile(ProfileParameterRef::new(id(98_010))),
    ] {
        let state = command_weighted_state(amount, SeamSideId::A);
        let saved = state.clone();
        let request = command_request(&state, 0, at(1, 2), at(2, 3));
        let (model, demand) = command_model_limits(&state);
        let limits = command_stop_limits(16, 1, 2, 2);
        let admission = request
            .admit_stop_partition_inputs(&state, model, limits)
            .expect("exact M2+N2+2N4+4B8 workspace");
        assert!(core::ptr::eq(admission.state(), &state));
        assert!(core::ptr::eq(admission.binding().request(), &request));
        assert!(core::ptr::eq(
            admission.binding().span(),
            state.graph().spans().first().expect("actual target")
        ));
        assert_eq!(admission.model_demand(), demand);
        assert_eq!(admission.model_limits(), model);
        assert_eq!(admission.limits(), limits);
        assert_eq!((admission.stop_count(), admission.landmark_count()), (2, 2));
        assert_eq!(admission.range_cache_item_bound(), 8);
        assert_eq!(admission.membership_item_bound(), 2);
        assert_eq!(admission.child_reference_item_bound(), 4);
        assert_eq!(admission.workspace_item_bound(), 16);
        assert_eq!(limits.range_limits().current(), limits.current());
        assert_eq!(limits.range_limits().portions(), limits.range_portions());
        assert_eq!(
            request
                .admit_stop_partition_inputs(&state, model, command_stop_limits(15, 1, 2, 2))
                .expect_err("one-below complete workspace"),
            Error::Workspace {
                target_span: id(97_000),
                stops: 2,
                landmarks: 2,
                range_portions: 2,
                required: 16,
                limit: 15
            }
        );
        assert_eq!(
            admission
                .binding()
                .span()
                .definition()
                .ease
                .declaration()
                .amount,
            amount
        );
        assert_eq!(state, saved);
    }
}
#[test]
fn command_stop_admission_counts_unused_registry_and_target_stops_separately_from_original_root() {
    use sc_core::command::CommandStopPartitionAdmissionError as Error;
    use sc_core::ontology::TurnPoint;
    let state = state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let request = command_request(&state, 0, at(3, 8), at(5, 8));
    let (model, _) = command_model_limits(&state);
    let admission = request
        .admit_stop_partition_inputs(&state, model, command_stop_limits(10, 1, 2, 2))
        .expect("zero active stops still counts full registry and four range caches");
    assert_eq!(
        (
            admission.stop_count(),
            admission.landmark_count(),
            admission.workspace_item_bound()
        ),
        (0, 2, 10)
    );
    assert_eq!(admission.child_reference_item_bound(), 0);
    assert_eq!(
        admission
            .binding()
            .span()
            .definition()
            .ease
            .portion()
            .expect("retained source")
            .source()
            .stops()
            .len(),
        2
    );
    let structure = state.inputs().structure();
    let unused = TurnPoint::new(
        id(98_002),
        EdgeAnchor {
            edge: state
                .graph()
                .ease_sources()
                .next()
                .expect("root")
                .side(SeamSideId::A)
                .range
                .edge(),
            param: at(1, 8),
        },
        structure.pieces().pieces().first().expect("actual Piece"),
        structure.topology(),
    )
    .expect("actual unused semantic Turn");
    let mut landmarks = state.landmarks().to_vec();
    landmarks.push(SewingLandmark::Turn(unused));
    let expanded = DesignSewing::new(DesignSewingDefinition {
        inputs: state.inputs().clone(),
        graph: state.graph().clone(),
        landmarks,
    })
    .expect("actual complete expanded registry");
    let (model, _) = command_model_limits(&expanded);
    assert_eq!(
        request
            .admit_stop_partition_inputs(&expanded, model, command_stop_limits(10, 1, 2, 2))
            .expect_err("unused registry entry cannot be hidden"),
        Error::Workspace {
            target_span: id(97_001),
            stops: 0,
            landmarks: 3,
            range_portions: 2,
            required: 11,
            limit: 10
        }
    );
    assert_eq!(
        request
            .admit_stop_partition_inputs(&expanded, model, command_stop_limits(11, 1, 2, 2))
            .expect("complete expanded metadata allowance")
            .workspace_item_bound(),
        11
    );
}
#[test]
fn command_stop_admission_preserves_model_binding_journal_and_checked_workspace_refusals() {
    use sc_core::command::{
        CommandSeamSplitBindingError, CommandStopPartitionAdmissionError as Error, SplitSeamSpan,
    };
    use sc_core::design::{DesignResourceLimitError, DesignResourceLimits};
    use sc_core::ontology::{
        DeterministicIdGenerator, PointJournalAdmissionIssue, PointJournalCollection,
    };
    use std::error::Error as _;
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let missing = SplitSeamSpan::new(
        id(99_999),
        request.cut(SeamSideId::A),
        request.cut(SeamSideId::B),
    );
    let (model, demand) = command_model_limits(&state);
    let less = DesignResourceLimits::new(
        NonZeroUsize::new(demand.collection_entries() - 1).expect("one below"),
        model.text_bytes(),
        model.counter_bits(),
    );
    let impossible = command_stop_limits(1, 1, 1, usize::MAX);
    let error = missing
        .admit_stop_partition_inputs(&state, less, impossible)
        .expect_err("model first");
    assert!(matches!(error, Error::Model(_)));
    assert!(error
        .source()
        .expect("original model cause")
        .downcast_ref::<DesignResourceLimitError>()
        .is_some());
    assert_eq!(
        missing
            .admit_stop_partition_inputs(&state, model, impossible)
            .expect_err("target before journal"),
        Error::Binding(CommandSeamSplitBindingError::MissingSpan {
            span: missing.span()
        })
    );
    let error = request
        .admit_stop_partition_inputs(&state, model, impossible)
        .expect_err("complete nested before overflow");
    assert_eq!(
        error,
        Error::Journal {
            target_span: id(97_000),
            issue: PointJournalAdmissionIssue::NestedItems {
                operation: state
                    .inputs()
                    .structure()
                    .topology()
                    .journal()
                    .first()
                    .expect("actual creation")
                    .operation(),
                collection: PointJournalCollection::DeclaredEdges,
                consumed: 0,
                requested: 2,
                limit: 1
            }
        }
    );
    assert!(error.source().is_none()); // Original fixed journal evidence is not a standard Error.
    let overflow = command_stop_limits(usize::MAX, 1, 2, usize::MAX);
    assert_eq!(
        request
            .admit_stop_partition_inputs(&state, model, overflow)
            .expect_err("cache multiplication before allocation"),
        Error::BoundOverflow {
            target_span: id(97_000),
            stops: 2,
            landmarks: 2,
            range_portions: usize::MAX
        }
    );
    assert_eq!(
        request
            .admit_stop_partition_inputs(
                &state,
                model,
                command_stop_limits(usize::MAX, 1, 2, usize::MAX / 4)
            )
            .expect_err("aggregate checked addition"),
        Error::BoundOverflow {
            target_span: id(97_000),
            stops: 2,
            landmarks: 2,
            range_portions: usize::MAX / 4
        }
    );
    let huge = request
        .admit_stop_partition_inputs(
            &state,
            model,
            command_stop_limits(usize::MAX, 1, 2, (usize::MAX - 8) / 4),
        )
        .expect("representable scalar bound allocates no cache");
    assert_eq!(huge.workspace_item_bound(), usize::MAX - 3);

    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure
        .topology
        .declare_edges(&mut DeterministicIdGenerator::from_start(110_000), 1)
        .expect("unrelated actual journal data");
    inputs.structure = DesignStructure::new(structure).expect("current structural source");
    let expanded = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual inputs"),
        graph: state.graph().clone(),
        landmarks: state.landmarks().to_vec(),
    })
    .expect("actual expanded journal owner");
    let (model, _) = command_model_limits(&expanded);
    assert_eq!(
        request
            .admit_stop_partition_inputs(&expanded, model, impossible)
            .expect_err("whole journal first"),
        Error::Journal {
            target_span: id(97_000),
            issue: PointJournalAdmissionIssue::JournalEntries {
                required: 2,
                limit: 1
            }
        }
    );
    assert_eq!(
        request
            .admit_stop_partition_inputs(&expanded, model, command_stop_limits(16, 2, 2, 2))
            .expect_err("all unrelated nested input counted"),
        Error::Journal {
            target_span: id(97_000),
            issue: PointJournalAdmissionIssue::NestedItems {
                operation: id(110_000),
                collection: PointJournalCollection::DeclaredEdges,
                consumed: 2,
                requested: 1,
                limit: 2
            }
        }
    );
    assert_eq!(
        request
            .admit_stop_partition_inputs(&expanded, model, command_stop_limits(16, 2, 3, 2))
            .expect("complete journal and workspace admitted")
            .workspace_item_bound(),
        16
    );
}
#[test]
fn command_stop_admission_keeps_held_current_copy_refusals_separate_from_input_metadata() {
    let state = state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.cut_plan = CutPlan::new(
        structure
            .cut_plan
            .copies()
            .iter()
            .map(|copy| {
                let mut definition = *copy.definition();
                if definition.id == id(96_000) {
                    definition.id = id(96_010);
                }
                definition
            })
            .collect(),
        structure.pieces.pieces(),
    )
    .expect("explicit current copy replacement");
    inputs.structure = DesignStructure::new(structure).expect("actual structure");
    let held = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("current inputs"),
        graph: state.graph().clone(),
        landmarks: state.landmarks().to_vec(),
    })
    .expect("actual inspectable held graph");
    let saved = held.clone();
    let request = command_request(&held, 0, at(3, 8), at(5, 8));
    let (model, _) = command_model_limits(&held);
    let admission = request
        .admit_stop_partition_inputs(&held, model, command_stop_limits(10, 1, 2, 2))
        .expect("bounded input metadata supplies no fresh current membership");
    assert_eq!(admission.binding().span().definition().a.copy, id(96_000));
    assert!(
        matches!(held.validate_current(),Err(DesignSewingError::Graph(issue))
        if matches!(*issue,SewingError::MissingCopy {span,side:SeamSideId::A,copy}
        if span==id(97_001) && copy==id(96_000)))
    );
    assert_eq!(held, saved);
}
