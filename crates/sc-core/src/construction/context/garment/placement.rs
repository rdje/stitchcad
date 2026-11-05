//! Original Closure/instance/role-to-notion membership, without current geometry/source queries.
use super::{AdmittedDraftingGarmentSource, DraftingGarmentProvider};
use crate::{
    design::{
        DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
        DesignResourceMeter, DesignResourceSource,
    },
    ontology::{Closure, ClosureInstance, ClosurePlacementRole, EntityId, NotionPlacement},
};
use core::fmt;
/// Exact authored requiring member/role request; numeric positions are never substituted for IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingClosurePlacementRequest {
    /// Actual requiring Closure identity.
    pub closure: EntityId,
    /// Original stable physical instance identity within that Closure.
    pub instance: EntityId,
    /// Original First/Second component role; hardware meaning comes from the Closure kind.
    pub role: ClosurePlacementRole,
}
/// Explicit fresh whole logical content and prospective original matching allowance.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingClosurePlacementLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingClosurePlacementLimits {
    content: DesignResourceLimits,
    inspections: usize,
}
impl DraftingClosurePlacementLimits {
    /// Select complete fresh garment/Root/provider content and (E+8)^2 member work explicitly.
    #[must_use]
    pub const fn new(content: DesignResourceLimits, matching_inspections: usize) -> Self {
        Self {
            content,
            inspections: matching_inspections,
        }
    }
    /// Complete fresh actual logical content allowance.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Maximum conservative original member-inspection bound, not CPU time.
    #[must_use]
    pub const fn matching_inspections(self) -> usize {
        self.inspections
    }
}
/// Complete original dynamic-entry population and checked prospective member matching bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingClosurePlacementShape {
    entries: usize,
    inspections: usize,
}
impl DraftingClosurePlacementShape {
    /// E: all actual garment/Root/provider dynamic entries, including unused members.
    #[must_use]
    pub const fn held_collection_entries(self) -> usize {
        self.entries
    }
    /// Checked (E+8)^2 covering three original linear member scans and fixed role selection.
    #[must_use]
    pub const fn matching_inspection_bound(self) -> usize {
        self.inspections
    }
}
fn bound(entries: usize) -> Option<usize> {
    let padded = entries.checked_add(8)?;
    padded.checked_mul(padded)
}
/// Private association of an actual Closure, its actual instance and exact First/Second role.
/// This is original membership only, not a source/currentness or physical execution certificate.
#[derive(Clone, Copy)]
pub struct DraftingClosurePlacementOwner<'a> {
    closure: &'a Closure,
    closure_index: usize,
    instance: &'a ClosureInstance,
    instance_index: usize,
    role: ClosurePlacementRole,
}
impl<'a> DraftingClosurePlacementOwner<'a> {
    /// Actual original requiring Closure member.
    #[must_use]
    pub const fn closure(self) -> &'a Closure {
        self.closure
    }
    /// Original zero-based Closure inventory position, not a programme position.
    #[must_use]
    pub const fn closure_index(self) -> usize {
        self.closure_index
    }
    /// Actual original instance inside that Closure, without copied placement IDs.
    #[must_use]
    pub const fn instance(self) -> &'a ClosureInstance {
        self.instance
    }
    /// Original zero-based local instance position, not an instance count or programme position.
    #[must_use]
    pub const fn instance_index(self) -> usize {
        self.instance_index
    }
    /// Exact requiring component role.
    #[must_use]
    pub const fn role(self) -> ClosurePlacementRole {
        self.role
    }
    /// Original instance's target identity in the selected role; no notion lookup occurs here.
    #[must_use]
    pub const fn target(self) -> EntityId {
        self.instance.placement(self.role)
    }
}
impl fmt::Debug for DraftingClosurePlacementOwner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingClosurePlacementOwner")
            .field("closure_index", &self.closure_index)
            .field("instance_index", &self.instance_index)
            .field("role", &self.role)
            .finish_non_exhaustive()
    }
}
/// First complete fresh work or original requiring-member/target refusal.
#[derive(Clone, Copy, Debug)]
pub enum DraftingClosurePlacementMemberRefusal<'a> {
    /// First cumulative actual garment/Root/provider field cause.
    Resources(DesignResourceLimitError),
    /// Complete original E+8 or square cannot fit usize before member lookup.
    PopulationOverflow,
    /// Whole conservative member work exceeds explicit policy before any lookup.
    MatchingInspections {
        /// Selected maximum.
        allowed: usize,
        /// Complete checked requirement.
        required: usize,
    },
    /// No actual Closure owns the requested identity.
    MissingClosure,
    /// The actual Closure owns no instance with the requested stable identity.
    MissingInstance {
        /// Original requiring Closure position.
        closure_index: usize,
        /// Actual original requiring Closure loan; other Closures cannot supply the instance.
        closure: &'a Closure,
    },
    /// The original role names no actual notion in this garment.
    MissingNotion {
        /// Actual original requiring Closure/instance/role association, including target identity.
        owner: DraftingClosurePlacementOwner<'a>,
    },
}
/// Exact admitted original parent/request/policy and first typed refusal, without accepted prefix.
pub struct DraftingClosurePlacementMemberError<
    's,
    'a,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    details: Box<Failure<'s, 'a, H, S>>,
}
struct Failure<'s, 'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> {
    parent: &'s AdmittedDraftingGarmentSource<'a, H, S>,
    request: DraftingClosurePlacementRequest,
    limits: DraftingClosurePlacementLimits,
    demand: Option<DesignResourceDemand>,
    shape: Option<DraftingClosurePlacementShape>,
    refusal: DraftingClosurePlacementMemberRefusal<'a>,
}
impl<'s, 'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    DraftingClosurePlacementMemberError<'s, 'a, H, S>
{
    /// Exact actual admitted garment/context/provider/snapshot parent.
    #[must_use]
    pub const fn parent(&self) -> &'s AdmittedDraftingGarmentSource<'a, H, S> {
        self.details.parent
    }
    /// Original requiring Closure/instance/role request, including absent IDs.
    #[must_use]
    pub const fn request(&self) -> DraftingClosurePlacementRequest {
        self.details.request
    }
    /// Explicit fresh content and member matching policy.
    #[must_use]
    pub const fn limits(&self) -> DraftingClosurePlacementLimits {
        self.details.limits
    }
    /// Complete fresh demand only after its whole original walk passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Complete prospective shape only after arithmetic and matching policy passed.
    #[must_use]
    pub const fn shape(&self) -> Option<DraftingClosurePlacementShape> {
        self.details.shape
    }
    /// Original first resource/work/member cause and any actual partial requiring loans.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingClosurePlacementMemberRefusal<'a> {
        &self.details.refusal
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for DraftingClosurePlacementMemberError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingClosurePlacementMemberError")
            .field("request", &self.details.request)
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Display
    for DraftingClosurePlacementMemberError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure placement member: {:?}", self.details.refusal)
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> std::error::Error
    for DraftingClosurePlacementMemberError<'_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingClosurePlacementMemberRefusal::Resources(cause) => Some(cause),
            _ => None,
        }
    }
}
/// Exact original requiring Closure/instance/role and canonical actual notion member.
/// Current copy/Piece/source/anchor/direction and physical hardware geometry remain separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(p:&mut BoundDraftingClosurePlacementMember<'_,'_,H,S>) {p.notion=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn escape<'s,'a,H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(p:BoundDraftingClosurePlacementMember<'s,'a,H,S>)->BoundDraftingClosurePlacementMember<'static,'a,H,S> {p}
/// ```
pub struct BoundDraftingClosurePlacementMember<
    's,
    'a,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    parent: &'s AdmittedDraftingGarmentSource<'a, H, S>,
    owner: DraftingClosurePlacementOwner<'a>,
    notion: &'a NotionPlacement,
    notion_index: usize,
    limits: DraftingClosurePlacementLimits,
    demand: DesignResourceDemand,
    shape: DraftingClosurePlacementShape,
}
impl<'s, 'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    BoundDraftingClosurePlacementMember<'s, 'a, H, S>
{
    /// Exact admitted original garment/context/source parent.
    #[must_use]
    pub const fn parent(&self) -> &'s AdmittedDraftingGarmentSource<'a, H, S> {
        self.parent
    }
    /// Original requiring Closure/instance/role and its original collection indices.
    #[must_use]
    pub const fn owner(&self) -> DraftingClosurePlacementOwner<'a> {
        self.owner
    }
    /// Actual canonical notion member; not a reauthored placement with equal fields.
    #[must_use]
    pub const fn notion(&self) -> &'a NotionPlacement {
        self.notion
    }
    /// Original zero-based notion inventory position, independent of requiring instance position.
    #[must_use]
    pub const fn notion_index(&self) -> usize {
        self.notion_index
    }
    /// Explicit original selected content and member matching allowance.
    #[must_use]
    pub const fn limits(&self) -> DraftingClosurePlacementLimits {
        self.limits
    }
    /// Complete fresh actual garment/Root/provider content demand, without prior credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Complete admitted prospective member shape.
    #[must_use]
    pub const fn shape(&self) -> DraftingClosurePlacementShape {
        self.shape
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingClosurePlacementMember<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingClosurePlacementMember")
            .field("owner", &self.owner)
            .field("notion_index", &self.notion_index)
            .finish_non_exhaustive()
    }
}
impl<'a, H: DesignReferenceHost + DesignResourceSource, S: DraftingGarmentProvider<H> + ?Sized>
    AdmittedDraftingGarmentSource<'a, H, S>
{
    /// Fresh complete garment/Root/provider content and checked (E+8)^2 work before three original
    /// Closure/instance/notion member scans. Roles select only the original instance's stable IDs.
    /// No currentness,copy/Piece/source,value,anchor/direction resolution,clone or successful result
    /// allocation occurs. The original parent demand grants no fresh credit;refusal is boxed.
    /// # Errors
    /// First complete content,arithmetic,matching quota,missing Closure,missing local instance,
    /// then missing canonical notion. Partial requiring loans remain diagnostics only.
    pub fn bind_closure_placement_member<'s>(
        &'s self,
        request: DraftingClosurePlacementRequest,
        limits: DraftingClosurePlacementLimits,
    ) -> Result<
        BoundDraftingClosurePlacementMember<'s, 'a, H, S>,
        DraftingClosurePlacementMemberError<'s, 'a, H, S>,
    > {
        use DraftingClosurePlacementMemberRefusal as R;
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
            let admitted = DraftingClosurePlacementShape {
                entries,
                inspections: required,
            };
            shape = Some(admitted);
            let garment = self.source().garment;
            let (closure_index, closure) = garment
                .closures()
                .closures()
                .iter()
                .enumerate()
                .find(|(_, c)| c.id() == request.closure)
                .ok_or(R::MissingClosure)?;
            let (instance_index, instance) = closure
                .definition()
                .instances
                .iter()
                .enumerate()
                .find(|(_, i)| i.id == request.instance)
                .ok_or(R::MissingInstance {
                    closure_index,
                    closure,
                })?;
            let owner = DraftingClosurePlacementOwner {
                closure,
                closure_index,
                instance,
                instance_index,
                role: request.role,
            };
            let (notion_index, notion) = garment
                .notions()
                .placements()
                .iter()
                .enumerate()
                .find(|(_, n)| n.id() == owner.target())
                .ok_or(R::MissingNotion { owner })?;
            Ok(BoundDraftingClosurePlacementMember {
                parent: self,
                owner,
                notion,
                notion_index,
                limits,
                demand: complete,
                shape: admitted,
            })
        })();
        result.map_err(|refusal| DraftingClosurePlacementMemberError {
            details: Box::new(Failure {
                parent: self,
                request,
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
    fn closure_placement_member_bound_checks_addition_and_square_overflow() {
        assert_eq!(bound(0), Some(64));
        assert_eq!(bound(13), Some(441));
        assert_eq!(bound(usize::MAX), None);
        assert_eq!(bound(usize::MAX - 8), None);
    }
}
