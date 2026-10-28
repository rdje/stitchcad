//! Explicit logical dynamic-content accounting, independent of authorization and publication.
use crate::ontology::EntityId;
use core::{
    fmt,
    num::{NonZeroU64, NonZeroUsize},
};

mod canonical;
mod constructions;
mod garment;
mod inputs;
mod layers;
mod pieces;
mod sewing;
mod topology;
pub use canonical::charge_canonical_input_resources;
pub use constructions::charge_construction_inventory_resources;
pub use garment::{
    charge_closure_inventory_resources, charge_notion_placement_inventory_resources,
    charge_pocket_inventory_resources,
};
pub use inputs::{
    charge_material_inventory_resources, charge_parameter_resources,
    charge_recipe_document_resources,
};
pub use layers::{charge_hem_inventory_resources, charge_layer_inventory_resources};
pub use pieces::{charge_cut_plan_resources, charge_piece_inventory_resources};
pub use sewing::{charge_sewing_graph_resources, charge_sewing_landmark_resources};
pub use topology::charge_topology_resources;

// Fixed value/reference payloads own no dynamic content. Removing Copy requires new accounting.
fn fixed_value<T: Copy>(_: &T) {}
fn fixed_slice<T: Copy>(_: &[T]) {}

/// Original top-level model scope of a resource charge; no model data or authority is carried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignResourceScope {
    /// Structural/canonical/Piece/topology content.
    Structure,
    /// Parameter/recipe/Material and supplied-reference content.
    Inputs,
    /// Sewing graph and semantic landmarks.
    Sewing,
    /// Garment composition/closure/layer/construction content.
    Garment,
    /// Actual immutable higher metadata source host.
    SourceHost,
}
/// Producer-declared static field path and original authored position, without copied private text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesignResourceLocation {
    /// Original model scope.
    pub scope: DesignResourceScope,
    /// Actual owning entity for nested fields when one exists; identity is data, not authority.
    pub owner: Option<EntityId>,
    /// Static field-path token chosen by the schema producer, never a copied user label/value.
    pub field: &'static str,
    /// Original zero-based authored collection position when this charge is member-specific.
    pub index: Option<usize>,
}
/// Platform-sized logical resource counts, separate from unbounded-counter bit counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignResourceCount {
    /// Entries in owned dynamic collections, including nested collections and reference vectors.
    CollectionEntries,
    /// Exact UTF-8 bytes of logical owned text fields, without shared-backing deduplication.
    TextBytes,
}
/// Explicit positive logical-content maxima; no default, allocator-size or CPU-time estimate.
/// ```compile_fail
/// use sc_core::design::DesignResourceLimits;
/// use core::num::NonZeroUsize;
/// fn widen(limits: &mut DesignResourceLimits) { limits.collection_entries = NonZeroUsize::MAX; }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesignResourceLimits {
    collection_entries: NonZeroUsize,
    text_bytes: NonZeroUsize,
    counter_bits: NonZeroU64,
}
impl DesignResourceLimits {
    /// Preserve all three host-chosen finite positive maxima, without inferring a platform policy.
    #[must_use]
    pub const fn new(
        collection_entries: NonZeroUsize,
        text_bytes: NonZeroUsize,
        counter_bits: NonZeroU64,
    ) -> Self {
        Self {
            collection_entries,
            text_bytes,
            counter_bits,
        }
    }
    /// Maximum cumulative entries across logical owned dynamic collections.
    #[must_use]
    pub const fn collection_entries(self) -> NonZeroUsize {
        self.collection_entries
    }
    /// Maximum cumulative logical UTF-8 field bytes.
    #[must_use]
    pub const fn text_bytes(self) -> NonZeroUsize {
        self.text_bytes
    }
    /// Maximum cumulative bits in logical unbounded counter magnitudes.
    #[must_use]
    pub const fn counter_bits(self) -> NonZeroU64 {
        self.counter_bits
    }
}
/// Successfully charged logical demand; constructing/finishing a meter proves no actual model walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesignResourceDemand {
    collection_entries: usize,
    text_bytes: usize,
    counter_bits: u64,
}
impl DesignResourceDemand {
    /// Cumulative successfully charged owned collection entries.
    #[must_use]
    pub const fn collection_entries(self) -> usize {
        self.collection_entries
    }
    /// Cumulative exact successfully charged logical text bytes.
    #[must_use]
    pub const fn text_bytes(self) -> usize {
        self.text_bytes
    }
    /// Cumulative successfully charged unbounded-counter bits.
    #[must_use]
    pub const fn counter_bits(self) -> u64 {
        self.counter_bits
    }
}
/// Exact original first excess; components remain valid even when their sum would overflow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignResourceLimitError {
    /// A logical collection/text charge exceeds remaining platform-sized room.
    Count {
        /// Exact count unit.
        unit: DesignResourceCount,
        /// Explicit configured maximum.
        maximum: usize,
        /// Exact successfully charged preceding count in this unit.
        consumed: usize,
        /// Exact original attempted charge, without clipping or addition.
        requested: usize,
        /// Original producer scope/field/member.
        location: DesignResourceLocation,
    },
    /// An unbounded-counter charge exceeds remaining bit room.
    CounterBits {
        /// Explicit configured maximum cumulative bit count.
        maximum: u64,
        /// Exact successfully charged preceding bit count.
        consumed: u64,
        /// Exact original attempted bit count.
        requested: u64,
        /// Original producer scope/field/member.
        location: DesignResourceLocation,
    },
}
impl fmt::Display for DesignResourceLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Design resource limit: {self:?}")
    }
}
impl std::error::Error for DesignResourceLimitError {}

mod sealed {
    pub trait Sealed {}
}

/// Restricted charging capability: a producer receives this trait object, never replaceable meter state.
/// Implementations are sealed; there is no policy setter, reset, model replacement or approval method.
/// ```compile_fail
/// use sc_core::design::{DesignResourceSink, DesignResourceLocation, DesignResourceLimitError};
/// struct ForeignSink;
/// impl DesignResourceSink for ForeignSink {
///     fn charge_entries(&mut self, _: usize, _: DesignResourceLocation) -> Result<(), DesignResourceLimitError> { Ok(()) }
///     fn charge_text(&mut self, _: &str, _: DesignResourceLocation) -> Result<(), DesignResourceLimitError> { Ok(()) }
///     fn charge_counter_bits(&mut self, _: u64, _: DesignResourceLocation) -> Result<(), DesignResourceLimitError> { Ok(()) }
/// }
/// ```
pub trait DesignResourceSink: sealed::Sealed {
    /// Charge a collection's exact length before traversing its nested records.
    /// # Errors
    /// Returns the original first excess, including an earlier ignored refusal.
    fn charge_entries(
        &mut self,
        entries: usize,
        location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError>;
    /// Charge exact UTF-8 bytes without copying/normalizing text or retaining a model borrow.
    /// # Errors
    /// Returns the original first excess, including an earlier ignored refusal.
    fn charge_text(
        &mut self,
        text: &str,
        location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError>;
    /// Charge an unbounded counter's actual magnitude bits, without serializing/cloning that counter.
    /// # Errors
    /// Returns the original first excess, including an earlier ignored refusal.
    fn charge_counter_bits(
        &mut self,
        bits: u64,
        location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError>;
}
/// First-failure-latching logical-content meter. It allocates/copies no model, collection or text.
/// Actual exhaustive producers and runtime enforcement remain separate implementation obligations.
/// ```compile_fail
/// use sc_core::design::{DesignResourceMeter, DesignResourceLimits};
/// fn replace_policy(meter: &mut DesignResourceMeter, limits: DesignResourceLimits) { meter.limits = limits; }
/// ```
/// ```compile_fail
/// fn finish_producer(sink: &mut dyn sc_core::design::DesignResourceSink) { let _ = sink.finish(); }
/// ```
#[derive(Debug)]
pub struct DesignResourceMeter {
    limits: DesignResourceLimits,
    demand: DesignResourceDemand,
    first_failure: Option<DesignResourceLimitError>,
}
impl DesignResourceMeter {
    /// Start an empty meter under explicit policy. This creates no model/authority capability.
    #[must_use]
    pub const fn new(limits: DesignResourceLimits) -> Self {
        Self {
            limits,
            demand: DesignResourceDemand {
                collection_entries: 0,
                text_bytes: 0,
                counter_bits: 0,
            },
            first_failure: None,
        }
    }
    /// Consume the meter and return charged demand only if every attempted charge was admitted.
    /// # Errors
    /// Returns the exact first refusal even if a producer ignored that error and attempted more work.
    pub fn finish(self) -> Result<DesignResourceDemand, DesignResourceLimitError> {
        match self.first_failure {
            Some(issue) => Err(issue),
            None => Ok(self.demand),
        }
    }
    fn charge_count(
        &mut self,
        unit: DesignResourceCount,
        requested: usize,
        location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError> {
        if let Some(issue) = self.first_failure {
            return Err(issue);
        }
        let (maximum, consumed) = match unit {
            DesignResourceCount::CollectionEntries => (
                self.limits.collection_entries.get(),
                self.demand.collection_entries,
            ),
            DesignResourceCount::TextBytes => {
                (self.limits.text_bytes.get(), self.demand.text_bytes)
            }
        };
        if requested > maximum - consumed {
            let issue = DesignResourceLimitError::Count {
                unit,
                maximum,
                consumed,
                requested,
                location,
            };
            self.first_failure = Some(issue);
            return Err(issue);
        }
        match unit {
            DesignResourceCount::CollectionEntries => self.demand.collection_entries += requested,
            DesignResourceCount::TextBytes => self.demand.text_bytes += requested,
        }
        Ok(())
    }
}
impl sealed::Sealed for DesignResourceMeter {}
impl DesignResourceSink for DesignResourceMeter {
    fn charge_entries(
        &mut self,
        entries: usize,
        location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError> {
        self.charge_count(DesignResourceCount::CollectionEntries, entries, location)
    }
    fn charge_text(
        &mut self,
        text: &str,
        location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError> {
        self.charge_count(DesignResourceCount::TextBytes, text.len(), location)
    }
    fn charge_counter_bits(
        &mut self,
        requested: u64,
        location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError> {
        if let Some(issue) = self.first_failure {
            return Err(issue);
        }
        let maximum = self.limits.counter_bits.get();
        let consumed = self.demand.counter_bits;
        if requested > maximum - consumed {
            let issue = DesignResourceLimitError::CounterBits {
                maximum,
                consumed,
                requested,
                location,
            };
            self.first_failure = Some(issue);
            return Err(issue);
        }
        self.demand.counter_bits += requested;
        Ok(())
    }
}
