//! Visible metadata/checking loans preserve real declarations without exposing the cursor map.
use sc_core::{
    construction::*,
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as B, FormulaDeclaration as D, FormulaDeclarationSource as S,
        FormulaExpressionDocument as Doc, FormulaGeometryDeclaration as G,
        FormulaInitialDeclaration as I, FormulaInputOrigin, FormulaKind as K, FormulaNameCursor,
        FormulaNamespace as N, FormulaNamespaceError as E, FormulaRecipe, FormulaReservedName,
        FormulaScalarInputOrigin, FormulaToleranceName,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use std::error::Error;
type TestResult = Result<(), Box<dyn Error>>;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn point() -> PointRef {
    PointRef::new(id(80), LocalTag::FIRST)
}
fn geometry<'a>(declaration: D<'a>) -> Result<G<'a>, String> {
    G::try_from(declaration).map_err(|_| "expected actual geometry metadata".into())
}

#[test]
fn visible_scope_reuses_expression_and_all_native_kernels_and_proofs_outlive_the_map() -> TestResult
{
    let recipe = FormulaRecipe::parse("let width:length=1 mm let next:length=width")?
        .normalize_literals()?;
    let p_name = MachineToken::new("p")?;
    let width = MachineToken::new("width")?;
    let expression = Doc::new("width+x(p)".into())?;
    let expression = expression.normalized()?;
    let point_input = PointConstruction::new(
        id(100),
        Doc::new("width".into())?,
        Doc::new("1 mm/0".into())?,
    );
    let point_arena = point_input
        .normalized(PointConstructionNormalizationLimits::new(4, 10))
        .map_err(|e| e.to_string())?;
    let arc_input = CenterArcConstruction::new(
        id(101),
        point(),
        Doc::new("width".into())?,
        Doc::new("0 deg".into())?,
        Doc::new("90 deg".into())?,
    )?;
    let arc_arena = arc_input
        .normalized(CenterArcNormalizationLimits::new(3, 9))
        .map_err(|e| e.to_string())?;
    let opening_input = SlashSpreadOpening::rotation(Doc::new("-450 deg".into())?);
    let opening_arena = opening_input
        .normalized(SlashSpreadNormalizationLimits::new(2, 5))
        .map_err(|e| e.to_string())?;
    let mut cursor = FormulaNameCursor::new(N::new([])?, &recipe);
    assert!(cursor.visible_names().resolve(&width).is_err());
    cursor
        .current()
        .map_err(|e| e.to_string())?
        .ok_or("first actual scalar")?
        .check_kinds()
        .map_err(|e| e.to_string())?;
    assert!(cursor.advance_metadata().map_err(|e| e.to_string())?);
    assert!(cursor
        .visible_names()
        .check_expression(&expression)
        .is_err());
    cursor
        .declare_geometry(geometry(D::point(&p_name, point()))?)
        .map_err(|e| e.to_string())?;
    assert_eq!(
        cursor
            .current()
            .map_err(|e| e.to_string())?
            .ok_or("second actual scalar")?
            .statement_index(),
        2
    );
    let proof = cursor
        .visible_names()
        .check_expression(&expression)
        .map_err(|e| e.to_string())?;
    let point_proof = point_arena
        .check_arguments_in_scope(cursor.visible_names())
        .map_err(|e| e.to_string())?;
    let arc_proof = arc_arena
        .check_arguments_in_scope(cursor.visible_names())
        .map_err(|e| e.to_string())?;
    let opening_proof = opening_arena
        .check_argument_in_scope(cursor.visible_names())
        .map_err(|e| e.to_string())?;
    assert!(cursor.advance_metadata().map_err(|e| e.to_string())?);
    assert!(cursor.current().map_err(|e| e.to_string())?.is_none());
    assert_eq!(
        cursor
            .visible_names()
            .resolve(&width)
            .map_err(|e| e.to_string())?
            .kind(),
        K::Length
    );
    drop(cursor);
    assert_eq!(proof.kind(), K::Length);
    let [width_use, point_use] = proof.dependencies() else {
        return Err("both original sourced names".into());
    };
    assert!(matches!(
        width_use.declaration().source(),
        S::Recipe {
            statement_index: 1,
            ..
        }
    ));
    assert!(matches!(point_use.declaration().source(),S::Point(reference) if reference==point()));
    assert_eq!(point_proof.coordinate(PointCoordinate::X).kind(), K::Length);
    assert_eq!(
        arc_proof.argument(CenterArcArgument::Radius).kind(),
        K::Length
    );
    assert_eq!(opening_proof.expression().kind(), K::Angle);
    Ok(())
}

#[test]
fn geometry_filter_and_insertions_keep_original_sources_positions_and_first_collisions(
) -> TestResult {
    let recipe = FormulaRecipe::parse("let width:length=1 mm let next:length=width")?
        .normalize_literals()?;
    let name = MachineToken::new("width")?;
    let reserved = MachineToken::new("eps_num")?;
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(201),
        source: id(202),
        state: LengthState::Unknown {
            observation: id(203),
        },
    })?;
    let length = D::length_input(&name, FormulaInputOrigin::Measurement, id(204), &record);
    let input = D::input(
        &name,
        FormulaScalarInputOrigin::Parameter,
        id(205),
        id(206),
        B::Count,
    );
    let recipe_declaration = D::recipe(&recipe, 1).ok_or("actual let metadata")?;
    for rejected in [
        length,
        input,
        recipe_declaration,
        D::reserved(FormulaReservedName::Tolerance(
            FormulaToleranceName::Numerical,
        )),
    ] {
        let original =
            G::try_from(rejected).expect_err("only point/edge metadata enters geometry port");
        assert_eq!(original.name(), rejected.name());
        assert_eq!(original.kind(), rejected.kind());
    }
    let edge = D::edge(&name, EdgeRef::new(id(81), LocalTag::FIRST));
    assert_eq!(geometry(edge)?.declaration().kind(), K::Edge);
    let initial = I::try_from(length).map_err(|_| "actual initial canonical length")?;
    let ns = N::new([])?
        .with_initial([initial])
        .map_err(|e| e.to_string())?;
    assert!(
        matches!(ns.visible_names().resolve(&name).map_err(|e|e.to_string())?.source(),S::LengthInput{declaration,..} if std::ptr::eq(declaration,&record))
    );
    let initial = I::try_from(input).map_err(|_| "actual initial scalar metadata")?;
    let error = ns
        .with_initial([initial])
        .expect_err("original initial sources collide");
    let [prior, attempted] = error.binding_sources();
    assert!(
        matches!(prior.source(),S::LengthInput{declaration,..} if std::ptr::eq(declaration,&record))
    );
    assert!(matches!(attempted.source(),S::Input{declaration,..} if declaration==id(206)));
    let mut cursor = FormulaNameCursor::new(N::new([])?, &recipe);
    cursor.advance_metadata().map_err(|e| e.to_string())?;
    let error = cursor
        .declare_geometry(geometry(D::point(&name, point()))?)
        .expect_err("geometry cannot shadow actual prior let");
    assert!(matches!(error, E::AmbiguousName { .. }));
    assert!(matches!(
        cursor
            .visible_names()
            .resolve(&name)
            .map_err(|e| e.to_string())?
            .source(),
        S::Recipe {
            statement_index: 1,
            ..
        }
    ));
    assert_eq!(
        cursor
            .current()
            .map_err(|e| e.to_string())?
            .ok_or("same current scalar after refusal")?
            .statement_index(),
        2
    );
    let error = cursor
        .declare_geometry(geometry(D::point(&reserved, point()))?)
        .expect_err("reserved source stays fixed");
    assert!(matches!(error, E::ReservedBinding { .. }));
    assert!(!format!("{:?}", cursor.visible_names()).contains("width"));
    Ok(())
}

#[test]
fn native_scope_adapter_retains_the_same_original_first_kind_refusal() -> TestResult {
    let input = PointConstruction::new(
        id(100),
        Doc::new("1 deg".into())?,
        Doc::new("missing_name".into())?,
    );
    let arena = input
        .normalized(PointConstructionNormalizationLimits::new(2, 6))
        .map_err(|e| e.to_string())?;
    let namespace = N::new([])?;
    let direct = arena
        .check_arguments(&namespace)
        .expect_err("wrong original X kind");
    let scoped = arena
        .check_arguments_in_scope(namespace.visible_names())
        .expect_err("same original X refusal");
    assert!(std::ptr::eq(scoped.normalized(), direct.normalized()));
    assert_eq!(scoped.coordinate(), direct.coordinate());
    assert_eq!(scoped.span(), direct.span());
    assert!(matches!(
        scoped.refusal(),
        PointConstructionCheckRefusal::Kind { actual: K::Angle }
    ));
    Ok(())
}
