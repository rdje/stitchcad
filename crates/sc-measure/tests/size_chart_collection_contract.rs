//! Exact chart coverage, canonical correspondence and authored-order contracts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError},
};
use sc_measure::{
    Landmark, LandmarkDefinition, Measurement, MeasurementBinding, MeasurementContext,
    MeasurementDefinition, MeasurementKind, MeasurementProcedure, MeasurementProcedureDefinition,
    MeasurementTable, MeasurementTableContext, MeasurementTableDefinition, MeasurementTableError,
    SizeChart, SizeChartBinding, SizeChartCollectionContext, SizeChartCollectionError as E,
    SizeChartContext, SizeChartDefinition, SizeChartError, SizeChartObservation,
    SizeChartObservationDefinition, SizeLabel, SizeMember, SizeMembership,
    SizeMembershipDefinition, SizeMembershipError, SizeSetReference, SizeSystem,
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
    observations: Vec<SizeChartObservation>,
}
impl Fixture {
    fn new() -> Self {
        let mut f = Self {
            declarations: (10..15)
                .map(|n| scalar(n, assumed(i64::try_from(n).unwrap() * 10_000)))
                .collect(),
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
            tables: vec![],
            observations: vec![],
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
        };
        let r = f.records();
        f.measurements = (1..6)
            .map(|n| {
                Measurement::new(
                    MeasurementDefinition {
                        id: id(n),
                        name: format!("Input {n}"),
                        token: MachineToken::new(if n == 5 {
                            "body_length"
                        } else if n % 2 == 1 {
                            "waist"
                        } else {
                            "hip"
                        })
                        .unwrap(),
                        entered_unit: Unit::Centimetre,
                        kind: if n == 5 {
                            MeasurementKind::Body
                        } else {
                            MeasurementKind::Garment
                        },
                        landmarks: if n == 5 {
                            [id(22), id(23)]
                        } else {
                            [id(20), id(21)]
                        },
                        procedure: if n == 5 { id(31) } else { id(30) },
                        declaration: id(n + 9),
                    },
                    &r,
                )
                .unwrap()
            })
            .collect();
        f.rebuild_tables();
        for (index, (member, pom)) in [(41, 1), (42, 0), (41, 0), (42, 1)].into_iter().enumerate() {
            let d = SizeChartObservationDefinition {
                id: id(60 + index as u128),
                membership: f.membership.reference(),
                member: id(member),
                design_table: id(50),
                chart_table: id(51),
                pom: MeasurementBinding::from(&f.measurements[pom]),
                measurement: MeasurementBinding::from(&f.measurements[pom + 2]),
                provenance: id(94 + index as u128),
            };
            f.observations.push(f.make_observation(d));
        }
        f
    }
    fn records(&self) -> MeasurementContext<'_> {
        MeasurementContext::new(&self.declarations, &self.landmarks, &self.procedures).unwrap()
    }
    fn with_context<R>(&self, callback: impl FnOnce(&SizeChartCollectionContext<'_>) -> R) -> R {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        let c = SizeChartContext::new(&self.membership, &self.tables, &m).unwrap();
        let ctx = SizeChartCollectionContext::new(&self.observations, &c).unwrap();
        callback(&ctx)
    }
    fn make_observation(&self, d: SizeChartObservationDefinition) -> SizeChartObservation {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        let c = SizeChartContext::new(&self.membership, &self.tables, &m).unwrap();
        SizeChartObservation::new(d, &c).unwrap()
    }
    fn definition(&self) -> SizeChartDefinition {
        SizeChartDefinition {
            id: id(70),
            membership: self.membership.reference(),
            design_table: id(50),
            poms: vec![
                MeasurementBinding::from(&self.measurements[1]),
                MeasurementBinding::from(&self.measurements[0]),
            ],
            observations: self
                .observations
                .iter()
                .map(SizeChartBinding::from)
                .collect(),
        }
    }
    fn chart(&self) -> SizeChart {
        self.with_context(|c| SizeChart::new(self.definition(), c).unwrap())
    }
    fn rebuild_tables(&mut self) {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        self.tables = vec![
            MeasurementTable::new(
                MeasurementTableDefinition {
                    id: id(50),
                    name: "Design inputs".to_owned(),
                    entries: [0, 1, 4]
                        .map(|i| MeasurementBinding::from(&self.measurements[i]))
                        .to_vec(),
                },
                &m,
            )
            .unwrap(),
            MeasurementTable::new(
                MeasurementTableDefinition {
                    id: id(51),
                    name: "Authored chart inputs".to_owned(),
                    entries: [2, 3]
                        .map(|i| MeasurementBinding::from(&self.measurements[i]))
                        .to_vec(),
                },
                &m,
            )
            .unwrap(),
        ];
    }
    fn replace_measurement(&mut self, index: usize, d: MeasurementDefinition) {
        self.measurements[index] = Measurement::new(d, &self.records()).unwrap();
    }
}
#[test]
fn complete_chart_preserves_member_and_pom_order_and_borrows_canonical_inputs() {
    let f = Fixture::new();
    let chart = f.chart();
    f.with_context(|c| {
        assert_eq!(chart.id(), id(70));
        assert!(chart.validate_complete(c).is_ok());
        assert!(std::ptr::eq(
            chart.members(c).unwrap(),
            f.membership.definition().members.as_slice()
        ));
        assert_eq!(
            chart
                .members(c)
                .unwrap()
                .iter()
                .map(|m| m.id)
                .collect::<Vec<_>>(),
            vec![id(42), id(41)]
        );
        let row = chart.observations_for_member(id(42), c).unwrap();
        assert_eq!(
            row.iter().map(|o| o.id()).collect::<Vec<_>>(),
            vec![id(63), id(61)]
        );
        assert!(std::ptr::eq(
            chart.observation(id(42), id(1), c).unwrap(),
            &f.observations[1]
        ));
        assert!(std::ptr::eq(
            chart.declaration(id(42), id(1), c).unwrap(),
            &f.declarations[2]
        ));
        assert_eq!(
            chart.authored_value(id(42), id(1), c).unwrap(),
            length(120_000)
        );
    });
}
#[test]
fn incomplete_draft_refuses_exact_missing_cell_without_interpolation() {
    let f = Fixture::new();
    let mut d = f.definition();
    d.observations.remove(1);
    f.with_context(|c| {
        let chart = SizeChart::new(d, c).unwrap();
        assert!(chart.validate_current(c).is_ok());
        let error = E::MissingCell {
            chart: id(70),
            member: id(42),
            pom: id(1),
        };
        assert_eq!(chart.validate_complete(c).err(), Some(error.clone()));
        assert_eq!(
            chart.observations_for_member(id(42), c).err(),
            Some(error.clone())
        );
        assert_eq!(chart.authored_value(id(42), id(1), c).err(), Some(error));
        assert!(chart.observations_for_member(id(41), c).is_ok());
    });
}
#[test]
fn reduced_target_list_cannot_certify_omitted_design_pom() {
    let f = Fixture::new();
    let mut d = f.definition();
    d.poms.retain(|p| p.measurement == id(1));
    d.observations.retain(|o| o.pom.measurement == id(1));
    f.with_context(|c| {
        let chart = SizeChart::new(d, c).unwrap();
        assert!(chart.validate_current(c).is_ok());
        assert_eq!(
            chart.validate_complete(c).err(),
            Some(E::MissingPom {
                chart: id(70),
                pom: id(2)
            })
        );
        assert_eq!(chart.observations_for_member(id(42), c).unwrap().len(), 1);
    });
}
#[test]
fn empty_inventory_is_inspectable_but_not_complete_and_keeps_table_revision_checks() {
    let mut f = Fixture::new();
    let mut d = f.definition();
    d.poms.clear();
    d.observations.clear();
    let chart = f.with_context(|c| {
        let chart = SizeChart::new(d, c).unwrap();
        assert!(chart.validate_current(c).is_ok());
        assert_eq!(chart.validate_complete(c).err(), Some(E::EmptyPoms(id(70))));
        assert!(chart.observations_for_member(id(42), c).unwrap().is_empty());
        chart
    });
    let mut table = f.tables[0].definition().clone();
    table
        .entries
        .retain(|entry| entry.kind == MeasurementKind::Body);
    let r = f.records();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    f.tables[0] = MeasurementTable::new(table, &m).unwrap();
    f.with_context(|c| assert_eq!(chart.validate_complete(c).err(), Some(E::EmptyPoms(id(70)))));
    f.tables.remove(0);
    f.with_context(|c| {
        assert_eq!(
            chart.validate_current(c).err(),
            Some(E::MissingTable {
                chart: id(70),
                table: id(50)
            })
        )
    });
}
#[test]
fn duplicate_pom_token_observation_and_valid_same_cell_are_refused() {
    for case in 0..4 {
        let mut f = Fixture::new();
        let mut d = f.definition();
        let expected = match case {
            0 => {
                d.poms.push(d.poms[0].clone());
                E::DuplicatePom {
                    chart: id(70),
                    pom: id(2),
                }
            }
            1 => {
                d.poms[1].token = d.poms[0].token.clone();
                E::DuplicateToken {
                    chart: id(70),
                    token: d.poms[0].token.clone(),
                }
            }
            2 => {
                d.observations.push(d.observations[0].clone());
                E::DuplicateIdentity(id(60))
            }
            _ => {
                let mut obs = f.observations[0].definition().clone();
                obs.id = id(65);
                obs.provenance = id(99);
                let obs = f.make_observation(obs);
                d.observations.push(SizeChartBinding::from(&obs));
                f.observations.push(obs);
                E::DuplicateCell {
                    chart: id(70),
                    member: id(41),
                    pom: id(2),
                }
            }
        };
        f.with_context(|c| assert_eq!(SizeChart::new(d, c).err(), Some(expected)));
    }
}
#[test]
fn authored_target_body_domain_is_refused_even_when_current_table_is_valid() {
    let f = Fixture::new();
    let mut d = f.definition();
    d.poms = vec![MeasurementBinding::from(&f.measurements[4])];
    d.observations.clear();
    f.with_context(|c| {
        assert_eq!(
            SizeChart::new(d, c).err(),
            Some(E::WrongKind {
                chart: id(70),
                pom: id(5),
                actual: MeasurementKind::Body
            })
        )
    });
}
#[test]
fn unknown_and_derived_cells_can_be_structurally_complete_but_never_numeric_defaults() {
    for (state, issue) in [
        (
            LengthState::Unknown {
                observation: id(100),
            },
            LengthValueError::RequiresObservation {
                declaration: id(12),
                observation: id(100),
            },
        ),
        (
            LengthState::Derived { formula: id(101) },
            LengthValueError::RequiresEvaluation {
                declaration: id(12),
                formula: id(101),
            },
        ),
    ] {
        let mut f = Fixture::new();
        let chart = f.chart();
        f.declarations[2] = scalar(12, state.clone());
        f.with_context(|c| {
            assert!(chart.validate_complete(c).is_ok());
            assert_eq!(chart.declaration(id(42), id(1), c).unwrap().state(), &state);
            let value = chart.authored_value(id(42), id(1), c);
            assert!(value.is_err());
            let e = value.err().unwrap();
            assert_eq!(
                e,
                E::InvalidObservation {
                    chart: id(70),
                    observation: id(61),
                    issue: Box::new(SizeChartError::UnavailableValue {
                        observation: id(61),
                        issue
                    })
                }
            );
            assert!(e.source().unwrap().source().is_some());
        });
    }
}
#[test]
fn same_cell_peer_does_not_replace_missing_canonical_observation() {
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.observations[1].definition().clone();
    d.id = id(65);
    f.observations[1] = f.make_observation(d);
    f.with_context(|c| {
        assert_eq!(
            chart.observation(id(42), id(1), c).err(),
            Some(E::MissingObservation {
                chart: id(70),
                observation: id(61)
            })
        )
    });
}
#[test]
fn all_saved_observation_targets_require_explicit_collection_replacement() {
    for case in 0..6 {
        let mut f = Fixture::new();
        let chart = f.chart();
        let mut d = f.observations[1].definition().clone();
        match case {
            0 => d.member = id(41),
            1 => d.chart_table = id(52),
            2 => {
                d.pom = MeasurementBinding::from(&f.measurements[1]);
                d.measurement = MeasurementBinding::from(&f.measurements[3]);
            }
            3 => {
                d.measurement = d.pom.clone();
                d.chart_table = d.design_table;
            }
            4 => d.design_table = id(52),
            _ => d.membership = d.membership.next_revision().unwrap(),
        }
        if case == 1 || case == 4 {
            let index = if case == 1 { 1 } else { 0 };
            let mut table = f.tables[index].definition().clone();
            table.id = id(52);
            let r = f.records();
            let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
            f.tables.push(MeasurementTable::new(table, &m).unwrap());
        }
        let original_membership = f.membership.clone();
        if case == 5 {
            let mut membership = f.membership.definition().clone();
            membership.reference = d.membership;
            f.membership = SizeMembership::new(membership).unwrap();
        }
        f.observations[1] = f.make_observation(d);
        f.membership = original_membership;
        f.with_context(|c| {
            assert_eq!(
                chart.observation(id(42), id(1), c).err(),
                Some(E::ReassignedObservation {
                    chart: id(70),
                    expected: Box::new(chart.definition().observations[1].clone()),
                    actual: Box::new(SizeChartBinding::from(&f.observations[1]))
                })
            )
        });
    }
}
#[test]
fn provenance_and_canonical_source_state_edits_remain_visible() {
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.observations[1].definition().clone();
    d.provenance = id(102);
    f.observations[1] = f.make_observation(d);
    let mut d = f.declarations[2].definition().clone();
    d.source = id(103);
    d.state = LengthState::Preference {
        value: length(120_003),
        provenance: id(104),
    };
    f.declarations[2] = LengthDeclaration::new(d).unwrap();
    f.observations.reverse();
    f.tables.reverse();
    f.with_context(|c| {
        assert!(chart.validate_complete(c).is_ok());
        assert_eq!(
            chart
                .observation(id(42), id(1), c)
                .unwrap()
                .definition()
                .provenance,
            id(102)
        );
        assert_eq!(
            chart.declaration(id(42), id(1), c).unwrap().source(),
            id(103)
        );
        assert_eq!(
            chart.authored_value(id(42), id(1), c).unwrap(),
            length(120_003)
        );
    });
}
#[test]
fn changed_pom_binding_needs_explicit_chart_and_observation_replacement() {
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.measurements[0].definition().clone();
    d.token = MachineToken::new("new_waist").unwrap();
    f.replace_measurement(0, d);
    f.rebuild_tables();
    for index in [1, 2] {
        let mut d = f.observations[index].definition().clone();
        d.pom = MeasurementBinding::from(&f.measurements[0]);
        f.observations[index] = f.make_observation(d);
    }
    f.with_context(|c| {
        assert_eq!(
            chart.validate_current(c).err(),
            Some(E::ReassignedPom {
                chart: id(70),
                expected: Box::new(chart.definition().poms[1].clone()),
                actual: Box::new(MeasurementBinding::from(&f.measurements[0]))
            })
        );
        let replacement = SizeChart::new(f.definition(), c).unwrap();
        assert!(replacement.validate_complete(c).is_ok());
        assert_ne!(chart.definition(), replacement.definition());
        assert!(chart.validate_current(c).is_err());
    });
}
#[test]
fn exact_membership_reference_and_stable_member_are_required() {
    for case in 0..3 {
        let mut f = Fixture::new();
        let chart = f.chart();
        let mut d = f.membership.definition().clone();
        match case {
            0 => d.reference.id = id(43),
            1 => d.reference.revision = Count::new(3),
            _ => d.members[0].id = id(43),
        };
        f.membership = SizeMembership::new(d).unwrap();
        f.with_context(|c| {
            if case < 2 {
                assert_eq!(
                    chart.validate_current(c).err(),
                    Some(E::MembershipMismatch {
                        chart: id(70),
                        expected: chart.definition().membership,
                        actual: f.membership.reference()
                    })
                );
            } else {
                assert_eq!(
                    chart.observation(id(42), id(1), c).err(),
                    Some(E::InvalidMember {
                        chart: id(70),
                        issue: SizeMembershipError::MissingMember {
                            set: id(40),
                            member: id(42)
                        }
                    })
                );
            }
        });
    }
}
#[test]
fn undeclared_pom_and_observation_from_another_design_table_are_refused() {
    let mut f = Fixture::new();
    let mut d = f.definition();
    d.poms.retain(|p| p.measurement == id(1));
    f.with_context(|c| {
        assert_eq!(
            SizeChart::new(d, c).err(),
            Some(E::MissingPom {
                chart: id(70),
                pom: id(2)
            })
        )
    });
    let mut table = f.tables[0].definition().clone();
    table.id = id(52);
    let r = f.records();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    f.tables.push(MeasurementTable::new(table, &m).unwrap());
    let mut d = f.observations[0].definition().clone();
    d.design_table = id(52);
    f.observations[0] = f.make_observation(d);
    f.with_context(|c| {
        assert_eq!(
            SizeChart::new(f.definition(), c).err(),
            Some(E::WrongDesignTable {
                chart: id(70),
                observation: id(60),
                expected: id(50),
                actual: id(52)
            })
        )
    });
}
#[test]
fn selected_row_does_not_certify_broken_other_rows_or_full_design_table() {
    let mut f = Fixture::new();
    let chart = f.chart();
    f.observations.remove(0);
    f.with_context(|c| {
        assert!(chart.observations_for_member(id(42), c).is_ok());
        assert_eq!(
            chart.validate_complete(c).err(),
            Some(E::MissingObservation {
                chart: id(70),
                observation: id(60)
            })
        );
    });
    let mut f = Fixture::new();
    let chart = f.chart();
    f.declarations.remove(4);
    f.with_context(|c| {
        assert!(chart.validate_current(c).is_ok());assert!(chart.observations_for_member(id(42),c).is_ok());
        let e=chart.validate_complete(c).err().unwrap();
        assert!(matches!(&e,E::InvalidTable {issue,..} if matches!(issue.as_ref(),MeasurementTableError::InvalidMeasurement {measurement,..} if *measurement==id(5))));
        assert!(e.source().unwrap().source().is_some());
    });
}
#[test]
fn shared_measurement_inputs_need_explicit_member_specific_correspondences() {
    let f = Fixture::new();
    let chart = f.chart();
    f.with_context(|c| {
        assert!(chart.validate_complete(c).is_ok());
        let a = chart.observation(id(42), id(1), c).unwrap();
        let b = chart.observation(id(41), id(1), c).unwrap();
        assert_ne!(a.id(), b.id());
        assert_ne!(a.definition().provenance, b.definition().provenance);
        assert!(std::ptr::eq(
            chart.declaration(id(42), id(1), c).unwrap(),
            chart.declaration(id(41), id(1), c).unwrap()
        ));
        let mut d = chart.definition().clone();
        d.id = id(71);
        let peer = SizeChart::new(d, c).unwrap();
        assert!(std::ptr::eq(peer.observation(id(42), id(1), c).unwrap(), a));
    });
}
#[test]
fn collection_and_context_identities_cannot_alias_supplied_records() {
    let f = Fixture::new();
    for n in [40, 41, 50, 1, 10, 20, 30, 60] {
        let mut d = f.definition();
        d.id = id(n);
        f.with_context(|c| {
            assert_eq!(
                SizeChart::new(d, c).err(),
                Some(E::DuplicateIdentity(id(n)))
            )
        });
    }
    for alias in [false, true] {
        let mut f = Fixture::new();
        if alias {
            f.declarations.push(scalar(60, assumed(0)));
        } else {
            f.observations.push(f.observations[0].clone());
        }
        let r = f.records();
        let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        let c = SizeChartContext::new(&f.membership, &f.tables, &m).unwrap();
        assert_eq!(
            SizeChartCollectionContext::new(&f.observations, &c).err(),
            Some(E::DuplicateIdentity(id(60)))
        );
    }
}
#[test]
fn current_design_table_expansion_invalidates_prior_complete_coverage() {
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.measurements[0].definition().clone();
    d.id = id(6);
    d.token = MachineToken::new("skirt_length").unwrap();
    f.measurements
        .push(Measurement::new(d, &f.records()).unwrap());
    let mut table = f.tables[0].definition().clone();
    table
        .entries
        .push(MeasurementBinding::from(&f.measurements[5]));
    let r = f.records();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    f.tables[0] = MeasurementTable::new(table, &m).unwrap();
    f.with_context(|c| {
        assert!(chart.validate_current(c).is_ok());
        assert_eq!(
            chart.validate_complete(c).err(),
            Some(E::MissingPom {
                chart: id(70),
                pom: id(6)
            })
        );
    });
}
#[test]
fn missing_required_chart_scalar_preserves_scoped_observation_error() {
    let mut f = Fixture::new();
    let chart = f.chart();
    f.declarations.remove(2);
    f.with_context(|c| {
        assert!(chart.observation(id(42),id(2),c).is_ok());
        let e=chart.declaration(id(42),id(1),c).err().unwrap();
        assert!(matches!(&e,E::InvalidObservation {observation,issue,..} if *observation==id(61) && matches!(issue.as_ref(),SizeChartError::InvalidTable {issue,..} if matches!(issue.as_ref(),MeasurementTableError::InvalidMeasurement {measurement,..} if *measurement==id(3)))));
        assert!(e.source().unwrap().source().unwrap().source().is_some());
        assert!(chart.validate_complete(c).is_err());
    });
}
