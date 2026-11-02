//! Exact existing-output source binding, without numeric geometry or planned-output materialization.
use super::{
    DraftingContext, DraftingContextSource, DraftingGeometryReference, DraftingGeometrySource,
};
use crate::design::DesignMetadataHost;
use core::fmt;

/// First original field that prevents binding a raw existing-output response to its context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingGeometryBindingIssue {
    /// Returned output kind or persistent reference differs from the exact request.
    Reference,
    /// Returned frame record is not the original selected context frame record.
    Frame,
    /// Returned snapshot record is not the original selected context snapshot record.
    Snapshot,
}

/// Original request, context and complete raw response at the first source-binding mismatch.
pub struct DraftingGeometryMismatch<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
{
    context: DraftingContext<'a, H, S>,
    requested: DraftingGeometryReference,
    returned: DraftingGeometrySource<'a, S::Frame, S::Snapshot, S::Geometry>,
    issue: DraftingGeometryBindingIssue,
}
impl<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingGeometryMismatch<'a, H, S>
{
    /// Original admitted context, retaining its actual expected root/frame/snapshot sources.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.context
    }
    /// Exact originally requested output kind and persistent reference.
    #[must_use]
    pub const fn requested(&self) -> DraftingGeometryReference {
        self.requested
    }
    /// Complete original raw response, including the actual foreign/mismatched source handles.
    #[must_use]
    pub const fn returned(&self) -> DraftingGeometrySource<'a, S::Frame, S::Snapshot, S::Geometry> {
        self.returned
    }
    /// First refusal field, checked in reference, frame, snapshot order.
    #[must_use]
    pub const fn issue(&self) -> DraftingGeometryBindingIssue {
        self.issue
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingGeometryMismatch<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingGeometryMismatch")
            .field("requested", &self.requested)
            .field("returned", &self.returned)
            .field("issue", &self.issue)
            .finish_non_exhaustive()
    }
}

/// Original provider lookup cause or complete raw source-binding mismatch; no accepted output.
/// Boxed error content keeps the common result compact without copying the held source records.
pub enum DraftingGeometryError<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> {
    /// Original typed provider refusal for this exact request, without domain-token translation.
    Source {
        /// Original requested kind and persistent output reference.
        requested: DraftingGeometryReference,
        /// Original provider cause, retained without replacing its type or source chain.
        cause: Box<S::Error>,
    },
    /// First raw response mismatch, retaining actual expected and returned source witnesses.
    Mismatch(Box<DraftingGeometryMismatch<'a, H, S>>),
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingGeometryError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source { requested, cause } => f
                .debug_struct("Source")
                .field("requested", requested)
                .field("cause", cause)
                .finish(),
            Self::Mismatch(error) => f.debug_tuple("Mismatch").field(error).finish(),
        }
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingGeometryError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source { cause, .. } => write!(f, "drafting output source: {cause}"),
            Self::Mismatch(error) => write!(f, "drafting output source {:?} mismatch", error.issue),
        }
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingGeometryError<'_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Source { cause, .. } => Some(cause.as_ref()),
            Self::Mismatch(_) => None,
        }
    }
}

/// Borrowed existing output after exact reference and original frame/snapshot record binding.
/// This retains the original opaque source under the immutable provider contract. It reads no
/// geometry values, certifies no numeric/physical feasibility and materializes no planned output.
/// It is not an accepted program, dependency graph, publication or release approval.
/// ```compile_fail,E0616
/// use sc_core::{construction::{DraftingGeometryBinding, DraftingGeometryReference,
///     DraftingContextSource}, design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     binding:&mut DraftingGeometryBinding<'_,H,S>,other:DraftingGeometryReference) {
///     binding.returned.reference=other;
/// }
/// ```
pub struct DraftingGeometryBinding<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
{
    context: DraftingContext<'a, H, S>,
    returned: DraftingGeometrySource<'a, S::Frame, S::Snapshot, S::Geometry>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingGeometryBinding<'_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingGeometryBinding<'_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingGeometryBinding<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingGeometryBinding")
            .field("reference", &self.returned.reference)
            .finish_non_exhaustive()
    }
}
impl<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingGeometryBinding<'a, H, S>
{
    /// Original actual containing context, without a reconstructed owner or frame.
    #[must_use]
    pub const fn context(self) -> DraftingContext<'a, H, S> {
        self.context
    }
    /// Exact originally requested and matched persistent output reference.
    #[must_use]
    pub const fn reference(self) -> DraftingGeometryReference {
        self.returned.reference
    }
    /// Original provider geometry record, not a computed or copied value.
    #[must_use]
    pub const fn geometry(self) -> &'a S::Geometry {
        self.returned.geometry
    }
    /// Original complete response whose frame/snapshot records match the held context.
    #[must_use]
    pub const fn source(self) -> DraftingGeometrySource<'a, S::Frame, S::Snapshot, S::Geometry> {
        self.returned
    }
}
impl<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> DraftingContext<'a, H, S> {
    /// Query one existing typed output and bind its exact original sources to this held context.
    /// Checks reference, actual frame record, then actual snapshot record before exposing a binding.
    /// Logical source admission already happened when constructing this context. No value query,
    /// implicit transform, repair, new identity, planned-output creation or program prefix occurs.
    /// # Errors
    /// Returns the original typed provider cause or first mismatch with complete original witnesses.
    pub fn bind_existing_geometry(
        self,
        requested: DraftingGeometryReference,
    ) -> Result<DraftingGeometryBinding<'a, H, S>, DraftingGeometryError<'a, H, S>> {
        let returned =
            self.source()
                .geometry(requested)
                .map_err(|cause| DraftingGeometryError::Source {
                    requested,
                    cause: Box::new(cause),
                })?;
        let issue = if returned.reference != requested {
            Some(DraftingGeometryBindingIssue::Reference)
        } else if !self.frame().is_same_record(returned.frame) {
            Some(DraftingGeometryBindingIssue::Frame)
        } else if !self.snapshot().is_same_record(returned.snapshot) {
            Some(DraftingGeometryBindingIssue::Snapshot)
        } else {
            None
        };
        if let Some(issue) = issue {
            return Err(DraftingGeometryError::Mismatch(Box::new(
                DraftingGeometryMismatch {
                    context: self,
                    requested,
                    returned,
                    issue,
                },
            )));
        }
        Ok(DraftingGeometryBinding {
            context: self,
            returned,
        })
    }
}
