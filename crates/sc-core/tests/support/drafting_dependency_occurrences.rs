//! Original roles/spans and repeated/untaken name uses stream from actual whole mixed proofs.
use super::{limits, root_with_recipe, scope, Source, TestResult};
use sc_core::{
    construction::*,
    ontology::{DirectedRange, Direction, EdgeRange, EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{FormulaExpressionDocument, FormulaRecipeDependencyRole as ScalarRole},
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn doc(text: &str) -> TestResult<FormulaExpressionDocument> {
    Ok(FormulaExpressionDocument::new(text.into())?)
}
#[test]
fn mixed_occurrences_keep_native_roles_repeated_branches_headers_original_spans_and_consumers(
) -> TestResult {
    let text = "let width:length=1 mm\nlet turn:angle=0 deg\nassert closure:eps_geo=width==width";
    let root = root_with_recipe(text)?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(id(100), doc("if(1==1,width,width)")?, doc("width")?);
    let arc = CenterArcConstruction::new(
        id(101),
        PointRef::new(id(900), LocalTag::FIRST),
        doc("width")?,
        doc("turn")?,
        doc("turn")?,
    )?;
    let range = DirectedRange {
        range: EdgeRange::whole(EdgeRef::new(id(901), LocalTag::FIRST)),
        direction: Direction::Original,
    };
    let slash = SlashSpreadConstruction::separation(
        id(102),
        id(800),
        range,
        SlashSpreadSide::Left,
        range,
        doc("width")?,
    )?;
    let walk = WalkSeamEdit::new(id(103), id(800), vec![]);
    let raw = DraftingProgramInputs::new(
        context,
        vec![
            DraftingProgramStep::Scalar { ordinal: 1 },
            DraftingProgramStep::Scalar { ordinal: 2 },
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
            DraftingProgramStep::Drafting(DraftingOperationInput::DrawArc(&arc)),
            DraftingProgramStep::Drafting(DraftingOperationInput::SlashSpread(&slash)),
            DraftingProgramStep::Drafting(DraftingOperationInput::WalkSeam(&walk)),
            DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
            DraftingProgramStep::Scalar { ordinal: 3 },
        ],
    );
    let pp = DraftingProgramLimits::new(limits(10000, 10000, 10000)?, 1000, 10000, 100000);
    let program = raw
        .admit(&pp)
        .map_err(|e| e.to_string())?
        .normalized()
        .map_err(|e| e.to_string())?;
    let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
    let np = DraftingProgramNameLimits::new(limits(10000, 10000, 10000)?, 128, 256);
    let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
    let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
    let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
    let initial = roles.bind_initial_sources().map_err(|e| e.to_string())?;
    let namespace = initial
        .bind_initial_namespace()
        .map_err(|e| e.to_string())?;
    let op = DraftingProgramOrderLimits::new(1000, 10000, 100000);
    let proof = namespace.check_order(&op).map_err(|e| e.to_string())?;
    source.trace.set(0);
    let uses: Vec<_> = proof.dependencies().collect();
    use DraftingProgramDependencyRole as R;
    let expected = [
        (3, R::Point(PointCoordinate::X)),
        (3, R::Point(PointCoordinate::X)),
        (3, R::Point(PointCoordinate::Y)),
        (4, R::CenterArc(CenterArcArgument::Radius)),
        (4, R::CenterArc(CenterArcArgument::StartAngle)),
        (4, R::CenterArc(CenterArcArgument::SweepAngle)),
        (5, R::SlashSpread),
        (7, R::Point(PointCoordinate::X)),
        (7, R::Point(PointCoordinate::X)),
        (7, R::Point(PointCoordinate::Y)),
        (8, R::Scalar(ScalarRole::AssertionTolerance)),
        (8, R::Scalar(ScalarRole::AssertionLeft)),
        (8, R::Scalar(ScalarRole::AssertionRight)),
    ];
    assert_eq!(
        uses.iter()
            .map(|u| (u.position(), u.role()))
            .collect::<Vec<_>>(),
        expected
    );
    assert_eq!(
        uses.iter()
            .map(|u| u.declaration().name())
            .collect::<Vec<_>>(),
        [
            "width", "width", "width", "width", "turn", "turn", "width", "width", "width", "width",
            "eps_geo", "width", "width"
        ]
    );
    for use_ in &uses {
        let copied = *use_;
        assert!(std::ptr::eq(
            copied.step(),
            proof
                .steps()
                .get(copied.position() - 1)
                .ok_or("actual consumer")?
        ));
        assert_eq!(
            copied.statement_index(),
            if copied.position() == 8 {
                Some(3)
            } else {
                None
            }
        );
        let original = match copied.role() {
            R::Point(role) => point.coordinate(role).source(),
            R::CenterArc(role) => arc.argument(role).source(),
            R::SlashSpread => slash.opening().document().source(),
            R::Scalar(_) => text,
        };
        let span = copied.span();
        assert_eq!(
            original.get(span.start()..span.end()),
            Some(copied.declaration().name())
        );
        assert!(!format!("{copied:?}").contains("width"));
    }
    let mut iter = proof.dependencies();
    assert_eq!(iter.nth(1).ok_or("second branch")?.position(), 3);
    assert_eq!(iter.count(), 11);
    let mut exhausted = proof.dependencies();
    for _ in exhausted.by_ref() {}
    assert!(exhausted.next().is_none() && exhausted.next().is_none());
    let assertion = proof.steps().last().ok_or("actual assertion")?;
    let DraftingProgramOrderProofKind::Scalar(statement) = assertion.kind() else {
        return Err("actual scalar proof".into());
    };
    assert_eq!(
        statement
            .dependencies()
            .map(|u| u.role())
            .collect::<Vec<_>>(),
        [
            ScalarRole::AssertionTolerance,
            ScalarRole::AssertionLeft,
            ScalarRole::AssertionRight
        ]
    );
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
