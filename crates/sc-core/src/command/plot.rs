//! Named plotter artifact inputs; actual profile pen/scale/calibration validation remains with execution.
use crate::ontology::{EntityId, SelectedProfilePin};

/// Single-instance PLT request using the selected profile's actual resolved plotter parameters.
/// Pen-per-function mapping, declared scale and calibration are not copied, guessed or defaulted here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportPlt {
    instance: EntityId,
    selected: SelectedProfilePin,
    parameters: EntityId,
}
impl ExportPlt {
    /// Preserve actual source and selected result claims; no pen map is extracted or file allocated.
    #[must_use]
    pub const fn new(
        instance: EntityId,
        selected: SelectedProfilePin,
        parameters: EntityId,
    ) -> Self {
        Self {
            instance,
            selected,
            parameters,
        }
    }
    /// Explicit immutable source Instance.
    #[must_use]
    pub const fn instance(&self) -> EntityId {
        self.instance
    }
    /// Exact expected profile/schema/content versions for the source/result.
    #[must_use]
    pub const fn selected_profile(&self) -> &SelectedProfilePin {
        &self.selected
    }
    /// Resolved result whose actual plotter bindings/uncertainty execution must inspect.
    #[must_use]
    pub const fn parameters(&self) -> EntityId {
        self.parameters
    }
    /// Registered plotter target identity, not proof of a writer or real-plotter fixture.
    #[must_use]
    pub const fn target_token(&self) -> &'static str {
        "plt"
    }
}
