//! Original layer offset intent; recipe, source geometry and execution scope retain their owners.
use super::{DesignInputs, DesignInputsError, DesignReferenceHost};
use crate::{
    ontology::{LayerInventory, LayerOffsetRelationship, LayerRef},
    value::CanonicalInputs,
};
use core::fmt;

/// Actual layer/current canonical owner and its original unexecuted offset relationship.
///
/// ```compile_fail
/// fn escape<H: sc_core::design::DesignReferenceHost>(state: sc_core::design::DesignInputs<H>,
///     layers: sc_core::ontology::LayerInventory)
///     -> Vec<sc_core::design::DesignLayerOffsetDeclaration<'static>> {
///     state.layer_offset_declarations(&layers).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct DesignLayerOffsetDeclaration<'a> {
    layer: LayerRef<'a>,
    canonical: &'a CanonicalInputs,
}
impl fmt::Debug for DesignLayerOffsetDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignLayerOffsetDeclaration")
            .field("kind", &self.layer.kind())
            .finish_non_exhaustive()
    }
}
impl<'a> DesignLayerOffsetDeclaration<'a> {
    /// Exact original distinct layer, including inspectable modelled Lining.
    #[must_use]
    pub const fn layer(self) -> LayerRef<'a> {
        self.layer
    }
    /// Actual current root's single canonical input owner; no operation value is inferred from it.
    #[must_use]
    pub const fn canonical_inputs(self) -> &'a CanonicalInputs {
        self.canonical
    }
    /// Original operation identity and directed source-list intent, without rewriting held ranges.
    #[must_use]
    pub const fn relationship(self) -> &'a LayerOffsetRelationship {
        &self.layer.definition().offset
    }
}
impl<H: DesignReferenceHost> DesignInputs<H> {
    /// Borrow every actual layer's offset intent after complete current root validation.
    ///
    /// Order is authored Facing, Lining, then Interfacing order. No operation lookup or numeric
    /// coercion is performed: recipe existence/kind/dependencies, current layer sources, assignments,
    /// global garment ids and physical validation remain separate. Modelled Lining stays inspectable;
    /// requested execution must apply the original layer's scope check. No partial views escape.
    ///
    /// # Errors
    /// Returns the original whole current root cause, including for an empty layer inventory.
    pub fn layer_offset_declarations<'a>(
        &'a self,
        layers: &'a LayerInventory,
    ) -> Result<Vec<DesignLayerOffsetDeclaration<'a>>, DesignInputsError<H::Error>> {
        self.validate_current()?;
        Ok(layers
            .entries()
            .map(|(_, layer)| DesignLayerOffsetDeclaration {
                layer,
                canonical: self.inputs(),
            })
            .collect())
    }
}
