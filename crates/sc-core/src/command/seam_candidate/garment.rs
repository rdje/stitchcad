//! Complete actual ownership and payload admission; held repair inspection stays possible.
use super::{charge_graph_reservation, CommandSeamSplitGraphAdmission, CommandSeamSplitGraphError};
use crate::{
    design::{
        charge_design_garment_resources, DesignGarment, DesignGarmentDefinition,
        DesignGarmentError, DesignGarmentPosition, DesignResourceDemand, DesignResourceLimitError,
        DesignResourceLimits, DesignResourceMeter, DesignResourceSource, DesignSewing,
        DesignSewingDefinition, DesignSewingError,
    },
    ontology::EntityId,
};
use core::fmt;

/// Actual owner, complete envelope/current-source, global namespace or clone/output refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandSeamSplitGarmentError<E> {
    /// Another garment cannot substitute its equal sewing/source body.
    Owner {
        /// Design owned by the actual complete source admission.
        expected_design: EntityId,
        /// Design owned by the supplied garment.
        provided_design: EntityId,
    },
    /// Original complete garment/graph reservation or final output refusal.
    Resources(DesignResourceLimitError),
    /// Complete original current garment refusal, before a namespace census or component clone.
    Current(Box<DesignGarmentError<E>>),
    /// A tentative child is already owned anywhere in the original garment.
    ChildOwned {
        /// Actual first/second A-ordered child slot.
        child: usize,
        /// Exact refused proposed identity.
        identity: EntityId,
        /// Original complete canonical owner position.
        owner: DesignGarmentPosition,
    },
    /// Original prepared graph construction refusal.
    Graph(Box<CommandSeamSplitGraphError>),
    /// Original sewing ownership/current-input constructor refusal.
    Sewing(Box<DesignSewingError<E>>),
    /// Original complete garment ownership/current-input constructor refusal.
    Garment(Box<DesignGarmentError<E>>),
    /// A clone changed original held content outside the prepared graph.
    ContextChanged {
        /// Original canonical component whose cloned content differed.
        field: &'static str,
    },
    /// Actual output exceeded the complete pre-copy reservation; no candidate escapes.
    ReservationExceeded {
        /// Original successfully admitted complete logical reservation.
        reserved: DesignResourceDemand,
        /// Actual final complete garment demand.
        actual: DesignResourceDemand,
    },
}
impl<E: fmt::Display> fmt::Display for CommandSeamSplitGarmentError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owner { expected_design, provided_design } => write!(f,
                "seam candidate actual garment owner differs: expected {expected_design},provided {provided_design}"),
            Self::Resources(issue) => write!(f, "seam candidate garment resources: {issue}"),
            Self::Current(issue) => write!(f, "seam candidate original current garment: {issue}"),
            Self::ChildOwned { child, identity, owner } => write!(f,
                "seam candidate child {child} identity {identity} already owned at {owner:?}"),
            Self::Graph(issue) => write!(f, "seam candidate graph: {issue}"),
            Self::Sewing(issue) => write!(f, "seam candidate sewing ownership: {issue}"),
            Self::Garment(issue) => write!(f, "seam candidate garment ownership: {issue}"),
            Self::ContextChanged { field } => write!(f, "seam candidate clone changed {field}"),
            Self::ReservationExceeded { reserved, actual } => write!(f,
                "seam candidate garment output {actual:?} exceeds reservation {reserved:?}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for CommandSeamSplitGarmentError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Resources(issue) => Some(issue),
            Self::Current(issue) => Some(issue.as_ref()),
            Self::Graph(issue) => Some(issue.as_ref()),
            Self::Sewing(issue) => Some(issue.as_ref()),
            Self::Garment(issue) => Some(issue.as_ref()),
            Self::Owner { .. }
            | Self::ChildOwned { .. }
            | Self::ContextChanged { .. }
            | Self::ReservationExceeded { .. } => None,
        }
    }
}

/// Exact original garment plus admitted graph and complete conservative clone/output envelope.
/// The resource host's honest exhaustive readonly contract and semantic Clone/PartialEq contracts
/// apply. Logical accounting does not bound allocator/RSS,temporary validation indices or CPU.
/// Private fields prevent replacing the actual garment owner:
/// ```compile_fail
/// use sc_core::{command::CommandSeamSplitGarmentAdmission,design::DesignGarment};
/// fn replace<'a,H>(admitted:&mut CommandSeamSplitGarmentAdmission<'a,H>,other:&'a DesignGarment<H>){
///     admitted.original=other;
/// }
/// ```
/// Borrowed original ownership cannot escape:
/// ```compile_fail
/// use sc_core::{command::CommandSeamSplitGarmentAdmission,design::DesignGarment};
/// fn escape<H>(admitted:&CommandSeamSplitGarmentAdmission<'_,H>)->&'static DesignGarment<H>{
///     admitted.original()
/// }
/// ```
pub struct CommandSeamSplitGarmentAdmission<'a, H> {
    original: &'a DesignGarment<H>,
    graph: CommandSeamSplitGraphAdmission<'a, H>,
    limits: DesignResourceLimits,
    reservation: DesignResourceDemand,
}
impl<H> fmt::Debug for CommandSeamSplitGarmentAdmission<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            original: _,
            graph,
            limits,
            reservation,
        } = self;
        f.debug_struct("CommandSeamSplitGarmentAdmission")
            .field("graph", graph)
            .field("limits", limits)
            .field("reservation", reservation)
            .finish()
    }
}
impl<'a, H> CommandSeamSplitGarmentAdmission<'a, H> {
    /// Exact complete original owner, without a same-ID substitute.
    #[must_use]
    pub const fn original(&self) -> &'a DesignGarment<H> {
        self.original
    }
    /// Exact owned graph preparation, including actual source/stop/history evidence.
    #[must_use]
    pub const fn graph_admission(&self) -> &CommandSeamSplitGraphAdmission<'a, H> {
        &self.graph
    }
    /// Selected complete clone/output envelope.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.limits
    }
    /// Complete conservative original-garment plus prepared-graph payload demand.
    #[must_use]
    pub const fn reservation(&self) -> DesignResourceDemand {
        self.reservation
    }
}

impl<'a, H: DesignResourceSource> CommandSeamSplitGraphAdmission<'a, H> {
    /// Bind the exact complete garment and admit all clone/output data before further allocation.
    /// Actual owner pointer precedes complete garment plus graph reservation. Then revalidate the
    /// whole original garment and check A-ordered proposed children against its complete identity
    /// inventory. Original retired identities remain owned. No garment/input/host clone occurs.
    ///
    /// This checks actual local namespaces, not a trusted reserve cursor or persistent global
    /// issuance. Production IDs still come from the private trusted bus stage before effects.
    /// ```
    /// use sc_core::command::{CommandSeamSplitGraphAdmission,CommandSeamSplitGarmentAdmission,
    ///     CommandSeamSplitGarmentError};
    /// use sc_core::design::{DesignGarment,DesignResourceSource,DesignResourceLimits};
    /// fn admit<'a,H:DesignResourceSource>(graph:CommandSeamSplitGraphAdmission<'a,H>,
    ///     original:&'a DesignGarment<H>,limits:DesignResourceLimits)
    ///     ->Result<CommandSeamSplitGarmentAdmission<'a,H>,CommandSeamSplitGarmentError<H::Error>>{
    ///     graph.admit_garment_candidate(original,limits)
    /// }
    /// ```
    /// # Errors
    /// Retains exact owner, complete resource/current source or first child ownership refusal.
    pub fn admit_garment_candidate(
        self,
        original: &'a DesignGarment<H>,
        limits: DesignResourceLimits,
    ) -> Result<CommandSeamSplitGarmentAdmission<'a, H>, CommandSeamSplitGarmentError<H::Error>>
    {
        use CommandSeamSplitGarmentError as Error;
        if !core::ptr::eq(self.source.state(), original.sewing()) {
            return Err(Error::Owner {
                expected_design: self.source.state().inputs().id(),
                provided_design: original.inputs().id(),
            });
        }
        let mut meter = DesignResourceMeter::new(limits);
        charge_design_garment_resources(original, &mut meter).map_err(Error::Resources)?;
        charge_graph_reservation(
            original.sewing().graph(),
            self.joined.capture().item_count(),
            self.joined.partitions().reference_count(),
            self.children,
            &mut meter,
        )
        .map_err(Error::Resources)?;
        let reservation = meter.finish().map_err(Error::Resources)?;
        original
            .validate_current()
            .map_err(|issue| Error::Current(Box::new(issue)))?;
        let owners = original.identities();
        for (child, identity) in self.children.into_iter().enumerate() {
            if let Some((_, owner)) = owners.iter().find(|(owned, _)| *owned == identity) {
                return Err(Error::ChildOwned {
                    child,
                    identity,
                    owner: *owner,
                });
            }
        }
        Ok(CommandSeamSplitGarmentAdmission {
            original,
            graph: self,
            limits,
            reservation,
        })
    }
}

impl<H: DesignResourceSource + Clone + PartialEq> CommandSeamSplitGarmentAdmission<'_, H> {
    /// Construct the complete globally owned candidate while retaining exact original held data.
    /// Move the prepared graph and clone all other actual fields once through exhaustive owner
    /// views. Owning constructors check complete namespaces/current inputs; compare unchanged
    /// context and measure actual final content under the original envelope/reservation.
    ///
    /// Old Gathers/references remain unchanged and may now require repair. The returned owner is
    /// inspectable, not a current-validation token. Call validate_current for the original typed
    /// unresolved causes before any publication policy. No repair choice,identity reserve advance,
    /// authorization,numeric/physical/provider execution or atomic effect occurs.
    /// ```
    /// use sc_core::command::{CommandSeamSplitGraphAdmission,CommandSeamSplitGarmentError};
    /// use sc_core::design::{DesignGarment,DesignResourceSource,DesignResourceLimits};
    /// fn candidate<'a,H:DesignResourceSource+Clone+PartialEq>(graph:CommandSeamSplitGraphAdmission<'a,H>,
    ///     original:&'a DesignGarment<H>,limits:DesignResourceLimits)
    ///     ->Result<DesignGarment<H>,CommandSeamSplitGarmentError<H::Error>>{
    ///     graph.admit_garment_candidate(original,limits)?.build()
    /// }
    /// ```
    /// # Errors
    /// Preserves actual graph/owning-constructor/resource causes, changed-clone field or output
    /// reservation refusal. Original garment and trusted reserves remain unchanged.
    pub fn build(self) -> Result<DesignGarment<H>, CommandSeamSplitGarmentError<H::Error>> {
        use CommandSeamSplitGarmentError as Error;
        let Self {
            original,
            graph,
            limits,
            reservation,
        } = self;
        let graph = graph
            .build()
            .map_err(|issue| Error::Graph(Box::new(issue)))?;
        let DesignGarmentDefinition {
            sewing,
            pockets,
            notions,
            closures,
            layers,
            hems,
            constructions,
            drafting,
        } = original.definition();
        let DesignSewingDefinition {
            inputs,
            graph: _,
            landmarks,
        } = sewing.definition();
        let sewing = DesignSewing::new(DesignSewingDefinition {
            inputs: inputs.clone(),
            graph,
            landmarks: landmarks.clone(),
        })
        .map_err(|issue| Error::Sewing(Box::new(issue)))?;
        let candidate = DesignGarment::new(DesignGarmentDefinition {
            sewing,
            pockets: pockets.clone(),
            notions: notions.clone(),
            closures: closures.clone(),
            layers: layers.clone(),
            hems: hems.clone(),
            constructions: constructions.clone(),
            drafting: drafting.clone(),
        })
        .map_err(|issue| Error::Garment(Box::new(issue)))?;
        for (field, unchanged) in [
            ("inputs", candidate.inputs() == original.inputs()),
            (
                "landmarks",
                candidate.sewing().landmarks() == original.sewing().landmarks(),
            ),
            ("pockets", candidate.pockets() == pockets),
            ("notions", candidate.notions() == notions),
            ("closures", candidate.closures() == closures),
            ("layers", candidate.layers() == layers),
            ("hems", candidate.hems() == hems),
            ("constructions", candidate.constructions() == constructions),
        ] {
            if !unchanged {
                return Err(Error::ContextChanged { field });
            }
        }
        let mut meter = DesignResourceMeter::new(limits);
        charge_design_garment_resources(&candidate, &mut meter).map_err(Error::Resources)?;
        let actual = meter.finish().map_err(Error::Resources)?;
        if actual.collection_entries() > reservation.collection_entries()
            || actual.text_bytes() > reservation.text_bytes()
            || actual.counter_bits() > reservation.counter_bits()
        {
            return Err(Error::ReservationExceeded {
                reserved: reservation,
                actual,
            });
        }
        Ok(candidate)
    }
}
