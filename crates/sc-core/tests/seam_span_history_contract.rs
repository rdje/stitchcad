//! Local identity history contracts, independent of graph/current/command acceptance.
#![allow(clippy::expect_used)]
use core::num::NonZeroUsize;
use sc_core::ontology::{
    EntityId, SeamSpanHistory, SeamSpanHistoryEntry as Entry, SeamSpanHistoryError as Error,
    SeamSpanHistoryLimits as Limits, SeamSpanHistoryState as State,
};

fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
fn limits(entries: usize, identities: usize) -> Limits {
    Limits::new(
        NonZeroUsize::new(entries).expect("positive selected entry allowance"),
        NonZeroUsize::new(identities).expect("positive selected identity allowance"),
    )
}
fn nested() -> Vec<Entry> {
    vec![
        Entry::Root(id(8)),
        Entry::Root(id(0)),
        Entry::Split {
            parent: id(8),
            children: [id(3), id(5)],
        },
        Entry::Split {
            parent: id(3),
            children: [id(1), id(2)],
        },
    ]
}

#[test]
fn nested_retirement_keeps_immediate_ordered_children_and_actual_record_borrows() {
    let entries = nested();
    let original_pointer = entries.as_ptr();
    let history = SeamSpanHistory::new(entries, limits(4, 6)).expect("valid complete history");
    assert_eq!(history.entries(), nested());
    assert_eq!(history.entries().as_ptr(), original_pointer);
    assert_eq!(history.identity_count(), 6);
    for (parent, birth, split, expected) in [(8, 0, 2, [3, 5]), (3, 2, 3, [1, 2])] {
        let state = history.state(id(parent)).expect("known retired parent");
        assert_eq!(
            state,
            State::RetiredSplit {
                birth_entry: birth,
                split_entry: split,
                children: &expected.map(id),
            }
        );
        let State::RetiredSplit { children, .. } = state else {
            unreachable!("asserted retirement")
        };
        let Some(Entry::Split {
            children: original, ..
        }) = history.entries().get(split)
        else {
            unreachable!("actual retirement record")
        };
        assert!(core::ptr::eq(children, original));
    }
    for (identity, birth_entry) in [(0, 1), (5, 2), (1, 3), (2, 3)] {
        assert_eq!(
            history.state(id(identity)),
            Some(State::Live { birth_entry })
        );
    }
    assert_eq!(history.state(id(99)), None);
}

#[test]
fn empty_and_full_width_roots_need_explicit_limits_without_numeric_order_policy() {
    let empty = SeamSpanHistory::new(vec![], limits(1, 1)).expect("explicitly admitted empty data");
    assert!(empty.entries().is_empty());
    assert_eq!(empty.identity_count(), 0);
    assert_eq!(empty.state(id(0)), None);
    let full = SeamSpanHistory::new(
        vec![Entry::Root(id(u128::MAX)), Entry::Root(id(0))],
        limits(2, 2),
    )
    .expect("supplied identities are neither generated nor silently sorted");
    assert_eq!(
        full.state(id(u128::MAX)),
        Some(State::Live { birth_entry: 0 })
    );
    assert_eq!(full.state(id(0)), Some(State::Live { birth_entry: 1 }));
}

#[test]
fn complete_entry_and_issued_counts_precede_semantics_and_include_retired_roots() {
    assert_eq!(
        SeamSpanHistory::new(nested(), limits(3, 6)),
        Err(Error::Entries {
            required: 4,
            limit: 3
        })
    );
    assert_eq!(
        SeamSpanHistory::new(nested(), limits(4, 5)),
        Err(Error::Identities {
            required: 6,
            limit: 5
        })
    );
    let invalid = vec![
        Entry::Split {
            parent: id(99),
            children: [id(1), id(1)],
        },
        Entry::Root(id(1)),
    ];
    assert_eq!(
        SeamSpanHistory::new(invalid.clone(), limits(1, 1)),
        Err(Error::Entries {
            required: 2,
            limit: 1
        })
    );
    assert_eq!(
        SeamSpanHistory::new(invalid.clone(), limits(2, 2)),
        Err(Error::Identities {
            required: 3,
            limit: 2
        })
    );
    assert_eq!(
        SeamSpanHistory::new(invalid, limits(2, 3)),
        Err(Error::MissingParent {
            entry: 0,
            parent: id(99)
        })
    );
}

#[test]
fn original_parent_and_birth_refusals_preserve_actual_entry_and_identity_priority() {
    let root = Entry::Root(id(1));
    let split = Entry::Split {
        parent: id(1),
        children: [id(2), id(3)],
    };
    let cases = [
        (
            vec![root, root],
            Error::ReusedIdentity {
                entry: 1,
                identity: id(1),
                first_birth: 0,
            },
        ),
        (
            vec![split, root],
            Error::MissingParent {
                entry: 0,
                parent: id(1),
            },
        ),
        (
            vec![root, split, split],
            Error::RetiredParent {
                entry: 2,
                parent: id(1),
                split_entry: 1,
            },
        ),
        (
            vec![root, split, root],
            Error::ReusedIdentity {
                entry: 2,
                identity: id(1),
                first_birth: 0,
            },
        ),
        (
            vec![
                root,
                split,
                Entry::Split {
                    parent: id(2),
                    children: [id(4), id(1)],
                },
            ],
            Error::ReusedIdentity {
                entry: 2,
                identity: id(1),
                first_birth: 0,
            },
        ),
        (
            vec![
                root,
                Entry::Root(id(9)),
                Entry::Split {
                    parent: id(1),
                    children: [id(9), id(1)],
                },
            ],
            Error::ReusedIdentity {
                entry: 2,
                identity: id(9),
                first_birth: 1,
            },
        ),
        (
            vec![
                root,
                Entry::Split {
                    parent: id(1),
                    children: [id(2), id(2)],
                },
            ],
            Error::ReusedIdentity {
                entry: 1,
                identity: id(2),
                first_birth: 1,
            },
        ),
    ];
    for (entries, expected) in cases {
        assert_eq!(SeamSpanHistory::new(entries, limits(8, 16)), Err(expected));
    }
}

#[test]
fn independent_chain_formula_keeps_all_ancestors_and_sibling_leaves_without_retargeting() {
    for depth in [1usize, 2, 17, 32, 257] {
        let mut entries = vec![Entry::Root(id(1))];
        for step in 0..depth {
            let parent = 2 * u128::try_from(step).expect("small explicit case") + 1;
            entries.push(Entry::Split {
                parent: id(parent),
                children: [id(parent + 2), id(parent + 1)],
            });
        }
        let history = SeamSpanHistory::new(entries, limits(depth + 1, 2 * depth + 1))
            .expect("closed-form chain population");
        assert_eq!(history.identity_count(), 2 * depth + 1);
        for step in 0..depth {
            let parent = 2 * u128::try_from(step).expect("small explicit case") + 1;
            assert_eq!(
                history.state(id(parent)),
                Some(State::RetiredSplit {
                    birth_entry: step,
                    split_entry: step + 1,
                    children: &[id(parent + 2), id(parent + 1)],
                })
            );
            assert_eq!(
                history.state(id(parent + 1)),
                Some(State::Live {
                    birth_entry: step + 1
                })
            );
        }
        let end = 2 * u128::try_from(depth).expect("small explicit case") + 1;
        assert_eq!(
            history.state(id(end)),
            Some(State::Live { birth_entry: depth })
        );
        assert_eq!(history.state(id(end + 1)), None);
    }
}

#[test]
fn canonical_identity_history_does_not_depend_on_selected_runtime_admission_policy() {
    let entries = vec![Entry::Root(id(7))];
    let tight = SeamSpanHistory::new(entries.clone(), limits(1, 1)).expect("valid exact input");
    let roomy = SeamSpanHistory::new(entries, limits(8, 16)).expect("same valid semantic records");
    assert_eq!(
        tight, roomy,
        "runtime allowance is not authored span content"
    );
}

#[test]
fn split_extension_keeps_exact_prefix_snapshot_and_immediate_child_order() {
    let original = SeamSpanHistory::new(nested(), limits(4, 6)).expect("actual accepted prefix");
    let saved = original.clone();
    let pointer = original.entries().as_ptr();
    let children = [id(u128::MAX), id(99)];
    let next = original
        .with_split(id(1), children, limits(5, 8))
        .expect("explicit complete next population");
    let roomy = original
        .with_split(id(1), children, limits(20, 30))
        .expect("runtime allowance does not change canonical content");
    assert_eq!(original, saved);
    assert_eq!(original.entries().as_ptr(), pointer);
    assert_eq!(next, roomy);
    assert_eq!(next.entries().len(), 5);
    assert_eq!(next.identity_count(), 8);
    assert_eq!(
        next.entries().last(),
        Some(&Entry::Split {
            parent: id(1),
            children
        })
    );
    assert_eq!(
        next.entries().iter().take(4).copied().collect::<Vec<_>>(),
        nested()
    );
    assert_eq!(original.state(id(1)), Some(State::Live { birth_entry: 3 }));
    assert_eq!(
        next.state(id(1)),
        Some(State::RetiredSplit {
            birth_entry: 3,
            split_entry: 4,
            children: &children,
        })
    );
    for identity in children {
        assert_eq!(next.state(identity), Some(State::Live { birth_entry: 4 }));
    }
    assert_eq!(next.state(id(8)), original.state(id(8)));
    assert_eq!(next.state(id(0)), original.state(id(0)));
    assert_eq!(next.state(id(5)), original.state(id(5)));
}

#[test]
fn split_extension_preserves_complete_quota_parent_and_child_refusal_priority() {
    let original = SeamSpanHistory::new(nested(), limits(4, 6)).expect("actual accepted prefix");
    let saved = original.clone();
    let cases = [
        (
            99,
            [8, 8],
            4,
            1,
            Error::Entries {
                required: 5,
                limit: 4,
            },
        ),
        (
            99,
            [8, 8],
            5,
            7,
            Error::Identities {
                required: 8,
                limit: 7,
            },
        ),
        (
            99,
            [8, 8],
            5,
            8,
            Error::MissingParent {
                entry: 4,
                parent: id(99),
            },
        ),
        (
            8,
            [8, 8],
            5,
            8,
            Error::RetiredParent {
                entry: 4,
                parent: id(8),
                split_entry: 2,
            },
        ),
        (
            1,
            [3, 8],
            5,
            8,
            Error::ReusedIdentity {
                entry: 4,
                identity: id(3),
                first_birth: 2,
            },
        ),
        (
            1,
            [99, 0],
            5,
            8,
            Error::ReusedIdentity {
                entry: 4,
                identity: id(0),
                first_birth: 1,
            },
        ),
        (
            1,
            [99, 99],
            5,
            8,
            Error::ReusedIdentity {
                entry: 4,
                identity: id(99),
                first_birth: 4,
            },
        ),
    ];
    for (parent, children, entries, identities, expected) in cases {
        assert_eq!(
            original.with_split(id(parent), children.map(id), limits(entries, identities)),
            Err(expected)
        );
        assert_eq!(original, saved);
    }
    let empty = SeamSpanHistory::new(vec![], limits(1, 1)).expect("empty accepted prefix");
    assert_eq!(
        empty.with_split(id(7), [id(9), id(3)], limits(1, 2)),
        Err(Error::MissingParent {
            entry: 0,
            parent: id(7)
        })
    );
}

#[test]
fn split_extension_matches_independent_full_map_validation_without_rewriting_prior_states() {
    let original = SeamSpanHistory::new(nested(), limits(4, 6)).expect("actual accepted prefix");
    for parent in [8, 3, 0, 5, 1, 99] {
        for children in [
            [8, 3],
            [99, 99],
            [99, u128::MAX],
            [0, u128::MAX],
            [u128::MAX, 8],
        ] {
            let record = Entry::Split {
                parent: id(parent),
                children: children.map(id),
            };
            for (entry_limit, id_limit) in [(4, 6), (5, 7), (5, 8), (9, 16)] {
                let cap = limits(entry_limit, id_limit);
                let mut rebuilt = nested();
                rebuilt.push(record);
                assert_eq!(
                    original.with_split(id(parent), children.map(id), cap),
                    SeamSpanHistory::new(rebuilt, cap),
                    "the linear accepted-prefix transition and complete map validation agree"
                );
            }
        }
    }
    assert_eq!(original.entries(), nested());
    assert_eq!(original.identity_count(), 6);
}
