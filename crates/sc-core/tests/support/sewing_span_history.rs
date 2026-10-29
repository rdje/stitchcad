//! Actual graph/history binding, held lookups and complete history accounting.
use super::{fixture, id, Fixture, SeamSideId, SeamSpanDefinition, SewingError, SewingGraph};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{
        charge_sewing_graph_resources, DesignResourceCount, DesignResourceLimitError,
        DesignResourceLimits, DesignResourceLocation, DesignResourceMeter, DesignResourceScope,
    },
    ontology::{
        EdgeAnchor, Param, Rational, SeamEasePortion, SeamSpanHistory,
        SeamSpanHistoryBindingError as Error, SeamSpanHistoryEntry as Entry, SeamSpanHistoryLimits,
        SewingGraphDefinition, SewingSpanLookup,
    },
};

fn history(entries: Vec<Entry>, roomy: bool) -> SeamSpanHistory {
    SeamSpanHistory::new(
        entries,
        SeamSpanHistoryLimits::new(
            NonZeroUsize::new(if roomy { 64 } else { 4 }).unwrap(),
            NonZeroUsize::new(if roomy { 128 } else { 8 }).unwrap(),
        ),
    )
    .unwrap()
}
fn midpoint(portion: &SeamEasePortion, side: SeamSideId) -> EdgeAnchor {
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
}
fn halves(portion: &SeamEasePortion) -> [SeamEasePortion; 2] {
    portion
        .split_at(
            midpoint(portion, SeamSideId::A),
            midpoint(portion, SeamSideId::B),
        )
        .unwrap()
}
fn quarters(f: &Fixture) -> SewingGraph {
    let parent = f.graph(&[]).unwrap();
    let root = parent.spans().first().unwrap().retain_ease_source();
    let [first, second] = halves(&root);
    let portions = halves(&first).into_iter().chain(halves(&second));
    SewingGraph::new(
        SewingGraphDefinition {
            id: parent.id(),
            spans: portions
                .zip(901_u128..)
                .map(|(portion, identity)| SeamSpanDefinition {
                    id: id(identity),
                    a: portion.side(SeamSideId::A),
                    b: portion.side(SeamSideId::B),
                    direction: portion.source().direction(),
                    ease: portion.into(),
                    stops: vec![],
                })
                .collect(),
        },
        &f.plan,
        &f.pieces,
        &[],
        &f.ledger,
    )
    .unwrap()
}
fn quarter_history() -> Vec<Entry> {
    vec![
        Entry::Root(id(301)),
        Entry::Split {
            parent: id(301),
            children: [id(910), id(911)],
        },
        Entry::Split {
            parent: id(910),
            children: [id(901), id(902)],
        },
        Entry::Split {
            parent: id(911),
            children: [id(903), id(904)],
        },
    ]
}

#[test]
fn span_history_binding_keeps_actual_live_borrows_legacy_absence_and_policy_independent_equality() {
    let f = fixture();
    let graph = f.graph(&[]).unwrap();
    assert!(graph.span_history().is_none());
    assert!(graph.lookup_span(id(99)).is_none());
    let Some(SewingSpanLookup::Live(span)) = graph.lookup_span(id(301)) else {
        unreachable!()
    };
    assert!(core::ptr::eq(span, graph.spans().first().unwrap()));
    let bound = graph
        .clone()
        .with_span_history(history(vec![Entry::Root(id(301))], false))
        .unwrap();
    let roomy = graph
        .with_span_history(history(vec![Entry::Root(id(301))], true))
        .unwrap();
    assert_eq!(bound, roomy);
    assert_eq!(bound.clone(), bound);
    assert_eq!(
        bound.span_history().unwrap().entries(),
        [Entry::Root(id(301))]
    );
    assert_eq!(
        bound.with_span_history(history(vec![], false)),
        Err(Error::AlreadyBound)
    );
}

#[test]
fn span_history_binding_rejects_actual_missing_retired_extra_and_graph_identity_records() {
    let mut f = fixture();
    assert_eq!(
        f.graph(&[])
            .unwrap()
            .with_span_history(history(vec![], false)),
        Err(Error::MissingLiveSpan {
            span: id(301),
            index: 0
        })
    );
    assert_eq!(
        f.graph(&[]).unwrap().with_span_history(history(
            vec![
                Entry::Root(id(301)),
                Entry::Split {
                    parent: id(301),
                    children: [id(401), id(402)]
                }
            ],
            false
        )),
        Err(Error::LiveSpanRetired {
            span: id(301),
            index: 0,
            split_entry: 1,
            children: [id(401), id(402)]
        })
    );
    assert_eq!(
        f.graph(&[]).unwrap().with_span_history(history(
            vec![Entry::Root(id(301)), Entry::Root(id(999))],
            false
        )),
        Err(Error::UnrepresentedLiveSpan {
            span: id(999),
            birth_entry: 1
        })
    );
    let mut second = f.span().clone();
    second.id = id(302);
    f.definition.spans.push(second);
    assert_eq!(
        f.graph(&[]).unwrap().with_span_history(history(
            vec![
                Entry::Root(id(300)),
                Entry::Split {
                    parent: id(300),
                    children: [id(301), id(302)]
                }
            ],
            false
        )),
        Err(Error::GraphIdentityCollision {
            span: id(300),
            birth_entry: 0
        })
    );
}

#[test]
fn span_history_binding_checks_retired_original_sources_and_actual_consumer_ancestry() {
    let f = fixture();
    let graph = quarters(&f);
    assert!(graph.lookup_span(id(301)).is_none());
    assert_eq!(graph.ease_sources().next().unwrap().span(), id(301));
    let roots: Vec<_> = (901_u128..=904)
        .map(|value| Entry::Root(id(value)))
        .collect();
    assert_eq!(
        graph
            .clone()
            .with_span_history(history(roots.clone(), true)),
        Err(Error::MissingSource {
            source_span: id(301),
            index: 0
        })
    );
    let mut source_live = roots;
    source_live.push(Entry::Root(id(301)));
    assert_eq!(
        graph.clone().with_span_history(history(source_live, true)),
        Err(Error::SourceNotRetired {
            source_span: id(301),
            index: 0,
            birth_entry: 4
        })
    );
    let unrelated = vec![
        Entry::Root(id(301)),
        Entry::Split {
            parent: id(301),
            children: [id(901), id(902)],
        },
        Entry::Root(id(77)),
        Entry::Split {
            parent: id(77),
            children: [id(903), id(904)],
        },
    ];
    assert_eq!(
        graph.clone().with_span_history(history(unrelated, false)),
        Err(Error::UnrelatedSource {
            span: id(903),
            source_span: id(301),
            index: 2
        })
    );
    let bound = graph
        .with_span_history(history(quarter_history(), false))
        .unwrap();
    for (identity, split, expected) in [
        (301, 1, [910, 911]),
        (910, 2, [901, 902]),
        (911, 3, [903, 904]),
    ] {
        let Some(SewingSpanLookup::RetiredSplit {
            split_entry,
            children,
            ..
        }) = bound.lookup_span(id(identity))
        else {
            unreachable!()
        };
        assert_eq!(split_entry, split);
        assert_eq!(*children, expected.map(id));
        let Some(Entry::Split {
            children: original, ..
        }) = bound.span_history().unwrap().entries().get(split)
        else {
            unreachable!()
        };
        assert!(core::ptr::eq(children, original));
    }
    for member in bound.spans() {
        let Some(SewingSpanLookup::Live(actual)) = bound.lookup_span(member.id()) else {
            unreachable!()
        };
        assert!(core::ptr::eq(actual, member));
    }
    bound
        .validate_current(&f.plan, &f.pieces, &[], &f.ledger)
        .unwrap();
}

#[test]
fn span_history_current_validation_rechecks_retired_ids_against_actual_external_namespaces() {
    for collision in [100_u128, 200, 400] {
        let mut f = fixture();
        let mark = f.notch(400, super::t(1, 4));
        let mut second = f.span().clone();
        second.id = id(302);
        f.definition.spans.push(second);
        let graph = f
            .graph(core::slice::from_ref(&mark))
            .unwrap()
            .with_span_history(history(
                vec![
                    Entry::Root(id(collision)),
                    Entry::Split {
                        parent: id(collision),
                        children: [id(301), id(302)],
                    },
                ],
                false,
            ))
            .unwrap();
        assert_eq!(
            graph.validate_current(&f.plan, &f.pieces, &[mark], &f.ledger),
            Err(SewingError::IdentityCollision { id: id(collision) })
        );
    }
}

fn resource_limits(entries: usize) -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::new(entries).unwrap(),
        NonZeroUsize::new(1).unwrap(),
        NonZeroU64::new(1).unwrap(),
    )
}
#[test]
fn span_history_resources_charge_actual_records_and_every_snapshot_without_source_queries() {
    let f = fixture();
    let graph = quarters(&f);
    let mut original = DesignResourceMeter::new(resource_limits(5));
    charge_sewing_graph_resources(&graph, &mut original).unwrap();
    assert_eq!(original.finish().unwrap().collection_entries(), 5);
    let bound = graph
        .with_span_history(history(quarter_history(), false))
        .unwrap();
    let mut meter = DesignResourceMeter::new(resource_limits(8));
    assert_eq!(
        charge_sewing_graph_resources(&bound, &mut meter),
        Err(DesignResourceLimitError::Count {
            unit: DesignResourceCount::CollectionEntries,
            maximum: 8,
            consumed: 5,
            requested: 4,
            location: DesignResourceLocation {
                scope: DesignResourceScope::Sewing,
                owner: Some(bound.id()),
                field: "sewing.span_history.entries",
                index: None
            },
        })
    );
    let mut full = DesignResourceMeter::new(resource_limits(18));
    charge_sewing_graph_resources(&bound, &mut full).unwrap();
    charge_sewing_graph_resources(&bound.clone(), &mut full).unwrap();
    assert_eq!(full.finish().unwrap().collection_entries(), 18);
}
