//! Original logical Pocket openings; declaration borrowing supplies no physical opening or scope.
use super::{DesignInputs, DesignInputsError, DesignReferenceHost};
use crate::{
    ontology::{
        Pocket, PocketInventory, PocketOpening, PocketOpeningValidation, ProfileBindingValidation,
    },
    value::CanonicalInputs,
};
use core::fmt;

/// Actual original Pocket/opening and current canonical owner, without copied source or value state.
/// ```compile_fail
/// fn escape<H: sc_core::design::DesignReferenceHost>(inputs: sc_core::design::DesignInputs<H>,
///     pockets: sc_core::ontology::PocketInventory)
///     -> Vec<sc_core::design::DesignPocketOpeningDeclaration<'static>> {
///     inputs.pocket_opening_declarations(&pockets).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct DesignPocketOpeningDeclaration<'a> {
    pocket: &'a Pocket,
    index: usize,
    canonical: &'a CanonicalInputs,
}
impl fmt::Debug for DesignPocketOpeningDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignPocketOpeningDeclaration")
            .field(
                "source",
                &match self.opening() {
                    PocketOpening::Declaration(_) => "declaration",
                    PocketOpening::Profile(_) => "profile",
                },
            )
            .finish_non_exhaustive()
    }
}
impl<'a> DesignPocketOpeningDeclaration<'a> {
    /// Exact original immutable Pocket, without a current-placement or component certificate.
    #[must_use]
    pub const fn pocket(self) -> &'a Pocket {
        self.pocket
    }
    /// Original zero-based authored Pocket position, without sorting ids.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Original logical recipe/Design or Profile opening declaration, without scalar conversion.
    #[must_use]
    pub const fn opening(self) -> &'a PocketOpening {
        &self.pocket.definition().opening
    }
    /// Actual whole current input root's single canonical owner.
    #[must_use]
    pub const fn canonical_inputs(self) -> &'a CanonicalInputs {
        self.canonical
    }
    /// Original G3 opening resolution/executable-scope obligation, independent of source placement.
    #[must_use]
    pub const fn opening_validation(self) -> PocketOpeningValidation {
        self.pocket.opening_validation()
    }
    /// Original Profile deferral; None means only no Profile opening field was authored.
    #[must_use]
    pub const fn profile_validation(self) -> Option<ProfileBindingValidation> {
        self.pocket.profile_binding_validation()
    }
}
impl<H: DesignReferenceHost> DesignInputs<H> {
    /// Borrow every actual Pocket's original opening after complete current input-root validation.
    ///
    /// Authored Pocket order, original objects/fields and sole canonical owner stay borrowed. Logical
    /// recipe/Design and Profile declarations keep their original domain/state owners and G3/G4
    /// deferrals, even when ids match numeric declarations. No scalar lookup, opening vocabulary,
    /// operation execution, value/default, placement/component-source or release proof is supplied.
    /// # Errors
    /// Returns the original whole current input-root cause, even for empty Pocket scope. No partial
    /// view collection escapes a refusal.
    pub fn pocket_opening_declarations<'a>(
        &'a self,
        pockets: &'a PocketInventory,
    ) -> Result<Vec<DesignPocketOpeningDeclaration<'a>>, DesignInputsError<H::Error>> {
        self.validate_current()?;
        Ok(pockets
            .pockets()
            .iter()
            .enumerate()
            .map(|(index, pocket)| DesignPocketOpeningDeclaration {
                pocket,
                index,
                canonical: self.inputs(),
            })
            .collect())
    }
}
