//! Actual held Gather inspection preserves immediate retirement evidence and existing refusals.
use super::*;
use core::num::NonZeroUsize;
use sc_core::ontology::{
    SeamSpanHistory, SeamSpanHistoryEntry as Entry, SeamSpanHistoryLimits, SewingSpanLookup,
};

fn history_limits() -> SeamSpanHistoryLimits {
    SeamSpanHistoryLimits::new(
        NonZeroUsize::new(8).unwrap(),
        NonZeroUsize::new(16).unwrap(),
    )
}
fn split(
    f: &Fixture,
    graph: &SewingGraph,
    target: EntityId,
    children: [EntityId; 2],
) -> SewingGraph {
    let span = graph
        .spans()
        .iter()
        .find(|span| span.id() == target)
        .unwrap();
    let portion = span.retain_ease_source();
    let middle = |side| {
        let range = portion.side(side).range;
        EdgeAnchor {
            edge: range.edge(),
            param: Param::new(
                range
                    .from()
                    .as_rational()
                    .checked_add(range.to().as_rational())
                    .unwrap()
                    .checked_div(Rational::from_integer(2))
                    .unwrap(),
            )
            .unwrap(),
        }
    };
    let definitions = portion
        .split_at(middle(SeamSideId::A), middle(SeamSideId::B))
        .unwrap()
        .into_iter()
        .zip(children)
        .map(|(portion, id)| SeamSpanDefinition {
            id,
            a: portion.side(SeamSideId::A),
            b: portion.side(SeamSideId::B),
            direction: portion.source().direction(),
            ease: portion.into(),
            stops: vec![],
        })
        .collect::<Vec<_>>();
    let mut spans = Vec::new();
    for span in graph.spans() {
        if span.id() == target {
            spans.extend(definitions.iter().cloned());
        } else {
            spans.push(span.definition().clone());
        }
    }
    SewingGraph::new(
        SewingGraphDefinition {
            id: graph.id(),
            spans,
        },
        &f.plan,
        &f.pieces,
        &[],
        &f.ledger,
    )
    .unwrap()
    .with_span_history(
        graph
            .span_history()
            .unwrap()
            .with_split(target, children, history_limits())
            .unwrap(),
    )
    .unwrap()
}

#[test]
fn held_gather_span_inspection_borrows_actual_live_and_nested_immediate_retirement_records() {
    let f = fixture();
    let graph = f
        .graph(&[])
        .unwrap()
        .with_span_history(
            SeamSpanHistory::new(vec![Entry::Root(f.span().id)], history_limits()).unwrap(),
        )
        .unwrap();
    let input = f.gather_definition(SeamSideId::A);
    let gather = f.gather(input.clone(), &graph).unwrap();
    let SewingSpanLookup::Live(actual) = gather.inspect_span_reference(&graph).unwrap() else {
        panic!("actual live member")
    };
    assert!(core::ptr::eq(actual, graph.spans().first().unwrap()));
    let children = [id(702), id(701)];
    let first = split(&f, &graph, input.span, children);
    let nested = split(&f, &first, children[0], [id(800), id(799)]);
    for candidate in [&first, &nested] {
        let SewingSpanLookup::RetiredSplit {
            birth_entry,
            split_entry,
            children: actual,
        } = gather.inspect_span_reference(candidate).unwrap()
        else {
            panic!("original immediate split")
        };
        assert_eq!((birth_entry, split_entry), (0, 1));
        assert_eq!(*actual, children);
        let Entry::Split {
            children: retained, ..
        } = candidate.span_history().unwrap().entries().get(1).unwrap()
        else {
            panic!("actual original split")
        };
        assert!(core::ptr::eq(actual, retained));
        assert_eq!(
            gather.intake_source(candidate),
            Err(GatherError::MissingSpan(input.span))
        );
        assert_eq!(
            gather.validate_current(f.pieces.first().unwrap(), &f.plan, candidate, &f.ledger),
            Err(GatherError::MissingSpan(input.span))
        );
        assert_eq!(gather.definition(), &input);
    }
    assert!(matches!(
        graph.lookup_span(input.span),
        Some(SewingSpanLookup::Live(_))
    ));
}

#[test]
fn held_gather_span_inspection_preserves_wrong_graph_unknown_and_legacy_absence_and_copy_checks() {
    let mut f = fixture();
    let graph = f
        .graph(&[])
        .unwrap()
        .with_span_history(
            SeamSpanHistory::new(vec![Entry::Root(f.span().id)], history_limits()).unwrap(),
        )
        .unwrap();
    let input = f.gather_definition(SeamSideId::A);
    let gather = f.gather(input.clone(), &graph).unwrap();
    let candidate = split(&f, &graph, input.span, [id(702), id(701)]);
    let raw = candidate
        .spans()
        .iter()
        .map(|span| span.definition().clone())
        .collect::<Vec<_>>();
    let legacy = SewingGraph::new(
        SewingGraphDefinition {
            id: graph.id(),
            spans: raw.clone(),
        },
        &f.plan,
        &f.pieces,
        &[],
        &f.ledger,
    )
    .unwrap();
    assert!(legacy
        .ease_sources()
        .any(|source| source.span() == input.span));
    assert_eq!(
        gather.inspect_span_reference(&legacy).unwrap_err(),
        GatherError::MissingSpan(input.span)
    );
    let wrong = SewingGraph::new(
        SewingGraphDefinition {
            id: id(399),
            spans: raw,
        },
        &f.plan,
        &f.pieces,
        &[],
        &f.ledger,
    )
    .unwrap();
    assert_eq!(
        gather.inspect_span_reference(&wrong).unwrap_err(),
        GatherError::WrongGraph {
            held: input.graph,
            provided: id(399)
        }
    );
    f.span_mut().a.copy = id(201);
    let changed = f.graph(&[]).unwrap();
    assert!(
        matches!(gather.inspect_span_reference(&changed).unwrap(),SewingSpanLookup::Live(span) if span.id()==input.span)
    );
    assert_eq!(
        gather.intake_source(&changed),
        Err(GatherError::CopyBindingChanged {
            held: input.copy,
            current: id(201)
        })
    );
    assert_eq!(gather.definition(), &input);
}
