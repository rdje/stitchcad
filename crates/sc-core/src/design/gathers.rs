//! Original Gather intake/closing joins over complete actual current sewing/construction contexts.
use super::{DesignReferenceHost, DesignSewing, DesignSewingAmount, DesignSewingError};
use crate::{
    ontology::{
        ConstructionInventory, ConstructionInventoryError, ConstructionInventoryPosition, EntityId,
        Gather, GatherIntakeSource, ProfileBindingValidation,
    },
    value::CanonicalInputs,
};
use core::fmt;
use std::collections::BTreeMap;

/// Original actual Gather, canonical span ease metadata and logical closing-operation intent.
///
/// This retains no second amount/allocation/value/state or readiness cache. Its shared lifetime
/// keeps the actual sewing root and construction inventory borrowed while the result is used.
/// ```compile_fail
/// fn escape<H: sc_core::design::DesignReferenceHost>(state: sc_core::design::DesignSewing<H>,
///     constructions: sc_core::ontology::ConstructionInventory)
///     -> Vec<sc_core::design::DesignGatherDeclaration<'static>> {
///     state.gather_declarations(&constructions).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct DesignGatherDeclaration<'a> {
    gather: &'a Gather,
    position: ConstructionInventoryPosition,
    intake: GatherIntakeSource<'a>,
    amount: DesignSewingAmount<'a>,
}
impl fmt::Debug for DesignGatherDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignGatherDeclaration")
            .field("gathered_side", &self.intake.gathered_side)
            .finish_non_exhaustive()
    }
}
impl<'a> DesignGatherDeclaration<'a> {
    /// Exact original Gather object, without retargeted copy/span/source intent.
    #[must_use]
    pub const fn gather(self) -> &'a Gather {
        self.gather
    }
    /// Original Gather family and zero-based authored index.
    #[must_use]
    pub const fn position(self) -> ConstructionInventoryPosition {
        self.position
    }
    /// Actual original span's signed A-minus-B declaration and selected gathered side.
    #[must_use]
    pub const fn intake_source(self) -> GatherIntakeSource<'a> {
        self.intake
    }
    /// Existing actual span/current formula metadata, without resolved sign or allocation.
    #[must_use]
    pub const fn amount_declaration(self) -> DesignSewingAmount<'a> {
        self.amount
    }
    /// Exact original required logical operation; no numeric alias or operation lookup is applied.
    #[must_use]
    pub const fn closing_operation(self) -> &'a EntityId {
        &self.gather.definition().closing_operation
    }
    /// Actual containing root's single canonical owner.
    #[must_use]
    pub const fn canonical_inputs(self) -> &'a CanonicalInputs {
        self.amount.canonical_inputs()
    }
    /// Profile ease remains deferred; None means only that this ease has no Profile amount field.
    #[must_use]
    pub const fn profile_validation(self) -> Option<ProfileBindingValidation> {
        self.amount.profile_validation()
    }
}
/// Original complete current-context cause or explicit failed metadata join; no partial views.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignGatherDeclarationError<E> {
    /// Original whole sewing/root/landmark/graph/amount cause, including unused sources.
    Sewing(Box<DesignSewingError<E>>),
    /// Original whole construction source/current-context cause, including unused constructions.
    Constructions(Box<ConstructionInventoryError>),
    /// A validated held span has no amount entry; fail closed rather than omit or substitute a view.
    MissingSpanAmount {
        /// Original actual Gather identity.
        gather: EntityId,
        /// Original Gather family/index.
        position: ConstructionInventoryPosition,
        /// Exact original held span identity.
        span: EntityId,
    },
}
impl<E: fmt::Display> fmt::Display for DesignGatherDeclarationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sewing(issue) => write!(f, "Gather declarations current sewing: {issue}"),
            Self::Constructions(issue) => {
                write!(f, "Gather declarations current constructions: {issue}")
            }
            Self::MissingSpanAmount {
                gather,
                position,
                span,
            } => write!(
                f,
                "Gather {gather} at {position:?} lacks original span {span} amount metadata"
            ),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignGatherDeclarationError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sewing(issue) => Some(issue.as_ref()),
            Self::Constructions(issue) => Some(issue.as_ref()),
            Self::MissingSpanAmount { .. } => None,
        }
    }
}
impl<H: DesignReferenceHost> DesignSewing<H> {
    /// Borrow joined actual Gather ease/closing sources after complete current context validation.
    ///
    /// First validates whole sewing inputs, all landmarks, graph and amount metadata, then all supplied
    /// construction families' current sources, even if no Gathers exist. Original Gather/copy/span/
    /// direction bindings and explicit signed ease rules must hold. Views retain the original ease
    /// declaration, selected side and existing canonical amount adapter, with no copied distribution,
    /// value, uncertainty, computed negation or fallback. Operation existence/kind/dependencies,
    /// global garment/root identities, resolved intake/conservation and physical/release proof remain
    /// separate obligations. This method does not mutate, repair or cache success.
    /// # Errors
    /// Returns original sewing, then construction-context cause, then a missing exact span-amount
    /// join. No partial view collection is returned on failure.
    pub fn gather_declarations<'a>(
        &'a self,
        constructions: &'a ConstructionInventory,
    ) -> Result<Vec<DesignGatherDeclaration<'a>>, DesignGatherDeclarationError<H::Error>> {
        let amounts = self
            .amount_declarations()
            .map_err(|issue| DesignGatherDeclarationError::Sewing(Box::new(issue)))?;
        let structure = self.inputs().structure();
        let pieces = structure.pieces();
        constructions
            .validate_current(
                pieces.pieces(),
                structure.cut_plan(),
                self.graph(),
                self.landmarks(),
                structure.topology(),
            )
            .map_err(|issue| DesignGatherDeclarationError::Constructions(Box::new(issue)))?;
        let amounts = amounts
            .into_iter()
            .map(|amount| (amount.span().id(), amount))
            .collect::<BTreeMap<_, _>>();
        let mut views = Vec::new();
        for (index, gather) in constructions.gathers().iter().enumerate() {
            let position = ConstructionInventoryPosition {
                kind: crate::ontology::ConstructionKind::Gather,
                index,
            };
            let piece = pieces.piece(gather.piece()).ok_or_else(|| {
                DesignGatherDeclarationError::Constructions(Box::new(
                    ConstructionInventoryError::MissingPiece {
                        construction: gather.id(),
                        position,
                        piece: gather.piece(),
                    },
                ))
            })?;
            let intake = gather
                .current_intake_source(
                    piece,
                    structure.cut_plan(),
                    self.graph(),
                    structure.topology(),
                )
                .map_err(|issue| {
                    DesignGatherDeclarationError::Constructions(Box::new(
                        ConstructionInventoryError::Gather {
                            construction: gather.id(),
                            position,
                            issue: Box::new(issue),
                        },
                    ))
                })?;
            let amount = amounts.get(&gather.definition().span).copied().ok_or(
                DesignGatherDeclarationError::MissingSpanAmount {
                    gather: gather.id(),
                    position,
                    span: gather.definition().span,
                },
            )?;
            views.push(DesignGatherDeclaration {
                gather,
                position,
                intake,
                amount,
            });
        }
        Ok(views)
    }
}
