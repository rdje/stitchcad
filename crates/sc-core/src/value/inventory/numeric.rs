//! Genuine numeric target membership;no formula metadata,source choice or numeric reads.
use super::{
    CanonicalInputPosition, CanonicalInputTarget, CanonicalInputs, LengthDeclaration,
    ScalarInputDeclaration, ScalarInputKind,
};
use core::fmt;

/// Original numeric member view. Public construction is raw data,not an inventory certificate;
/// the private CanonicalNumericInputBinding retains the actual membership association.
#[derive(Clone, Copy)]
pub enum CanonicalNumericInputMember<'a> {
    /// Actual dedicated length and original authored position.
    Length {
        /// Actual zero-based original length position.
        index: usize,
        /// Actual original immutable record.
        declaration: &'a LengthDeclaration,
    },
    /// Actual general scalar,including a Length-kind scalar,and original authored position.
    Scalar {
        /// Actual zero-based original scalar position.
        index: usize,
        /// Actual original immutable record.
        declaration: &'a ScalarInputDeclaration,
    },
}
impl CanonicalNumericInputMember<'_> {
    /// Original family and position;the position is not an entity identity.
    #[must_use]
    pub const fn position(self) -> CanonicalInputPosition {
        match self {
            Self::Length {
                index,
                declaration: _,
            } => CanonicalInputPosition::Length { index },
            Self::Scalar {
                index,
                declaration: _,
            } => CanonicalInputPosition::Scalar { index },
        }
    }
}
impl fmt::Debug for CanonicalNumericInputMember<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (family, identity) = match self {
            Self::Length {
                index: _,
                declaration,
            } => ("Length", declaration.id()),
            Self::Scalar {
                index: _,
                declaration,
            } => ("Scalar", declaration.id()),
        };
        f.debug_struct(family)
            .field("position", &self.position())
            .field("declaration", &identity)
            .finish()
    }
}
/// Original first representation/missing/kind cause. Other-family positions borrow their actual
/// containing inventory through the error;no family conversion or alternate member is selected.
#[derive(Clone, Copy)]
pub enum CanonicalNumericInputRefusal<'a> {
    /// Requested dedicated length is absent,including an actual scalar/logical identity mismatch.
    MissingLength {
        /// Original other-family position,if this identity is owned elsewhere.
        other: Option<CanonicalInputPosition>,
    },
    /// Requested general scalar is absent,including an actual dedicated-length/logical mismatch.
    MissingScalar {
        /// Original other-family position,if this identity is owned elsewhere.
        other: Option<CanonicalInputPosition>,
    },
    /// Original actual scalar member has another declared kind. The original record is retained.
    ScalarKind {
        /// Original expected scalar kind.
        expected: ScalarInputKind,
        /// Actual original scalar position.
        index: usize,
        /// Original actual member,without copied state or value.
        actual: &'a ScalarInputDeclaration,
    },
}
impl fmt::Debug for CanonicalNumericInputRefusal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLength { other } => f
                .debug_struct("MissingLength")
                .field("other", other)
                .finish(),
            Self::MissingScalar { other } => f
                .debug_struct("MissingScalar")
                .field("other", other)
                .finish(),
            Self::ScalarKind {
                expected,
                index,
                actual,
            } => f
                .debug_struct("ScalarKind")
                .field("expected", expected)
                .field("index", index)
                .field("actual_kind", &actual.kind())
                .finish(),
        }
    }
}
/// Original requested target,actual immutable inventory and typed lending refusal.
#[derive(Clone, Copy)]
pub struct CanonicalNumericInputError<'a> {
    inputs: &'a CanonicalInputs,
    target: CanonicalInputTarget,
    refusal: CanonicalNumericInputRefusal<'a>,
}
impl<'a> CanonicalNumericInputError<'a> {
    /// Exact supplied original inventory,not an equal-data substitute.
    #[must_use]
    pub const fn inputs(self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Exact original requested numeric representation and scalar kind.
    #[must_use]
    pub const fn target(self) -> CanonicalInputTarget {
        self.target
    }
    /// Original missing/family or kind cause with original positions/member loans.
    #[must_use]
    pub const fn refusal(self) -> CanonicalNumericInputRefusal<'a> {
        self.refusal
    }
}
impl fmt::Debug for CanonicalNumericInputError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CanonicalNumericInputError")
            .field("target", &self.target)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for CanonicalNumericInputError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "canonical numeric target: {self:?}")
    }
}
// The actual scalar cause borrows its inventory;refusal() exposes it without erasing its lifetime.
impl std::error::Error for CanonicalNumericInputError<'_> {}

/// Actual original numeric target-to-inventory association. No formula name,origin,metadata input
/// ID,selected value,Root/Profile source,Derived supplier or work allowance is manufactured.
/// ```compile_fail,E0616
/// fn alter<'a>(p:&mut sc_core::value::CanonicalNumericInputBinding<'a>,other:&'a sc_core::value::CanonicalInputs) {p.inputs=other;}
/// ```
/// ```compile_fail
/// use sc_core::value::*;
/// fn escape(inputs:&CanonicalInputs,target:CanonicalInputTarget)->CanonicalNumericInputBinding<'static> {inputs.resolve_numeric(target).unwrap()}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct CanonicalNumericInputBinding<'a> {
    inputs: &'a CanonicalInputs,
    target: CanonicalInputTarget,
    member: CanonicalNumericInputMember<'a>,
}
impl<'a> CanonicalNumericInputBinding<'a> {
    /// Exact original canonical owner supplying this member.
    #[must_use]
    pub const fn inputs(self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Original requested representation and kind,retained without formula conversion.
    #[must_use]
    pub const fn target(self) -> CanonicalInputTarget {
        self.target
    }
    /// Actual original member and its original position. All state/provenance remains there.
    #[must_use]
    pub const fn member(self) -> CanonicalNumericInputMember<'a> {
        self.member
    }
    /// Actual original family/position,not an identity or ordered program location.
    #[must_use]
    pub const fn position(self) -> CanonicalInputPosition {
        self.member.position()
    }
}
impl CanonicalInputs {
    /// Resolve a genuine original numeric target directly,without inventing formula metadata.
    /// Dedicated lengths and general Length-kind scalars remain distinct. Exact scalar kind
    /// precedes success,including Unknown/Derived records. No value/state read,allocation,clone,
    /// source/currentness lookup or dependency-order acceptance occurs. The containing checker
    /// owns complete fresh content and aggregate lookup work before calling this primitive.
    /// ```
    /// use sc_core::{value::*,ontology::EntityId};
    /// let id=EntityId::from_bits(1);
    /// let record=LengthDeclaration::new(LengthDeclarationDefinition {
    ///     id,source:EntityId::from_bits(2),state:LengthState::Unknown {observation:EntityId::from_bits(3)},
    /// })?;
    /// let inputs=CanonicalInputs::new(vec![record],vec![])?;
    /// let binding=inputs.resolve_numeric(CanonicalInputTarget::Length(id)).map_err(|e|e.to_string())?;
    /// assert!(std::ptr::eq(binding.inputs(),&inputs));
    /// assert_eq!(binding.position(),CanonicalInputPosition::Length {index:0});
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Original missing representation with actual other-family position,or first actual scalar
    /// kind mismatch. Both retain exact supplied inventory and requested target/member witnesses.
    pub fn resolve_numeric(
        &self,
        target: CanonicalInputTarget,
    ) -> Result<CanonicalNumericInputBinding<'_>, CanonicalNumericInputError<'_>> {
        let refuse = |refusal| CanonicalNumericInputError {
            inputs: self,
            target,
            refusal,
        };
        let member = match target {
            CanonicalInputTarget::Length(identity) => {
                let (index, declaration) = self
                    .lengths()
                    .iter()
                    .enumerate()
                    .find(|(_, d)| d.id() == identity)
                    .ok_or_else(|| {
                        refuse(CanonicalNumericInputRefusal::MissingLength {
                            other: self
                                .scalars()
                                .iter()
                                .position(|d| d.id() == identity)
                                .map(|index| CanonicalInputPosition::Scalar { index })
                                .or_else(|| {
                                    self.logicals()
                                        .declarations()
                                        .iter()
                                        .position(|d| d.id() == identity)
                                        .map(|index| CanonicalInputPosition::Logical { index })
                                }),
                        })
                    })?;
                CanonicalNumericInputMember::Length { index, declaration }
            }
            CanonicalInputTarget::Scalar {
                declaration: identity,
                kind,
            } => {
                let (index, declaration) = self
                    .scalars()
                    .iter()
                    .enumerate()
                    .find(|(_, d)| d.id() == identity)
                    .ok_or_else(|| {
                        refuse(CanonicalNumericInputRefusal::MissingScalar {
                            other: self
                                .lengths()
                                .iter()
                                .position(|d| d.id() == identity)
                                .map(|index| CanonicalInputPosition::Length { index })
                                .or_else(|| {
                                    self.logicals()
                                        .declarations()
                                        .iter()
                                        .position(|d| d.id() == identity)
                                        .map(|index| CanonicalInputPosition::Logical { index })
                                }),
                        })
                    })?;
                if declaration.kind() != kind {
                    return Err(refuse(CanonicalNumericInputRefusal::ScalarKind {
                        expected: kind,
                        index,
                        actual: declaration,
                    }));
                }
                CanonicalNumericInputMember::Scalar { index, declaration }
            }
        };
        Ok(CanonicalNumericInputBinding {
            inputs: self,
            target,
            member,
        })
    }
}
