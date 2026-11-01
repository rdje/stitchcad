//! Exact selected A+B/result groups,original ordering and no source-copy forks;physical truth pending.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_pairing.rs"]
mod common;
use common::copies::{with_paired_results, with_results};
use common::{admit, base, id, row};
use sc_core::{construction::*, ontology::*};
#[test]
fn exact_selected_and_additional_pairs_preserve_actual_source_result_fields_and_requested_order() {
    with_results(|results| check_complete(&results));
    with_paired_results(|results| check_complete(&results));
}
fn check_complete(results: &DeclaredJoinResultCopies<'_>) {
    let raw = JoinCopySourceManifest::new(id(1000), base(results));
    let source = admit(&raw, results);
    let c = source.copy_count();
    let q = results.copy_count();
    let limits = JoinCopyPairLimits::new(c, q, 3 * q);
    let pairs = source
        .require_pairs(limits)
        .expect("exact selected A+B and one A+B per result");
    assert!(std::ptr::eq(pairs.sources(), &source));
    assert_eq!(pairs.limits(), limits);
    assert_eq!(
        (
            limits.copies(),
            limits.results(),
            limits.workspace_entries()
        ),
        (c, q, 3 * q)
    );
    assert_eq!(
        (
            pairs.pair_count(),
            pairs.consumed_copy_count(),
            pairs.workspace_entries()
        ),
        (q, 2 * q, 3 * q)
    );
    assert!(std::ptr::eq(
        pairs.selected_result().definition(),
        results
            .copy(id(6000))
            .expect("selected pair's actual result request")
            .definition()
    ));
    assert_eq!(
        pairs.pairs().map(|p| p.result().id()).collect::<Vec<_>>(),
        results.copies().map(|r| r.id()).collect::<Vec<_>>()
    );
    assert_eq!(
        pairs
            .pairs()
            .rev()
            .map(|p| p.result().id())
            .collect::<Vec<_>>(),
        results.copies().rev().map(|r| r.id()).collect::<Vec<_>>()
    );
    for pair in pairs.pairs() {
        assert!(std::ptr::eq(pair.context(), &pairs));
        assert!(std::ptr::eq(
            pairs
                .pair(pair.result().id())
                .expect("actual indexed pair")
                .result()
                .definition(),
            pair.result().definition()
        ));
        for side in [SeamSideId::A, SeamSideId::B] {
            let source = pair.source(side);
            assert_eq!(source.side(), Some(side));
            assert!(std::ptr::eq(
                source.source(),
                results
                    .source_copy(source.source().id())
                    .expect("actual canonical source copy")
            ));
            assert!(std::ptr::eq(
                source
                    .result()
                    .expect("actual explicit JoinTo target")
                    .definition(),
                pair.result().definition()
            ));
            let expected_id = match (pair.result().id() == id(6000), side) {
                (true, SeamSideId::A) => 200,
                (true, SeamSideId::B) => 201,
                (false, SeamSideId::A) => 203,
                (false, SeamSideId::B) => 204,
            };
            assert_eq!(source.source().id(), id(expected_id));
            assert_eq!(
                source.source().orientation(),
                if pair.result().id() == id(6000) {
                    CopyOrientation::Authored
                } else {
                    CopyOrientation::Reflected
                }
            );
        }
    }
    assert!(pairs.pair(id(202)).is_none());
    assert!(matches!(
        pairs
            .sources()
            .assignment(id(202))
            .expect("unrelated actual source")
            .intent(),
        JoinCopySourceIntent::RetainOriginal
    ));
}
#[test]
fn whole_copy_result_workspace_quotas_precede_selected_retention_and_keep_actual_selection_loans() {
    with_results(|results| {
        let values = [202, 200, 201]
            .into_iter()
            .map(|s| row(&results, s, None))
            .collect();
        let raw = JoinCopySourceManifest::new(id(1000), values);
        let sources = admit(&raw, &results);
        use JoinCopyPairRefusal as R;
        for (limits, cause) in [
            (
                JoinCopyPairLimits::new(2, 0, 0),
                R::Copies {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinCopyPairLimits::new(3, 0, 0),
                R::Results {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                JoinCopyPairLimits::new(3, 1, 2),
                R::Workspace {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinCopyPairLimits::new(3, 1, 3),
                R::SelectedRetained {
                    side: SeamSideId::A,
                },
            ),
        ] {
            let error = sources
                .require_pairs(limits)
                .expect_err("whole resources before selected consumption");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.sources(), &sources));
            assert!(error.refused_result().is_none());
            if matches!(cause, R::SelectedRetained { .. }) {
                let a = error.refused_source().expect("actual selected retained A");
                assert_eq!(a.source().id(), id(200));
                assert!(std::ptr::eq(
                    a.assignment(),
                    sources
                        .assignment(id(200))
                        .expect("actual A action")
                        .assignment()
                ));
                assert_eq!(
                    error
                        .selected_source(SeamSideId::B)
                        .expect("actual selected B")
                        .source()
                        .id(),
                    id(201)
                );
            } else {
                assert!(error.refused_source().is_none());
                assert!(error.selected_source(SeamSideId::A).is_none());
            }
        }
        let raw = JoinCopySourceManifest::new(
            id(1000),
            vec![
                row(&results, 202, None),
                row(&results, 200, Some(6000)),
                row(&results, 201, None),
            ],
        );
        let sources = admit(&raw, &results);
        let error = sources
            .require_pairs(JoinCopyPairLimits::new(3, 1, 3))
            .expect_err("selected B must contribute");
        assert_eq!(
            error.refusal(),
            R::SelectedRetained {
                side: SeamSideId::B
            }
        );
        assert_eq!(
            error.refused_source().expect("actual B").source().id(),
            id(201)
        );
    });
}
#[test]
fn selected_result_mismatch_precedes_groups_and_duplicate_sides_lend_actual_second_source() {
    with_paired_results(|results| {
        use JoinCopyPairRefusal as R;
        let mut values = base(&results);
        *values.get_mut(2).expect("selected B row") = row(&results, 201, Some(6001));
        let raw = JoinCopySourceManifest::new(id(1000), values);
        let sources = admit(&raw, &results);
        let error = sources
            .require_pairs(JoinCopyPairLimits::new(5, 2, 6))
            .expect_err("actual selected pair cannot target different results");
        assert_eq!(error.refusal(), R::SelectedResultMismatch);
        assert!(error.refused_source().is_none());
        assert!(error.refused_result().is_none());
        for (side, target) in [(SeamSideId::A, 6000), (SeamSideId::B, 6001)] {
            assert!(std::ptr::eq(
                error
                    .selected_source(side)
                    .expect("actual selected source action")
                    .result()
                    .expect("actual authored JoinTo request")
                    .definition(),
                results
                    .copy(id(target))
                    .expect("actual result request")
                    .definition()
            ));
        }
        for (side, changed, retained, first, second) in [
            (SeamSideId::A, 203, 204, 1, 3),
            (SeamSideId::B, 204, 203, 2, 4),
        ] {
            let mut values = base(&results);
            let i = if changed == 203 { 3 } else { 4 };
            let j = if retained == 203 { 3 } else { 4 };
            *values.get_mut(i).expect("actual duplicate contribution") =
                row(&results, changed, Some(6000));
            *values.get_mut(j).expect("other retained copy") = row(&results, retained, None);
            let raw = JoinCopySourceManifest::new(id(1000), values);
            let sources = admit(&raw, &results);
            let error = sources
                .require_pairs(JoinCopyPairLimits::new(5, 2, 6))
                .expect_err("one source of each side per result");
            assert_eq!(
                error.refusal(),
                R::DuplicateSide {
                    result_copy: id(6000),
                    side,
                    first,
                    second
                }
            );
            let actual = error
                .refused_source()
                .expect("actual original second contribution");
            assert_eq!(actual.source().id(), id(changed));
            assert!(std::ptr::eq(
                actual.assignment(),
                raw.assignments()
                    .get(second)
                    .expect("actual original raw row")
            ));
            assert!(std::ptr::eq(
                error
                    .refused_result()
                    .expect("actual result request")
                    .definition(),
                results
                    .copy(id(6000))
                    .expect("actual explicit result")
                    .definition()
            ));
        }
    });
}
#[test]
fn missing_sides_follow_original_result_order_without_fabricated_source_or_pair_prefix() {
    with_paired_results(|results| {
        for (extra_a, extra_b, missing) in [
            (None, None, SeamSideId::A),
            (Some(6001), None, SeamSideId::B),
            (None, Some(6001), SeamSideId::A),
        ] {
            let mut values = base(&results);
            *values.get_mut(3).expect("additional A") = row(&results, 203, extra_a);
            *values.get_mut(4).expect("additional B") = row(&results, 204, extra_b);
            let raw = JoinCopySourceManifest::new(id(1000), values);
            let sources = admit(&raw, &results);
            let error = sources
                .require_pairs(JoinCopyPairLimits::new(5, 2, 6))
                .expect_err("every explicit result needs actual A+B sources");
            assert_eq!(
                error.refusal(),
                JoinCopyPairRefusal::MissingSide {
                    result_copy: id(6001),
                    side: missing
                }
            );
            assert!(error.refused_source().is_none());
            assert!(error.selected_source(SeamSideId::A).is_none());
            assert!(std::ptr::eq(
                error
                    .refused_result()
                    .expect("actual first requested result")
                    .definition(),
                results
                    .manifest()
                    .copies()
                    .first()
                    .expect("original requested order is not numeric id order")
            ));
        }
    });
}
