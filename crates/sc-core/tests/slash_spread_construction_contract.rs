//! Original spatial field roles and amount coupling,without geometric region/transform claims.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{
        SlashSpreadConstruction as Spread, SlashSpreadConstructionError as Error,
        SlashSpreadNormalizationLimits as Limits, SlashSpreadOpeningMode as Mode,
        SlashSpreadSide as Side, SlashSpreadSource as Source,
    },
    ontology::{
        DirectedRange, Direction, EdgeRange, EdgeRef, EntityId, LocalTag, Param, PointRef, Rational,
    },
    recipe::{FormulaExpressionDocument as Doc, FormulaKind, FormulaNamespace},
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn point(creator: u128) -> PointRef {
    PointRef::new(id(creator), LocalTag::new(17))
}
fn range(creator: u128, direction: Direction) -> DirectedRange {
    let parameter =
        |n| Param::new(Rational::new(n, 7).expect("exact bound")).expect("unit interval");
    DirectedRange {
        range: EdgeRange::new(
            EdgeRef::new(id(creator), LocalTag::new(13)),
            parameter(2),
            parameter(5),
        )
        .expect("positive original range"),
        direction,
    }
}
fn doc(source: &str) -> Doc {
    Doc::new(source.into()).expect("original amount document")
}
fn rotation() -> Spread {
    Spread::rotation(
        id(10),
        id(50),
        range(99, Direction::Reversed),
        Side::Left,
        point(80),
        doc("-450 deg"),
    )
    .expect("distinct original source operations")
}
fn separation() -> Spread {
    Spread::separation(
        id(10),
        id(50),
        range(99, Direction::Reversed),
        Side::Right,
        range(70, Direction::Original),
        doc("1 mm/0"),
    )
    .expect("distinct original sources")
}

#[test]
fn both_variants_keep_exact_original_field_borrows_and_actual_embedded_amount_proofs() {
    for construction in [rotation(), separation()] {
        assert_eq!(construction.operation(), id(10));
        assert_eq!(construction.piece(), id(50));
        assert_eq!(*construction.cut(), range(99, Direction::Reversed));
        assert_eq!(construction.dependencies().len(), 2);
        let mut uses = construction.dependencies();
        let cut = uses.next().expect("cut first");
        assert!(std::ptr::eq(cut.construction(), &construction));
        assert!(
            matches!(cut.source(),Source::Cut(actual) if std::ptr::eq(actual,construction.cut()))
        );
        let motion = uses.next().expect("actual motion source");
        assert!(std::ptr::eq(motion.construction(), &construction));
        match construction.mode() {
            Mode::Rotation => {
                let pivot = construction.pivot().expect("actual required pivot");
                assert!(
                    matches!(motion.source(),Source::Pivot(actual) if std::ptr::eq(actual,pivot))
                );
                assert_eq!(*pivot, point(80));
                assert_eq!(construction.direction(), None);
                assert_eq!(construction.moving_side(), Side::Left);
            }
            Mode::Separation => {
                let direction = construction.direction().expect("actual required axis");
                assert!(
                    matches!(motion.source(),Source::Direction(actual) if std::ptr::eq(actual,direction))
                );
                assert_eq!(*direction, range(70, Direction::Original));
                assert_eq!(construction.pivot(), None);
                assert_eq!(construction.moving_side(), Side::Right);
            }
        }
        assert!(uses.next().is_none());
        assert_eq!(
            construction
                .dependencies()
                .next_back()
                .expect("motion last")
                .source(),
            motion.source()
        );
        let normalized = construction
            .opening()
            .normalized(Limits::new(3, 7))
            .expect("actual component shape");
        let checked = normalized
            .check_argument(&FormulaNamespace::new([]).expect("namespace"))
            .expect("kind proof without rotation or division");
        assert!(std::ptr::eq(
            checked.normalized().opening(),
            construction.opening()
        ));
        assert_eq!(
            checked.expression().kind(),
            construction.mode().required_kind()
        );
    }
}

#[test]
fn definite_self_supply_reports_original_piece_cut_pivot_or_axis_in_actual_role_order() {
    let own_cut = range(10, Direction::Reversed);
    let own_point = point(10);
    assert_eq!(
        Spread::rotation(id(10), id(10), own_cut, Side::Left, own_point, doc("1 deg"))
            .expect_err("Piece first"),
        Error::SelfPiece(id(10))
    );
    assert_eq!(
        Spread::rotation(
            id(10),
            id(50),
            own_cut,
            Side::Right,
            own_point,
            doc("1 deg")
        )
        .expect_err("cut before pivot"),
        Error::SelfCut {
            operation: id(10),
            cut: own_cut
        }
    );
    let cut = range(99, Direction::Original);
    assert_eq!(
        Spread::rotation(id(10), id(50), cut, Side::Left, own_point, doc("1 deg"))
            .expect_err("actual pivot"),
        Error::SelfPivot {
            operation: id(10),
            pivot: own_point
        }
    );
    assert_eq!(
        Spread::separation(id(10), id(50), own_cut, Side::Left, own_cut, doc("1 mm"))
            .expect_err("cut before movement axis"),
        Error::SelfCut {
            operation: id(10),
            cut: own_cut
        }
    );
    assert_eq!(
        Spread::separation(id(10), id(50), cut, Side::Right, own_cut, doc("1 mm"))
            .expect_err("actual movement axis"),
        Error::SelfDirection {
            operation: id(10),
            direction: own_cut
        }
    );
}

#[test]
fn moving_bank_and_directed_cut_are_explicit_intent_and_repeated_sources_keep_distinct_fields() {
    let cut = range(99, Direction::Original);
    let left = Spread::separation(id(10), id(50), cut, Side::Left, cut, doc("0 mm"))
        .expect("repeated source uses");
    let right = Spread::separation(id(10), id(50), cut, Side::Right, cut, doc("0 mm"))
        .expect("explicit opposite bank");
    assert_ne!(left, right);
    assert_eq!(left.opening(), right.opening());
    for construction in [&left, &right] {
        let uses = construction
            .dependencies()
            .map(|dependency| dependency.source())
            .collect::<Vec<_>>();
        assert_eq!(
            uses,
            vec![
                Source::Cut(construction.cut()),
                Source::Direction(construction.direction().expect("axis"))
            ]
        );
        assert_eq!(
            uses.iter()
                .map(|source| source.creator())
                .collect::<Vec<_>>(),
            vec![id(99), id(99)]
        );
    }
    let reverse = Spread::separation(
        id(10),
        id(50),
        DirectedRange {
            direction: Direction::Reversed,
            ..cut
        },
        Side::Left,
        cut,
        doc("0 mm"),
    )
    .expect("explicit reversed cut");
    assert_ne!(left, reverse);
    assert_eq!(left.cut().range, reverse.cut().range);
    // No side swapping,Piece classification or zero-gap geometric validity was inferred.
}

#[test]
fn complete_variant_keeps_chosen_mode_even_when_its_actual_amount_has_the_wrong_kind() {
    let cut = range(99, Direction::Original);
    let rotation = Spread::rotation(id(10), id(50), cut, Side::Left, point(80), doc("1 mm"))
        .expect("authored rotation");
    let separation = Spread::separation(
        id(10),
        id(50),
        cut,
        Side::Right,
        range(70, Direction::Reversed),
        doc("90 deg"),
    )
    .expect("authored separation");
    for (construction, mode, required) in [
        (rotation, Mode::Rotation, FormulaKind::Angle),
        (separation, Mode::Separation, FormulaKind::Length),
    ] {
        assert_eq!(construction.mode(), mode);
        let normalized = construction
            .opening()
            .normalized(Limits::new(1, 3))
            .expect("actual component");
        let error = normalized
            .check_argument(&FormulaNamespace::new([]).expect("namespace"))
            .expect_err("wrong explicit kind");
        assert_eq!(error.required_kind(), required);
        assert_eq!(error.mode(), mode);
        assert!(std::ptr::eq(
            error.normalized().opening(),
            construction.opening()
        ));
    }
}
