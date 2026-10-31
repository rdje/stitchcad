//! Explicit original control layouts and authentic field dependencies, without geometric promotion.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{
        BezierConstruction as Curve, BezierControlPoints as Controls, BezierDegree as Degree,
        BezierPointRole as Role,
    },
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn point(creator: u128, tag: u32) -> PointRef {
    PointRef::new(id(creator), LocalTag::new(tag))
}
fn quadratic(a: PointRef, b: PointRef, c: PointRef) -> Controls {
    Controls::Quadratic {
        start: a,
        control: b,
        end: c,
    }
}
fn cubic(a: PointRef, b: PointRef, c: PointRef, d: PointRef) -> Controls {
    Controls::Cubic {
        start: a,
        first_control: b,
        second_control: c,
        end: d,
    }
}

#[test]
fn explicit_variants_keep_exact_original_roles_and_underlying_owner_field_borrows() {
    let (a, b, c, d) = (point(99, 4), point(80, 8), point(70, 9), point(60, 12));
    for controls in [quadratic(a, b, d), cubic(a, b, c, d)] {
        let curve = Curve::new(id(10), controls).expect("id sort order provides no chronology");
        let fields = match curve.controls() {
            Controls::Quadratic {
                start,
                control,
                end,
            } => vec![
                (Role::Start, start),
                (Role::FirstControl, control),
                (Role::End, end),
            ],
            Controls::Cubic {
                start,
                first_control,
                second_control,
                end,
            } => vec![
                (Role::Start, start),
                (Role::FirstControl, first_control),
                (Role::SecondControl, second_control),
                (Role::End, end),
            ],
        };
        let dependencies = curve.dependencies().collect::<Vec<_>>();
        assert_eq!(dependencies.len(), fields.len());
        assert_eq!(curve.dependency_count(), fields.len());
        for (dependency, (role, field)) in dependencies.iter().zip(&fields) {
            assert!(std::ptr::eq(dependency.construction(), &curve));
            assert_eq!(dependency.role(), *role);
            assert_eq!(dependency.point(), **field);
            assert!(std::ptr::eq(dependency.point_ref(), *field));
        }
        assert_eq!(curve.output(), EdgeRef::new(id(10), LocalTag::FIRST));
        assert_eq!(
            curve
                .dependencies()
                .rev()
                .map(|source| source.role())
                .collect::<Vec<_>>(),
            fields
                .iter()
                .rev()
                .map(|(role, _)| *role)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn every_actual_self_supply_role_and_first_refusal_keep_original_tag_and_operation() {
    let (a, b, c, d, own) = (
        point(99, 4),
        point(80, 8),
        point(70, 9),
        point(60, 12),
        point(10, 17),
    );
    for (controls, wanted) in [
        (quadratic(own, b, d), Role::Start),
        (quadratic(a, own, d), Role::FirstControl),
        (quadratic(a, b, own), Role::End),
        (cubic(own, b, c, d), Role::Start),
        (cubic(a, own, c, d), Role::FirstControl),
        (cubic(a, b, own, d), Role::SecondControl),
        (cubic(a, b, c, own), Role::End),
        (cubic(own, own, own, own), Role::Start),
    ] {
        let error =
            Curve::new(id(10), controls).expect_err("own operation cannot supply prior controls");
        assert_eq!(error.operation(), id(10));
        assert_eq!(error.role(), wanted);
        assert_eq!(error.point(), own);
    }
}

#[test]
fn coincident_source_values_keep_chosen_degree_and_every_original_use_without_promotion() {
    let source = point(99, 4);
    let quadratic = Curve::new(id(10), quadratic(source, source, source))
        .expect("declaration grants no degeneracy proof");
    let cubic = Curve::new(id(10), cubic(source, source, source, source))
        .expect("explicit cubic source intent");
    assert_ne!(quadratic, cubic);
    assert_eq!(quadratic.degree(), Degree::Quadratic);
    assert_eq!(cubic.degree(), Degree::Cubic);
    assert_eq!(
        quadratic
            .dependencies()
            .map(|use_| use_.point())
            .collect::<Vec<_>>(),
        [source; 3]
    );
    assert_eq!(
        cubic
            .dependencies()
            .map(|use_| use_.point())
            .collect::<Vec<_>>(),
        [source; 4]
    );
    assert_eq!(
        quadratic
            .dependencies()
            .map(|use_| use_.role())
            .collect::<Vec<_>>(),
        [Role::Start, Role::FirstControl, Role::End]
    );
    assert_eq!(
        cubic
            .dependencies()
            .map(|use_| use_.role())
            .collect::<Vec<_>>(),
        [
            Role::Start,
            Role::FirstControl,
            Role::SecondControl,
            Role::End
        ]
    );
    assert_eq!(quadratic.output(), cubic.output());
    // G2 must elevate actual quadratic coordinates exactly; declaration metadata creates no controls.
}

#[test]
fn reversed_cubic_intent_changes_original_source_meaning_without_reassigning_output_role() {
    let (a, b, c, d) = (point(99, 4), point(80, 8), point(70, 9), point(60, 12));
    let forward = Curve::new(id(10), cubic(a, b, c, d)).expect("original input");
    let reversed = Curve::new(id(10), cubic(d, c, b, a)).expect("reversed input");
    assert_ne!(forward, reversed);
    assert_eq!(forward.output(), reversed.output());
    assert_eq!(
        forward
            .dependencies()
            .map(|use_| use_.point())
            .collect::<Vec<_>>(),
        [a, b, c, d]
    );
    assert_eq!(
        reversed
            .dependencies()
            .map(|use_| use_.point())
            .collect::<Vec<_>>(),
        [d, c, b, a]
    );
    // Current topology repair and publication of this edit remain actual program/bus obligations.
}
