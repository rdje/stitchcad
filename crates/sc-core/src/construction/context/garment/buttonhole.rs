//! Original Closure-to-Root Buttonhole relation after fresh complete content/work admission.
use super::{AdmittedDraftingGarmentSource, DraftingGarmentProvider};
use crate::{
    design::{
        DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
        DesignResourceMeter, DesignResourceSource,
    },
    ontology::{ButtonholeLengthSource, Closure, EntityId},
    recipe::{
        SemanticOperation, SemanticOperationBinding, SemanticOperationKind,
        SemanticOperationLookupError,
    },
};
use core::fmt;

/// Explicit fresh whole garment/provider content and prospective matching allowance.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingClosureButtonholeLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingClosureButtonholeLimits {
    content: DesignResourceLimits,
    inspections: usize,
}
impl DraftingClosureButtonholeLimits {
    /// Select complete logical content and conservative (E+8)^2 matching inspections explicitly.
    #[must_use]
    pub const fn new(content: DesignResourceLimits, matching_inspections: usize) -> Self {
        Self {
            content,
            inspections: matching_inspections,
        }
    }
    /// Complete fresh actual garment/Root/provider logical content allowance.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Maximum conservative original Closure/member inspection bound; not CPU time.
    #[must_use]
    pub const fn matching_inspections(self) -> usize {
        self.inspections
    }
}
/// Complete actual dynamic-entry population and prospective original matching bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingClosureButtonholeShape {
    entries: usize,
    inspections: usize,
}
impl DraftingClosureButtonholeShape {
    /// E: complete original garment/Root/provider dynamic collection entries.
    #[must_use]
    pub const fn held_collection_entries(self) -> usize {
        self.entries
    }
    /// Checked (E+8)^2, covering two linear member scans and one fixed origin comparison.
    #[must_use]
    pub const fn matching_inspection_bound(self) -> usize {
        self.inspections
    }
}
fn bound(entries: usize) -> Option<usize> {
    let padded = entries.checked_add(8)?;
    padded.checked_mul(padded)
}
/// First complete content/work or original Closure/operation/canonical-button refusal.
#[derive(Clone, Copy, Debug)]
pub enum DraftingClosureButtonholeRefusal<'a> {
    /// First fresh cumulative actual garment/Root/provider content cause.
    Resources(DesignResourceLimitError),
    /// Prospective matching arithmetic cannot fit usize, before any member lookup.
    PopulationOverflow,
    /// Complete original matching requirement exceeds explicit policy, before lookup.
    MatchingInspections {
        /// Selected maximum.
        allowed: usize,
        /// Checked whole original requirement.
        required: usize,
    },
    /// No actual Closure has the requested identity; no same-kind substitution is attempted.
    MissingClosure,
    /// The actual original Closure is another kind; no derivation is invented.
    NotButtonhole {
        /// Original zero-based Closure inventory position.
        index: usize,
        /// Actual original member, without a copied definition.
        closure: &'a Closure,
    },
    /// Original Root lookup cause, preserving actual wrong-family member when present.
    Operation(SemanticOperationLookupError<'a>),
    /// Actual typed operation names a different button origin or representation.
    ButtonOrigin {
        /// Canonical original Closure size/derivation view, without authored hole length.
        source: ButtonholeLengthSource<'a>,
        /// Actual original Root operation, retaining its different original button source.
        operation: SemanticOperationBinding<'a>,
    },
}
/// Exact admitted original parent, request/policy and first typed refusal; no accepted prefix.
pub struct DraftingClosureButtonholeError<
    's,
    'a,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    details: Box<Failure<'s, 'a, H, S>>,
}
struct Failure<'s, 'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> {
    parent: &'s AdmittedDraftingGarmentSource<'a, H, S>,
    closure: EntityId,
    limits: DraftingClosureButtonholeLimits,
    demand: Option<DesignResourceDemand>,
    shape: Option<DraftingClosureButtonholeShape>,
    refusal: DraftingClosureButtonholeRefusal<'a>,
}
impl<'s, 'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    DraftingClosureButtonholeError<'s, 'a, H, S>
{
    /// Exact original Root-bound garment/context/provider/snapshot admission.
    #[must_use]
    pub const fn parent(&self) -> &'s AdmittedDraftingGarmentSource<'a, H, S> {
        self.details.parent
    }
    /// Original requested Closure identity, including an absent target.
    #[must_use]
    pub const fn closure(&self) -> EntityId {
        self.details.closure
    }
    /// Explicit fresh content and matching policy.
    #[must_use]
    pub const fn limits(&self) -> DraftingClosureButtonholeLimits {
        self.details.limits
    }
    /// Complete fresh demand only after the entire original resource walk passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Complete admitted prospective shape only after arithmetic and matching quota passed.
    #[must_use]
    pub const fn shape(&self) -> Option<DraftingClosureButtonholeShape> {
        self.details.shape
    }
    /// First original typed cause; borrowed operation/member witnesses remain inspectable.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingClosureButtonholeRefusal<'a> {
        &self.details.refusal
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for DraftingClosureButtonholeError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingClosureButtonholeError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Display
    for DraftingClosureButtonholeError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure Buttonhole relation: {:?}", self.details.refusal)
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> std::error::Error
    for DraftingClosureButtonholeError<'_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingClosureButtonholeRefusal::Resources(cause) => Some(cause),
            // Operation lookup errors retain original loans and cannot become a 'static cause.
            _ => None,
        }
    }
}
/// Actual original Closure and typed Root operation with exactly the canonical button origin.
/// No value, logical domain, policy signature, mixed order, current placement or length is proved.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(
///     p:&mut BoundDraftingClosureButtonhole<'_,'_,H,S>) {p.operation=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn escape<'s,'a,H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(
///     p:BoundDraftingClosureButtonhole<'s,'a,H,S>)->BoundDraftingClosureButtonhole<'static,'a,H,S> {p}
/// ```
pub struct BoundDraftingClosureButtonhole<
    's,
    'a,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    parent: &'s AdmittedDraftingGarmentSource<'a, H, S>,
    closure: &'a Closure,
    closure_index: usize,
    source: ButtonholeLengthSource<'a>,
    operation: SemanticOperationBinding<'a>,
    limits: DraftingClosureButtonholeLimits,
    demand: DesignResourceDemand,
    shape: DraftingClosureButtonholeShape,
}
impl<'s, 'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    BoundDraftingClosureButtonhole<'s, 'a, H, S>
{
    /// Exact original admitted garment parent; no rebuilt context or equal-ID substitute.
    #[must_use]
    pub const fn parent(&self) -> &'s AdmittedDraftingGarmentSource<'a, H, S> {
        self.parent
    }
    /// Actual original Closure member, without copied fields or inferred hardware.
    #[must_use]
    pub const fn closure(&self) -> &'a Closure {
        self.closure
    }
    /// Original zero-based garment Closure inventory position, not a program position.
    #[must_use]
    pub const fn closure_index(&self) -> usize {
        self.closure_index
    }
    /// One original canonical button/derivation source; physical validation stays deferred to G3.
    #[must_use]
    pub const fn source(&self) -> ButtonholeLengthSource<'a> {
        self.source
    }
    /// Exact original Root semantic recipe/member/index, not a mixed-program ordering certificate.
    #[must_use]
    pub const fn operation(&self) -> SemanticOperationBinding<'a> {
        self.operation
    }
    /// Original explicit fresh content/matching policy.
    #[must_use]
    pub const fn limits(&self) -> DraftingClosureButtonholeLimits {
        self.limits
    }
    /// Entire fresh actual garment/Root/provider content, with no prior admission credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Complete admitted prospective matching shape.
    #[must_use]
    pub const fn shape(&self) -> DraftingClosureButtonholeShape {
        self.shape
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingClosureButtonhole<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingClosureButtonhole")
            .field("closure_index", &self.closure_index)
            .field("operation", &self.operation)
            .finish_non_exhaustive()
    }
}
impl<'a, H: DesignReferenceHost + DesignResourceSource, S: DraftingGarmentProvider<H> + ?Sized>
    AdmittedDraftingGarmentSource<'a, H, S>
{
    /// Freshly charge the entire actual garment/Root/provider, then admit checked (E+8)^2 before
    /// matching an original Closure and its exact Root Buttonhole operation. Compare complete
    /// NotionSize origin representations;equal numeric IDs in Declaration and Profile differ.
    /// Two linear member scans plus one fixed origin comparison fit the conservative bound.
    /// No source/currentness/value callback, clone or successful result allocation occurs.
    /// A refused diagnostic is boxed. Prior stored demands/limits grant no fresh content credit.
    /// # Errors
    /// First complete content, arithmetic, matching quota, missing/nonbutton Closure, original
    /// typed Root operation lookup, then different canonical button origin. No prefix escapes.
    pub fn bind_closure_buttonhole<'s>(
        &'s self,
        closure: EntityId,
        limits: DraftingClosureButtonholeLimits,
    ) -> Result<
        BoundDraftingClosureButtonhole<'s, 'a, H, S>,
        DraftingClosureButtonholeError<'s, 'a, H, S>,
    > {
        use DraftingClosureButtonholeRefusal as R;
        let mut demand = None;
        let mut shape = None;
        let result = (|| {
            let mut meter = DesignResourceMeter::new(limits.content());
            let charged = self.charge_resources(&mut meter);
            let complete = meter.finish().map_err(R::Resources)?;
            charged.map_err(R::Resources)?;
            demand = Some(complete);
            let entries = complete.collection_entries();
            let required = bound(entries).ok_or(R::PopulationOverflow)?;
            if required > limits.matching_inspections() {
                return Err(R::MatchingInspections {
                    allowed: limits.matching_inspections(),
                    required,
                });
            }
            let admitted = DraftingClosureButtonholeShape {
                entries,
                inspections: required,
            };
            shape = Some(admitted);
            let (closure_index, member) = self
                .source()
                .garment
                .closures()
                .closures()
                .iter()
                .enumerate()
                .find(|(_, c)| c.id() == closure)
                .ok_or(R::MissingClosure)?;
            let source = member.buttonhole_length_source().ok_or(R::NotButtonhole {
                index: closure_index,
                closure: member,
            })?;
            let operation = self
                .context()
                .scope()
                .root()
                .semantic_recipe()
                .resolve(
                    source.derivation().operation,
                    SemanticOperationKind::Buttonhole,
                )
                .map_err(R::Operation)?;
            match operation.operation() {
                SemanticOperation::Buttonhole(buttonhole)
                    if &buttonhole.definition().button == source.button_size() => {}
                SemanticOperation::Buttonhole(_) | SemanticOperation::LogicalDerivation(_) => {
                    return Err(R::ButtonOrigin { source, operation });
                }
            }
            Ok(BoundDraftingClosureButtonhole {
                parent: self,
                closure: member,
                closure_index,
                source,
                operation,
                limits,
                demand: complete,
                shape: admitted,
            })
        })();
        result.map_err(|refusal| DraftingClosureButtonholeError {
            details: Box::new(Failure {
                parent: self,
                closure,
                limits,
                demand,
                shape,
                refusal,
            }),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::bound;
    #[test]
    fn closure_buttonhole_matching_bound_checks_both_arithmetic_boundaries() {
        assert_eq!(bound(0), Some(64));
        assert_eq!(bound(13), Some(441));
        assert_eq!(bound(usize::MAX), None);
        assert_eq!(bound(usize::MAX - 8), None);
    }
}
