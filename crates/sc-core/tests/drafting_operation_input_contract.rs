//! Raw source composition,not frame/currentness or accepted-program evidence.
use sc_core::{
    command::{AddNotch, CloseDart},
    construction::*,
    ontology::*,
    recipe::{FormulaExpressionDocument as Doc, FormulaExpressionDocumentError},
};
use std::{collections::HashSet, error::Error};

fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
fn point(value: u128) -> PointRef {
    PointRef::new(id(value), LocalTag::new(17))
}
fn range() -> DirectedRange {
    DirectedRange {
        range: EdgeRange::whole(EdgeRef::new(id(9000), LocalTag::new(23))),
        direction: Direction::Reversed,
    }
}
fn doc(text: &str) -> Result<Doc, FormulaExpressionDocumentError> {
    Doc::new(text.into())
}
fn bindings() -> NotchProfileBindings {
    let parameter = |value| ProfileParameterRef::new(id(value));
    NotchProfileBindings {
        style: parameter(31),
        sample: NotchDimensionBindings {
            depth: parameter(32),
            width: parameter(33),
        },
        production: NotchDimensionBindings {
            depth: parameter(34),
            width: parameter(35),
        },
        encoding: parameter(36),
    }
}

#[test]
fn mixed_family_views_keep_each_actual_source_and_history_identity() -> Result<(), Box<dyn Error>> {
    use DraftingOperationInput as I;
    let p = PointConstruction::new(id(1001), doc("1 mm/0")?, doc("2 mm")?);
    let line = LineConstruction::new(id(1002), point(9001), point(9002))?;
    let arc = CenterArcConstruction::new(
        id(1003),
        point(9003),
        doc("5 mm")?,
        doc("0 deg")?,
        doc("90 deg")?,
    )?;
    let bezier = BezierConstruction::new(
        id(1004),
        BezierControlPoints::Quadratic {
            start: point(9004),
            control: point(9005),
            end: point(9004),
        },
    )?;
    let offset = OffsetConstruction::new(id(1005), range(), OffsetSide::Left, id(8005))?;
    let slash = SlashSpreadConstruction::rotation(
        id(1006),
        id(8006),
        range(),
        SlashSpreadSide::Right,
        point(9006),
        doc("-450 deg")?,
    )?;
    let dart = CloseDart::new(id(8007));
    let notch = AddNotch::new(
        id(8008),
        EdgeAnchor {
            edge: range().range.edge(),
            param: Param::END,
        },
        bindings(),
    );
    let grainline = GrainlinePlacement::new(id(1009), id(8009))?;
    let walk = WalkSeamEdit::new(id(1010), id(8010), Vec::new());
    let truing = ConstrainedSeamTruing::new(id(1011), id(8011), SeamSideId::B, id(7011), id(6011))?;
    let mirror = MirrorPieceCopy::new(id(1012), id(8012), MirrorAxis::AlongRange(range()))?;
    let join = JoinPiecesAtSeam::new(id(1013), id(8013), id(7013), id(6013))?;
    // This array is raw caller order,not an accepted or renumbered program.
    let inputs = [
        I::JoinPieces(&join),
        I::ConstructPoint(&p),
        I::CloseDart {
            operation: id(1007),
            request: &dart,
        },
        I::DrawLine(&line),
        I::MirrorPiece(&mirror),
        I::AddNotch {
            operation: id(1008),
            request: &notch,
        },
        I::DrawArc(&arc),
        I::PlaceGrainline(&grainline),
        I::DrawBezier(&bezier),
        I::TrueSeam(&truing),
        I::OffsetEdge(&offset),
        I::WalkSeam(&walk),
        I::SlashSpread(&slash),
    ];
    let expected = [
        1013, 1001, 1007, 1002, 1012, 1008, 1003, 1009, 1004, 1011, 1005, 1010, 1006,
    ];
    for (input, identity) in inputs.into_iter().zip(expected) {
        assert_eq!(input.operation(), id(identity));
        let same_source = match input {
            I::ConstructPoint(actual) => std::ptr::eq(actual, &p),
            I::DrawLine(actual) => std::ptr::eq(actual, &line),
            I::DrawArc(actual) => std::ptr::eq(actual, &arc),
            I::DrawBezier(actual) => std::ptr::eq(actual, &bezier),
            I::OffsetEdge(actual) => std::ptr::eq(actual, &offset),
            I::SlashSpread(actual) => std::ptr::eq(actual, &slash),
            I::CloseDart { request, .. } => std::ptr::eq(request, &dart),
            I::AddNotch { request, .. } => std::ptr::eq(request, &notch),
            I::PlaceGrainline(actual) => std::ptr::eq(actual, &grainline),
            I::WalkSeam(actual) => std::ptr::eq(actual, &walk),
            I::TrueSeam(actual) => std::ptr::eq(actual, &truing),
            I::MirrorPiece(actual) => std::ptr::eq(actual, &mirror),
            I::JoinPieces(actual) => std::ptr::eq(actual, &join),
        };
        assert!(same_source);
    }
    assert_eq!(
        inputs
            .map(|input| input.family())
            .into_iter()
            .collect::<HashSet<_>>()
            .len(),
        13
    );
    Ok(())
}

#[test]
fn semantic_operation_identity_is_explicit_and_never_taken_from_target_or_new_notch() {
    let dart = CloseDart::new(id(700));
    let notch = AddNotch::new(
        id(701),
        EdgeAnchor {
            edge: range().range.edge(),
            param: Param::START,
        },
        bindings(),
    );
    for operation in [id(0), id(700), id(5000)] {
        let close = DraftingOperationInput::CloseDart {
            operation,
            request: &dart,
        };
        let add = DraftingOperationInput::AddNotch {
            operation,
            request: &notch,
        };
        assert_eq!(close.operation(), operation);
        assert_eq!(add.operation(), operation);
        assert_eq!(dart.dart(), id(700));
        assert_eq!(notch.piece(), id(701));
        assert_eq!(notch.representation(), bindings());
        assert_eq!(notch.definition_with_id(id(999)).id, id(999));
        assert_ne!(add.operation(), id(999));
    }
}

#[test]
fn inspecting_or_copying_an_input_neither_reads_values_nor_discloses_authored_documents(
) -> Result<(), Box<dyn Error>> {
    let point = PointConstruction::new(id(44), doc("private_customer_length/0")?, doc("1 mm/0")?);
    let original = point.clone();
    let input = DraftingOperationInput::ConstructPoint(&point);
    let copied = input;
    assert_eq!(copied.operation(), id(44));
    assert_eq!(copied.family(), DraftingOperationFamily::ConstructPoint);
    assert!(
        matches!(copied,DraftingOperationInput::ConstructPoint(actual) if std::ptr::eq(actual,&point))
    );
    let debug = format!("{copied:?}");
    assert!(debug.contains("ConstructPoint"));
    assert!(!debug.contains("private_customer_length") && !debug.contains("1 mm/0"));
    assert_eq!(point, original);
    Ok(())
}

#[test]
fn cubic_layout_and_repeated_sources_remain_in_the_original_declaration(
) -> Result<(), Box<dyn Error>> {
    let original = BezierControlPoints::Cubic {
        start: point(93),
        first_control: point(91),
        second_control: point(93),
        end: point(90),
    };
    let curve = BezierConstruction::new(id(10), original)?;
    let input = DraftingOperationInput::DrawBezier(&curve);
    let DraftingOperationInput::DrawBezier(actual) = input else {
        unreachable!("explicit raw variant");
    };
    assert!(std::ptr::eq(actual, &curve));
    assert_eq!(actual.degree(), BezierDegree::Cubic);
    assert_eq!(actual.dependencies().count(), 4);
    assert_eq!(input.operation(), id(10));
    Ok(())
}
