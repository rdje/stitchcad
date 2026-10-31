//! Authored Piece-copy and axis sources,without generated copy identities or geometry.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{
        MirrorAxis as Axis, MirrorAxisSource as Source, MirrorPieceCopy as Copy,
        MirrorPieceCopyError as Error,
    },
    ontology::{
        DirectedRange, Direction, EdgeRange, EdgeRef, EntityId, LocalTag, Param, PointRef, Rational,
    },
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn point(creator: u128, tag: u32) -> PointRef {
    PointRef::new(id(creator), LocalTag::new(tag))
}
fn range(creator: u128, direction: Direction) -> DirectedRange {
    let parameter =
        |n| Param::new(Rational::new(n, 5).expect("exact bound")).expect("unit interval");
    DirectedRange {
        range: EdgeRange::new(
            EdgeRef::new(id(creator), LocalTag::new(37)),
            parameter(1),
            parameter(4),
        )
        .expect("positive original interval"),
        direction,
    }
}

#[test]
fn both_explicit_axis_modes_keep_their_original_underlying_fields_and_owning_copy() {
    for axis in [
        Axis::ThroughPoints {
            start: point(99, 7),
            end: point(80, 9),
        },
        Axis::AlongRange(range(70, Direction::Reversed)),
    ] {
        let input = Copy::new(id(10), id(50), axis).expect("identity sorting is not chronology");
        assert_eq!(input.operation(), id(10));
        assert_eq!(input.source_piece(), id(50));
        assert_eq!(*input.axis(), axis);
        let expected = match input.axis() {
            Axis::ThroughPoints { start, end } => vec![Source::Start(start), Source::End(end)],
            Axis::AlongRange(range) => vec![Source::Range(range)],
        };
        assert_eq!(input.dependency_count(), expected.len());
        let dependencies = input.dependencies().collect::<Vec<_>>();
        assert_eq!(dependencies.len(), expected.len());
        for (dependency, field) in dependencies.iter().zip(&expected) {
            assert!(std::ptr::eq(dependency.input(), &input));
            match (dependency.source(), *field) {
                (Source::Start(actual), Source::Start(wanted))
                | (Source::End(actual), Source::End(wanted)) => {
                    assert!(std::ptr::eq(actual, wanted));
                }
                (Source::Range(actual), Source::Range(wanted)) => {
                    assert!(std::ptr::eq(actual, wanted))
                }
                _ => assert_eq!(dependency.source(), *field),
            }
        }
        assert_eq!(
            input
                .dependencies()
                .rev()
                .map(|dependency| dependency.source())
                .collect::<Vec<_>>(),
            expected.into_iter().rev().collect::<Vec<_>>()
        );
    }
}

#[test]
fn actual_local_self_supply_keeps_original_point_tags_range_and_first_role() {
    let own = point(10, 41);
    let other = point(99, 7);
    assert_eq!(
        Copy::new(
            id(10),
            id(10),
            Axis::ThroughPoints {
                start: own,
                end: own
            }
        )
        .expect_err("Piece before axis"),
        Error::SelfPiece(id(10))
    );
    assert_eq!(
        Copy::new(
            id(10),
            id(50),
            Axis::ThroughPoints {
                start: own,
                end: own
            }
        )
        .expect_err("first original point"),
        Error::SelfAxisStart {
            operation: id(10),
            point: own
        }
    );
    assert_eq!(
        Copy::new(
            id(10),
            id(50),
            Axis::ThroughPoints {
                start: other,
                end: own
            }
        )
        .expect_err("second original point"),
        Error::SelfAxisEnd {
            operation: id(10),
            point: own
        }
    );
    let source = range(10, Direction::Reversed);
    assert_eq!(
        Copy::new(id(10), id(50), Axis::AlongRange(source)).expect_err("actual original range"),
        Error::SelfAxisRange {
            operation: id(10),
            range: source
        }
    );
}

#[test]
fn repeated_axis_points_keep_both_source_uses_without_a_geometric_nondegeneracy_claim() {
    let source = point(99, 7);
    let input = Copy::new(
        id(10),
        id(50),
        Axis::ThroughPoints {
            start: source,
            end: source,
        },
    )
    .expect("source declaration grants no valid geometric axis");
    assert_eq!(
        *input.axis(),
        Axis::ThroughPoints {
            start: source,
            end: source
        }
    );
    assert_eq!(
        input
            .dependencies()
            .map(|dependency| dependency.source())
            .collect::<Vec<_>>(),
        vec![Source::Start(&source), Source::End(&source)]
    );
    assert_eq!(
        input
            .dependencies()
            .map(|dependency| dependency.source().creator())
            .collect::<Vec<_>>(),
        vec![id(99), id(99)]
    );
    assert_eq!(input.dependency_count(), 2);
}

#[test]
fn axis_order_and_range_traversal_remain_authored_without_implicit_normalization() {
    let start = point(99, 7);
    let end = point(80, 9);
    let forward =
        Copy::new(id(10), id(50), Axis::ThroughPoints { start, end }).expect("original axis");
    let reverse = Copy::new(
        id(10),
        id(50),
        Axis::ThroughPoints {
            start: end,
            end: start,
        },
    )
    .expect("reversed source intent");
    assert_ne!(forward, reverse);
    assert_eq!(forward.operation(), reverse.operation());
    let source = range(70, Direction::Original);
    let original = Copy::new(id(10), id(50), Axis::AlongRange(source)).expect("original range");
    let reverse = Copy::new(
        id(10),
        id(50),
        Axis::AlongRange(DirectedRange {
            direction: Direction::Reversed,
            ..source
        }),
    )
    .expect("explicit reverse traversal");
    assert_ne!(original, reverse);
    assert_eq!(original.source_piece(), reverse.source_piece());
    // Canonical geometric reflection equality and current source ownership remain separate checks.
}
