//! Custom member-of-one, current body/Ease correspondence and structural coverage contracts.
#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing
)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use sc_measure::{
    CompressionPermission, Ease, EaseBinding, EaseDefinition, EaseSet, EaseSetContext,
    EaseSetDefinition, FitIntent, Landmark, LandmarkDefinition, Measurement, MeasurementBinding,
    MeasurementContext, MeasurementDefinition, MeasurementKind, MeasurementProcedure,
    MeasurementProcedureDefinition, MeasurementTable, MeasurementTableContext,
    MeasurementTableDefinition, MtmChart, MtmChartContext, MtmChartDefinition, SizeLabel,
    SizeMember, SizeMembership, SizeMembershipDefinition, SizeSetReference, SizeSystem,
};
use sc_units::{Count, Length, Unit};
pub(crate) fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
pub(crate) fn length(n: i64) -> Length {
    Length::from_micrometres(n).unwrap()
}
pub(crate) fn assumed(n: i64) -> LengthState {
    LengthState::Assumed {
        value: length(n),
        assumption: id(90),
    }
}
pub(crate) fn scalar(n: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(n),
        source: id(91),
        state,
    })
    .unwrap()
}
pub(crate) struct Fixture {
    pub(crate) declarations: Vec<LengthDeclaration>,
    pub(crate) landmarks: Vec<Landmark>,
    pub(crate) procedures: Vec<MeasurementProcedure>,
    pub(crate) measurements: Vec<Measurement>,
    pub(crate) tables: Vec<MeasurementTable>,
    pub(crate) eases: Vec<Ease>,
    pub(crate) sets: Vec<EaseSet>,
    pub(crate) membership: SizeMembership,
}
impl Fixture {
    pub(crate) fn new() -> Self {
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
    pub(crate) fn records(&self) -> MeasurementContext<'_> {
        MeasurementContext::new(&self.declarations, &self.landmarks, &self.procedures).unwrap()
    }
    pub(crate) fn with_context<R>(&self, callback: impl FnOnce(&MtmChartContext<'_>) -> R) -> R {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        let e = EaseSetContext::new(&self.tables, &self.eases, &m).unwrap();
        let c = MtmChartContext::new(&self.membership, &self.sets, &e).unwrap();
        callback(&c)
    }
    pub(crate) fn make_set(&self, d: EaseSetDefinition) -> EaseSet {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        let e = EaseSetContext::new(&self.tables, &self.eases, &m).unwrap();
        EaseSet::new(d, &e).unwrap()
    }
    pub(crate) fn make_ease(&self, d: EaseDefinition) -> Ease {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        Ease::new(d, &m).unwrap()
    }
    pub(crate) fn definition(&self) -> MtmChartDefinition {
        MtmChartDefinition {
            id: id(70),
            membership: self.membership.reference(),
            member: id(41),
            ease_set: self.sets[0].definition().clone(),
            provenance: id(100),
        }
    }
    pub(crate) fn chart(&self) -> MtmChart {
        self.with_context(|c| MtmChart::new(self.definition(), c).unwrap())
    }
    pub(crate) fn add_table_peer(&mut self, index: usize, n: u128) {
        let mut d = self.tables[index].definition().clone();
        d.id = id(n);
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        self.tables.push(MeasurementTable::new(d, &m).unwrap());
    }
}
