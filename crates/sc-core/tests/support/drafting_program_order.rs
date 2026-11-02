//! Actual complete mixed kinds and prior-only names,without geometry/numeric execution.
use super::{limits, root_with_recipe, scope, Source, TestResult};
use sc_core::{
    construction::*,
    name::MachineToken,
    ontology::{DirectedRange, Direction, EdgeRange, EdgeRef, EntityId, LocalTag},
    recipe::{
        FormulaCheckedStatementKind, FormulaDeclarationSource, FormulaExpressionDocument,
        FormulaKind, FormulaNamespaceError, FormulaStatementCheckRefusal,
    },
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn doc(text: &str) -> TestResult<FormulaExpressionDocument> {
    Ok(FormulaExpressionDocument::new(text.into())?)
}
#[test]
fn mixed_order_keeps_all_original_proofs_and_refuses_self_future_headers_and_late_kinds(
) -> TestResult {
    for mode in 0..14 {
        let recipe=match mode {
            1=>"let width:length=width\nlet second:length=x(anchor)\nassert closure:eps_geo=second==width",
            2=>"let width:length=x(anchor)\nlet second:length=x(anchor)\nassert closure:eps_geo=second==width",
            8=>"let width:length=1 mm\nlet second:length=x(anchor)\nassert closure:eps_geo=width==1 deg",
            9=>"let width:length=1 mm\nlet anchor:length=private_missing\nassert closure:eps_geo=width==width",
            _=>"let width:length=1 mm\nlet second:length=x(anchor)\nassert closure:eps_geo=second==width",
        };
        let root = root_with_recipe(recipe)?;
        let source = Source::new(&root);
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
            .map_err(|e| e.to_string())?;
        let point = PointConstruction::new(
            id(100),
            doc(match mode {
                3 | 13 => "x(anchor)",
                4 => "x(future_edge)",
                5 => "1 deg",
                10..=12 => "private_missing",
                _ => "width",
            })?,
            doc("1 mm/0")?,
        );
        let arc = CenterArcConstruction::new(
            id(101),
            point.output(),
            doc(if mode == 6 { "1 deg" } else { "5 mm" })?,
            doc("0 deg")?,
            doc("90 deg")?,
        )?;
        let range = DirectedRange {
            range: EdgeRange::whole(EdgeRef::new(id(900), LocalTag::FIRST)),
            direction: Direction::Original,
        };
        let slash = SlashSpreadConstruction::separation(
            id(102),
            id(800),
            range,
            SlashSpreadSide::Left,
            range,
            doc(if mode == 7 { "1 deg" } else { "second" })?,
        )?;
        let walk = WalkSeamEdit::new(id(103), id(800), vec![]);
        let raw = DraftingProgramInputs::new(
            context,
            vec![
                DraftingProgramStep::Scalar { ordinal: 1 },
                DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
                DraftingProgramStep::Scalar { ordinal: 2 },
                DraftingProgramStep::Drafting(DraftingOperationInput::DrawArc(&arc)),
                DraftingProgramStep::Drafting(DraftingOperationInput::SlashSpread(&slash)),
                DraftingProgramStep::Drafting(DraftingOperationInput::WalkSeam(&walk)),
                DraftingProgramStep::Scalar { ordinal: 3 },
            ],
        );
        let program_policy =
            DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 1000, 10000, 100000);
        let program = raw
            .admit(&program_policy)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        // Original output vector is deliberately not in mixed-position order.
        let mut outputs = vec![
            DraftingOutputName::new(
                4,
                MachineToken::new("future_edge")?,
                DraftingGeometryNameSource::CenterArc(&arc),
            ),
            DraftingOutputName::new(
                2,
                MachineToken::new(match mode {
                    10 => "eps_geo",
                    12 => "width",
                    _ => "anchor",
                })?,
                DraftingGeometryNameSource::Point(&point),
            ),
        ];
        if mode == 11 {
            outputs.push(DraftingOutputName::new(
                2,
                MachineToken::new("anchor")?,
                DraftingGeometryNameSource::Point(&point),
            ));
        }
        let names = DraftingProgramNameInputs::new(&program, vec![], outputs);
        let name_policy = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 128, 256);
        let work = names
            .admit_name_work(&name_policy)
            .map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        let namespace = initial
            .bind_initial_namespace()
            .map_err(|e| e.to_string())?;
        let policy = if mode == 13 {
            DraftingProgramOrderLimits::new(0, 0, 0)
        } else {
            DraftingProgramOrderLimits::new(1000, 10000, 100000)
        };
        source.trace.set(0);
        match namespace.check_order(&policy) {
            Ok(proof) => {
                assert_eq!(mode, 0);
                assert!(
                    std::ptr::eq(proof.inputs(), &initial) && std::ptr::eq(proof.limits(), &policy)
                );
                assert_eq!(
                    proof
                        .steps()
                        .iter()
                        .map(|s| s.position())
                        .collect::<Vec<_>>(),
                    [1, 2, 3, 4, 5, 6, 7]
                );
                assert_eq!(proof.shape().step_count(), 7);
                let uses: Vec<_> = proof.dependencies().collect();
                assert_eq!(
                    uses.iter()
                        .map(|u| u.declaration().name())
                        .collect::<Vec<_>>(),
                    ["width", "anchor", "second", "eps_geo", "second", "width"]
                );
                assert_eq!(
                    uses.iter().map(|u| u.position()).collect::<Vec<_>>(),
                    [2, 3, 5, 7, 7, 7]
                );
                assert_eq!(
                    uses.iter().map(|u| u.statement_index()).collect::<Vec<_>>(),
                    [None, Some(2), None, Some(3), Some(3), Some(3)]
                );
                assert!(!format!("{proof:?}").contains("anchor"));
                for step in proof.steps() {
                    match step.kind() {
                        DraftingProgramOrderProofKind::Scalar(statement) => {
                            let ordinal = match step.position() {
                                1 => 1,
                                3 => 2,
                                7 => 3,
                                _ => return Err("actual scalar position".into()),
                            };
                            assert_eq!(statement.statement_index(), ordinal);
                            assert!(std::ptr::eq(
                                statement.statement(),
                                program
                                    .recipe()
                                    .normalized()
                                    .statements()
                                    .get(ordinal - 1)
                                    .ok_or("original scalar owner")?
                            ));
                            if ordinal == 2 {
                                let FormulaCheckedStatementKind::Let { expression } =
                                    statement.kind()
                                else {
                                    return Err("actual second let proof".into());
                                };
                                assert!(expression.dependencies().iter().any(|dependency|matches!(dependency.declaration().source(),FormulaDeclarationSource::Point(p) if p==point.output())));
                            }
                        }
                        DraftingProgramOrderProofKind::Point(checked) => {
                            assert_eq!(step.position(), 2);
                            assert!(std::ptr::eq(checked.normalized().point(), &point));
                            assert_eq!(
                                checked.coordinate(PointCoordinate::Y).kind(),
                                FormulaKind::Length
                            );
                        }
                        DraftingProgramOrderProofKind::CenterArc(checked) => {
                            assert_eq!(step.position(), 4);
                            assert!(std::ptr::eq(checked.normalized().arc(), &arc));
                            assert_eq!(
                                checked.argument(CenterArcArgument::SweepAngle).kind(),
                                FormulaKind::Angle
                            );
                        }
                        DraftingProgramOrderProofKind::SlashSpread(checked) => {
                            assert_eq!(step.position(), 5);
                            assert!(std::ptr::eq(
                                checked.normalized().opening(),
                                slash.opening()
                            ));
                            assert_eq!(checked.expression().kind(), FormulaKind::Length);
                        }
                        DraftingProgramOrderProofKind::Other(input) => {
                            assert_eq!(step.position(), 6);
                            assert!(
                                matches!(input,DraftingOperationInput::WalkSeam(actual) if std::ptr::eq(*actual,&walk))
                            );
                        }
                    }
                }
            }
            Err(error) => {
                assert_ne!(mode, 0);
                assert!(
                    std::ptr::eq(error.inputs(), &initial) && std::ptr::eq(error.limits(), &policy)
                );
                assert!(!format!("{error:?} {error}").contains("private_missing"));
                match error.refusal() {
                    DraftingProgramOrderRefusal::Work(
                        DraftingProgramOrderWorkRefusal::Expressions { allowed, required },
                    ) => {
                        assert_eq!(mode, 13);
                        assert_eq!((*allowed, *required), (0, 10));
                    }
                    DraftingProgramOrderRefusal::Scalar { position, cause } => {
                        assert!(matches!(mode, 1 | 2 | 8));
                        assert_eq!(
                            (*position, cause.statement_index()),
                            if mode == 8 { (7, 3) } else { (1, 1) }
                        );
                        assert!(std::ptr::eq(
                            cause.statement(),
                            program
                                .recipe()
                                .normalized()
                                .statements()
                                .get(cause.statement_index() - 1)
                                .ok_or("original bad statement")?
                        ));
                        if mode == 8 {
                            assert!(matches!(
                                cause.refusal(),
                                FormulaStatementCheckRefusal::AssertionDimension(_)
                            ));
                        } else {
                            assert_eq!(cause.token(), "formula_unbound_name");
                        }
                    }
                    DraftingProgramOrderRefusal::Point { position, cause } => {
                        assert!(matches!(mode, 3..=5));
                        assert_eq!(*position, 2);
                        assert_eq!(cause.coordinate(), PointCoordinate::X);
                        assert!(std::ptr::eq(cause.normalized().point(), &point));
                        if mode == 5 {
                            assert!(matches!(
                                cause.refusal(),
                                PointConstructionCheckRefusal::Kind {
                                    actual: FormulaKind::Angle
                                }
                            ));
                        } else {
                            assert_eq!(cause.token(), "formula_unbound_name");
                        }
                    }
                    DraftingProgramOrderRefusal::CenterArc { position, cause } => {
                        assert_eq!(mode, 6);
                        assert_eq!(*position, 4);
                        assert_eq!(cause.argument(), CenterArcArgument::Radius);
                        assert!(std::ptr::eq(cause.normalized().arc(), &arc));
                        assert!(matches!(
                            cause.refusal(),
                            CenterArcCheckRefusal::Kind {
                                actual: FormulaKind::Angle
                            }
                        ));
                    }
                    DraftingProgramOrderRefusal::SlashSpread { position, cause } => {
                        assert_eq!(mode, 7);
                        assert_eq!(*position, 5);
                        assert!(std::ptr::eq(cause.normalized().opening(), slash.opening()));
                        assert!(matches!(
                            cause.refusal(),
                            SlashSpreadCheckRefusal::Kind {
                                actual: FormulaKind::Angle
                            }
                        ));
                    }
                    DraftingProgramOrderRefusal::Header {
                        position,
                        output_index,
                        cause,
                    } => {
                        assert!(matches!(mode, 9..=12));
                        assert_eq!(*position, if mode == 9 { 3 } else { 2 });
                        assert_eq!(
                            *output_index,
                            match mode {
                                9 => None,
                                11 => Some(2),
                                _ => Some(1),
                            }
                        );
                        match cause.as_ref() {
                            FormulaNamespaceError::ReservedBinding { attempted, .. } => {
                                assert_eq!(mode, 10);
                                assert_eq!(attempted.name(), "eps_geo");
                            }
                            FormulaNamespaceError::AmbiguousName { sources } => {
                                let [first, second] = **sources;
                                if mode == 9 {
                                    assert!(
                                        matches!(first.source(),FormulaDeclarationSource::Point(p) if p==point.output())
                                    );
                                    assert!(matches!(
                                        second.source(),
                                        FormulaDeclarationSource::Recipe {
                                            statement_index: 2,
                                            ..
                                        }
                                    ));
                                } else if mode == 11 {
                                    assert!(
                                        matches!(first.source(),FormulaDeclarationSource::Point(p) if p==point.output())
                                    );
                                    assert_eq!(first.name(), second.name());
                                    assert!(!std::ptr::eq(first.name(), second.name()));
                                } else {
                                    assert_eq!(mode, 12);
                                    assert!(matches!(
                                        first.source(),
                                        FormulaDeclarationSource::Recipe {
                                            statement_index: 1,
                                            ..
                                        }
                                    ));
                                    assert!(
                                        matches!(second.source(),FormulaDeclarationSource::Point(p) if p==point.output())
                                    );
                                }
                            }
                            other => {
                                return Err(
                                    format!("original source pair expected:{other:?}").into()
                                )
                            }
                        }
                    }
                    other => {
                        return Err(format!("original mixed refusal expected:{other:?}").into())
                    }
                }
            }
        }
        assert_eq!(source.trace.get(), 0);
    }
    Ok(())
}
#[test]
fn empty_mixed_order_accepts_zero_checker_work_and_no_callback() -> TestResult {
    let root = root_with_recipe("")?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 1000, 1000)?)
        .map_err(|e| e.to_string())?;
    let raw = DraftingProgramInputs::new(context, vec![]);
    let pp = DraftingProgramLimits::new(limits(1000, 1000, 1000)?, 0, 0, 0);
    let program = raw
        .admit(&pp)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    let np = DraftingProgramNameLimits::new(limits(1000, 1000, 1000)?, 8, 8);
    let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
    let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
    let namespace = initial
        .bind_initial_namespace()
        .map_err(|e| e.to_string())?;
    let policy = DraftingProgramOrderLimits::new(0, 0, 0);
    source.trace.set(0);
    let proof = namespace.check_order(&policy).map_err(|e| e.to_string())?;
    assert!(proof.steps().is_empty());
    let mut uses = proof.dependencies();
    assert!(uses.next().is_none() && uses.next().is_none());
    assert_eq!(proof.shape().retained_record_bound(), 0);
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
