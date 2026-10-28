//! Actual owned Design integration for original-notch context; quantity/effect acceptance follows.
use super::{fixture, id, with_sewing_graph, DesignInputs, DesignSources, DesignStructure};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{DesignSewing, DesignSewingAmountSource, DesignSewingDefinition, DesignSewingError},
    ontology::{
        CopyOrientation, CutCopyDefinition, CutPlan, EaseAmount, EaseDistribution, EdgeAnchor,
        Notch, NotchContextLimits, NotchDefinition, NotchDimensionBindings, NotchProfileBindings,
        NotchSourceValidationIssue, OriginalPointLimits, Param, ProfileBindingValidation,
        ProfileParameterRef, Rational, SeamSideId, SeamSpanDefinition, SewingError, SewingGraph,
        SewingGraphDefinition, SewingLandmark, StopLandmark, UniquePointLimits,
    },
};

fn at(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("exact fraction")).expect("unit position")
}
fn limits(journal: usize, portions: usize) -> NotchContextLimits {
    NotchContextLimits::new(
        NonZeroUsize::new(10).expect("all2 stops+2 copies+2 Pieces+2 boundaries+2 Notches"),
        OriginalPointLimits::new(
            UniquePointLimits::new(
                NonZeroUsize::new(journal).expect("actual journal population"),
                NonZeroUsize::new(2).expect("actual2 declared edges"),
            ),
            NonZeroUsize::new(1).expect("no inverse branching in these cases"),
            NonZeroU64::new(1_000).expect("explicit original integer allowance"),
        ),
        NonZeroUsize::new(portions).expect("explicit coverage work allowance"),
    )
}
fn state(amount: EaseAmount, side: SeamSideId) -> DesignSewing<DesignSources> {
    state_at(amount, side, at(1, 2), at(1, 2))
}
fn state_at(
    amount: EaseAmount,
    side: SeamSideId,
    a_cut: Param,
    b_cut: Param,
) -> DesignSewing<DesignSources> {
    let (inputs, graph) = with_sewing_graph(fixture(), vec![amount]);
    let mut definition = graph
        .spans()
        .first()
        .expect("actual root")
        .definition()
        .clone();
    let structure = inputs.structure();
    let (index, edge) = match side {
        SeamSideId::A => (0, definition.a.range.edge()),
        SeamSideId::B => (1, definition.b.range.edge()),
    };
    let piece = structure
        .pieces()
        .pieces()
        .get(index)
        .expect("actual side owner");
    let profile = ProfileParameterRef::new(id(98_010));
    let representation = NotchProfileBindings {
        style: profile,
        sample: NotchDimensionBindings {
            depth: profile,
            width: profile,
        },
        production: NotchDimensionBindings {
            depth: profile,
            width: profile,
        },
        encoding: profile,
    };
    let landmarks = [(98_000, at(1, 4)), (98_001, at(3, 4))]
        .into_iter()
        .map(|(identity, param)| {
            SewingLandmark::Notch(
                Notch::new(
                    NotchDefinition {
                        id: id(identity),
                        anchor: EdgeAnchor { edge, param },
                        representation,
                    },
                    piece,
                    structure.topology(),
                )
                .expect("actual semantic Notch"),
            )
        })
        .collect::<Vec<_>>();
    definition
        .ease
        .declared_mut()
        .expect("actual declaration")
        .distribution = EaseDistribution::BetweenNotches {
        side,
        start: id(98_001),
        end: id(98_000),
    };
    definition.stops = [98_000, 98_001]
        .map(|identity| StopLandmark {
            side,
            landmark: id(identity),
        })
        .to_vec();
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(94_001),
            spans: vec![definition],
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        &landmarks,
        structure.topology(),
    )
    .expect("actual original notch source");
    let root = graph.spans().first().expect("root").retain_ease_source();
    let portions = root
        .split_at(
            EdgeAnchor {
                edge: root.side(SeamSideId::A).range.edge(),
                param: a_cut,
            },
            EdgeAnchor {
                edge: root.side(SeamSideId::B).range.edge(),
                param: b_cut,
            },
        )
        .expect("descriptive retained source partition,no physical correspondence claim");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(94_000),
            spans: portions
                .into_iter()
                .zip(97_001_u128..)
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
        structure.cut_plan(),
        structure.pieces().pieces(),
        &landmarks,
        structure.topology(),
    )
    .expect("actual retained children with full original stops");
    DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks,
    })
    .expect("actual complete cross-crate input and sewing owner")
}

#[test]
fn original_notch_design_context_binds_retained_views_actual_notches_and_symbolic_sources() {
    for side in [SeamSideId::A, SeamSideId::B] {
        for amount in [
            EaseAmount::Explicit(sc_units::Length::from_micrometres(-1).expect("signed literal")),
            EaseAmount::Formula(id(5_001)),
            EaseAmount::Profile(ProfileParameterRef::new(id(98_010))),
        ] {
            let state = state(amount, side);
            let saved = state.clone();
            let structure = state.inputs().structure();
            let canonical = state
                .graph()
                .ease_sources()
                .next()
                .expect("one canonical source");
            assert_eq!(state.graph().ease_sources().len(), 1);
            assert_eq!(canonical.span(), id(97_000));
            assert_eq!(
                canonical
                    .stops()
                    .iter()
                    .map(|stop| stop.landmark)
                    .collect::<Vec<_>>(),
                [id(98_000), id(98_001)]
            );
            let views = state
                .amount_declarations()
                .expect("actual complete current sewing sources");
            assert_eq!(views.len(), 2);
            for view in views {
                assert!(view.span().definition().stops.is_empty());
                assert!(core::ptr::eq(
                    view.canonical_inputs(),
                    state.inputs().inputs()
                ));
                match (amount, view.source()) {
                    (
                        EaseAmount::Explicit(expected),
                        DesignSewingAmountSource::Explicit(actual),
                    ) => assert_eq!(expected, *actual),
                    (
                        EaseAmount::Formula(expected),
                        DesignSewingAmountSource::Formula {
                            parameter,
                            declaration: _,
                        },
                    ) => assert_eq!(expected, parameter.id()),
                    (EaseAmount::Profile(expected), DesignSewingAmountSource::Profile(actual)) => {
                        assert_eq!(expected, actual);
                        assert_eq!(
                            view.profile_validation(),
                            Some(ProfileBindingValidation::DeferredToG4)
                        );
                    }
                    _ => unreachable!("actual source kind must survive"),
                }
                let source = view
                    .portion()
                    .expect("actual child source mapping")
                    .source();
                assert!(core::ptr::eq(source, canonical));
                let admission = source
                    .admit_notch_context(
                        structure.cut_plan(),
                        structure.pieces().pieces(),
                        state.landmarks(),
                        structure.topology(),
                        limits(1, 2),
                    )
                    .expect("complete actual dependencies");
                assert_eq!(admission.dependency_item_count(), 10);
                let proof = admission
                    .validate_sources()
                    .expect("bounded actual local context");
                assert_eq!(proof.interval_item_bound(), 20);
                assert_eq!(proof.index_item_bound(), 31);
                let context = proof.original_context().expect("actual original interval");
                let (start, end) = context.notches();
                assert_eq!((start.id(), end.id()), (id(98_001), id(98_000)));
                let SewingLandmark::Notch(expected) =
                    state.landmarks().get(1).expect("named start")
                else {
                    unreachable!("Notch")
                };
                assert!(core::ptr::eq(start, expected));
                assert!(core::ptr::eq(context.lower(), context.end()));
                assert!(core::ptr::eq(context.upper(), context.start()));
                assert_eq!(
                    context
                        .lower()
                        .unique()
                        .expect("unique")
                        .coordinate()
                        .as_param(),
                    Ok(at(1, 4))
                );
                assert_eq!(
                    context
                        .upper()
                        .unique()
                        .expect("unique")
                        .coordinate()
                        .as_param(),
                    Ok(at(3, 4))
                );
                assert_eq!(
                    context.lower().admission().source_edge(),
                    canonical.side(side).range.edge()
                );
                assert_eq!(context.candidate_item_bound(), 2);
                assert_eq!(source.declaration().amount, amount);
            }
            assert_eq!(state, saved);
        }
    }
}

#[test]
fn original_notch_design_context_rechecks_copy_replacement_against_full_original_source() {
    let state = state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.cut_plan = CutPlan::new(
        structure
            .cut_plan
            .copies()
            .iter()
            .map(|copy| CutCopyDefinition {
                id: if copy.id() == id(96_000) {
                    id(96_010)
                } else {
                    copy.id()
                },
                piece: copy.piece(),
                orientation: CopyOrientation::Authored,
            })
            .collect(),
        structure.pieces.pieces(),
    )
    .expect("explicit replacement copy");
    inputs.structure = DesignStructure::new(structure).expect("actual replacement structural root");
    let held = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual current inputs"),
        graph: state.graph().clone(),
        landmarks: state.landmarks().to_vec(),
    })
    .expect("original held graph remains inspectable");
    assert!(
        matches!(held.validate_current(), Err(DesignSewingError::Graph(issue))
        if matches!(*issue,SewingError::MissingCopy { span,side:SeamSideId::A,copy }
        if span==id(97_001) && copy==id(96_000)))
    );
    let source = held
        .graph()
        .ease_sources()
        .next()
        .expect("actual full original source");
    let structure = held.inputs().structure();
    let admitted = source
        .admit_notch_context(
            structure.cut_plan(),
            structure.pieces().pieces(),
            held.landmarks(),
            structure.topology(),
            limits(1, 2),
        )
        .expect("bounded input evidence alone is not ownership proof");
    let error = admitted
        .validate_sources()
        .expect_err("recheck actual current copy binding");
    assert_eq!(error.source_span(), id(97_000));
    assert!(
        matches!(error.issue(),NotchSourceValidationIssue::Structural(issue)
        if matches!(issue.as_ref(),SewingError::MissingCopy { span,side:SeamSideId::A,copy }
        if *span==id(97_000) && *copy==id(96_000)))
    );
    assert_eq!(source.declaration().amount, EaseAmount::Formula(id(5_001)));
}

#[test]
fn original_notch_design_context_keeps_wide_originals_without_clearing_child_endpoint_choices() {
    use sc_core::ontology::{DeterministicIdGenerator, Resolution};
    let state = state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let edge = state
        .graph()
        .ease_sources()
        .next()
        .expect("original source")
        .side(SeamSideId::A)
        .range
        .edge();
    let mut edits = DeterministicIdGenerator::from_start(110_000);
    let fragments = structure
        .topology
        .split(&mut edits, edge, at(1, 2))
        .expect("actual later split");
    let piece = structure
        .pieces
        .pieces()
        .first()
        .expect("actual held Piece");
    let landmarks = state
        .landmarks()
        .iter()
        .zip([at(1, i64::MAX), at(1, 2)])
        .map(|(landmark, param)| {
            let SewingLandmark::Notch(notch) = landmark else {
                unreachable!("actual Notch")
            };
            let mut definition = notch.definition().clone();
            definition.anchor = EdgeAnchor {
                edge: fragments.first(),
                param,
            };
            SewingLandmark::Notch(
                Notch::new(definition, piece, &structure.topology)
                    .expect("explicit later-frame Notch replacement"),
            )
        })
        .collect::<Vec<_>>();
    assert!(matches!(
        structure.topology.resolve(edge, at(1, 2)),
        Resolution::SplitPoint {
            first: _,
            second: _
        }
    ));
    inputs.structure = DesignStructure::new(structure).expect("actual edited structural root");
    let held = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual cross-crate current inputs"),
        graph: state.graph().clone(),
        landmarks,
    })
    .expect("held child graph and explicit current landmark replacements");
    let saved = held.clone();
    assert!(
        matches!(held.validate_current(), Err(DesignSewingError::Graph(issue))
        if matches!(*issue,SewingError::UnresolvedRange { span,side:SeamSideId::A,evidence:_ } if span==id(97_001)))
    );
    let source = held
        .graph()
        .ease_sources()
        .next()
        .expect("same original source");
    let structure = held.inputs().structure();
    let context = source
        .admit_notch_context(
            structure.cut_plan(),
            structure.pieces().pieces(),
            held.landmarks(),
            structure.topology(),
            limits(2, 3),
        )
        .expect("complete actual inputs")
        .validate_sources()
        .expect("full original source remains covered")
        .original_context()
        .expect("unique distinct actual original Notches");
    let lower = context
        .lower()
        .unique()
        .expect("unique tiny original")
        .coordinate();
    assert_eq!(
        lower.as_u128_ratio(),
        Ok((1, 2 * u128::try_from(i64::MAX).expect("positive maximum")))
    );
    assert!(lower.as_param().is_err());
    assert_eq!(
        context
            .upper()
            .unique()
            .expect("unique upper")
            .coordinate()
            .as_param(),
        Ok(at(1, 4))
    );
    assert!(matches!(
        structure.topology().resolve(edge, at(1, 2)),
        Resolution::SplitPoint {
            first: _,
            second: _
        }
    ));
    assert_eq!(source.declaration().amount, EaseAmount::Formula(id(5_001)));
    assert_eq!(held, saved);
}

#[test]
fn original_notch_design_quantity_keeps_original_signed_source_and_current_refusal_boundaries() {
    use sc_core::ontology::NotchEaseLimits;
    use sc_units::Length;
    for side in [SeamSideId::A, SeamSideId::B] {
        for sign in [-1, 1] {
            let parent = Length::from_micrometres(2 * sign).expect("full original signed total");
            let state = state_at(EaseAmount::Explicit(parent), side, at(1, 4), at(3, 4));
            let saved = state.clone();
            let structure = state.inputs().structure();
            let canonical = state
                .graph()
                .ease_sources()
                .next()
                .expect("single original source");
            let context = canonical
                .admit_notch_context(
                    structure.cut_plan(),
                    structure.pieces().pieces(),
                    state.landmarks(),
                    structure.topology(),
                    limits(1, 2),
                )
                .expect("actual full original dependencies")
                .validate_sources()
                .expect("actual local current source")
                .original_context()
                .expect("actual original Notches");
            let quantities = context
                .admit_explicit(NotchEaseLimits::new(
                    NonZeroU64::new(473).expect("separate quantity allowance"),
                ))
                .expect("full signed source under exact conservative limit");
            let portions = state
                .amount_declarations()
                .expect("complete actual current declarations")
                .into_iter()
                .map(|view| {
                    let DesignSewingAmountSource::Explicit(actual) = view.source() else {
                        unreachable!("actual literal source")
                    };
                    assert_eq!(*actual, parent);
                    assert!(view.span().definition().stops.is_empty());
                    let portion = view.portion().expect("actual retained target domain");
                    assert!(core::ptr::eq(portion.source(), canonical));
                    portion.side(side).range
                })
                .collect::<Vec<_>>();
            let amounts = portions
                .iter()
                .map(|range| {
                    quantities
                        .amount_in(*range)
                        .expect("original rounded boundaries")
                })
                .collect::<Vec<_>>();
            assert_eq!(amounts, [Length::ZERO, parent]);
            assert_eq!(
                amounts
                    .first()
                    .expect("first")
                    .checked_add(*amounts.get(1).expect("second")),
                Ok(parent)
            );
            assert_eq!(quantities.amount_in(canonical.side(side).range), Ok(parent));
            assert_eq!(canonical.stops().len(), 2);
            let (_, uniform) = with_sewing_graph(fixture(), vec![EaseAmount::Explicit(parent)]);
            let uniform = uniform
                .spans()
                .first()
                .expect("actual Uniform control")
                .retain_ease_source();
            assert_eq!(
                uniform.source().uniform_explicit_cumulative_at_a(at(1, 4)),
                Ok(Length::from_micrometres(sign).expect("Uniform spends half quantum at quarter"))
            );
            assert_eq!(state, saved);

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
            inputs.structure =
                DesignStructure::new(structure).expect("actual current structural snapshot");
            let held = DesignSewing::new(DesignSewingDefinition {
                inputs: DesignInputs::new(inputs).expect("actual current cross-crate root"),
                graph: state.graph().clone(),
                landmarks: state.landmarks().to_vec(),
            })
            .expect("held source is inspectable after actual replacement");
            assert!(
                matches!(held.amount_declarations(),Err(DesignSewingError::Graph(issue))
                if matches!(*issue,SewingError::MissingCopy {span,side:SeamSideId::A,copy}
                if span==id(97_001) && copy==id(96_000)))
            );
            let source = held
                .graph()
                .ease_sources()
                .next()
                .expect("held full original source");
            let structure = held.inputs().structure();
            let issue = source
                .admit_notch_context(
                    structure.cut_plan(),
                    structure.pieces().pieces(),
                    held.landmarks(),
                    structure.topology(),
                    limits(1, 2),
                )
                .expect("input evidence remains available")
                .validate_sources()
                .expect_err("fresh actual copy binding must refuse");
            assert!(
                matches!(issue.issue(),NotchSourceValidationIssue::Structural(issue)
                if matches!(issue.as_ref(),SewingError::MissingCopy {span,side:SeamSideId::A,copy}
                if *span==id(97_000) && *copy==id(96_000)))
            );
            assert_eq!(
                portions
                    .iter()
                    .map(|range| quantities
                        .amount_in(*range)
                        .expect("original immutable mathematical snapshot"))
                    .collect::<Vec<_>>(),
                amounts
            );
            assert_eq!(state, saved);
        }
    }
}

#[test]
fn original_notch_design_quantity_preserves_formula_and_profile_sources_without_values() {
    use sc_core::ontology::{NotchEaseLimits, NotchEaseWorkspaceError};
    for side in [SeamSideId::A, SeamSideId::B] {
        for amount in [
            EaseAmount::Formula(id(5_001)),
            EaseAmount::Profile(ProfileParameterRef::new(id(98_010))),
        ] {
            let state = state(amount, side);
            let saved = state.clone();
            let views = state
                .amount_declarations()
                .expect("actual metadata is currently valid");
            assert_eq!(views.len(), 2);
            let source = views
                .first()
                .expect("actual target")
                .portion()
                .expect("retained source")
                .source();
            let structure = state.inputs().structure();
            let context = source
                .admit_notch_context(
                    structure.cut_plan(),
                    structure.pieces().pieces(),
                    state.landmarks(),
                    structure.topology(),
                    limits(1, 2),
                )
                .expect("actual dependencies")
                .validate_sources()
                .expect("actual current source")
                .original_context()
                .expect("actual original coordinates");
            assert_eq!(
                context
                    .admit_explicit(NotchEaseLimits::new(
                        NonZeroU64::new(1).expect("tiny explicit quantity limit")
                    ))
                    .expect_err("coordinates and metadata cannot supply a symbolic numeric value"),
                NotchEaseWorkspaceError::Amount {
                    source_span: id(97_000),
                    side,
                    amount
                }
            );
            assert_eq!(source.declaration().amount, amount);
            assert_eq!(state, saved);
        }
    }
}

fn command_model_limits(
    state: &DesignSewing<DesignSources>,
) -> (
    sc_core::design::DesignResourceLimits,
    sc_core::design::DesignResourceDemand,
) {
    use sc_core::design::{
        charge_design_sewing_resources, DesignResourceLimits, DesignResourceMeter,
    };
    let generous = DesignResourceLimits::new(
        NonZeroUsize::new(1_000_000).expect("finite census"),
        NonZeroUsize::new(1_000_000).expect("finite text census"),
        NonZeroU64::new(1_000).expect("finite counter census"),
    );
    let mut meter = DesignResourceMeter::new(generous);
    charge_design_sewing_resources(state, &mut meter)
        .expect("existing independent complete model meter");
    let demand = meter.finish().expect("complete actual demand");
    (
        DesignResourceLimits::new(
            NonZeroUsize::new(demand.collection_entries().max(1)).expect("exact actual population"),
            NonZeroUsize::new(demand.text_bytes().max(1)).expect("exact actual text"),
            NonZeroU64::new(demand.counter_bits().max(1)).expect("exact actual counter bits"),
        ),
        demand,
    )
}
fn command_capture_limits(items: usize) -> sc_core::command::CommandEaseCaptureLimits {
    sc_core::command::CommandEaseCaptureLimits::new(
        NonZeroUsize::new(items).expect("explicit capture count"),
    )
}
fn command_request(
    state: &DesignSewing<DesignSources>,
    index: usize,
    a_cut: Param,
    b_cut: Param,
) -> sc_core::command::SplitSeamSpan {
    let span = state
        .graph()
        .spans()
        .get(index)
        .expect("actual selected target");
    sc_core::command::SplitSeamSpan::new(
        span.id(),
        EdgeAnchor {
            edge: span.definition().a.range.edge(),
            param: a_cut,
        },
        EdgeAnchor {
            edge: span.definition().b.range.edge(),
            param: b_cut,
        },
    )
}
#[test]
fn command_seam_source_notch_joins_actual_complete_design_capture_and_original_context_quantities()
{
    use sc_core::ontology::NotchEaseLimits;
    use sc_units::Length;
    for side in [SeamSideId::A, SeamSideId::B] {
        for sign in [-1, 1] {
            let parent = Length::from_micrometres(sign).expect("signed literal");
            let state = state(EaseAmount::Explicit(parent), side);
            let saved = state.clone();
            let request = command_request(&state, 0, at(3, 8), at(5, 8));
            let (model_limits, demand) = command_model_limits(&state);
            let source = request
                .admit_sewing_source(&state, model_limits, command_capture_limits(3))
                .expect("complete actual model/current source/target/capture admission");
            assert!(core::ptr::eq(source.state(), &state));
            assert!(core::ptr::eq(source.binding().request(), &request));
            assert!(core::ptr::eq(
                source.binding().span(),
                state.graph().spans().first().expect("target")
            ));
            let metadata = source.amount_declaration();
            assert!(core::ptr::eq(metadata.span(), source.binding().span()));
            assert!(core::ptr::eq(
                metadata.canonical_inputs(),
                state.inputs().inputs()
            ));
            assert_eq!(source.model_demand(), demand);
            assert_eq!(source.model_limits(), model_limits);
            assert_eq!(source.capture_admission().item_count(), 3);
            assert_eq!(source.capture_admission().source_span(), id(97_000));
            let capture = source.capture_admission().capture();
            assert!(core::ptr::eq(
                capture.source(),
                state
                    .graph()
                    .ease_sources()
                    .next()
                    .expect("canonical original")
            ));
            let structure = source.state().inputs().structure();
            let context = capture
                .source()
                .admit_notch_context(
                    structure.cut_plan(),
                    structure.pieces().pieces(),
                    source.state().landmarks(),
                    structure.topology(),
                    limits(1, 2),
                )
                .expect("actual full source dependencies")
                .validate_sources()
                .expect("bounded current original context")
                .original_context()
                .expect("actual original Notches");
            let view = capture
                .notch_explicit_ease(
                    &context,
                    NotchEaseLimits::new(
                        NonZeroU64::new(473).expect("separate exact quantity allowance"),
                    ),
                )
                .expect("actual captured original command quantities");
            assert_eq!(view.span().id(), id(97_001));
            assert_eq!(view.original_source().span(), id(97_000));
            assert_eq!(
                (
                    view.parent_amount(),
                    view.first_amount(),
                    view.second_amount()
                ),
                (parent, Length::ZERO, parent)
            );
            assert_eq!(view.source_parent_amount(), parent);
            assert_eq!(view.original_source().stops().len(), 2);
            assert!(view.span().definition().stops.is_empty());
            assert!(core::ptr::eq(view.original_context(), &context));
            assert!(core::ptr::eq(
                context.validation().admission().ledger(),
                structure.topology()
            ));
            assert_eq!(state, saved);
        }
    }
}
fn command_weighted_state(amount: EaseAmount, side: SeamSideId) -> DesignSewing<DesignSources> {
    use sc_core::ontology::{DeclaredEase, WeightedEaseRegion};
    let state = state(amount, side);
    let source = state
        .graph()
        .ease_sources()
        .next()
        .expect("actual original source");
    let definition = SeamSpanDefinition {
        id: source.span(),
        a: source.side(SeamSideId::A),
        b: source.side(SeamSideId::B),
        direction: source.direction(),
        ease: DeclaredEase {
            amount,
            distribution: EaseDistribution::Weighted {
                side,
                regions: vec![
                    WeightedEaseRegion {
                        from: Param::START,
                        to: at(1, 2),
                        weight: Rational::ONE,
                    },
                    WeightedEaseRegion {
                        from: at(1, 2),
                        to: Param::END,
                        weight: Rational::from_integer(3),
                    },
                ],
            },
        }
        .into(),
        stops: source.stops().to_vec(),
    };
    let structure = state.inputs().structure();
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: state.graph().id(),
            spans: vec![definition],
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        state.landmarks(),
        structure.topology(),
    )
    .expect("actual full weighted source with real Notches");
    DesignSewing::new(DesignSewingDefinition {
        inputs: state.inputs().clone(),
        graph,
        landmarks: state.landmarks().to_vec(),
    })
    .expect("actual owned weighted sewing state")
}
#[test]
fn command_seam_source_weighted_joins_actual_complete_design_capture_and_density_quantities() {
    use sc_core::ontology::WeightedEaseLimits;
    use sc_units::Length;
    for side in [SeamSideId::A, SeamSideId::B] {
        for sign in [-1, 1] {
            let parent = Length::from_micrometres(sign).expect("signed literal");
            let state = command_weighted_state(EaseAmount::Explicit(parent), side);
            let saved = state.clone();
            let request = command_request(&state, 0, at(1, 2), at(2, 3));
            let (model_limits, demand) = command_model_limits(&state);
            let source = request
                .admit_sewing_source(&state, model_limits, command_capture_limits(5))
                .expect("complete actual weighted model source");
            assert_eq!(source.model_demand(), demand);
            assert_eq!(source.capture_admission().item_count(), 5);
            let metadata = source.amount_declaration();
            let DesignSewingAmountSource::Explicit(actual) = metadata.source() else {
                unreachable!("literal")
            };
            assert_eq!(*actual, parent);
            assert!(core::ptr::eq(
                metadata.canonical_inputs(),
                state.inputs().inputs()
            ));
            let capture = source.capture_admission().capture();
            let view = capture
                .weighted_explicit_ease(WeightedEaseLimits::new(
                    NonZeroUsize::new(2).expect("full original regions"),
                    NonZeroU64::new(598).expect("separate exact quantity allowance"),
                ))
                .expect("original weighted command quantities");
            assert!(core::ptr::eq(view.span(), metadata.span()));
            assert_eq!(view.source_parent_amount(), parent);
            assert_eq!(view.parent_amount(), parent);
            assert_eq!(
                view.first_amount(),
                if side == SeamSideId::A {
                    Length::ZERO
                } else {
                    parent
                }
            );
            assert_eq!(
                view.first_amount().checked_add(view.second_amount()),
                Ok(parent)
            );
            assert_eq!(view.weighted_admission().region_count(), 2);
            assert_eq!(state, saved);
        }
    }
}
#[test]
fn command_seam_source_preserves_model_current_target_and_capture_refusal_precedence() {
    use sc_core::command::{
        CommandEaseCaptureCollection, CommandEaseCaptureError, CommandSeamSplitBindingError,
        CommandSeamSplitSourceError as Error, SplitSeamSpan,
    };
    use sc_core::design::{DesignResourceCount, DesignResourceLimitError, DesignResourceLimits};
    use std::error::Error as _;
    let state = state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let request = command_request(&state, 0, at(3, 8), at(5, 8));
    let absent = SplitSeamSpan::new(
        id(99_999),
        request.cut(SeamSideId::A),
        request.cut(SeamSideId::B),
    );
    let (exact, demand) = command_model_limits(&state);
    let less = DesignResourceLimits::new(
        NonZeroUsize::new(demand.collection_entries() - 1).expect("one below"),
        exact.text_bytes(),
        exact.counter_bits(),
    );
    let error = absent
        .admit_sewing_source(&state, less, command_capture_limits(1))
        .expect_err("complete model envelope before missing target or capture");
    assert!(matches!(error, Error::Resources(_)));
    let issue = error
        .source()
        .expect("original resource cause")
        .downcast_ref::<DesignResourceLimitError>()
        .expect("actual resource refusal");
    assert_eq!(
        error
            .source()
            .expect("original cause")
            .downcast_ref::<DesignResourceLimitError>(),
        Some(issue)
    );
    assert!(
        matches!(issue,DesignResourceLimitError::Count {unit:DesignResourceCount::CollectionEntries,
        maximum,consumed,requested:2,location}
        if *maximum==demand.collection_entries()-1 && *consumed==demand.collection_entries()-2
        && location.field=="sewing.landmarks")
    );
    assert!(
        matches!(absent.admit_sewing_source(&state,exact,command_capture_limits(1)).expect_err("actual target"),
        Error::Binding(CommandSeamSplitBindingError::MissingSpan {span}) if span==absent.span())
    );
    let edge = request.cut(SeamSideId::A).edge;
    let wrong = SplitSeamSpan::new(
        request.span(),
        request.cut(SeamSideId::B),
        request.cut(SeamSideId::B),
    );
    assert!(
        matches!(wrong.admit_sewing_source(&state,exact,command_capture_limits(1)).expect_err("actual cut before capture"),
        Error::Binding(CommandSeamSplitBindingError::SourceFrame {span,side:SeamSideId::A,expected,cut:_})
        if span==request.span() && expected==edge)
    );
    let error = request
        .admit_sewing_source(&state, exact, command_capture_limits(2))
        .expect_err("capture complete original stops");
    let expected = CommandEaseCaptureError::Items {
        target: id(97_001),
        source_span: id(97_000),
        collection: CommandEaseCaptureCollection::OriginalStops,
        consumed: 1,
        requested: 2,
        limit: 2,
    };
    assert_eq!(
        error
            .source()
            .expect("capture cause")
            .downcast_ref::<CommandEaseCaptureError>(),
        Some(&expected)
    );
    assert!(matches!(error,Error::Capture(issue) if issue==expected));

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
    inputs.structure = DesignStructure::new(structure).expect("actual current structure");
    let held = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("current inputs"),
        graph: state.graph().clone(),
        landmarks: state.landmarks().to_vec(),
    })
    .expect("held graph remains inspectable");
    let (selected, _) = command_model_limits(&held);
    let error = absent
        .admit_sewing_source(&held, selected, command_capture_limits(1))
        .expect_err("complete current graph before target or capture");
    let original = error
        .source()
        .expect("actual current source chain")
        .downcast_ref::<DesignSewingError<crate::DesignSourcesError>>();
    assert!(matches!(original,Some(DesignSewingError::Graph(issue))
        if matches!(issue.as_ref(),SewingError::MissingCopy {span,side:SeamSideId::A,copy}
        if *span==id(97_001) && *copy==id(96_000))));
    assert!(matches!(error, Error::Current(_)));
    // The source-only binding still exposes the held metadata;the new complete entry refuses it.
    assert!(request.bind_source_cuts(held.graph()).is_ok());
}
#[test]
fn command_seam_source_preserves_actual_formula_profile_metadata_without_supplying_values() {
    use sc_core::command::CommandNotchEaseError;
    use sc_core::ontology::{NotchEaseLimits, NotchEaseWorkspaceError};
    for amount in [
        EaseAmount::Formula(id(5_001)),
        EaseAmount::Profile(ProfileParameterRef::new(id(98_010))),
    ] {
        let state = state(amount, SeamSideId::A);
        let request = command_request(&state, 0, at(3, 8), at(5, 8));
        let (model_limits, _) = command_model_limits(&state);
        let source = request
            .admit_sewing_source(&state, model_limits, command_capture_limits(3))
            .expect("actual current declaration metadata");
        let metadata = source.amount_declaration();
        match (amount, metadata.source()) {
            (
                EaseAmount::Formula(expected),
                DesignSewingAmountSource::Formula {
                    parameter,
                    declaration,
                },
            ) => {
                assert_eq!(parameter.id(), expected);
                assert!(core::ptr::eq(
                    parameter,
                    state
                        .inputs()
                        .parameters()
                        .parameters()
                        .first()
                        .expect("actual metadata")
                ));
                assert_eq!(
                    declaration.source().kind(),
                    sc_core::recipe::FormulaKind::Length
                );
                assert_eq!(metadata.profile_validation(), None);
            }
            (EaseAmount::Profile(expected), DesignSewingAmountSource::Profile(actual)) => {
                assert_eq!(actual, expected);
                assert_eq!(
                    metadata.profile_validation(),
                    Some(ProfileBindingValidation::DeferredToG4)
                );
            }
            _ => unreachable!("actual metadata kind"),
        }
        let capture = source.capture_admission().capture();
        let structure = state.inputs().structure();
        let context = capture
            .source()
            .admit_notch_context(
                structure.cut_plan(),
                structure.pieces().pieces(),
                state.landmarks(),
                structure.topology(),
                limits(1, 2),
            )
            .expect("actual dependencies")
            .validate_sources()
            .expect("actual local context")
            .original_context()
            .expect("original Notches");
        assert!(
            matches!(capture.notch_explicit_ease(&context,NotchEaseLimits::new(
            NonZeroU64::new(1).expect("tiny numeric allowance"))).expect_err("no symbolic value"),
            CommandNotchEaseError::Workspace {target_span,issue:NotchEaseWorkspaceError::Amount {
                source_span,side:SeamSideId::A,amount:actual}}
            if target_span==id(97_001) && source_span==id(97_000) && actual==amount)
        );
    }
}
