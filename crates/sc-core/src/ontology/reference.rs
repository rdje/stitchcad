//! Stable topological references: addressing an edge or a point by identity, never by index.
//!
//! The normative source is `docs/book/src/spec/ontology.md` §1 ("Geometric sub-entities are addressed
//! through stable topological references, not indices") and the glossary entries `EdgeRef`, `PointRef`,
//! `parameterized reference` and `stable topological reference`. The coupled decision is
//! `docs/decisions/decision_entity-identity-ulid-injected-generator.md`.
//!
//! A reference names the **operation that created** the sub-entity (by its [`EntityId`]) plus a **local,
//! persistent tag** distinguishing the several sub-entities one operation can create. Neither half is an
//! index into a mutable array, so a reference survives the edits around it — the property §1.1's whole
//! contract exists to preserve, and the one `.3b`'s property tests prove under split, merge and reverse.

use core::fmt;

use super::id::EntityId;
use super::rational::Rational;

/// A local, persistent tag distinguishing the sub-entities one creating operation produced.
///
/// It is *local* to the operation (the first edge an operation draws is tag `0`, the second `1`, and so on)
/// and *persistent* (assigned once, never reused, never renumbered by a later edit). It is deliberately not
/// a global index: a global index would shift when anything before it changes, which is exactly the
/// instability §1 forbids.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalTag(u32);

impl LocalTag {
    /// The first tag an operation assigns.
    pub const FIRST: Self = Self(0);

    /// Builds a local tag.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// The tag value.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for LocalTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A reference to an edge — of a piece boundary, of a hole, or of an internal construction line.
///
/// Identified by the [`EntityId`] of the operation that created the edge plus a [`LocalTag`]. Construction
/// is infallible: any `(creator, tag)` pair is a well-formed reference. Whether it *resolves* to a live
/// edge is a separate question — an edit can orphan it, and `.3b` turns an orphan into a visible repair
/// task rather than a silent reassignment (§1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeRef {
    creator: EntityId,
    tag: LocalTag,
}

impl EdgeRef {
    /// Builds a reference to the edge `tag` created by operation `creator`.
    #[must_use]
    pub const fn new(creator: EntityId, tag: LocalTag) -> Self {
        Self { creator, tag }
    }

    /// The entity id of the operation that created the edge.
    #[must_use]
    pub const fn creator(self) -> EntityId {
        self.creator
    }

    /// The local, persistent tag of the edge within its creating operation.
    #[must_use]
    pub const fn tag(self) -> LocalTag {
        self.tag
    }
}

impl fmt::Display for EdgeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "edge({}: {})", self.creator, self.tag)
    }
}

/// A reference to a constructed point — an intersection, a notch anchor, a grade point.
///
/// Identified the same way as an [`EdgeRef`]: the creating operation's [`EntityId`] plus a [`LocalTag`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointRef {
    creator: EntityId,
    tag: LocalTag,
}

impl PointRef {
    /// Builds a reference to the point `tag` created by operation `creator`.
    #[must_use]
    pub const fn new(creator: EntityId, tag: LocalTag) -> Self {
        Self { creator, tag }
    }

    /// The entity id of the operation that created the point.
    #[must_use]
    pub const fn creator(self) -> EntityId {
        self.creator
    }

    /// The local, persistent tag of the point within its creating operation.
    #[must_use]
    pub const fn tag(self) -> LocalTag {
        self.tag
    }
}

impl fmt::Display for PointRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "point({}: {})", self.creator, self.tag)
    }
}

/// A position along an edge: an exact rational constrained to `[0, 1]` of that edge's own length.
///
/// This is the `t` of a *parameterized reference* (ontology §1). It is a [`Rational`], not a float and not a
/// vertex index, so it survives a change of tessellation and a change of units; the `[0, 1]` bound is
/// enforced **at construction**, so a `Param` is never out of range and a consumer never re-checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Param(Rational);

impl Param {
    /// The start of the edge (`t = 0`).
    pub const START: Self = Self(Rational::ZERO);

    /// The end of the edge (`t = 1`).
    pub const END: Self = Self(Rational::ONE);

    /// Builds a parameter, rejecting a rational outside `[0, 1]`.
    ///
    /// # Errors
    ///
    /// Returns [`ParamError::OutOfRange`] when `t` is negative or greater than one. The value is carried in
    /// the diagnostic so the offending position is visible, not just reported.
    pub fn new(t: Rational) -> Result<Self, ParamError> {
        if t.in_unit_interval() {
            Ok(Self(t))
        } else {
            Err(ParamError::OutOfRange { value: t })
        }
    }

    /// The exact rational position this parameter carries.
    #[must_use]
    pub const fn as_rational(self) -> Rational {
        self.0
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t={}", self.0)
    }
}

/// The one way a parameter can be invalid: it is outside `[0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamError {
    /// The rational was negative or greater than one.
    OutOfRange {
        /// The offending value.
        value: Rational,
    },
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange { value } => write!(
                f,
                "a parameter along an edge must be in [0, 1]; {value} is not (ontology §1)"
            ),
        }
    }
}

impl std::error::Error for ParamError {}

#[cfg(test)]
mod tests {
    use super::{EdgeRef, LocalTag, Param, ParamError, PointRef, Rational};
    use crate::ontology::id::{DeterministicIdGenerator, EntityId, IdGenerator};

    #[test]
    fn a_reference_names_its_creator_and_tag() {
        let creator = EntityId::from_bits(7);
        let edge = EdgeRef::new(creator, LocalTag::new(2));
        assert_eq!(edge.creator(), creator);
        assert_eq!(edge.tag(), LocalTag::new(2));
        assert_eq!(edge.to_string(), format!("edge({creator}: 2)"));
    }

    #[test]
    fn a_point_reference_is_built_the_same_way() {
        let creator = EntityId::from_bits(9);
        let point = PointRef::new(creator, LocalTag::FIRST);
        assert_eq!(point.creator(), creator);
        assert_eq!(point.tag(), LocalTag::FIRST);
    }

    #[test]
    fn references_from_distinct_creators_or_tags_are_distinct() {
        let a = EntityId::from_bits(1);
        let b = EntityId::from_bits(2);
        assert_ne!(
            EdgeRef::new(a, LocalTag::FIRST),
            EdgeRef::new(b, LocalTag::FIRST)
        );
        assert_ne!(
            EdgeRef::new(a, LocalTag::new(0)),
            EdgeRef::new(a, LocalTag::new(1))
        );
    }

    #[test]
    fn a_parameter_enforces_the_unit_interval_at_construction() {
        assert_eq!(Param::new(Rational::ZERO).unwrap(), Param::START);
        assert_eq!(Param::new(Rational::ONE).unwrap(), Param::END);
        assert!(Param::new(Rational::new(1, 2).unwrap()).is_ok());
        let err = Param::new(Rational::new(3, 2).unwrap()).unwrap_err();
        assert!(matches!(err, ParamError::OutOfRange { .. }), "got {err:?}");
        assert!(Param::new(Rational::new(-1, 2).unwrap()).is_err());
    }

    #[test]
    fn a_parameter_keeps_its_exact_rational() {
        let t = Rational::new(1, 3).unwrap();
        assert_eq!(Param::new(t).unwrap().as_rational(), t);
    }

    #[test]
    fn references_built_from_a_deterministic_generator_are_reproducible() {
        let mut g = DeterministicIdGenerator::new();
        let e1 = EdgeRef::new(g.next_id(), LocalTag::FIRST);
        let e2 = EdgeRef::new(g.next_id(), LocalTag::FIRST);
        let mut h = DeterministicIdGenerator::new();
        let f1 = EdgeRef::new(h.next_id(), LocalTag::FIRST);
        let f2 = EdgeRef::new(h.next_id(), LocalTag::FIRST);
        assert_eq!(
            (e1, e2),
            (f1, f2),
            "the same generator yields the same references"
        );
    }
}
