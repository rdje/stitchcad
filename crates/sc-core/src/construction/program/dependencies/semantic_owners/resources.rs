//! Fresh original whole source content, requests and held proof records; no aggregate work grant.
use super::DraftingProgramSemanticInputs;
use crate::{
    construction::{DraftingGarmentProvider, DraftingInitialInputProvider},
    design::{
        DesignReferenceHost, DesignResourceLimitError, DesignResourceScope, DesignResourceSink,
        DesignResourceSource,
    },
};
impl<
        's,
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignReferenceHost + DesignResourceSource,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Freshly append the actual whole garment/Root/provider,raw/normalized programme records,
    /// extra names,all original selection rows/pins and complete held proof records to one sink.
    /// Root/provider accounting runs once through the admitted garment's original source owner;
    /// the record-only programme component never reenters it. All unused/repeated original fields
    /// count. Previous admissions,constructor shapes,limits and stored demands supply no credit.
    ///
    /// Only the required exhaustive provider resource walk is called. No garment/source/currentness/
    /// member/value query,source-selection match,clone,normalization or result allocation occurs.
    /// This cumulative content producer leaves the meter unfinished;aggregate shape,comparison,
    /// child-walk/result policies and actual source coverage remain separate admission stages.
    /// ```
    /// use sc_core::{construction::*,design::*};
    /// fn whole_content<H:DesignReferenceHost+DesignResourceSource,
    ///     S:DraftingInitialInputProvider<H>+DraftingGarmentProvider<H>+?Sized>(
    ///     inputs:&DraftingProgramSemanticInputs<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     sink:&mut dyn DesignResourceSink)->Result<(),DesignResourceLimitError> {
    ///     inputs.charge_resources(sink)
    /// }
    /// ```
    /// # Errors
    /// Original first garment/provider,programme/name,selection or proof field refusal,including
    /// any earlier or ignored provider sink failure. No accepted partial demand is returned.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            geometry,
            garment,
            selections,
        } = self;
        garment.charge_resources(sink)?;
        let names = geometry
            .work()
            .inputs()
            .inputs()
            .order()
            .inputs()
            .roles()
            .positions()
            .work()
            .inputs();
        names.program().charge_record_resources(sink)?;
        names.charge_extra_resources(sink)?;
        let scope = DesignResourceScope::CommandState;
        let owner = Some(garment.source().garment.inputs().id());
        selections.charge_record_resources(sink, scope, owner)?;
        geometry.charge_retained_resources(sink, scope, owner)
    }
}
