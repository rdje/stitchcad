//! Actual Design ownership and resources for bound live/retired span histories.
use super::{
    id, state, DesignSewing, DesignSewingDefinition, DesignSources, EaseAmount, SeamSideId,
};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{
        charge_design_sewing_resources, DesignReferenceHost, DesignResourceLimits,
        DesignResourceMeter, DesignSewingError, DesignSewingPosition,
    },
    ontology::{
        EdgeAnchor, EntityId, Param, Rational, SeamSpanDefinition, SeamSpanHistory,
        SeamSpanHistoryEntry as Entry, SeamSpanHistoryLimits, SewingGraph, SewingGraphDefinition,
    },
};
use sc_units::Length;

fn nested_graph(
    state: &DesignSewing<DesignSources>,
) -> (SewingGraph, EntityId, EntityId, EntityId) {
    let graph = state.graph();
    let first = graph.spans().first().expect("actual first retained child");
    let second = graph.spans().last().expect("actual second retained child");
    let root = first
        .definition()
        .ease
        .portion()
        .expect("actual original portion");
    let midpoint = |side| {
        let range = root.side(side).range;
        EdgeAnchor {
            edge: range.edge(),
            param: Param::new(
                range
                    .from()
                    .as_rational()
                    .checked_add(range.to().as_rational())
                    .expect("small actual interval")
                    .checked_div(Rational::from_integer(2))
                    .expect("exact midpoint"),
            )
            .expect("actual interior position"),
        }
    };
    let portions = root
        .split_at(midpoint(SeamSideId::A), midpoint(SeamSideId::B))
        .expect("actual source portion split");
    let mut definitions: Vec<_> = portions
        .into_iter()
        .zip(99_201_u128..)
        .map(|(portion, identity)| SeamSpanDefinition {
            id: id(identity),
            a: portion.side(SeamSideId::A),
            b: portion.side(SeamSideId::B),
            direction: portion.source().direction(),
            ease: portion.into(),
            stops: vec![],
        })
        .collect();
    definitions.push(second.definition().clone());
    let structure = state.inputs().structure();
    let candidate = SewingGraph::new(
        SewingGraphDefinition {
            id: graph.id(),
            spans: definitions,
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        state.landmarks(),
        structure.topology(),
    )
    .expect("actual current graph with retained full source and three leaves");
    (candidate, root.source().span(), first.id(), second.id())
}
fn history(source: EntityId, retired: EntityId, sibling: EntityId, roomy: bool) -> SeamSpanHistory {
    SeamSpanHistory::new(
        vec![
            Entry::Root(source),
            Entry::Split {
                parent: source,
                children: [retired, sibling],
            },
            Entry::Split {
                parent: retired,
                children: [id(99_201), id(99_202)],
            },
        ],
        SeamSpanHistoryLimits::new(
            NonZeroUsize::new(if roomy { 16 } else { 3 }).expect("positive record allowance"),
            NonZeroUsize::new(if roomy { 32 } else { 5 }).expect("positive identity allowance"),
        ),
    )
    .expect("valid local identity history")
}
fn owned<H: DesignReferenceHost + Clone>(
    original: &DesignSewing<H>,
    graph: SewingGraph,
) -> Result<DesignSewing<H>, DesignSewingError<H::Error>> {
    DesignSewing::new(DesignSewingDefinition {
        inputs: original.inputs().clone(),
        graph,
        landmarks: original.landmarks().to_vec(),
    })
}
fn resources(state: &DesignSewing<DesignSources>) -> sc_core::design::DesignResourceDemand {
    let mut meter = DesignResourceMeter::new(DesignResourceLimits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroU64::MAX,
    ));
    charge_design_sewing_resources(state, &mut meter).expect("complete actual root walk");
    meter.finish().expect("no partial resource demand")
}

#[test]
fn span_history_design_owns_every_retired_id_and_source_alias_once_and_preserves_actual_body() {
    let original = state(EaseAmount::Explicit(Length::ZERO), SeamSideId::A);
    let saved = original.clone();
    let (graph, source, retired, sibling) = nested_graph(&original);
    let legacy =
        owned(&original, graph.clone()).expect("held actual graph without fabricated past history");
    let bound = owned(
        &original,
        graph
            .clone()
            .with_span_history(history(source, retired, sibling, false))
            .expect("actual leaves and source ancestry"),
    )
    .expect("complete actual ownership");
    let roomy = owned(
        &original,
        graph
            .with_span_history(history(source, retired, sibling, true))
            .expect("same metadata under different input policy"),
    )
    .expect("complete actual ownership");
    assert_eq!(bound, roomy);
    assert_eq!(bound.identities().len(), legacy.identities().len() + 1);
    let identities = bound.identities();
    for (identity, entry, child) in [
        (source, 0, None),
        (retired, 1, Some(0)),
        (sibling, 1, Some(1)),
        (id(99_201), 2, Some(0)),
        (id(99_202), 2, Some(1)),
    ] {
        let matches: Vec<_> = identities
            .iter()
            .filter(|(actual, _)| *actual == identity)
            .collect();
        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches.first().expect("exact canonical birth").1,
            DesignSewingPosition::SpanHistory { entry, child }
        );
    }
    assert!(core::ptr::eq(
        bound.graph().ease_sources().next().expect("canonical root"),
        original
            .graph()
            .ease_sources()
            .next()
            .expect("unchanged actual original root")
    ));
    assert_eq!(
        bound
            .amount_declarations()
            .expect("actual complete current sources")
            .len(),
        3
    );
    let before = resources(&legacy);
    let after = resources(&bound);
    assert_eq!(after.collection_entries(), before.collection_entries() + 3);
    assert_eq!(after.text_bytes(), before.text_bytes());
    assert_eq!(after.counter_bits(), before.counter_bits());
    assert_eq!(resources(&bound.clone()), after);
    assert_eq!(original, saved);
}

#[test]
fn span_history_design_rejects_retired_namespace_collision_with_actual_root_inputs() {
    let original = state(EaseAmount::Explicit(Length::ZERO), SeamSideId::A);
    let saved = original.clone();
    let (graph, source, _, sibling) = nested_graph(&original);
    let (collision, position) = original
        .inputs()
        .identities()
        .into_iter()
        .next()
        .expect("actual canonical input owner");
    let graph = graph
        .with_span_history(history(source, collision, sibling, false))
        .expect("local history attachment cannot grant global input ownership");
    assert!(
        matches!(owned(&original, graph), Err(DesignSewingError::DuplicateIdentity {
        identity, first: DesignSewingPosition::Inputs(actual), duplicate: DesignSewingPosition::SpanHistory { entry: 1, child: Some(0) }
    }) if identity == collision && actual == position)
    );
    assert_eq!(original, saved);
}
