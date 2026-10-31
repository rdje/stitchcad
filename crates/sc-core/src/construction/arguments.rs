//! Private kernels; actual schema fields/roles supply documents and required kinds.
use crate::recipe::{
    FormulaCheckedExpression, FormulaExpressionCheckError, FormulaExpressionDocument, FormulaKind,
    FormulaNamespace, FormulaNormalizedExpression,
};

pub(super) enum PopulationRefusal {
    Overflow,
    Nodes { allowed: usize, required: usize },
    RetainedRecords { allowed: usize, required: usize },
}
// No caller count participates: complete populations come from immutable document factories.
pub(super) fn measure<const N: usize>(
    documents: [&FormulaExpressionDocument; N],
    node_limit: usize,
    retained_limit: usize,
) -> Result<(usize, usize), PopulationRefusal> {
    let nodes = documents.into_iter().try_fold(0usize, |total, document| {
        total
            .checked_add(document.node_count())
            .ok_or(PopulationRefusal::Overflow)
    })?;
    let records = nodes
        .checked_mul(2)
        .and_then(|n| n.checked_add(N))
        .ok_or(PopulationRefusal::Overflow)?;
    if nodes > node_limit {
        return Err(PopulationRefusal::Nodes {
            allowed: node_limit,
            required: nodes,
        });
    }
    if records > retained_limit {
        return Err(PopulationRefusal::RetainedRecords {
            allowed: retained_limit,
            required: records,
        });
    }
    Ok((nodes, records))
}

pub(super) enum CheckRefusal<'e, 's> {
    Expression(FormulaExpressionCheckError<'e, 's>),
    Kind { actual: FormulaKind },
}
// Only private schema checkers select required kinds from their actual closed argument roles.
pub(super) fn check_expression<'e, 's, 'd>(
    expression: &'e FormulaNormalizedExpression<'s>,
    namespace: &FormulaNamespace<'d>,
    required: FormulaKind,
) -> Result<FormulaCheckedExpression<'e, 's, 'd>, CheckRefusal<'e, 's>> {
    let proof = expression
        .check_kinds(namespace)
        .map_err(CheckRefusal::Expression)?;
    if proof.kind() != required {
        return Err(CheckRefusal::Kind {
            actual: proof.kind(),
        });
    }
    Ok(proof)
}
