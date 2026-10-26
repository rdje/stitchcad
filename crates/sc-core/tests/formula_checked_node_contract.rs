//! Actual node certificates preserve source owners, static roles and resolved declaration identities.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBinaryOperator as B, FormulaBuiltin as F, FormulaBuiltinOperand as O,
        FormulaCheckedNode as Node, FormulaCheckedNodeForm as Form,
        FormulaCheckedStatementKind as Statement, FormulaDeclaration as D,
        FormulaDeclarationSource as Source, FormulaExpression, FormulaInitialDeclaration,
        FormulaInputOrigin, FormulaKind as K, FormulaNamespace, FormulaNormalizedExpression,
        FormulaRecipe, FormulaReservedName, FormulaScalarInputOrigin, FormulaToleranceName as T,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};

fn normalized(source: &str) -> FormulaNormalizedExpression<'_> {
    FormulaExpression::parse(source)
        .expect("syntax")
        .normalize_literals()
        .expect("literal inputs")
}
fn walk<'p, 'd>(root: Node<'p, 'd>) -> Vec<Node<'p, 'd>> {
    let mut pending = vec![root];
    let mut nodes = Vec::new();
    while let Some(node) = pending.pop() {
        nodes.push(node);
        match node.form() {
            Form::Literal(_) | Form::Name { .. } => {}
            Form::Negate(child) | Form::Square(child) => pending.push(child),
            Form::Binary { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Form::Call { arguments, .. } => pending.extend(arguments.iter().rev()),
            Form::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                pending.push(else_branch);
                pending.push(then_branch);
                pending.push(condition);
            }
        }
    }
    nodes
}

#[test]
fn every_actual_form_has_its_own_kind_span_owner_and_unknown_name_source() {
    let name = MachineToken::new("body_width").expect("name");
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(1),
        source: EntityId::from_bits(2),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(3),
        },
    })
    .expect("canonical unknown");
    let declaration = D::length_input(
        &name,
        FormulaInputOrigin::Measurement,
        EntityId::from_bits(4),
        &record,
    );
    let namespace =
        FormulaNamespace::new([FormulaInitialDeclaration::try_from(declaration).expect("initial")])
            .expect("namespace");
    let source =
        "if(is_base_size,clamp((body_width),-2 mm,hypot(3 mm,4 mm)),max(1 mm,(2 mm)^2/(1 mm)))";
    let expression = normalized(source);
    let proof = expression
        .check_kinds(&namespace)
        .expect("all static branches");
    assert_eq!(proof.root().kind(), proof.kind());
    assert_eq!(proof.root().span(), expression.root().span());
    assert_eq!(proof.node_count(), expression.node_count());
    let nodes = walk(proof.root());
    assert_eq!(nodes.len(), proof.node_count());
    let mut forms = [false; 7];
    for node in nodes {
        assert!(std::ptr::eq(node.expression(), &expression));
        assert_eq!(node.operand(), O::Value(node.kind()));
        let text = source
            .get(node.span().start()..node.span().end())
            .expect("original span");
        assert!(!format!("{node:?}").contains("body_width"));
        assert!(!format!("{:?}", node.form()).contains("body_width"));
        match node.form() {
            Form::Literal(_) => {
                forms[0] = true;
                assert_eq!(node.kind(), K::Length);
            }
            Form::Name {
                name: actual,
                declaration,
            } => {
                forms[1] = true;
                assert_eq!(actual, declaration.name());
                if actual == "body_width" {
                    assert_eq!(text, "(body_width)");
                    assert_eq!(node.kind(), K::Length);
                    assert!(
                        matches!(declaration.source(), Source::LengthInput { declaration: canonical, .. }
                        if std::ptr::eq(canonical, &record))
                    );
                } else {
                    assert_eq!(actual, "is_base_size");
                    assert_eq!(node.kind(), K::Boolean);
                }
            }
            Form::Negate(child) => {
                forms[2] = true;
                assert_eq!(node.kind(), K::Length);
                assert_eq!(child.kind(), K::Length);
            }
            Form::Square(child) => {
                forms[3] = true;
                assert_eq!(node.kind(), K::Area);
                assert_eq!(child.kind(), K::Length);
            }
            Form::Binary {
                operator,
                left,
                right,
            } => {
                forms[4] = true;
                assert_eq!(operator, B::Divide);
                assert_eq!(node.kind(), K::Length);
                assert_eq!(left.kind(), K::Area);
                assert_eq!(right.kind(), K::Length);
            }
            Form::Call { builtin, arguments } => {
                forms[5] = true;
                assert!(matches!(builtin, F::Clamp | F::Hypot | F::Max));
                assert_eq!(node.kind(), K::Length);
                assert!(!arguments.is_empty());
                assert_eq!(arguments.len(), if builtin == F::Clamp { 3 } else { 2 });
                assert!(arguments
                    .iter()
                    .all(|argument| argument.kind() == K::Length));
            }
            Form::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                forms[6] = true;
                assert_eq!(node.kind(), K::Length);
                assert_eq!(condition.kind(), K::Boolean);
                assert_eq!(then_branch.kind(), K::Length);
                assert_eq!(else_branch.kind(), K::Length);
            }
        }
    }
    assert!(forms.into_iter().all(|seen| seen));
    assert!(
        matches!(record.state(), LengthState::Unknown { observation } if *observation == EntityId::from_bits(3))
    );
    assert_eq!(
        proof
            .dependencies()
            .iter()
            .map(|dependency| dependency.declaration().name())
            .collect::<Vec<_>>(),
        ["is_base_size", "body_width"]
    );
    assert_eq!(proof.canonical_expression(), expression.canonical_form());
}

const KINDS: [K; 8] = [
    K::Length,
    K::Angle,
    K::Area,
    K::Ratio,
    K::Count,
    K::Boolean,
    K::Point,
    K::Edge,
];
const NAMES: [&str; 8] = [
    "length_value",
    "angle_value",
    "area_value",
    "ratio_value",
    "count_value",
    "boolean_value",
    "point_value",
    "edge_value",
];
fn namespace(names: &[MachineToken; 8]) -> FormulaNamespace<'_> {
    FormulaNamespace::new(
        names
            .iter()
            .zip(KINDS)
            .map(|(name, kind)| match kind {
                K::Point => D::point(
                    name,
                    PointRef::new(EntityId::from_bits(21), LocalTag::FIRST),
                ),
                K::Edge => D::edge(
                    name,
                    EdgeRef::new(EntityId::from_bits(22), LocalTag::new(9)),
                ),
                _ => D::input(
                    name,
                    FormulaScalarInputOrigin::Parameter,
                    EntityId::from_bits(31),
                    EntityId::from_bits(32),
                    kind.binding_kind().expect("bindable scalar"),
                ),
            })
            .map(|declaration| FormulaInitialDeclaration::try_from(declaration).expect("initial")),
    )
    .expect("namespace")
}

#[test]
fn all_eight_name_kinds_keep_actual_sources_and_grouping_without_a_value_read() {
    let names = NAMES.map(|name| MachineToken::new(name).expect("name"));
    let namespace = namespace(&names);
    for (kind, name) in KINDS.into_iter().zip(NAMES) {
        let source = format!("({name})");
        let expression = normalized(&source);
        let proof = expression
            .check_kinds(&namespace)
            .expect("metadata-only name");
        assert_eq!(proof.node_count(), 1);
        let root = proof.root();
        assert_eq!(root.kind(), kind);
        assert_eq!(root.operand(), O::Value(kind));
        assert_eq!(root.span(), expression.root().span());
        assert_eq!(
            source.get(root.span().start()..root.span().end()),
            Some(source.as_str())
        );
        let Form::Name {
            name: actual,
            declaration,
        } = root.form()
        else {
            unreachable!("name fixture");
        };
        assert_eq!(actual, name);
        assert_eq!(declaration.name(), name);
        assert_eq!(declaration.kind(), kind);
        let dependency = proof.dependencies().first().expect("actual name use");
        assert_eq!(dependency.span(), root.span());
        assert_eq!(dependency.declaration().name(), declaration.name());
        match (kind, declaration.source()) {
            (K::Point, Source::Point(point)) => assert_eq!(
                point,
                PointRef::new(EntityId::from_bits(21), LocalTag::FIRST)
            ),
            (K::Edge, Source::Edge(edge)) => assert_eq!(
                edge,
                EdgeRef::new(EntityId::from_bits(22), LocalTag::new(9))
            ),
            (
                _,
                Source::Input {
                    input, declaration, ..
                },
            ) => {
                assert_eq!(input, EntityId::from_bits(31));
                assert_eq!(declaration, EntityId::from_bits(32));
            }
            _ => unreachable!("actual fixture source"),
        }
    }
}

#[test]
fn grouped_tolerance_roles_are_certified_only_on_actual_direct_name_nodes() {
    let namespace = FormulaNamespace::new([]).expect("reserved namespace");
    let expression = normalized("within(1 mm,1 mm,(eps_geo))");
    let proof = expression
        .check_kinds(&namespace)
        .expect("direct symbolic class");
    assert_eq!(proof.root().operand(), O::Value(K::Boolean));
    let Form::Call { builtin, arguments } = proof.root().form() else {
        unreachable!("within fixture");
    };
    assert_eq!(builtin, F::Within);
    assert_eq!(arguments.len(), 3);
    let first = arguments.iter().next().expect("first");
    let last = arguments.iter().next_back().expect("last");
    assert_eq!(first.operand(), O::Value(K::Length));
    assert_eq!(last.operand(), O::Tolerance(T::Geometric));
    assert!(
        matches!(last.form(), Form::Name { name: "eps_geo", declaration }
        if matches!(declaration.source(), Source::Reserved(FormulaReservedName::Tolerance(T::Geometric))))
    );
    let computed = normalized("eps_geo+0 um");
    let computed = computed
        .check_kinds(&namespace)
        .expect("ordinary Length sum");
    assert_eq!(computed.root().operand(), O::Value(K::Length));
    let Form::Binary { left, right, .. } = computed.root().form() else {
        unreachable!("sum fixture");
    };
    assert_eq!(left.operand(), O::Tolerance(T::Geometric));
    assert_eq!(right.operand(), O::Value(K::Length));
}

#[test]
fn resolved_call_certificates_cover_every_closed_callee_without_reordering_arguments() {
    let names = NAMES.map(|name| MachineToken::new(name).expect("name"));
    let namespace = namespace(&names);
    for (source, expected, count) in [
        ("sqrt(area_value)", F::Sqrt, 1),
        ("hypot(length_value,length_value)", F::Hypot, 2),
        ("abs(length_value)", F::Abs, 1),
        ("min(length_value,length_value)", F::Min, 2),
        ("max(length_value,length_value)", F::Max, 2),
        ("clamp(length_value,length_value,length_value)", F::Clamp, 3),
        ("round_to(length_value,length_value)", F::RoundTo, 2),
        ("sin(angle_value)", F::Sin, 1),
        ("cos(angle_value)", F::Cos, 1),
        ("tan(angle_value)", F::Tan, 1),
        ("atan(ratio_value)", F::Atan, 1),
        ("atan2(length_value,length_value)", F::Atan2, 2),
        ("arc_length(angle_value,length_value)", F::ArcLength, 2),
        ("within(length_value,length_value,eps_geo)", F::Within, 3),
        ("x(point_value)", F::X, 1),
        ("y(point_value)", F::Y, 1),
        ("dist(point_value,point_value)", F::Dist, 2),
        ("dir(point_value,point_value)", F::Dir, 2),
        ("len(edge_value)", F::Len, 1),
        ("param_at(edge_value,length_value)", F::ParamAt, 2),
        ("point_at(edge_value,ratio_value)", F::PointAt, 2),
    ] {
        let expression = normalized(source);
        let proof = expression
            .check_kinds(&namespace)
            .expect("actual call signature");
        let Form::Call { builtin, arguments } = proof.root().form() else {
            unreachable!("call fixture");
        };
        assert_eq!(builtin, expected);
        assert_eq!(arguments.len(), count);
        let spans = arguments
            .iter()
            .map(|argument| argument.span())
            .collect::<Vec<_>>();
        assert!(spans
            .windows(2)
            .all(|pair| pair.first().expect("left").start() < pair.last().expect("right").start()));
        assert_eq!(
            arguments
                .iter()
                .rev()
                .map(|argument| argument.span())
                .collect::<Vec<_>>(),
            spans.into_iter().rev().collect::<Vec<_>>()
        );
    }
}

#[test]
fn whole_statement_views_retain_actual_prior_let_sources_and_global_locations() {
    let source =
        "let first:length=1 mm\nassert closure:eps_geo=first==first\nlet second:length=first";
    let recipe = FormulaRecipe::parse(source)
        .expect("recipe")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    for statement in proof.statements() {
        let expressions = match statement.kind() {
            Statement::Let { expression } => vec![expression],
            Statement::Assert { left, right } => vec![left, right],
        };
        for expression in expressions {
            assert_eq!(
                expression.node_count(),
                expression.expression().node_count()
            );
            if let Form::Name { name, declaration } = expression.root().form() {
                assert_eq!(name, "first");
                assert_eq!(declaration.name(), "first");
                assert!(matches!(
                    declaration.source(),
                    Source::Recipe {
                        statement_index: 1,
                        ..
                    }
                ));
                let span = expression.root().span();
                assert_eq!(source.get(span.start()..span.end()), Some("first"));
                assert!(span.start() > 20);
            }
        }
    }
}

#[test]
fn the_full_expression_node_bound_is_certified_and_walked_without_recursive_views() {
    let source = format!("-({})", vec!["1 um"; 128].join("+"));
    let expression = normalized(&source);
    let proof = expression
        .check_kinds(&FormulaNamespace::new([]).expect("namespace"))
        .expect("bounded kind proof");
    assert_eq!(proof.node_count(), 256);
    assert_eq!(walk(proof.root()).len(), 256);
    assert_eq!(proof.root().kind(), K::Length);
    assert!(proof.dependencies().is_empty());
}
