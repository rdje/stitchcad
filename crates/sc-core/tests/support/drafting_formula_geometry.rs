//! Original selector geometry uses retain actual Root/mixed consumers, all branches and indices.
use super::{
    initial_geometry, limits, point, root_with_host, scope, EmptyHost, Fault, Provider, TestResult,
};
use sc_core::{
    construction::*,
    design::DesignSourceReferences,
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag},
    recipe::*,
    value::CanonicalInputs,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn doc(s: &str) -> TestResult<FormulaExpressionDocument> {
    Ok(FormulaExpressionDocument::new(s.into())?)
}
#[test]
fn formula_geometry_uses_keep_unused_root_sources_mixed_slots_original_indices_and_fused_end(
) -> TestResult {
    for rich in [true, false] {
        let text = if rich {
            concat!(
                "let width:length=x(private_point)+len(private_edge)\n",
                "assert closure:eps_geo=x(anchor)==len(boundary)+len(private_edge)",
            )
        } else {
            "let literal:length=1 mm"
        };
        let sources = if rich {
            [
                (9001, "if(1==1,x(private_point),len(private_edge))"),
                (9002, "len(private_edge)+x(private_point)"),
                (9003, "if(size_count==1,x(private_point),x(private_point))"),
            ]
            .into_iter()
            .map(|(n, s)| Ok(FormulaSource::new(id(n), doc(s)?)))
            .collect::<TestResult<Vec<_>>>()?
        } else {
            vec![]
        };
        let count = sources.len();
        let root = root_with_host(
            text,
            sources,
            EmptyHost,
            CanonicalInputs::new(vec![], vec![])?,
            vec![],
            DesignSourceReferences {
                measurement_table: None,
                ease_set: None,
                size_set: None,
            },
        )?;
        let other = super::root()?;
        let provider = Provider::new(&root, &other, Fault::None)?;
        let context = scope(&root)?
            .bind_drafting_context(&provider, limits(10000, 10000, 10000)?)
            .map_err(|e| e.to_string())?;
        let point_input = PointConstruction::new(
            id(500),
            doc("if(size_count==1,x(private_point),x(private_point))")?,
            doc("width")?,
        );
        let arc = CenterArcConstruction::new(
            id(600),
            point(80),
            doc("width")?,
            doc("0 deg")?,
            doc("90 deg")?,
        )?;
        let steps = if rich {
            vec![
                DraftingProgramStep::Scalar { ordinal: 1 },
                DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point_input)),
                DraftingProgramStep::Drafting(DraftingOperationInput::DrawArc(&arc)),
                DraftingProgramStep::Scalar { ordinal: 2 },
            ]
        } else {
            vec![DraftingProgramStep::Scalar { ordinal: 1 }]
        };
        let raw = DraftingProgramInputs::new(context, steps);
        let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 1000, 10000);
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let point_name = MachineToken::new("private_point")?;
        let edge_name = MachineToken::new("private_edge")?;
        let outputs = if rich {
            vec![
                DraftingOutputName::new(
                    2,
                    MachineToken::new("anchor")?,
                    DraftingGeometryNameSource::Point(&point_input),
                ),
                DraftingOutputName::new(
                    3,
                    MachineToken::new("boundary")?,
                    DraftingGeometryNameSource::CenterArc(&arc),
                ),
            ]
        } else {
            vec![]
        };
        let names = DraftingProgramNameInputs::new(
            &program,
            vec![
                initial_geometry(&point_name, DraftingGeometryReference::Point(point(80)))?,
                initial_geometry(
                    &edge_name,
                    DraftingGeometryReference::Edge(EdgeRef::new(id(81), LocalTag::FIRST)),
                )?,
            ],
            outputs,
        );
        let np = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
        let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        let namespace = initial
            .bind_initial_namespace()
            .map_err(|e| e.to_string())?;
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(count, 1000, 0, 10000))
            .map_err(|e| e.to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(count, 0, count + 1))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(count, count * 2))
            .map_err(|e| e.to_string())?;
        let op = DraftingProgramOrderLimits::new(100, 1000, 10000);
        let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
        let inputs = sources
            .dependency_inputs(&order, &graph)
            .map_err(|e| e.to_string())?;
        let cp = if rich {
            DraftingProgramCanonicalLimits::new(11, 11)
        } else {
            DraftingProgramCanonicalLimits::new(0, 0)
        };
        let bound = inputs
            .bind_canonical_inputs(&cp)
            .map_err(|e| e.to_string())?;
        provider.calls.set(0);
        provider.base.trace.set(0);
        let uses: Vec<_> = bound.formula_geometry_dependencies().collect();
        let requests: Vec<_> = bound.geometry_requests().collect();
        if rich {
            assert_eq!(
                requests
                    .iter()
                    .map(|r| r.occurrence().position())
                    .collect::<Vec<_>>(),
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(4),
                    Some(4)
                ]
            );
            for request in &requests {
                match request.target().map_err(|e| format!("{e:?}"))? {
                    DraftingProgramGeometryTarget::Existing => {
                        assert!(
                            matches!(request.occurrence().reference(),
                            DraftingGeometryReference::Point(p) if p==point(80))
                                || matches!(request.occurrence().reference(),
                            DraftingGeometryReference::Edge(e) if e==EdgeRef::new(id(81),LocalTag::FIRST))
                        );
                    }
                    DraftingProgramGeometryTarget::Earlier(creator) => {
                        assert_eq!(request.occurrence().position(), Some(4));
                        assert!(!creator.requires_role_binding());
                        assert_eq!(creator.reference(), request.occurrence().reference());
                        match creator.input() {
                            DraftingOperationInput::ConstructPoint(actual) => {
                                assert!(std::ptr::eq(actual, &point_input));
                                assert_eq!(creator.step().position(), 2);
                            }
                            DraftingOperationInput::DrawArc(actual) => {
                                assert!(std::ptr::eq(actual, &arc));
                                assert_eq!(creator.step().position(), 3);
                            }
                            _ => return Err("actual formula creator".into()),
                        }
                    }
                }
            }
        } else {
            assert!(requests.is_empty());
        }
        let mut request_end = bound.geometry_requests().skip(requests.len());
        for _ in 0..3 {
            assert!(request_end.next().is_none());
        }
        if rich {
            assert_eq!(uses.len(), 13);
            assert_eq!(
                uses.iter()
                    .map(|use_| use_.declaration().name())
                    .collect::<Vec<_>>(),
                [
                    "private_point",
                    "private_edge",
                    "private_edge",
                    "private_point",
                    "private_point",
                    "private_point",
                    "private_point",
                    "private_edge",
                    "private_point",
                    "private_point",
                    "anchor",
                    "boundary",
                    "private_edge",
                ]
            );
            use DraftingGeometryReference::{Edge, Point};
            let p = Point(point(80));
            let e = Edge(EdgeRef::new(id(81), LocalTag::FIRST));
            assert_eq!(
                uses.iter().map(|use_| use_.reference()).collect::<Vec<_>>(),
                [
                    p,
                    e,
                    e,
                    p,
                    p,
                    p,
                    p,
                    e,
                    p,
                    p,
                    Point(point(500)),
                    Edge(EdgeRef::new(id(600), LocalTag::FIRST)),
                    e
                ]
            );
            let source_indices = [(0, 0), (0, 1), (1, 0), (1, 1), (2, 1), (2, 2)];
            let mixed_indices = [0, 1, 3, 4, 8, 9, 10];
            for (index, use_) in uses.iter().enumerate() {
                let original = match use_.consumer() {
                    DraftingProgramFormulaGeometryConsumer::Source {
                        source_index,
                        source,
                    } => {
                        assert_eq!(
                            (source_index, use_.index()),
                            *source_indices.get(index).ok_or("source position")?
                        );
                        assert!(std::ptr::eq(
                            source,
                            sources
                                .proof()
                                .sources()
                                .get(source_index)
                                .ok_or("actual source")?
                        ));
                        let actual = source
                            .proof()
                            .dependencies()
                            .get(use_.index())
                            .ok_or("actual source occurrence")?;
                        assert_eq!(use_.span(), actual.span());
                        assert!(std::ptr::eq(
                            use_.declaration().name(),
                            actual.declaration().name()
                        ));
                        source.normalized().source().expression().source()
                    }
                    DraftingProgramFormulaGeometryConsumer::Mixed(slot) => {
                        assert_eq!(
                            use_.index(),
                            *mixed_indices.get(index - 6).ok_or("mixed position")?
                        );
                        assert!(std::ptr::eq(
                            slot,
                            bound
                                .occurrences()
                                .get(use_.index())
                                .ok_or("actual mixed slot")?
                        ));
                        assert_eq!(use_.span(), slot.occurrence().span());
                        assert!(std::ptr::eq(
                            use_.declaration().name(),
                            slot.occurrence().declaration().name()
                        ));
                        match slot.occurrence().role() {
                            DraftingProgramDependencyRole::Scalar(_) => text,
                            DraftingProgramDependencyRole::Point(role) => {
                                point_input.coordinate(role).source()
                            }
                            DraftingProgramDependencyRole::CenterArc(role) => {
                                arc.argument(role).source()
                            }
                            DraftingProgramDependencyRole::SlashSpread => {
                                return Err("no Slash input in fixture".into())
                            }
                        }
                    }
                };
                assert_eq!(
                    original.get(use_.span().start()..use_.span().end()),
                    Some(use_.declaration().name())
                );
                assert!(!format!("{use_:?} {:?}", use_.consumer()).contains("private"));
            }
            let mut skipped = bound.formula_geometry_dependencies();
            assert_eq!(skipped.nth(5).ok_or("last source branch")?.index(), 2);
            assert!(matches!(
                skipped.next().ok_or("first mixed occurrence")?.consumer(),
                DraftingProgramFormulaGeometryConsumer::Mixed(_)
            ));
            assert_eq!(skipped.count(), 6);
        } else {
            assert!(uses.is_empty());
        }
        let mut exhausted = bound.formula_geometry_dependencies();
        for _ in exhausted.by_ref() {}
        assert!(exhausted.next().is_none() && exhausted.next().is_none());
        assert_eq!((provider.calls.get(), provider.base.trace.get()), (0, 0));
    }
    Ok(())
}
