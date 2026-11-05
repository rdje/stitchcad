//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::{CheckedDraftingProgramOrder, DraftingProgramOrderStep};
use crate::{
    construction::{
        CenterArcArgument, DraftingInitialInputProvider, DraftingProgramOrderProofKind,
        PointCoordinate,
    },
    design::{
        fixed_value, DesignMetadataHost, DesignResourceLimitError, DesignResourceLocation,
        DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
};
fn at(
    scope: DesignResourceScope,
    owner: Option<EntityId>,
    field: &'static str,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope,
        owner,
        field,
        index,
    }
}
impl<
        'q,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > CheckedDraftingProgramOrder<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            inputs,
            limits,
            shape,
            steps,
        } = self;
        fixed_value(inputs);
        fixed_value(limits);
        fixed_value(shape);
        sink.charge_entries(
            steps.len(),
            at(scope, owner, "checked.drafting.order_steps", None),
        )?;
        for step in steps {
            let DraftingProgramOrderStep { position, kind } = step;
            fixed_value(position);
            match kind {
                DraftingProgramOrderProofKind::Semantic { ordinal, operation } => {
                    fixed_value(ordinal);
                    fixed_value(operation);
                }
                DraftingProgramOrderProofKind::Scalar(statement) => {
                    statement.charge_retained_resources(sink, scope, owner)?
                }
                DraftingProgramOrderProofKind::Point(point) => {
                    for (coordinate, expression) in point.dependency_expressions() {
                        let (certificates, dependencies) = match coordinate {
                            PointCoordinate::X => (
                                "checked.drafting.point.x.certificates",
                                "checked.drafting.point.x.dependencies",
                            ),
                            PointCoordinate::Y => (
                                "checked.drafting.point.y.certificates",
                                "checked.drafting.point.y.dependencies",
                            ),
                        };
                        expression.charge_retained_resources(
                            sink,
                            at(scope, owner, certificates, Some(*position)),
                            at(scope, owner, dependencies, Some(*position)),
                        )?;
                    }
                }
                DraftingProgramOrderProofKind::CenterArc(arc) => {
                    for (argument, expression) in arc.dependency_expressions() {
                        let (certificates, dependencies) = match argument {
                            CenterArcArgument::Radius => (
                                "checked.drafting.arc.radius.certificates",
                                "checked.drafting.arc.radius.dependencies",
                            ),
                            CenterArcArgument::StartAngle => (
                                "checked.drafting.arc.start_angle.certificates",
                                "checked.drafting.arc.start_angle.dependencies",
                            ),
                            CenterArcArgument::SweepAngle => (
                                "checked.drafting.arc.sweep_angle.certificates",
                                "checked.drafting.arc.sweep_angle.dependencies",
                            ),
                        };
                        expression.charge_retained_resources(
                            sink,
                            at(scope, owner, certificates, Some(*position)),
                            at(scope, owner, dependencies, Some(*position)),
                        )?;
                    }
                }
                DraftingProgramOrderProofKind::SlashSpread(opening) => {
                    for expression in opening.dependency_expressions() {
                        expression.charge_retained_resources(
                            sink,
                            at(
                                scope,
                                owner,
                                "checked.drafting.slash.opening.certificates",
                                Some(*position),
                            ),
                            at(
                                scope,
                                owner,
                                "checked.drafting.slash.opening.dependencies",
                                Some(*position),
                            ),
                        )?;
                    }
                }
                DraftingProgramOrderProofKind::Other(input) => fixed_value(input),
            }
        }
        Ok(())
    }
}
