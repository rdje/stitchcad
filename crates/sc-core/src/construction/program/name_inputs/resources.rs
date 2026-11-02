//! Extra logical name inputs; complete program/normalized-owner accounting composes separately.
use super::{DraftingGeometryNameSource as G, DraftingOutputName, DraftingProgramNameInputs};
use crate::{
    construction::{
        edge_output_roles::OutputRoleResourceKind, operation_input::resources::fixed,
        DraftingContextSource, DraftingOperationInput as I,
    },
    design::{
        charge_length_declaration_resources, DesignMetadataHost, DesignResourceLimitError,
        DesignResourceLocation, DesignResourceScope, DesignResourceSink, DesignResourceSource,
    },
    ontology::EntityId,
    recipe::{FormulaDeclaration, FormulaDeclarationSource as S},
};
fn location(owner: Option<EntityId>, field: &'static str, index: usize) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner,
        field,
        index: Some(index),
    }
}
impl<H: DesignResourceSource, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramNameInputs<'_, '_, '_, H, S>
{
    /// Charge the complete actual containing normalized program,then all additional name fields
    /// into the same cumulative sink. Both exhaustive producers run freshly;prior demands and
    /// conservative bounds supply no credit. Repeated source occurrences count in both components.
    /// No namespace allocation,source validation,position association or value query occurs.
    /// This is held-content accounting;prospective name work and whole admission remain separate.
    /// # Errors
    /// Original first raw/normalized field refusal precedes additional-name fields;the sink is
    /// never reset and an earlier failure survives even an empty program and empty name vectors.
    ///
    /// ```
    /// use sc_core::{construction::{DraftingContextSource, DraftingProgramNameInputs},
    ///     design::{DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
    ///     DesignResourceMeter, DesignResourceSource}};
    /// fn complete_name_demand<H:DesignResourceSource,S:DraftingContextSource<H> + ?Sized>(
    ///     names:&DraftingProgramNameInputs<'_, '_, '_,H,S>,policy:DesignResourceLimits)
    ///     -> Result<DesignResourceDemand,DesignResourceLimitError> {
    ///     let mut meter=DesignResourceMeter::new(policy);
    ///     names.charge_resources(&mut meter)?;
    ///     meter.finish()
    /// }
    /// ```
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        self.program.charge_resources(sink)?;
        self.charge_extra_resources(sink)
    }
}
fn initial(
    declaration: FormulaDeclaration<'_>,
    index: usize,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (name, source) = declaration.resource_parts();
    let owner = match source {
        S::Input { input, .. } | S::LengthInput { input, .. } => Some(input),
        S::Point(p) => Some(p.creator()),
        S::Edge(e) => Some(e.creator()),
        S::Recipe { .. } | S::Reserved(_) => None,
    };
    sink.charge_text(name, location(owner, "drafting.names.initial.name", index))?;
    match source {
        S::Input {
            origin,
            input,
            declaration,
            kind,
        } => {
            fixed(&origin);
            fixed(&input);
            fixed(&declaration);
            fixed(&kind);
        }
        S::LengthInput {
            origin,
            input,
            declaration,
        } => {
            fixed(&origin);
            fixed(&input);
            charge_length_declaration_resources(
                declaration,
                location(
                    Some(declaration.id()),
                    "drafting.names.initial.length.evidence",
                    index,
                ),
                sink,
            )?;
        }
        S::Point(p) => fixed(&p),
        S::Edge(e) => fixed(&e),
        S::Recipe {
            statement_index,
            kind,
            span,
            name_span,
        } => {
            fixed(&statement_index);
            fixed(&kind);
            fixed(&span);
            fixed(&name_span);
        }
        S::Reserved(r) => fixed(&r),
    }
    Ok(())
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramNameInputs<'_, '_, '_, H, S>
{
    /// Charge complete extra initial/named-output input fields into the cumulative restricted sink.
    /// This component includes canonical Length evidence and actual fixed-schema documents,or
    /// whole typed role-catalogue definitions/active lists/retained indexes and key text. Every
    /// logical occurrence counts,including repeated source loans;no address/id deduplication.
    ///
    /// It borrows actual fields without cloning,namespace/source lookup,values or prior-work totals.
    /// The complete containing program and normalized arenas compose separately;this component
    /// grants no whole content,association,source truth,namespace or executable-program admission.
    /// # Errors
    /// Original first sink excess with actual metadata/operation field and collection-specific
    /// authored index. Fixed native source roles retain their existing original field locations.
    /// Empty components also propagate an earlier latched refusal.
    /// ```
    /// use sc_core::{construction::{DraftingContextSource, DraftingProgramNameInputs},
    ///     design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError,
    ///     DesignResourceLimits, DesignResourceMeter}};
    /// fn extra_demand<H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    ///     names:&DraftingProgramNameInputs<'_, '_, '_,H,S>,policy:DesignResourceLimits)
    ///     -> Result<DesignResourceDemand,DesignResourceLimitError> {
    ///     let mut meter=DesignResourceMeter::new(policy);
    ///     names.charge_extra_resources(&mut meter)?;
    ///     meter.finish()
    /// }
    /// ```
    pub fn charge_extra_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            program: _,
            initial: initials,
            outputs,
        } = self;
        sink.charge_entries(
            initials.len(),
            DesignResourceLocation {
                scope: DesignResourceScope::CommandState,
                owner: None,
                field: "drafting.names.initial",
                index: None,
            },
        )?;
        for (index, entry) in initials.iter().enumerate() {
            initial(entry.declaration(), index, sink)?;
        }
        sink.charge_entries(
            outputs.len(),
            DesignResourceLocation {
                scope: DesignResourceScope::CommandState,
                owner: None,
                field: "drafting.names.outputs",
                index: None,
            },
        )?;
        for (index, output) in outputs.iter().enumerate() {
            let DraftingOutputName {
                position,
                name,
                source,
            } = output;
            fixed(position);
            let owner = match source.reference() {
                crate::construction::DraftingGeometryReference::Point(p) => p.creator(),
                crate::construction::DraftingGeometryReference::Edge(e) => e.creator(),
            };
            sink.charge_text(
                name.resource_text(),
                location(Some(owner), "drafting.names.outputs.name", index),
            )?;
            match *source {
                G::Point(p) => I::ConstructPoint(p).charge_resources(sink)?,
                G::Line(l) => I::DrawLine(l).charge_resources(sink)?,
                G::CenterArc(a) => I::DrawArc(a).charge_resources(sink)?,
                G::Bezier(b) => I::DrawBezier(b).charge_resources(sink)?,
                G::PointRole(role) => role.catalog().charge_retained_resources(index, sink)?,
                G::EdgeRole(role) => role.catalog().charge_retained_resources(
                    OutputRoleResourceKind::Edge,
                    index,
                    sink,
                )?,
            }
        }
        Ok(())
    }
}
