//! Seal actual captured ease and stop-view provenance without cloning,queries or new authority.
use super::{CommandStopChild, CommandStopChildView, CommandStopPartitions};
use crate::{
    command::{CommandEaseCapture, SplitSeamSpan},
    ontology::SeamSpan,
};
use core::fmt;

/// Exact source-owner mismatch;actual expected/captured objects remain borrowed,not just their IDs.
#[derive(Clone, Copy)]
pub enum CommandStopSourceBindingError<'a> {
    /// Captured request owner differs,even if its body and target identity compare equal.
    Request {
        /// Original request from the actual stop input/context/view pipeline.
        expected: &'a SplitSeamSpan,
        /// Original request retained by the actual ease capture.
        captured: &'a SplitSeamSpan,
    },
    /// Same request but a different actual graph member supplied the capture.
    Target {
        /// Actual immutable target that supplied the stop context.
        expected: &'a SeamSpan,
        /// Actual immutable target that supplied the ease capture.
        captured: &'a SeamSpan,
    },
}
impl fmt::Debug for CommandStopSourceBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request { expected, captured } => f
                .debug_struct("CommandStopSourceBindingError::Request")
                .field("expected_target", &expected.span())
                .field("captured_target", &captured.span())
                .finish(),
            Self::Target { expected, captured } => f
                .debug_struct("CommandStopSourceBindingError::Target")
                .field("expected_target", &expected.id())
                .field("captured_target", &captured.id())
                .finish(),
        }
    }
}
impl fmt::Display for CommandStopSourceBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request { expected, captured } => write!(
                f,
                "command stop capture request owners differ: expected target {},captured target {}",
                expected.span(),
                captured.span()
            ),
            Self::Target { expected, captured } => write!(
                f,
                "command stop capture target owners differ: expected {},captured {}",
                expected.id(),
                captured.id()
            ),
        }
    }
}
impl std::error::Error for CommandStopSourceBindingError<'_> {}

/// Exact externally owned ease capture and ordered stop partitions bound to one request/target.
/// This proves source-owner coherence,not full original-root currentness,quantity availability,
/// physical correspondence,whole command resource admission,fresh authority or atomic effects.
/// Private fields prevent fabricated success:
/// ```compile_fail
/// use sc_core::command::{CommandEaseCapture,CommandStopPartitions,CommandStopSourceBinding};
/// fn forge<'a,H>(capture:&'a CommandEaseCapture<'a>,partitions:&'a CommandStopPartitions<'a,H>)
///     ->CommandStopSourceBinding<'a,H>{CommandStopSourceBinding {capture,partitions}}
/// ```
/// Both actual owners must outlive the joined view:
/// ```compile_fail
/// use sc_core::command::{CommandEaseCapture,CommandStopPartitions,CommandStopSourceBinding};
/// fn escape<H>(capture:&CommandEaseCapture<'_>,partitions:&CommandStopPartitions<'_,H>)
///     ->CommandStopSourceBinding<'static,H>{partitions.bind_source_capture(capture).unwrap()}
/// ```
pub struct CommandStopSourceBinding<'a, H> {
    capture: &'a CommandEaseCapture<'a>,
    partitions: &'a CommandStopPartitions<'a, H>,
}
impl<H> fmt::Debug for CommandStopSourceBinding<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            capture,
            partitions,
        } = self;
        f.debug_struct("CommandStopSourceBinding")
            .field("target", &capture.binding().span().id())
            .field("source", &capture.source().span())
            .field("references", &partitions.reference_count())
            .finish()
    }
}
impl<'a, H> CommandStopSourceBinding<'a, H> {
    /// Exact captured root and admitted metadata/target provenance;no recapture occurs.
    #[must_use]
    pub const fn capture(&self) -> &'a CommandEaseCapture<'a> {
        self.capture
    }
    /// Exact actual ordered stops,complete memberships,current evidence and input/limit owners.
    #[must_use]
    pub const fn partitions(&self) -> &'a CommandStopPartitions<'a, H> {
        self.partitions
    }
    /// Actual child stop rows and source pair bound to this exact captured request/target.
    #[must_use]
    pub fn child(&self, child: CommandStopChild) -> CommandStopChildView<'_> {
        self.partitions.child(child)
    }
}
impl<H> CommandStopPartitions<'_, H> {
    /// Seal exact request and actual target owner identity before using captured ease with stops.
    /// Check request pointer first,then target pointer. Equal IDs or source bodies do not substitute.
    /// Both tokens stay externally owned;no allocation,clone,query,value read or lifetime extension.
    /// Actual Design source admission and full original quantity/context requirements remain their
    /// own stages;this join cannot upgrade metadata into original-root/physical/effect readiness.
    /// ```
    /// use sc_core::command::{CommandEaseCapture,CommandStopPartitions,CommandStopSourceBinding,
    ///     CommandStopSourceBindingError};
    /// fn join<'a,H>(capture:&'a CommandEaseCapture<'a>,partitions:&'a CommandStopPartitions<'a,H>)
    ///     ->Result<CommandStopSourceBinding<'a,H>,CommandStopSourceBindingError<'a>>{
    ///     partitions.bind_source_capture(capture)
    /// }
    /// ```
    /// # Errors
    /// Retains actual expected/captured request or target borrows on mismatch. No partial joined
    /// view or fabricated standard error cause is returned.
    pub fn bind_source_capture<'a>(
        &'a self,
        capture: &'a CommandEaseCapture<'a>,
    ) -> Result<CommandStopSourceBinding<'a, H>, CommandStopSourceBindingError<'a>> {
        let expected = self.memberships().context().ownership().inputs().binding();
        let captured = capture.binding();
        if !core::ptr::eq(expected.request(), captured.request()) {
            return Err(CommandStopSourceBindingError::Request {
                expected: expected.request(),
                captured: captured.request(),
            });
        }
        if !core::ptr::eq(expected.span(), captured.span()) {
            return Err(CommandStopSourceBindingError::Target {
                expected: expected.span(),
                captured: captured.span(),
            });
        }
        Ok(CommandStopSourceBinding {
            capture,
            partitions: self,
        })
    }
}
