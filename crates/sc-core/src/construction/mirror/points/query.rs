//! Original PointRef candidate eligibility,not geometric/current-source correspondence.
use super::{AdmittedMirrorPiecePoints, MirrorPiecePoint};
use crate::ontology::{LocalTag, PointRef};
use core::fmt;

/// Explicit original source identity query;the same creator with a different tag is different.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPiecePointQuery {
    source: PointRef,
}
impl MirrorPiecePointQuery {
    /// Retain the actual original source field without resolving,rewriting or transforming it.
    #[must_use]
    pub const fn new(source: PointRef) -> Self {
        Self { source }
    }
    /// Actual original query field,not a current coordinate or an output lookup.
    #[must_use]
    pub const fn source(&self) -> &PointRef {
        &self.source
    }
}
/// Explicit whole candidate-scan and complete match allowances,without defaults.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPiecePointQueryLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPiecePointQueryLimits {
    candidates: usize,
    matches: usize,
}
impl MirrorPiecePointQueryLimits {
    /// Bound all actual candidates,including unrelated sources,and the complete resulting matches.
    #[must_use]
    pub const fn new(candidates: usize, matches: usize) -> Self {
        Self {
            candidates,
            matches,
        }
    }
    /// Whole actual candidate scan allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Whole resulting match allowance,not a prefix/truncation policy.
    #[must_use]
    pub const fn matches(self) -> usize {
        self.matches
    }
}
/// Whole source-scan or complete match refusal;no prefix result escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPiecePointQueryRefusal {
    /// Whole actual candidate population exceeds policy before scanning any source.
    Candidates {
        /// Complete actual candidates.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete actual matches exceed policy before a result view is lent.
    Matches {
        /// Complete matching candidates.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
}
/// Original owner/request/policy and complete count refusal remain inspectable.
pub struct MirrorPiecePointQueryError<'a> {
    points: &'a AdmittedMirrorPiecePoints<'a>,
    query: &'a MirrorPiecePointQuery,
    limits: MirrorPiecePointQueryLimits,
    refusal: MirrorPiecePointQueryRefusal,
}
impl<'a> MirrorPiecePointQueryError<'a> {
    /// Actual whole admitted candidate/role/source/identity owner.
    #[must_use]
    pub const fn points(&self) -> &'a AdmittedMirrorPiecePoints<'a> {
        self.points
    }
    /// Actual original query,including its exact creator/tag field.
    #[must_use]
    pub const fn query(&self) -> &'a MirrorPiecePointQuery {
        self.query
    }
    /// Original explicit whole-work/result policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPiecePointQueryLimits {
        self.limits
    }
    /// First whole-population cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPiecePointQueryRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPiecePointQueryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPiecePointQueryError")
            .field("query", &self.query)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPiecePointQueryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror original-point eligibility: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPiecePointQueryError<'_> {}

/// Proposed original-identity eligibility only,without actual source/geometry/liveness proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPiecePointMatchKind {
    /// One proposed candidate names this exact original source identity.
    Unique,
    /// Several explicit roles name this source;none is chosen automatically.
    Ambiguous,
    /// No proposed candidate names this source;the original may remain globally live.
    Gap,
}
impl<'s> AdmittedMirrorPiecePoints<'s> {
    /// Count complete original-source matches after admitting all actual N candidates.
    /// Admission scans O(N) with constant auxiliary space and no new index/array. Match iteration
    /// scans the admitted original order;repeated caller passes retain their own work responsibility.
    /// Equality compares original PointRefs only,not current coordinates,membership or reflection.
    /// # Errors
    /// Whole candidate count first,then complete match count. No truncated/partial view escapes.
    pub fn query_source<'a>(
        &'a self,
        query: &'a MirrorPiecePointQuery,
        limits: MirrorPiecePointQueryLimits,
    ) -> Result<MirrorPiecePointMatch<'a>, MirrorPiecePointQueryError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| MirrorPiecePointQueryError {
            points: self,
            query,
            limits,
            refusal,
        };
        let n = self.point_count();
        if n > limits.candidates {
            return Err(refuse(MirrorPiecePointQueryRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        let mut count = 0usize;
        let mut unique_tag = None;
        for point in self.points() {
            if point.source() == query.source() {
                // Each match is one of the actual N disjoint candidate entries,so this fits usize.
                count += 1;
                unique_tag = if count == 1 {
                    Some(point.candidate().tag())
                } else {
                    None
                };
            }
        }
        if count > limits.matches {
            return Err(refuse(MirrorPiecePointQueryRefusal::Matches {
                required: count,
                allowed: limits.matches,
            }));
        }
        Ok(MirrorPiecePointMatch {
            points: self,
            query,
            limits,
            count,
            unique_tag,
        })
    }
}

/// Complete readonly original-source eligibility;not a map,registration repair or issued point.
/// ```compile_fail
/// fn replace<'a>(mut result:sc_core::construction::MirrorPiecePointMatch<'a>,
///     other:&'a sc_core::construction::MirrorPiecePointQuery) { result.query=other; }
/// ```
/// ```compile_fail
/// fn escape(points:sc_core::construction::AdmittedMirrorPiecePoints<'_>,
///     query:sc_core::construction::MirrorPiecePointQuery)->sc_core::construction::MirrorPiecePointMatch<'static> {
///     points.query_source(&query,sc_core::construction::MirrorPiecePointQueryLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct MirrorPiecePointMatch<'a> {
    points: &'a AdmittedMirrorPiecePoints<'a>,
    query: &'a MirrorPiecePointQuery,
    limits: MirrorPiecePointQueryLimits,
    count: usize,
    unique_tag: Option<LocalTag>,
}
impl<'a> MirrorPiecePointMatch<'a> {
    /// Actual original whole admitted point-candidate owner.
    #[must_use]
    pub const fn points(&self) -> &'a AdmittedMirrorPiecePoints<'a> {
        self.points
    }
    /// Actual original query/request field.
    #[must_use]
    pub const fn query(&self) -> &'a MirrorPiecePointQuery {
        self.query
    }
    /// Original explicit whole-work/result policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPiecePointQueryLimits {
        self.limits
    }
    /// Complete actual match count,not a capped prefix.
    #[must_use]
    pub const fn match_count(&self) -> usize {
        self.count
    }
    /// Original-identity candidate classification,without actual geometry/currentness evidence.
    #[must_use]
    pub const fn kind(&self) -> MirrorPiecePointMatchKind {
        match self.count {
            0 => MirrorPiecePointMatchKind::Gap,
            1 => MirrorPiecePointMatchKind::Unique,
            _ => MirrorPiecePointMatchKind::Ambiguous,
        }
    }
    /// Actual one candidate only for Unique;Ambiguous/Gap never silently pick a role.
    #[must_use]
    pub fn unique(&self) -> Option<MirrorPiecePoint<'_>> {
        self.unique_tag.and_then(|tag| self.points.point(tag))
    }
    /// Every actual matching candidate in original proposal order,including reverse iteration.
    pub fn matches(&self) -> impl DoubleEndedIterator<Item = MirrorPiecePoint<'_>> {
        self.points
            .points()
            .filter(move |point| point.source() == self.query.source())
    }
}
