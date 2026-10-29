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

fn ownership_limits(items: usize) -> sc_core::command::CommandStopOwnershipLimits {
    sc_core::command::CommandStopOwnershipLimits::new(
        NonZeroUsize::new(items).expect("ownership workspace"),
    )
}
#[test]
fn command_stop_ownership_admission_borrows_exact_inputs_and_counts_complete_schema() {
    use sc_core::command::CommandStopOwnershipAdmissionError as Error;
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let saved = state.clone();
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let (model, _) = command_model_limits(&state);
    let inputs = request
        .admit_stop_partition_inputs(&state, model, command_stop_limits(16, 1, 2, 2))
        .expect("complete actual stop inputs");
    let admitted = inputs
        .admit_ownership_workspace(ownership_limits(10))
        .expect("C2+P2+(E2+1)*B2 =10");
    assert!(core::ptr::eq(admitted.inputs(), &inputs));
    assert!(core::ptr::eq(admitted.inputs().state(), &state));
    assert_eq!(
        (
            admitted.copy_count(),
            admitted.piece_count(),
            admitted.edge_count()
        ),
        (2, 2, 2)
    );
    assert_eq!(admitted.ownership_item_bound(), 4);
    assert_eq!(admitted.workspace_item_bound(), 10);
    assert_eq!(admitted.limits(), ownership_limits(10));
    assert_eq!(admitted.inputs().workspace_item_bound(), 16);
    assert_eq!(admitted.inputs().limits().range_portions().get(), 2);
    assert_eq!(
        inputs
            .admit_ownership_workspace(ownership_limits(9))
            .expect_err("one below ownership"),
        Error::Workspace {
            target_span: id(97_000),
            copies: 2,
            pieces: 2,
            edges: 2,
            range_portions: 2,
            required: 10,
            limit: 9
        }
    );
    assert_eq!(state, saved);
}
fn expanded_ownership_state() -> sc_core::design::DesignSewing<sc_measure::DesignSources> {
    use sc_core::ontology::{
        CopyOrientation, CutCopyDefinition, DeterministicIdGenerator, DirectedEdge, Direction,
        Piece, PieceInventory,
    };
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let extra = structure
        .topology
        .declare_edges(&mut DeterministicIdGenerator::from_start(110_000), 3)
        .expect("actual unused edge declarations");
    let mut pieces = structure.pieces.pieces().to_vec();
    let first = pieces.first_mut().expect("first actual Piece");
    let mut definition = first.definition().clone();
    definition
        .construction_lines
        .extend(extra.iter().skip(1).map(|edge| DirectedEdge {
            edge: *edge,
            direction: Direction::Original,
        }));
    *first =
        Piece::new(definition, &structure.topology).expect("actual expanded held edge inventory");
    let mut unused = first.definition().clone();
    unused.id = id(120_000);
    unused.boundary = vec![DirectedEdge {
        edge: *extra.first().expect("unused boundary"),
        direction: Direction::Original,
    }];
    unused.holes.clear();
    unused.construction_lines.clear();
    unused.fold_edges.clear();
    unused.cut_on_fold = false;
    let unused = Piece::new(unused, &structure.topology).expect("actual unused Piece");
    let mut copies = structure
        .cut_plan
        .copies()
        .iter()
        .map(|copy| *copy.definition())
        .collect::<Vec<_>>();
    copies.push(CutCopyDefinition {
        id: id(120_010),
        piece: unused.id(),
        orientation: CopyOrientation::Authored,
    });
    pieces.push(unused);
    structure.cut_plan =
        CutPlan::new(copies, &pieces).expect("complete plan includes unused Piece's required copy");
    structure.pieces = PieceInventory::new(pieces).expect("actual expanded Piece registry");
    inputs.structure =
        DesignStructure::new(structure).expect("actual born-consistent immutable root");
    DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual inputs"),
        graph: state.graph().clone(),
        landmarks: state.landmarks().to_vec(),
    })
    .expect("actual expanded ownership input owner")
}
#[test]
fn command_stop_ownership_admission_keeps_unused_copies_pieces_edges_and_checked_overflow() {
    use sc_core::command::CommandStopOwnershipAdmissionError as Error;
    let state = expanded_ownership_state();
    let saved = state.clone();
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let (model, _) = command_model_limits(&state);
    let inputs = request
        .admit_stop_partition_inputs(&state, model, command_stop_limits(16, 2, 5, 2))
        .expect("actual stop inputs count unchanged target/registry plus complete journal");
    let admitted = inputs
        .admit_ownership_workspace(ownership_limits(18))
        .expect("C3+P3+(E5+1)*B2 =18");
    assert_eq!(
        (
            admitted.copy_count(),
            admitted.piece_count(),
            admitted.edge_count()
        ),
        (3, 3, 5)
    );
    assert_eq!(admitted.ownership_item_bound(), 10);
    assert_eq!(admitted.workspace_item_bound(), 18);
    assert_eq!((inputs.stop_count(), inputs.landmark_count()), (2, 2));
    assert_eq!(
        inputs
            .admit_ownership_workspace(ownership_limits(17))
            .expect_err("unused data cannot be omitted"),
        Error::Workspace {
            target_span: id(97_000),
            copies: 3,
            pieces: 3,
            edges: 5,
            range_portions: 2,
            required: 18,
            limit: 17
        }
    );
    let peak = (usize::MAX - 8) / 4;
    let huge = request
        .admit_stop_partition_inputs(&state, model, command_stop_limits(usize::MAX, 2, 5, peak))
        .expect("earlier independent stop/pair reservation still fits");
    assert_eq!(huge.workspace_item_bound(), usize::MAX - 3);
    assert_eq!(
        huge.admit_ownership_workspace(ownership_limits(usize::MAX))
            .expect_err("actual complete ownership cache multiplication must fit separately"),
        Error::BoundOverflow {
            target_span: id(97_000),
            copies: 3,
            pieces: 3,
            edges: 5,
            range_portions: peak
        }
    );
    assert_eq!(state, saved);
}
#[test]
fn command_stop_ownership_admission_is_metadata_not_current_missing_copy_acceptance() {
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
    .expect("actual current copy replacement");
    inputs.structure = DesignStructure::new(structure).expect("actual immutable root");
    let held = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual inputs"),
        graph: state.graph().clone(),
        landmarks: state.landmarks().to_vec(),
    })
    .expect("actual held sewing state");
    let request = command_request(&held, 0, at(3, 8), at(5, 8));
    let (model, _) = command_model_limits(&held);
    let inputs = request
        .admit_stop_partition_inputs(&held, model, command_stop_limits(10, 1, 2, 2))
        .expect("zero target stops still admits actual raw model metadata");
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(10))
        .expect("ownership input budget only");
    assert_eq!(ownership.inputs().stop_count(), 0);
    assert_eq!(ownership.workspace_item_bound(), 10);
    assert_eq!(
        ownership.inputs().binding().span().definition().a.copy,
        id(96_000)
    );
    assert!(
        matches!(held.validate_current(),Err(DesignSewingError::Graph(issue))
        if matches!(*issue,SewingError::MissingCopy {span,side:SeamSideId::A,copy}
        if span==id(97_001) && copy==id(96_000)))
    );
}

#[path = "stop_partition_context.rs"]
mod stop_partition_context;
