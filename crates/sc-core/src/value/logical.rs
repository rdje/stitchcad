//! Immutable authored finite logical selections, separate from scalar formulas and physical values.
use crate::{name::MachineToken, ontology::EntityId};
use core::fmt;
mod limits;
pub use limits::{LogicalInputLimits, LogicalInputShape};

/// Logical source role; neither variant is a scalar formula kind or a physical specification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicalInputKind {
    /// Authored opening-type domain; core invents no supported physical opening vocabulary.
    PocketOpening,
    /// Authored hardware designation domain; no diameter or dimensions are inferred from its code.
    NotionSize,
}
/// Authored logical state label; a design choice is distinct from an unknown factual input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicalInputStateTag {
    /// Claimed known selection with scoped-evidence references.
    Known,
    /// Explicit human assumption.
    Assumed,
    /// Observation still required; no selection or fallback exists.
    Unknown,
    /// Explicit designer selection, without promoting it to a known fact.
    Choice,
    /// Explicit overridable preference.
    Preference,
    /// Sole semantic computation source, without an authored or cached result.
    Derived,
}
/// Authored selection/provenance. Unknown and Derived structurally cannot carry a result.
/// ```compile_fail,E0559
/// use sc_core::{value::LogicalInputState,name::MachineToken,ontology::EntityId};
/// fn unknown(value:MachineToken)->LogicalInputState {
///     LogicalInputState::Unknown {observation:EntityId::from_bits(1),value}
/// }
/// ```
/// ```compile_fail,E0559
/// use sc_core::{value::LogicalInputState,name::MachineToken,ontology::EntityId};
/// fn derived(value:MachineToken)->LogicalInputState {
///     LogicalInputState::Derived {operation:EntityId::from_bits(1),value}
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub enum LogicalInputState {
    /// Claimed known domain member; evidence truth/existence remains a separate source obligation.
    Known {
        /// Original authored member, without interpreting hardware or opening geometry.
        value: MachineToken,
        /// Required nonempty distinct scoped-evidence references.
        evidence: Vec<EntityId>,
    },
    /// Explicit assumed domain member, without a known-fact claim.
    Assumed {
        /// Original assumed member.
        value: MachineToken,
        /// Original human-assumption reference, not independent actor/approval proof.
        assumption: EntityId,
    },
    /// Unknown fact with no domain member or default.
    Unknown {
        /// Original observation request.
        observation: EntityId,
    },
    /// Explicit selected design choice, not an observed fact or unknown fallback.
    Choice {
        /// Original chosen domain member.
        value: MachineToken,
        /// Original design-decision provenance.
        decision: EntityId,
    },
    /// Explicit preference, not substitution over a separately unknown fact.
    Preference {
        /// Original preferred domain member.
        value: MachineToken,
        /// Original preference provenance.
        provenance: EntityId,
    },
    /// Sole semantic computation source; formula syntax does not gain enum-valued operands.
    Derived {
        /// Original typed semantic operation reference, to be bound by recipe integration.
        operation: EntityId,
    },
}
impl LogicalInputState {
    /// Original label, without reading a selected member or resolving referenced records.
    #[must_use]
    pub const fn tag(&self) -> LogicalInputStateTag {
        match self {
            Self::Known { .. } => LogicalInputStateTag::Known,
            Self::Assumed { .. } => LogicalInputStateTag::Assumed,
            Self::Unknown { .. } => LogicalInputStateTag::Unknown,
            Self::Choice { .. } => LogicalInputStateTag::Choice,
            Self::Preference { .. } => LogicalInputStateTag::Preference,
            Self::Derived { .. } => LogicalInputStateTag::Derived,
        }
    }
    fn selected(&self) -> Option<&MachineToken> {
        match self {
            Self::Known { value, .. }
            | Self::Assumed { value, .. }
            | Self::Choice { value, .. }
            | Self::Preference { value, .. } => Some(value),
            Self::Unknown { .. } | Self::Derived { .. } => None,
        }
    }
    fn evidence(&self) -> &[EntityId] {
        match self {
            Self::Known { evidence, .. } => evidence,
            _ => &[],
        }
    }
}
impl fmt::Debug for LogicalInputState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tag().fmt(f)
    }
}
/// Editable logical declaration. Domain members and any selection are original machine identifiers,
/// not localized labels, vendor-code normalization, scalar strings or generated geometry.
#[derive(Clone, PartialEq, Eq)]
pub struct LogicalInputDeclarationDefinition {
    /// Original canonical declaration identity; actual registry membership is checked separately.
    pub id: EntityId,
    /// Original source-record reference, without an existence or provenance-truth certificate.
    pub source: EntityId,
    /// Original typed logical role.
    pub kind: LogicalInputKind,
    /// Original nonempty distinct finite domain in authored order; core supplies no vocabulary.
    pub domain: Vec<MachineToken>,
    /// Exactly one authored factual/choice/preference/derived state and its required provenance.
    pub state: LogicalInputState,
}
impl fmt::Debug for LogicalInputDeclarationDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogicalInputDeclarationDefinition")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("domain_members", &self.domain.len())
            .field("state", &self.state.tag())
            .finish_non_exhaustive()
    }
}
/// First complete shape/quota or original domain/state field refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicalInputRefusal {
    /// Complete owned-text or conservative comparison arithmetic does not fit usize.
    PopulationOverflow,
    /// Complete domain population exceeds its selected allowance.
    DomainMembers {
        /// Selected maximum.
        allowed: usize,
        /// Complete requirement.
        required: usize,
    },
    /// Complete original known-state evidence population exceeds its selected allowance.
    EvidenceRecords {
        /// Selected maximum.
        allowed: usize,
        /// Complete requirement.
        required: usize,
    },
    /// Complete domain plus separately owned selection text exceeds its selected allowance.
    TextBytes {
        /// Selected maximum.
        allowed: usize,
        /// Complete requirement.
        required: usize,
    },
    /// Complete conservative D*D+E*E+D comparison bound exceeds its selected allowance.
    ValidationInspections {
        /// Selected maximum.
        allowed: usize,
        /// Complete requirement.
        required: usize,
    },
    /// Every logical declaration needs its original possible domain, even while Unknown/Derived.
    EmptyDomain,
    /// Original repeated domain member, without sorting or silently collapsing it.
    DuplicateDomain {
        /// First zero-based authored member.
        first: usize,
        /// First repeated zero-based authored member.
        duplicate: usize,
    },
    /// Authored selection is not in this actual domain; payload text stays private.
    SelectionOutsideDomain {
        /// Original selection-bearing state.
        state: LogicalInputStateTag,
    },
    /// A claimed Known selection has no scoped-evidence reference.
    NoEvidence,
    /// A Known declaration repeats an evidence reference.
    DuplicateEvidence {
        /// First original evidence position.
        first: usize,
        /// First repeated evidence position.
        duplicate: usize,
        /// Original repeated identity.
        evidence: EntityId,
    },
}
/// Original declaration identity and first exact refusal; no partial declaration is returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalInputDeclarationError {
    /// Actual refused declaration identity.
    pub declaration: EntityId,
    /// First complete-bound or original field/index cause.
    pub refusal: LogicalInputRefusal,
}
impl fmt::Display for LogicalInputDeclarationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "logical declaration {}: {:?}",
            self.declaration, self.refusal
        )
    }
}
impl std::error::Error for LogicalInputDeclarationError {}
/// Immutable structurally validated finite logical declaration. This certifies no canonical Root
/// membership, actual Profile pin, evidence truth, operation binding, physical scope or readiness.
/// ```compile_fail,E0616
/// fn alter(record:&mut sc_core::value::LogicalInputDeclaration) {record.definition.domain.clear();}
/// ```
/// ```compile_fail,E0308
/// fn scalar(record:sc_core::value::LogicalInputDeclaration)->sc_core::value::ScalarInputDeclaration {record}
/// ```
#[derive(Clone)]
pub struct LogicalInputDeclaration {
    definition: LogicalInputDeclarationDefinition,
    limits: LogicalInputLimits,
    shape: LogicalInputShape,
}
// Constructor allowances are proof metadata; they do not change authored semantic equality.
impl PartialEq for LogicalInputDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.definition == other.definition
    }
}
impl Eq for LogicalInputDeclaration {}
impl fmt::Debug for LogicalInputDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogicalInputDeclaration")
            .field("definition", &self.definition)
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl LogicalInputDeclaration {
    /// Admit complete content/comparison bounds before domain/state validation, using no validation
    /// workspace allocation. Existing owned input vectors are supplied by the caller, not budgeted
    /// allocator capacity. Checks preserve original domain/evidence order and referenced provenance.
    ///
    /// ```
    /// use sc_core::{name::MachineToken,ontology::EntityId,value::*};
    /// # fn main()->Result<(),Box<dyn std::error::Error>> {
    /// let record=LogicalInputDeclaration::new(LogicalInputDeclarationDefinition {
    ///     id:EntityId::from_bits(1),source:EntityId::from_bits(2),
    ///     kind:LogicalInputKind::PocketOpening,
    ///     domain:vec![MachineToken::new("option_a")?,MachineToken::new("option_b")?],
    ///     state:LogicalInputState::Choice {value:MachineToken::new("option_b")?,decision:EntityId::from_bits(3)},
    /// },LogicalInputLimits::new(2,0,24,6))?;
    /// assert_eq!(record.definition().state.tag(),LogicalInputStateTag::Choice);
    /// # Ok(()) }
    /// ```
    /// # Errors
    /// Population arithmetic, domain/evidence/text/comparison quotas, empty/duplicate domain,
    /// selection membership, then missing/duplicate Known evidence, in that order.
    pub fn new(
        definition: LogicalInputDeclarationDefinition,
        limits: LogicalInputLimits,
    ) -> Result<Self, LogicalInputDeclarationError> {
        let refuse = |refusal| LogicalInputDeclarationError {
            declaration: definition.id,
            refusal,
        };
        let shape = LogicalInputShape::admit(&definition, limits).map_err(refuse)?;
        if definition.domain.is_empty() {
            return Err(refuse(LogicalInputRefusal::EmptyDomain));
        }
        for (duplicate, member) in definition.domain.iter().enumerate() {
            if let Some(first) = definition
                .domain
                .iter()
                .take(duplicate)
                .position(|prior| prior == member)
            {
                return Err(refuse(LogicalInputRefusal::DuplicateDomain {
                    first,
                    duplicate,
                }));
            }
        }
        if definition
            .state
            .selected()
            .is_some_and(|value| !definition.domain.contains(value))
        {
            return Err(refuse(LogicalInputRefusal::SelectionOutsideDomain {
                state: definition.state.tag(),
            }));
        }
        if matches!(definition.state, LogicalInputState::Known { .. })
            && definition.state.evidence().is_empty()
        {
            return Err(refuse(LogicalInputRefusal::NoEvidence));
        }
        for (duplicate, evidence) in definition.state.evidence().iter().enumerate() {
            if let Some(first) = definition
                .state
                .evidence()
                .iter()
                .take(duplicate)
                .position(|prior| prior == evidence)
            {
                return Err(refuse(LogicalInputRefusal::DuplicateEvidence {
                    first,
                    duplicate,
                    evidence: *evidence,
                }));
            }
        }
        Ok(Self {
            definition,
            limits,
            shape,
        })
    }
    /// Original immutable authored definition, including actual domain/state/provenance loans.
    #[must_use]
    pub const fn definition(&self) -> &LogicalInputDeclarationDefinition {
        &self.definition
    }
    /// Actual original declaration identity; this does not certify an inventory association.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Original admitted logical role, without a formula operand kind.
    #[must_use]
    pub const fn kind(&self) -> LogicalInputKind {
        self.definition.kind
    }
    /// Original explicit constructor allowances, not runtime or canonical source policies.
    #[must_use]
    pub const fn limits(&self) -> LogicalInputLimits {
        self.limits
    }
    /// Complete original held populations and conservative comparison allowance requirement.
    #[must_use]
    pub const fn shape(&self) -> LogicalInputShape {
        self.shape
    }
}

mod inventory;
pub use inventory::{
    LogicalInputBinding, LogicalInputInventoryError, LogicalInputInventoryLimits,
    LogicalInputInventoryRefusal, LogicalInputInventoryShape, LogicalInputLookupError,
    LogicalInputs,
};
