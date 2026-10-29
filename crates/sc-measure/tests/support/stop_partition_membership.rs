//! Real owned Design stop rows,with independent frozen expected closed memberships.
#[path = "stop_partition_views.rs"]
mod stop_partition_views;
use super::{
    admitted, at, command_request, command_weighted_state, id, ownership_limits, self_seam_state,
    with_ledger, DesignInputs, DesignSewing, DesignSewingDefinition, DesignStructure, EaseAmount,
    EdgeAnchor, SeamSideId, SewingLandmark,
};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    command::{CommandStopMembershipIssue as Issue, CommandStopPairMembership as Membership},
    ontology::{
        CopyOrientation, CutCopyDefinition, CutPlan, DeclaredEase, DeterministicIdGenerator,
        Direction, EaseDistribution, Mirroring, Notch, OriginalPointLimits, Param, Piece,
        PieceInventory, SeamDirection, SeamSpanDefinition, SewingError, SewingGraph,
        SewingGraphDefinition, StopLandmark, TurnPoint,
    },
};
use sc_units::Length;

fn with_graph(
    state: &DesignSewing<sc_measure::DesignSources>,
    definition: SeamSpanDefinition,
    landmarks: Vec<SewingLandmark>,
) -> DesignSewing<sc_measure::DesignSources> {
    let structure = state.inputs().structure();
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: state.graph().id(),
            spans: vec![definition],
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        &landmarks,
        structure.topology(),
    )
    .expect("actual graph authoring contract");
    DesignSewing::new(DesignSewingDefinition {
        inputs: state.inputs().clone(),
        graph,
        landmarks,
    })
    .expect("actual immutable Design owner")
}
fn oracle_state(
    side: SeamSideId,
    direction: SeamDirection,
    param: Param,
    wide: bool,
) -> DesignSewing<sc_measure::DesignSources> {
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), side);
    let mut definition = state
        .graph()
        .spans()
        .first()
        .expect("actual source")
        .definition()
        .clone();
    definition.direction = direction;
    let edge = match side {
        SeamSideId::A => definition.a.range.edge(),
        SeamSideId::B => definition.b.range.edge(),
    };
    let mut ledger = state.inputs().structure().topology().clone();
    let anchor = if wide {
        let mut ids = DeterministicIdGenerator::from_start(110_000);
        let fragment = ledger
            .split(&mut ids, edge, at(1, 3))
            .expect("real later source fragment")
            .first();
        EdgeAnchor {
            edge: fragment,
            param: at(1, i64::MAX),
        }
    } else {
        EdgeAnchor { edge, param }
    };
    let copy = match side {
        SeamSideId::A => definition.a.copy,
        SeamSideId::B => definition.b.copy,
    };
    let piece_id = state
        .inputs()
        .structure()
        .cut_plan()
        .copy(copy)
        .expect("actual side copy")
        .piece();
    let piece = state
        .inputs()
        .structure()
        .pieces()
        .piece(piece_id)
        .expect("actual Piece");
    let SewingLandmark::Notch(template) = state.landmarks().first().expect("actual template")
    else {
        unreachable!("real Notch")
    };
    let mut notch = template.definition().clone();
    notch.anchor = anchor;
    let marks = vec![
        SewingLandmark::Notch(Notch::new(notch, piece, &ledger).expect("real semantic Notch")),
        SewingLandmark::Turn(
            TurnPoint::new(id(98_001), anchor, piece, &ledger).expect("real semantic Turn"),
        ),
    ];
    definition.stops = vec![
        StopLandmark {
            side,
            landmark: id(98_001),
        },
        StopLandmark {
            side,
            landmark: id(98_000),
        },
    ];
    let state = with_ledger(&state, ledger, marks.clone());
    with_graph(&state, definition, marks)
}

#[test]
fn command_stop_memberships_match_all_independent_rows_and_preserve_actual_authored_objects() {
    let mut count = 0;
    let mut shared = 0;
    for row in include_str!("../../../sc-core/tests/fixtures/seam_stop_partition.tsv")
        .lines()
        .filter(|line| !line.starts_with('#'))
    {
        let fields = row.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 9);
        let field = |index| *fields.get(index).expect("frozen field");
        let side = match field(3) {
            "A" => SeamSideId::A,
            "B" => SeamSideId::B,
            _ => unreachable!("frozen side"),
        };
        let direction = match field(4) {
            "Same" => SeamDirection::Same,
            "Opposite" => SeamDirection::Opposite,
            _ => unreachable!("frozen direction"),
        };
        let wide = field(0).starts_with("wide-later");
        let param = if wide {
            Param::START
        } else {
            at(
                field(1).parse().expect("frozen numerator"),
                field(2).parse().expect("frozen denominator"),
            )
        };
        let state = oracle_state(side, direction, param, wide);
        let saved = state.clone();
        let request = command_request(&state, 0, at(1, 2), at(2, 3));
        let inputs = admitted(
            &state,
            &request,
            if wide { 2 } else { 1 },
            2,
            if wide { 3 } else { 2 },
        );
        let ownership = inputs
            .admit_ownership_workspace(ownership_limits(256))
            .expect("explicit fixture ownership cap");
        let context = ownership
            .validate_current_context()
            .expect("actual complete bounded context");
        let memberships = context
            .derive_stop_memberships()
            .expect("all actual closed memberships");
        assert!(core::ptr::eq(memberships.context(), &context));
        assert_eq!(memberships.rows().len(), 2);
        let expected = (field(7) == "1", field(8) == "1");
        shared += usize::from(expected == (true, true));
        for (ordinal, (row, stop)) in memberships
            .rows()
            .iter()
            .zip(&inputs.binding().span().definition().stops)
            .enumerate()
        {
            assert_eq!(row.ordinal(), ordinal);
            assert!(core::ptr::eq(row.stop(), stop));
            assert_eq!(
                (
                    row.membership().includes_first(),
                    row.membership().includes_second()
                ),
                expected
            );
            assert_eq!(row.copy().piece(), row.landmark().landmark().piece());
            assert!(core::ptr::eq(
                row.copy(),
                context.copy(row.copy().id()).expect("exact copy index")
            ));
            let semantic = state
                .landmarks()
                .iter()
                .find(|mark| mark.id() == stop.landmark)
                .expect("actual semantic object");
            assert!(core::ptr::eq(row.landmark().landmark(), semantic));
            assert_eq!(
                row.landmark().current(),
                context
                    .landmark(stop.landmark)
                    .expect("actual cached evidence")
                    .current()
            );
            assert_eq!(row.stop().side, side);
            assert_eq!(
                row.stop().landmark,
                if ordinal == 0 { id(98_001) } else { id(98_000) }
            );
        }
        assert!(matches!(
            memberships
                .rows()
                .first()
                .expect("authored Turn first")
                .landmark()
                .landmark(),
            SewingLandmark::Turn(_)
        ));
        assert!(matches!(
            memberships
                .rows()
                .last()
                .expect("authored Notch second")
                .landmark()
                .landmark(),
            SewingLandmark::Notch(_)
        ));
        assert_eq!(state, saved);
        count += 1;
    }
    assert_eq!((count, shared), (32, 4));
}

#[test]
fn command_stop_memberships_keep_zero_target_rows_separate_from_full_original_stops() {
    let state = super::super::state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let request = command_request(&state, 0, at(3, 8), at(5, 8));
    let inputs = admitted(&state, &request, 1, 2, 2);
    assert_eq!(inputs.stop_count(), 0);
    assert_eq!(
        inputs
            .binding()
            .span()
            .retain_ease_source()
            .source()
            .stops()
            .len(),
        2
    );
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(10))
        .expect("actual admitted ownership");
    let context = ownership
        .validate_current_context()
        .expect("N0 still validates current context");
    let memberships = context
        .derive_stop_memberships()
        .expect("actual empty authored target");
    assert!(memberships.rows().is_empty());
    assert!(core::ptr::eq(memberships.context(), &context));
}

#[test]
fn command_stop_memberships_preserve_original_refusal_and_ordinal_after_valid_prefix() {
    use std::error::Error as _;
    for kind in ["missing", "wrong-piece", "outside-side"] {
        let state = self_seam_state(SeamDirection::Same);
        let mut marks = state.landmarks().to_vec();
        if kind == "missing" {
            marks.retain(|mark| mark.id() != id(98_000));
        } else {
            let structure = state.inputs().structure();
            let piece = if kind == "wrong-piece" {
                structure
                    .pieces()
                    .pieces()
                    .last()
                    .expect("actual other Piece")
            } else {
                structure
                    .pieces()
                    .pieces()
                    .first()
                    .expect("actual same Piece")
            };
            let SewingLandmark::Notch(notch) = marks.first().expect("actual original Notch") else {
                unreachable!("actual Notch")
            };
            let mut definition = notch.definition().clone();
            definition.anchor = EdgeAnchor {
                edge: piece.edges().next().expect("actual Piece edge").edge,
                param: at(3, 4),
            };
            *marks.first_mut().expect("actual semantic replacement") = SewingLandmark::Notch(
                Notch::new(definition, piece, structure.topology())
                    .expect("explicit actual held registry replacement"),
            );
        }
        let held = DesignSewing::new(DesignSewingDefinition {
            inputs: state.inputs().clone(),
            graph: state.graph().clone(),
            landmarks: marks,
        })
        .expect("held actual target stays inspectable");
        let saved = held.clone();
        let request = command_request(&held, 0, at(1, 4), at(3, 4));
        let inputs = admitted(&held, &request, 1, 2, 2);
        let ownership = inputs
            .admit_ownership_workspace(ownership_limits(10))
            .expect("exact actual owner cap");
        let context = ownership
            .validate_current_context()
            .expect("complete registry/target context alone does not prove stop references");
        let error = context
            .derive_stop_memberships()
            .expect_err("no accepted prefix despite valid first B reference");
        let stop = StopLandmark {
            side: SeamSideId::A,
            landmark: id(98_000),
        };
        assert_eq!(
            (error.target_span(), error.ordinal(), error.stop()),
            (id(97_000), 1, stop)
        );
        let expected = if kind == "missing" {
            SewingError::MissingLandmark {
                span: id(97_000),
                stop,
            }
        } else {
            SewingError::StopOutsideSide {
                span: id(97_000),
                stop,
            }
        };
        let Issue::Structural(issue) = error.issue() else {
            unreachable!("actual source refusal")
        };
        assert_eq!(*issue, expected);
        assert!(core::ptr::eq(
            error
                .source()
                .expect("original standard source")
                .downcast_ref::<SewingError>()
                .expect("original concrete SewingError"),
            issue
        ));
        assert_eq!(held, saved);
    }
}

fn two_tags_state(
    direction: SeamDirection,
    reflected: bool,
) -> DesignSewing<sc_measure::DesignSources> {
    let state = if reflected {
        command_weighted_state(EaseAmount::Explicit(Length::ZERO), SeamSideId::A)
    } else {
        self_seam_state(direction)
    };
    let mut definition = state
        .graph()
        .spans()
        .first()
        .expect("actual target")
        .definition()
        .clone();
    definition.direction = direction;
    definition.ease = DeclaredEase {
        amount: EaseAmount::Explicit(Length::ZERO),
        distribution: EaseDistribution::Uniform,
    }
    .into();
    let mut inputs = state.inputs().definition().clone();
    let mut structure = inputs.structure.definition().clone();
    if reflected {
        let mut pieces = structure.pieces.pieces().to_vec();
        let mut piece = pieces.first().expect("actual Piece").definition().clone();
        piece.quantity = 2;
        piece.mirroring = Mirroring::MirroredPairs;
        *pieces.first_mut().expect("actual Piece replacement") =
            Piece::new(piece, &structure.topology).expect("real paired Piece");
        structure.pieces = PieceInventory::new(pieces).expect("actual complete Piece inventory");
        let mut copies = structure
            .cut_plan
            .copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect::<Vec<_>>();
        copies.push(CutCopyDefinition {
            id: id(96_002),
            piece: structure
                .pieces
                .pieces()
                .first()
                .expect("actual paired Piece")
                .id(),
            orientation: CopyOrientation::Reflected,
        });
        structure.cut_plan = CutPlan::new(copies, structure.pieces.pieces())
            .expect("actual complete balanced copy plan");
        definition.b.copy = id(96_002);
        definition.b.range = definition.a.range;
    }
    let piece = structure
        .pieces
        .pieces()
        .first()
        .expect("actual semantic Piece");
    let SewingLandmark::Notch(notch) = state.landmarks().first().expect("actual Notch") else {
        unreachable!("actual semantic Notch")
    };
    let mut mark = notch.definition().clone();
    mark.anchor = EdgeAnchor {
        edge: definition.a.range.edge(),
        param: at(1, 2),
    };
    let marks = vec![SewingLandmark::Notch(
        Notch::new(mark, piece, &structure.topology).expect("real shared boundary mark"),
    )];
    definition.stops = vec![
        StopLandmark {
            side: SeamSideId::B,
            landmark: id(98_000),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(98_000),
        },
    ];
    inputs.structure = DesignStructure::new(structure).expect("actual current structural owner");
    let state = DesignSewing::new(DesignSewingDefinition {
        inputs: DesignInputs::new(inputs).expect("actual inputs"),
        graph: state.graph().clone(),
        landmarks: marks.clone(),
    })
    .expect("actual held component");
    with_graph(&state, definition, marks)
}
#[test]
fn command_stop_memberships_keep_reflected_copies_and_two_tags_of_one_shared_semantic_mark() {
    for reflected in [false, true] {
        for direction in [SeamDirection::Same, SeamDirection::Opposite] {
            let state = two_tags_state(direction, reflected);
            let request = command_request(
                &state,
                0,
                if reflected { at(1, 2) } else { at(1, 4) },
                if reflected { at(2, 3) } else { at(3, 4) },
            );
            let inputs = admitted(&state, &request, 1, 2, 2);
            let ownership = inputs
                .admit_ownership_workspace(ownership_limits(256))
                .expect("explicit fixture allowance");
            let context = ownership
                .validate_current_context()
                .expect("actual copy/target context");
            let memberships = context
                .derive_stop_memberships()
                .expect("preserve both explicit side tags");
            assert_eq!(memberships.rows().len(), 2);
            let b = memberships.rows().first().expect("B authored first");
            let a = memberships.rows().last().expect("A authored second");
            assert_eq!(
                (b.ordinal(), b.stop().side, a.ordinal(), a.stop().side),
                (0, SeamSideId::B, 1, SeamSideId::A)
            );
            assert!(core::ptr::eq(
                b.landmark().landmark(),
                a.landmark().landmark()
            ));
            assert_eq!(
                b.membership(),
                if direction == SeamDirection::Same {
                    Membership::First
                } else {
                    Membership::Second
                }
            );
            assert_eq!(
                a.membership(),
                if reflected {
                    Membership::Both
                } else {
                    Membership::Second
                }
            );
            if reflected {
                assert_ne!(a.copy().id(), b.copy().id());
                assert_eq!(b.copy().orientation(), CopyOrientation::Reflected);
                assert_eq!(a.copy().piece(), b.copy().piece());
            } else {
                assert!(core::ptr::eq(a.copy(), b.copy()));
            }
        }
    }
}

#[test]
fn command_stop_memberships_keep_every_current_boundary_without_selecting_original_preimages() {
    let state = command_weighted_state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let edge = state
        .graph()
        .spans()
        .first()
        .expect("actual source")
        .definition()
        .a
        .range
        .edge();
    let mut ledger = state.inputs().structure().topology().clone();
    let mut ids = DeterministicIdGenerator::from_start(110_000);
    let split = ledger
        .split(&mut ids, edge, at(1, 2))
        .expect("actual source split");
    ledger
        .reverse(&mut ids, split.second())
        .expect("actual reversed second fragment");
    let one = Length::from_micrometres(1).expect("actual positive weights");
    let merged = ledger
        .merge(&mut ids, split.first(), one, split.second(), one)
        .expect("actual noninjective source mapping");
    let piece = state
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual held Piece");
    let marks = state
        .landmarks()
        .iter()
        .zip([at(1, 2), at(1, 4)])
        .map(|(mark, param)| {
            let SewingLandmark::Notch(notch) = mark else {
                unreachable!("actual Notch")
            };
            let mut definition = notch.definition().clone();
            definition.anchor = EdgeAnchor {
                edge: merged,
                param,
            };
            SewingLandmark::Notch(
                Notch::new(definition, piece, &ledger)
                    .expect("real later anchor with known current point"),
            )
        })
        .collect();
    let state = with_ledger(&state, ledger, marks);
    let request = command_request(&state, 0, at(3, 4), at(2, 3));
    let inputs = admitted(&state, &request, 4, 2, 4);
    let ownership = inputs
        .admit_ownership_workspace(ownership_limits(256))
        .expect("explicit fixture workspace");
    let context = ownership
        .validate_current_context()
        .expect("all target points are unique and interval coverage complete");
    let memberships = context
        .derive_stop_memberships()
        .expect("retain every closed CURRENT membership");
    let row = memberships.rows().first().expect("actual authored mark");
    assert_eq!(row.membership(), Membership::Both);
    assert_ne!(
        row.landmark().current().param(),
        context
            .boundary(SeamSideId::A, super::Boundary::Cut)
            .resolved()
            .expect("unique requested cut")
            .param()
    );
    let projected = state
        .inputs()
        .structure()
        .topology()
        .admit_original_point(
            merged,
            at(1, 2),
            edge,
            OriginalPointLimits::new(
                inputs.limits().current(),
                NonZeroUsize::new(2).expect("two actual candidates"),
                NonZeroU64::new(1270).expect("explicit numeric workspace"),
            ),
        )
        .expect("actual inverse admission")
        .project()
        .expect("complete actual inverse evidence");
    assert_eq!(projected.candidates().len(), 2);
    let originals = projected
        .candidates()
        .iter()
        .map(|candidate| {
            (
                candidate
                    .coordinate()
                    .as_param()
                    .expect("these particular originals fit"),
                candidate.direction(),
            )
        })
        .collect::<Vec<_>>();
    assert!(originals.contains(&(at(1, 2), Direction::Original)));
    assert!(originals.contains(&(Param::END, Direction::Reversed)));
    assert_eq!(
        (
            row.landmark().resolution().reference(),
            row.landmark().resolution().param()
        ),
        (merged, at(1, 2))
    );
}
