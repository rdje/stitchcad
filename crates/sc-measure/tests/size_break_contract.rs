//! Explicit axis/POM/adjacent-pair signed sources, current records and structural refusals.
#![allow(clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{
        LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError,
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState,
    },
};
use sc_measure::*;
use sc_units::{Count, Length, Unit};
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn length(n: i64) -> Length {
    Length::from_micrometres(n).expect("fixture length")
}
fn label(s: &str) -> SizeLabel {
    SizeLabel::new(s.to_owned()).expect("fixture label")
}
fn declaration(n: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(n),
        source: id(n + 1000),
        state,
    })
    .expect("canonical fixture")
}
struct Fixture {
    lengths: Vec<LengthDeclaration>,
    scalars: Vec<ScalarInputDeclaration>,
    landmarks: Vec<Landmark>,
    procedures: Vec<MeasurementProcedure>,
    measurements: Vec<Measurement>,
    table: MeasurementTable,
    membership: SizeMembership,
    axes: SizeAxes,
}
impl Fixture {
    fn new(kind: MeasurementKind) -> Self {
        let lengths = vec![
            declaration(
                100,
                LengthState::Assumed {
                    value: length(40_000),
                    assumption: id(800),
                },
            ),
            declaration(
                101,
                LengthState::Assumed {
                    value: length(50_000),
                    assumption: id(800),
                },
            ),
            declaration(
                102,
                LengthState::Known {
                    value: length(0),
                    evidence: vec![id(801)],
                },
            ),
            declaration(
                103,
                LengthState::Preference {
                    value: length(-10_000),
                    provenance: id(804),
                },
            ),
            declaration(
                150,
                LengthState::Unknown {
                    observation: id(802),
                },
            ),
            declaration(151, LengthState::Derived { formula: id(803) }),
        ];
        let landmarks = (300..302)
            .map(|n| {
                Landmark::new(LandmarkDefinition {
                    id: id(n),
                    name: format!("Fixture landmark {n}"),
                    kind,
                    source: id(900),
                })
                .expect("landmark")
            })
            .collect::<Vec<_>>();
        let procedures = vec![MeasurementProcedure::new(MeasurementProcedureDefinition {
            id: id(400),
            name: "Fixture method".to_owned(),
            kind,
            source: id(900),
            documentation: "Authored test protocol, no physical certification.".to_owned(),
        })
        .expect("procedure")];
        let records = MeasurementContext::new(&lengths, &landmarks, &procedures).expect("records");
        let measurements = [("waist", 150), ("hip", 151)]
            .into_iter()
            .enumerate()
            .map(|(i, (token, source))| {
                Measurement::new(
                    MeasurementDefinition {
                        id: id(200 + i as u128),
                        name: token.to_owned(),
                        token: MachineToken::new(token).expect("token"),
                        entered_unit: Unit::Centimetre,
                        kind,
                        landmarks: [id(300), id(301)],
                        procedure: id(400),
                        declaration: id(source),
                    },
                    &records,
                )
                .expect("measurement")
            })
            .collect::<Vec<_>>();
        let metadata = MeasurementTableContext::new(&measurements, &records).expect("metadata");
        let table = MeasurementTable::new(
            MeasurementTableDefinition {
                id: id(500),
                name: "Explicit dimensions".to_owned(),
                entries: measurements.iter().map(MeasurementBinding::from).collect(),
            },
            &metadata,
        )
        .expect("table");
        let membership = SizeMembership::new(SizeMembershipDefinition {
            reference: SizeSetReference {
                id: id(1),
                revision: Count::new(3),
            },
            system: SizeSystem::Custom,
            members: vec![
                SizeMember {
                    id: id(2),
                    label: label("2XL"),
                },
                SizeMember {
                    id: id(3),
                    label: label("S"),
                },
                SizeMember {
                    id: id(4),
                    label: label("M"),
                },
            ],
            base: id(3),
        })
        .expect("membership");
        let refs = lengths.iter().collect::<Vec<_>>();
        let sources = SizeAxisSourceContext::new(&refs, &[]).expect("shared sources");
        let axis = SizeAxis::new(
            SizeAxisDefinition {
                reference: membership.reference(),
                id: id(10),
                name: "House size".to_owned(),
                kind: SizeAxisKind::Label,
                values: (11..14)
                    .map(|n| SizeAxisValueDefinition {
                        id: id(n),
                        source: SizeAxisValueSource::Label(label(&format!("Value {n}"))),
                    })
                    .collect(),
            },
            &sources,
        )
        .expect("axis");
        let axes = SizeAxes::new(
            SizeAxesDefinition {
                reference: membership.reference(),
                axes: vec![axis],
                points: (2..5)
                    .map(|n| SizeMemberPoint {
                        member: id(n),
                        coordinates: vec![SizeAxisCoordinate {
                            axis: id(10),
                            value: id(n + 9),
                        }],
                    })
                    .collect(),
            },
            &membership,
            &sources,
        )
        .expect("axes");
        Self {
            lengths,
            scalars: vec![],
            landmarks,
            procedures,
            measurements,
            table,
            membership,
            axes,
        }
    }
    fn definition(&self) -> SizeBreakDefinition {
        SizeBreakDefinition {
            id: id(700),
            reference: self.membership.reference(),
            axis: id(10),
            from: id(2),
            to: id(3),
            dimension_table: self.table.id(),
            dimension: MeasurementBinding::from(&self.measurements[0]),
            delta: id(100),
        }
    }
    fn with_context<T>(&self, run: impl FnOnce(&SizeBreakContext<'_>) -> T) -> T {
        let records = MeasurementContext::new(&self.lengths, &self.landmarks, &self.procedures)
            .expect("records");
        let metadata =
            MeasurementTableContext::new(&self.measurements, &records).expect("metadata");
        let refs = self.lengths.iter().collect::<Vec<_>>();
        let scalars = self.scalars.iter().collect::<Vec<_>>();
        let sources = SizeAxisSourceContext::new(&refs, &scalars).expect("sources");
        let context = SizeBreakContext::new(
            &self.membership,
            &self.axes,
            &sources,
            &self.table,
            &metadata,
        )
        .expect("current break context");
        run(&context)
    }
}
#[test]
fn same_axis_step_keeps_distinct_dimensions_and_exact_uneven_zero_negative_deltas() {
    for kind in [MeasurementKind::Body, MeasurementKind::Garment] {
        let f = Fixture::new(kind);
        f.with_context(|context| {
            let d = f.definition();
            let waist = SizeBreak::new(d.clone(), context).expect("waist break");
            assert_eq!(waist.definition(), &d);
            assert!(std::ptr::eq(
                waist.delta(context).expect("delta"),
                &f.lengths[0]
            ));
            assert!(std::ptr::eq(
                waist.dimension(context).expect("dimension"),
                &f.measurements[0]
            ));
            assert_eq!(
                waist.authored_value(context).expect("signed delta"),
                length(40_000)
            );
            let mut hip = d.clone();
            hip.id = id(701);
            hip.dimension = MeasurementBinding::from(&f.measurements[1]);
            hip.delta = id(101);
            let hip = SizeBreak::new(hip, context).expect("same step distinct dimension");
            assert_eq!(
                hip.authored_value(context).expect("hip delta"),
                length(50_000)
            );
            let mut next = d.clone();
            next.from = id(3);
            next.to = id(4);
            next.delta = id(101);
            assert_eq!(
                SizeBreak::new(next, context)
                    .expect("uneven next step")
                    .authored_value(context)
                    .expect("delta"),
                length(50_000)
            );
            for (delta, expected) in [(102, 0), (103, -10_000)] {
                let mut d = d.clone();
                d.delta = id(delta);
                assert_eq!(
                    SizeBreak::new(d, context)
                        .expect("explicit signed break")
                        .authored_value(context)
                        .expect("delta"),
                    length(expected)
                );
            }
            assert_eq!(
                context.membership().base().expect("authored base").id,
                id(3)
            );
        });
    }
}
#[test]
fn all_five_original_states_sources_and_read_refusals_survive_without_chart_subtraction() {
    let states = [
        LengthState::Known {
            value: length(0),
            evidence: vec![id(801)],
        },
        LengthState::Assumed {
            value: length(-1),
            assumption: id(800),
        },
        LengthState::Unknown {
            observation: id(802),
        },
        LengthState::Preference {
            value: length(2),
            provenance: id(804),
        },
        LengthState::Derived { formula: id(803) },
    ];
    for state in states {
        let mut f = Fixture::new(MeasurementKind::Garment);
        f.lengths[0] = declaration(100, state.clone());
        f.with_context(|context| {
            let value = SizeBreak::new(f.definition(), context)
                .expect("state retained without numeric read");
            let original = value.delta(context).expect("original source");
            assert!(std::ptr::eq(original, &f.lengths[0]));
            assert_eq!(original.state(), &state);
            assert_eq!(original.source(), id(1100));
            match state {
                LengthState::Unknown { observation } => {
                    let refusal = SizeBreakError::Value(LengthValueError::RequiresObservation {
                        declaration: id(100),
                        observation,
                    });
                    assert_eq!(value.authored_value(context), Err(refusal.clone()));
                    assert!(refusal.source().is_some());
                }
                LengthState::Derived { formula } => {
                    let refusal = SizeBreakError::Value(LengthValueError::RequiresEvaluation {
                        declaration: id(100),
                        formula,
                    });
                    assert_eq!(value.authored_value(context), Err(refusal.clone()));
                    assert!(refusal.source().is_some());
                }
                _ => assert_eq!(
                    value
                        .authored_value(context)
                        .expect("present authored delta"),
                    original.authored_value().expect("canonical")
                ),
            }
        });
    }
}
#[test]
fn exact_pins_axis_members_and_adjacency_refuse_without_sorting_or_defaults() {
    let f = Fixture::new(MeasurementKind::Garment);
    f.with_context(|context| {
        let d = f.definition();
        let mut wrong = d.clone();
        wrong.reference.revision = Count::new(4);
        assert_eq!(
            SizeBreak::new(wrong.clone(), context).err(),
            Some(SizeBreakError::MembershipMismatch {
                expected: wrong.reference,
                actual: f.membership.reference()
            })
        );
        let mut wrong = d.clone();
        wrong.axis = id(99);
        assert_eq!(
            SizeBreak::new(wrong, context).err(),
            Some(SizeBreakError::MissingAxis(id(99)))
        );
        let mut wrong = d.clone();
        wrong.from = id(99);
        let member_error = SizeBreakError::Member(SizeMembershipError::MissingMember {
            set: id(1),
            member: id(99),
        });
        assert_eq!(
            SizeBreak::new(wrong, context).err(),
            Some(member_error.clone())
        );
        assert!(member_error.source().is_some());
        for (from, to) in [(2, 4), (3, 2), (2, 2)] {
            let mut wrong = d.clone();
            wrong.from = id(from);
            wrong.to = id(to);
            assert_eq!(
                SizeBreak::new(wrong, context).err(),
                Some(SizeBreakError::NotAdjacent {
                    set: id(1),
                    from: id(from),
                    to: id(to)
                })
            );
        }
        for alias in [1, 2, 10, 11, 100, 200, 300, 400, 500] {
            let mut wrong = d.clone();
            wrong.id = id(alias);
            assert_eq!(
                SizeBreak::new(wrong, context).err(),
                Some(SizeBreakError::DuplicateIdentity(id(alias)))
            );
        }
    });
}
#[test]
fn dimension_and_delta_sources_are_explicit_and_current() {
    let mut f = Fixture::new(MeasurementKind::Garment);
    f.scalars.push(
        ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
            id: id(1000),
            source: id(1100),
            kind: ScalarInputKind::Length,
            state: ScalarInputState::Unknown {
                observation: id(802),
            },
        })
        .expect("general scalar fixture"),
    );
    f.with_context(|context| {
        let d = f.definition();
        let mut wrong = d.clone();
        wrong.dimension_table = id(999);
        assert_eq!(
            SizeBreak::new(wrong, context).err(),
            Some(SizeBreakError::TableMismatch {
                expected: id(999),
                actual: id(500)
            })
        );
        for which in 0..3 {
            let mut wrong = d.clone();
            match which {
                0 => wrong.dimension.token = MachineToken::new("another_waist").expect("token"),
                1 => wrong.dimension.kind = MeasurementKind::Body,
                _ => wrong.dimension.declaration = id(101),
            }
            assert_eq!(
                SizeBreak::new(wrong.clone(), context).err(),
                Some(SizeBreakError::DimensionMismatch {
                    expected: Box::new(wrong.dimension),
                    actual: Box::new(d.dimension.clone())
                })
            );
        }
        let mut wrong = d.clone();
        wrong.delta = id(999);
        assert_eq!(
            SizeBreak::new(wrong, context).err(),
            Some(SizeBreakError::MissingDelta(id(999)))
        );
        let mut wrong = d.clone();
        wrong.delta = id(1000);
        assert_eq!(
            SizeBreak::new(wrong, context).err(),
            Some(SizeBreakError::DeltaRole(id(1000)))
        );
    });
    let value =
        f.with_context(|context| SizeBreak::new(f.definition(), context).expect("old snapshot"));
    f.lengths[0] = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(100),
        source: id(9999),
        state: LengthState::Unknown {
            observation: id(8888),
        },
    })
    .expect("replacement");
    f.with_context(|context| {
        assert_eq!(
            value.delta(context).expect("current replacement").source(),
            id(9999)
        );
        assert_eq!(
            value.authored_value(context),
            Err(SizeBreakError::Value(
                LengthValueError::RequiresObservation {
                    declaration: id(100),
                    observation: id(8888)
                }
            ))
        );
    });
    let mut reordered = f.membership.definition().clone();
    reordered.members.reverse();
    f.membership = SizeMembership::new(reordered).expect("explicit reorder");
    f.with_context(|context| {
        assert_eq!(
            value.validate_current(context),
            Err(SizeBreakError::NotAdjacent {
                set: id(1),
                from: id(2),
                to: id(3)
            })
        )
    });
}
#[test]
fn shared_inventory_rejects_disagreeing_canonical_content_and_cross_kind_aliases() {
    let f = Fixture::new(MeasurementKind::Garment);
    let records =
        MeasurementContext::new(&f.lengths, &f.landmarks, &f.procedures).expect("records");
    let metadata = MeasurementTableContext::new(&f.measurements, &records).expect("metadata");
    let changed = declaration(
        100,
        LengthState::Unknown {
            observation: id(8888),
        },
    );
    let sources = SizeAxisSourceContext::new(&[&changed], &[]).expect("different actual source");
    assert_eq!(
        SizeBreakContext::new(&f.membership, &f.axes, &sources, &f.table, &metadata).err(),
        Some(SizeBreakError::ConflictingSource(id(100)))
    );
    let scalar = ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(100),
        source: id(1100),
        kind: ScalarInputKind::Length,
        state: ScalarInputState::Unknown {
            observation: id(802),
        },
    })
    .expect("role conflict fixture");
    let sources = SizeAxisSourceContext::new(&[], &[&scalar]).expect("distinct inventory");
    assert_eq!(
        SizeBreakContext::new(&f.membership, &f.axes, &sources, &f.table, &metadata).err(),
        Some(SizeBreakError::ConflictingSource(id(100)))
    );
}
