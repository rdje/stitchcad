//! Independently authored header/comparison cases through real ordered scopes and source owners.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBinaryOperator as Binary, FormulaBindingKind as Binding,
        FormulaBuiltinOperand as Operand, FormulaCheckedOperation as Operation,
        FormulaCheckedStatementKind as Checked, FormulaDeclaration as Declaration,
        FormulaDeclarationSource as Source, FormulaExpressionCheckRefusal as ExpressionRefusal,
        FormulaInitialDeclaration as Initial, FormulaInputOrigin, FormulaKind as Kind,
        FormulaNameCursor as Cursor, FormulaNamespace, FormulaNormalizedRecipe,
        FormulaNormalizedStatementKind as Syntax, FormulaOperandRequirement, FormulaRecipe,
        FormulaResultRequirement, FormulaScalarInputOrigin,
        FormulaStatementCheckRefusal as Refusal, FormulaStatementExpression as Part,
        FormulaToleranceName,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
const INPUTS: [(Kind, &str); 8] = [
    (Kind::Length, "length_input"),
    (Kind::Angle, "angle_input"),
    (Kind::Area, "area_input"),
    (Kind::Ratio, "ratio_input"),
    (Kind::Count, "count_input"),
    (Kind::Boolean, "boolean_input"),
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
fn every_binding_annotation_compares_genuine_kind_and_retains_exact_owner() {
    let names = names();
    let mut cases = 0;
    for (annotation, wanted, token) in ANNOTATIONS {
        for (actual, name) in INPUTS {
            let source = format!(" \nlet saved: {token} = {name}\t");
            let recipe = normalized(&source);
            let cursor = Cursor::new(namespace(&names), &recipe);
            let scope = cursor.current().unwrap().unwrap();
            let result = scope.check_kinds();
            assert_eq!(cursor.current().unwrap().unwrap().statement_index(), 1);
            if actual == wanted {
                assert!(result.is_ok());
                let proof = result.unwrap();
                drop(cursor);
                assert!(std::ptr::eq(
                    proof.statement(),
                    recipe.statements().first().unwrap()
                ));
                assert_eq!(proof.statement_index(), 1);
                assert_eq!(
                    proof.canonical_statement(),
                    proof.statement().canonical_form()
                );
                let Checked::Let { expression } = proof.kind() else {
                    {
                        assert_role(false);
                        return;
                    }
                };
                let Syntax::Let {
                    expression: original,
                    ..
                } = proof.statement().kind()
                else {
                    {
                        assert_role(false);
                        return;
                    }
                };
                assert!(std::ptr::eq(expression.expression(), original));
                assert_eq!(expression.kind(), wanted);
                assert_eq!(expression.dependencies().len(), 1);
                assert_eq!(
                    expression
                        .dependencies()
                        .first()
                        .unwrap()
                        .declaration()
                        .name(),
                    name
                );
            } else {
                assert!(result.is_err());
                let error = result.unwrap_err();
                drop(cursor);
                assert!(std::ptr::eq(
                    error.statement(),
                    recipe.statements().first().unwrap()
                ));
                assert_eq!(error.statement_index(), 1);
                assert_eq!(error.token(), "formula_dimension");
                assert_eq!(error.span(), error.statement().annotation_span());
                assert_eq!(
                    error.canonical_statement(),
                    error.statement().canonical_form()
                );
                let Refusal::BindingDimension(args) = error.refusal() else {
                    {
                        assert_role(false);
                        return;
                    }
                };
                assert_eq!(args.declared_kind(), annotation);
                assert_eq!(args.expression_kind(), actual);
                assert_eq!(args.wanted_kind(), wanted);
            }
            cases += 1;
        }
    }
    println!("current let contract: {cases} independent declared/expression pairs");
}

#[test]
fn all_assertion_kind_pairs_and_five_classes_check_both_operands_before_comparison() {
    let names = names();
    let mut cases = 0;
    for class in ["eps_num", "eps_geo", "eps_fmt", "eps_imp", "eps_phys"] {
        for (left_kind, left_name) in INPUTS {
            for (right_kind, right_name) in INPUTS {
                let source = format!("assert closure:{class}={left_name}=={right_name}");
                let recipe = normalized(&source);
                let cursor = Cursor::new(namespace(&names), &recipe);
                let result = cursor.current().unwrap().unwrap().check_kinds();
                assert_eq!(cursor.current().unwrap().unwrap().statement_index(), 1);
                if left_kind == right_kind && arithmetic(left_kind) {
                    assert!(result.is_ok());
                    let proof = result.unwrap();
                    drop(cursor);
                    assert_eq!(proof.statement_index(), 1);
                    let Checked::Assert { left, right } = proof.kind() else {
                        {
                            assert_role(false);
                            return;
                        }
                    };
                    let Syntax::Assert {
                        tolerance,
                        left: original_left,
                        right: original_right,
                    } = proof.statement().kind()
                    else {
                        {
                            assert_role(false);
                            return;
                        }
                    };
                    assert_eq!(tolerance.token(), class);
                    assert!(std::ptr::eq(left.expression(), original_left));
                    assert!(std::ptr::eq(right.expression(), original_right));
                    assert_eq!(left.kind(), left_kind);
                    assert_eq!(right.kind(), right_kind);
                    assert_eq!(
                        left.dependencies().first().unwrap().declaration().name(),
                        left_name
                    );
                    assert_eq!(
                        right.dependencies().first().unwrap().declaration().name(),
                        right_name
                    );
                } else {
                    assert!(result.is_err());
                    let error = result.unwrap_err();
                    drop(cursor);
                    assert_eq!(error.token(), "formula_dimension");
                    assert_eq!(error.span(), error.statement().span());
                    assert_eq!(
                        error.canonical_statement(),
                        error.statement().canonical_form()
                    );
                    let Refusal::AssertionDimension(args) = error.refusal() else {
                        {
                            assert_role(false);
                            return;
                        }
                    };
                    assert_eq!(args.operation(), Operation::Binary(Binary::Equal));
                    assert_eq!(
                        args.operands(),
                        &[Operand::Value(left_kind), Operand::Value(right_kind)]
                    );
                    assert_eq!(args.wanted_signatures().len(), 1);
                    let row = args.wanted_signatures().first().unwrap();
                    assert_eq!(
                        row.operand_requirements(),
                        &[FormulaOperandRequirement::Arithmetic; 2]
                    );
                    assert_eq!(
                        row.result_requirement(),
                        FormulaResultRequirement::Exact(Kind::Boolean)
                    );
                    assert!(!args.arc_length_hint());
                }
                cases += 1;
            }
        }
    }
    println!("current assertion contract: {cases} independent class/operand pairs");
}

#[test]
fn child_priority_parts_spans_and_canonical_owners_are_never_fabricated() {
    for (source, part, token, missing) in [
        (
            "let saved:boolean=missing",
            Part::Binding,
            "formula_unbound_name",
            Some("missing"),
        ),
        (
            "let saved:length=saved",
            Part::Binding,
            "formula_unbound_name",
            Some("saved"),
        ),
        (
            "let saved:length=later let later:length=1 mm",
            Part::Binding,
            "formula_unbound_name",
            Some("later"),
        ),
        (
            "let saved:length=if(is_base_size,missing,1 mm)",
            Part::Binding,
            "formula_unbound_name",
            Some("missing"),
        ),
        (
            "let saved:length=if(is_base_size,1 mm,missing)",
            Part::Binding,
            "formula_unbound_name",
            Some("missing"),
        ),
        (
            "let saved:boolean=nurbs(missing)",
            Part::Binding,
            "env_nurbs",
            None,
        ),
        (
            "let saved:boolean=bogus(missing)",
            Part::Binding,
            "formula_unbound_name",
            None,
        ),
        (
            "assert closure:eps_num=missing_left==missing_right",
            Part::AssertionLeft,
            "formula_unbound_name",
            Some("missing_left"),
        ),
        (
            "assert closure:eps_num=1==missing_right",
            Part::AssertionRight,
            "formula_unbound_name",
            Some("missing_right"),
        ),
        (
            "assert closure:eps_num=1 mm+1.0==missing_right",
            Part::AssertionLeft,
            "formula_dimension",
            None,
        ),
    ] {
        let recipe = normalized(source);
        let cursor = Cursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
        let result = cursor.current().unwrap().unwrap().check_kinds();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.statement_index(), 1);
        assert_eq!(error.token(), token);
        assert_eq!(cursor.current().unwrap().unwrap().statement_index(), 1);
        let Refusal::Expression {
            part: actual,
            error: nested,
        } = error.refusal()
        else {
            {
                assert_role(false);
                return;
            }
        };
        assert_eq!(*actual, part);
        assert_eq!(error.span(), nested.span());
        let original = match (error.statement().kind(), part) {
            (Syntax::Let { expression, .. }, Part::Binding) => expression,
            (Syntax::Assert { left, .. }, Part::AssertionLeft) => left,
            (Syntax::Assert { right, .. }, Part::AssertionRight) => right,
            _ => {
                assert_role(false);
                return;
            }
        };
        assert!(std::ptr::eq(nested.expression(), original));
        assert_eq!(nested.canonical_expression(), original.canonical_form());
        if let Some(name) = missing {
            let ExpressionRefusal::UnboundName(args) = nested.refusal() else {
                {
                    assert_role(false);
                    return;
                }
            };
            assert_eq!(args.name(), name);
            assert_eq!(
                source.get(nested.span().start()..nested.span().end()),
                Some(name)
            );
        }
    }
    let input_names = names();
    let recipe = normalized("assert closure:eps_num=point_input==missing_right");
    let cursor = Cursor::new(namespace(&input_names), &recipe);
    let error = cursor
        .current()
        .unwrap()
        .unwrap()
        .check_kinds()
        .unwrap_err();
    assert!(matches!(
        error.refusal(),
        Refusal::Expression {
            part: Part::AssertionRight,
            ..
        }
    ));
    assert_eq!(error.token(), "formula_unbound_name");
}

#[test]
fn immutable_proofs_survive_advance_with_real_prior_sources_and_all_branch_dependencies() {
    let source = "let first:length=1 mm assert closure:eps_fmt=if(is_base_size,first,first)==first let result:boolean=first==first";
    let recipe = normalized(source);
    let proofs = {
        let mut cursor = Cursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
        let mut proofs = Vec::new();
        for (position, actual) in recipe.statements().iter().enumerate() {
            let proof = cursor.current().unwrap().unwrap().check_kinds().unwrap();
            assert_eq!(proof.statement_index(), position + 1);
            assert!(std::ptr::eq(proof.statement(), actual));
            assert!(cursor.advance_metadata().unwrap());
            proofs.push(proof);
        }
        assert!(cursor.current().unwrap().is_none());
        proofs
    };
    let Checked::Assert { left, right } = proofs.get(1).unwrap().kind() else {
        {
            assert_role(false);
            return;
        }
    };
    assert_eq!(left.dependencies().len(), 3);
    assert_eq!(
        left.dependencies().first().unwrap().declaration().name(),
        "is_base_size"
    );
    for dependency in left
        .dependencies()
        .iter()
        .skip(1)
        .chain(right.dependencies())
    {
        assert_eq!(dependency.declaration().name(), "first");
        assert!(matches!(
            dependency.declaration().source(),
            Source::Recipe {
                statement_index: 1,
                ..
            }
        ));
        assert_eq!(
            source.get(dependency.span().start()..dependency.span().end()),
            Some("first")
        );
    }
    assert_eq!(
        proofs.first().unwrap().canonical_statement().as_str(),
        "(bind first length length:1000)"
    );
    // Earlier metadata is not proof: this boundary must not pretend it checked a prior RHS.
    let recipe = normalized("let first:length=missing let second:length=first");
    let mut cursor = Cursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
    assert!(cursor.current().unwrap().unwrap().check_kinds().is_err());
    assert!(cursor.advance_metadata().unwrap());
    let proof = cursor.current().unwrap().unwrap().check_kinds().unwrap();
    assert_eq!(proof.statement_index(), 2);
    let Checked::Let { expression } = proof.kind() else {
        {
            assert_role(false);
            return;
        }
    };
    assert!(matches!(
        expression
            .dependencies()
            .first()
            .unwrap()
            .declaration()
            .source(),
        Source::Recipe {
            statement_index: 1,
            ..
        }
    ));
}

#[test]
fn assertion_payload_preserves_only_direct_symbolic_class_roles() {
    for (left, role) in [
        (
            "eps_num",
            Operand::Tolerance(FormulaToleranceName::Numerical),
        ),
        (
            "((eps_num))",
            Operand::Tolerance(FormulaToleranceName::Numerical),
        ),
        ("eps_num+0 mm", Operand::Value(Kind::Length)),
        ("min(eps_num)", Operand::Value(Kind::Length)),
    ] {
        let source = format!("assert closure:eps_fmt={left}==1.0");
        let recipe = normalized(&source);
        let cursor = Cursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
        let error = cursor
            .current()
            .unwrap()
            .unwrap()
            .check_kinds()
            .unwrap_err();
        let Refusal::AssertionDimension(args) = error.refusal() else {
            {
                assert_role(false);
                return;
            }
        };
        assert_eq!(args.operands(), &[role, Operand::Value(Kind::Ratio)]);
    }
}

#[test]
fn canonical_unknown_records_runtime_domains_and_customer_privacy_stay_separate() {
    let name = MachineToken::new("customer_input").unwrap();
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(41),
        source: EntityId::from_bits(42),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(43),
        },
    })
    .unwrap();
    let source = "let customer_result:length=customer_input+123456789 um";
    let recipe = normalized(source);
    let proof = {
        let ns = FormulaNamespace::new([Initial::try_from(Declaration::length_input(
            &name,
            FormulaInputOrigin::Measurement,
            EntityId::from_bits(44),
            &record,
        ))
        .unwrap()])
        .unwrap();
        let cursor = Cursor::new(ns, &recipe);
        let result = cursor.current().unwrap().unwrap().check_kinds();
        assert!(result.is_ok());
        result.unwrap()
    };
    let Checked::Let { expression } = proof.kind() else {
        {
            assert_role(false);
            return;
        }
    };
    assert!(
        matches!(expression.dependencies().first().unwrap().declaration().source(),
        Source::LengthInput { declaration, .. } if std::ptr::eq(declaration, &record))
    );
    for source in [
        "let saved:length=1 mm/0.0",
        "let saved:length=sqrt(-(1 mm)^2)",
        "assert closure:eps_phys=1 mm/0.0==2 mm",
        "let saved:count=1-2",
    ] {
        let recipe = normalized(source);
        let cursor = Cursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
        assert!(cursor.current().unwrap().unwrap().check_kinds().is_ok());
    }
    let recipe = normalized("let customer_result:ratio=123456789 um");
    let cursor = Cursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
    let error = cursor
        .current()
        .unwrap()
        .unwrap()
        .check_kinds()
        .unwrap_err();
    for text in [
        format!("{proof:?}"),
        format!("{:?}", proof.kind()),
        format!("{error:?}"),
        format!("{:?}", error.refusal()),
        error.to_string(),
    ] {
        for customer in ["customer_result", "customer_input", "123456789"] {
            assert!(!text.contains(customer));
        }
    }
    assert_eq!(error.to_string(), "formula_dimension");
}

#[test]
fn bounded_current_statements_and_complete_prior_sources_stay_flat_on_a_small_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let wide = format!(
                "min({})",
                std::iter::repeat_n("1 mm", 255)
                    .collect::<Vec<_>>()
                    .join(",")
            );
            let grouped = format!("{}1 mm{}", "(".repeat(4000), ")".repeat(4000));
            let mut conditional = String::from("1 mm");
            for _ in 0..16 {
                conditional = format!("if(is_base_size,{conditional},1 mm)");
            }
            for expression in [wide, grouped, conditional] {
                let source = format!("assert closure:eps_phys={expression}=={expression}");
                let recipe = normalized(&source);
                let cursor = Cursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
                let result = cursor.current().unwrap().unwrap().check_kinds();
                assert!(result.is_ok());
                assert!(matches!(result.unwrap().kind(), Checked::Assert { .. }));
            }
            let mut source = String::from("let value_0:count=1\n");
            for index in 1..4096 {
                source.push_str(&format!("let value_{index}:count=value_{}\n", index - 1));
            }
            let recipe = normalized(&source);
            let mut cursor = Cursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
            let mut proofs = Vec::new();
            for index in 1..=4096 {
                let result = cursor.current().unwrap().unwrap().check_kinds();
                assert!(result.is_ok());
                let proof = result.unwrap();
                assert_eq!(proof.statement_index(), index);
                let Checked::Let { expression } = proof.kind() else {
                    {
                        assert_role(false);
                        return;
                    }
                };
                assert_eq!(expression.kind(), Kind::Count);
                if index > 1 {
                    assert_eq!(expression.dependencies().len(), 1);
                    assert!(
                        matches!(expression.dependencies().first().unwrap().declaration().source(),
                    Source::Recipe { statement_index, .. } if statement_index == index - 1)
                    );
                }
                assert!(cursor.advance_metadata().unwrap());
                proofs.push(proof);
            }
            drop(cursor);
            assert_eq!(proofs.len(), 4096);
            assert_eq!(proofs.last().unwrap().statement_index(), 4096);
        })
        .unwrap()
        .join()
        .unwrap();
}
