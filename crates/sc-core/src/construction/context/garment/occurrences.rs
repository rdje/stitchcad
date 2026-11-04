//! Original Pocket/Closure fields with actual garment/member locations;no source admission.
use crate::{
    design::{fixed_value, DesignGarment, DesignMetadataHost, DesignReferenceHost},
    ontology::{
        ButtonholeDerivation, Closure, ClosureDefinition, ClosureInstance, ClosureKind,
        ClosureLength, DirectedRange, EdgeAnchor, EntityId, NotionSize, Pocket, PocketDefinition,
        PocketOpening, PocketPieceRef,
    },
};
use core::{fmt, iter::FusedIterator};
/// Actual original Pocket/Closure member and its zero-based authored inventory position.
#[derive(Clone, Copy)]
pub enum DraftingGarmentSemanticOwner<'a> {
    /// Original Pocket member.
    Pocket {
        /// Actual original inventory position.
        index: usize,
        /// Actual original object.
        pocket: &'a Pocket,
    },
    /// Original Closure member.
    Closure {
        /// Actual original inventory position.
        index: usize,
        /// Actual original object.
        closure: &'a Closure,
    },
}
impl fmt::Debug for DraftingGarmentSemanticOwner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pocket { index, .. } => f.debug_tuple("Pocket").field(index).finish(),
            Self::Closure { index, .. } => f.debug_tuple("Closure").field(index).finish(),
        }
    }
}
/// Canonical logical role within an actual Closure kind,without selected size or vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingClosureSizeRole {
    /// HookAndBar hook origin.
    Hook,
    /// HookAndBar bar origin.
    Bar,
    /// ButtonAndButtonhole canonical button origin.
    Button,
}
/// One original Pocket/Closure field. Public view construction supplies no owner/source proof.
#[derive(Clone, Copy)]
pub enum DraftingGarmentSemanticField<'a> {
    /// Original physical served copy/expected-Piece reference.
    PocketServed(&'a PocketPieceRef),
    /// Original source-frame attachment position.
    PocketPosition(&'a EdgeAnchor),
    /// Original source-frame directed orientation.
    PocketOrientation(&'a DirectedRange),
    /// Original logical declaration/Profile opening origin.
    PocketOpening(&'a PocketOpening),
    /// Every original component reference,without deduplication or recipe lookup.
    PocketComponent {
        /// Zero-based original component position.
        index: usize,
        /// Original physical copy/expected-Piece field.
        reference: &'a PocketPieceRef,
    },
    /// Original zipper length provenance,without a value read or numeric source binding.
    ClosureLength(&'a ClosureLength),
    /// Original typed logical hardware-size source.
    ClosureSize {
        /// Actual original kind role.
        role: DraftingClosureSizeRole,
        /// Exact original source field.
        source: &'a NotionSize,
    },
    /// Original buttonhole-operation relation,without an authored hole length.
    ClosureButtonhole(&'a ButtonholeDerivation),
    /// Every original stable physical instance and its original placement fields.
    ClosureInstance {
        /// Zero-based original instance position.
        index: usize,
        /// Complete original instance,not its index as an identity.
        instance: &'a ClosureInstance,
    },
    /// Raw unsupported Fly gap field;normal Closure construction already refuses that envelope.
    ClosureUnsupportedFly(&'a EntityId),
}
impl fmt::Debug for DraftingGarmentSemanticField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PocketServed(_) => f.write_str("PocketServed"),
            Self::PocketPosition(_) => f.write_str("PocketPosition"),
            Self::PocketOrientation(_) => f.write_str("PocketOrientation"),
            Self::PocketOpening(_) => f.write_str("PocketOpening"),
            Self::PocketComponent { index, .. } => {
                f.debug_tuple("PocketComponent").field(index).finish()
            }
            Self::ClosureLength(_) => f.write_str("ClosureLength"),
            Self::ClosureSize { role, .. } => f.debug_tuple("ClosureSize").field(role).finish(),
            Self::ClosureButtonhole(_) => f.write_str("ClosureButtonhole"),
            Self::ClosureInstance { index, .. } => {
                f.debug_tuple("ClosureInstance").field(index).finish()
            }
            Self::ClosureUnsupportedFly(_) => f.write_str("ClosureUnsupportedFly"),
        }
    }
}
/// Actual original garment/member/field occurrence. This grants no content/work allowance,
/// currentness,source association or executable proof. Root records have no invented mixed position.
/// ```compile_fail,E0616
/// use sc_core::{construction::DraftingGarmentSemanticUse,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost>(use_:&mut DraftingGarmentSemanticUse<'_,H>) {use_.field=todo!();}
/// ```
/// Original data owners cannot escape:
/// ```compile_fail
/// use sc_core::{construction::DraftingGarmentSemanticUse,design::DesignMetadataHost};
/// fn escape<'a,H:DesignMetadataHost>(use_:DraftingGarmentSemanticUse<'a,H>)->DraftingGarmentSemanticUse<'static,H> {use_}
/// ```
pub struct DraftingGarmentSemanticUse<'a, H: DesignMetadataHost> {
    garment: &'a DesignGarment<H>,
    owner: DraftingGarmentSemanticOwner<'a>,
    field: DraftingGarmentSemanticField<'a>,
}
impl<H: DesignMetadataHost> Copy for DraftingGarmentSemanticUse<'_, H> {}
impl<H: DesignMetadataHost> Clone for DraftingGarmentSemanticUse<'_, H> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost> fmt::Debug for DraftingGarmentSemanticUse<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingGarmentSemanticUse")
            .field("owner", &self.owner)
            .field("field", &self.field)
            .finish_non_exhaustive()
    }
}
impl<'a, H: DesignMetadataHost> DraftingGarmentSemanticUse<'a, H> {
    /// Exact actual garment owning this original member/field occurrence.
    #[must_use]
    pub const fn garment(self) -> &'a DesignGarment<H> {
        self.garment
    }
    /// Exact original member and its actual zero-based inventory position.
    #[must_use]
    pub const fn owner(self) -> DraftingGarmentSemanticOwner<'a> {
        self.owner
    }
    /// Exact original typed dependency field and any original nested index.
    #[must_use]
    pub const fn field(self) -> DraftingGarmentSemanticField<'a> {
        self.field
    }
}
impl<H: DesignReferenceHost> DesignGarment<H> {
    /// Stream every original Pocket field in definition order,then every Closure kind/instance
    /// field in original inventory order. Borrow actual buffers;allocate/clone/query/read no value.
    /// Exhaustive definition/kind/instance guards classify growth. This scoped raw view covers only
    /// Pockets/Closures,not other garment families or semantic/native program slots. It grants no
    /// new resource/work allowance;callers must preflight complete walks before bounded matching.
    /// Root locations never manufacture a mixed-program position,creator or selected logical value.
    /// ```
    /// use sc_core::design::{DesignGarment,DesignReferenceHost};
    /// fn count<H:DesignReferenceHost>(garment:&DesignGarment<H>)->usize {
    ///     garment.pocket_closure_dependencies().count()
    /// }
    /// ```
    pub fn pocket_closure_dependencies(
        &self,
    ) -> impl FusedIterator<Item = DraftingGarmentSemanticUse<'_, H>> + '_ {
        let pockets =
            self.pockets()
                .pockets()
                .iter()
                .enumerate()
                .flat_map(move |(index, pocket)| {
                    let PocketDefinition {
                        id,
                        served,
                        position,
                        orientation,
                        opening,
                        components,
                    } = pocket.definition();
                    fixed_value(id);
                    [
                        DraftingGarmentSemanticField::PocketServed(served),
                        DraftingGarmentSemanticField::PocketPosition(position),
                        DraftingGarmentSemanticField::PocketOrientation(orientation),
                        DraftingGarmentSemanticField::PocketOpening(opening),
                    ]
                    .into_iter()
                    .chain(components.iter().enumerate().map(|(index, reference)| {
                        DraftingGarmentSemanticField::PocketComponent { index, reference }
                    }))
                    .map(move |field| DraftingGarmentSemanticUse {
                        garment: self,
                        owner: DraftingGarmentSemanticOwner::Pocket { index, pocket },
                        field,
                    })
                });
        let closures =
            self.closures()
                .closures()
                .iter()
                .enumerate()
                .flat_map(move |(index, closure)| {
                    let ClosureDefinition {
                        id,
                        kind,
                        instances,
                    } = closure.definition();
                    fixed_value(id);
                    let fields = match kind {
                        ClosureKind::CentredZipper { length } => [
                            Some(DraftingGarmentSemanticField::ClosureLength(length)),
                            None,
                        ],
                        ClosureKind::HookAndBar { hook, bar } => [
                            Some(DraftingGarmentSemanticField::ClosureSize {
                                role: DraftingClosureSizeRole::Hook,
                                source: hook,
                            }),
                            Some(DraftingGarmentSemanticField::ClosureSize {
                                role: DraftingClosureSizeRole::Bar,
                                source: bar,
                            }),
                        ],
                        ClosureKind::ButtonAndButtonhole { button, hole } => [
                            Some(DraftingGarmentSemanticField::ClosureSize {
                                role: DraftingClosureSizeRole::Button,
                                source: button,
                            }),
                            Some(DraftingGarmentSemanticField::ClosureButtonhole(hole)),
                        ],
                        ClosureKind::Fly { trousers_gap } => [
                            Some(DraftingGarmentSemanticField::ClosureUnsupportedFly(
                                trousers_gap,
                            )),
                            None,
                        ],
                    };
                    fields
                        .into_iter()
                        .flatten()
                        .chain(instances.iter().enumerate().map(|(index, instance)| {
                            let ClosureInstance { id, first, second } = instance;
                            fixed_value(id);
                            fixed_value(first);
                            fixed_value(second);
                            DraftingGarmentSemanticField::ClosureInstance { index, instance }
                        }))
                        .map(move |field| DraftingGarmentSemanticUse {
                            garment: self,
                            owner: DraftingGarmentSemanticOwner::Closure { index, closure },
                            field,
                        })
                });
        pockets.chain(closures).fuse()
    }
}
