//! Complete actual garment declaration views; source/metadata checks remain distinct from execution.
use super::{
    DesignClosureDeclaration, DesignClosureDeclarationError, DesignConstructionDeclaration,
    DesignConstructionDeclarationError, DesignGarment, DesignGarmentError, DesignGatherDeclaration,
    DesignGatherDeclarationError, DesignHemDeclaration, DesignHemDeclarationError,
    DesignInputsError, DesignLayerOffsetDeclaration, DesignPocketOpeningDeclaration,
    DesignReferenceHost, DesignSewingAmount, DesignSewingError,
};
use crate::{
    ontology::{EntityId, LayerInventoryError, LayerInventoryPosition, LayerMaterialView},
    value::CanonicalInputs,
};
use core::fmt;
use std::collections::BTreeMap;

/// Existing original adapters, without copied authored values, state, uncertainty or geometry.
/// Direct construction/inspection supplies no independent current-validation certificate.
#[derive(Clone, Copy)]
pub enum DesignGarmentDeclarationSource<'a> {
    /// Actual original sewing span and signed ease metadata.
    SewingAmount(DesignSewingAmount<'a>),
    /// Actual original Pocket and logical opening declaration.
    PocketOpening(DesignPocketOpeningDeclaration<'a>),
    /// Actual original Closure field role, including canonical buttonhole derivation intent.
    Closure(DesignClosureDeclaration<'a>),
    /// Actual original distinct layer's paired Material assignment and unexecuted offset intent.
    Layer {
        /// Existing actual Material/unresolved-reason adapter, without substituted assignment.
        assignment: LayerMaterialView<'a>,
        /// Existing actual layer/offset relationship adapter.
        offset: DesignLayerOffsetDeclaration<'a>,
    },
    /// Actual original Hem depth/fold role.
    Hem(DesignHemDeclaration<'a>),
    /// Actual original Dart/fold/grain/allowance field role.
    Construction(DesignConstructionDeclaration<'a>),
    /// Actual original Gather, selected signed span ease and logical closing intent.
    Gather(DesignGatherDeclaration<'a>),
}
impl fmt::Debug for DesignGarmentDeclarationSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SewingAmount(_) => "SewingAmount",
            Self::PocketOpening(_) => "PocketOpening",
            Self::Closure(_) => "Closure",
            Self::Layer { .. } => "Layer",
            Self::Hem(_) => "Hem",
            Self::Construction(_) => "Construction",
            Self::Gather(_) => "Gather",
        })
    }
}
/// Actual original role adapter and the complete current garment's single canonical owner.
///
/// No geometry, numeric availability, resolved logical domain, operation execution or release
/// certificate follows. All results keep the actual garment owner borrowed while used.
/// ```compile_fail
/// fn escape<H: sc_core::design::DesignReferenceHost>(garment: sc_core::design::DesignGarment<H>)
///     -> Vec<sc_core::design::DesignGarmentDeclaration<'static>> {
///     garment.declarations().unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct DesignGarmentDeclaration<'a> {
    canonical: &'a CanonicalInputs,
    source: DesignGarmentDeclarationSource<'a>,
}
impl fmt::Debug for DesignGarmentDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignGarmentDeclaration")
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}
impl<'a> DesignGarmentDeclaration<'a> {
    /// Actual original containing root's sole canonical owner, never a copied inventory.
    #[must_use]
    pub const fn canonical_inputs(self) -> &'a CanonicalInputs {
        self.canonical
    }
    /// Original typed adapter with its distinct semantic role, object and declaration origins.
    #[must_use]
    pub const fn source(self) -> DesignGarmentDeclarationSource<'a> {
        self.source
    }
}
/// Original complete current-source or declaration-adapter cause, without a partial output prefix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignGarmentDeclarationError<E> {
    /// Whole garment ownership/current source/assignment refusal, before any metadata views.
    Garment(Box<DesignGarmentError<E>>),
    /// Original complete current sewing amount/context cause.
    Sewing(Box<DesignSewingError<E>>),
    /// Original input-root cause from a logical Pocket or layer-offset query.
    Inputs(Box<DesignInputsError<E>>),
    /// Original Closure scope/formula-metadata cause.
    Closures(Box<DesignClosureDeclarationError<E>>),
    /// Original complete layer/Material assignment cause.
    Layers(Box<LayerInventoryError>),
    /// Original Hem declaration/formula-metadata cause.
    Hems(Box<DesignHemDeclarationError<E>>),
    /// Original construction field/parameter/kind cause with actual family/position/role.
    Constructions(Box<DesignConstructionDeclarationError<E>>),
    /// Original complete current Gather/span/amount join cause.
    Gathers(Box<DesignGatherDeclarationError<E>>),
    /// Actual original layer lacks a paired adapter; refuse rather than truncate or substitute.
    LayerAdapterMismatch {
        /// Actual original layer identity.
        layer: EntityId,
        /// Actual distinct family and authored index.
        position: LayerInventoryPosition,
    },
}
impl<E: fmt::Display> fmt::Display for DesignGarmentDeclarationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Garment(issue) => write!(f, "garment declarations current sources: {issue}"),
            Self::Sewing(issue) => write!(f, "garment sewing declarations: {issue}"),
            Self::Inputs(issue) => write!(f, "garment declaration inputs: {issue}"),
            Self::Closures(issue) => write!(f, "garment Closure declarations: {issue}"),
            Self::Layers(issue) => write!(f, "garment layer declarations: {issue}"),
            Self::Hems(issue) => write!(f, "garment Hem declarations: {issue}"),
            Self::Constructions(issue) => write!(f, "garment construction declarations: {issue}"),
            Self::Gathers(issue) => write!(f, "garment Gather declarations: {issue}"),
            Self::LayerAdapterMismatch { layer, position } => write!(
                f, "garment layer {layer} at {position:?} lacks its actual paired Material/offset adapters"
            ),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignGarmentDeclarationError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Garment(issue) => Some(issue.as_ref()),
            Self::Sewing(issue) => Some(issue.as_ref()),
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::Closures(issue) => Some(issue.as_ref()),
            Self::Layers(issue) => Some(issue.as_ref()),
            Self::Hems(issue) => Some(issue.as_ref()),
            Self::Constructions(issue) => Some(issue.as_ref()),
            Self::Gathers(issue) => Some(issue.as_ref()),
            Self::LayerAdapterMismatch { .. } => None,
        }
    }
}
impl<H: DesignReferenceHost> DesignGarment<H> {
    /// Borrow all implemented garment declarations only after complete current garment validation.
    ///
    /// Order is authored sewing spans, Pocket openings, Closure field roles, original distinct layers
    /// (paired Material/offset intent per layer), Hem roles, Dart/Tuck/Pleat intake/closing roles,
    /// joined Gathers, then grain/allowance roles. Independent notion placements have source fields;
    /// their hardware size roles live on actual Closures. Existing adapters retain original numeric
    /// metadata/logical origins, Profile/physical deferrals and concrete errors. Unknown/Derived stay
    /// unread; no value, state, uncertainty, operation result or profile policy is copied/defaulted.
    /// Empty/unused sources remain obligations. No partial collection escapes a late refusal.
    /// Logical domain resolution, operation existence/kind/dependencies, requested execution envelope,
    /// geometry, conservation and physical/release proof remain separate.
    /// # Errors
    /// Returns original whole garment cause first, then original adapter causes in documented order.
    /// Missing actual layer adapters fail closed without truncating the collection.
    pub fn declarations(
        &self,
    ) -> Result<Vec<DesignGarmentDeclaration<'_>>, DesignGarmentDeclarationError<H::Error>> {
        use DesignGarmentDeclarationSource as Source;
        self.validate_current()
            .map_err(|issue| DesignGarmentDeclarationError::Garment(Box::new(issue)))?;
        let canonical = self.inputs().inputs();
        let wrap = |source| DesignGarmentDeclaration { canonical, source };
        let mut views = Vec::new();
        views.extend(
            self.sewing()
                .amount_declarations()
                .map_err(|issue| DesignGarmentDeclarationError::Sewing(Box::new(issue)))?
                .into_iter()
                .map(|value| wrap(Source::SewingAmount(value))),
        );
        views.extend(
            self.inputs()
                .pocket_opening_declarations(self.pockets())
                .map_err(|issue| DesignGarmentDeclarationError::Inputs(Box::new(issue)))?
                .into_iter()
                .map(|value| wrap(Source::PocketOpening(value))),
        );
        views.extend(
            self.inputs()
                .closure_declarations(self.closures())
                .map_err(|issue| DesignGarmentDeclarationError::Closures(Box::new(issue)))?
                .into_iter()
                .map(|value| wrap(Source::Closure(value))),
        );
        let assignments = self
            .layers()
            .material_assignments(self.inputs().materials(), canonical)
            .map_err(|issue| DesignGarmentDeclarationError::Layers(Box::new(issue)))?
            .into_iter()
            .map(|assignment| {
                let layer = match assignment {
                    LayerMaterialView::Assigned { layer, .. }
                    | LayerMaterialView::Unresolved { layer, .. } => layer,
                };
                (layer.id(), assignment)
            })
            .collect::<BTreeMap<_, _>>();
        let offsets = self
            .inputs()
            .layer_offset_declarations(self.layers())
            .map_err(|issue| DesignGarmentDeclarationError::Inputs(Box::new(issue)))?
            .into_iter()
            .map(|offset| (offset.layer().id(), offset))
            .collect::<BTreeMap<_, _>>();
        for (position, layer) in self.layers().entries() {
            let (Some(assignment), Some(offset)) =
                (assignments.get(&layer.id()), offsets.get(&layer.id()))
            else {
                return Err(DesignGarmentDeclarationError::LayerAdapterMismatch {
                    layer: layer.id(),
                    position,
                });
            };
            views.push(wrap(Source::Layer {
                assignment: *assignment,
                offset: *offset,
            }));
        }
        views.extend(
            self.inputs()
                .hem_declarations(self.hems())
                .map_err(|issue| DesignGarmentDeclarationError::Hems(Box::new(issue)))?
                .into_iter()
                .map(|value| wrap(Source::Hem(value))),
        );
        views.extend(
            self.inputs()
                .construction_intake_declarations(self.constructions())
                .map_err(|issue| DesignGarmentDeclarationError::Constructions(Box::new(issue)))?
                .into_iter()
                .map(|value| wrap(Source::Construction(value))),
        );
        views.extend(
            self.sewing()
                .gather_declarations(self.constructions())
                .map_err(|issue| DesignGarmentDeclarationError::Gathers(Box::new(issue)))?
                .into_iter()
                .map(|value| wrap(Source::Gather(value))),
        );
        views.extend(
            self.inputs()
                .grain_allowance_declarations(self.constructions())
                .map_err(|issue| DesignGarmentDeclarationError::Constructions(Box::new(issue)))?
                .into_iter()
                .map(|value| wrap(Source::Construction(value))),
        );
        Ok(views)
    }
}
