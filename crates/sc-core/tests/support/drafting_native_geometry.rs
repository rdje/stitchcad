//! Complete mixed consumers retain the original direct field producer and actual programme slots.
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
fn point(n: u128) -> PointRef {
    PointRef::new(id(n), LocalTag::FIRST)
}
fn doc(text: &str) -> TestResult<FormulaExpressionDocument> {
    Ok(FormulaExpressionDocument::new(text.into())?)
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
fn same_input(a: DraftingOperationInput<'_>, b: DraftingOperationInput<'_>) -> bool {
    use DraftingOperationInput as I;
    match (a, b) {
        (I::ConstructPoint(a), I::ConstructPoint(b)) => std::ptr::eq(a, b),
        (I::DrawLine(a), I::DrawLine(b)) => std::ptr::eq(a, b),
        (I::DrawArc(a), I::DrawArc(b)) => std::ptr::eq(a, b),
        (I::DrawBezier(a), I::DrawBezier(b)) => std::ptr::eq(a, b),
        (I::OffsetEdge(a), I::OffsetEdge(b)) => std::ptr::eq(a, b),
        (I::SlashSpread(a), I::SlashSpread(b)) => std::ptr::eq(a, b),
        (
            I::CloseDart {
                operation: a,
                request: ar,
            },
            I::CloseDart {
                operation: b,
                request: br,
            },
        ) => a == b && std::ptr::eq(ar, br),
        (
            I::AddNotch {
                operation: a,
                request: ar,
            },
            I::AddNotch {
                operation: b,
                request: br,
            },
        ) => a == b && std::ptr::eq(ar, br),
        (I::PlaceGrainline(a), I::PlaceGrainline(b)) => std::ptr::eq(a, b),
        (I::WalkSeam(a), I::WalkSeam(b)) => std::ptr::eq(a, b),
        (I::TrueSeam(a), I::TrueSeam(b)) => std::ptr::eq(a, b),
        (I::MirrorPiece(a), I::MirrorPiece(b)) => std::ptr::eq(a, b),
        (I::JoinPieces(a), I::JoinPieces(b)) => std::ptr::eq(a, b),
        _ => false,
    }
}
#[test]
fn native_geometry_keeps_complete_mixed_consumers_all_families_and_original_selectors() -> TestResult
{
    for mode in 0..3 {
        let root = root_with_recipe(
            "let private_width:length=1 mm/0\nlet middle:length=2 mm/0\nlet last:length=3 mm/0",
        )?;
        let source = Source::new(&root);
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
            .map_err(|e| e.to_string())?;
        let range = DirectedRange {
            range: EdgeRange::whole(EdgeRef::new(id(9000), LocalTag::new(23))),
            direction: Direction::Reversed,
        };
        let p = PointConstruction::new(id(1001), doc("private_width")?, doc("1 mm/0")?);
        let line = LineConstruction::new(id(1002), point(91), point(91))?;
        // Equal identities and fields do not make these two distinct original inputs interchangeable.
        let repeated_line = LineConstruction::new(id(1002), point(91), point(91))?;
        let arc = CenterArcConstruction::new(
            id(1003),
            point(92),
            doc("private_width")?,
            doc("0 deg")?,
            doc("90 deg")?,
        )?;
        let bezier = BezierConstruction::new(
            id(1004),
            if mode == 1 {
                BezierControlPoints::Quadratic {
                    start: point(93),
                    control: point(94),
                    end: point(93),
                }
            } else {
                BezierControlPoints::Cubic {
                    start: point(93),
                    first_control: point(94),
                    second_control: point(93),
                    end: point(95),
                }
            },
        )?;
        let offset = OffsetConstruction::new(id(1005), range, OffsetSide::Left, id(8005))?;
        let slash = if mode == 1 {
            SlashSpreadConstruction::rotation(
                id(1006),
                id(8006),
                range,
                SlashSpreadSide::Right,
                point(96),
                doc("-450 deg")?,
            )?
        } else {
            SlashSpreadConstruction::separation(
                id(1006),
                id(8006),
                range,
                SlashSpreadSide::Right,
                range,
                doc("private_width")?,
            )?
        };
        let dart = CloseDart::new(id(8007));
        let notch = AddNotch::new(
            id(8008),
            EdgeAnchor {
                edge: range.range.edge(),
                param: Param::END,
            },
            bindings(),
        );
        let grain = GrainlinePlacement::new(id(1009), id(8009))?;
        let pair = WalkBalancePair::new(
            WalkBalanceNotch::new(
                EdgeAnchor {
                    edge: range.range.edge(),
                    param: Param::START,
                },
                bindings(),
            ),
            WalkBalanceNotch::new(
                EdgeAnchor {
                    edge: range.range.edge(),
                    param: Param::END,
                },
                bindings(),
            ),
        );
        let walk = WalkSeamEdit::new(id(1010), id(8010), vec![pair; 3]);
        let truing =
            ConstrainedSeamTruing::new(id(1011), id(8011), SeamSideId::B, id(7011), id(6011))?;
        let mirror = MirrorPieceCopy::new(
            id(1012),
            id(8012),
            if mode == 1 {
                MirrorAxis::ThroughPoints {
                    start: point(97),
                    end: point(98),
                }
            } else {
                MirrorAxis::AlongRange(range)
            },
        )?;
        let join = JoinPiecesAtSeam::new(id(1013), id(8013), id(7013), id(6013))?;
        use DraftingOperationInput as I;
        let native = [
            I::ConstructPoint(&p),
            I::DrawLine(&line),
            I::DrawArc(&arc),
            I::DrawBezier(&bezier),
            I::OffsetEdge(&offset),
            I::SlashSpread(&slash),
            I::CloseDart {
                operation: id(1007),
                request: &dart,
            },
            I::AddNotch {
                operation: id(1008),
                request: &notch,
            },
            I::PlaceGrainline(&grain),
            I::WalkSeam(&walk),
            I::TrueSeam(&truing),
            I::MirrorPiece(&mirror),
            I::JoinPieces(&join),
            I::DrawLine(&repeated_line),
        ];
        let mut steps = vec![DraftingProgramStep::Scalar { ordinal: 1 }];
        if mode != 0 {
            steps.extend(
                native[..2]
                    .iter()
                    .copied()
                    .map(DraftingProgramStep::Drafting),
            );
        }
        steps.push(DraftingProgramStep::Scalar { ordinal: 2 });
        if mode != 0 {
            steps.extend(
                native[2..]
                    .iter()
                    .copied()
                    .map(DraftingProgramStep::Drafting),
            );
        }
        steps.push(DraftingProgramStep::Scalar { ordinal: 3 });
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
        let count = if mode == 0 { 0 } else { 20 };
        let slots = if mode == 0 { 3 } else { 17 };
        let gp = DraftingProgramGeometryLimits::new(
            limits(100000, 100000, 100000)?,
            count,
            count * 2,
            count * slots,
        );
        let work = bound.admit_geometry_work(&gp).map_err(|e| e.to_string())?;
        assert_eq!(
            (
                work.shape().formula_count(),
                work.shape().native_count(),
                work.shape().step_count(),
                work.shape().occurrence_count(),
                work.shape().retained_record_bound(),
                work.shape().creator_inspection_bound()
            ),
            (0, count, slots, count, count * 2, count * slots)
        );
        assert_eq!(
            source.trace.get(),
            2,
            "only the fresh resource walk runs before geometry work admission"
        );
        source.trace.set(0);
        let uses: Vec<_> = bound.native_geometry_dependencies().collect();
        let mut expected = vec![];
        if mode != 0 {
            let counts = [
                0,
                2,
                1,
                if mode == 1 { 3 } else { 4 },
                1,
                2,
                0,
                1,
                0,
                6,
                0,
                if mode == 1 { 2 } else { 1 },
                0,
                2,
            ];
            for (index, count) in counts.into_iter().enumerate() {
                expected
                    .extend((0..count).map(|field| (index + if index < 2 { 2 } else { 3 }, field)));
            }
        }
        assert_eq!(
            uses.iter()
                .map(|u| (u.position(), u.native().index()))
                .collect::<Vec<_>>(),
            expected
        );
        for use_ in &uses {
            assert!(std::ptr::eq(
                use_.step(),
                order
                    .steps()
                    .get(use_.position() - 1)
                    .ok_or("checked consumer")?
            ));
            let input = *native
                .get(use_.position() - if use_.position() < 4 { 2 } else { 3 })
                .ok_or("original native input")?;
            assert!(same_input(use_.native().input(), input));
            assert_eq!(use_.reference(), use_.native().reference());
            if let DraftingNativeGeometryField::WalkAnchor {
                pair_index,
                pair,
                side,
                anchor,
            } = use_.native().field()
            {
                let original = walk.pairs().get(pair_index).ok_or("original pair")?;
                assert!(std::ptr::eq(pair, original));
                let notch = if side == SeamSideId::A {
                    original.mark(SeamSideId::A)
                } else {
                    original.mark(SeamSideId::B)
                };
                assert!(std::ptr::eq(anchor, notch.anchor()));
                assert_eq!(
                    anchor.param,
                    if side == SeamSideId::A {
                        Param::START
                    } else {
                        Param::END
                    }
                );
            }
            let debug = format!("{use_:?}");
            assert!(!debug.contains("private_width") && !debug.contains("450"));
        }
        for (index, expected) in uses.iter().enumerate() {
            let actual = bound
                .native_geometry_dependencies()
                .nth(index)
                .ok_or("nth native field")?;
            assert!(std::ptr::eq(actual.step(), expected.step()));
            assert_eq!(actual.native().index(), expected.native().index());
        }
        let mut end = bound.native_geometry_dependencies().skip(uses.len());
        for _ in 0..3 {
            assert!(end.next().is_none());
        }
        assert_eq!(
            source.trace.get(),
            0,
            "dependency inspection cannot query sources or values"
        );
    }
    Ok(())
}
