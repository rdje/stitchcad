//! Existing sourced numeric kernels; no replacement backend or caller-supplied AST values.
use super::{
    FormulaScalarExecutionFailure as Failure, FormulaScalarExecutionPlan,
    FormulaScalarExecutionReadKind as Read,
};
use crate::recipe::{
    FormulaBuiltin as B, FormulaCheckedNode, FormulaCheckedNodeForm as Form,
    FormulaEvaluatedScalar as V,
};

pub(super) fn call<'a>(
    plan: &FormulaScalarExecutionPlan<'a>,
    node: FormulaCheckedNode<'a, 'a>,
    builtin: B,
    values: &[V],
) -> Result<V, Failure<'a>> {
    let argument = |index| {
        let Some(value) = values.get(index) else {
            unreachable!("complete checked call retains its actual fixed argument count");
        };
        *value
    };
    let result = match builtin {
        B::Abs => argument(0).checked_abs(),
        B::Sqrt => argument(0).checked_sqrt(),
        B::Hypot => argument(0).checked_hypot(argument(1)),
        B::Min => V::checked_min(values),
        B::Max => V::checked_max(values),
        B::Clamp => argument(0).checked_clamp(argument(1), argument(2)),
        B::RoundTo => argument(0).checked_round_to(argument(1)),
        B::Sin => argument(0).checked_sin(),
        B::Cos => argument(0).checked_cos(),
        B::Atan => argument(0).checked_atan(),
        B::Atan2 => argument(0).checked_atan2(argument(1)),
        B::ArcLength => argument(0).checked_arc_length(argument(1)),
        B::Within => {
            let Form::Call { arguments, .. } = node.form() else {
                unreachable!("actual checked within call retains its own argument nodes");
            };
            let Some(class_node) = arguments.iter().next_back() else {
                unreachable!("checked within retains its actual direct tolerance-name operand");
            };
            let Some(index) = class_node.dependency_index() else {
                unreachable!("direct certified tolerance operand retains its name occurrence");
            };
            let Some(read) = plan.sources.get(index).and_then(|slot| slot.read) else {
                unreachable!("all actual within operands were evaluated before application");
            };
            let Read::Tolerance(tolerance) = read.source else {
                unreachable!("actual direct reserved class was read from its quantitative context");
            };
            return argument(0)
                .compare_tolerance(argument(1), tolerance)
                .map(|comparison| comparison.value())
                .map_err(Failure::Comparison);
        }
        B::If => unreachable!("actual conditional syntax uses selected branch tasks,not callbacks"),
        B::Tan | B::X | B::Y | B::Dist | B::Dir | B::Len | B::ParamAt | B::PointAt => {
            unreachable!("private complete scalar plan excludes unimplemented capability families")
        }
    };
    result.map_err(Failure::Numeric)
}
