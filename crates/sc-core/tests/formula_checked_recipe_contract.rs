//! Independently authored whole-recipe boundaries; all assertions use only the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as Binding, FormulaCheckedRecipe,
        FormulaCheckedStatementKind as Checked, FormulaDeclaration as Declaration,
        FormulaDeclarationSource as Source, FormulaExpressionCheckRefusal as ExpressionRefusal,
        FormulaInitialDeclaration as Initial, FormulaInputOrigin, FormulaKind as Kind,
        FormulaNamespace, FormulaNormalizedRecipe, FormulaRecipe,
        FormulaRecipeCheckRefusal as Refusal, FormulaRecipeDependencyRole as Role,
        FormulaReservedName, FormulaScalarInputOrigin,
        FormulaStatementCheckRefusal as StatementRefusal,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
const INPUTS: [(Kind, &str); 8] = [
    (Kind::Length, "width"),
    (Kind::Angle, "angle_input"),
    (Kind::Area, "area_input"),
    (Kind::Ratio, "ratio_input"),
    (Kind::Count, "count_input"),
    (Kind::Boolean, "flag"),
    (Kind::Point, "point_input"),
    (Kind::Edge, "edge_input"),
];
const ANNOTATIONS: [(Binding, Kind, &str); 6] = [
    (Binding::Length, Kind::Length, "length"),
    (Binding::Angle, Kind::Angle, "angle"),
    (Binding::Area, Kind::Area, "area"),
    (Binding::Ratio, Kind::Ratio, "ratio"),
    (Binding::Count, Kind::Count, "count"),
    (Binding::Boolean, Kind::Boolean, "boolean"),
];
fn normalized(source: &str) -> FormulaNormalizedRecipe<'_> {
    FormulaRecipe::parse(source)
        .unwrap()
        .normalize_literals()
        .unwrap()
}
fn names() -> [MachineToken; 8] {
    INPUTS.map(|(_, name)| MachineToken::new(name).unwrap())
}
fn namespace(names: &[MachineToken; 8]) -> FormulaNamespace<'_> {
    FormulaNamespace::new(names.iter().zip(INPUTS).map(|(name, (kind, _))| {
        let declaration = match kind {
            Kind::Point => Declaration::point(
                name,
                PointRef::new(EntityId::from_bits(11), LocalTag::new(1)),
            ),
            Kind::Edge => Declaration::edge(
                name,
                EdgeRef::new(EntityId::from_bits(12), LocalTag::new(2)),
            ),
            _ => Declaration::input(
                name,
                FormulaScalarInputOrigin::Parameter,
                EntityId::from_bits(13),
                EntityId::from_bits(14),
                kind.binding_kind().unwrap(),
            ),
        };
        Initial::try_from(declaration).unwrap()
    }))
    .unwrap()
}
fn exact_owner(proof: &FormulaCheckedRecipe<'_>, recipe: &FormulaNormalizedRecipe<'_>) {
    assert!(std::ptr::eq(proof.recipe(), recipe));
    assert_eq!(proof.canonical_recipe(), recipe.canonical_form());
    assert_eq!(proof.statements().len(), recipe.statements().len());
    for (index, (checked, original)) in proof
        .statements()
        .iter()
        .zip(recipe.statements())
        .enumerate()
    {
        assert!(std::ptr::eq(checked.statement(), original));
        assert_eq!(checked.statement_index(), index + 1);
        assert_eq!(checked.canonical_statement(), original.canonical_form());
    }
}
fn assert_role(condition: bool) {
    assert!(condition);
}
fn arithmetic(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::Length | Kind::Angle | Kind::Area | Kind::Ratio | Kind::Count
    )
}

#[test]
fn every_binding_pair_in_complete_order_accepts_no_partial_prefix() {
    let names = names();
    let namespace = namespace(&names);
    let mut cases = 0;
    for prefix in ["", "assert prior:eps_num=1==1\nlet earlier:count=1\n"] {
        for (annotation, wanted, token) in ANNOTATIONS {
            for (actual, name) in INPUTS {
                let source =
                    format!(" \t{prefix}let saved:{token}={name}\nassert after:eps_num=1==1\n");
                let recipe = normalized(&source);
                let result = recipe.check_kinds(namespace.clone());
                let ordinal = if prefix.is_empty() { 1 } else { 3 };
                if actual == wanted {
                    assert!(result.is_ok());
                    let proof = result.unwrap();
                    exact_owner(&proof, &recipe);
                    let Checked::Let { expression } =
                        proof.statements().get(ordinal - 1).unwrap().kind()
                    else {
                        assert_role(false);
                        return;
                    };
                    assert_eq!(expression.kind(), wanted);
                    let dependencies: Vec<_> = proof
                        .dependencies()
                        .filter(|d| d.statement_index() == ordinal)
                        .collect();
                    assert_eq!(dependencies.len(), 1);
                    assert_eq!(dependencies.first().unwrap().role(), Role::Binding);
                    assert_eq!(dependencies.first().unwrap().declaration().name(), name);
                } else {
                    assert!(result.is_err());
                    let error = result.unwrap_err();
                    assert!(std::ptr::eq(error.recipe(), &recipe));
                    assert!(std::ptr::eq(
                        error.statement(),
                        recipe.statements().get(ordinal - 1).unwrap()
                    ));
                    assert_eq!(error.statement_index(), ordinal);
                    assert_eq!(error.token(), "formula_dimension");
                    assert_eq!(error.span(), error.statement().annotation_span());
                    assert_eq!(error.canonical_recipe(), recipe.canonical_form());
                    assert_eq!(
                        error.canonical_statement(),
                        error.statement().canonical_form()
                    );
                    let Refusal::Statement(nested) = error.refusal() else {
                        assert_role(false);
                        return;
                    };
                    assert!(std::ptr::eq(nested.statement(), error.statement()));
                    assert_eq!(nested.statement_index(), ordinal);
                    let StatementRefusal::BindingDimension(args) = nested.refusal() else {
                        assert_role(false);
                        return;
                    };
                    assert_eq!(args.declared_kind(), annotation);
                    assert_eq!(args.expression_kind(), actual);
                    assert_eq!(args.wanted_kind(), wanted);
                }
                // The caller's reusable initial namespace never acquires earlier/saved let metadata.
                assert_eq!(namespace.declarations().len(), 16);
                assert!(namespace
                    .resolve(&MachineToken::new("earlier").unwrap())
                    .is_err());
                cases += 1;
            }
        }
    }
    println!("whole binding matrix:{cases} independently expected complete/late cases");
}

#[test]
fn every_assertion_pair_and_class_retains_header_and_operand_dependency_roles() {
    let names = names();
    let namespace = namespace(&names);
    let mut cases = 0;
    for prefix in ["", "let first:length=width\n"] {
        for class in ["eps_num", "eps_geo", "eps_fmt", "eps_imp", "eps_phys"] {
            for (left_kind, left_name) in INPUTS {
                for (right_kind, right_name) in INPUTS {
                    let source = format!("{prefix}assert closure:{class}={left_name}=={right_name}\nlet last:count=1");
                    let recipe = normalized(&source);
                    let ordinal = if prefix.is_empty() { 1 } else { 2 };
                    let result = recipe.check_kinds(namespace.clone());
                    if left_kind == right_kind && arithmetic(left_kind) {
                        assert!(result.is_ok());
                        let proof = result.unwrap();
                        exact_owner(&proof, &recipe);
                        let uses: Vec<_> = proof
                            .dependencies()
                            .filter(|d| d.statement_index() == ordinal)
                            .collect();
                        assert_eq!(uses.len(), 3);
                        assert_eq!(
                            uses.iter().map(|d| d.role()).collect::<Vec<_>>(),
                            [
                                Role::AssertionTolerance,
                                Role::AssertionLeft,
                                Role::AssertionRight
                            ]
                        );
                        assert_eq!(
                            uses.iter()
                                .map(|d| d.declaration().name())
                                .collect::<Vec<_>>(),
                            [class, left_name, right_name]
                        );
                        assert_eq!(
                            uses.first().unwrap().span(),
                            recipe
                                .statements()
                                .get(ordinal - 1)
                                .unwrap()
                                .annotation_span()
                        );
                        let Source::Reserved(FormulaReservedName::Tolerance(tolerance)) =
                            uses.first().unwrap().declaration().source()
                        else {
                            assert_role(false);
                            return;
                        };
                        assert_eq!(tolerance.token(), class);
                        assert_eq!(uses.get(1).unwrap().declaration().kind(), left_kind);
                        assert_eq!(uses.get(2).unwrap().declaration().kind(), right_kind);
                    } else {
                        assert!(result.is_err());
                        let error = result.unwrap_err();
                        assert_eq!(error.statement_index(), ordinal);
                        assert_eq!(error.token(), "formula_dimension");
                        assert_eq!(
                            error.span(),
                            recipe.statements().get(ordinal - 1).unwrap().span()
                        );
                        let Refusal::Statement(nested) = error.refusal() else {
                            assert_role(false);
                            return;
                        };
                        let StatementRefusal::AssertionDimension(args) = nested.refusal() else {
                            assert_role(false);
                            return;
                        };
                        assert_eq!(
                            args.operands().iter().map(|o| o.kind()).collect::<Vec<_>>(),
                            [left_kind, right_kind]
                        );
                        assert_eq!(args.wanted_signatures().len(), 1);
                    }
                    cases += 1;
                }
            }
        }
    }
    println!("whole assertion matrix:{cases} independently expected class/kind/position cases");
}

#[test]
fn all_late_child_failures_preserve_original_scope_and_publish_no_proof() {
    for (tail, token, missing) in [
        ("let bad:length=bad", "formula_unbound_name", Some("bad")),
        (
            "let bad:length=future\nlet future:length=1 mm",
            "formula_unbound_name",
            Some("future"),
        ),
        (
            "let bad:length=if(is_base_size,first,missing)",
            "formula_unbound_name",
            Some("missing"),
        ),
        (
            "let bad:length=if(is_base_size,missing,first)",
            "formula_unbound_name",
            Some("missing"),
        ),
        (
            "let bad:length=bogus(missing)",
            "formula_unbound_name",
            None,
        ),
        ("let bad:length=nurbs(missing)", "env_nurbs", None),
        (
            "let bad:length=solve(missing)",
            "env_sketch_constraints",
            None,
        ),
        (
            "assert bad:eps_num=missing_left==missing_right",
            "formula_unbound_name",
            Some("missing_left"),
        ),
        (
            "assert bad:eps_num=1==missing_right",
            "formula_unbound_name",
            Some("missing_right"),
        ),
        (
            "assert bad:eps_num=1 mm+1.0==missing_right",
            "formula_dimension",
            None,
        ),
    ] {
        let source =
            format!(" \nlet first:length=1 mm\nassert prior:eps_geo=first==first\n{tail}\n");
        let recipe = normalized(&source);
        let result = recipe.check_kinds(FormulaNamespace::new([]).unwrap());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(std::ptr::eq(error.recipe(), &recipe));
        assert!(std::ptr::eq(
            error.statement(),
            recipe.statements().get(2).unwrap()
        ));
        assert_eq!(error.statement_index(), 3);
        assert_eq!(error.token(), token);
        assert_eq!(error.canonical_recipe(), recipe.canonical_form());
        let Refusal::Statement(nested) = error.refusal() else {
            assert_role(false);
            return;
        };
        assert_eq!(error.span(), nested.span());
        let StatementRefusal::Expression { error: child, .. } = nested.refusal() else {
            assert_role(false);
            return;
        };
        if let Some(name) = missing {
            let ExpressionRefusal::UnboundName(query) = child.refusal() else {
                assert_role(false);
                return;
            };
            assert_eq!(query.name(), name);
        }
        assert!(!format!("{error:?}").contains(tail));
        assert_eq!(error.to_string(), token);
    }
    // Fully checked input refuses at the earliest static statement, not a later header collision.
    let recipe = normalized("let first:length=missing\nlet first:length=1 mm");
    let result = recipe.check_kinds(FormulaNamespace::new([]).unwrap());
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.statement_index(), 1);
    assert_eq!(error.token(), "formula_unbound_name");
}

#[test]
fn reserved_initial_and_prior_recipe_collisions_preserve_both_actual_sources() {
    let names = names();
    let namespace = namespace(&names);
    for reserved in FormulaReservedName::ALL {
        for (_, _, annotation) in ANNOTATIONS {
            let source = format!(
                "let first:count=1\nassert prior:eps_num=1==1\nlet {}:{annotation}=missing",
                reserved.token()
            );
            let recipe = normalized(&source);
            let result = recipe.check_kinds(namespace.clone());
            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!(error.token(), "formula_rebinding");
            assert_eq!(error.statement_index(), 3);
            assert_eq!(
                error.span(),
                recipe.statements().get(2).unwrap().name_span()
            );
            let Refusal::Namespace(collision) = error.refusal() else {
                assert_role(false);
                return;
            };
            let [prior, attempted] = collision.binding_sources();
            assert!(matches!(prior.source(), Source::Reserved(actual) if actual == reserved));
            assert!(
                matches!(attempted.source(), Source::Recipe { statement_index: 3, span, name_span, .. }
                if span == recipe.statements().get(2).unwrap().span() && name_span == error.span())
            );
            assert_eq!(attempted.name(), reserved.token());
        }
    }
    for (_, name) in INPUTS {
        let source = format!("let first:count=1\nlet {name}:length=missing");
        let recipe = normalized(&source);
        let result = recipe.check_kinds(namespace.clone());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.token(), "formula_ambiguous_name");
        assert_eq!(error.statement_index(), 2);
        let Refusal::Namespace(collision) = error.refusal() else {
            assert_role(false);
            return;
        };
        let [prior, attempted] = collision.binding_sources();
        assert_eq!(prior.name(), name);
        assert_eq!(attempted.name(), name);
        assert_eq!(
            prior.kind(),
            INPUTS.iter().find(|(_, input)| *input == name).unwrap().0
        );
        assert!(matches!(
            attempted.source(),
            Source::Recipe {
                statement_index: 2,
                ..
            }
        ));
    }
    let recipe = normalized(
        "let first:length=1 mm\nassert prior:eps_geo=first==first\nlet first:length=missing",
    );
    let result = recipe.check_kinds(FormulaNamespace::new([]).unwrap());
    assert!(result.is_err());
    let error = result.unwrap_err();
    let Refusal::Namespace(collision) = error.refusal() else {
        assert_role(false);
        return;
    };
    assert_eq!(error.token(), "formula_rebinding");
    assert_eq!(error.statement_index(), 3);
    let [prior, attempted] = collision.binding_sources();
    assert!(matches!(
        prior.source(),
        Source::Recipe {
            statement_index: 1,
            ..
        }
    ));
    assert!(matches!(
        attempted.source(),
        Source::Recipe {
            statement_index: 3,
            ..
        }
    ));
}

#[test]
fn complete_dependencies_preserve_unknown_records_repetitions_and_strictly_prior_suppliers() {
    let width = MachineToken::new("width").unwrap();
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(21),
        source: EntityId::from_bits(22),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(23),
        },
    })
    .unwrap();
    let before = record.clone();
    let namespace = FormulaNamespace::new([Initial::try_from(Declaration::length_input(
        &width,
        FormulaInputOrigin::Measurement,
        EntityId::from_bits(24),
        &record,
    ))
    .unwrap()])
    .unwrap();
    let source = " \tlet first:length=width/0\nassert test:eps_fmt=first==width\nlet second:length=if(is_base_size,first+first,width)\nlet third:length=second+first";
    let recipe = normalized(source);
    let proof = recipe.check_kinds(namespace.clone()).unwrap();
    exact_owner(&proof, &recipe);
    let dependencies: Vec<_> = proof.dependencies().collect();
    assert_eq!(
        dependencies
            .iter()
            .map(|d| (d.statement_index(), d.role(), d.declaration().name()))
            .collect::<Vec<_>>(),
        [
            (1, Role::Binding, "width"),
            (2, Role::AssertionTolerance, "eps_fmt"),
            (2, Role::AssertionLeft, "first"),
            (2, Role::AssertionRight, "width"),
            (3, Role::Binding, "is_base_size"),
            (3, Role::Binding, "first"),
            (3, Role::Binding, "first"),
            (3, Role::Binding, "width"),
            (4, Role::Binding, "second"),
            (4, Role::Binding, "first")
        ]
    );
    for dependency in &dependencies {
        let span = dependency.span();
        assert_eq!(
            &source[span.start()..span.end()],
            dependency.declaration().name()
        );
        match dependency.declaration().source() {
            Source::LengthInput {
                declaration,
                input,
                origin,
            } => {
                assert!(std::ptr::eq(declaration, &record));
                assert_eq!(input, EntityId::from_bits(24));
                assert_eq!(origin, FormulaInputOrigin::Measurement);
            }
            Source::Recipe {
                statement_index,
                span,
                name_span,
                ..
            } => {
                assert!(statement_index < dependency.statement_index());
                assert_eq!(
                    span,
                    recipe.statements().get(statement_index - 1).unwrap().span()
                );
                assert_eq!(
                    name_span,
                    recipe
                        .statements()
                        .get(statement_index - 1)
                        .unwrap()
                        .name_span()
                );
            }
            Source::Reserved(_) => (),
            _ => {
                assert_role(false);
                return;
            }
        }
    }
    assert_eq!(proof.dependencies().count(), dependencies.len()); // fresh traversal is deterministic
    assert_ne!(
        dependencies.get(5).unwrap().span(),
        dependencies.get(6).unwrap().span()
    );
    assert_eq!(record, before);
    assert!(namespace
        .resolve(&MachineToken::new("first").unwrap())
        .is_err());
    assert!(!format!("{proof:?}").contains(source));
    assert!(!format!("{proof:?}").contains("first"));
    assert!(!format!("{:?}", dependencies.first().unwrap()).contains("width"));
    // Canonical recipe bytes describe authored recipe inputs; resolved supplier identity stays separate.
    let other = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(31),
        source: EntityId::from_bits(32),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(33),
        },
    })
    .unwrap();
    let other_namespace = FormulaNamespace::new([Initial::try_from(Declaration::length_input(
        &width,
        FormulaInputOrigin::Measurement,
        EntityId::from_bits(34),
        &other,
    ))
    .unwrap()])
    .unwrap();
    let other_proof = recipe.check_kinds(other_namespace).unwrap();
    assert_eq!(proof.canonical_recipe(), other_proof.canonical_recipe());
    assert!(
        matches!(other_proof.dependencies().next().unwrap().declaration().source(),
        Source::LengthInput { declaration, .. } if std::ptr::eq(declaration, &other))
    );
}

#[test]
fn whole_static_acceptance_never_evaluates_false_assertions_or_numeric_domains() {
    for (source, dependencies) in [
        ("", 0),
        (" \t\n", 0),
        ("assert false_check:eps_num=1==2", 1),
        ("let division:length=1 mm/0", 0),
        ("let domain:length=sqrt(-(1 mm)^2)", 0),
        ("let wide:count=340282366920938463463374607431768211455", 0),
        ("let unavailable:length=eps_fmt", 1),
        ("let context:boolean=is_base_size", 1),
    ] {
        let recipe = normalized(source);
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).unwrap())
            .unwrap();
        exact_owner(&proof, &recipe);
        assert_eq!(proof.dependencies().count(), dependencies);
    }
}

#[test]
fn complete_limits_and_late_error_are_checked_on_a_small_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let maximum = format!("{}1 mm", "-".repeat(255)); // exactly256 semantic nodes
            let source = (0..4096)
                .map(|index| format!("assert c{index}:eps_geo={maximum}=={maximum}"))
                .collect::<Vec<_>>()
                .join("\n");
            let recipe = normalized(&source);
            let proof = recipe
                .check_kinds(FormulaNamespace::new([]).unwrap())
                .unwrap();
            exact_owner(&proof, &recipe);
            assert_eq!(proof.statements().len(), 4096);
            assert_eq!(proof.dependencies().count(), 4096); // one actual tolerance header per assertion
            drop(proof);
            drop(recipe);
            let source = (0..4095)
                .map(|index| format!("let n{index}:count=1"))
                .chain([String::from("let bad:length=missing")])
                .collect::<Vec<_>>()
                .join("\n");
            let recipe = normalized(&source);
            let result = recipe.check_kinds(FormulaNamespace::new([]).unwrap());
            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!(error.statement_index(), 4096);
            assert_eq!(error.token(), "formula_unbound_name");
            let mut deep = String::from("1 mm");
            for _ in 0..16 {
                deep = format!("if(is_base_size,1 mm,{deep})");
            }
            let source = format!("let deep:length={}{}", "-".repeat(207), deep); // 49+207=256 nodes
            let recipe = normalized(&source);
            let proof = recipe
                .check_kinds(FormulaNamespace::new([]).unwrap())
                .unwrap();
            assert_eq!(proof.dependencies().count(), 16);
            exact_owner(&proof, &recipe);
            let grouped = format!(
                "let grouped:length={}1 mm{}",
                "(".repeat(4000),
                ")".repeat(4000)
            );
            let recipe = normalized(&grouped);
            exact_owner(
                &recipe
                    .check_kinds(FormulaNamespace::new([]).unwrap())
                    .unwrap(),
                &recipe,
            );
        })
        .unwrap()
        .join()
        .unwrap();
}
