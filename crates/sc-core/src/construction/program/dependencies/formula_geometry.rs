//! Original geometry name uses in actual Root sources and complete mixed canonical slots.
use super::{BoundDraftingProgramCanonicalInputs, DraftingProgramCanonicalOccurrence};
use crate::{
    construction::{DraftingGeometryReference, DraftingInitialInputProvider},
    design::DesignMetadataHost,
    recipe::{
        FormulaCheckedSource, FormulaDeclaration, FormulaDeclarationSource, FormulaSourceSpan,
    },
};
use core::{fmt, iter::FusedIterator};

/// Inspect the actual original formula consumer; this enum alone is no dependency certificate.
#[derive(Clone, Copy)]
pub enum DraftingProgramFormulaGeometryConsumer<'v, 'j> {
    /// Original complete checked Root source, including unused source bodies.
    Source {
        /// Zero-based position in the selected original complete source-proof vector.
        source_index: usize,
        /// Actual native proof, preserving its original expression arena and source owner.
        source: &'v FormulaCheckedSource<'j>,
    },
    /// Actual occurrence slot in the complete mixed canonical association vector.
    Mixed(&'v DraftingProgramCanonicalOccurrence<'j>),
}
impl fmt::Debug for DraftingProgramFormulaGeometryConsumer<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source { source_index, .. } => f
                .debug_struct("Source")
                .field("source_index", source_index)
                .finish_non_exhaustive(),
            Self::Mixed(slot) => f
                .debug_struct("Mixed")
                .field("position", &slot.occurrence().position())
                .finish_non_exhaustive(),
        }
    }
}
/// One original Point/Edge name occurrence, with its actual checked source or mixed consumer.
/// Private construction prevents replacing its declaration, span, reference or containing slot.
/// This is original dependency evidence; it accepts no geometry supplier or execution result.
/// ```compile_fail,E0616
/// fn alter(use_:&mut sc_core::construction::DraftingProgramFormulaGeometryUse<'_, '_>) {use_.index=0;}
/// ```
/// ```compile_fail
/// fn escape<'v,'j>(use_:sc_core::construction::DraftingProgramFormulaGeometryUse<'v,'j>)
///     ->sc_core::construction::DraftingProgramFormulaGeometryUse<'static,'j> {use_}
/// ```
#[derive(Clone, Copy)]
pub struct DraftingProgramFormulaGeometryUse<'v, 'j> {
    consumer: DraftingProgramFormulaGeometryConsumer<'v, 'j>,
    index: usize,
    span: FormulaSourceSpan,
    declaration: FormulaDeclaration<'j>,
    reference: DraftingGeometryReference,
}
impl fmt::Debug for DraftingProgramFormulaGeometryUse<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramFormulaGeometryUse")
            .field("consumer", &self.consumer)
            .field("index", &self.index)
            .field("span", &self.span)
            .field("reference", &self.reference)
            .finish_non_exhaustive()
    }
}
impl<'v, 'j> DraftingProgramFormulaGeometryUse<'v, 'j> {
    /// Actual original source proof or complete mixed canonical occurrence slot.
    #[must_use]
    pub const fn consumer(self) -> DraftingProgramFormulaGeometryConsumer<'v, 'j> {
        self.consumer
    }
    /// Original zero-based dependency index in this source expression or complete mixed stream.
    /// Nongeometry uses still occupy their original indices; this is not a filtered ordinal.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual name-node span in the original source body or mixed operand document.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
    /// Exact original Point/Edge declaration, including the borrowed metadata name.
    #[must_use]
    pub const fn declaration(self) -> FormulaDeclaration<'j> {
        self.declaration
    }
    /// Original persistent reference, without looking up or accepting a geometry supplier.
    #[must_use]
    pub const fn reference(self) -> DraftingGeometryReference {
        self.reference
    }
}
fn geometry_reference(source: FormulaDeclarationSource<'_>) -> Option<DraftingGeometryReference> {
    match source {
        FormulaDeclarationSource::Point(point) => Some(DraftingGeometryReference::Point(point)),
        FormulaDeclarationSource::Edge(edge) => Some(DraftingGeometryReference::Edge(edge)),
        FormulaDeclarationSource::Input { .. }
        | FormulaDeclarationSource::LengthInput { .. }
        | FormulaDeclarationSource::Recipe { .. }
        | FormulaDeclarationSource::Reserved(_) => None,
    }
}
impl<
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Stream original formula geometry uses: every Root source in original order, then the
    /// complete mixed occurrence stream. Each cohort retains its actual consumer and original
    /// local dependency index. All branches/repeated uses participate; nongeometry names filter
    /// out without renumbering later uses. No buffers, source extraction, callbacks or values.
    /// This does not enumerate native reference fields or admit any geometry/frame/currentness.
    ///
    /// ```
    /// use sc_core::{construction::*,design::DesignMetadataHost};
    /// fn geometry_formula_uses<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     inputs:&BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_,H,S>) -> usize {
    ///     inputs.formula_geometry_dependencies().count()
    /// }
    /// ```
    pub fn formula_geometry_dependencies(
        &self,
    ) -> impl FusedIterator<Item = DraftingProgramFormulaGeometryUse<'_, 'j>> + '_ {
        let sources = self
            .inputs()
            .sources()
            .proof()
            .sources()
            .iter()
            .enumerate()
            .flat_map(|(source_index, source)| {
                source
                    .proof()
                    .dependencies()
                    .iter()
                    .copied()
                    .enumerate()
                    .filter_map(move |(index, use_)| {
                        Some(DraftingProgramFormulaGeometryUse {
                            consumer: DraftingProgramFormulaGeometryConsumer::Source {
                                source_index,
                                source,
                            },
                            index,
                            span: use_.span(),
                            declaration: use_.declaration(),
                            reference: geometry_reference(use_.declaration().source())?,
                        })
                    })
            });
        let mixed = self
            .occurrences()
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                let use_ = slot.occurrence();
                Some(DraftingProgramFormulaGeometryUse {
                    consumer: DraftingProgramFormulaGeometryConsumer::Mixed(slot),
                    index,
                    span: use_.span(),
                    declaration: use_.declaration(),
                    reference: geometry_reference(use_.declaration().source())?,
                })
            });
        sources.chain(mixed).fuse()
    }
}
