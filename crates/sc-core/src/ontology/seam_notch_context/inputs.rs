//! Exhaustive borrowed dependency census;no indexes,resolution,rebirth or owning clones.
use super::{
    CutPlan, NotchSourceAdmissionIssue as Issue, NotchSourceCollection as Collection, Piece,
    SeamEaseSource, SewingLandmark,
};
use crate::ontology::{
    DeclaredEase, EaseDistribution, EntityId, LabelText, MaterialAssignment, NotchDefinition,
    PieceDefinition,
};

fn fixed<T: Copy>(_: &T) {}
fn fixed_slice<T: Copy>(_: &[T]) {}
struct Census {
    consumed: usize,
    limit: usize,
}
impl Census {
    fn charge(
        &mut self,
        requested: usize,
        collection: Collection,
        owner: Option<EntityId>,
        index: Option<usize>,
    ) -> Result<(), Issue> {
        let consumed = self.consumed;
        let total = consumed
            .checked_add(requested)
            .ok_or(Issue::CountOverflow {
                collection,
                owner,
                index,
                consumed,
                requested,
            })?;
        if total > self.limit {
            return Err(Issue::Items {
                collection,
                owner,
                index,
                consumed,
                requested,
                limit: self.limit,
            });
        }
        self.consumed = total;
        Ok(())
    }
}
pub(super) fn admit(
    source: &SeamEaseSource,
    plan: &CutPlan,
    pieces: &[Piece],
    marks: &[SewingLandmark],
    limit: usize,
) -> Result<usize, Issue> {
    let (id, a, b, direction, ease, stops) = source.resource_parts();
    fixed(&a);
    fixed(&b);
    fixed(&direction);
    let DeclaredEase {
        amount,
        distribution,
    } = ease;
    fixed(amount);
    // Eligibility already selected BetweenNotches;exhaustive views expose future owned fields.
    match distribution {
        EaseDistribution::Uniform => {}
        EaseDistribution::Weighted { side, regions } => {
            fixed(side);
            fixed_slice(regions);
        }
        EaseDistribution::BetweenNotches { side, start, end } => {
            fixed(side);
            fixed(start);
            fixed(end);
        }
    }
    let mut census = Census { consumed: 0, limit };
    census.charge(stops.len(), Collection::SourceStops, Some(id), None)?;
    fixed_slice(stops);
    let copies = plan.resource_copies();
    census.charge(copies.len(), Collection::CutCopies, None, None)?;
    fixed_slice(copies);
    census.charge(pieces.len(), Collection::Pieces, None, None)?;
    for (index, piece) in pieces.iter().enumerate() {
        let PieceDefinition {
            id,
            boundary,
            holes,
            construction_lines,
            quantity,
            mirroring,
            cut_on_fold,
            fold_edges,
            cutting_side,
            material,
            layer_index,
            label,
        } = piece.resource_definition();
        fixed(quantity);
        fixed(mirroring);
        fixed(cut_on_fold);
        fixed(cutting_side);
        fixed(layer_index);
        census.charge(
            boundary.len(),
            Collection::PieceBoundary,
            Some(*id),
            Some(index),
        )?;
        fixed_slice(boundary);
        census.charge(holes.len(), Collection::PieceHoles, Some(*id), Some(index))?;
        for (hole_index, hole) in holes.iter().enumerate() {
            census.charge(
                hole.len(),
                Collection::PieceHoleEdges,
                Some(*id),
                Some(hole_index),
            )?;
            fixed_slice(hole);
        }
        census.charge(
            construction_lines.len(),
            Collection::PieceConstructionLines,
            Some(*id),
            Some(index),
        )?;
        fixed_slice(construction_lines);
        census.charge(
            fold_edges.len(),
            Collection::PieceFoldEdges,
            Some(*id),
            Some(index),
        )?;
        fixed_slice(fold_edges);
        // Metadata is not traversed or cloned;its text bytes are not dependency items.
        match material {
            MaterialAssignment::Assigned(id) => fixed(id),
            MaterialAssignment::Unresolved(_reason) => {}
        }
        let LabelText {
            name: _,
            size: _,
            fabric: _,
            colorway: _,
        } = label;
    }
    census.charge(marks.len(), Collection::Landmarks, None, None)?;
    for mark in marks {
        match mark {
            SewingLandmark::Notch(notch) => {
                let (piece, definition) = notch.resource_parts();
                fixed(&piece);
                let NotchDefinition {
                    id,
                    anchor,
                    representation,
                } = definition;
                fixed(id);
                fixed(anchor);
                fixed(representation);
            }
            SewingLandmark::Turn(turn) => fixed(&turn.resource_fields()),
        }
    }
    Ok(census.consumed)
}
