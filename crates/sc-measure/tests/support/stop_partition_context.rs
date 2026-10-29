//! Actual bounded local current context over real owned cross-crate Design inputs.
#[path = "stop_partition_membership.rs"]
mod stop_partition_membership;
use super::{
    at, command_model_limits, command_request, command_stop_limits, command_weighted_state,
    expanded_ownership_state, id, ownership_limits, DesignInputs, DesignSewing,
    DesignSewingDefinition, DesignStructure, EaseAmount, EdgeAnchor, SeamSideId, SewingLandmark,
};
use sc_core::{
    command::{
        CommandStopContextBoundary as Boundary, CommandStopContextIssue as Issue,
        CommandStopContextRange as Location, CommandStopPartitionAdmission, SplitSeamSpan,
    },
    ontology::{
        DeclaredEase, DeterministicIdGenerator, DirectedEdge, Direction, EaseDistribution,
        EdgeRange, IdentityLedger, Notch, OffsetInterval, OrphaningEdit, Param, Piece,
        PieceInventory, RangeCoverageIssue, RangeIssue, RangePortion, SeamDirection, SewingError,
        SewingGraph, SewingGraphDefinition, StopLandmark, TurnPoint, UniquePointOutcome,
    },
};
use sc_measure::DesignSources;
use sc_units::Length;

fn with_ledger(
    state: &DesignSewing<DesignSources>,
    ledger: IdentityLedger,
    landmarks: Vec<SewingLandmark>,
) -> DesignSewing<DesignSources> {
    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.topology = ledger;
    inputs.structure = DesignStructure::new(structure).expect("actual current structural owner");
    DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual current inputs"),
        graph: state.graph().clone(),
        landmarks,
    })
    .expect("held graph and actual current inputs remain inspectable")
}

fn admitted<'a>(
    state: &'a DesignSewing<DesignSources>,
    request: &'a SplitSeamSpan,
    journal: usize,
    nested: usize,
    peak: usize,
) -> CommandStopPartitionAdmission<'a, DesignSources> {
    let (model, _) = command_model_limits(state);
    request
        .admit_stop_partition_inputs(
            state,
            model,
            command_stop_limits(128, journal, nested, peak),
        )
        .expect("explicit fixture input workspace and complete model envelope")
}
#[test]
fn command_stop_context_uses_actual_complete_registry_piece_and_paired_range_evidence() {
    for side in [SeamSideId::A, SeamSideId::B] {
        let state = command_weighted_state(EaseAmount::Formula(id(5_001)), side);
        let saved = state.clone();
        let request = command_request(&state, 0, at(1, 2), at(2, 3));
        let inputs = admitted(&state, &request, 1, 2, 2);
        let ownership = inputs
            .admit_ownership_workspace(ownership_limits(10))
            .expect("exact actual owner workspace");
        let context = ownership
            .validate_current_context()
            .expect("actual bounded local current context");
        assert!(core::ptr::eq(context.ownership(), &ownership));
        assert_eq!(
            (
                context.copy_count(),
                context.piece_count(),
                context.landmark_count(),
                context.owned_item_count()
            ),
            (2, 2, 2, 2)
        );
        for copy in state.inputs().structure().cut_plan().copies() {
            assert!(core::ptr::eq(
                context.copy(copy.id()).expect("complete copy index"),
                copy
            ));
        }
        for piece in state.inputs().structure().pieces().pieces() {
            assert!(core::ptr::eq(
                context.piece(piece.id()).expect("actual Piece"),
                piece
            ));
            let rows = context
                .piece_portions(piece.id())
                .expect("complete Piece interval rows");
            assert_eq!(rows.len(), 1);
            let row = rows.first().expect("one original edge");
            assert_eq!((row.source_edge_index(), row.portion_index()), (0, 0));
            assert!(
                matches!(row.portion(),RangePortion::Resolved(part) if part.range()==row.held())
            );
        }
        for mark in state.landmarks() {
            let evidence = context
                .landmark(mark.id())
                .expect("all actual semantic entries");
            assert!(core::ptr::eq(evidence.landmark(), mark));
            assert_eq!(evidence.resolution().resolved(), Some(evidence.current()));
        }
        let [first, second] = inputs.binding().paired_intervals();
        for side in [SeamSideId::A, SeamSideId::B] {
            for (coverage, pair) in [
                (context.first_pair_coverage(side), first),
                (context.second_pair_coverage(side), second),
            ] {
                assert_eq!(coverage.held(), pair.side(side).range);
                assert!(coverage.has_full_coverage());
                assert_eq!(coverage.portions().len(), 1);
            }
            let at = context.boundary(side, Boundary::Cut);
            assert_eq!(
                (at.reference(), at.param()),
                (request.cut(side).edge, request.cut(side).param)
            );
            assert!(at.resolved().is_some());
        }
        assert_eq!(state, saved);
    }
}
#[test]
fn command_stop_context_validates_unused_turn_on_unused_piece_and_preserves_actual_objects() {
    let state = expanded_ownership_state();
    let structure = state.inputs().structure();
    let piece = structure
        .pieces()
        .piece(id(120_000))
        .expect("actual unused Piece");
    let edge = piece.edges().next().expect("actual held boundary").edge;
    let turn = TurnPoint::new(
        id(120_020),
        EdgeAnchor {
            edge,
            param: at(1, 2),
        },
        piece,
        structure.topology(),
    )
    .expect("actual unused semantic Turn");
    let mut landmarks = state.landmarks().to_vec();
    landmarks.push(SewingLandmark::Turn(turn));
    let state = DesignSewing::new(DesignSewingDefinition {
        inputs: state.inputs().clone(),
        graph: state.graph().clone(),
        landmarks,
    })
    .expect("actual complete owned registry");
    let saved = state.clone();
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let inputs = admitted(&state, &request, 2, 5, 2);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(18))
        .expect("all unused schema data admitted");
    let context = ownership
        .validate_current_context()
        .expect("unused Turn still validates against actual unused Piece");
    assert_eq!(
        (
            context.copy_count(),
            context.piece_count(),
            context.landmark_count(),
            context.owned_item_count()
        ),
        (3, 3, 3, 5)
    );
    let actual = state
        .landmarks()
        .last()
        .expect("actual authored unused mark");
    let evidence = context
        .landmark(actual.id())
        .expect("complete current registry");
    assert!(core::ptr::eq(evidence.landmark(), actual));
    assert_eq!(
        (evidence.current().edge(), evidence.current().param()),
        (edge, at(1, 2))
    );
    assert_eq!(state, saved);
}
fn self_seam_state(direction: SeamDirection) -> DesignSewing<DesignSources> {
    let state = command_weighted_state(EaseAmount::Explicit(Length::ZERO), SeamSideId::A);
    let mut definition = state
        .graph()
        .spans()
        .first()
        .expect("actual source")
        .definition()
        .clone();
    let edge = definition.a.range.edge();
    definition.a.range = EdgeRange::new(edge, Param::START, at(1, 2)).expect("A half");
    definition.b.copy = definition.a.copy;
    definition.b.range = EdgeRange::new(edge, at(1, 2), Param::END).expect("B half");
    definition.direction = direction;
    definition.ease = DeclaredEase {
        amount: EaseAmount::Explicit(Length::ZERO),
        distribution: EaseDistribution::Uniform,
    }
    .into();
    definition.stops = vec![
        StopLandmark {
            side: SeamSideId::B,
            landmark: id(98_001),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(98_000),
        },
    ];
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
    .expect("actual touching disjoint self seam");
    DesignSewing::new(DesignSewingDefinition {
        inputs: state.inputs().clone(),
        graph,
        landmarks: state.landmarks().to_vec(),
    })
    .expect("actual immutable self-seam input owner")
}
#[test]
fn command_stop_context_preserves_touching_same_copy_and_directional_pair_order() {
    for direction in [SeamDirection::Same, SeamDirection::Opposite] {
        let state = self_seam_state(direction);
        let request = command_request(&state, 0, at(1, 4), at(3, 4));
        let inputs = admitted(&state, &request, 1, 2, 2);
        let ownership = inputs
            .admit_ownership_workspace(ownership_limits(10))
            .expect("actual complete workspace");
        let context = ownership
            .validate_current_context()
            .expect("no positive self-overlap at a shared endpoint");
        assert_eq!(
            inputs.binding().span().definition().a.copy,
            inputs.binding().span().definition().b.copy
        );
        let [first, second] = inputs.binding().paired_intervals();
        assert_eq!(
            context.first_pair_coverage(SeamSideId::B).held(),
            first.side(SeamSideId::B).range
        );
        assert_eq!(
            context.second_pair_coverage(SeamSideId::B).held(),
            second.side(SeamSideId::B).range
        );
        assert_eq!(
            context
                .boundary(SeamSideId::A, Boundary::End)
                .resolved()
                .expect("A upper")
                .param(),
            at(1, 2)
        );
        assert_eq!(
            context
                .boundary(SeamSideId::B, Boundary::Start)
                .resolved()
                .expect("B lower")
                .param(),
            at(1, 2)
        );
    }
}
#[test]
fn command_stop_context_keeps_range_quota_and_missing_copy_even_for_zero_target_stops() {
    use sc_core::ontology::RangeCoverageError;
    use std::error::Error as _;
    let state = command_weighted_state(EaseAmount::Explicit(Length::ZERO), SeamSideId::A);
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let inputs = admitted(&state, &request, 1, 2, 1);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(7))
        .expect("small selected range limit still has valid metadata bound");
    let error = ownership
        .validate_current_context()
        .expect_err("identity interval fold needs peak2");
    assert_eq!(error.target_span(), id(97_000));
    let Issue::CoverageLimit {
        reference: Location::PieceEdge { piece, index: 0 },
        issue,
    } = error.issue()
    else {
        unreachable!("actual first Piece coverage limit")
    };
    assert_eq!(
        *piece,
        state
            .inputs()
            .structure()
            .pieces()
            .pieces()
            .first()
            .expect("actual first Piece")
            .id()
    );
    assert!(matches!(
        issue.issue(),
        RangeCoverageIssue::Portions {
            retained: 1,
            requested: 1,
            limit: 1,
            operation: _
        }
    ));
    assert!(core::ptr::eq(
        error
            .source()
            .expect("original range error")
            .downcast_ref::<RangeCoverageError>()
            .expect("concrete original query refusal"),
        issue
    ));

    let state = super::state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.cut_plan = super::CutPlan::new(
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
    .expect("actual copy replacement");
    inputs.structure = DesignStructure::new(structure).expect("actual current structural root");
    let held = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual inputs"),
        graph: state.graph().clone(),
        landmarks: state.landmarks().to_vec(),
    })
    .expect("held source stays inspectable");
    let request = command_request(&held, 0, at(3, 8), at(5, 8));
    let inputs = admitted(&held, &request, 1, 2, 1);
    assert_eq!(inputs.stop_count(), 0);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(7))
        .expect("input metadata is not fresh current proof");
    let error = ownership
        .validate_current_context()
        .expect_err("named copy refuses before any range query,even N0");
    assert!(
        matches!(error.issue(),Issue::Structural(SewingError::MissingCopy {span,side:SeamSideId::A,copy})
        if *span==id(97_001)&&*copy==id(96_000))
    );
}

#[test]
fn command_stop_context_preserves_cut_and_endpoint_choices_after_actual_rejoin() {
    for retained in [false, true] {
        for rejoin in [false, true] {
            let state = if retained {
                super::state(EaseAmount::Formula(id(5_001)), SeamSideId::A)
            } else {
                command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A)
            };
            let edge = state
                .graph()
                .spans()
                .first()
                .expect("actual target")
                .definition()
                .a
                .range
                .edge();
            let mut ledger = state.inputs().structure().topology().clone();
            let mut ids = DeterministicIdGenerator::from_start(110_000);
            let split = ledger
                .split(&mut ids, edge, at(1, 2))
                .expect("actual split");
            let operation = ledger
                .journal()
                .last()
                .expect("actual split entry")
                .operation();
            if rejoin {
                let one = Length::from_micrometres(1).expect("positive actual merge weights");
                ledger
                    .merge(&mut ids, split.first(), one, split.second(), one)
                    .expect("actual rejoin");
            }
            let state = with_ledger(&state, ledger, state.landmarks().to_vec());
            let (a_cut, b_cut) = if retained {
                (at(3, 8), at(5, 8))
            } else {
                (at(1, 2), at(2, 3))
            };
            let request = command_request(&state, 0, a_cut, b_cut);
            let inputs = admitted(&state, &request, if rejoin { 3 } else { 2 }, 2, 3);
            assert_eq!(inputs.stop_count(), if retained { 0 } else { 2 });
            let ownership = inputs
                .admit_ownership_workspace(ownership_limits(256))
                .expect("explicit fixture cap");
            let error = ownership
                .validate_current_context()
                .expect_err("first held split choice survives a later rejoin,even N0");
            let Issue::Boundary {
                side: SeamSideId::A,
                boundary,
                resolution,
            } = error.issue()
            else {
                unreachable!("actual held target point choice: {error:?}")
            };
            assert_eq!(
                *boundary,
                if retained {
                    Boundary::End
                } else {
                    Boundary::Cut
                }
            );
            assert_eq!(
                (resolution.reference(), resolution.param()),
                (edge, at(1, 2))
            );
            assert!(
                matches!(resolution.outcome(),UniquePointOutcome::SplitChoice {operation: actual, source,..} if *actual==operation && *source==edge)
            );
        }
    }
}

#[test]
fn command_stop_context_retains_actual_lost_interior_despite_surviving_boundaries_and_marks() {
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let edge = state
        .graph()
        .spans()
        .first()
        .expect("actual target")
        .definition()
        .a
        .range
        .edge();
    let mut ledger = state.inputs().structure().topology().clone();
    let mut ids = DeterministicIdGenerator::from_start(110_000);
    ledger
        .offset(
            &mut ids,
            edge,
            &[
                OffsetInterval::new(Param::START, at(2, 5)).expect("lower survivor"),
                OffsetInterval::new(at(3, 5), Param::END).expect("upper survivor"),
            ],
        )
        .expect("actual lost interior");
    let state = with_ledger(&state, ledger, state.landmarks().to_vec());
    let saved = state.clone();
    let request = command_request(&state, 0, at(1, 4), at(2, 3));
    let inputs = admitted(&state, &request, 2, 4, 4);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(256))
        .expect("explicit fixture workspace");
    let error = ownership
        .validate_current_context()
        .expect_err("full interval cannot be inferred from surviving endpoints or cut");
    let Issue::Coverage {
        reference: Location::TargetSide(SeamSideId::A),
        evidence,
    } = error.issue()
    else {
        unreachable!("complete actual target interval: {error:?}")
    };
    assert_eq!(evidence.held(), EdgeRange::whole(edge));
    assert_eq!(evidence.portions().len(), 3);
    assert!(!evidence.has_full_coverage());
    assert_eq!(
        evidence
            .portions()
            .iter()
            .filter(|part| matches!(part, RangePortion::Resolved(_)))
            .count(),
        2
    );
    assert!(evidence.portions().iter().any(|part| matches!(part,RangePortion::Unresolved(repair) if matches!(repair.issue(),RangeIssue::Orphaned(OrphaningEdit::OffsetUnmapped {..})))));
    assert_eq!(state, saved);
}

#[test]
fn command_stop_context_validates_unused_numeric_turn_and_retains_original_numeric_source() {
    use std::error::Error as _;
    let state = expanded_ownership_state();
    let structure = state.inputs().structure();
    let piece = structure
        .pieces()
        .pieces()
        .first()
        .expect("actual complete Piece");
    let edge = piece
        .definition()
        .construction_lines
        .first()
        .expect("actual non-target edge")
        .edge;
    let turn = TurnPoint::new(
        id(120_020),
        EdgeAnchor {
            edge,
            param: at(1, i64::MAX),
        },
        piece,
        structure.topology(),
    )
    .expect("real held unused Turn before edit");
    let mut landmarks = state.landmarks().to_vec();
    landmarks.push(SewingLandmark::Turn(turn));
    let mut ledger = structure.topology().clone();
    let mut ids = DeterministicIdGenerator::from_start(130_000);
    ledger
        .split(&mut ids, edge, at(2, 3))
        .expect("actual construction-edge split");
    let state = with_ledger(&state, ledger, landmarks);
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let inputs = admitted(&state, &request, 3, 5, 3);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(256))
        .expect("explicit actual fixture workspace");
    let error = ownership
        .validate_current_context()
        .expect_err("unused registry numeric failure cannot be filtered out");
    assert!(
        matches!(error.issue(),Issue::LandmarkPoint {landmark,resolution} if *landmark==id(120_020) && resolution.reference()==edge && resolution.param()==at(1,i64::MAX))
    );
    assert_eq!(
        error
            .source()
            .expect("original numeric source")
            .downcast_ref::<sc_units::UnitError>(),
        Some(&sc_units::UnitError::Overflow {
            operation: "Rational::checked_div"
        })
    );
}

#[test]
fn command_stop_context_retains_numeric_internal_range_cause_with_all_six_points_unique() {
    use std::error::Error as _;
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let edge = state
        .graph()
        .spans()
        .first()
        .expect("actual target")
        .definition()
        .a
        .range
        .edge();
    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let mut ids = DeterministicIdGenerator::from_start(110_000);
    let split = structure
        .topology
        .split(&mut ids, edge, at(1, 2))
        .expect("actual split");
    let one = Length::from_micrometres(1).expect("actual positive lengths");
    let merged = structure
        .topology
        .merge(&mut ids, split.first(), one, split.second(), one)
        .expect("actual merge");
    let low = at(1, i64::MAX);
    let fragments = structure
        .topology
        .offset(
            &mut ids,
            merged,
            &[
                OffsetInterval::new(Param::START, low).expect("tiny first survivor"),
                OffsetInterval::new(low, Param::END).expect("remaining survivor"),
            ],
        )
        .expect("actual wide-ratio offset");
    let mut pieces = structure.pieces.pieces().to_vec();
    let mut definition = pieces.first().expect("actual Piece").definition().clone();
    definition.boundary = fragments
        .iter()
        .map(|edge| DirectedEdge {
            edge: *edge,
            direction: Direction::Original,
        })
        .collect();
    let piece =
        Piece::new(definition, &structure.topology).expect("actual current Piece boundaries");
    let landmarks = state
        .landmarks()
        .iter()
        .zip([at(1, 4), at(3, 4)])
        .map(|(mark, param)| {
            let SewingLandmark::Notch(notch) = mark else {
                unreachable!("actual Notch fixture")
            };
            let mut definition = notch.definition().clone();
            definition.anchor = EdgeAnchor {
                edge: *fragments.first().expect("actual current tiny fragment"),
                param,
            };
            SewingLandmark::Notch(
                Notch::new(definition, &piece, &structure.topology)
                    .expect("actual current semantic anchor"),
            )
        })
        .collect();
    *pieces.first_mut().expect("actual Piece replacement") = piece;
    structure.pieces = PieceInventory::new(pieces).expect("actual immutable Piece inventory");
    inputs.structure = DesignStructure::new(structure).expect("actual current structural owner");
    let state = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual inputs"),
        graph: state.graph().clone(),
        landmarks,
    })
    .expect("held actual source with current Pieces/marks");
    let request = command_request(&state, 0, at(i64::MAX / 2 + 1, i64::MAX), at(2, 3));
    let inputs = admitted(&state, &request, 4, 4, 6);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(256))
        .expect("explicit fixture workspace");
    for side in [SeamSideId::A, SeamSideId::B] {
        let definition = inputs.binding().span().definition();
        let range = match side {
            SeamSideId::A => definition.a.range,
            SeamSideId::B => definition.b.range,
        };
        for param in [range.from(), request.cut(side).param, range.to()] {
            assert!(state
                .inputs()
                .structure()
                .topology()
                .resolve_unique_point(range.edge(), param, inputs.limits().current())
                .resolved()
                .is_some());
        }
    }
    let error = ownership
        .validate_current_context()
        .expect_err("internal range arithmetic still genuinely exceeds the final stored field");
    let Issue::Coverage {
        reference: Location::TargetSide(SeamSideId::A),
        evidence,
    } = error.issue()
    else {
        unreachable!("actual internal interval refusal: {error:?}")
    };
    assert_eq!(evidence.held(), EdgeRange::whole(edge));
    assert!(!evidence.has_full_coverage());
    assert_eq!(
        error
            .source()
            .expect("original internal numeric cause")
            .downcast_ref::<sc_units::UnitError>(),
        Some(&sc_units::UnitError::Overflow {
            operation: "Rational::checked_relative_progress"
        })
    );
}

#[test]
fn command_stop_context_retains_unrelated_piece_repairs_without_claiming_whole_piece_health() {
    let state = expanded_ownership_state();
    let piece = state
        .inputs()
        .structure()
        .pieces()
        .piece(id(120_000))
        .expect("actual unused Piece");
    let edge = piece.edges().next().expect("actual held unused edge").edge;
    let mut ledger = state.inputs().structure().topology().clone();
    let mut ids = DeterministicIdGenerator::from_start(130_000);
    ledger
        .offset(
            &mut ids,
            edge,
            &[
                OffsetInterval::new(Param::START, at(2, 5)).expect("lower survivor"),
                OffsetInterval::new(at(3, 5), Param::END).expect("upper survivor"),
            ],
        )
        .expect("actual unrelated lost interior");
    let state = with_ledger(&state, ledger, state.landmarks().to_vec());
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let inputs = admitted(&state, &request, 3, 7, 4);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(256))
        .expect("explicit fixture allowance");
    let context = ownership
        .validate_current_context()
        .expect("local target and complete actual registry remain valid");
    let parts = context
        .piece_portions(id(120_000))
        .expect("complete unused Piece evidence");
    assert_eq!((context.owned_item_count(), parts.len()), (7, 3));
    for part in parts {
        assert_eq!(
            (part.held(), part.source_edge_index()),
            (EdgeRange::whole(edge), 0)
        );
    }
    let lost = parts
        .iter()
        .find(|part| matches!(part.portion(), RangePortion::Unresolved(_)))
        .expect("repair is retained rather than filtered");
    assert_eq!(lost.portion_index(), 1);
    assert!(
        matches!(lost.portion(),RangePortion::Unresolved(repair) if matches!(repair.issue(),RangeIssue::Orphaned(OrphaningEdit::OffsetUnmapped {source,..}) if source==edge))
    );
    assert_eq!(
        parts
            .iter()
            .filter(|part| matches!(part.portion(), RangePortion::Resolved(_)))
            .count(),
        2
    );
}

#[test]
fn command_stop_context_accepts_real_later_anchors_without_narrowing_their_original_positions() {
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let edge = state
        .graph()
        .spans()
        .first()
        .expect("actual target")
        .definition()
        .a
        .range
        .edge();
    let mut ledger = state.inputs().structure().topology().clone();
    let mut ids = DeterministicIdGenerator::from_start(110_000);
    let split = ledger
        .split(&mut ids, edge, at(1, 3))
        .expect("actual topology split distinct from command cuts");
    let piece = state
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual Piece with held whole edge");
    let landmarks = state
        .landmarks()
        .iter()
        .zip([at(1, i64::MAX), at(1, 2)])
        .map(|(mark, param)| {
            let SewingLandmark::Notch(notch) = mark else {
                unreachable!("real semantic Notch")
            };
            let mut definition = notch.definition().clone();
            definition.anchor = EdgeAnchor {
                edge: split.first(),
                param,
            };
            SewingLandmark::Notch(
                Notch::new(definition, piece, &ledger).expect("real later-frame anchor"),
            )
        })
        .collect();
    let state = with_ledger(&state, ledger, landmarks);
    let request = command_request(&state, 0, at(1, 2), at(2, 3));
    let inputs = admitted(&state, &request, 2, 2, 3);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(256))
        .expect("explicit actual fixture workspace");
    let context = ownership
        .validate_current_context()
        .expect("current point does not require a stored original Param");
    let mark = context
        .landmark(id(98_000))
        .expect("actual current semantic evidence");
    assert!(core::ptr::eq(
        mark.landmark(),
        state
            .landmarks()
            .first()
            .expect("actual original registry object")
    ));
    assert_eq!(
        (mark.current().edge(), mark.current().param()),
        (split.first(), at(1, i64::MAX))
    );
    assert_eq!(
        (mark.resolution().reference(), mark.resolution().param()),
        (split.first(), at(1, i64::MAX))
    );
    assert_eq!(
        context.first_pair_coverage(SeamSideId::A).portions().len(),
        2
    );
}
