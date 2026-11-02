//! Borrowed selected-name population before namespace allocation or semantic callbacks.
use super::{DesignReferenceHost, DesignSourceReferences};
use crate::{recipe::FormulaDeclaration, value::CanonicalInputs};
use core::iter::FusedIterator;

/// Readonly,allocation-free selected measurement/Ease declaration inventory for bounded checking.
/// This supplements the existing allocating `DesignFormulaHost` factory;bounded mixed-program
/// integration uses this facet instead of first constructing that factory's declaration vector.
///
/// The iterator borrows actual selected table rows then Ease rows,in authored order. Its exact
/// length includes every original row,including one fallible item for a broken binding. Do not
/// filter unavailable/invalid/repeated rows or substitute cached totals. Unprovided selections
/// have no rows;missing selected objects remain current-reference refusals in later Root validation.
///
/// Construction,`len()` and drop only borrow/count immutable source fields: no allocation,source
/// validation,iteration,formatting,values or namespace construction. Item production subsequently
/// retains original tokens,Measurement/Ease identity and actual dedicated canonical Length loans,
/// or the original typed host binding cause. No general scalar Length substitutes for that owner.
/// Shared access cannot mutate the inventory. These are honest/exhaustive host obligations,not a
/// security boundary against dishonest host code. Implementations need neither Clone nor Debug.
pub trait DesignFormulaDeclarationSource: DesignReferenceHost {
    /// Complete borrowed row iterator;per-row binding failures remain items,not omitted rows.
    type Declarations<'a>: ExactSizeIterator<Item = Result<FormulaDeclaration<'a>, Self::Error>>
        + FusedIterator
    where
        Self: 'a;

    /// Borrow the actual selected source population without validating or consuming its items.
    /// Core first admits full held content,then uses the actual length for complete prospective
    /// name-work bounds. Later source/order checking consumes the rows after Root validation.
    fn formula_declaration_sources<'a>(
        &'a self,
        references: DesignSourceReferences,
        canonical: &'a CanonicalInputs,
    ) -> Self::Declarations<'a>;
}
