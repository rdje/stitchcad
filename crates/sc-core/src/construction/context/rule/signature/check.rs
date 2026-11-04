//! Complete result/count/first original named role and exact kind,without copied names or values.
use super::DraftingRuleSignatureIssue as I;
use crate::{
    recipe::{
        CanonicalNumericInputKind, LogicalDerivationInputSource, SemanticOperation,
        SemanticRuleDeclaration, SemanticRuleInputKind, SemanticRuleSignature,
    },
    value::CanonicalInputTarget,
};
pub(super) fn numeric(target: CanonicalInputTarget) -> CanonicalNumericInputKind {
    match target {
        CanonicalInputTarget::Length(_) => CanonicalNumericInputKind::Length,
        CanonicalInputTarget::Scalar { kind, .. } => CanonicalNumericInputKind::Scalar(kind),
    }
}
pub(super) fn logical(source: LogicalDerivationInputSource) -> SemanticRuleInputKind {
    match source {
        LogicalDerivationInputSource::Logical(target) => {
            SemanticRuleInputKind::Logical(target.kind)
        }
        LogicalDerivationInputSource::Profile { kind, .. } => SemanticRuleInputKind::Logical(kind),
        LogicalDerivationInputSource::Numeric(target) => {
            SemanticRuleInputKind::Numeric(numeric(target))
        }
    }
}
fn count(inputs: usize, requirements: usize) -> Result<(), I> {
    if inputs != requirements {
        Err(I::InputCount {
            inputs,
            requirements,
        })
    } else {
        Ok(())
    }
}
pub(super) fn signature(
    operation: &SemanticOperation,
    rule: &SemanticRuleDeclaration,
) -> Result<(), I> {
    match (operation, &rule.definition().signature) {
        (
            SemanticOperation::LogicalDerivation(operation),
            SemanticRuleSignature::Logical {
                result,
                inputs: requirements,
            },
        ) => {
            let definition = operation.definition();
            if definition.output.kind != *result {
                return Err(I::ResultKind {
                    expected: definition.output.kind,
                    actual: *result,
                });
            }
            count(definition.inputs.len(), requirements.len())?;
            for (input_index, input) in definition.inputs.iter().enumerate() {
                let (requirement_index, requirement) = requirements
                    .iter()
                    .enumerate()
                    .find(|(_, requirement)| requirement.role == input.role)
                    .ok_or(I::MissingRole { input: input_index })?;
                let actual = logical(input.source);
                if actual != requirement.kind {
                    return Err(I::InputKind {
                        input: input_index,
                        requirement: requirement_index,
                        expected: requirement.kind,
                        actual,
                    });
                }
            }
        }
        (
            SemanticOperation::Buttonhole(operation),
            SemanticRuleSignature::Buttonhole {
                inputs: requirements,
            },
        ) => {
            let definition = operation.definition();
            count(definition.inputs.len(), requirements.len())?;
            for (input_index, input) in definition.inputs.iter().enumerate() {
                let (requirement_index, requirement) = requirements
                    .iter()
                    .enumerate()
                    .find(|(_, requirement)| requirement.role == input.role)
                    .ok_or(I::MissingRole { input: input_index })?;
                let actual = numeric(input.target);
                if actual != requirement.kind {
                    return Err(I::InputKind {
                        input: input_index,
                        requirement: requirement_index,
                        expected: SemanticRuleInputKind::Numeric(requirement.kind),
                        actual: SemanticRuleInputKind::Numeric(actual),
                    });
                }
            }
        }
        _ => return Err(I::Family),
    }
    Ok(())
}
