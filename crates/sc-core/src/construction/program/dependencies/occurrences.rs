//! Streaming genuine mixed dependency occurrences;no values,canonical grant or edge-vector copy.
use crate::{
    construction::{
        CenterArcArgument, CheckedDraftingProgramOrder, DraftingInitialInputProvider,
        DraftingProgramOrderProofKind as K, DraftingProgramOrderStep, PointCoordinate,
    },
    design::DesignMetadataHost,
    recipe::{
        FormulaDeclaration, FormulaNameDependency, FormulaRecipeDependencyRole, FormulaSourceSpan,
    },
};
use core::{fmt, iter::FusedIterator};

/// Original scalar header/operand or native argument role;none is an invented combined expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramDependencyRole {
    /// Original scalar Binding/AssertionTolerance/Left/Right metadata role.
    Scalar(FormulaRecipeDependencyRole),
    /// Actual original point coordinate argument.
    Point(PointCoordinate),
    /// Actual original center-arc scalar argument.
    CenterArc(CenterArcArgument),
    /// Actual original slash/spread amount argument;mode remains on its real proof owner.
    SlashSpread,
}
/// One original dependency occurrence pinned to its genuine complete mixed consumer proof.
/// Header occurrences retain their actual annotation span;they have no fabricated AST node.
/// Copying this view copies only loans/metadata. It proves no canonical/current geometry source.
/// ```compile_fail,E0616
/// fn alter(use_:&mut sc_core::construction::DraftingProgramNameDependency<'_, '_>) {use_.role=todo!();}
/// ```
/// ```compile_fail
/// fn escape<'v,'q>(use_:sc_core::construction::DraftingProgramNameDependency<'v,'q>)
///     ->sc_core::construction::DraftingProgramNameDependency<'static,'q> {use_}
/// ```
#[derive(Clone, Copy)]
pub struct DraftingProgramNameDependency<'v, 'q> {
    step: &'v DraftingProgramOrderStep<'q>,
    role: DraftingProgramDependencyRole,
    span: FormulaSourceSpan,
    declaration: FormulaDeclaration<'q>,
}
impl<'v, 'q> DraftingProgramNameDependency<'v, 'q> {
    /// Actual complete original scalar/native consumer,not a detached index or caller hint.
    #[must_use]
    pub const fn step(self) -> &'v DraftingProgramOrderStep<'q> {
        self.step
    }
    /// Genuine one-based mixed position of this actual consumer.
    #[must_use]
    pub const fn position(self) -> usize {
        self.step.position()
    }
    /// Actual scalar-source ordinal,or None for a native argument occurrence.
    #[must_use]
    pub fn statement_index(self) -> Option<usize> {
        match self.step.kind() {
            K::Scalar(proof) => Some(proof.statement_index()),
            K::Point(_) | K::CenterArc(_) | K::SlashSpread(_) | K::Other(_) => None,
        }
    }
    /// Original header/operand/argument role.
    #[must_use]
    pub const fn role(self) -> DraftingProgramDependencyRole {
        self.role
    }
    /// Original source use or assertion-header span,without renumbering or source extraction.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
    /// Exact original supplier metadata;explicit inspection can expose authored names.
    #[must_use]
    pub const fn declaration(self) -> FormulaDeclaration<'q> {
        self.declaration
    }
}
impl fmt::Debug for DraftingProgramNameDependency<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramNameDependency")
            .field("position", &self.position())
            .field("statement_index", &self.statement_index())
            .field("role", &self.role)
            .field("span", &self.span)
            .finish_non_exhaustive()
    }
}
fn step_dependencies<'v, 'q>(
    step: &'v DraftingProgramOrderStep<'q>,
) -> impl Iterator<Item = DraftingProgramNameDependency<'v, 'q>> + 'v {
    let scalar = match step.kind() {
        K::Scalar(proof) => Some(proof),
        K::Point(_) | K::CenterArc(_) | K::SlashSpread(_) | K::Other(_) => None,
    };
    let scalar = scalar.into_iter().flat_map(move |proof| {
        proof
            .dependencies()
            .map(move |use_| DraftingProgramNameDependency {
                step,
                role: DraftingProgramDependencyRole::Scalar(use_.role()),
                span: use_.span(),
                declaration: use_.declaration(),
            })
    });
    let operands: [Option<(DraftingProgramDependencyRole, &[FormulaNameDependency<'q>])>; 3] =
        match step.kind() {
            K::Point(proof) => {
                let [(xr, x), (yr, y)] = proof.dependency_expressions();
                [
                    Some((DraftingProgramDependencyRole::Point(xr), x.dependencies())),
                    Some((DraftingProgramDependencyRole::Point(yr), y.dependencies())),
                    None,
                ]
            }
            K::CenterArc(proof) => {
                let [(rr, r), (sr, s), (wr, w)] = proof.dependency_expressions();
                [
                    Some((
                        DraftingProgramDependencyRole::CenterArc(rr),
                        r.dependencies(),
                    )),
                    Some((
                        DraftingProgramDependencyRole::CenterArc(sr),
                        s.dependencies(),
                    )),
                    Some((
                        DraftingProgramDependencyRole::CenterArc(wr),
                        w.dependencies(),
                    )),
                ]
            }
            K::SlashSpread(proof) => {
                let [expression] = proof.dependency_expressions();
                [
                    Some((
                        DraftingProgramDependencyRole::SlashSpread,
                        expression.dependencies(),
                    )),
                    None,
                    None,
                ]
            }
            K::Scalar(_) | K::Other(_) => [None, None, None],
        };
    scalar.chain(
        operands
            .into_iter()
            .flatten()
            .flat_map(move |(role, dependencies)| {
                dependencies
                    .iter()
                    .copied()
                    .map(move |use_| DraftingProgramNameDependency {
                        step,
                        role,
                        span: use_.span(),
                        declaration: use_.declaration(),
                    })
            }),
    )
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
    /// Stream every original mixed name occurrence without allocating or cloning a whole edge
    /// vector. Scalar statements reuse the original whole-recipe producer,including tolerance
    /// header before assertion left/right. Native order is Point X/Y,Arc Radius/Start/Sweep,
    /// Slash opening;Other has no scalar operands. All repeated/untaken uses remain distinct.
    /// Native proof-owner field guards keep added arguments from silently disappearing.
    ///
    /// ```
    /// use sc_core::{construction::{CheckedDraftingProgramOrder,DraftingInitialInputProvider},design::DesignMetadataHost};
    /// fn name_uses<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     order:&CheckedDraftingProgramOrder<'_, '_, '_, '_, '_, '_, '_,H,S>)->usize {
    ///     order.dependencies().count()
    /// }
    /// ```
    pub fn dependencies(
        &self,
    ) -> impl FusedIterator<Item = DraftingProgramNameDependency<'_, 'q>> + '_ {
        self.steps().iter().flat_map(step_dependencies).fuse()
    }
}
