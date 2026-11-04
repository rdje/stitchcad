//! Complete actual request row/pin content;no Root/provider/source or coordinate lookup.
use super::{
    DraftingClosureSelectionField as C, DraftingNativeSelectionField as N,
    DraftingPocketSelectionField as P, DraftingRuleSelection, DraftingSemanticSelectionField as S,
    DraftingSemanticSelectionInputs, DraftingSemanticSelectionLocation as L,
    DraftingSemanticSelectionRow,
};
use crate::{
    design::{
        charge_selected_profile_pin_resources, fixed_value, DesignResourceLimitError,
        DesignResourceLocation, DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
};
impl DraftingSemanticSelectionInputs<'_> {
    /// Append the complete original row vector and every original selected-pin string to a caller's
    /// cumulative sink. Repeated/unused/invalid selections still count before lookup. Scope/owner
    /// must describe the actual containing record;opaque pin labels supply no manufactured ID.
    /// Static field tokens and original row indices identify each cause. No validation,source/
    /// currentness/value query,Root/provider callback,clone or constructor-demand credit occurs.
    /// Caller owns and finishes the meter;record accounting grants no whole-source allowance.
    /// ```
    /// use sc_core::{construction::DraftingSemanticSelectionInputs,design::*};
    /// fn records(inputs:&DraftingSemanticSelectionInputs<'_>,sink:&mut dyn DesignResourceSink)
    ///     ->Result<(),DesignResourceLimitError> {
    ///     inputs.charge_record_resources(sink,DesignResourceScope::CommandState,None)
    /// }
    /// ```
    /// # Errors
    /// Original first row/text or earlier latched cause,including for empty/fixed-only records.
    pub fn charge_record_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self { rows } = self;
        sink.charge_entries(
            rows.len(),
            DesignResourceLocation {
                scope,
                owner,
                field: "drafting.semantic.selections.rows",
                index: None,
            },
        )?;
        for (index, row) in rows.iter().enumerate() {
            match row {
                DraftingSemanticSelectionRow::Profile { location, selected } => {
                    location_fields(location);
                    charge_selected_profile_pin_resources(
                        selected,
                        sink,
                        scope,
                        owner,
                        Some(index),
                    )?;
                }
                DraftingSemanticSelectionRow::Rule { ordinal, origin } => {
                    fixed_value(ordinal);
                    match origin {
                        DraftingRuleSelection::Design => {}
                        DraftingRuleSelection::Profile(selected) => {
                            charge_selected_profile_pin_resources(
                                selected,
                                sink,
                                scope,
                                owner,
                                Some(index),
                            )?
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
fn location_fields(location: &L) {
    match location {
        L::Pocket { pocket, field } => {
            fixed_value(pocket);
            match field {
                P::Served | P::Position | P::Orientation | P::Opening => {}
                P::Component { index } => fixed_value(index),
            }
        }
        L::Closure { closure, field } => {
            fixed_value(closure);
            match field {
                C::Length | C::Buttonhole | C::UnsupportedFly => {}
                C::Size(role) => fixed_value(role),
                C::Instance { index } => fixed_value(index),
            }
        }
        L::Semantic { ordinal, field } => {
            fixed_value(ordinal);
            match field {
                S::ButtonholeButton | S::ButtonholePolicy | S::LogicalOutput | S::LogicalRule => {}
                S::ButtonholeInput { index } | S::LogicalInput { index } => fixed_value(index),
            }
        }
        L::Native { position, field } => {
            fixed_value(position);
            match field {
                N::Target(role) => fixed_value(role),
                N::NotchProfile(role) => fixed_value(role),
                N::WalkNotchProfile {
                    pair_index,
                    side,
                    role,
                } => {
                    fixed_value(pair_index);
                    fixed_value(side);
                    fixed_value(role);
                }
            }
        }
    }
}
