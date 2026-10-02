//! Authored chart member/revision, correspondence, current metadata and value contracts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError},
};
use sc_measure::{
    Landmark, LandmarkDefinition, Measurement, MeasurementBinding, MeasurementContext,
    MeasurementDefinition, MeasurementError, MeasurementKind, MeasurementProcedure,
    MeasurementProcedureDefinition, MeasurementTable, MeasurementTableContext,
    MeasurementTableDefinition, MeasurementTableError, SizeChartContext, SizeChartError,
    SizeChartObservation, SizeChartObservationDefinition, SizeChartRole, SizeLabel, SizeMember,
    SizeMembership, SizeMembershipDefinition, SizeMembershipError, SizeSetReference, SizeSystem,
};
use sc_units::{Count, Length, Unit};
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
    membership: SizeMembership,
    tables: Vec<MeasurementTable>,
}
impl Fixture {
    fn new() -> Self {
        let mut f = Self {
            declarations: vec![scalar(10, assumed(740_000)), scalar(11, assumed(780_000))],
            landmarks: (20..24)
                .map(|n| {
                    Landmark::new(LandmarkDefinition {
                        id: id(n),
                        name: format!("Fixture {n}"),
                        kind: if n < 22 {
                            MeasurementKind::Garment
                        } else {
                            MeasurementKind::Body
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
                            MeasurementKind::Garment
                        } else {
                            MeasurementKind::Body
                        },
                        source: id(93),
                        documentation: "Authored fixture, not physical certification.".to_owned(),
                    })
                    .unwrap()
                })
                .collect(),
            measurements: vec![],
            membership: SizeMembership::new(SizeMembershipDefinition {
                reference: SizeSetReference {
                    id: id(40),
                    revision: Count::new(2),
                },
                system: SizeSystem::Custom,
                members: vec![
                    SizeMember {
                        id: id(42),
                        label: SizeLabel::new("M".to_owned()).unwrap(),
                    },
                    SizeMember {
                        id: id(41),
                        label: SizeLabel::new("S".to_owned()).unwrap(),
                    },
                ],
                base: id(41),
            })
            .unwrap(),
            tables: vec![],
        };
        let r = f.records();
        f.measurements = (1..3)
            .map(|n| {
                Measurement::new(
                    MeasurementDefinition {
                        id: id(n),
                        name: "Garment waist".to_owned(),
                        token: MachineToken::new("waist").unwrap(),
                        entered_unit: Unit::Centimetre,
                        kind: MeasurementKind::Garment,
                        landmarks: [id(20), id(21)],
                        procedure: id(30),
                        declaration: id(n + 9),
                    },
                    &r,
                )
                .unwrap()
            })
            .collect();
        f.rebuild_tables();
        f
    }
    fn records(&self) -> MeasurementContext<'_> {
        MeasurementContext::new(&self.declarations, &self.landmarks, &self.procedures).unwrap()
    }
    fn with_context<R>(&self, callback: impl FnOnce(&SizeChartContext<'_>) -> R) -> R {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        let c = SizeChartContext::new(&self.membership, &self.tables, &m).unwrap();
        callback(&c)
    }
    fn definition(&self) -> SizeChartObservationDefinition {
        SizeChartObservationDefinition {
            id: id(60),
            membership: self.membership.reference(),
            member: id(42),
            design_table: id(50),
            chart_table: id(51),
            pom: MeasurementBinding::from(&self.measurements[0]),
            measurement: MeasurementBinding::from(&self.measurements[1]),
            provenance: id(94),
        }
    }
    fn observation(&self) -> SizeChartObservation {
        self.with_context(|c| SizeChartObservation::new(self.definition(), c).unwrap())
    }
    fn replace_measurement(&mut self, index: usize, definition: MeasurementDefinition) {
        self.measurements[index] = Measurement::new(definition, &self.records()).unwrap();
    }
    fn rebuild_tables(&mut self) {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        self.tables = self
            .measurements
            .iter()
            .enumerate()
            .map(|(index, measurement)| {
                MeasurementTable::new(
                    MeasurementTableDefinition {
                        id: id(50 + index as u128),
                        name: format!("Fixture table {index}"),
                        entries: vec![MeasurementBinding::from(measurement)],
                    },
                    &m,
                )
                .unwrap()
            })
            .collect();
    }
}
#[test]
fn values_members_and_both_roles_are_borrowed_without_recomputing() {
    let f = Fixture::new();
    let o = f.observation();
    f.with_context(|c| {
        assert_eq!(o.id(), id(60));
        assert_eq!(o.definition().provenance, id(94));
        assert!(std::ptr::eq(
            o.member(c).unwrap(),
            &f.membership.definition().members[0]
        ));
        assert!(std::ptr::eq(o.declaration(c).unwrap(), &f.declarations[1]));
        for (role, index) in [
            (SizeChartRole::DesignPom, 0),
            (SizeChartRole::Observation, 1),
        ] {
            assert!(std::ptr::eq(
                o.measurement(role, c).unwrap(),
                &f.measurements[index]
            ));
        }
        assert_eq!(o.authored_value(c).unwrap(), length(780_000));
    });
}
#[test]
fn both_authored_roles_require_garment_domain() {
    for (role, index) in [
        (SizeChartRole::DesignPom, 0),
        (SizeChartRole::Observation, 1),
    ] {
        let mut f = Fixture::new();
        let mut d = f.measurements[index].definition().clone();
        d.kind = MeasurementKind::Body;
        d.landmarks = [id(22), id(23)];
        d.procedure = id(31);
        f.replace_measurement(index, d);
        f.rebuild_tables();
        f.with_context(|c| {
            assert_eq!(
                SizeChartObservation::new(f.definition(), c).err(),
                Some(SizeChartError::WrongKind {
                    observation: id(60),
                    role,
                    actual: MeasurementKind::Body
                })
            )
        });
    }
}
#[test]
fn exact_set_identity_and_revision_are_required() {
    for change_id in [false, true] {
        let mut f = Fixture::new();
        let o = f.observation();
        let mut d = f.membership.definition().clone();
        if change_id {
            d.reference.id = id(43);
        } else {
            d.reference.revision = Count::new(3);
        }
        f.membership = SizeMembership::new(d).unwrap();
        f.with_context(|c| {
            assert_eq!(
                o.validate_current(c).err(),
                Some(SizeChartError::MembershipMismatch {
                    observation: id(60),
                    expected: o.definition().membership,
                    actual: f.membership.reference(),
                })
            )
        });
    }
}
#[test]
fn same_label_member_replacement_cannot_restore_missing_identity() {
    let mut f = Fixture::new();
    let o = f.observation();
    let mut d = f.membership.definition().clone();
    d.members[0].id = id(43);
    f.membership = SizeMembership::new(d).unwrap();
    f.with_context(|c| {
        let error = o.member(c).err().unwrap();
        assert_eq!(
            error,
            SizeChartError::InvalidMember {
                observation: id(60),
                issue: SizeMembershipError::MissingMember {
                    set: id(40),
                    member: id(42)
                }
            }
        );
        assert!(error.source().is_some());
    });
}
#[test]
fn reorder_and_relabel_preserve_member_identity_without_position_lookup() {
    let mut f = Fixture::new();
    let o = f.observation();
    let mut d = f.membership.definition().clone();
    d.members[0].label = SizeLabel::new("New M".to_owned()).unwrap();
    d.members.reverse();
    f.membership = SizeMembership::new(d).unwrap();
    f.with_context(|c| {
        assert_eq!(o.member(c).unwrap().id, id(42));
        assert_eq!(o.member(c).unwrap().label.as_str(), "New M");
        assert_eq!(o.authored_value(c).unwrap(), length(780_000));
    });
}
#[test]
fn same_content_table_peer_does_not_replace_named_table() {
    let mut f = Fixture::new();
    let o = f.observation();
    let mut d = f.tables[1].definition().clone();
    d.id = id(52);
    let r = f.records();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    f.tables[1] = MeasurementTable::new(d, &m).unwrap();
    f.with_context(|c| {
        assert_eq!(
            o.validate_current(c).err(),
            Some(SizeChartError::MissingTable {
                observation: id(60),
                role: SizeChartRole::Observation,
                table: id(51),
            })
        )
    });
}
#[test]
fn table_membership_and_exact_metadata_identity_are_required() {
    for absent_metadata in [false, true] {
        let mut f = Fixture::new();
        let o = f.observation();
        if absent_metadata {
            let mut d = f.measurements[1].definition().clone();
            d.id = id(3);
            f.replace_measurement(1, d);
        } else {
            let mut d = f.tables[1].definition().clone();
            d.entries.clear();
            let r = f.records();
            let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
            f.tables[1] = MeasurementTable::new(d, &m).unwrap();
        }
        f.with_context(|c| {
            let issue = if absent_metadata {
                MeasurementTableError::MissingMeasurement {
                    table: id(51),
                    measurement: id(2),
                }
            } else {
                MeasurementTableError::MissingBinding {
                    table: id(51),
                    measurement: id(2),
                }
            };
            assert_eq!(
                o.validate_current(c).err(),
                Some(SizeChartError::InvalidTable {
                    observation: id(60),
                    role: SizeChartRole::Observation,
                    issue: Box::new(issue),
                })
            );
        });
    }
}
#[test]
fn explicit_table_rebinding_does_not_silently_rebind_an_observation() {
    for index in 0..2 {
        for change in 0..3 {
            let mut f = Fixture::new();
            let o = f.observation();
            let mut d = f.measurements[index].definition().clone();
            match change {
                0 => d.token = MachineToken::new("renamed_waist").unwrap(),
                1 => {
                    d.kind = MeasurementKind::Body;
                    d.landmarks = [id(22), id(23)];
                    d.procedure = id(31);
                }
                _ => d.declaration = if index == 0 { id(11) } else { id(10) },
            }
            f.replace_measurement(index, d);
            f.rebuild_tables();
            f.with_context(|c| {
                let role = if index == 0 {
                    SizeChartRole::DesignPom
                } else {
                    SizeChartRole::Observation
                };
                let expected = if index == 0 {
                    o.definition().pom.clone()
                } else {
                    o.definition().measurement.clone()
                };
                assert_eq!(
                    o.validate_current(c).err(),
                    Some(SizeChartError::ReassignedBinding {
                        observation: id(60),
                        role,
                        expected: Box::new(expected),
                        actual: Box::new(MeasurementBinding::from(&f.measurements[index])),
                    })
                );
                if change != 1 {
                    let replacement = SizeChartObservation::new(f.definition(), c).unwrap();
                    assert_ne!(replacement.definition(), o.definition());
                    assert!(replacement.validate_current(c).is_ok());
                    assert!(o.validate_current(c).is_err());
                }
            });
        }
    }
}
#[test]
fn stale_table_binding_preserves_underlying_diagnostic() {
    let mut f = Fixture::new();
    let o = f.observation();
    let mut d = f.measurements[1].definition().clone();
    d.token = MachineToken::new("renamed_waist").unwrap();
    f.replace_measurement(1, d);
    f.with_context(|c| {
        let e = o.validate_current(c).err().unwrap();
        assert!(matches!(&e, SizeChartError::InvalidTable { role: SizeChartRole::Observation, issue, .. }
            if matches!(issue.as_ref(), MeasurementTableError::TokenMismatch { measurement, .. } if *measurement == id(2))));
        assert!(e.source().is_some());
    });
}
#[test]
fn every_required_current_metadata_target_is_checked() {
    for missing in 0..3 {
        let mut f = Fixture::new();
        let o = f.observation();
        let issue = match missing {
            0 => {
                f.declarations.remove(1);
                MeasurementError::MissingDeclaration(id(11))
            }
            1 => {
                f.landmarks.remove(0);
                MeasurementError::MissingLandmark(id(20))
            }
            _ => {
                f.procedures.remove(0);
                MeasurementError::MissingProcedure(id(30))
            }
        };
        f.with_context(|c| {
            let role = if missing == 0 {
                SizeChartRole::Observation
            } else {
                SizeChartRole::DesignPom
            };
            let (table, measurement) = if missing == 0 {
                (id(51), id(2))
            } else {
                (id(50), id(1))
            };
            let e = o.validate_current(c).err().unwrap();
            assert_eq!(
                e,
                SizeChartError::InvalidTable {
                    observation: id(60),
                    role,
                    issue: Box::new(MeasurementTableError::InvalidMeasurement {
                        table,
                        measurement,
                        issue
                    })
                }
            );
            assert!(e.source().unwrap().source().is_some());
        });
    }
}
#[test]
fn canonical_state_source_and_metadata_edits_are_visible_without_caches() {
    let mut f = Fixture::new();
    let o = f.observation();
    let mut d = f.declarations[1].definition().clone();
    d.source = id(95);
    d.state = LengthState::Preference {
        value: length(790_001),
        provenance: id(96),
    };
    f.declarations[1] = LengthDeclaration::new(d).unwrap();
    let mut d = f.measurements[1].definition().clone();
    d.name = "Current observation".to_owned();
    d.entered_unit = Unit::Millimetre;
    f.replace_measurement(1, d);
    let mut d = f.procedures[0].definition().clone();
    d.documentation = "Current authored procedure text.".to_owned();
    f.procedures[0] = MeasurementProcedure::new(d).unwrap();
    f.declarations.reverse();
    f.measurements.reverse();
    f.tables.reverse();
    f.with_context(|c| {
        assert_eq!(o.authored_value(c).unwrap(), length(790_001));
        assert_eq!(o.declaration(c).unwrap().source(), id(95));
        assert_eq!(
            o.declaration(c).unwrap().state(),
            &LengthState::Preference {
                value: length(790_001),
                provenance: id(96)
            }
        );
        let m = o.measurement(SizeChartRole::Observation, c).unwrap();
        assert_eq!(m.definition().entered_unit, Unit::Millimetre);
        assert_eq!(m.definition().name, "Current observation");
        assert_eq!(
            m.procedure(&f.records())
                .unwrap()
                .definition()
                .documentation,
            "Current authored procedure text."
        );
    });
}
#[test]
fn unknown_and_derived_chart_values_refuse_numeric_fallback() {
    for (state, issue) in [
        (
            LengthState::Unknown {
                observation: id(97),
            },
            LengthValueError::RequiresObservation {
                declaration: id(11),
                observation: id(97),
            },
        ),
        (
            LengthState::Derived { formula: id(98) },
            LengthValueError::RequiresEvaluation {
                declaration: id(11),
                formula: id(98),
            },
        ),
    ] {
        let mut f = Fixture::new();
        let o = f.observation();
        f.declarations[1] = scalar(11, state.clone());
        f.with_context(|c| {
            assert!(o.validate_current(c).is_ok());
            assert_eq!(o.declaration(c).unwrap().state(), &state);
            let e = o.authored_value(c).err().unwrap();
            assert_eq!(
                e,
                SizeChartError::UnavailableValue {
                    observation: id(60),
                    issue
                }
            );
            assert!(e.source().is_some());
            assert!(SizeChartObservation::new(f.definition(), c).is_ok());
        });
    }
}
#[test]
fn authored_base_input_can_serve_both_roles_without_duplicating_the_scalar() {
    let f = Fixture::new();
    let mut d = f.definition();
    d.member = id(41);
    d.chart_table = d.design_table;
    d.measurement = d.pom.clone();
    f.with_context(|c| {
        let o = SizeChartObservation::new(d, c).unwrap();
        assert!(std::ptr::eq(
            o.measurement(SizeChartRole::DesignPom, c).unwrap(),
            o.measurement(SizeChartRole::Observation, c).unwrap()
        ));
        assert!(std::ptr::eq(o.declaration(c).unwrap(), &f.declarations[0]));
        assert_eq!(o.authored_value(c).unwrap(), length(740_000));
    });
}
#[test]
fn selected_role_query_does_not_certify_other_role_or_numeric_query() {
    let mut f = Fixture::new();
    let o = f.observation();
    f.tables.remove(0);
    f.with_context(|c| {
        assert!(o.measurement(SizeChartRole::Observation, c).is_ok());
        assert_eq!(
            o.declaration(c).err(),
            Some(SizeChartError::MissingTable {
                observation: id(60),
                role: SizeChartRole::DesignPom,
                table: id(50),
            })
        );
        assert!(o.validate_current(c).is_err());
        assert!(o.authored_value(c).is_err());
    });
}
#[test]
fn observation_identity_cannot_alias_any_supplied_semantic_record() {
    let f = Fixture::new();
    for n in [40, 41, 50, 1, 10, 20, 30] {
        let mut d = f.definition();
        d.id = id(n);
        f.with_context(|c| {
            assert_eq!(
                SizeChartObservation::new(d, c).err(),
                Some(SizeChartError::DuplicateIdentity(id(n)))
            )
        });
    }
}
#[test]
fn context_refuses_duplicate_tables_and_cross_kind_set_member_identities() {
    for collision in 0..5 {
        let mut f = Fixture::new();
        let expected = match collision {
            0 => {
                f.tables.push(f.tables[0].clone());
                id(50)
            }
            1 | 2 => {
                let n = if collision == 1 { 40 } else { 41 };
                let mut d = f.tables[0].definition().clone();
                d.id = id(n);
                let r = f.records();
                let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
                f.tables[0] = MeasurementTable::new(d, &m).unwrap();
                id(n)
            }
            3 => {
                f.declarations.push(scalar(40, assumed(0)));
                id(40)
            }
            _ => {
                let mut d = f.membership.definition().clone();
                d.members[0].id = id(1);
                f.membership = SizeMembership::new(d).unwrap();
                id(1)
            }
        };
        let r = f.records();
        let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        assert_eq!(
            SizeChartContext::new(&f.membership, &f.tables, &m).err(),
            Some(SizeChartError::DuplicateIdentity(expected))
        );
    }
}
