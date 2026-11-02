//! Complete actual native-input fields; no parsing, target binding or value access.
use super::DraftingOperationInput;
use crate::{
    design::{
        DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
    recipe::FormulaExpressionDocument,
};

// Dynamic ownership cannot silently enter a previously fixed schema field or pair element.
pub(in crate::construction) fn fixed<T: Copy>(_: &T) {}
pub(in crate::construction) fn fixed_slice<T: Copy>(_: &[T]) {}

fn location(operation: EntityId, field: &'static str) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(operation),
        field,
        index: None,
    }
}

impl DraftingOperationInput<'_> {
    /// Charge every actual owned native-input field into the caller's cumulative restricted sink.
    /// Original and canonical argument buffers both count, including repeated equal documents.
    /// Walk's pair vector counts its original length; fixed references and cached node counts
    /// own no dynamic content. Private exhaustive schema views and Copy guards enforce that split.
    ///
    /// Locations retain the authored operation identity and static field role. No containing
    /// program position is invented. This reads no values, parses/normalizes nothing, performs
    /// no target lookup and grants no whole-program, geometry or runtime resource admission.
    /// # Errors
    /// Returns the original first sink refusal in schema/argument/source-before-canonical order.
    /// Fixed-only and empty inputs also propagate any earlier latched sink refusal.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let operation = self.operation();
        sink.charge_entries(0, location(operation, "drafting.input"))?;
        match self {
            Self::ConstructPoint(input) => {
                let [x, y] = input.resource_documents();
                charge_document(
                    x,
                    operation,
                    "drafting.point.x.source",
                    "drafting.point.x.canonical",
                    sink,
                )?;
                charge_document(
                    y,
                    operation,
                    "drafting.point.y.source",
                    "drafting.point.y.canonical",
                    sink,
                )?;
            }
            Self::DrawArc(input) => {
                let [radius, start, sweep] = input.resource_documents();
                charge_document(
                    radius,
                    operation,
                    "drafting.arc.radius.source",
                    "drafting.arc.radius.canonical",
                    sink,
                )?;
                charge_document(
                    start,
                    operation,
                    "drafting.arc.start_angle.source",
                    "drafting.arc.start_angle.canonical",
                    sink,
                )?;
                charge_document(
                    sweep,
                    operation,
                    "drafting.arc.sweep_angle.source",
                    "drafting.arc.sweep_angle.canonical",
                    sink,
                )?;
            }
            Self::SlashSpread(input) => charge_document(
                input.resource_opening().resource_document(),
                operation,
                "drafting.slash_spread.opening.source",
                "drafting.slash_spread.opening.canonical",
                sink,
            )?,
            Self::WalkSeam(input) => {
                let pairs = input.resource_pairs();
                sink.charge_entries(pairs.len(), location(operation, "drafting.walk.pairs"))?;
                fixed_slice(pairs);
            }
            Self::DrawLine(input) => input.resource_fields(),
            Self::DrawBezier(input) => input.resource_fields(),
            Self::OffsetEdge(input) => input.resource_fields(),
            Self::PlaceGrainline(input) => input.resource_fields(),
            Self::TrueSeam(input) => input.resource_fields(),
            Self::MirrorPiece(input) => input.resource_fields(),
            Self::JoinPieces(input) => input.resource_fields(),
            Self::CloseDart { operation, request } => {
                fixed(operation);
                fixed(*request);
            }
            Self::AddNotch { operation, request } => {
                fixed(operation);
                fixed(*request);
            }
        }
        Ok(())
    }
}

fn charge_document(
    document: &FormulaExpressionDocument,
    operation: EntityId,
    source_field: &'static str,
    canonical_field: &'static str,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (source, canonical, nodes) = document.resource_parts();
    fixed(&nodes);
    sink.charge_text(source, location(operation, source_field))?;
    sink.charge_text(canonical, location(operation, canonical_field))
}
