//! Original typed semantic fields tied to actual normalized program/step owners.
use super::{DraftingProgramStepKind, DraftingProgramStepView, NormalizedDraftingProgram};
use crate::{
    construction::{
        DraftingContextSource, DraftingSemanticSelectionField, DraftingSemanticSelectionLocation,
    },
    design::{fixed_value, DesignMetadataHost},
    ontology::{EntityId, NotionSize},
    recipe::{
        ButtonholeOperationDefinition, ButtonholePolicyInput, LogicalDerivationDefinition,
        LogicalDerivationInput, SemanticOperation,
    },
    value::LogicalInputTarget,
};
use core::{fmt, iter::FusedIterator};
/// Original typed semantic source/output field,without association,selected value or work credit.
#[derive(Clone, Copy)]
pub enum DraftingSemanticField<'a> {
    /// Original canonical button-size origin.
    ButtonholeButton(&'a NotionSize),
    /// Original explicit physical-policy source identity.
    ButtonholePolicy(&'a EntityId),
    /// Every original named numeric policy input,including repeated targets.
    ButtonholeInput {
        /// Zero-based original input position.
        index: usize,
        /// Complete actual role/target record.
        input: &'a ButtonholePolicyInput,
    },
    /// Original typed logical output,without canonical/Derived association.
    LogicalOutput(&'a LogicalInputTarget),
    /// Original explicit logical-rule source identity.
    LogicalRule(&'a EntityId),
    /// Every original named logical/Profile/numeric input,without filtering variants.
    LogicalInput {
        /// Zero-based original input position.
        index: usize,
        /// Complete actual role/typed-source record.
        input: &'a LogicalDerivationInput,
    },
}
impl fmt::Debug for DraftingSemanticField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ButtonholeButton(_) => f.write_str("ButtonholeButton"),
            Self::ButtonholePolicy(_) => f.write_str("ButtonholePolicy"),
            Self::ButtonholeInput { index, .. } => {
                f.debug_tuple("ButtonholeInput").field(index).finish()
            }
            Self::LogicalOutput(_) => f.write_str("LogicalOutput"),
            Self::LogicalRule(_) => f.write_str("LogicalRule"),
            Self::LogicalInput { index, .. } => f.debug_tuple("LogicalInput").field(index).finish(),
        }
    }
}
/// Actual original semantic operation/field at its genuine normalized program step. Complete
/// original coverage has been admitted;namespace,kinds,sources,dependency order and values have not.
/// Retaining the exact program prevents equal-ID steps from another owner substituting in joins.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingContextSource<H>>(use_:&mut DraftingProgramSemanticUse<'_,'_,'_,H,S>) {use_.ordinal=1;}
/// ```
/// A view cannot outlive its actual normalized owner:
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(program:&'n NormalizedDraftingProgram<'p,'a,H,S>)
///     ->Option<DraftingProgramSemanticUse<'static,'p,'a,H,S>> {program.semantic_dependencies().next()}
/// ```
pub struct DraftingProgramSemanticUse<
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    program: &'n NormalizedDraftingProgram<'p, 'a, H, S>,
    step: DraftingProgramStepView<'n, 'a>,
    ordinal: usize,
    operation: &'a SemanticOperation,
    field: DraftingSemanticField<'a>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingProgramSemanticUse<'_, '_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingProgramSemanticUse<'_, '_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramSemanticUse<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramSemanticUse")
            .field("position", &self.step.position())
            .field("ordinal", &self.ordinal)
            .field("field", &self.field)
            .finish_non_exhaustive()
    }
}
impl<'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramSemanticUse<'n, 'p, 'a, H, S>
{
    /// Exact complete normalized program owner,not an equal-ID or recomputed substitute.
    #[must_use]
    pub const fn program(self) -> &'n NormalizedDraftingProgram<'p, 'a, H, S> {
        self.program
    }
    /// Genuine original mixed step view;semantic ordinal and field indices remain separate.
    #[must_use]
    pub const fn step(self) -> DraftingProgramStepView<'n, 'a> {
        self.step
    }
    /// Genuine original one-based mixed position,without scalar/native ordinal reconstruction.
    #[must_use]
    pub const fn position(self) -> usize {
        self.step.position()
    }
    /// Original one-based semantic recipe ordinal,independent of mixed position.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Actual original member of this program's Root semantic recipe.
    #[must_use]
    pub const fn operation(self) -> &'a SemanticOperation {
        self.operation
    }
    /// Exact actual typed semantic dependency/output field.
    #[must_use]
    pub const fn field(self) -> DraftingSemanticField<'a> {
        self.field
    }
    /// Fixed location of this original field using its genuine semantic ordinal,not its mixed
    /// position. This syntax projection accepts no row,rule origin,Profile need,source or work.
    #[must_use]
    pub const fn selection_location(self) -> DraftingSemanticSelectionLocation {
        use DraftingSemanticSelectionField as S;
        let field = match self.field {
            DraftingSemanticField::ButtonholeButton(_) => S::ButtonholeButton,
            DraftingSemanticField::ButtonholePolicy(_) => S::ButtonholePolicy,
            DraftingSemanticField::ButtonholeInput { index, input: _ } => {
                S::ButtonholeInput { index }
            }
            DraftingSemanticField::LogicalOutput(_) => S::LogicalOutput,
            DraftingSemanticField::LogicalRule(_) => S::LogicalRule,
            DraftingSemanticField::LogicalInput { index, input: _ } => S::LogicalInput { index },
        };
        DraftingSemanticSelectionLocation::Semantic {
            ordinal: self.ordinal,
            field,
        }
    }
}
impl<'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    NormalizedDraftingProgram<'p, 'a, H, S>
{
    /// Stream all original semantic operation headers/inputs in actual mixed order,then original
    /// definition/input order. Scalar/native slots contribute no semantic-operation fields. This
    /// does not classify their other dependency obligations or Root Pocket/Closure references.
    /// No query,value,clone,result vector or new arena;the caller owns complete work preflight.
    /// ```
    /// use sc_core::{construction::*,design::DesignMetadataHost};
    /// fn uses<H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(program:&NormalizedDraftingProgram<'_,'_,H,S>)->usize {
    ///     program.semantic_dependencies().count()
    /// }
    /// ```
    pub fn semantic_dependencies(
        &self,
    ) -> impl FusedIterator<Item = DraftingProgramSemanticUse<'_, 'p, 'a, H, S>> + '_ {
        self.steps()
            .filter_map(|step| match step.kind() {
                DraftingProgramStepKind::Semantic { ordinal, operation } => {
                    Some((step, ordinal, operation))
                }
                DraftingProgramStepKind::Scalar { .. } | DraftingProgramStepKind::Drafting(_) => {
                    None
                }
            })
            .flat_map(move |(step, ordinal, operation)| {
                fields(operation).map(move |field| DraftingProgramSemanticUse {
                    program: self,
                    step,
                    ordinal,
                    operation,
                    field,
                })
            })
            .fuse()
    }
}
fn fields(operation: &SemanticOperation) -> impl Iterator<Item = DraftingSemanticField<'_>> {
    let (headers, button_inputs, logical_inputs) = match operation {
        SemanticOperation::Buttonhole(operation) => {
            let ButtonholeOperationDefinition {
                id,
                button,
                policy,
                inputs,
            } = operation.definition();
            fixed_value(id);
            (
                [
                    DraftingSemanticField::ButtonholeButton(button),
                    DraftingSemanticField::ButtonholePolicy(policy),
                ],
                inputs.as_slice(),
                &[][..],
            )
        }
        SemanticOperation::LogicalDerivation(operation) => {
            let LogicalDerivationDefinition {
                id,
                output,
                rule,
                inputs,
            } = operation.definition();
            fixed_value(id);
            (
                [
                    DraftingSemanticField::LogicalOutput(output),
                    DraftingSemanticField::LogicalRule(rule),
                ],
                &[][..],
                inputs.as_slice(),
            )
        }
    };
    headers
        .into_iter()
        .chain(button_inputs.iter().enumerate().map(|(index, input)| {
            let ButtonholePolicyInput { role, target } = input;
            let _: &crate::name::MachineToken = role;
            fixed_value(target);
            DraftingSemanticField::ButtonholeInput { index, input }
        }))
        .chain(logical_inputs.iter().enumerate().map(|(index, input)| {
            let LogicalDerivationInput { role, source } = input;
            let _: &crate::name::MachineToken = role;
            fixed_value(source);
            DraftingSemanticField::LogicalInput { index, input }
        }))
}
