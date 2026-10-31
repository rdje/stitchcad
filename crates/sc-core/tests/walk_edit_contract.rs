//! Actual requested pairs and canonical source frames;no fixture claims geometric correspondence.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{
        WalkBalanceNotch as Mark, WalkBalancePair as Pair, WalkEditAdmissionRefusal as Admission,
        WalkEditBindingRefusal as Binding, WalkEditLimits as Limits, WalkSeamEdit as Edit,
    },
    ontology::*,
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("exact rational")).expect("unit interval")
}
fn profiles(seed: u128) -> NotchProfileBindings {
    let p = |n| ProfileParameterRef::new(id(seed + n));
    NotchProfileBindings {
        style: p(0),
        sample: NotchDimensionBindings {
            depth: p(1),
            width: p(2),
        },
        production: NotchDimensionBindings {
            depth: p(3),
            width: p(4),
        },
        encoding: p(5),
    }
}
struct Fixture {
    ledger: IdentityLedger,
    pieces: Vec<Piece>,
    plan: CutPlan,
    graph: SewingGraph,
    extra: EdgeRef,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let [a, b, extra] = std::array::from_fn(|_| {
        *ledger
            .declare_edges(&mut ids, 1)
            .expect("source allocation")
            .first()
            .expect("one source")
    });
    let pieces = [(id(100), a, vec![extra]), (id(101), b, vec![])]
        .into_iter()
        .map(|(identity, boundary, lines)| {
            let directed = |edge| DirectedEdge {
                edge,
                direction: Direction::Original,
            };
            Piece::new(
                PieceDefinition {
                    id: identity,
                    boundary: vec![directed(boundary)],
                    holes: vec![],
                    construction_lines: lines.into_iter().map(directed).collect(),
                    quantity: 1,
                    mirroring: Mirroring::Single,
                    cut_on_fold: false,
                    fold_edges: vec![],
                    cutting_side: CuttingSide::Face,
                    material: MaterialAssignment::Unresolved("test material".into()),
                    layer_index: 0,
                    label: LabelText {
                        name: "test".into(),
                        size: "M".into(),
                        fabric: "woven".into(),
                        colorway: "plain".into(),
                    },
                },
                &ledger,
            )
            .expect("structural Piece")
        })
        .collect::<Vec<_>>();
    let plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(200),
                piece: id(100),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(201),
                piece: id(101),
                orientation: CopyOrientation::Authored,
            },
        ],
        &pieces,
    )
    .expect("actual copy plan");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![SeamSpanDefinition {
                id: id(301),
                a: SeamSide {
                    copy: id(200),
                    range: EdgeRange::new(a, t(1, 7), t(6, 7)).expect("A source interval"),
                },
                b: SeamSide {
                    copy: id(201),
                    range: EdgeRange::new(b, t(1, 5), t(4, 5)).expect("B source interval"),
                },
                direction: SeamDirection::Opposite,
                ease: DeclaredEase {
                    amount: EaseAmount::Profile(ProfileParameterRef::new(id(9000))),
                    distribution: EaseDistribution::Uniform,
                }
                .into(),
                stops: vec![],
            }],
        },
        &plan,
        &pieces,
        &[],
        &ledger,
    )
    .expect("actual structurally valid graph");
    Fixture {
        ledger,
        pieces,
        plan,
        graph,
        extra,
    }
}
impl Fixture {
    fn span(&self) -> &SeamSpan {
        self.graph.spans().first().expect("canonical source")
    }
    fn pair(&self, a: Param, b: Param) -> Pair {
        Pair::new(
            Mark::new(
                EdgeAnchor {
                    edge: self.span().definition().a.range.edge(),
                    param: a,
                },
                profiles(1000),
            ),
            Mark::new(
                EdgeAnchor {
                    edge: self.span().definition().b.range.edge(),
                    param: b,
                },
                profiles(2000),
            ),
        )
    }
}
fn replace(pair: Pair, side: SeamSideId, anchor: EdgeAnchor) -> Pair {
    let changed = Mark::new(anchor, *pair.mark(side).representation());
    match side {
        SeamSideId::A => Pair::new(changed, *pair.mark(SeamSideId::B)),
        SeamSideId::B => Pair::new(*pair.mark(SeamSideId::A), changed),
    }
}

#[test]
fn actual_pair_and_span_fields_keep_all_original_owners_profiles_order_repeats_and_endpoints() {
    let f = fixture();
    let definition = f.span().definition();
    let repeated = f.pair(t(1, 3), t(2, 5));
    let endpoints = f.pair(definition.a.range.from(), definition.b.range.to());
    let edit = Edit::new(id(10), f.span().id(), vec![repeated, repeated, endpoints]);
    let before = edit.clone();
    let graph_before = f.graph.clone();
    let limits = Limits::new(3);
    let admitted = edit
        .admit(limits)
        .expect("complete original request population");
    assert!(std::ptr::eq(admitted.edit(), &edit));
    assert_eq!(admitted.limits(), limits);
    assert_eq!(limits.pairs(), 3);
    assert_eq!(admitted.notch_request_count(), 6);
    let bound = admitted
        .bind_source(&f.graph)
        .expect("actual original edge frames/inclusive bounds");
    assert!(std::ptr::eq(bound.admitted(), &admitted));
    assert!(std::ptr::eq(bound.graph(), &f.graph));
    assert!(std::ptr::eq(bound.span(), f.span()));
    assert_eq!(bound.pairs().len(), 3);
    for ((index, pair), view) in edit.pairs().iter().enumerate().zip(bound.pairs()) {
        assert_eq!(view.index(), index);
        assert!(std::ptr::eq(view.pair(), pair));
        assert!(std::ptr::eq(view.admitted(), &admitted));
        assert!(std::ptr::eq(view.graph(), &f.graph));
        assert!(std::ptr::eq(view.span(), f.span()));
        for side in [SeamSideId::A, SeamSideId::B] {
            let source = match side {
                SeamSideId::A => &definition.a,
                SeamSideId::B => &definition.b,
            };
            assert!(std::ptr::eq(view.source_side(side), source));
            assert!(std::ptr::eq(
                view.pair().mark(side).anchor(),
                pair.mark(side).anchor()
            ));
            assert!(std::ptr::eq(
                view.pair().mark(side).representation(),
                pair.mark(side).representation()
            ));
        }
    }
    assert_eq!(
        bound
            .pairs()
            .rev()
            .map(|pair| pair.index())
            .collect::<Vec<_>>(),
        vec![2, 1, 0]
    );
    assert_eq!(edit, before);
    assert_eq!(f.graph, graph_before);
    assert_eq!(edit.operation(), id(10));
    assert_eq!(edit.span_id(), id(301));
    assert_ne!(
        repeated.mark(SeamSideId::A).representation(),
        repeated.mark(SeamSideId::B).representation()
    );
}

#[test]
fn complete_population_quota_precedes_empty_or_local_sources_and_refusal_retains_all_owned_inputs()
{
    let f = fixture();
    let pair = f.pair(t(1, 3), t(2, 5));
    let edit = Edit::new(id(301), id(301), vec![pair, pair]);
    let before = edit.clone();
    let limits = Limits::new(1);
    let error = edit
        .admit(limits)
        .expect_err("whole count before definite self-span");
    assert!(std::ptr::eq(error.edit(), &edit));
    assert_eq!(error.limits(), limits);
    assert_eq!(
        error.refusal(),
        Admission::Pairs {
            allowed: 1,
            required: 2
        }
    );
    assert_eq!(edit, before);
    assert_eq!(
        edit.admit(Limits::new(2))
            .expect_err("exact quota reaches original local error")
            .refusal(),
        Admission::SelfSpan
    );
    let empty = Edit::new(id(301), id(301), vec![]);
    assert_eq!(
        empty
            .admit(Limits::new(0))
            .expect_err("report-only walking is a query")
            .refusal(),
        Admission::Empty
    );
    let valid = Edit::new(id(10), id(301), vec![pair]);
    assert_eq!(
        valid
            .admit(Limits::new(0))
            .expect_err("complete pair population")
            .refusal(),
        Admission::Pairs {
            allowed: 0,
            required: 1
        }
    );
    assert_eq!(
        valid
            .admit(Limits::new(1))
            .expect("exact allowance")
            .notch_request_count(),
        2
    );
}

#[test]
fn first_actual_pair_and_side_local_self_anchor_keeps_original_index_tag_and_parameter() {
    let f = fixture();
    let pair = f.pair(t(1, 3), t(2, 5));
    for side in [SeamSideId::A, SeamSideId::B] {
        let anchor = *pair.mark(side).anchor();
        let edit = Edit::new(anchor.edge.creator(), id(301), vec![pair]);
        assert_eq!(
            edit.admit(Limits::new(1))
                .expect_err("actual side self-supply")
                .refusal(),
            Admission::SelfAnchor {
                pair: 0,
                side,
                anchor
            }
        );
    }
    let other = EdgeAnchor {
        edge: f.extra,
        param: t(1, 2),
    };
    let first = Pair::new(
        Mark::new(other, profiles(1000)),
        Mark::new(other, profiles(2000)),
    );
    let anchor = *pair.mark(SeamSideId::A).anchor();
    let both = replace(pair, SeamSideId::B, anchor);
    let edit = Edit::new(anchor.edge.creator(), id(301), vec![first, both]);
    assert_eq!(
        edit.admit(Limits::new(2))
            .expect_err("later original pair;A before B")
            .refusal(),
        Admission::SelfAnchor {
            pair: 1,
            side: SeamSideId::A,
            anchor
        }
    );
}

#[test]
fn actual_missing_span_then_original_pair_side_edge_or_position_refusals_keep_source_context() {
    let f = fixture();
    let pair = f.pair(t(1, 3), t(2, 5));
    let invalid = replace(
        pair,
        SeamSideId::A,
        EdgeAnchor {
            edge: f.extra,
            param: Param::START,
        },
    );
    let missing = Edit::new(id(10), id(999), vec![invalid]);
    let admitted = missing.admit(Limits::new(1)).expect("input stage");
    let error = admitted
        .bind_source(&f.graph)
        .expect_err("missing actual target before mark checks");
    assert!(std::ptr::eq(error.admitted(), &admitted));
    assert!(std::ptr::eq(error.graph(), &f.graph));
    assert_eq!(error.refusal(), Binding::MissingSpan);
    for (side, anchor, edge_error) in [
        (
            SeamSideId::A,
            EdgeAnchor {
                edge: f.extra,
                param: Param::START,
            },
            true,
        ),
        (
            SeamSideId::A,
            EdgeAnchor {
                edge: f.span().definition().a.range.edge(),
                param: Param::START,
            },
            false,
        ),
        (
            SeamSideId::B,
            EdgeAnchor {
                edge: f.span().definition().b.range.edge(),
                param: Param::END,
            },
            false,
        ),
        (
            SeamSideId::B,
            EdgeAnchor {
                edge: f.extra,
                param: t(1, 2),
            },
            true,
        ),
    ] {
        let bad = replace(pair, side, anchor);
        let edit = Edit::new(id(10), id(301), vec![pair, bad]);
        let admitted = edit
            .admit(Limits::new(2))
            .expect("complete two-pair request");
        let error = admitted
            .bind_source(&f.graph)
            .expect_err("first original later bad field");
        let source = match side {
            SeamSideId::A => &f.span().definition().a.range,
            SeamSideId::B => &f.span().definition().b.range,
        };
        let expected = if edge_error {
            Binding::Edge {
                pair: 1,
                side,
                anchor: &anchor,
                source,
            }
        } else {
            Binding::Position {
                pair: 1,
                side,
                anchor: &anchor,
                source,
            }
        };
        assert_eq!(error.refusal(), expected);
        if let Binding::Edge {
            anchor: held,
            source: original,
            ..
        }
        | Binding::Position {
            anchor: held,
            source: original,
            ..
        } = error.refusal()
        {
            assert!(std::ptr::eq(
                held,
                edit.pairs()
                    .get(1)
                    .expect("actual original pair")
                    .mark(side)
                    .anchor()
            ));
            assert!(std::ptr::eq(original, source));
        }
        assert!(std::ptr::eq(error.admitted().edit(), &edit));
    }
}

#[test]
fn bounds_do_not_prove_correspondence_profile_truth_or_current_graph_after_source_deletion() {
    let mut f = fixture();
    let pair = f.pair(t(1, 3), t(2, 5));
    assert_ne!(
        pair.mark(SeamSideId::A).anchor().param,
        pair.mark(SeamSideId::B).anchor().param
    );
    let edit = Edit::new(id(10), id(301), vec![pair]);
    let admitted = edit.admit(Limits::new(1)).expect("explicit requests");
    let original = f.span().definition().clone();
    assert_eq!(
        f.graph.validate_current(&f.plan, &f.pieces, &[], &f.ledger),
        Ok(())
    );
    f.ledger
        .delete(
            &mut DeterministicIdGenerator::from_start(10000),
            original.a.range.edge(),
        )
        .expect("delete actual source");
    let error = f
        .graph
        .validate_current(&f.plan, &f.pieces, &[], &f.ledger)
        .expect_err("real missing current range");
    assert!(
        matches!(error,SewingError::UnresolvedRange { span,side:SeamSideId::A,evidence }
        if span==id(301) && !evidence.has_full_coverage())
    );
    let bound = admitted
        .bind_source(&f.graph)
        .expect("original association/bounds remain structural");
    assert_eq!(bound.span().definition(), &original);
    assert_eq!(bound.span().definition().direction, SeamDirection::Opposite);
    assert_eq!(
        bound.span().definition().ease.declaration().amount,
        EaseAmount::Profile(ProfileParameterRef::new(id(9000)))
    );
    assert_eq!(
        f.graph.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
}
