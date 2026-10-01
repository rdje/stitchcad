//! Individual Ease domain, current binding, uncertainty and compression contracts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError},
};
use sc_measure::{
    CompressionPermission, Ease, EaseDefinition, EaseError, EaseSide, FitIntent, Landmark,
    LandmarkDefinition, Measurement, MeasurementBinding, MeasurementContext, MeasurementDefinition,
    MeasurementError, MeasurementKind, MeasurementProcedure, MeasurementProcedureDefinition,
    MeasurementTableContext,
};
use sc_units::{Length, Unit};
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn length(n: i64) -> Length {
    Length::from_micrometres(n).unwrap()
}
fn scalar(n: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(n),
        source: id(90),
        state,
    })
    .unwrap()
}
fn assumed(n: i64) -> LengthState {
    LengthState::Assumed {
        value: length(n),
        assumption: id(91),
    }
}
struct Fixture {
    declarations: Vec<LengthDeclaration>,
    landmarks: Vec<Landmark>,
    procedures: Vec<MeasurementProcedure>,
    measurements: Vec<Measurement>,
}
impl Fixture {
    fn new() -> Self {
        let mut f = Self {
            declarations: vec![
                scalar(10, assumed(740_000)),
                scalar(11, assumed(780_000)),
                scalar(12, assumed(40_000)),
            ],
            landmarks: (20..24)
                .map(|n| {
                    Landmark::new(LandmarkDefinition {
                        id: id(n),
                        name: format!("Fixture {n}"),
                        kind: if n < 22 {
                            MeasurementKind::Body
                        } else {
                            MeasurementKind::Garment
                        },
                        source: id(92),
                    })
                    .unwrap()
                })
                .collect(),
            procedures: (30..32)
                .map(|n| {
                    MeasurementProcedure::new(MeasurementProcedureDefinition {
                        id: id(n),
                        name: format!("Fixture {n}"),
                        kind: if n == 30 {
                            MeasurementKind::Body
                        } else {
                            MeasurementKind::Garment
                        },
                        source: id(93),
                        documentation: "Authored fixture only; no physical protocol certification."
                            .to_owned(),
                    })
                    .unwrap()
                })
                .collect(),
            measurements: vec![],
        };
        let records = f.records();
        f.measurements = (1..3)
            .map(|n| {
                Measurement::new(
                    MeasurementDefinition {
                        id: id(n),
                        name: "Waist".to_owned(),
                        token: MachineToken::new(if n == 1 {
                            "body_waist"
                        } else {
                            "garment_waist"
                        })
                        .unwrap(),
                        entered_unit: Unit::Centimetre,
                        kind: if n == 1 {
                            MeasurementKind::Body
                        } else {
                            MeasurementKind::Garment
                        },
                        landmarks: if n == 1 {
                            [id(20), id(21)]
                        } else {
                            [id(22), id(23)]
                        },
                        procedure: if n == 1 { id(30) } else { id(31) },
                        declaration: id(9 + n),
                    },
                    &records,
                )
                .unwrap()
            })
            .collect();
        f
    }
    fn records(&self) -> MeasurementContext<'_> {
        MeasurementContext::new(&self.declarations, &self.landmarks, &self.procedures).unwrap()
    }
    fn definition(&self) -> EaseDefinition {
        EaseDefinition {
            id: id(3),
            body: MeasurementBinding::from(&self.measurements[0]),
            garment: MeasurementBinding::from(&self.measurements[1]),
            declaration: id(12),
            fit: FitIntent::Semi,
            provenance: id(94),
            compression: CompressionPermission::Forbidden,
        }
    }
    fn ease(&self) -> Ease {
        let r = self.records();
        let c = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        Ease::new(self.definition(), &c).unwrap()
    }
    fn replace_measurement(&mut self, index: usize, definition: MeasurementDefinition) {
        let replacement = Measurement::new(definition, &self.records()).unwrap();
        self.measurements[index] = replacement;
    }
}
#[test]
fn canonical_amount_and_metadata_are_borrowed_without_recomputing_or_copying() {
    let f = Fixture::new();
    let r = f.records();
    let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    let e = f.ease();
    assert_eq!(e.id(), id(3));
    assert_eq!(e.definition().provenance, id(94));
    assert_eq!(e.definition().fit, FitIntent::Semi);
    assert!(std::ptr::eq(e.declaration(&c).unwrap(), &f.declarations[2]));
    assert!(std::ptr::eq(
        e.measurement(EaseSide::Body, &c).unwrap(),
        &f.measurements[0]
    ));
    assert!(std::ptr::eq(
        e.measurement(EaseSide::Garment, &c).unwrap(),
        &f.measurements[1]
    ));
    assert_eq!(e.authored_value(&c).unwrap(), length(40_000));
}
#[test]
fn fit_classes_are_ordered_without_a_compression_default() {
    assert!(FitIntent::Close < FitIntent::Semi && FitIntent::Semi < FitIntent::Loose);
    for fit in [FitIntent::Close, FitIntent::Semi, FitIntent::Loose] {
        let mut f = Fixture::new();
        f.declarations[2] = scalar(12, assumed(-1));
        let r = f.records();
        let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        let mut d = f.definition();
        d.fit = fit;
        assert!(
            matches!(Ease::new(d,&c),Err(EaseError::UndeclaredCompression{amount,..}) if amount==length(-1))
        );
    }
}
#[test]
fn authored_sides_must_be_body_then_garment() {
    let f = Fixture::new();
    let r = f.records();
    let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    for side in [EaseSide::Body, EaseSide::Garment] {
        let mut d = f.definition();
        let actual = if side == EaseSide::Body {
            d.body = f.definition().garment;
            MeasurementKind::Garment
        } else {
            d.garment = f.definition().body;
            MeasurementKind::Body
        };
        assert_eq!(
            Ease::new(d, &c).err(),
            Some(EaseError::WrongKind {
                ease: id(3),
                side,
                actual
            })
        );
    }
}
#[test]
fn amount_must_not_alias_either_measurement_scalar() {
    let f = Fixture::new();
    let r = f.records();
    let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    for n in [10, 11] {
        let mut d = f.definition();
        d.declaration = id(n);
        assert_eq!(
            Ease::new(d, &c).err(),
            Some(EaseError::AliasedDeclaration(id(n)))
        );
    }
}
#[test]
fn current_mapping_identity_collisions_refuse() {
    let mut f = Fixture::new();
    let e = f.ease();
    f.declarations.push(scalar(3, assumed(0)));
    let r = f.records();
    let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    assert_eq!(
        e.validate_current(&c).err(),
        Some(EaseError::DuplicateIdentity(id(3)))
    );
}
#[test]
fn missing_identity_is_not_replaced_by_same_token_peer() {
    let mut f = Fixture::new();
    let e = f.ease();
    let mut d = f.measurements[0].definition().clone();
    d.id = id(4);
    f.replace_measurement(0, d);
    let r = f.records();
    let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    assert_eq!(
        e.validate_current(&c).err(),
        Some(EaseError::MissingMeasurement {
            ease: id(3),
            side: EaseSide::Body,
            measurement: id(1)
        })
    );
}
#[test]
fn same_id_binding_reassignments_require_explicit_replacement() {
    for change in 0..3 {
        let mut f = Fixture::new();
        let e = f.ease();
        let mut d = f.measurements[0].definition().clone();
        match change {
            0 => d.token = MachineToken::new("new_waist").unwrap(),
            1 => {
                d.kind = MeasurementKind::Garment;
                d.landmarks = [id(22), id(23)];
                d.procedure = id(31);
            }
            _ => d.declaration = id(11),
        };
        f.replace_measurement(0, d);
        let r = f.records();
        let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        assert!(
            matches!(e.validate_current(&c),Err(EaseError::ReassignedBinding{ease,side:EaseSide::Body,expected,actual}) if ease==id(3) && *expected==e.definition().body && *actual==MeasurementBinding::from(&f.measurements[0]))
        );
        if change != 1 {
            let replacement = Ease::new(f.definition(), &c).unwrap();
            assert!(replacement.validate_current(&c).is_ok());
            assert_ne!(replacement.definition().body, e.definition().body);
        }
    }
}
#[test]
fn missing_current_metadata_preserves_side_and_underlying_error() {
    let mut f = Fixture::new();
    let e = f.ease();
    f.landmarks.remove(2);
    let r = f.records();
    let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    assert_eq!(
        e.validate_current(&c).err(),
        Some(EaseError::InvalidMeasurement {
            ease: id(3),
            side: EaseSide::Garment,
            issue: MeasurementError::MissingLandmark(id(22)),
        })
    );
    let error = e.validate_current(&c).err().unwrap();
    assert_eq!(
        error,
        EaseError::InvalidMeasurement {
            ease: id(3),
            side: EaseSide::Garment,
            issue: MeasurementError::MissingLandmark(id(22))
        }
    );
    assert!(error.source().is_some());
    assert!(e.measurement(EaseSide::Body, &c).is_ok());
}
#[test]
fn all_authored_value_states_enforce_negative_permission_and_preserve_sign() {
    for amount in [-40_000, 0, 40_000] {
        for state in [
            assumed(amount),
            LengthState::Known {
                value: length(amount),
                evidence: vec![id(95)],
            },
            LengthState::Preference {
                value: length(amount),
                provenance: id(96),
            },
        ] {
            let mut f = Fixture::new();
            f.declarations[2] = scalar(12, state.clone());
            let r = f.records();
            let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
            let d = f.definition();
            if amount < 0 {
                assert_eq!(
                    Ease::new(d.clone(), &c).err(),
                    Some(EaseError::UndeclaredCompression {
                        ease: id(3),
                        declaration: id(12),
                        amount: length(amount)
                    })
                );
            } else {
                assert_eq!(
                    Ease::new(d.clone(), &c)
                        .unwrap()
                        .authored_value(&c)
                        .unwrap(),
                    length(amount)
                );
            }
            let mut declared = d;
            declared.compression = CompressionPermission::Declared { provenance: id(97) };
            let e = Ease::new(declared, &c).unwrap();
            assert_eq!(e.authored_value(&c).unwrap(), length(amount));
            assert_eq!(e.declaration(&c).unwrap().state(), &state);
        }
    }
}
#[test]
fn unknown_and_derived_are_inspectable_without_numeric_fallback() {
    for (state, issue) in [
        (
            LengthState::Unknown {
                observation: id(98),
            },
            LengthValueError::RequiresObservation {
                declaration: id(12),
                observation: id(98),
            },
        ),
        (
            LengthState::Derived { formula: id(99) },
            LengthValueError::RequiresEvaluation {
                declaration: id(12),
                formula: id(99),
            },
        ),
    ] {
        let mut f = Fixture::new();
        f.declarations[2] = scalar(12, state.clone());
        let e = f.ease();
        let r = f.records();
        let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        assert_eq!(e.declaration(&c).unwrap().state(), &state);
        let error = e.authored_value(&c).err().unwrap();
        assert_eq!(error, EaseError::UnavailableValue { ease: id(3), issue });
        assert!(error.source().is_some());
        assert_eq!(
            e.validate_amount(length(-1)).err(),
            Some(EaseError::UndeclaredCompression {
                ease: id(3),
                declaration: id(12),
                amount: length(-1)
            })
        );
        assert!(e.validate_amount(Length::ZERO).is_ok());
    }
}
#[test]
fn current_amount_edits_are_visible_and_missing_amount_refuses() {
    let mut f = Fixture::new();
    let e = f.ease();
    f.declarations[2] = scalar(12, assumed(-1));
    {
        let r = f.records();
        let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        assert_eq!(
            e.authored_value(&c).err(),
            Some(EaseError::UndeclaredCompression {
                ease: id(3),
                declaration: id(12),
                amount: length(-1)
            })
        );
    }
    f.declarations[2] = scalar(
        12,
        LengthState::Unknown {
            observation: id(98),
        },
    );
    {
        let r = f.records();
        let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        assert!(matches!(
            e.authored_value(&c),
            Err(EaseError::UnavailableValue { .. })
        ));
    }
    f.declarations.pop();
    let r = f.records();
    let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    assert_eq!(
        e.validate_current(&c).err(),
        Some(EaseError::MissingDeclaration {
            ease: id(3),
            declaration: id(12)
        })
    );
}
#[test]
fn shared_spelling_is_legal_across_body_and_pom_namespaces() {
    let mut f = Fixture::new();
    let mut d = f.measurements[1].definition().clone();
    d.token = f.measurements[0].definition().token.clone();
    f.replace_measurement(1, d);
    assert_eq!(
        f.ease().definition().body.token,
        f.ease().definition().garment.token
    );
}
#[test]
fn same_id_source_state_and_inventory_order_edits_do_not_create_caches() {
    let mut f = Fixture::new();
    let e = f.ease();
    let mut d = f.declarations[2].definition().clone();
    d.source = id(100);
    d.state = LengthState::Preference {
        value: length(20_000),
        provenance: id(101),
    };
    f.declarations[2] = LengthDeclaration::new(d).unwrap();
    f.declarations[0] = scalar(
        10,
        LengthState::Unknown {
            observation: id(102),
        },
    );
    f.declarations.reverse();
    f.measurements.reverse();
    let r = f.records();
    let c = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    assert_eq!(e.authored_value(&c).unwrap(), length(20_000));
    assert_eq!(e.declaration(&c).unwrap().source(), id(100));
    assert_eq!(e.measurement(EaseSide::Body, &c).unwrap().id(), id(1));
    assert!(matches!(
        e.measurement(EaseSide::Body, &c)
            .unwrap()
            .declaration(&r)
            .unwrap()
            .authored_value(),
        Err(LengthValueError::RequiresObservation { .. })
    ));
}
