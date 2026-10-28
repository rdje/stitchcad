//! Explicit per-capture metadata admission before an actual bound source is materialized.
use super::CommandSeamSplitBinding;
use crate::ontology::{
    DeclaredEase, EaseDistribution, EntityId, SeamEase, SeamEasePortion, SeamEaseSource, SeamSpan,
    SeamSpanDefinition, StopLandmark,
};
use core::{fmt, num::NonZeroUsize};

/// Explicit positive logical item limit for one source capture;no default.
/// Count one root,complete original weighted regions and complete original stops. This bounds
/// captured populations,not allocator overhead,bytes,CPU or the aggregate command/snapshot budget.
/// ```compile_fail
/// let limits:sc_core::command::CommandEaseCaptureLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandEaseCaptureLimits {
    items: NonZeroUsize,
}
impl CommandEaseCaptureLimits {
    /// Choose the per-capture logical population allowance explicitly.
    #[must_use]
    pub const fn new(items: NonZeroUsize) -> Self {
        Self { items }
    }
    /// Actual selected root/region/stop item allowance.
    #[must_use]
    pub const fn items(self) -> NonZeroUsize {
        self.items
    }
}
/// Actual original variable collection being admitted after the one logical root item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandEaseCaptureCollection {
    /// Every original density region,without target/child truncation.
    WeightedRegions,
    /// Every original semantic stop,including those absent from the consuming child.
    OriginalStops,
}
/// Concrete per-capture population refusal with actual target/original identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandEaseCaptureError {
    /// Checked aggregate population exceeds the caller's positive allowance.
    Items {
        /// Actual requested graph member.
        target: EntityId,
        /// Actual full original source span identity.
        source_span: EntityId,
        /// Actual original collection.
        collection: CommandEaseCaptureCollection,
        /// Already admitted logical root/collection items.
        consumed: usize,
        /// Complete next original collection population.
        requested: usize,
        /// Actual selected allowance.
        limit: usize,
    },
    /// Aggregate arithmetic cannot be represented;no capture was materialized.
    CountOverflow {
        /// Actual requested graph member.
        target: EntityId,
        /// Actual original source span.
        source_span: EntityId,
        /// Actual original collection.
        collection: CommandEaseCaptureCollection,
        /// Already admitted items.
        consumed: usize,
        /// Complete next collection population.
        requested: usize,
    },
}
impl fmt::Display for CommandEaseCaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command ease capture: {self:?}")
    }
}
impl std::error::Error for CommandEaseCaptureError {}

/// Borrowed actual request/span plus admitted complete original capture cost;no owned population.
/// Admission does not clone regions/stops,resolve topology,read symbolic values or evaluate queries.
/// Copy permits reuse under the same per-capture bound;each materialization still needs its separate
/// aggregate command/snapshot accounting. This token grants no execution/current/physical authority.
/// ```compile_fail
/// use sc_core::command::{CommandEaseCaptureAdmission,CommandEaseCaptureLimits,CommandSeamSplitBinding};
/// fn forge<'a>(binding:CommandSeamSplitBinding<'a>,limits:CommandEaseCaptureLimits)->CommandEaseCaptureAdmission<'a>{
///     CommandEaseCaptureAdmission {binding,item_count:1,limits}
/// }
/// ```
#[derive(Clone, Copy)]
pub struct CommandEaseCaptureAdmission<'a> {
    binding: CommandSeamSplitBinding<'a>,
    item_count: usize,
    limits: CommandEaseCaptureLimits,
}
impl fmt::Debug for CommandEaseCaptureAdmission<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            binding,
            item_count,
            limits,
        } = self;
        f.debug_struct("CommandEaseCaptureAdmission")
            .field("target", &binding.span().id())
            .field("source", &source_view(binding.span()).0)
            .field("item_count", item_count)
            .field("limits", limits)
            .finish()
    }
}
impl<'a> CommandEaseCaptureAdmission<'a> {
    /// Actual bound request and graph member;owners remain borrowed.
    #[must_use]
    pub const fn binding(self) -> CommandSeamSplitBinding<'a> {
        self.binding
    }
    /// Full original source identity;retired original and current target need not be equal.
    #[must_use]
    pub fn source_span(self) -> EntityId {
        source_view(self.binding.span()).0
    }
    /// One logical root plus complete original region/stop populations.
    #[must_use]
    pub const fn item_count(self) -> usize {
        self.item_count
    }
    /// Actual selected per-capture allowance.
    #[must_use]
    pub const fn limits(self) -> CommandEaseCaptureLimits {
        self.limits
    }
    fn charge(
        &mut self,
        collection: CommandEaseCaptureCollection,
        requested: usize,
    ) -> Result<(), CommandEaseCaptureError> {
        let target = self.binding.span().id();
        let source_span = self.source_span();
        let consumed = self.item_count;
        let total =
            consumed
                .checked_add(requested)
                .ok_or(CommandEaseCaptureError::CountOverflow {
                    target,
                    source_span,
                    collection,
                    consumed,
                    requested,
                })?;
        if total > self.limits.items.get() {
            return Err(CommandEaseCaptureError::Items {
                target,
                source_span,
                collection,
                consumed,
                requested,
                limit: self.limits.items.get(),
            });
        }
        self.item_count = total;
        Ok(())
    }
    /// Materialize one admitted logical capture:clone a declared body once,or share the actual
    /// retained root unchanged. Original target intervals remain the captured portion. Immutable
    /// borrows prevent changing counted inputs between admission and this explicit operation.
    #[must_use]
    pub fn capture(self) -> CommandEaseCapture<'a> {
        let Self {
            binding,
            item_count,
            limits,
        } = self;
        CommandEaseCapture {
            binding,
            portion: binding.span().retain_ease_source(),
            item_count,
            limits,
        }
    }
}

/// Materialized admitted source with actual original request/target provenance and target intervals.
/// Owns its captured root for a declaration;retained roots share immutable backing. No derived
/// quantity,semantic-notch context,current readiness,physical correspondence or effect is supplied.
/// ```compile_fail
/// use sc_core::command::{CommandEaseCapture,CommandEaseCaptureLimits,CommandSeamSplitBinding};
/// fn forge<'a>(binding:CommandSeamSplitBinding<'a>,portion:sc_core::ontology::SeamEasePortion,
///     limits:CommandEaseCaptureLimits)->CommandEaseCapture<'a>{
///     CommandEaseCapture {binding,portion,item_count:1,limits}
/// }
/// ```
/// Source borrows stay tied to this owner:
/// ```compile_fail
/// fn escape(capture:sc_core::command::CommandEaseCapture<'_>)->&'static sc_core::ontology::SeamEaseSource {
///     capture.source()
/// }
/// ```
pub struct CommandEaseCapture<'a> {
    binding: CommandSeamSplitBinding<'a>,
    portion: SeamEasePortion,
    item_count: usize,
    limits: CommandEaseCaptureLimits,
}
impl fmt::Debug for CommandEaseCapture<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            binding,
            portion,
            item_count,
            limits,
        } = self;
        f.debug_struct("CommandEaseCapture")
            .field("target", &binding.span().id())
            .field("source", &portion.source().span())
            .field("item_count", item_count)
            .field("limits", limits)
            .finish()
    }
}
impl<'a> CommandEaseCapture<'a> {
    /// Actual original request/target binding,not a same-ID reconstruction.
    #[must_use]
    pub const fn binding(&self) -> CommandSeamSplitBinding<'a> {
        self.binding
    }
    /// Full original source;the borrow cannot outlive this materialized capture.
    #[must_use]
    pub fn source(&self) -> &SeamEaseSource {
        self.portion.source()
    }
    /// Actual target's original source intervals,without child renormalization.
    #[must_use]
    pub const fn portion(&self) -> &SeamEasePortion {
        &self.portion
    }
    /// Admitted per-capture logical source population,including shared retained roots.
    #[must_use]
    pub const fn item_count(&self) -> usize {
        self.item_count
    }
    /// Exact caller-selected limits.
    #[must_use]
    pub const fn limits(&self) -> CommandEaseCaptureLimits {
        self.limits
    }
}
impl<'a> CommandSeamSplitBinding<'a> {
    /// Admit the complete original source before capture clones or shares any owned metadata.
    /// Count one logical root,then full original weighted regions,then full original stops.
    /// Symbolic declarations remain valid metadata. No source collection is cloned or queried.
    /// ```
    /// use sc_core::command::{CommandEaseCapture,CommandEaseCaptureError,CommandEaseCaptureLimits,
    ///     CommandSeamSplitBinding};
    /// fn retain<'a>(binding:&CommandSeamSplitBinding<'a>,limits:CommandEaseCaptureLimits)
    ///     ->Result<CommandEaseCapture<'a>,CommandEaseCaptureError>{
    ///     let admitted=binding.admit_ease_capture(limits)?;
    ///     Ok(admitted.capture())
    /// }
    /// ```
    ///
    /// # Errors
    /// Actual original collection quota/count overflow preserves target/source/consumed/requested
    /// and selected limit. No accepted partial token or capture escapes a refusal.
    pub fn admit_ease_capture(
        &self,
        limits: CommandEaseCaptureLimits,
    ) -> Result<CommandEaseCaptureAdmission<'a>, CommandEaseCaptureError> {
        let (_, declared, stops) = source_view(self.span());
        let DeclaredEase {
            amount,
            distribution,
        } = declared;
        fixed(amount);
        let regions = match distribution {
            EaseDistribution::Uniform => 0,
            EaseDistribution::Weighted { side, regions } => {
                fixed(side);
                fixed_slice(regions);
                regions.len()
            }
            EaseDistribution::BetweenNotches { side, start, end } => {
                fixed(side);
                fixed(start);
                fixed(end);
                0
            }
        };
        let mut admitted = CommandEaseCaptureAdmission {
            binding: *self,
            item_count: 1,
            limits,
        };
        admitted.charge(CommandEaseCaptureCollection::WeightedRegions, regions)?;
        admitted.charge(CommandEaseCaptureCollection::OriginalStops, stops.len())?;
        Ok(admitted)
    }
}
fn fixed<T: Copy>(_: &T) {}
fn fixed_slice<T: Copy>(_: &[T]) {}
fn source_view(span: &SeamSpan) -> (EntityId, &DeclaredEase, &[StopLandmark]) {
    let SeamSpanDefinition {
        id,
        a,
        b,
        direction,
        ease,
        stops,
    } = span.definition();
    fixed(id);
    fixed(a);
    fixed(b);
    fixed(direction);
    fixed_slice(stops);
    match ease {
        SeamEase::Declared(declaration) => (*id, declaration, stops),
        SeamEase::Portion(portion) => {
            let (a, b) = portion.resource_sides();
            fixed(&a);
            fixed(&b);
            let (id, a, b, direction, ease, stops) = portion.source().resource_parts();
            fixed(&a);
            fixed(&b);
            fixed(&direction);
            fixed_slice(stops);
            (id, ease, stops)
        }
    }
}
