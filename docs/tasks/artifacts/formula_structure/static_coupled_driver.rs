//! Read-only public API adapter for the independent coupled static oracle.
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::*,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use std::io::{self, BufRead};

fn decode(encoded: &str) -> String {
    assert_eq!(encoded.len() % 2, 0);
    let bytes: Vec<_> = encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect();
    String::from_utf8(bytes).unwrap()
}
fn joined(values: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    values
        .into_iter()
        .map(|value| value.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}
fn dimension(error: &FormulaDimensionRefusal) -> String {
    use FormulaOperandRequirement as Requirement;
    let requirement = |value: Requirement| match value {
        Requirement::Exact(kind) => kind.token(),
        Requirement::Arithmetic => "T",
        Requirement::Negatable => "N",
        Requirement::ToleranceName => "tolerance",
    };
    let mut rows: Vec<_> = error
        .wanted_signatures()
        .iter()
        .map(|row| {
            let result = match row.result_requirement() {
                FormulaResultRequirement::Exact(kind) => kind.token(),
                FormulaResultRequirement::Operand(index) => {
                    requirement(row.operand_requirements()[index])
                }
            };
            format!(
                "{}|{}|{}",
                joined(row.operand_requirements().iter().copied().map(requirement)),
                usize::from(matches!(row.arity(), FormulaBuiltinArity::OneOrMore)),
                result
            )
        })
        .collect();
    rows.sort();
    format!(
        "DIM:{};{};{};{};{}",
        error.operation().token(),
        joined(
            error
                .operands()
                .iter()
                .map(|operand| operand.kind().token())
        ),
        joined(error.operands().iter().map(|operand| match operand {
            FormulaBuiltinOperand::Tolerance(name) => name.token(),
            FormulaBuiltinOperand::Value(_) => "none",
        })),
        rows.join("#"),
        usize::from(error.arc_length_hint())
    )
}
fn expression_refusal(error: &FormulaExpressionCheckRefusal<'_>) -> String {
    match error {
        FormulaExpressionCheckRefusal::Dimension(error) => dimension(error),
        FormulaExpressionCheckRefusal::UnboundName(error) => {
            let mut origins: Vec<_> = error
                .origins_searched()
                .iter()
                .map(|origin| origin.token())
                .collect();
            origins.sort();
            format!("NAME:{}:{}", error.name(), joined(origins))
        }
        FormulaExpressionCheckRefusal::Call(error) => format!(
            "CALL:{}:{}:{}",
            error.name(),
            joined(error.origins_searched().iter().map(|source| source.token())),
            joined(
                error
                    .alternatives()
                    .iter()
                    .map(|alternative| alternative.token())
            )
        ),
    }
}
fn dependencies(proof: &FormulaCheckedExpression<'_, '_, '_>) -> String {
    joined(proof.dependencies().iter().map(|dependency| {
        let declaration = dependency.declaration();
        let ordinal = match declaration.source() {
            FormulaDeclarationSource::Recipe {
                statement_index, ..
            } => statement_index.to_string(),
            _ => "none".to_owned(),
        };
        format!(
            "{}:{}:{}",
            declaration.name(),
            declaration.origin().token(),
            ordinal
        )
    }))
}
fn expression(source: &str, namespace: &FormulaNamespace<'_>) -> String {
    let parsed = match FormulaExpression::parse(source) {
        Ok(parsed) => parsed,
        Err(error) => return format!("ERR\t{}\t-", error.diagnostic_code()),
    };
    let normalized = match parsed.normalize_literals() {
        Ok(normalized) => normalized,
        Err(error) => return format!("ERR\t{}\t-", error.diagnostic_code()),
    };
    match normalized.check_kinds(namespace) {
        Ok(proof) => {
            assert!(std::ptr::eq(proof.expression(), &normalized));
            assert_eq!(proof.canonical_expression(), normalized.canonical_form());
            format!("OK\t{}\t{}", proof.kind().token(), dependencies(&proof))
        }
        Err(error) => format!(
            "ERR\t{}\t{}",
            error.token(),
            expression_refusal(error.refusal())
        ),
    }
}
fn recipe(source: &str, namespace: FormulaNamespace<'_>) -> String {
    let parsed = match FormulaRecipe::parse(source) {
        Ok(parsed) => parsed,
        Err(error) => {
            return format!(
                "ERR\t{}\tSTAGE:syntax:{}",
                error.diagnostic_code(),
                error
                    .statement_index()
                    .map_or_else(|| "none".to_owned(), |index| index.to_string())
            )
        }
    };
    let normalized = match parsed.normalize_literals() {
        Ok(normalized) => normalized,
        Err(error) => {
            return format!(
                "ERR\t{}\tSTAGE:literal:{}",
                error.diagnostic_code(),
                error.statement_index()
            )
        }
    };
    let mut cursor = FormulaNameCursor::new(namespace, &normalized);
    let mut summaries = Vec::new();
    loop {
        let scope = match cursor.current() {
            Ok(Some(scope)) => scope,
            Ok(None) => break,
            Err(error) => {
                let sources = error.binding_sources();
                return format!(
                    "ERR\t{}\tCOLLISION:{}:{}",
                    error.token(),
                    error.name(),
                    joined(sources.iter().map(|source| source.origin().token()))
                );
            }
        };
        let ordinal = scope.statement_index();
        let statement = scope.statement();
        let proof = match scope.check_kinds() {
            Ok(proof) => proof,
            Err(error) => {
                let payload = match error.refusal() {
                    FormulaStatementCheckRefusal::Expression { error, .. } => {
                        expression_refusal(error.refusal())
                    }
                    FormulaStatementCheckRefusal::BindingDimension(error) => format!(
                        "BIND:{}:{}:{}",
                        error.declared_kind().token(),
                        error.expression_kind().token(),
                        error.wanted_kind().token()
                    ),
                    FormulaStatementCheckRefusal::AssertionDimension(error) => dimension(error),
                };
                return format!("ERR\t{}\t{}", error.token(), payload);
            }
        };
        assert_eq!(proof.statement_index(), ordinal);
        assert!(std::ptr::eq(
            proof.statement(),
            &normalized.statements()[ordinal - 1]
        ));
        assert_eq!(proof.canonical_statement(), statement.canonical_form());
        let summary = match (proof.kind(), statement.kind()) {
            (
                FormulaCheckedStatementKind::Let { expression },
                FormulaNormalizedStatementKind::Let {
                    expression: original,
                    ..
                },
            ) => {
                assert!(std::ptr::eq(expression.expression(), original));
                format!(
                    "{}:let:{}:{}",
                    ordinal,
                    expression.kind().token(),
                    dependencies(expression)
                )
            }
            (
                FormulaCheckedStatementKind::Assert { left, right },
                FormulaNormalizedStatementKind::Assert {
                    left: original_left,
                    right: original_right,
                    ..
                },
            ) => {
                assert!(std::ptr::eq(left.expression(), original_left));
                assert!(std::ptr::eq(right.expression(), original_right));
                format!(
                    "{}:assert:{},{}:{}~{}",
                    ordinal,
                    left.kind().token(),
                    right.kind().token(),
                    dependencies(left),
                    dependencies(right)
                )
            }
            _ => unreachable!("proof and actual statement must share their role"),
        };
        summaries.push(summary);
        assert!(cursor.advance_metadata().unwrap());
    }
    format!("OK\t{}\t{}", summaries.len(), summaries.join(";"))
}
fn namespace<'a>(
    names: &'a [MachineToken],
    kinds: &[(&str, &str)],
    length: &'a LengthDeclaration,
) -> FormulaNamespace<'a> {
    FormulaNamespace::new(names.iter().zip(kinds).enumerate().map(
        |(index, (name, (kind, origin)))| {
            let id = EntityId::from_bits(index as u128 + 100);
            let declaration = match *kind {
                "point" => FormulaDeclaration::point(name, PointRef::new(id, LocalTag::FIRST)),
                "edge" => FormulaDeclaration::edge(name, EdgeRef::new(id, LocalTag::FIRST)),
                "length" if matches!(*origin, "measurement" | "ease") => {
                    FormulaDeclaration::length_input(
                        name,
                        if *origin == "measurement" {
                            FormulaInputOrigin::Measurement
                        } else {
                            FormulaInputOrigin::Ease
                        },
                        id,
                        length,
                    )
                }
                _ => FormulaDeclaration::input(
                    name,
                    match *origin {
                        "parameter" => FormulaScalarInputOrigin::Parameter,
                        "profile" => FormulaScalarInputOrigin::Profile,
                        "material" => FormulaScalarInputOrigin::Material,
                        _ => unreachable!("authored input origin"),
                    },
                    id,
                    EntityId::from_bits(index as u128 + 200),
                    FormulaKind::from_token(kind)
                        .unwrap()
                        .binding_kind()
                        .unwrap(),
                ),
            };
            FormulaInitialDeclaration::try_from(declaration).unwrap()
        },
    ))
    .unwrap()
}
fn main() {
    // The first line supplies the oracle's authored initial metadata; it supplies no values.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let catalog = lines.next().unwrap().unwrap();
    let fields: Vec<_> = catalog
        .split(';')
        .map(|entry| entry.split(':').collect::<Vec<_>>())
        .collect();
    assert!(fields.iter().all(|entry| entry.len() == 3));
    let names: Vec<_> = fields
        .iter()
        .map(|entry| MachineToken::new(entry[0]).unwrap())
        .collect();
    let kinds: Vec<_> = fields.iter().map(|entry| (entry[1], entry[2])).collect();
    let length = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(1),
        source: EntityId::from_bits(2),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(3),
        },
    })
    .unwrap();
    let namespace = namespace(&names, &kinds, &length);
    for line in lines {
        let line = line.unwrap();
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 3);
        let source = decode(fields[2]);
        let outcome = match fields[1] {
            "E" => expression(&source, &namespace),
            "R" => recipe(&source, namespace.clone()),
            _ => unreachable!("authored adapter mode"),
        };
        println!("{}\t{}", fields[0], outcome);
    }
}
