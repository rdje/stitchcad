//! Actual canonical declaration association;no source edit,point map or current-Design certificate.
use super::{AdmittedOffsetCandidates, OffsetCandidateFragment};
use crate::ontology::{EdgeRef, IdentityLedger, JournalEntry, LocalTag, TopologyEdit};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit complete binding allowances;earlier ledger/input storage and later execution are separate.
/// ```compile_fail
/// let limits:sc_core::construction::OffsetDeclarationLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffsetDeclarationLimits {
    roles: usize,
    journal: usize,
}
impl OffsetDeclarationLimits {
    /// Bound all original roles and the entire supplied journal,not a selected lookup prefix.
    #[must_use]
    pub const fn new(roles: usize, journal: usize) -> Self {
        Self { roles, journal }
    }
    /// Complete candidate-association allowance.
    #[must_use]
    pub const fn roles(self) -> usize {
        self.roles
    }
    /// Complete supplied journal-entry allowance.
    #[must_use]
    pub const fn journal_entries(self) -> usize {
        self.journal
    }
}
/// Complete admission or first actual declaration-association refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetDeclarationRefusal {
    /// The two N-entry workspace populations cannot be counted.
    WorkspaceOverflow,
    /// Complete original role population exceeds the selected policy.
    Roles {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Entire actual journal exceeds the selected policy before lookup.
    Journal {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// No actual record has the required operation id.
    MissingOperation,
    /// More than one actual record has that operation id;no first-match selection.
    AmbiguousOperation {
        /// Actual first journal index.
        first: usize,
        /// Actual next journal index.
        second: usize,
    },
    /// Actual uniquely named record is not Declared;destructive offset is not a substitute.
    Kind {
        /// Actual journal index,not a recipe statement ordinal.
        entry: usize,
    },
    /// Actual declaration has a different complete output population.
    Count {
        /// Actual journal index.
        entry: usize,
        /// Complete original candidate population.
        required: usize,
        /// Actual declaration population.
        actual: usize,
    },
    /// First actual original candidate tag has no matching field in the declaration.
    Tag {
        /// Actual journal index.
        entry: usize,
        /// Original candidate occurrence index.
        fragment: usize,
        /// Actual required explicit tag.
        tag: LocalTag,
    },
}
/// Original admitted owner,actual provided ledger,policy and cause survive every refusal.
pub struct OffsetDeclarationError<'a> {
    admitted: &'a AdmittedOffsetCandidates<'a>,
    ledger: &'a IdentityLedger,
    limits: OffsetDeclarationLimits,
    refusal: OffsetDeclarationRefusal,
}
impl<'a> OffsetDeclarationError<'a> {
    /// Actual admitted manifest and bound source/allowance.
    #[must_use]
    pub const fn admitted(&self) -> &'a AdmittedOffsetCandidates<'a> {
        self.admitted
    }
    /// Complete actual provided ledger,including original offending records.
    #[must_use]
    pub const fn ledger(&self) -> &'a IdentityLedger {
        self.ledger
    }
    /// Original explicit resource policy.
    #[must_use]
    pub const fn limits(&self) -> OffsetDeclarationLimits {
        self.limits
    }
    /// Complete admission or first actual source-association cause.
    #[must_use]
    pub const fn refusal(&self) -> OffsetDeclarationRefusal {
        self.refusal
    }
}
impl fmt::Debug for OffsetDeclarationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OffsetDeclarationError")
            .field("operation", &self.admitted.manifest().operation())
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for OffsetDeclarationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "offset declaration binding: {:?}", self.refusal)
    }
}
impl std::error::Error for OffsetDeclarationError<'_> {}
#[derive(Debug)]
struct Association<'a> {
    index: usize,
    fragment: &'a OffsetCandidateFragment,
    edge: &'a EdgeRef,
}

/// Complete actual declaration association,not liveness,global ownership,role continuity or geometry.
/// ```compile_fail
/// fn escape(admitted:sc_core::construction::AdmittedOffsetCandidates<'_>,ledger:sc_core::ontology::IdentityLedger)
///     ->sc_core::construction::DeclaredOffsetCandidates<'static> {
///     admitted.bind_declaration(&ledger,sc_core::construction::OffsetDeclarationLimits::new(1,1)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct DeclaredOffsetCandidates<'a> {
    admitted: &'a AdmittedOffsetCandidates<'a>,
    ledger: &'a IdentityLedger,
    entry: &'a JournalEntry,
    entry_index: usize,
    limits: OffsetDeclarationLimits,
    workspace_entries: usize,
    outputs: Vec<Association<'a>>,
}
impl<'a> DeclaredOffsetCandidates<'a> {
    /// Actual admitted manifest/input owner.
    #[must_use]
    pub const fn admitted(&self) -> &'a AdmittedOffsetCandidates<'a> {
        self.admitted
    }
    /// Actual ledger supplying the record;current Design membership is separate.
    #[must_use]
    pub const fn ledger(&self) -> &'a IdentityLedger {
        self.ledger
    }
    /// Actual immutable canonical Declared journal record.
    #[must_use]
    pub const fn entry(&self) -> &'a JournalEntry {
        self.entry
    }
    /// Actual journal position,not a recipe statement ordinal or persistent output identity.
    #[must_use]
    pub const fn entry_index(&self) -> usize {
        self.entry_index
    }
    /// Original explicit binding policy.
    #[must_use]
    pub const fn limits(&self) -> OffsetDeclarationLimits {
        self.limits
    }
    /// Complete N-entry tag index plus N-entry association population,not allocation bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Complete actual field pairs in original candidate order;declaration order is not rewritten.
    pub fn outputs(
        &self,
    ) -> impl ExactSizeIterator<Item = DeclaredOffsetCandidate<'_>> + DoubleEndedIterator {
        self.outputs
            .iter()
            .map(move |association| DeclaredOffsetCandidate {
                declaration: self,
                association,
            })
    }
}
/// One authentic original candidate/actual journal-edge association;no copied replacement fields.
/// ```compile_fail
/// fn forge<'a>(mut view:sc_core::construction::DeclaredOffsetCandidate<'a>,
///     declaration:&'a sc_core::construction::DeclaredOffsetCandidates<'a>)
///     ->sc_core::construction::DeclaredOffsetCandidate<'a> {
///     view.declaration=declaration;view
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct DeclaredOffsetCandidate<'a> {
    declaration: &'a DeclaredOffsetCandidates<'a>,
    association: &'a Association<'a>,
}
impl<'a> DeclaredOffsetCandidate<'a> {
    /// Actual complete bound declaration and its policy/source owners.
    #[must_use]
    pub const fn declaration(self) -> &'a DeclaredOffsetCandidates<'a> {
        self.declaration
    }
    /// Actual original candidate occurrence index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original manifest candidate field.
    #[must_use]
    pub const fn candidate(self) -> &'a OffsetCandidateFragment {
        self.association.fragment
    }
    /// Actual original EdgeRef field from the provided journal's Declared record.
    #[must_use]
    pub const fn edge(self) -> &'a EdgeRef {
        self.association.edge
    }
}
impl<'s> AdmittedOffsetCandidates<'s> {
    /// Admit complete N/2N roles/workspaces and J journal entries before record lookup/allocation.
    /// Match one actual operation record,Declared kind,count,then all explicit tags. Declaration
    /// tags need not be in candidate order. The two workspaces have N entries each;this grants no
    /// prior-storage,whole-allocation-byte,query-batch,geometry or publication resource certificate.
    /// Original declarations may now be historical;source/root/liveness/geometry need their own checks.
    /// # Errors
    /// Workspace count,role quota,journal quota,missing/ambiguous operation,kind,count,then first tag.
    /// No ledger/source edit or accepted association prefix is returned.
    pub fn bind_declaration<'a>(
        &'a self,
        ledger: &'a IdentityLedger,
        limits: OffsetDeclarationLimits,
    ) -> Result<DeclaredOffsetCandidates<'a>, OffsetDeclarationError<'a>>
    where
        's: 'a,
    {
        let admitted: &AdmittedOffsetCandidates<'a> = self;
        let refuse = |refusal| OffsetDeclarationError {
            admitted,
            ledger,
            limits,
            refusal,
        };
        let required = admitted.fragments().len();
        let workspace_entries = required
            .checked_mul(2)
            .ok_or_else(|| refuse(OffsetDeclarationRefusal::WorkspaceOverflow))?;
        let journal = ledger.journal();
        if required > limits.roles {
            return Err(refuse(OffsetDeclarationRefusal::Roles {
                required,
                allowed: limits.roles,
            }));
        }
        if journal.len() > limits.journal {
            return Err(refuse(OffsetDeclarationRefusal::Journal {
                required: journal.len(),
                allowed: limits.journal,
            }));
        }
        let mut records = journal
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.operation() == admitted.manifest().operation());
        let (entry_index, entry) = records
            .next()
            .ok_or_else(|| refuse(OffsetDeclarationRefusal::MissingOperation))?;
        if let Some((second, _)) = records.next() {
            return Err(refuse(OffsetDeclarationRefusal::AmbiguousOperation {
                first: entry_index,
                second,
            }));
        }
        let TopologyEdit::Declared { edges } = entry.edit() else {
            return Err(refuse(OffsetDeclarationRefusal::Kind {
                entry: entry_index,
            }));
        };
        if edges.len() != required {
            return Err(refuse(OffsetDeclarationRefusal::Count {
                entry: entry_index,
                required,
                actual: edges.len(),
            }));
        }
        // Both ledger declaration constructors retain unique tags and their actual creating id.
        // Private journal construction preserves that per-entry invariant,even if a caller's
        // external id streams violate global nonreuse and create the ambiguous case checked above.
        let by_tag = edges
            .iter()
            .map(|edge| (edge.tag(), edge))
            .collect::<BTreeMap<_, _>>();
        let mut outputs = Vec::with_capacity(required);
        for (index, fragment) in admitted.fragments().iter().enumerate() {
            let edge = by_tag.get(&fragment.tag()).copied().ok_or_else(|| {
                refuse(OffsetDeclarationRefusal::Tag {
                    entry: entry_index,
                    fragment: index,
                    tag: fragment.tag(),
                })
            })?;
            outputs.push(Association {
                index,
                fragment,
                edge,
            });
        }
        Ok(DeclaredOffsetCandidates {
            admitted,
            ledger,
            entry,
            entry_index,
            limits,
            workspace_entries,
            outputs,
        })
    }
}
