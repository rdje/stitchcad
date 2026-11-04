//! Complete raw native semantic fields,actual input owners and original pair/Profile roles.
use super::{
    native_geometry::{bindings, same_input},
    TestResult,
};
use sc_core::{
    command::{AddNotch, CloseDart},
    construction::*,
    ontology::*,
    recipe::FormulaExpressionDocument,
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
fn profile_field(
    profile: &NotchProfileBindings,
    role: DraftingNotchProfileRole,
) -> &ProfileParameterRef {
    use DraftingNotchProfileRole as R;
    match role {
        R::Style => &profile.style,
        R::SampleDepth => &profile.sample.depth,
        R::SampleWidth => &profile.sample.width,
        R::ProductionDepth => &profile.production.depth,
        R::ProductionWidth => &profile.production.width,
        R::Encoding => &profile.encoding,
    }
}
#[test]
fn all_thirteen_families_preserve_every_target_profile_role_and_original_input() -> TestResult {
    use DraftingNativeSemanticTargetRole as R;
    use DraftingOperationInput as I;
    for rotated in [false, true] {
        let range = DirectedRange {
            range: EdgeRange::whole(EdgeRef::new(id(9000), LocalTag::new(23))),
            direction: Direction::Reversed,
        };
        let p = PointConstruction::new(id(1001), doc("1 mm/0")?, doc("2 mm/0")?);
        let line = LineConstruction::new(id(1002), point(91), point(91))?;
        let arc = CenterArcConstruction::new(
            id(1003),
            point(92),
            doc("1 mm/0")?,
            doc("0 deg")?,
            doc("90 deg")?,
        )?;
        let bezier = BezierConstruction::new(
            id(1004),
            if rotated {
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
        let slash = if rotated {
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
                doc("1 mm/0")?,
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
        let walk = WalkSeamEdit::new(id(1010), id(8010), vec![pair; 2]);
        let truing =
            ConstrainedSeamTruing::new(id(1011), id(8011), SeamSideId::B, id(7011), id(6011))?;
        let mirror = MirrorPieceCopy::new(
            id(1012),
            id(8012),
            if rotated {
                MirrorAxis::ThroughPoints {
                    start: point(97),
                    end: point(98),
                }
            } else {
                MirrorAxis::AlongRange(range)
            },
        )?;
        let join = JoinPiecesAtSeam::new(id(1013), id(8013), id(7013), id(6013))?;
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
        ];
        let mut targets = vec![];
        let mut profiles = vec![];
        let mut walk_fields = vec![];
        let mut total = 0;
        for (input, expected) in native
            .into_iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 7, 1, 25, 3, 1, 3])
        {
            let mut count = 0;
            let mut uses = input.semantic_dependencies();
            for use_ in uses.by_ref() {
                assert!(same_input(use_.input(), input));
                count += 1;
                total += 1;
                match use_.field() {
                    DraftingNativeSemanticField::Target { role, reference } => {
                        targets.push((role, *reference))
                    }
                    DraftingNativeSemanticField::NotchProfile { role, reference } => {
                        assert_eq!(*reference, *profile_field(&bindings(), role));
                        profiles.push(role);
                    }
                    DraftingNativeSemanticField::WalkNotchProfile {
                        pair_index,
                        side,
                        role,
                        reference,
                    } => {
                        let actual = walk
                            .pairs()
                            .get(pair_index)
                            .ok_or("actual pair")?
                            .mark(side)
                            .representation();
                        assert!(core::ptr::eq(reference, profile_field(actual, role)));
                        walk_fields.push((pair_index, side, role));
                    }
                }
            }
            assert_eq!(count, expected);
            assert!(uses.next().is_none());
            assert!(uses.next().is_none());
        }
        assert_eq!(total, 43);
        assert_eq!(
            targets,
            [
                (R::OffsetAllowance, id(8005)),
                (R::SlashPiece, id(8006)),
                (R::Dart, id(8007)),
                (R::NotchPiece, id(8008)),
                (R::Grainline, id(8009)),
                (R::WalkSpan, id(8010)),
                (R::TruingSpan, id(8011)),
                (R::TruingPiece, id(7011)),
                (R::TruingFitTolerance, id(6011)),
                (R::MirrorPiece, id(8012)),
                (R::JoinSpan, id(8013)),
                (R::JoinPieceA, id(7013)),
                (R::JoinPieceB, id(6013))
            ]
        );
        let roles = [
            DraftingNotchProfileRole::Style,
            DraftingNotchProfileRole::SampleDepth,
            DraftingNotchProfileRole::SampleWidth,
            DraftingNotchProfileRole::ProductionDepth,
            DraftingNotchProfileRole::ProductionWidth,
            DraftingNotchProfileRole::Encoding,
        ];
        assert_eq!(profiles, roles);
        let expected = (0..2)
            .flat_map(|index| {
                [SeamSideId::A, SeamSideId::B]
                    .into_iter()
                    .flat_map(move |side| roles.into_iter().map(move |role| (index, side, role)))
            })
            .collect::<Vec<_>>();
        assert_eq!(walk_fields, expected);
    }
    Ok(())
}
#[test]
fn equal_ids_in_distinct_inputs_keep_distinct_actual_target_and_profile_field_loans() -> TestResult
{
    let axis = MirrorAxis::ThroughPoints {
        start: point(90),
        end: point(91),
    };
    let mirrors = [MirrorPieceCopy::new(id(10), id(100), axis)?; 2];
    let mut targets = vec![];
    for input in &mirrors {
        let use_ = DraftingOperationInput::MirrorPiece(input)
            .semantic_dependencies()
            .next()
            .ok_or("original target")?;
        assert!(
            matches!(use_.input(),DraftingOperationInput::MirrorPiece(actual) if core::ptr::eq(actual,input))
        );
        let DraftingNativeSemanticField::Target { reference, .. } = use_.field() else {
            return Err("actual Piece target".into());
        };
        targets.push(reference);
    }
    let [a, b]: [&EntityId; 2] = targets.try_into().map_err(|_| "two targets")?;
    assert_eq!(a, b);
    assert!(!core::ptr::eq(a, b));
    let notches = [AddNotch::new(
        id(100),
        EdgeAnchor {
            edge: EdgeRef::new(id(1), LocalTag::FIRST),
            param: Param::START,
        },
        bindings(),
    ); 2];
    let mut styles = vec![];
    for request in &notches {
        let use_ = DraftingOperationInput::AddNotch {
            operation: id(11),
            request,
        }
        .semantic_dependencies()
        .nth(1)
        .ok_or("original style")?;
        let DraftingNativeSemanticField::NotchProfile {
            role: DraftingNotchProfileRole::Style,
            reference,
        } = use_.field()
        else {
            return Err("actual style".into());
        };
        styles.push(reference);
    }
    let [a, b]: [&ProfileParameterRef; 2] = styles.try_into().map_err(|_| "two style fields")?;
    assert_eq!(a, b);
    assert!(!core::ptr::eq(a, b));
    Ok(())
}
#[test]
fn empty_walk_still_retains_its_actual_span_without_fabricated_profile_occurrences() -> TestResult {
    let walk = WalkSeamEdit::new(id(10), id(20), vec![]);
    let mut uses = DraftingOperationInput::WalkSeam(&walk).semantic_dependencies();
    assert!(
        matches!(uses.next().ok_or("actual span")?.field(),DraftingNativeSemanticField::Target{role:DraftingNativeSemanticTargetRole::WalkSpan,reference} if *reference==id(20))
    );
    assert!(uses.next().is_none());
    assert!(uses.next().is_none());
    Ok(())
}
