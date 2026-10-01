//! The garment ontology — every first-class object in a StitchCAD design, and the identity that ties them
//! together.
//!
//! Normative source: `docs/book/src/spec/ontology.md` (gate G0, specified by `G0-CONTRACT.3`). Implemented
//! across three ordered slices: **`.3a`** the identity layer, **`.3b`** the persistent-identity contract
//! that resolves references under split/merge/reverse/delete (both landed), and **`.3c`** the
//! geometry-bearing object types (`Piece` landed at `.3c.1`; sewing spans, marks and constructions follow).
//!
//! The identity layer is the foundation the other two consume: a reference is meaningless without a stable
//! id to point at, and the contract is meaningless without a reference to resolve. It is dependency-free and
//! builds for `wasm32-unknown-unknown`, like the rest of `sc-core`'s core.
//!
//! | Submodule | Contents | Leaf |
//! | --- | --- | --- |
//! | [`piece`] | immutable structurally validated `Piece`, complete label view, deferred geometry | `.3c.1` |
//! | [`id`] | `EntityId` (a ULID) and the injected `IdGenerator` | `.3a` |
//! | [`rational`] | the bounded exact rational a parameter is stored in | `.3a` |
//! | [`reference`] | `EdgeRef`, `PointRef`, `LocalTag` and the `[0, 1]` `Param` | `.3a` |
//! | [`topology`] | the persistent-identity contract: the edit journal, resolution, `RepairTask`s | `.3b` |

pub mod id;
pub mod piece;
pub mod rational;
pub mod reference;
pub mod topology;

pub use id::{DeterministicIdGenerator, EntityId, IdError, IdGenerator, MAX_TIMESTAMP_MS};
pub use rational::Rational;
pub use reference::{EdgeRef, LocalTag, Param, ParamError, PointRef};
pub use topology::{
    Direction, IdentityLedger, JournalEntry, LedgerError, OffsetFragment, OffsetInterval,
    OpenRepair, OrphaningEdit, Registration, ReleaseReadiness, RepairTask, Resolution, ResolvedRef,
    SplitFragments, SplitSide, TopologyEdit, MAX_EDGES_PER_OPERATION,
};

pub use piece::{
    CuttingSide, DirectedEdge, GeometricValidation, LabelField, LabelText, LoopLocation,
    MaterialAssignment, Mirroring, Piece, PieceDefinition, PieceError, PrintedLabel,
};
