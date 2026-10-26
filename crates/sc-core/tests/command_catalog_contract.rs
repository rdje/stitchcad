//! Compare native discovery metadata with the independently authored normative command table.
#![allow(clippy::expect_used)]
use sc_core::command::{CommandAuthority as A, CommandId as Id, CommandUndoGranularity as G};
use std::collections::BTreeSet;

const BOOK: &str = include_str!("../../../docs/book/src/spec/command-layer.md");

#[test]
fn declared_catalog_matches_every_book_row_without_order_or_metadata_drift() {
    let section = BOOK
        .split("### 1.1 The commands")
        .nth(1)
        .expect("command table")
        .split("## 2.")
        .next()
        .expect("section boundary");
    let rows = section
        .lines()
        .filter(|line| line.starts_with("| `"))
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().trim_matches('`'))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        rows.len(),
        Id::ALL.len(),
        "both additions and omissions must be visible"
    );
    let mut names = BTreeSet::new();
    for (id, cells) in Id::ALL.into_iter().zip(rows) {
        let [name, class, authority, reversible, granularity, _description]: [&str; 6] =
            cells.try_into().expect("six normative cells");
        assert!(names.insert(id.token()), "no duplicated catalog identity");
        assert_eq!(id.token(), name);
        assert_eq!(Id::from_token(name), Some(id));
        assert_eq!(id.class().token(), class, "{name}");
        assert_eq!(id.required_authority().token(), authority, "{name}");
        assert_eq!(id.reversibility().token(), reversible, "{name}");
        let grouping = if granularity == "not applicable" {
            Some(G::NotApplicable)
        } else if granularity == "one group per measurement changed" {
            Some(G::PerMeasurement)
        } else if granularity.starts_with("one group") {
            Some(G::AtomicGroup)
        } else {
            None
        }
        .expect("declared undo grouping");
        assert_eq!(id.undo_granularity(), grouping, "{name}");
    }
}

#[test]
fn lookup_is_exact_and_required_permission_metadata_cannot_be_confused_with_preview() {
    for token in [
        "",
        "getpiecelist",
        " GetPieceList",
        "GetPieceList ",
        "GetPieceList\n",
        "Approve",
    ] {
        assert_eq!(Id::from_token(token), None, "{token:?}");
    }
    assert_eq!(Id::PrepareRelease.required_authority(), A::Generate);
    assert_eq!(Id::ApprovePackage.required_authority(), A::Approve);
    assert_eq!(Id::ChangeMeasurement.required_authority(), A::Commit);
    // These are metadata values, not an authorization operation or an actor-held grant.
    assert_eq!(A::Inspect.token(), "inspect");
    assert_eq!(A::Propose.token(), "propose");
    assert!(
        BOOK.contains("preview needs only `inspect`")
            || BOOK.contains("needs no authority beyond `inspect`")
    );
}
