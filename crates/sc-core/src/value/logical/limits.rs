//! Complete original held content and conservative allocation-free comparison work.
use super::{LogicalInputDeclarationDefinition, LogicalInputRefusal};

/// Explicit constructor allowances. These are logical populations and comparisons, not allocator
/// bytes/capacity, string-comparison byte work, CPU time or runtime execution limits.
/// ```compile_fail
/// let policy:sc_core::value::LogicalInputLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalInputLimits {
    domain: usize,
    evidence: usize,
    text: usize,
    inspections: usize,
}
impl LogicalInputLimits {
    /// Select complete domain/evidence/text/comparison allowances explicitly, including zero.
    #[must_use]
    pub const fn new(
        domain_members: usize,
        evidence_records: usize,
        text_bytes: usize,
        validation_inspections: usize,
    ) -> Self {
        Self {
            domain: domain_members,
            evidence: evidence_records,
            text: text_bytes,
            inspections: validation_inspections,
        }
    }
    /// Maximum complete original domain entries.
    #[must_use]
    pub const fn domain_members(self) -> usize {
        self.domain
    }
    /// Maximum complete original Known evidence entries.
    #[must_use]
    pub const fn evidence_records(self) -> usize {
        self.evidence
    }
    /// Maximum domain text plus the separately owned selection text, in UTF-8 bytes.
    #[must_use]
    pub const fn text_bytes(self) -> usize {
        self.text
    }
    /// Maximum conservative D*D+E*E+D member/identity comparisons.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
}
/// Complete actual owned text/populations and conservative comparison-work bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalInputShape {
    domain: usize,
    evidence: usize,
    text: usize,
    inspections: usize,
}
impl LogicalInputShape {
    /// Complete original domain entries, including repeated invalid members.
    #[must_use]
    pub const fn domain_members(self) -> usize {
        self.domain
    }
    /// Complete original Known evidence entries, including repeated invalid references.
    #[must_use]
    pub const fn evidence_records(self) -> usize {
        self.evidence
    }
    /// Actual separately owned domain/selection text bytes, without deduplication.
    #[must_use]
    pub const fn text_bytes(self) -> usize {
        self.text
    }
    /// Conservative D*D+E*E+D comparison requirement, not measured CPU work.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
    pub(super) fn admit(
        definition: &LogicalInputDeclarationDefinition,
        limits: LogicalInputLimits,
    ) -> Result<Self, LogicalInputRefusal> {
        use LogicalInputRefusal as R;
        let domain = definition.domain.len();
        let evidence = definition.state.evidence().len();
        let text = definition
            .domain
            .iter()
            .try_fold(0usize, |n, member| {
                n.checked_add(member.as_str().len())
                    .ok_or(R::PopulationOverflow)
            })?
            .checked_add(
                definition
                    .state
                    .selected()
                    .map_or(0, |member| member.as_str().len()),
            )
            .ok_or(R::PopulationOverflow)?;
        let inspections = domain
            .checked_mul(domain)
            .and_then(|n| {
                evidence
                    .checked_mul(evidence)
                    .and_then(|e| n.checked_add(e))
            })
            .and_then(|n| n.checked_add(domain))
            .ok_or(R::PopulationOverflow)?;
        if domain > limits.domain {
            return Err(R::DomainMembers {
                allowed: limits.domain,
                required: domain,
            });
        }
        if evidence > limits.evidence {
            return Err(R::EvidenceRecords {
                allowed: limits.evidence,
                required: evidence,
            });
        }
        if text > limits.text {
            return Err(R::TextBytes {
                allowed: limits.text,
                required: text,
            });
        }
        if inspections > limits.inspections {
            return Err(R::ValidationInspections {
                allowed: limits.inspections,
                required: inspections,
            });
        }
        Ok(Self {
            domain,
            evidence,
            text,
            inspections,
        })
    }
}
