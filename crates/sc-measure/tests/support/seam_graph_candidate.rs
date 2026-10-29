//! Actual complete source/stop/history graph candidates; no bus or synthetic value providers.
#[path = "seam_garment_candidate.rs"]
mod seam_garment_candidate;
use super::{
    admitted, at, command_capture_limits, command_model_limits, command_request,
    command_weighted_state, id, ownership_limits, DesignSewing, DesignSewingDefinition, EaseAmount,
    SeamSideId, SewingGraph, SewingGraphDefinition,
};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    command::{
        CommandSeamSplitGraphError as Error, CommandSeamSplitGraphLimits as Limits,
        CommandSeamSplitSourceAdmission, CommandStopChild as Child, CommandStopSourceBinding,
        SplitSeamSpan,
    },
    design::{
        charge_sewing_graph_resources, DesignResourceLimitError, DesignResourceLimits,
        DesignResourceMeter,
    },
    ontology::{
        Param, ProfileParameterRef, Rational, SeamDirection, SeamSpanHistory,
        SeamSpanHistoryEntry as Entry, SeamSpanHistoryError as HistoryError, SeamSpanHistoryLimits,
        SeamSpanHistoryState, SewingError, SewingSpanLookup,
    },
};
use sc_measure::DesignSources;
use sc_units::Length;

fn limits(resources: usize, records: usize, identities: usize) -> Limits {
    Limits::new(
        DesignResourceLimits::new(
            NonZeroUsize::new(resources).expect("explicit graph payload allowance"),
            NonZeroUsize::new(1).expect("no owned graph text"),
            NonZeroU64::new(1).expect("no owned graph counters"),
        ),
        SeamSpanHistoryLimits::new(
            NonZeroUsize::new(records).expect("complete next history"),
            NonZeroUsize::new(identities).expect("all issued identities"),
        ),
    )
}
fn owned(state: &DesignSewing<DesignSources>, graph: SewingGraph) -> DesignSewing<DesignSources> {
    DesignSewing::new(DesignSewingDefinition {
        inputs: state.inputs().clone(),
        graph,
        landmarks: state.landmarks().to_vec(),
    })
    .expect("actual canonical ownership, not just a graph")
}
fn state(
    amount: EaseAmount,
    side: SeamSideId,
    direction: SeamDirection,
) -> DesignSewing<DesignSources> {
    let original = command_weighted_state(amount, side);
    let mut target = original
        .graph()
        .spans()
        .first()
        .expect("actual target")
        .definition()
        .clone();
    target.direction = direction;
    let mut before = target.clone();
    before.id = id(100_010);
    let mut after = target.clone();
    after.id = id(100_011);
    let entries = vec![
        Entry::Root(before.id),
        Entry::Root(target.id),
        Entry::Root(after.id),
    ];
    let structure = original.inputs().structure();
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: original.graph().id(),
            spans: vec![before, target, after],
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        original.landmarks(),
        structure.topology(),
    )
    .expect("actual target with unrelated authored siblings")
    .with_span_history(
        SeamSpanHistory::new(entries, limits(100, 3, 3).history())
            .expect("actual independent roots, no invented past splits"),
    )
    .expect("exact actual graph/history leaves");
    owned(&original, graph)
}
fn with_join<R>(
    state: &DesignSewing<DesignSources>,
    request: &SplitSeamSpan,
    inspect: impl for<'a> FnOnce(
        &'a CommandSeamSplitSourceAdmission<'a, DesignSources>,
        &'a CommandStopSourceBinding<'a, DesignSources>,
    ) -> R,
) -> R {
    let (model, _) = command_model_limits(state);
    let source = request
        .admit_sewing_source(state, model, command_capture_limits(5))
        .expect("actual complete current source");
    let capture = source.capture_admission().capture();
    let inputs = admitted(state, request, 1, 2, 2);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(10))
        .expect("actual owners");
    let context = ownership
        .validate_current_context()
        .expect("actual current stop context");
    let memberships = context
        .derive_stop_memberships()
        .expect("actual closed memberships");
    let partitions = memberships
        .partition_stops()
        .expect("actual authored stop order");
    let joined = partitions
        .bind_source_capture(&capture)
        .expect("exact actual owners");
    inspect(&source, &joined)
}

#[test]
fn seam_graph_candidate_replaces_only_actual_target_and_preserves_full_sources_and_pair_order() {
    let children = [id(100_099), id(100_050)];
    for direction in [SeamDirection::Same, SeamDirection::Opposite] {
        for side in [SeamSideId::A, SeamSideId::B] {
            for amount in [
                EaseAmount::Explicit(Length::from_micrometres(-1).expect("signed amount")),
                EaseAmount::Formula(id(5_001)),
                EaseAmount::Profile(ProfileParameterRef::new(id(98_010))),
            ] {
                let state = state(amount, side, direction);
                let saved = state.clone();
                let request = command_request(&state, 1, at(1, 2), at(2, 3));
                with_join(&state, &request, |source, joined| {
                    let cap = limits(100, 4, 5);
                    let admitted = joined
                        .admit_graph_candidate(source, children, cap)
                        .expect("actual join");
                    assert!(core::ptr::eq(admitted.source(), source));
                    assert!(core::ptr::eq(admitted.stop_source(), joined));
                    assert_eq!(admitted.children(), children);
                    assert_eq!(admitted.limits(), cap);
                    assert_eq!(admitted.reservation().collection_entries(), 28);
                    let graph = admitted.build().expect("current structural candidate");
                    assert_eq!(graph.id(), state.graph().id());
                    assert_eq!(
                        graph
                            .spans()
                            .iter()
                            .map(|span| span.id())
                            .collect::<Vec<_>>(),
                        [id(100_010), children[0], children[1], id(100_011)]
                    );
                    assert_eq!(graph.spans().first(), state.graph().spans().first());
                    assert_eq!(graph.spans().last(), state.graph().spans().last());
                    assert!(core::ptr::eq(
                        graph.ease_sources().next().expect("one full original root"),
                        joined.capture().source()
                    ));
                    for ((span, child), pair) in graph
                        .spans()
                        .iter()
                        .skip(1)
                        .take(2)
                        .zip([Child::First, Child::Second])
                        .zip(source.binding().paired_intervals())
                    {
                        let def = span.definition();
                        assert_eq!(def.a, pair.side(SeamSideId::A));
                        assert_eq!(def.b, pair.side(SeamSideId::B));
                        assert_eq!(def.direction, direction);
                        assert_eq!(
                            def.ease.declaration(),
                            joined.capture().source().declaration()
                        );
                        assert_eq!(
                            def.stops,
                            joined
                                .child(child)
                                .rows()
                                .iter()
                                .map(|row| *row.stop())
                                .collect::<Vec<_>>()
                        );
                    }
                    let history = graph.span_history().expect("actual extended history");
                    assert_eq!(
                        history
                            .entries()
                            .iter()
                            .take(3)
                            .copied()
                            .collect::<Vec<_>>(),
                        state
                            .graph()
                            .span_history()
                            .expect("actual original history")
                            .entries()
                    );
                    assert_eq!(
                        history.entries().last(),
                        Some(&Entry::Split {
                            parent: request.span(),
                            children
                        })
                    );
                    assert!(
                        matches!(graph.lookup_span(request.span()), Some(SewingSpanLookup::RetiredSplit { children: ids, .. }) if *ids == children)
                    );
                    let mut meter = DesignResourceMeter::new(cap.resources());
                    charge_sewing_graph_resources(&graph, &mut meter)
                        .expect("actual final graph walk");
                    assert_eq!(
                        meter
                            .finish()
                            .expect("complete output demand")
                            .collection_entries(),
                        23
                    );
                    let candidate = owned(&state, graph);
                    assert_eq!(
                        candidate
                            .amount_declarations()
                            .expect("actual current full owner")
                            .len(),
                        4
                    );
                });
                assert_eq!(state, saved);
            }
        }
    }
}

#[test]
fn seam_graph_candidate_preserves_nested_history_original_root_and_shared_closed_cut_stops() {
    let original = state(
        EaseAmount::Explicit(Length::ZERO),
        SeamSideId::A,
        SeamDirection::Opposite,
    );
    let first = command_request(&original, 1, at(1, 2), at(2, 3));
    let graph = with_join(&original, &first, |source, joined| {
        joined
            .admit_graph_candidate(source, [id(100_099), id(100_050)], limits(100, 4, 5))
            .expect("first actual history transition")
            .build()
            .expect("first candidate")
    });
    let intermediate = owned(&original, graph);
    let saved = intermediate.clone();
    let span = intermediate
        .graph()
        .spans()
        .get(1)
        .expect("actual first child");
    let middle = |side| {
        let range = match side {
            SeamSideId::A => span.definition().a.range,
            SeamSideId::B => span.definition().b.range,
        };
        Param::new(
            range
                .from()
                .as_rational()
                .checked_add(range.to().as_rational())
                .expect("small exact intervals")
                .checked_div(Rational::from_integer(2))
                .expect("exact midpoint"),
        )
        .expect("interior")
    };
    let second = command_request(
        &intermediate,
        1,
        middle(SeamSideId::A),
        middle(SeamSideId::B),
    );
    let grandchildren = [id(200_003), id(200_002)];
    with_join(&intermediate, &second, |source, joined| {
        assert_eq!(
            joined.partitions().reference_count(),
            2,
            "one cut stop shared by both children"
        );
        let graph = joined
            .admit_graph_candidate(source, grandchildren, limits(100, 5, 7))
            .expect("extend actual existing history")
            .build()
            .expect("nested candidate");
        let history = graph
            .span_history()
            .expect("whole original prefix plus one transition");
        assert_eq!(history.identity_count(), 7);
        assert_eq!(
            history.state(first.span()),
            intermediate
                .graph()
                .span_history()
                .expect("old history")
                .state(first.span())
        );
        assert_eq!(
            history.state(second.span()),
            Some(SeamSpanHistoryState::RetiredSplit {
                birth_entry: 3,
                split_entry: 4,
                children: &grandchildren,
            })
        );
        assert!(core::ptr::eq(
            graph
                .ease_sources()
                .next()
                .expect("canonical original root"),
            intermediate
                .graph()
                .ease_sources()
                .next()
                .expect("same original root")
        ));
        for span in graph.spans().iter().skip(1).take(2) {
            assert_eq!(
                span.definition().stops,
                source.binding().span().definition().stops
            );
            assert_eq!(
                span.definition()
                    .ease
                    .portion()
                    .expect("real retained context")
                    .source()
                    .span(),
                first.span()
            );
        }
        assert_eq!(
            owned(&intermediate, graph)
                .amount_declarations()
                .expect("actual current Design")
                .len(),
            5
        );
    });
    assert_eq!(intermediate, saved);
}

#[test]
fn seam_graph_candidate_refuses_owner_substitution_and_legacy_history_before_copy_admission() {
    let state = state(
        EaseAmount::Formula(id(5_001)),
        SeamSideId::A,
        SeamDirection::Same,
    );
    let saved = state.clone();
    let request = command_request(&state, 1, at(1, 2), at(2, 3));
    let other_request = Box::new(request);
    let other_state = state.clone();
    let (model, _) = command_model_limits(&state);
    let children = [id(100_099), id(100_050)];
    with_join(&state, &request, |_source, joined| {
        let other_source = request
            .admit_sewing_source(&other_state, model, command_capture_limits(5))
            .expect("equal but different owner");
        assert!(matches!(
            joined.admit_graph_candidate(&other_source, children, limits(1, 1, 1)),
            Err(Error::SourceOwner { .. })
        ));
        let other_source = other_request
            .admit_sewing_source(&state, model, command_capture_limits(5))
            .expect("equal but different request");
        assert!(matches!(
            joined.admit_graph_candidate(&other_source, children, limits(1, 1, 1)),
            Err(Error::RequestOwner { .. })
        ));
    });
    assert_eq!(state, saved);
    let legacy = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let request = command_request(&legacy, 0, at(1, 2), at(2, 3));
    with_join(&legacy, &request, |source, joined| {
        assert!(
            matches!(joined.admit_graph_candidate(source, children, limits(1, 1, 1)), Err(Error::MissingHistory { graph }) if graph==legacy.graph().id())
        );
    });
}

#[test]
fn seam_graph_candidate_preserves_exact_resource_history_and_current_namespace_refusals() {
    use std::error::Error as _;
    let state = state(
        EaseAmount::Formula(id(5_001)),
        SeamSideId::A,
        SeamDirection::Same,
    );
    let saved = state.clone();
    let request = command_request(&state, 1, at(1, 2), at(2, 3));
    let children = [id(100_099), id(100_050)];
    with_join(&state, &request, |source, joined| {
        let issue = joined
            .admit_graph_candidate(source, [request.span(); 2], limits(27, 1, 1))
            .expect_err("resource reservation before history semantics");
        assert!(
            matches!(issue, Error::Resources(DesignResourceLimitError::Count { maximum:27, consumed:25, requested:3, location, .. })
            if location.field == "candidate.span_split.children_and_history")
        );
        assert!(issue
            .source()
            .expect("original resource cause")
            .downcast_ref::<DesignResourceLimitError>()
            .is_some());
        assert_eq!(
            joined
                .admit_graph_candidate(source, [request.span(); 2], limits(28, 3, 3))
                .expect_err("whole next entry quota"),
            Error::History(HistoryError::Entries {
                required: 4,
                limit: 3
            })
        );
        assert_eq!(
            joined
                .admit_graph_candidate(source, [request.span(); 2], limits(28, 4, 4))
                .expect_err("whole issued quota"),
            Error::History(HistoryError::Identities {
                required: 5,
                limit: 4
            })
        );
        assert_eq!(
            joined
                .admit_graph_candidate(source, [request.span(); 2], limits(28, 4, 5))
                .expect_err("no target ID reuse"),
            Error::History(HistoryError::ReusedIdentity {
                entry: 3,
                identity: request.span(),
                first_birth: 1
            })
        );
        let graph_id = state.graph().id();
        let issue = joined
            .admit_graph_candidate(source, [graph_id, children[1]], limits(28, 4, 5))
            .expect("history-only identity rules do not confer global freshness")
            .build()
            .expect_err("actual graph namespace collision");
        assert!(
            matches!(&issue, Error::Graph(error) if matches!(error.as_ref(), SewingError::IdentityCollision { id } if *id==graph_id))
        );
        assert!(issue
            .source()
            .expect("original graph cause")
            .downcast_ref::<SewingError>()
            .is_some());
        joined
            .admit_graph_candidate(source, children, limits(28, 4, 5))
            .expect("exact resource boundary")
            .build()
            .expect("valid current retry");
    });
    assert_eq!(state, saved);
}

#[test]
fn seam_graph_candidate_actual_held_gather_and_request_observe_retirement_without_retargeting() {
    use sc_core::{
        command::CommandSeamSplitBindingError,
        ontology::{DirectedRange, Direction, Gather, GatherDefinition, GatherError},
    };
    let state = state(
        EaseAmount::Formula(id(5_001)),
        SeamSideId::A,
        SeamDirection::Opposite,
    );
    let request = command_request(&state, 1, at(1, 2), at(2, 3));
    let source = state
        .graph()
        .spans()
        .get(1)
        .expect("actual original target");
    let held = source.definition().a;
    let structure = state.inputs().structure();
    let piece = structure
        .cut_plan()
        .copy(held.copy)
        .expect("actual held copy")
        .piece();
    let piece = structure
        .pieces()
        .pieces()
        .iter()
        .find(|candidate| candidate.id() == piece)
        .expect("actual owner");
    let definition = GatherDefinition {
        id: id(300_001),
        graph: state.graph().id(),
        span: source.id(),
        side: SeamSideId::A,
        copy: held.copy,
        direction: DirectedRange {
            range: held.range,
            direction: Direction::Original,
        },
        closing_operation: id(300_002),
    };
    let gather = Gather::new(
        definition.clone(),
        piece,
        structure.cut_plan(),
        state.graph(),
        structure.topology(),
    )
    .expect("actual held Gather before splitting");
    let children = [id(100_099), id(100_050)];
    let graph = with_join(&state, &request, |source, joined| {
        joined
            .admit_graph_candidate(source, children, limits(28, 4, 5))
            .expect("real candidate admission")
            .build()
            .expect("real paired graph")
    });
    let SewingSpanLookup::RetiredSplit {
        children: actual,
        birth_entry,
        split_entry,
    } = gather
        .inspect_span_reference(&graph)
        .expect("explicit old-target retirement")
    else {
        unreachable!("actual target must remain retired")
    };
    assert_eq!((birth_entry, split_entry), (1, 3));
    assert_eq!(*actual, children);
    assert_eq!(gather.definition(), &definition);
    assert_eq!(
        gather.current_intake_source(piece, structure.cut_plan(), &graph, structure.topology()),
        Err(GatherError::MissingSpan(request.span()))
    );
    assert_eq!(
        request
            .bind_source_cuts(&graph)
            .expect_err("old request never selects a child"),
        CommandSeamSplitBindingError::MissingSpan {
            span: request.span()
        }
    );
    assert!(
        matches!(gather.inspect_span_reference(state.graph()).expect("original unchanged live member"),SewingSpanLookup::Live(span) if core::ptr::eq(span,source))
    );
}
