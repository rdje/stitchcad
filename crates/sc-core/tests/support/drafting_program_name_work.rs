//! Complete content precedes actual host cardinality;no row/binding or namespace prefix is visited.
use super::{limits, root, root_with_host, scope, Source, SourceError, TestResult};
use core::cell::Cell;
use sc_core::{
    construction::*,
    design::*,
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBindingKind, FormulaDeclaration, FormulaExpressionDocument,
        FormulaInitialDeclaration, FormulaInputOrigin, FormulaScalarInputOrigin,
    },
    value::{CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
struct Row {
    id: EntityId,
    name: MachineToken,
    target: EntityId,
}
// Real borrowed rows/canonical bindings;neither this provider nor the iterator needs Clone/Debug.
struct Host {
    rows: Vec<Row>,
    factories: Cell<usize>,
    reads: Cell<usize>,
    validations: Cell<usize>,
}
impl DesignMetadataHost for Host {
    type Error = SourceError;
    fn entities(&self) -> Vec<DesignHostEntity> {
        std::iter::once(DesignHostEntity {
            id: id(90),
            kind: DesignHostEntityKind::MeasurementTable,
            index: 0,
        })
        .chain(
            self.rows
                .iter()
                .enumerate()
                .map(|(index, row)| DesignHostEntity {
                    id: row.id,
                    kind: DesignHostEntityKind::Measurement,
                    index,
                }),
        )
        .collect()
    }
    fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), Self::Error> {
        self.validations.set(self.validations.get() + 1);
        for row in &self.rows {
            canonical.length(row.target).ok_or(SourceError::Missing)?;
        }
        Ok(())
    }
    fn measurement_declaration<'a>(
        &'a self,
        measurement: EntityId,
        canonical: &'a CanonicalInputs,
    ) -> Result<&'a LengthDeclaration, Self::Error> {
        let row = self
            .rows
            .iter()
            .find(|row| row.id == measurement)
            .ok_or(SourceError::Missing)?;
        canonical.length(row.target).ok_or(SourceError::Missing)
    }
}
impl DesignSizeSetHost for Host {
    fn validate_size_set(
        &self,
        _: DesignSizeSetReference,
        _: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        Err(SourceError::Missing)
    }
}
impl DesignReferenceHost for Host {
    fn validate_references(
        &self,
        refs: DesignSourceReferences,
        canonical: &CanonicalInputs,
    ) -> Result<(), Self::Error> {
        self.validate_current(canonical)?;
        if refs.measurement_table == Some(id(90))
            && refs.ease_set.is_none()
            && refs.size_set.is_none()
        {
            Ok(())
        } else {
            Err(SourceError::Missing)
        }
    }
}
impl DesignResourceSource for Host {
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let location = DesignResourceLocation {
            scope: DesignResourceScope::Inputs,
            owner: Some(id(90)),
            field: "measurement.rows",
            index: None,
        };
        sink.charge_entries(self.rows.len(), location)?;
        for (index, row) in self.rows.iter().enumerate() {
            sink.charge_text(
                row.name.as_str(),
                DesignResourceLocation {
                    owner: Some(row.id),
                    index: Some(index),
                    ..location
                },
            )?;
        }
        Ok(())
    }
}
struct Rows<'a> {
    rows: std::slice::Iter<'a, Row>,
    canonical: &'a CanonicalInputs,
    reads: &'a Cell<usize>,
}
impl<'a> Iterator for Rows<'a> {
    type Item = Result<FormulaDeclaration<'a>, SourceError>;
    fn next(&mut self) -> Option<Self::Item> {
        let row = self.rows.next()?;
        self.reads.set(self.reads.get() + 1);
        Some(
            self.canonical
                .length(row.target)
                .ok_or(SourceError::Missing)
                .map(|record| {
                    FormulaDeclaration::length_input(
                        &row.name,
                        FormulaInputOrigin::Measurement,
                        row.id,
                        record,
                    )
                }),
        )
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rows.size_hint()
    }
}
impl ExactSizeIterator for Rows<'_> {}
impl std::iter::FusedIterator for Rows<'_> {}
impl DesignFormulaDeclarationSource for Host {
    type Declarations<'a> = Rows<'a>;
    fn formula_declaration_sources<'a>(
        &'a self,
        refs: DesignSourceReferences,
        canonical: &'a CanonicalInputs,
    ) -> Rows<'a> {
        self.factories.set(self.factories.get() + 1);
        let rows = if refs.measurement_table == Some(id(90)) {
            self.rows.as_slice()
        } else {
            &[]
        };
        Rows {
            rows: rows.iter(),
            canonical,
            reads: &self.reads,
        }
    }
}
#[test]
fn actual_host_rows_and_complete_populations_precede_binding_or_namespace_work() -> TestResult {
    let canonical = CanonicalInputs::new(
        [101, 102]
            .into_iter()
            .map(|n| {
                LengthDeclaration::new(LengthDeclarationDefinition {
                    id: id(n),
                    source: id(n + 1000),
                    state: LengthState::Unknown {
                        observation: id(n + 2000),
                    },
                })
            })
            .collect::<Result<Vec<_>, _>>()?,
        vec![],
    )?;
    let host = Host {
        rows: vec![
            Row {
                id: id(91),
                name: MachineToken::new("measured_width")?,
                target: id(101),
            },
            Row {
                id: id(92),
                name: MachineToken::new("measured_height")?,
                target: id(102),
            },
        ],
        factories: Cell::new(0),
        reads: Cell::new(0),
        validations: Cell::new(0),
    };
    let root = root_with_host(
        "let scalar:length=1 mm\nassert closure:eps_geo=scalar==1 mm",
        vec![],
        host,
        canonical,
        vec![DesignParameterDefinition {
            id: id(93),
            token: MachineToken::new("parameter")?,
            target: DesignParameterTarget::Length(id(101)),
        }],
        DesignSourceReferences {
            measurement_table: Some(id(90)),
            ease_set: None,
            size_set: None,
        },
    )?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let inputs = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Scalar { ordinal: 2 },
        ],
    );
    let raw_policy = DraftingProgramLimits::new(limits(1000, 10000, 1000)?, 10, 100, 1000);
    let program = inputs
        .admit(&raw_policy)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(
        id(100),
        FormulaExpressionDocument::new("1 mm".into())?,
        FormulaExpressionDocument::new("2 mm".into())?,
    );
    let initial_name = MachineToken::new("extra_angle")?;
    let initial = FormulaInitialDeclaration::try_from(FormulaDeclaration::input(
        &initial_name,
        FormulaScalarInputOrigin::Profile,
        id(201),
        id(202),
        FormulaBindingKind::Angle,
    ))
    .map_err(|_| "initial input")?;
    // Positions remain invalid raw requests:this work owner proves no source/position association.
    let names = DraftingProgramNameInputs::new(
        &program,
        vec![initial],
        vec![
            DraftingOutputName::new(
                0,
                MachineToken::new("first")?,
                DraftingGeometryNameSource::Point(&point),
            ),
            DraftingOutputName::new(
                usize::MAX,
                MachineToken::new("second")?,
                DraftingGeometryNameSource::Point(&point),
            ),
        ],
    );
    let host = root.sources().host();
    let validations = host.validations.get();
    let policy = DraftingProgramNameLimits::new(limits(1000, 10000, 1000)?, 16, 18);
    source.trace.set(0);
    let admitted = names.admit_name_work(&policy).map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(admitted.inputs(), &names) && std::ptr::eq(admitted.limits(), &policy));
    let shape = admitted.shape();
    assert_eq!(
        (
            shape.parameter_count(),
            shape.host_declaration_count(),
            shape.initial_count(),
            shape.output_count(),
            shape.statement_count()
        ),
        (1, 2, 1, 2, 2)
    );
    // Independent bound:reserved8 + parameters1 + host2 + initial1 + outputs2 + statements2 =16;
    // two prospective associations give18 records,including the assertion's conservative slot.
    assert_eq!(
        (shape.namespace_entry_bound(), shape.retained_record_bound()),
        (16, 18)
    );
    assert_eq!(
        (host.factories.get(), host.reads.get(), source.trace.get()),
        (1, 0, 2)
    );
    assert_eq!(host.validations.get(), validations);
    let mut meter = DesignResourceMeter::new(policy.content());
    names.charge_resources(&mut meter)?;
    assert_eq!(admitted.demand(), meter.finish()?);
    for (namespace, records, expected) in [
        (
            15,
            0,
            DraftingProgramNameWorkRefusal::NamespaceEntries {
                allowed: 15,
                required: 16,
            },
        ),
        (
            16,
            17,
            DraftingProgramNameWorkRefusal::RetainedRecords {
                allowed: 17,
                required: 18,
            },
        ),
    ] {
        let policy = DraftingProgramNameLimits::new(limits(1000, 10000, 1000)?, namespace, records);
        let error = names
            .admit_name_work(&policy)
            .expect_err("whole prospective count before row binding");
        assert!(std::ptr::eq(error.inputs(), &names) && std::ptr::eq(error.limits(), &policy));
        assert_eq!(error.refusal(), expected);
    }
    assert_eq!(host.reads.get(), 0);
    let factories = host.factories.get();
    source.trace.set(0);
    let bad = DraftingProgramNameLimits::new(limits(1000, 1, 1000)?, 0, 0);
    let error = names
        .admit_name_work(&bad)
        .expect_err("content before iterator factory and zero quotas");
    assert!(matches!(
        error.refusal(),
        DraftingProgramNameWorkRefusal::Resources(DesignResourceLimitError::Count {
            location: DesignResourceLocation {
                field: "parameters.token",
                ..
            },
            ..
        })
    ));
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(
        (host.factories.get(), host.reads.get(), source.trace.get()),
        (factories, 0, 0)
    );
    assert_eq!(host.validations.get(), validations);
    assert!(!format!("{admitted:?} {error:?}").contains("extra_angle"));
    // The same genuine inventory later yields the actual canonical loans without reading Unknown values.
    let rows = host
        .formula_declaration_sources(root.references(), root.inputs())
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(
        rows.iter().map(|d| d.name()).collect::<Vec<_>>(),
        ["measured_width", "measured_height"]
    );
    assert_eq!(host.reads.get(), 2);
    for (declaration, identity) in rows.iter().zip([101, 102]) {
        let sc_core::recipe::FormulaDeclarationSource::LengthInput { declaration, .. } =
            declaration.source()
        else {
            return Err("actual dedicated canonical Length loan".into());
        };
        assert!(std::ptr::eq(
            declaration,
            root.inputs()
                .length(id(identity))
                .ok_or("canonical owner")?
        ));
        assert!(matches!(declaration.state(), LengthState::Unknown { .. }));
    }
    let absent = CanonicalInputs::new(vec![], vec![])?;
    let mut broken = host.formula_declaration_sources(root.references(), &absent);
    assert_eq!(broken.size_hint(), (2, Some(2)));
    assert_eq!(host.reads.get(), 2);
    assert!(matches!(broken.next(), Some(Err(SourceError::Missing))));
    assert_eq!(broken.len(), 1);
    assert!(matches!(broken.next(), Some(Err(SourceError::Missing))));
    assert!(broken.next().is_none() && broken.next().is_none());
    assert_eq!(host.reads.get(), 4);
    Ok(())
}
#[test]
fn empty_name_work_still_reserves_all_reserved_metadata() -> TestResult {
    let root = root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let input = DraftingProgramInputs::new(context, vec![]);
    let raw = DraftingProgramLimits::new(limits(1000, 1000, 1000)?, 0, 0, 0);
    let program = input
        .admit(&raw)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    let policy = DraftingProgramNameLimits::new(limits(1000, 1000, 1000)?, 8, 8);
    assert_eq!(
        names
            .admit_name_work(&policy)
            .map_err(|e| e.to_string())?
            .shape()
            .retained_record_bound(),
        8
    );
    let policy = DraftingProgramNameLimits::new(limits(1000, 1000, 1000)?, 0, 0);
    assert_eq!(
        names
            .admit_name_work(&policy)
            .expect_err("reserved metadata is real work")
            .refusal(),
        DraftingProgramNameWorkRefusal::NamespaceEntries {
            allowed: 0,
            required: 8
        }
    );
    Ok(())
}
