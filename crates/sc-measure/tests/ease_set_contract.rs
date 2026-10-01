//! Set namespaces, current canonical mapping bindings and table membership regression contracts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError},
};
use sc_measure::{
    CompressionPermission, Ease, EaseBinding, EaseDefinition, EaseError, EaseSet, EaseSetContext,
    EaseSetDefinition, EaseSetError, EaseSide, FitIntent, Landmark, LandmarkDefinition,
    Measurement, MeasurementBinding, MeasurementContext, MeasurementDefinition, MeasurementKind,
    MeasurementProcedure, MeasurementProcedureDefinition, MeasurementTable,
    MeasurementTableContext, MeasurementTableDefinition, MeasurementTableError,
};
use sc_units::{Length, Unit};
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn token(s: &str) -> MachineToken {
    MachineToken::new(s).unwrap()
}
fn value(n: i64) -> Length {
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
        value: value(n),
        assumption: id(91),
    }
}
struct Fixture {
    declarations: Vec<LengthDeclaration>,
    landmarks: Vec<Landmark>,
    procedures: Vec<MeasurementProcedure>,
    measurements: Vec<Measurement>,
    tables: Vec<MeasurementTable>,
    eases: Vec<Ease>,
}
impl Fixture {
    fn new() -> Self {
        let mut f = Self {
            declarations: [10, 11, 12, 13, 14]
                .into_iter()
                .map(|n| scalar(n, assumed(40_000)))
                .collect(),
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
                        documentation: "Authored test procedure, no physical certification."
                            .to_owned(),
                    })
                    .unwrap()
                })
                .collect(),
            measurements: vec![],
            tables: vec![],
            eases: vec![],
        };
        let r = f.records();
        f.measurements = (1..4)
            .map(|n| {
                Measurement::new(
                    MeasurementDefinition {
                        id: id(n),
                        name: format!("Fixture input {n}"),
                        token: token(&format!("measurement_{n}")),
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
                        declaration: id(if n == 3 { 13 } else { n + 9 }),
                    },
                    &r,
                )
                .unwrap()
            })
            .collect();
        let r = f.records();
        let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
        f.tables = vec![
            MeasurementTable::new(
                MeasurementTableDefinition {
                    id: id(40),
                    name: "Body inputs".to_owned(),
                    entries: vec![MeasurementBinding::from(&f.measurements[0])],
                },
                &m,
            )
            .unwrap(),
            MeasurementTable::new(
                MeasurementTableDefinition {
                    id: id(41),
                    name: "Garment POMs".to_owned(),
                    entries: f.measurements[1..]
                        .iter()
                        .map(MeasurementBinding::from)
                        .collect(),
                },
                &m,
            )
            .unwrap(),
        ];
        f.initialize_eases();
        f
    }
    fn records(&self) -> MeasurementContext<'_> {
        MeasurementContext::new(&self.declarations, &self.landmarks, &self.procedures).unwrap()
    }
    fn initialize_eases(&mut self) {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        self.eases = (0..2)
            .map(|n| {
                Ease::new(
                    EaseDefinition {
                        id: id(50 + n as u128),
                        body: MeasurementBinding::from(&self.measurements[0]),
                        garment: MeasurementBinding::from(&self.measurements[n + 1]),
                        declaration: id(if n == 0 { 12 } else { 14 }),
                        fit: FitIntent::Semi,
                        provenance: id(94),
                        compression: CompressionPermission::Forbidden,
                    },
                    &m,
                )
                .unwrap()
            })
            .collect();
    }
    fn context<T>(&self, action: impl FnOnce(&EaseSetContext<'_>) -> T) -> T {
        let r = self.records();
        let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
        let c = EaseSetContext::new(&self.tables, &self.eases, &m).unwrap();
        action(&c)
    }
    fn definition(&self) -> EaseSetDefinition {
        EaseSetDefinition {
            id: id(60),
            body_table: id(40),
            garment_table: id(41),
            entries: self
                .eases
                .iter()
                .enumerate()
                .map(|(n, e)| {
                    EaseBinding::new(token(if n == 0 { "ease_first" } else { "ease_second" }), e)
                })
                .collect(),
        }
    }
    fn set(&self) -> EaseSet {
        self.context(|c| EaseSet::new(self.definition(), c).unwrap())
    }
    fn replace_ease(&mut self, n: usize, d: EaseDefinition) {
        let replacement = {
            let r = self.records();
            let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
            Ease::new(d, &m).unwrap()
        };
        self.eases[n] = replacement;
    }
    fn replace_table(&mut self, n: usize, d: MeasurementTableDefinition) {
        let replacement = {
            let r = self.records();
            let m = MeasurementTableContext::new(&self.measurements, &r).unwrap();
            MeasurementTable::new(d, &m).unwrap()
        };
        self.tables[n] = replacement;
    }
}
#[test]
fn ordered_unique_set_queries_borrow_current_mappings_and_shared_body_is_legal() {
    let mut f = Fixture::new();
    let set = f.set();
    assert_eq!(set.id(), id(60));
    assert_eq!(set.definition().entries[0].ease, id(50));
    assert_eq!(set.definition().entries[1].ease, id(51));
    f.context(|c| {
        assert!(std::ptr::eq(
            set.ease_for_pom(id(2), c).unwrap(),
            &f.eases[0]
        ));
        assert!(std::ptr::eq(
            set.ease_by_token(&token("ease_second"), c).unwrap(),
            &f.eases[1]
        ));
        assert_eq!(
            set.ease_by_id(id(51), c).unwrap().definition().body,
            set.ease_by_id(id(50), c).unwrap().definition().body
        );
    });
    f.eases.reverse();
    f.tables.reverse();
    f.measurements.reverse();
    f.declarations.reverse();
    f.context(|c| {
        assert_eq!(set.ease_for_pom(id(2), c).unwrap().id(), id(50));
        assert!(set.validate_current(c).is_ok());
    });
}
#[test]
fn empty_drafts_require_existing_tables_and_queries_never_default() {
    let f = Fixture::new();
    let mut d = f.definition();
    d.entries.clear();
    let set = f.context(|c| EaseSet::new(d, c).unwrap());
    f.context(|c| {
        assert_eq!(
            set.ease_for_pom(id(2), c).err(),
            Some(EaseSetError::MissingPom {
                set: id(60),
                pom: id(2)
            })
        );
        assert_eq!(
            set.ease_by_token(&token("ease_first"), c).err(),
            Some(EaseSetError::MissingToken {
                set: id(60),
                token: token("ease_first")
            })
        );
        assert_eq!(
            set.ease_by_id(id(50), c).err(),
            Some(EaseSetError::MissingBinding {
                set: id(60),
                ease: id(50)
            })
        );
        let mut missing = set.definition().clone();
        missing.body_table = id(100);
        assert_eq!(
            EaseSet::new(missing, c).err(),
            Some(EaseSetError::MissingTable {
                set: id(60),
                side: EaseSide::Body,
                table: id(100)
            })
        );
    });
}
#[test]
fn duplicate_mapping_tokens_and_poms_refuse_without_coalescing() {
    for mode in 0..3 {
        let mut f = Fixture::new();
        if mode == 2 {
            let mut d = f.eases[1].definition().clone();
            d.garment = f.eases[0].definition().garment.clone();
            f.replace_ease(1, d);
        }
        let mut d = f.definition();
        let expected = match mode {
            0 => {
                d.entries[1].ease = d.entries[0].ease;
                EaseSetError::DuplicateIdentity(id(50))
            }
            1 => {
                d.entries[1].token = d.entries[0].token.clone();
                EaseSetError::DuplicateToken {
                    set: id(60),
                    token: token("ease_first"),
                }
            }
            _ => EaseSetError::DuplicatePom {
                set: id(60),
                pom: id(2),
            },
        };
        f.context(|c| assert_eq!(EaseSet::new(d, c).err(), Some(expected)));
    }
}
#[test]
fn contexts_refuse_duplicate_and_cross_kind_identities_before_lookup() {
    let f = Fixture::new();
    let r = f.records();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    assert_eq!(
        EaseSetContext::new(&[f.tables[0].clone(), f.tables[0].clone()], &f.eases, &m).err(),
        Some(EaseSetError::DuplicateIdentity(id(40)))
    );
    assert_eq!(
        EaseSetContext::new(&f.tables, &[f.eases[0].clone(), f.eases[0].clone()], &m).err(),
        Some(EaseSetError::DuplicateIdentity(id(50)))
    );
    let mut d = f.eases[0].definition().clone();
    d.id = id(40);
    let e = Ease::new(d, &m).unwrap();
    assert_eq!(
        EaseSetContext::new(&f.tables, &[e], &m).err(),
        Some(EaseSetError::DuplicateIdentity(id(40)))
    );
    let mut declarations = f.declarations.clone();
    declarations.push(scalar(40, assumed(0)));
    let r = MeasurementContext::new(&declarations, &f.landmarks, &f.procedures).unwrap();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    assert_eq!(
        EaseSetContext::new(&f.tables, &f.eases, &m).err(),
        Some(EaseSetError::DuplicateIdentity(id(40)))
    );
    let mut declarations = f.declarations.clone();
    declarations.push(scalar(50, assumed(0)));
    let r = MeasurementContext::new(&declarations, &f.landmarks, &f.procedures).unwrap();
    let m = MeasurementTableContext::new(&f.measurements, &r).unwrap();
    assert_eq!(
        EaseSetContext::new(&f.tables, &f.eases, &m).err(),
        Some(EaseSetError::DuplicateIdentity(id(50)))
    );
}
#[test]
fn set_identity_collisions_refuse_at_creation_and_in_current_context() {
    let mut f = Fixture::new();
    let set = f.set();
    f.context(|c| {
        for n in [1, 10, 20, 30, 40, 50] {
            let mut d = f.definition();
            d.id = id(n);
            assert_eq!(
                EaseSet::new(d, c).err(),
                Some(EaseSetError::DuplicateIdentity(id(n)))
            );
        }
    });
    f.declarations.push(scalar(60, assumed(0)));
    f.context(|c| {
        assert_eq!(
            set.validate_current(c).err(),
            Some(EaseSetError::DuplicateIdentity(id(60)))
        )
    });
}
#[test]
fn removal_never_replaces_saved_mapping_with_same_pom_peer() {
    let mut f = Fixture::new();
    let set = f.set();
    let mut d = f.eases[0].definition().clone();
    d.id = id(52);
    f.replace_ease(0, d);
    f.context(|c| {
        assert_eq!(
            set.ease_for_pom(id(2), c).err(),
            Some(EaseSetError::MissingEase {
                set: id(60),
                ease: id(50)
            })
        )
    });
}
#[test]
fn current_retargeted_mapping_needs_explicit_new_set_binding() {
    for change in 0..3 {
        let mut f = Fixture::new();
        let set = f.set();
        let mut d = f.eases[0].definition().clone();
        match change {
            0 => d.garment = MeasurementBinding::from(&f.measurements[2]),
            1 => d.declaration = id(14),
            _ => {
                let mut md = f.measurements[0].definition().clone();
                md.id = id(4);
                let replacement = Measurement::new(md, &f.records()).unwrap();
                f.measurements.push(replacement);
                d.body = MeasurementBinding::from(&f.measurements[3]);
                let mut td = f.tables[0].definition().clone();
                td.entries.push(d.body.clone());
                td.entries[1].token = token("new_body");
                let mut md = f.measurements[3].definition().clone();
                md.token = token("new_body");
                let replacement = Measurement::new(md, &f.records()).unwrap();
                f.measurements[3] = replacement;
                d.body = MeasurementBinding::from(&f.measurements[3]);
                f.replace_table(0, td);
            }
        }
        f.replace_ease(0, d);
        f.context(|c|{assert!(matches!(set.ease_for_pom(id(2),c),Err(EaseSetError::ReassignedEase{set:owner,ease,expected,actual}) if owner==id(60) && ease==id(50) && *expected==set.definition().entries[0] && *actual==EaseBinding::new(token("ease_first"),&f.eases[0])));
            let mut replacement=set.definition().clone();replacement.entries[0]=EaseBinding::new(token("ease_first"),&f.eases[0]);if change==0 {replacement.entries.pop();}assert!(EaseSet::new(replacement,c).is_ok());
        });
    }
}
#[test]
fn selected_current_table_membership_is_required_even_when_metadata_exists() {
    for side in [EaseSide::Body, EaseSide::Garment] {
        let mut f = Fixture::new();
        let set = f.set();
        let n = if side == EaseSide::Body { 0 } else { 1 };
        let mut d = f.tables[n].definition().clone();
        d.entries
            .retain(|b| b.measurement != id(if n == 0 { 1 } else { 2 }));
        f.replace_table(n, d);
        f.context(|c| {
            let expected = EaseSetError::InvalidTableMember {
                set: id(60),
                ease: id(50),
                side,
                issue: Box::new(MeasurementTableError::MissingBinding {
                    table: id(if n == 0 { 40 } else { 41 }),
                    measurement: id(if n == 0 { 1 } else { 2 }),
                }),
            };
            assert_eq!(set.ease_for_pom(id(2), c).err(), Some(expected));
            let error = set.ease_for_pom(id(2), c).err().unwrap();
            assert!(error.source().is_some());
        });
    }
}
#[test]
fn missing_named_tables_cannot_be_replaced_by_content_equivalent_tables() {
    for side in [EaseSide::Body, EaseSide::Garment] {
        let mut f = Fixture::new();
        let set = f.set();
        let n = if side == EaseSide::Body { 0 } else { 1 };
        let mut d = f.tables[n].definition().clone();
        d.id = id(42);
        f.replace_table(n, d);
        f.context(|c| {
            assert_eq!(
                set.validate_current(c).err(),
                Some(EaseSetError::MissingTable {
                    set: id(60),
                    side,
                    table: id(if n == 0 { 40 } else { 41 })
                })
            )
        });
    }
}
#[test]
fn one_mixed_table_can_own_both_sides() {
    let mut f = Fixture::new();
    let mut d = f.tables[0].definition().clone();
    d.entries = f
        .measurements
        .iter()
        .map(MeasurementBinding::from)
        .collect();
    f.replace_table(0, d);
    let mut d = f.definition();
    d.garment_table = d.body_table;
    f.context(|c| assert!(EaseSet::new(d, c).is_ok()));
}
#[test]
fn current_fit_provenance_amount_state_and_permission_are_borrowed() {
    let mut f = Fixture::new();
    let set = f.set();
    let mut d = f.eases[0].definition().clone();
    d.fit = FitIntent::Loose;
    d.provenance = id(100);
    d.compression = CompressionPermission::Declared {
        provenance: id(101),
    };
    f.replace_ease(0, d);
    f.declarations[2] = scalar(12, assumed(-10));
    f.context(|c| {
        let ease = set.ease_for_pom(id(2), c).unwrap();
        assert_eq!(ease.definition().fit, FitIntent::Loose);
        assert_eq!(ease.definition().provenance, id(100));
        assert_eq!(ease.authored_value(c.measurements()).unwrap(), value(-10));
    });
    for state in [
        LengthState::Unknown {
            observation: id(102),
        },
        LengthState::Derived { formula: id(103) },
    ] {
        f.declarations[2] = scalar(12, state.clone());
        f.context(|c| {
            let ease = set.ease_for_pom(id(2), c).unwrap();
            assert_eq!(ease.declaration(c.measurements()).unwrap().state(), &state);
            assert!(matches!(
                ease.authored_value(c.measurements()),
                Err(EaseError::UnavailableValue {
                    issue: LengthValueError::RequiresObservation { .. }
                        | LengthValueError::RequiresEvaluation { .. },
                    ..
                })
            ));
        });
    }
}
#[test]
fn current_invalid_amount_and_table_targets_preserve_underlying_errors() {
    let mut f = Fixture::new();
    let set = f.set();
    f.declarations[2] = scalar(12, assumed(-1));
    f.context(|c| {
        assert_eq!(
            set.ease_for_pom(id(2), c).err(),
            Some(EaseSetError::InvalidEase {
                set: id(60),
                ease: id(50),
                issue: Box::new(EaseError::UndeclaredCompression {
                    ease: id(50),
                    declaration: id(12),
                    amount: value(-1)
                })
            })
        );
        assert!(set.ease_for_pom(id(2), c).err().unwrap().source().is_some());
    });
    f.declarations[2] = scalar(12, assumed(0));
    let mut md = f.measurements[0].definition().clone();
    md.token = token("changed_body");
    let replacement = Measurement::new(md, &f.records()).unwrap();
    f.measurements[0] = replacement;
    f.context(|c|assert!(matches!(set.ease_for_pom(id(2),c),Err(EaseSetError::InvalidTableMember{side:EaseSide::Body,issue,..}) if matches!(*issue,MeasurementTableError::TokenMismatch{measurement,..} if measurement==id(1)))));
}
#[test]
fn selected_queries_do_not_certify_unrelated_entries_or_table_members() {
    let mut f = Fixture::new();
    let set = f.set();
    f.eases.pop();
    f.context(|c| {
        assert!(set.ease_for_pom(id(2), c).is_ok());
        assert_eq!(
            set.validate_current(c).err(),
            Some(EaseSetError::MissingEase {
                set: id(60),
                ease: id(51)
            })
        );
    });
    f.measurements.pop();
    f.context(|c| assert!(set.ease_for_pom(id(2), c).is_ok()));
}
#[test]
fn different_sets_can_share_tokens_and_amounts_without_implicit_defaults() {
    let mut f = Fixture::new();
    let mut d = f.eases[1].definition().clone();
    d.declaration = id(12);
    f.replace_ease(1, d);
    let set = f.set();
    f.context(|c| {
        let mut d = f.definition();
        d.id = id(61);
        let second = EaseSet::new(d, c).unwrap();
        assert_eq!(
            set.ease_by_token(&token("ease_first"), c).unwrap().id(),
            second.ease_by_token(&token("ease_first"), c).unwrap().id()
        );
        assert_eq!(
            set.ease_for_pom(id(2), c)
                .unwrap()
                .declaration(c.measurements())
                .unwrap()
                .id(),
            set.ease_for_pom(id(3), c)
                .unwrap()
                .declaration(c.measurements())
                .unwrap()
                .id()
        );
        assert_eq!(
            set.ease_for_pom(id(4), c).err(),
            Some(EaseSetError::MissingPom {
                set: id(60),
                pom: id(4)
            })
        );
    });
}
