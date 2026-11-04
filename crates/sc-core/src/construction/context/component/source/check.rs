//! Exact original current headers and complete captured recipe association;no source callbacks.
use super::{
    AdmittedDraftingComponentWork, DraftingComponentProvider, DraftingComponentResponse,
    DraftingComponentSourceIssue as I,
};
use crate::{
    construction::{
        DraftingComponentCreator, DraftingComponentOrigin, DraftingComponentRecipeSource,
        DraftingComponentSource, DraftingImportedComponentSource, DraftingOperationInput,
        DraftingProgramStep,
    },
    design::{fixed_value, DesignReferenceHost},
    ontology::SeamSideId,
};
pub(super) fn association<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>(
    work: &AdmittedDraftingComponentWork<'_, '_, H, S>,
    source: DraftingComponentResponse<'_, H, S>,
) -> Result<Option<usize>, I> {
    let DraftingComponentSource {
        origin,
        frame,
        snapshot,
    } = source;
    if !core::ptr::eq(origin.piece(), work.piece()) {
        return Err(I::Piece);
    }
    let context = work.parent().context();
    if !frame.is_same_record(context.frame()) {
        return Err(I::Frame);
    }
    if !snapshot.is_same_record(context.snapshot()) {
        return Err(I::Snapshot);
    }
    match origin {
        DraftingComponentOrigin::Imported(DraftingImportedComponentSource { piece, source }) => {
            fixed_value(&piece);
            fixed_value(&source);
            Ok(None)
        }
        DraftingComponentOrigin::Recipe(DraftingComponentRecipeSource {
            program,
            piece,
            creator,
        }) => {
            let original = program.context();
            if !original.frame().is_same_record(frame) {
                return Err(I::RecipeFrame);
            }
            if *creator.piece_id() != piece.id() {
                return Err(I::CreatorPiece);
            }
            let root = original.scope().root().structure();
            match creator {
                DraftingComponentCreator::Mirror(declaration) => {
                    let actual = declaration.roles().metadata().cut().source().piece();
                    if !root
                        .pieces()
                        .piece(actual.id())
                        .is_some_and(|expected| core::ptr::eq(actual, expected))
                    {
                        return Err(I::RecipePiece { side: None });
                    }
                }
                DraftingComponentCreator::Join(declaration) => {
                    let sources = declaration.roles().banks().sources();
                    let inventory = sources.population().metadata().splice().inventory();
                    if !core::ptr::eq(inventory.inventory(), root.pieces())
                        || !core::ptr::eq(inventory.source().pieces(), root.pieces().pieces())
                    {
                        return Err(I::RecipeInventory);
                    }
                    if !core::ptr::eq(inventory.source().plan(), root.cut_plan()) {
                        return Err(I::RecipeCutPlan);
                    }
                    for side in [SeamSideId::A, SeamSideId::B] {
                        let actual = sources.piece(side);
                        if !root
                            .pieces()
                            .piece(actual.id())
                            .is_some_and(|expected| core::ptr::eq(actual, expected))
                        {
                            return Err(I::RecipePiece { side: Some(side) });
                        }
                    }
                }
            }
            let mut found = None;
            for (index, step) in program.steps().iter().enumerate() {
                let DraftingProgramStep::Drafting(input) = step else {
                    continue;
                };
                if same_creator(creator, *input) {
                    let position = index + 1; // A real Vec member index is strictly below usize::MAX.
                    if let Some(first) = found {
                        return Err(I::RepeatedCreator {
                            first,
                            second: position,
                        });
                    }
                    found = Some(position);
                }
            }
            found.map(Some).ok_or(I::MissingCreator)
        }
    }
}
fn same_creator(creator: DraftingComponentCreator<'_>, input: DraftingOperationInput<'_>) -> bool {
    match creator {
        DraftingComponentCreator::Mirror(declaration) => {
            matches!(input,DraftingOperationInput::MirrorPiece(actual) if core::ptr::eq(actual,declaration.roles().metadata().cut().source().input()))
        }
        DraftingComponentCreator::Join(declaration) => {
            matches!(input,DraftingOperationInput::JoinPieces(actual) if core::ptr::eq(actual,declaration.roles().banks().sources().population().metadata().splice().source().edit()))
        }
    }
}
