//! Size identities, human labels, authored order and base selection; chart/axes are separate contracts.
use core::fmt;
use sc_core::ontology::EntityId;
use sc_units::Count;
use std::collections::BTreeSet;

/// Pinned SizeSet identity and authored revision; not a current-registry certificate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SizeSetReference {
    /// Stable referenced set identity.
    pub id: EntityId,
    /// Exact authored revision, including initial revision zero.
    pub revision: Count,
}
impl SizeSetReference {
    /// Produce a successor retaining identity, without wrapping or mutating the original reference.
    /// Command/registry validation must separately enforce currentness and authorized transitions.
    /// # Errors
    /// Returns [`SizeMembershipError::RevisionOverflow`] at the Count limit.
    pub fn next_revision(self) -> Result<Self, SizeMembershipError> {
        let revision = self
            .revision
            .get()
            .checked_add(1)
            .ok_or(SizeMembershipError::RevisionOverflow(self))?;
        Ok(Self {
            id: self.id,
            revision: Count::new(revision),
        })
    }
}
/// Authored designation-system selection, without imported standards mapping data or defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SizeSystem {
    /// EN 13402 designation intent; content/mapping verification remains separately owned.
    En13402,
    /// ASTM D5585 designation intent; content/mapping verification remains separately owned.
    AstmD5585,
    /// Human labels such as S, M and L; no arithmetic or dimensions implied.
    Alphanumeric,
    /// Numeric-looking labels remain names, not measurements or instantiation order.
    Numeric,
    /// Authored house/client designations, including a single-member bespoke range.
    Custom,
}
/// Validated nonblank human label, preserving exact Unicode/case/spacing without token semantics.
/// ```compile_fail
/// fn replace(label: &mut sc_measure::SizeLabel) { label.0 = String::new(); }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SizeLabel(String);
impl SizeLabel {
    /// Preserve authored bytes after checking that the label is not entirely whitespace.
    /// # Errors
    /// Returns [`SizeMembershipError::BlankLabel`] for a blank label.
    pub fn new(label: String) -> Result<Self, SizeMembershipError> {
        if label.trim().is_empty() {
            return Err(SizeMembershipError::BlankLabel);
        }
        Ok(Self(label))
    }
    /// Exact human-facing label; no conversion to a machine token, quantity or measurement.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for SizeLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
/// Stable member identity and human designation; vector position never supplies identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeMember {
    /// Stable member identity, distinct from its owning set and peer members.
    pub id: EntityId,
    /// Human-readable name with no measurement or quantity implied.
    pub label: SizeLabel,
}
/// Authored membership foundation, awaiting separately validated axes/chart/break contracts.
/// ```compile_fail
/// fn quantities(definition: &mut sc_measure::SizeMembershipDefinition) {
///     definition.quantities = vec![sc_units::Count::new(1)];
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeMembershipDefinition {
    /// Referenced owning set identity and exact authored revision.
    pub reference: SizeSetReference,
    /// Declared designation system; it does not supply measurements.
    pub system: SizeSystem,
    /// Nonempty authored instantiation order; unique member identities and exact labels.
    pub members: Vec<SizeMember>,
    /// Exactly one member identity, independent of label spelling or vector position.
    pub base: EntityId,
}
/// Immutable validated membership foundation; not yet a complete SizeSet or path-readiness proof.
/// ```compile_fail
/// fn reorder(membership: &mut sc_measure::SizeMembership) { membership.definition.members.reverse(); }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeMembership {
    definition: SizeMembershipDefinition,
}
/// Membership/label/revision refusal; no missing base or label is inferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeMembershipError {
    /// Labels must contain non-whitespace human content.
    BlankLabel,
    /// Exactly one base requires at least one member.
    EmptyMembers(EntityId),
    /// Member identity repeats or aliases its owning set.
    DuplicateIdentity(EntityId),
    /// Two members carry the same exact label in one set.
    DuplicateLabel {
        /// Owning set.
        set: EntityId,
        /// Repeated authored label.
        label: SizeLabel,
    },
    /// Authored base is not a member.
    MissingBase {
        /// Owning set.
        set: EntityId,
        /// Missing base identity.
        base: EntityId,
    },
    /// Queried member identity is absent.
    MissingMember {
        /// Owning set.
        set: EntityId,
        /// Requested member.
        member: EntityId,
    },
    /// Queried exact label is absent; labels are not parsed or normalized.
    MissingLabel {
        /// Owning set.
        set: EntityId,
        /// Requested label.
        label: SizeLabel,
    },
    /// A successor would wrap the Count revision domain.
    RevisionOverflow(SizeSetReference),
}
impl fmt::Display for SizeMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankLabel => write!(f, "size label must be nonblank"),
            Self::EmptyMembers(set) => write!(f, "size set {set} needs at least one member"),
            Self::DuplicateIdentity(id) => write!(f, "size membership reuses identity {id}"),
            Self::DuplicateLabel { set, label } => {
                write!(f, "size set {set} repeats label {label:?}")
            }
            Self::MissingBase { set, base } => {
                write!(f, "size set {set} base {base} is not a member")
            }
            Self::MissingMember { set, member } => {
                write!(f, "size set {set} has no member {member}")
            }
            Self::MissingLabel { set, label } => {
                write!(f, "size set {set} has no exact label {label:?}")
            }
            Self::RevisionOverflow(reference) => write!(
                f,
                "size set {} revision {} cannot advance",
                reference.id, reference.revision
            ),
        }
    }
}
impl std::error::Error for SizeMembershipError {}
impl SizeMembership {
    /// Validate nonempty authored order, unique identities/labels and explicit base membership.
    /// Does not sort, parse labels, or derive measurements or axes.
    /// # Errors
    /// Returns [`SizeMembershipError`] for empty/ambiguous members or absent base.
    pub fn new(definition: SizeMembershipDefinition) -> Result<Self, SizeMembershipError> {
        let set = definition.reference.id;
        if definition.members.is_empty() {
            return Err(SizeMembershipError::EmptyMembers(set));
        }
        let mut identities = BTreeSet::from([set]);
        let mut labels = BTreeSet::new();
        for member in &definition.members {
            if !identities.insert(member.id) {
                return Err(SizeMembershipError::DuplicateIdentity(member.id));
            }
            if !labels.insert(member.label.as_str()) {
                return Err(SizeMembershipError::DuplicateLabel {
                    set,
                    label: member.label.clone(),
                });
            }
        }
        if !definition
            .members
            .iter()
            .any(|member| member.id == definition.base)
        {
            return Err(SizeMembershipError::MissingBase {
                set,
                base: definition.base,
            });
        }
        Ok(Self { definition })
    }
    /// Borrow validated content without unchecked mutation; caller owns registry/revision checks.
    #[must_use]
    pub const fn definition(&self) -> &SizeMembershipDefinition {
        &self.definition
    }
    /// Exact pinned owning reference.
    #[must_use]
    pub const fn reference(&self) -> SizeSetReference {
        self.definition.reference
    }
    /// Borrow one member by stable identity, independent of current position or label spelling.
    /// # Errors
    /// Returns [`SizeMembershipError::MissingMember`] for an absent identity.
    pub fn member(&self, member: EntityId) -> Result<&SizeMember, SizeMembershipError> {
        self.definition
            .members
            .iter()
            .find(|value| value.id == member)
            .ok_or(SizeMembershipError::MissingMember {
                set: self.reference().id,
                member,
            })
    }
    /// Borrow one member by exact authored label; no numeric or token interpretation.
    /// # Errors
    /// Returns [`SizeMembershipError::MissingLabel`] for an absent exact label.
    pub fn member_by_label(&self, label: &SizeLabel) -> Result<&SizeMember, SizeMembershipError> {
        self.definition
            .members
            .iter()
            .find(|member| &member.label == label)
            .ok_or_else(|| SizeMembershipError::MissingLabel {
                set: self.reference().id,
                label: label.clone(),
            })
    }
    /// Borrow the authored base member without positional or label-derived selection.
    /// # Errors
    /// Returns [`SizeMembershipError::MissingMember`] if the base identity is absent.
    /// Validated immutable membership always contains its base; the fallible query avoids a panic.
    pub fn base(&self) -> Result<&SizeMember, SizeMembershipError> {
        self.member(self.definition.base)
    }
}
