//! Ordered borrowed row lists and actual owned retained-child Graph construction.
use super::{
    admitted, at, command_request, id, oracle_state, ownership_limits, two_tags_state,
    DesignSewing, DesignSewingDefinition, EaseAmount, Param, SeamDirection, SeamSideId,
    SeamSpanDefinition, SewingGraph, SewingGraphDefinition,
};
use core::num::NonZeroUsize;
use sc_core::command::{CommandEaseCaptureLimits, CommandStopChild as Child};

#[test]
fn command_stop_partitions_match_frozen_ordered_children_and_actual_design_retained_roots() {
    let mut cases = 0;
    let mut shared = 0;
    for text in include_str!("../../../sc-core/tests/fixtures/seam_stop_partition.tsv")
        .lines()
        .filter(|line| !line.starts_with('#'))
    {
        let fields = text.split('\t').collect::<Vec<_>>();
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
            .expect("explicit fixture cap");
        let context = ownership
            .validate_current_context()
            .expect("actual complete bounded context");
        let memberships = context
            .derive_stop_memberships()
            .expect("complete actual authored rows");
        let partitions = memberships
            .partition_stops()
            .expect("two bounded child reference lists");
        assert!(core::ptr::eq(partitions.memberships(), &memberships));
        let expected = [field(7) == "1", field(8) == "1"];
        let expected_count = expected.into_iter().filter(|belongs| *belongs).count() * 2;
        assert_eq!(partitions.reference_count(), expected_count);
        assert!(partitions.reference_count() <= inputs.child_reference_item_bound());
        shared += usize::from(expected == [true, true]);
        for ((child, pair), belongs) in [Child::First, Child::Second]
            .into_iter()
            .zip(inputs.binding().paired_intervals())
            .zip(expected)
        {
            let view = partitions.child(child);
            assert_eq!(view.child(), child);
            assert_eq!(view.pair(), pair);
            assert_eq!(view.rows().len(), if belongs { 2 } else { 0 });
            for (row, original) in view.rows().iter().zip(memberships.rows()) {
                assert!(core::ptr::eq(*row, original));
                assert!(core::ptr::eq(row.stop(), original.stop()));
                assert!(core::ptr::eq(
                    row.landmark().landmark(),
                    original.landmark().landmark()
                ));
                assert_eq!(row.ordinal(), original.ordinal());
            }
        }
        if expected == [true, true] {
            assert_eq!(
                partitions.reference_count(),
                inputs.child_reference_item_bound()
            );
            for (a, b) in partitions
                .child(Child::First)
                .rows()
                .iter()
                .zip(partitions.child(Child::Second).rows())
            {
                assert!(core::ptr::eq(*a, *b));
            }
        }

        // Actual source admission/capture and Graph constructors independently consume the views.
        // The sealed cross-token identity join is the next production slice,not implied here.
        let (model, _) = super::super::command_model_limits(&state);
        let source = request
            .admit_sewing_source(
                &state,
                model,
                CommandEaseCaptureLimits::new(
                    NonZeroUsize::new(5).expect("full root/two regions/two original stops"),
                ),
            )
            .expect("real actual Design source admission");
        let capture = source.capture_admission().capture();
        let portions = capture
            .portion()
            .split_at(request.cut(SeamSideId::A), request.cut(SeamSideId::B))
            .expect("actual original source portions");
        let definitions = portions
            .into_iter()
            .zip([Child::First, Child::Second])
            .zip([id(99_001), id(99_002)])
            .map(|((portion, child), id)| {
                let view = partitions.child(child);
                assert_eq!(view.pair().side(SeamSideId::A), portion.side(SeamSideId::A));
                assert_eq!(view.pair().side(SeamSideId::B), portion.side(SeamSideId::B));
                SeamSpanDefinition {
                    id,
                    a: portion.side(SeamSideId::A),
                    b: portion.side(SeamSideId::B),
                    direction: portion.source().direction(),
                    ease: portion.into(),
                    stops: view.rows().iter().map(|row| *row.stop()).collect(),
                }
            })
            .collect();
        let structure = state.inputs().structure();
        let graph = SewingGraph::new(
            SewingGraphDefinition {
                id: state.graph().id(),
                spans: definitions,
            },
            structure.cut_plan(),
            structure.pieces().pieces(),
            state.landmarks(),
            structure.topology(),
        )
        .expect("actual retained child graph accepts exact closed stop lists");
        assert_eq!(graph.ease_sources().count(), 1);
        assert!(core::ptr::eq(
            graph
                .ease_sources()
                .next()
                .expect("one actual original source"),
            capture.source()
        ));
        for (span, child) in graph.spans().iter().zip([Child::First, Child::Second]) {
            assert_eq!(
                span.definition().stops,
                partitions
                    .child(child)
                    .rows()
                    .iter()
                    .map(|row| *row.stop())
                    .collect::<Vec<_>>()
            );
        }
        let candidate = DesignSewing::new(DesignSewingDefinition {
            inputs: state.inputs().clone(),
            graph,
            landmarks: state.landmarks().to_vec(),
        })
        .expect("real immutable candidate owner");
        assert_eq!(
            candidate
                .amount_declarations()
                .expect("actual candidate current metadata preflight")
                .len(),
            2
        );
        assert_eq!(candidate.landmarks(), state.landmarks());
        assert_eq!(state, saved);
        cases += 1;
    }
    assert_eq!((cases, shared), (32, 4));
}

#[test]
fn command_stop_partitions_keep_shared_semantic_side_tags_and_actual_copy_identity() {
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
                .expect("explicit actual fixture budget");
            let context = ownership
                .validate_current_context()
                .expect("actual current metadata");
            let memberships = context
                .derive_stop_memberships()
                .expect("actual authored B/A rows");
            let partitions = memberships
                .partition_stops()
                .expect("both ordered row-reference lists");
            assert_eq!(partitions.reference_count(), if reflected { 3 } else { 2 });
            for child in [Child::First, Child::Second] {
                let rows = partitions.child(child).rows();
                assert!(rows.windows(2).all(|pair| pair
                    .first()
                    .expect("earlier authored row")
                    .ordinal()
                    < pair.last().expect("later authored row").ordinal()));
                for row in rows {
                    assert!(core::ptr::eq(
                        *row,
                        memberships
                            .rows()
                            .get(row.ordinal())
                            .expect("actual original membership row")
                    ));
                    let pair = partitions.child(child).pair();
                    assert_eq!(pair.side(row.stop().side).copy, row.copy().id());
                }
            }
        }
    }
}

#[test]
fn command_stop_partitions_keep_empty_target_views_without_original_root_backfill() {
    let state = super::super::super::state(EaseAmount::Formula(id(5_001)), SeamSideId::A);
    let request = command_request(&state, 0, at(3, 8), at(5, 8));
    let inputs = admitted(&state, &request, 1, 2, 2);
    assert_eq!(inputs.child_reference_item_bound(), 0);
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
        .expect("exact current ownership allowance");
    let context = ownership
        .validate_current_context()
        .expect("actual N0 current context");
    let memberships = context
        .derive_stop_memberships()
        .expect("actual N0 memberships");
    let partitions = memberships
        .partition_stops()
        .expect("no implicit child allocation/default references");
    assert_eq!(partitions.reference_count(), 0);
    for child in [Child::First, Child::Second] {
        assert!(partitions.child(child).rows().is_empty());
    }
}
