//! Complete creator classification uses genuine original owners, before any source callback.
use super::{limits, root_with_recipe, scope, Source, TestResult};
use sc_core::{
    command::{AddNotch, CloseDart},
    construction::*,
    ontology::*,
    recipe::*,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn p(n: u128) -> PointRef {
    PointRef::new(id(n), LocalTag::FIRST)
}
fn e(n: u128) -> EdgeRef {
    EdgeRef::new(id(n), LocalTag::FIRST)
}
fn range(n: u128) -> DirectedRange {
    DirectedRange {
        range: EdgeRange::whole(e(n)),
        direction: Direction::Reversed,
    }
}
fn doc(s: &str) -> TestResult<FormulaExpressionDocument> {
    Ok(FormulaExpressionDocument::new(s.into())?)
}
fn bindings() -> NotchProfileBindings {
    let p = |n| ProfileParameterRef::new(id(n));
    NotchProfileBindings {
        style: p(31),
        sample: NotchDimensionBindings {
            depth: p(32),
            width: p(33),
        },
        production: NotchDimensionBindings {
            depth: p(34),
            width: p(35),
        },
        encoding: p(36),
    }
}
#[test]
fn geometry_requests_classify_complete_creators_fixed_outputs_unnamed_roles_and_refusals(
) -> TestResult {
    use DraftingGeometryReference as R;
    use DraftingOperationInput as I;
    use DraftingProgramGeometryCreatorRefusal as E;
    for mode in 0..22 {
        let root = root_with_recipe("let private_width:length=1 mm/0")?;
        let source = Source::new(&root);
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
            .map_err(|e| e.to_string())?;
        let point = PointConstruction::new(id(1001), doc("1 mm/0")?, doc("2 mm")?);
        let duplicate = PointConstruction::new(id(1001), doc("1 mm/0")?, doc("2 mm")?);
        let future = PointConstruction::new(id(1003), doc("1 mm")?, doc("2 mm")?);
        let line = LineConstruction::new(id(1002), p(80), p(81))?;
        let arc = CenterArcConstruction::new(
            id(1002),
            p(80),
            doc("1 mm")?,
            doc("0 deg")?,
            doc("90 deg")?,
        )?;
        let bezier = BezierConstruction::new(
            id(1002),
            BezierControlPoints::Quadratic {
                start: p(80),
                control: p(81),
                end: p(80),
            },
        )?;
        let offset = OffsetConstruction::new(id(1002), range(900), OffsetSide::Left, id(800))?;
        let slash = SlashSpreadConstruction::rotation(
            id(1002),
            id(800),
            range(900),
            SlashSpreadSide::Left,
            p(80),
            doc("90 deg")?,
        )?;
        let dart = CloseDart::new(id(800));
        let notch = AddNotch::new(
            id(800),
            EdgeAnchor {
                edge: e(900),
                param: Param::START,
            },
            bindings(),
        );
        let grain = GrainlinePlacement::new(id(1002), id(800))?;
        let walk = WalkSeamEdit::new(id(1002), id(800), vec![]);
        let truing =
            ConstrainedSeamTruing::new(id(1002), id(800), SeamSideId::A, id(801), id(802))?;
        let mirror = MirrorPieceCopy::new(
            id(1002),
            id(800),
            MirrorAxis::ThroughPoints {
                start: p(80),
                end: p(81),
            },
        )?;
        let join = JoinPiecesAtSeam::new(id(1002), id(800), id(801), id(802))?;
        let candidate = match mode {
            6 | 11 => I::DrawLine(&line),
            12 => I::DrawArc(&arc),
            13 => I::DrawBezier(&bezier),
            7 => I::CloseDart {
                operation: id(1002),
                request: &dart,
            },
            8 | 14 => I::OffsetEdge(&offset),
            15 => I::SlashSpread(&slash),
            16 => I::TrueSeam(&truing),
            17 => I::JoinPieces(&join),
            18 => I::AddNotch {
                operation: id(1002),
                request: &notch,
            },
            19 => I::PlaceGrainline(&grain),
            20 => I::WalkSeam(&walk),
            _ => I::MirrorPiece(&mirror),
        };
        let requested = match mode {
            0 => R::Point(p(80)),
            1 | 4 => R::Point(p(1001)),
            2 => R::Edge(e(2000)),
            3 => R::Point(p(1003)),
            5 => R::Point(PointRef::new(id(1001), LocalTag::new(77))),
            6..=9 | 18..=20 => R::Point(p(1002)),
            21 => R::Edge(e(1001)),
            _ => R::Edge(e(1002)),
        };
        let consumer_line = LineConstruction::new(
            id(2000),
            match requested {
                R::Point(p) => p,
                R::Edge(_) => p(80),
            },
            p(82),
        )?;
        let consumer_offset = OffsetConstruction::new(
            id(2000),
            match requested {
                R::Edge(e) => DirectedRange {
                    range: EdgeRange::whole(if mode == 2 { self::e(900) } else { e }),
                    direction: Direction::Original,
                },
                R::Point(_) => range(900),
            },
            OffsetSide::Left,
            id(800),
        )?;
        let consumer_notch = AddNotch::new(
            id(800),
            EdgeAnchor {
                edge: e(2000),
                param: Param::END,
            },
            bindings(),
        );
        let consumer = if mode == 2 {
            I::AddNotch {
                operation: id(2000),
                request: &consumer_notch,
            }
        } else if matches!(requested, R::Edge(_)) {
            I::OffsetEdge(&consumer_offset)
        } else {
            I::DrawLine(&consumer_line)
        };
        let mut steps = vec![
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Drafting(I::ConstructPoint(&point)),
            DraftingProgramStep::Drafting(candidate),
            DraftingProgramStep::Drafting(consumer),
            DraftingProgramStep::Drafting(I::ConstructPoint(&future)),
        ];
        if mode == 4 {
            steps.push(DraftingProgramStep::Drafting(I::ConstructPoint(&duplicate)));
        }
        let raw = DraftingProgramInputs::new(context, steps);
        let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 100, 10000, 100000);
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
        let np = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 64, 128);
        let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        let namespace = initial
            .bind_initial_namespace()
            .map_err(|e| e.to_string())?;
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(0, 0, 0, 0))
            .map_err(|e| e.to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(0, 0, 1))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(0, 0))
            .map_err(|e| e.to_string())?;
        let op = DraftingProgramOrderLimits::new(100, 10000, 100000);
        let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
        let inputs = sources
            .dependency_inputs(&order, &graph)
            .map_err(|e| e.to_string())?;
        let cp = DraftingProgramCanonicalLimits::new(100, 100);
        let bound = inputs
            .bind_canonical_inputs(&cp)
            .map_err(|e| e.to_string())?;
        source.trace.set(0);
        let requests: Vec<_> = bound.geometry_requests().collect();
        assert!(requests
            .iter()
            .zip(requests.iter().skip(1))
            .all(|(a, b)| a.occurrence().position() <= b.occurrence().position()));
        let request=requests.iter().find(|r|matches!(r.occurrence(),DraftingProgramGeometryOccurrence::Native(u) if u.position()==4&&u.native().index()==0)).ok_or("original consumer request")?;
        assert_eq!(request.occurrence().reference(), requested);
        let DraftingProgramGeometryOccurrence::Native(u) = request.occurrence() else {
            return Err("native occurrence".into());
        };
        assert!(std::ptr::eq(
            u.step(),
            order.steps().get(3).ok_or("actual checked consumer")?
        ));
        let expected_step = match (mode, request.target()) {
            (0, Ok(DraftingProgramGeometryTarget::Existing)) => None,
            (1 | 9..=17, Ok(DraftingProgramGeometryTarget::Earlier(r))) => {
                let expected = if mode == 1 { 2 } else { 3 };
                assert_eq!(r.step().position(), expected);
                assert_eq!(r.reference(), requested);
                assert_eq!(r.requires_role_binding(), matches!(mode, 9 | 10 | 14..=17));
                assert_eq!(
                    r.input().operation(),
                    if mode == 1 { id(1001) } else { id(1002) }
                );
                if mode == 1 {
                    let I::ConstructPoint(actual) = r.input() else {
                        return Err("actual point".into());
                    };
                    assert!(std::ptr::eq(actual, &point));
                }
                Some(r.step())
            }
            (2, Err(E::SelfCreator { step })) => {
                assert_eq!(step.position(), 4);
                Some(step)
            }
            (3, Err(E::FutureCreator { step })) => {
                assert_eq!(step.position(), 5);
                Some(step)
            }
            (4, Err(E::Ambiguous { first, second })) => {
                assert_eq!((first.position(), second.position()), (2, 6));
                Some(first)
            }
            (5, Err(E::FixedOutput { step, actual })) => {
                assert_eq!(actual, R::Point(point.output()));
                Some(step)
            }
            (6, Err(E::FixedOutput { step, actual })) => {
                assert_eq!(actual, R::Edge(line.output()));
                Some(step)
            }
            (7 | 18..=20, Err(E::Unsupported { step })) => {
                assert_eq!(step.position(), 3);
                Some(step)
            }
            (8, Err(E::OutputKind { step })) => {
                assert_eq!(step.position(), 3);
                Some(step)
            }
            (21, Err(E::FixedOutput { step, actual })) => {
                assert_eq!(actual, R::Point(point.output()));
                Some(step)
            }
            (_, other) => return Err(format!("mode {mode}: {other:?}").into()),
        };
        if let Some(step) = expected_step {
            let DraftingProgramStepKind::Drafting(actual) = step.kind() else {
                return Err("native candidate".into());
            };
            assert!(std::ptr::eq(
                actual,
                program
                    .operations()
                    .get(step.position() - 1)
                    .and_then(Option::as_ref)
                    .ok_or("actual normalized slot")?
            ));
        }
        for (i, expected) in requests.iter().enumerate() {
            let actual = bound.geometry_requests().nth(i).ok_or("nth request")?;
            assert_eq!(
                actual.occurrence().position(),
                expected.occurrence().position()
            );
            assert_eq!(
                actual.occurrence().reference(),
                expected.occurrence().reference()
            );
        }
        let mut end = bound.geometry_requests().skip(requests.len());
        for _ in 0..3 {
            assert!(end.next().is_none());
        }
        assert_eq!(
            source.trace.get(),
            0,
            "classification must precede source callbacks"
        );
        assert!(!format!("{requests:?}").contains("private_width"));
    }
    Ok(())
}
