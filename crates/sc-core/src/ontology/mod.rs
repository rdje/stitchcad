//! The garment ontology — every first-class object in a StitchCAD design, and the identity that ties them
//! together.
//!
//! Normative source: `docs/book/src/spec/ontology.md` (gate G0, specified by `G0-CONTRACT.3`). Implemented
//! across three ordered slices: **`.3a`** the identity layer (this module today), **`.3b`** the
//! persistent-identity contract that resolves references under split/merge/reverse/delete, and **`.3c`** the
//! geometry-bearing object types (`Piece`, `SeamSpan`/`SewingGraph`, `Notch`, and the rest).
//!
//! The identity layer is the foundation the other two consume: a reference is meaningless without a stable
//! id to point at, and the contract is meaningless without a reference to resolve. It is dependency-free and
//! builds for `wasm32-unknown-unknown`, like the rest of `sc-core`'s core.
//!
//! | Submodule | Contents | Leaf |
//! | --- | --- | --- |
//! | [`id`] | `EntityId` (a ULID) and the injected `IdGenerator` | `.3a` |
//! | [`rational`] | the bounded exact rational a parameter is stored in | `.3a` |
//! | [`reference`] | `EdgeRef`, `PointRef`, `LocalTag` and the `[0, 1]` `Param` | `.3a` |

pub mod id;
pub mod rational;
pub mod reference;

pub use id::{DeterministicIdGenerator, EntityId, IdError, IdGenerator, MAX_TIMESTAMP_MS};
pub use rational::Rational;
pub use reference::{EdgeRef, LocalTag, Param, ParamError, PointRef};
