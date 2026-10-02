//! Custom member-of-one, current body/Ease correspondence and structural coverage contracts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError},
};
use sc_measure::{
    CompressionPermission, Ease, EaseBinding, EaseDefinition, EaseError, EaseSet, EaseSetContext,
    EaseSetDefinition, EaseSetError, FitIntent, Landmark, LandmarkDefinition, Measurement,
    MeasurementBinding, MeasurementContext, MeasurementDefinition, MeasurementKind,
    MeasurementProcedure, MeasurementProcedureDefinition, MeasurementTable,
    MeasurementTableContext, MeasurementTableDefinition, MtmChart, MtmChartContext,
    MtmChartDefinition, MtmChartError as E, SizeLabel, SizeMember, SizeMembership,
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
fn assumed(n: i64) -> LengthState {
    LengthState::Assumed {
        value: length(n),
        assumption: id(90),
    }
}
fn scalar(n: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(n),
        source: id(91),
        state,
    })
    .unwrap()
}
struct Fixture {
    declarations: Vec<LengthDeclaration>,
    landmarks: Vec<Landmark>,
    procedures: Vec<MeasurementProcedure>,
    measurements: Vec<Measurement>,
    tables: Vec<MeasurementTable>,
    eases: Vec<Ease>,
    sets: Vec<EaseSet>,
    membership: SizeMembership,
}
impl Fixture {
    fn new() -> Self {
        let mut f = Self {
            declarations: vec![
                scalar(10, assumed(740_000)),
                scalar(11, assumed(780_000)),
                scalar(12, LengthState::Derived { formula: id(92) }),
                scalar(13, assumed(40_000)),
                scalar(14, assumed(60_000)),
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
                        source: id(93),
                    })
                    .unwrap()
                })
                .collect(),
            procedures: (24..26)
                .map(|n| {
                    MeasurementProcedure::new(MeasurementProcedureDefinition {
                        id: id(n),
                        name: format!("Fixture {n}"),
                        kind: if n == 24 {
                            MeasurementKind::Body
                        } else {
                            MeasurementKind::Garment
                        },
                        source: id(94),
                        documentation: "Authored fixture, not physical certification.".to_owned(),
                    })
                    .unwrap()
                })
                .collect(),
            measurements: vec![],
            tables: vec![],
            eases: vec![],
            sets: vec![],
            membership: SizeMembership::new(SizeMembershipDefinition {
                reference: SizeSetReference {
                    id: id(40),
                    revision: Count::new(2),
                },
                system: SizeSystem::Custom,
                members: vec![SizeMember {
                    id: id(41),
                    label: SizeLabel::new("Client A".to_owned()).unwrap(),
                }],
                base: id(41),
            })
            .unwrap(),
        };
        let r = f.records();
        f.measurements = (1..4)
            .map(|n| {
                Measurement::new(
                    MeasurementDefinition {
                        id: id(n),
                        name: format!("Fixture input {n}"),
                        token: MachineToken::new(match n {
                            1 => "body_waist",
                            2 => "garment_waist",
                            _ => "waistband",
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
                        procedure: if n == 1 { id(24) } else { id(25) },
                        declaration: id(n + 9),
                    },
                    &r,
                )
                .unwrap()
            })
            .collect();
        let r = f.records();
        let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        let tables = vec![
            MeasurementTable::new(
                MeasurementTableDefinition {
                    id: id(50),
                    name: "Body inputs".to_owned(),
                    entries: vec![MeasurementBinding::from(&f.measurements[0])],
                },
                &m,
            )
            .unwrap(),
            MeasurementTable::new(
                MeasurementTableDefinition {
                    id: id(51),
                    name: "Design POMs".to_owned(),
                    entries: [2, 1]
                        .map(|i| MeasurementBinding::from(&f.measurements[i]))
                        .to_vec(),
                },
                &m,
            )
            .unwrap(),
        ];
        let eases = (0..2)
            .map(|n| {
                Ease::new(
                    EaseDefinition {
                        id: id(30 + n as u128),
                        body: MeasurementBinding::from(&f.measurements[0]),
                        garment: MeasurementBinding::from(&f.measurements[n + 1]),
                        declaration: id(13 + n as u128),
                        fit: FitIntent::Semi,
                        provenance: id(95 + n as u128),
                        compression: CompressionPermission::Forbidden,
                    },
                    &m,
                )
                .unwrap()
            })
            .collect();
        f.tables = tables;
        f.eases = eases;
        f.sets = vec![f.make_set(EaseSetDefinition {
            id: id(60),
            body_table: id(50),
            garment_table: id(51),
            entries: f
                .eases
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    EaseBinding::new(
                        MachineToken::new(if i == 0 {
                            "waist_ease"
                        } else {
                            "waistband_ease"
                        })
                        .unwrap(),
                        e,
                    )
                })
                .collect(),
        })];
        f
    }
    fn records(&self) -> MeasurementContext<'_> {
        MeasurementContext::new(&self.declarations, &self.landmarks, &self.procedures).unwrap()
    }
    fn with_context<R>(&self, callback: impl FnOnce(&MtmChartContext<'_>) -> R) -> R {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        let e = EaseSetContext::new(&self.tables, &self.eases, &m).unwrap();
        let c = MtmChartContext::new(&self.membership, &self.sets, &e).unwrap();
        callback(&c)
    }
    fn make_set(&self, d: EaseSetDefinition) -> EaseSet {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        let e = EaseSetContext::new(&self.tables, &self.eases, &m).unwrap();
        EaseSet::new(d, &e).unwrap()
    }
    fn make_ease(&self, d: EaseDefinition) -> Ease {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        Ease::new(d, &m).unwrap()
    }
    fn definition(&self) -> MtmChartDefinition {
        MtmChartDefinition {
            id: id(70),
            membership: self.membership.reference(),
            member: id(41),
            ease_set: self.sets[0].definition().clone(),
            provenance: id(100),
        }
    }
    fn chart(&self) -> MtmChart {
        self.with_context(|c| MtmChart::new(self.definition(), c).unwrap())
    }
    fn add_table_peer(&mut self, index: usize, n: u128) {
        let mut d = self.tables[index].definition().clone();
        d.id = id(n);
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        self.tables.push(MeasurementTable::new(d, &m).unwrap());
    }
}
#[test]
fn sole_member_inputs_and_canonical_mapping_are_borrowed_without_garment_evaluation() {
    let f = Fixture::new();
    let chart = f.chart();
    f.with_context(|c| {
        assert_eq!(chart.id(), id(70));
        assert_eq!(chart.definition().provenance, id(100));
        assert!(chart.validate_complete(c).is_ok());
        assert!(std::ptr::eq(
            chart.member(c).unwrap(),
            &f.membership.definition().members[0]
        ));
        assert!(std::ptr::eq(chart.ease_set(c).unwrap(), &f.sets[0]));
        assert!(std::ptr::eq(
            chart.ease_for_pom(id(2), c).unwrap(),
            &f.eases[0]
        ));
        assert!(std::ptr::eq(
            chart.garment_pom(id(3), c).unwrap(),
            &f.measurements[2]
        ));
        assert!(std::ptr::eq(
            chart.body_declaration(id(2), c).unwrap(),
            &f.declarations[0]
        ));
        assert!(std::ptr::eq(
            chart.ease_declaration(id(2), c).unwrap(),
            &f.declarations[3]
        ));
        assert_eq!(chart.body_value(id(2), c).unwrap(), length(740_000));
        assert_eq!(chart.ease_value(id(2), c).unwrap(), length(40_000));
        assert_eq!(
            chart.garment_pom(id(3), c).unwrap().definition().kind,
            MeasurementKind::Garment
        );
    });
}
#[test]
fn mtm_requires_custom_system_and_exactly_one_member() {
    for system in [
        SizeSystem::En13402,
        SizeSystem::AstmD5585,
        SizeSystem::Numeric,
        SizeSystem::Alphanumeric,
    ] {
        let mut f = Fixture::new();
        let chart = f.chart();
        let mut d = f.membership.definition().clone();
        d.system = system;
        f.membership = SizeMembership::new(d).unwrap();
        f.with_context(|c| {
            assert_eq!(
                chart.validate_current(c).err(),
                Some(E::WrongSystem {
                    chart: id(70),
                    actual: system
                })
            )
        });
    }
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.membership.definition().clone();
    d.members.push(SizeMember {
        id: id(42),
        label: SizeLabel::new("Client B".to_owned()).unwrap(),
    });
    f.membership = SizeMembership::new(d).unwrap();
    f.with_context(|c| {
        assert_eq!(
            chart.validate_current(c).err(),
            Some(E::WrongMemberCount {
                chart: id(70),
                actual: 2
            })
        )
    });
}
#[test]
fn exact_set_revision_and_member_identity_are_required_not_matching_labels() {
    for case in 0..3 {
        let mut f = Fixture::new();
        let chart = f.chart();
        let mut d = f.membership.definition().clone();
        match case {
            0 => d.reference.id = id(43),
            1 => d.reference.revision = Count::new(3),
            _ => {
                d.members[0].id = id(42);
                d.base = id(42);
            }
        };
        f.membership = SizeMembership::new(d).unwrap();
        f.with_context(|c| {
            let e = if case < 2 {
                E::MembershipMismatch {
                    chart: id(70),
                    expected: chart.definition().membership,
                    actual: f.membership.reference(),
                }
            } else {
                E::InvalidMember {
                    chart: id(70),
                    issue: SizeMembershipError::MissingMember {
                        set: id(40),
                        member: id(41),
                    },
                }
            };
            assert_eq!(chart.validate_current(c).err(), Some(e));
        });
    }
}
#[test]
fn same_content_set_peer_does_not_restore_a_missing_canonical_identity() {
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.sets[0].definition().clone();
    d.id = id(61);
    f.sets[0] = f.make_set(d);
    f.with_context(|c| {
        assert_eq!(
            chart.validate_current(c).err(),
            Some(E::MissingEaseSet {
                chart: id(70),
                ease_set: id(60)
            })
        )
    });
}
#[test]
fn every_set_target_and_authored_order_requires_explicit_chart_replacement() {
    for case in 0..5 {
        let mut f = Fixture::new();
        let chart = f.chart();
        let mut d = f.sets[0].definition().clone();
        match case {
            0 => {
                f.add_table_peer(0, 52);
                d.body_table = id(52);
            }
            1 => {
                f.add_table_peer(1, 52);
                d.garment_table = id(52);
            }
            2 => d.entries[0].token = MachineToken::new("new_ease_token").unwrap(),
            3 => d.entries.reverse(),
            _ => {
                let mut ease = f.eases[0].definition().clone();
                ease.declaration = id(14);
                f.eases[0] = f.make_ease(ease);
                d.entries[0] = EaseBinding::new(d.entries[0].token.clone(), &f.eases[0]);
            }
        }
        f.sets[0] = f.make_set(d);
        f.with_context(|c| {
            assert_eq!(
                chart.validate_current(c).err(),
                Some(E::ReassignedEaseSet {
                    chart: id(70),
                    expected: Box::new(chart.definition().ease_set.clone()),
                    actual: Box::new(f.sets[0].definition().clone())
                })
            );
            let replacement = MtmChart::new(f.definition(), c).unwrap();
            assert!(replacement.validate_complete(c).is_ok());
            assert_ne!(replacement.definition(), chart.definition());
            assert!(chart.validate_current(c).is_err());
        });
    }
}
#[test]
fn incomplete_and_empty_drafts_cannot_certify_design_pom_coverage() {
    let mut f = Fixture::new();
    let mut d = f.sets[0].definition().clone();
    d.entries.pop();
    f.sets[0] = f.make_set(d);
    let chart = f.chart();
    f.with_context(|c| {
        assert!(chart.validate_current(c).is_ok());
        assert!(chart.ease_for_pom(id(2), c).is_ok());
        assert_eq!(
            chart.validate_complete(c).err(),
            Some(E::InvalidEaseSet {
                chart: id(70),
                ease_set: id(60),
                issue: Box::new(EaseSetError::MissingPom {
                    set: id(60),
                    pom: id(3)
                })
            })
        );
    });
    let mut d = f.sets[0].definition().clone();
    d.entries.clear();
    f.sets[0] = f.make_set(d);
    let chart = f.chart();
    f.with_context(|c| {
        assert_eq!(
            chart.validate_complete(c).err(),
            Some(E::EmptyMappings(id(70)))
        )
    });
    let mut d = f.tables[1].definition().clone();
    d.entries.clear();
    let r = f.records();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    f.tables[1] = MeasurementTable::new(d, &m).unwrap();
    f.with_context(|c| {
        assert!(chart.validate_current(c).is_ok());
        assert_eq!(
            chart.validate_complete(c).err(),
            Some(E::EmptyMappings(id(70)))
        );
    });
}
#[test]
fn new_current_design_pom_invalidates_prior_complete_coverage() {
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.measurements[2].definition().clone();
    d.id = id(4);
    d.token = MachineToken::new("waist_finish").unwrap();
    f.measurements
        .push(Measurement::new(d, &f.records()).unwrap());
    let mut d = f.tables[1].definition().clone();
    d.entries.push(MeasurementBinding::from(&f.measurements[3]));
    let r = f.records();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    f.tables[1] = MeasurementTable::new(d, &m).unwrap();
    f.with_context(|c| {
        assert!(chart.validate_current(c).is_ok());
        assert_eq!(
            chart.validate_complete(c).err(),
            Some(E::InvalidEaseSet {
                chart: id(70),
                ease_set: id(60),
                issue: Box::new(EaseSetError::MissingPom {
                    set: id(60),
                    pom: id(4)
                })
            })
        );
    });
}
#[test]
fn unknown_and_derived_body_inputs_remain_inspectable_and_refuse_numeric_defaults() {
    for (state, issue) in [
        (
            LengthState::Unknown {
                observation: id(110),
            },
            LengthValueError::RequiresObservation {
                declaration: id(10),
                observation: id(110),
            },
        ),
        (
            LengthState::Derived { formula: id(111) },
            LengthValueError::RequiresEvaluation {
                declaration: id(10),
                formula: id(111),
            },
        ),
    ] {
        let mut f = Fixture::new();
        let chart = f.chart();
        f.declarations[0] = scalar(10, state.clone());
        f.with_context(|c| {
            assert!(chart.validate_complete(c).is_ok());
            assert!(MtmChart::new(f.definition(), c).is_ok());
            assert_eq!(chart.body_declaration(id(2), c).unwrap().state(), &state);
            let value = chart.body_value(id(2), c);
            assert!(value.is_err());
            let e = value.err().unwrap();
            assert_eq!(
                e,
                E::UnavailableBodyValue {
                    chart: id(70),
                    pom: id(2),
                    issue
                }
            );
            assert!(e.source().is_some());
        });
    }
}
#[test]
fn unknown_and_derived_ease_amounts_remain_distinct_unresolved_inputs() {
    for (state, issue) in [
        (
            LengthState::Unknown {
                observation: id(112),
            },
            LengthValueError::RequiresObservation {
                declaration: id(13),
                observation: id(112),
            },
        ),
        (
            LengthState::Derived { formula: id(113) },
            LengthValueError::RequiresEvaluation {
                declaration: id(13),
                formula: id(113),
            },
        ),
    ] {
        let mut f = Fixture::new();
        let chart = f.chart();
        f.declarations[3] = scalar(13, state.clone());
        f.with_context(|c| {
            assert!(chart.validate_complete(c).is_ok());
            assert_eq!(chart.ease_declaration(id(2), c).unwrap().state(), &state);
            assert_eq!(chart.body_value(id(2), c).unwrap(), length(740_000));
            let value = chart.ease_value(id(2), c);
            assert!(value.is_err());
            let e = value.err().unwrap();
            assert_eq!(
                e,
                E::InvalidEase {
                    chart: id(70),
                    ease: id(30),
                    issue: Box::new(EaseError::UnavailableValue {
                        ease: id(30),
                        issue
                    })
                }
            );
            assert!(e.source().unwrap().source().is_some());
        });
    }
}
#[test]
fn current_fit_provenance_source_and_state_remain_visible_without_set_snapshot_caches() {
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.eases[0].definition().clone();
    d.fit = FitIntent::Loose;
    d.provenance = id(114);
    f.eases[0] = f.make_ease(d);
    let mut d = f.declarations[0].definition().clone();
    d.source = id(115);
    d.state = LengthState::Preference {
        value: length(750_001),
        provenance: id(116),
    };
    f.declarations[0] = LengthDeclaration::new(d).unwrap();
    f.tables.reverse();
    f.eases.reverse();
    f.with_context(|c| {
        assert!(chart.validate_complete(c).is_ok());
        let ease = chart.ease_for_pom(id(2), c).unwrap();
        assert_eq!(ease.definition().fit, FitIntent::Loose);
        assert_eq!(ease.definition().provenance, id(114));
        assert_eq!(chart.body_declaration(id(2), c).unwrap().source(), id(115));
        assert_eq!(chart.body_value(id(2), c).unwrap(), length(750_001));
    });
}
#[test]
fn current_compression_permission_is_enforced_without_expanding_the_v1_envelope() {
    let mut f = Fixture::new();
    let chart = f.chart();
    f.declarations[3] = scalar(13, assumed(-1));
    f.with_context(|c| {
        let value = chart.ease_value(id(2), c);
        assert!(value.is_err());
        let error = value.err().unwrap();
        assert_eq!(
            error,
            E::InvalidEaseSet {
                chart: id(70),
                ease_set: id(60),
                issue: Box::new(EaseSetError::InvalidEase {
                    set: id(60),
                    ease: id(30),
                    issue: Box::new(EaseError::UndeclaredCompression {
                        ease: id(30),
                        declaration: id(13),
                        amount: length(-1),
                    }),
                }),
            }
        );
        assert!(error.source().unwrap().source().is_some());
    });
    let mut definition = f.eases[0].definition().clone();
    definition.compression = CompressionPermission::Declared {
        provenance: id(117),
    };
    f.eases[0] = f.make_ease(definition);
    f.with_context(|c| assert_eq!(chart.ease_value(id(2), c).unwrap(), length(-1)));
}
#[test]
fn missing_body_metadata_table_member_and_mapping_are_never_defaulted() {
    for case in 0..5 {
        let mut f = Fixture::new();
        let chart = f.chart();
        match case {
            0 => {
                f.landmarks.remove(0);
            }
            1 => {
                let mut d = f.tables[0].definition().clone();
                d.entries.clear();
                let r = f.records();
                let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
                f.tables[0] = MeasurementTable::new(d, &m).unwrap();
            }
            2 => {
                let mut d = f.eases[0].definition().clone();
                d.id = id(32);
                f.eases[0] = f.make_ease(d);
            }
            3 => {
                f.tables.remove(0);
            }
            _ => {
                let mut d = f.measurements[0].definition().clone();
                d.id = id(4);
                f.measurements[0] = Measurement::new(d, &f.records()).unwrap();
            }
        }
        f.with_context(|c| {
            let result = chart.body_value(id(2), c);
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert!(matches!(
                &error,
                E::InvalidEaseSet { chart, ease_set, .. }
                    if (*chart, *ease_set) == (id(70), id(60))
            ));
            assert!(error.source().is_some());
            assert!(chart.validate_complete(c).is_err());
        });
    }
}
#[test]
fn selected_query_does_not_certify_other_mappings_or_full_design_metadata() {
    let mut f = Fixture::new();
    let chart = f.chart();
    f.eases.remove(1);
    f.with_context(|c| {
        assert_eq!(chart.body_value(id(2), c).unwrap(), length(740_000));
        assert_eq!(
            chart.validate_current(c).err(),
            Some(E::InvalidEaseSet {
                chart: id(70),
                ease_set: id(60),
                issue: Box::new(EaseSetError::MissingEase {
                    set: id(60),
                    ease: id(31)
                })
            })
        );
    });
    let mut f = Fixture::new();
    let chart = f.chart();
    let mut d = f.measurements[0].definition().clone();
    d.id = id(4);
    d.token = MachineToken::new("extra_body").unwrap();
    f.measurements
        .push(Measurement::new(d, &f.records()).unwrap());
    let mut d = f.tables[1].definition().clone();
    d.entries.push(MeasurementBinding::from(&f.measurements[3]));
    let r = f.records();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    f.tables[1] = MeasurementTable::new(d, &m).unwrap();
    f.measurements.remove(3);
    f.with_context(|c| {
        assert!(chart.validate_current(c).is_ok());
        assert!(chart.body_value(id(2), c).is_ok());
        assert!(matches!(
            chart.validate_complete(c),
            Err(E::InvalidDesignTable { .. })
        ));
    });
}
#[test]
fn grade_rule_input_always_refuses_for_complete_and_empty_authored_mtm_charts() {
    let mut f = Fixture::new();
    let complete = f.chart();
    assert_eq!(
        complete.validate_grade_rule_input().err(),
        Some(E::GradeRulesUnsupported {
            chart: id(70),
            member: id(41)
        })
    );
    let mut d = f.sets[0].definition().clone();
    d.entries.clear();
    f.sets[0] = f.make_set(d);
    let draft = f.chart();
    assert_eq!(
        draft.validate_grade_rule_input().err(),
        Some(E::GradeRulesUnsupported {
            chart: id(70),
            member: id(41)
        })
    );
}
#[test]
fn chart_and_context_identities_cannot_alias_members_sets_or_canonical_records() {
    let f = Fixture::new();
    for n in [40, 41, 60, 50, 30, 1, 10, 20, 24] {
        let mut d = f.definition();
        d.id = id(n);
        f.with_context(|c| {
            assert_eq!(MtmChart::new(d, c).err(), Some(E::DuplicateIdentity(id(n))))
        });
    }
    for case in 0..5 {
        let mut f = Fixture::new();
        let expected = match case {
            0 => {
                f.sets.push(f.sets[0].clone());
                id(60)
            }
            1 | 2 => {
                let n = if case == 1 { 40 } else { 41 };
                let mut d = f.sets[0].definition().clone();
                d.id = id(n);
                f.sets[0] = f.make_set(d);
                id(n)
            }
            3 => {
                let mut d = f.membership.definition().clone();
                d.members[0].id = id(1);
                d.base = id(1);
                f.membership = SizeMembership::new(d).unwrap();
                id(1)
            }
            _ => {
                f.declarations.push(scalar(40, assumed(0)));
                id(40)
            }
        };
        let r = f.records();
        let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        let e = EaseSetContext::new(&f.tables, &f.eases, &m).unwrap();
        assert_eq!(
            MtmChartContext::new(&f.membership, &f.sets, &e).err(),
            Some(E::DuplicateIdentity(expected))
        );
    }
}
