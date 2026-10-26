//! Explicit axis/POM/adjacent-pair signed sources, current records and structural refusals.
#![allow(clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, ScalarInputDeclaration},
};
use sc_measure::*;
use sc_units::{Count, Length, Unit};
pub(crate) fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
pub(crate) fn length(n: i64) -> Length {
    Length::from_micrometres(n).expect("fixture length")
}
pub(crate) fn label(s: &str) -> SizeLabel {
    SizeLabel::new(s.to_owned()).expect("fixture label")
}
pub(crate) fn declaration(n: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(n),
        source: id(n + 1000),
        state,
    })
    .expect("canonical fixture")
}
pub(crate) struct Fixture {
    pub(crate) lengths: Vec<LengthDeclaration>,
    pub(crate) scalars: Vec<ScalarInputDeclaration>,
    pub(crate) landmarks: Vec<Landmark>,
    pub(crate) procedures: Vec<MeasurementProcedure>,
    pub(crate) measurements: Vec<Measurement>,
    pub(crate) table: MeasurementTable,
    pub(crate) membership: SizeMembership,
    pub(crate) axes: SizeAxes,
}
impl Fixture {
    pub(crate) fn new(kind: MeasurementKind) -> Self {
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
    pub(crate) fn definition(&self) -> SizeBreakDefinition {
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
    pub(crate) fn with_context<T>(&self, run: impl FnOnce(&SizeBreakContext<'_>) -> T) -> T {
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
