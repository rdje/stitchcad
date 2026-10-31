//! Explicit choices retain original input;these tests perform no geometric transform or effect fit.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*, recipe::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn source(n: u128) -> DirectedRange {
    DirectedRange {
        range: EdgeRange::whole(EdgeRef::new(id(n), LocalTag::new(17))),
        direction: Direction::Reversed,
    }
}
fn inputs() -> [SlashSpreadConstruction; 2] {
    [
        SlashSpreadConstruction::rotation(
            id(10),
            id(50),
            source(20),
            SlashSpreadSide::Left,
            PointRef::new(id(30), LocalTag::new(4)),
            FormulaExpressionDocument::new("-450 deg".into()).expect("angle"),
        )
        .expect("rotation intent"),
        SlashSpreadConstruction::separation(
            id(11),
            id(50),
            source(20),
            SlashSpreadSide::Right,
            source(40),
            FormulaExpressionDocument::new("1 mm/0".into()).expect("distance"),
        )
        .expect("separation intent"),
    ]
}
#[test]
fn both_modes_and_all_explicit_policy_combinations_retain_original_fields_and_formula_proofs() {
    for input in inputs() {
        for boundary in [
            SlashSpreadBoundaryPolicy::StraightBridges,
            SlashSpreadBoundaryPolicy::TrimOverlaps,
        ] {
            for fullness in [
                SlashSpreadFullnessIntent::Added,
                SlashSpreadFullnessIntent::Removed,
                SlashSpreadFullnessIntent::Unchanged,
            ] {
                let output = SlashSpreadOutputIntent::new(input.clone(), boundary, fullness);
                assert_eq!(output.input(), &input);
                assert_eq!(output.boundary_policy(), &boundary);
                assert_eq!(output.fullness(), &fullness);
                let actual = output.input();
                for dependency in actual.dependencies() {
                    assert!(std::ptr::eq(dependency.construction(), actual));
                    match dependency.source() {
                        SlashSpreadSource::Cut(field) => assert!(std::ptr::eq(field, actual.cut())),
                        SlashSpreadSource::Pivot(field) => {
                            assert!(std::ptr::eq(field, actual.pivot().expect("pivot")))
                        }
                        SlashSpreadSource::Direction(field) => {
                            assert!(std::ptr::eq(field, actual.direction().expect("axis")))
                        }
                    }
                }
                let normalized = actual
                    .opening()
                    .normalized(SlashSpreadNormalizationLimits::new(3, 7))
                    .expect("original amount shape");
                let checked = normalized
                    .check_argument(&FormulaNamespace::new([]).expect("namespace"))
                    .expect("original kind without execution");
                assert!(std::ptr::eq(
                    checked.normalized().opening(),
                    actual.opening()
                ));
                assert_eq!(checked.expression().kind(), actual.mode().required_kind());
            }
        }
    }
}
#[test]
fn different_choices_are_distinct_even_for_the_same_unexecuted_zero_input() {
    let input = SlashSpreadConstruction::separation(
        id(10),
        id(50),
        source(20),
        SlashSpreadSide::Left,
        source(40),
        FormulaExpressionDocument::new("0 mm".into()).expect("zero document"),
    )
    .expect("source roles");
    let added = SlashSpreadOutputIntent::new(
        input.clone(),
        SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Added,
    );
    let unchanged = SlashSpreadOutputIntent::new(
        input.clone(),
        SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Unchanged,
    );
    let trimmed = SlashSpreadOutputIntent::new(
        input.clone(),
        SlashSpreadBoundaryPolicy::TrimOverlaps,
        SlashSpreadFullnessIntent::Unchanged,
    );
    assert_ne!(added, unchanged);
    assert_ne!(unchanged, trimmed);
    assert_eq!(added.input(), &input);
    assert_eq!(unchanged.input(), &input);
    assert_eq!(trimmed.input(), &input);
    // Neither sign,zero nor policy naming was accepted as geometric support or measured effect.
}
