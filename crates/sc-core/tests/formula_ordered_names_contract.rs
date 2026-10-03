//! Ordered metadata scopes retain actual recipe ownership, prefix visibility and refusal sources.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as B, FormulaDeclaration as D, FormulaDeclarationSource as S,
        FormulaInitialDeclaration as I, FormulaInputOrigin as L, FormulaKind as K,
        FormulaNameCursor as Cursor, FormulaNamespace as N, FormulaNamespaceError as E,
        FormulaNormalizedRecipe as Recipe, FormulaOrigin as O, FormulaRecipe,
        FormulaReservedContext as C, FormulaReservedName as R, FormulaScalarInputOrigin as P,
        FormulaStatementNameScope as Scope, FormulaToleranceName as T,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
fn normalized(source: &str) -> Recipe<'_> {
    FormulaRecipe::parse(source)
        .unwrap()
        .normalize_literals()
        .unwrap()
}
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn initial(d: D<'_>) -> I<'_> {
    I::try_from(d).unwrap()
}
fn current<'s, 'a>(cursor: &'s Cursor<'a>) -> Scope<'s, 'a> {
    let result = cursor.current();
    assert!(result.is_ok());
    let scope = result.unwrap();
    assert!(scope.is_some());
    scope.unwrap()
}
fn resolved<'a>(scope: Scope<'_, 'a>, name: &MachineToken) -> D<'a> {
    let result = scope.resolve(name);
    assert!(result.is_ok());
    result.unwrap()
}
fn missing(scope: Scope<'_, '_>, name: &MachineToken) {
    let result = scope.resolve(name);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.token(), "formula_unbound_name");
    assert_eq!(error.name(), name.as_str());
    assert_eq!(
        error.origins_searched(),
        &[
            O::Measurement,
            O::Ease,
            O::Parameter,
            O::Profile,
            O::Material,
            O::Geometry,
            O::Recipe,
            O::Size,
            O::Tolerance
        ]
    );
}
fn blocked<'a>(cursor: &Cursor<'a>) -> E<'a> {
    let result = cursor.current();
    assert!(result.is_err());
    result.unwrap_err()
}
fn assert_recipe(d: D<'_>, index: usize, name: &str, kind: B, source: &str, whole: &str) {
    assert_eq!(d.name(), name);
    assert_eq!(d.origin(), O::Recipe);
    assert_eq!(d.kind(), K::from(kind));
    let S::Recipe {
        statement_index,
        kind: annotation,
        span,
        name_span,
    } = d.source()
    else {
        assert!(matches!(d.source(), S::Recipe { .. }));
        return;
    };
    assert_eq!(statement_index, index);
    assert_eq!(annotation, kind);
    let start = source.find(whole).unwrap();
    assert_eq!(span.start(), start);
    assert_eq!(span.end(), start + whole.len());
    let offset = whole.find(name).unwrap();
    assert_eq!(name_span.start(), start + offset);
    assert_eq!(name_span.end(), start + offset + name.len());
}

#[test]
fn scopes_keep_exact_owner_and_only_initial_or_prior_names_with_assertion_gaps() {
    let source = " \nlet first:length=unknown_rhs\nassert label:eps_fmt=first==first\nlet second:angle=90 deg\nassert last:eps_num=first==first\t";
    let recipe = normalized(source);
    let input = MachineToken::new("body_width").unwrap();
    let canonical = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(1),
        source: id(2),
        state: LengthState::Unknown { observation: id(3) },
    })
    .unwrap();
    let initial_ns = N::new([initial(D::length_input(
        &input,
        L::Measurement,
        id(4),
        &canonical,
    ))])
    .unwrap();
    let original_ns = initial_ns.clone();
    let mut cursor = Cursor::new(initial_ns, &recipe);
    let first = MachineToken::new("first").unwrap();
    let second = MachineToken::new("second").unwrap();
    let label = MachineToken::new("label").unwrap();
    let last = MachineToken::new("last").unwrap();
    for (position, actual) in recipe.statements().iter().enumerate() {
        let scope = current(&cursor);
        assert_eq!(scope.statement_index(), position + 1);
        assert!(std::ptr::eq(scope.statement(), actual));
        assert!(
            matches!(resolved(scope,&input).source(),S::LengthInput{declaration,..} if std::ptr::eq(declaration,&canonical))
        );
        for name in [&label, &last] {
            missing(scope, name);
        }
        if position == 0 {
            missing(scope, &first);
        } else {
            assert_recipe(
                resolved(scope, &first),
                1,
                "first",
                B::Length,
                source,
                "let first:length=unknown_rhs",
            );
        }
        if position < 3 {
            missing(scope, &second);
        } else {
            assert_recipe(
                resolved(scope, &second),
                3,
                "second",
                B::Angle,
                source,
                "let second:angle=90 deg",
            );
        }
        assert!(cursor.advance_metadata().unwrap());
    }
    assert!(cursor.current().unwrap().is_none());
    assert!(!cursor.advance_metadata().unwrap());
    assert!(!cursor.advance_metadata().unwrap());
    assert!(original_ns.resolve(&first).is_err());
    assert_eq!(original_ns.declarations().len(), 9);
}

#[test]
fn six_let_kinds_remain_actual_annotations_and_all_reserved_contexts_visible() {
    for (kind, annotation) in [
        (B::Length, "length"),
        (B::Angle, "angle"),
        (B::Area, "area"),
        (B::Ratio, "ratio"),
        (B::Count, "count"),
        (B::Boolean, "boolean"),
    ] {
        let whole = format!("let result:{annotation}=missing");
        let source = format!("{whole} assert observer:eps_geo=result==result");
        let recipe = normalized(&source);
        let mut cursor = Cursor::new(N::new([]).unwrap(), &recipe);
        let result = MachineToken::new("result").unwrap();
        missing(current(&cursor), &result);
        assert!(cursor.advance_metadata().unwrap());
        let scope = current(&cursor);
        assert_recipe(resolved(scope, &result), 1, "result", kind, &source, &whole);
        for reserved in [
            R::Tolerance(T::Numerical),
            R::Tolerance(T::Geometric),
            R::Tolerance(T::Format),
            R::Tolerance(T::Importer),
            R::Tolerance(T::Physical),
            R::SizeIndex,
            R::SizeCount,
            R::IsBaseSize,
        ] {
            let query = MachineToken::new(reserved.token()).unwrap();
            assert!(
                matches!(resolved(scope,&query).source(),S::Reserved(actual) if actual == reserved)
            );
        }
        assert!(cursor.advance_metadata().unwrap());
        assert!(cursor.current().unwrap().is_none());
    }
}

#[test]
fn every_reserved_binding_keeps_fixed_metadata_and_actual_attempt_after_assertions() {
    for (name, reserved, context) in [
        ("eps_num", R::Tolerance(T::Numerical), C::Always),
        ("eps_geo", R::Tolerance(T::Geometric), C::Always),
        ("eps_fmt", R::Tolerance(T::Format), C::Export),
        ("eps_imp", R::Tolerance(T::Importer), C::Export),
        ("eps_phys", R::Tolerance(T::Physical), C::Profile),
        ("size_index", R::SizeIndex, C::Size),
        ("size_count", R::SizeCount, C::Size),
        ("is_base_size", R::IsBaseSize, C::Size),
    ] {
        for (kind, annotation) in [
            (B::Length, "length"),
            (B::Angle, "angle"),
            (B::Area, "area"),
            (B::Ratio, "ratio"),
            (B::Count, "count"),
            (B::Boolean, "boolean"),
        ] {
            let whole = format!("let {name}:{annotation}=missing");
            let source =
                format!("assert first:eps_num=1 cm==1 cm assert second:eps_geo=2 cm==2 cm {whole}");
            let recipe = normalized(&source);
            let mut cursor = Cursor::new(N::new([]).unwrap(), &recipe);
            assert!(cursor.advance_metadata().unwrap());
            assert!(cursor.advance_metadata().unwrap());
            let error = blocked(&cursor);
            assert_eq!(error.token(), "formula_rebinding");
            assert_eq!(error.name(), name);
            assert!(
                matches!(error,E::ReservedBinding{reserved:actual,..} if actual==reserved && actual.required_context()==context)
            );
            let [prior, attempt] = error.binding_sources();
            assert!(matches!(prior.source(),S::Reserved(actual) if actual==reserved));
            assert_recipe(attempt, 3, name, kind, &source, &whole);
            for _ in 0..2 {
                let result = cursor.advance_metadata();
                assert!(result.is_err());
                let repeated = blocked(&cursor);
                assert_recipe(
                    repeated.binding_sources()[1],
                    3,
                    name,
                    kind,
                    &source,
                    &whole,
                );
            }
        }
    }
}

#[test]
fn repeated_let_keeps_both_real_ordinals_and_never_advances_or_shadows_after_refusal() {
    let source = " \nlet seam:length=1 cm\nassert gap:eps_geo=seam==seam\nlet seam:angle=90 deg\nlet later:ratio=1\t";
    let recipe = normalized(source);
    let mut cursor = Cursor::new(N::new([]).unwrap(), &recipe);
    assert!(cursor.advance_metadata().unwrap());
    assert!(cursor.advance_metadata().unwrap());
    for _ in 0..3 {
        let error = blocked(&cursor);
        assert_eq!(error.token(), "formula_rebinding");
        assert_eq!(error.name(), "seam");
        assert!(matches!(error, E::RecipeRebinding { .. }));
        let [prior, attempt] = error.binding_sources();
        assert_recipe(prior, 1, "seam", B::Length, source, "let seam:length=1 cm");
        assert_recipe(
            attempt,
            3,
            "seam",
            B::Angle,
            source,
            "let seam:angle=90 deg",
        );
        let result = cursor.advance_metadata();
        assert!(result.is_err());
        assert!(!format!("{error:?}").contains("seam"));
        assert_eq!(error.to_string(), "formula_rebinding");
        let cloned = error.clone();
        assert_recipe(
            cloned.binding_sources()[0],
            1,
            "seam",
            B::Length,
            source,
            "let seam:length=1 cm",
        );
    }
}

#[test]
fn input_and_geometry_collisions_keep_actual_input_before_recipe_attempt() {
    let name = MachineToken::new("occupied").unwrap();
    let point = PointRef::new(id(91), LocalTag::new(7));
    let canonical = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(81),
        source: id(82),
        state: LengthState::Unknown {
            observation: id(83),
        },
    })
    .unwrap();
    let declarations = [
        D::length_input(&name, L::Measurement, id(84), &canonical),
        D::length_input(&name, L::Ease, id(85), &canonical),
        D::input(&name, P::Parameter, id(61), id(62), B::Length),
        D::input(&name, P::Profile, id(71), id(72), B::Length),
        D::input(&name, P::Material, id(73), id(74), B::Ratio),
        D::point(&name, point),
        D::edge(&name, EdgeRef::new(id(92), LocalTag::new(9))),
    ];
    for original in declarations {
        let source = "assert gap:eps_geo=1 cm==1 cm let occupied:angle=missing";
        let recipe = normalized(source);
        let mut cursor = Cursor::new(N::new([initial(original)]).unwrap(), &recipe);
        assert!(cursor.advance_metadata().unwrap());
        let error = blocked(&cursor);
        assert_eq!(error.token(), "formula_ambiguous_name");
        assert_eq!(error.name(), "occupied");
        assert!(matches!(error, E::AmbiguousName { .. }));
        let [prior, attempt] = error.binding_sources();
        assert_eq!(prior.origin(), original.origin());
        assert_eq!(prior.kind(), original.kind());
        match original.source() {
            S::LengthInput {
                declaration: expected,
                input: expected_input,
                ..
            } => assert!(
                matches!(prior.source(),S::LengthInput{declaration,input,..} if std::ptr::eq(declaration,expected)&&input==expected_input)
            ),
            S::Input {
                input: expected,
                declaration: expected_declaration,
                ..
            } => assert!(
                matches!(prior.source(),S::Input{input,declaration,..} if input==expected&&declaration==expected_declaration)
            ),
            S::Point(expected) => {
                assert!(matches!(prior.source(),S::Point(actual) if actual==expected))
            }
            S::Edge(expected) => {
                assert!(matches!(prior.source(), S::Edge(actual) if actual == expected))
            }
            _ => unreachable!(),
        }
        assert_recipe(
            attempt,
            2,
            "occupied",
            B::Angle,
            source,
            "let occupied:angle=missing",
        );
        let result = cursor.advance_metadata();
        assert!(result.is_err());
    }
}

#[test]
fn self_forward_and_labels_never_bind_or_reorder_and_header_error_wins() {
    let source="let first:length=second assert first:eps_fmt=first==first let second:length=second assert eps_num:eps_num=first==second";
    let recipe = normalized(source);
    let mut cursor = Cursor::new(N::new([]).unwrap(), &recipe);
    let first = MachineToken::new("first").unwrap();
    let second = MachineToken::new("second").unwrap();
    missing(current(&cursor), &first);
    missing(current(&cursor), &second);
    assert!(cursor.advance_metadata().unwrap());
    assert_eq!(resolved(current(&cursor), &first).origin(), O::Recipe);
    assert!(cursor.advance_metadata().unwrap());
    missing(current(&cursor), &second);
    assert!(cursor.advance_metadata().unwrap());
    assert_eq!(resolved(current(&cursor), &second).origin(), O::Recipe);
    assert!(cursor.advance_metadata().unwrap());
    assert!(cursor.current().unwrap().is_none());
    let invalid = normalized("let eps_fmt:length=undeclared");
    let cursor = Cursor::new(N::new([]).unwrap(), &invalid);
    assert_eq!(blocked(&cursor).token(), "formula_rebinding");
}

#[test]
fn empty_end_and_maximum_recipe_are_fused_on_small_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let empty = normalized("  \n");
            let mut cursor = Cursor::new(N::new([]).unwrap(), &empty);
            assert!(cursor.current().unwrap().is_none());
            assert!(!cursor.advance_metadata().unwrap());
            let source = (0..4096)
                .map(|i| format!("assert check_{i}:eps_num=1 cm==1 cm"))
                .collect::<Vec<_>>()
                .join("\n");
            let recipe = normalized(&source);
            let mut cursor = Cursor::new(N::new([]).unwrap(), &recipe);
            for i in 1..=4096 {
                let scope = current(&cursor);
                assert_eq!(scope.statement_index(), i);
                assert_eq!(scope.statement().name(), format!("check_{}", i - 1));
                assert!(cursor.advance_metadata().unwrap());
            }
            for _ in 0..3 {
                assert!(cursor.current().unwrap().is_none());
                assert!(!cursor.advance_metadata().unwrap());
            }
            let source = (0..4096)
                .map(|i| format!("let item_{i}:ratio=missing"))
                .collect::<Vec<_>>()
                .join("\n");
            let recipe = normalized(&source);
            let mut cursor = Cursor::new(N::new([]).unwrap(), &recipe);
            for i in 1..=4096 {
                assert_eq!(current(&cursor).statement_index(), i);
                assert!(cursor.advance_metadata().unwrap());
            }
            assert!(cursor.current().unwrap().is_none());
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn cursor_and_scope_default_formatting_omit_recipe_payload() {
    let recipe = normalized("let private_customer_binding:length=1 cm");
    let cursor = Cursor::new(N::new([]).unwrap(), &recipe);
    let scope = current(&cursor);
    for debug in [
        format!("{cursor:?}"),
        format!("{cursor:#?}"),
        format!("{scope:?}"),
        format!("{scope:#?}"),
    ] {
        assert!(!debug.contains("private_customer_binding"));
        assert!(!debug.contains("1 cm"));
    }
}
