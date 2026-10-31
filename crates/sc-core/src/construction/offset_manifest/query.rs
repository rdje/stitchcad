//! Original-source candidate membership only;no output parameter or geometric correspondence.
use super::{AdmittedOffsetCandidates, OffsetCandidateFragment};
use crate::ontology::{EdgeAnchor, EdgeRef};
use core::fmt;

/// Original query is outside the admitted manifest's exact source frame or selected domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetCandidateQueryRefusal {
    /// Edge identity differs;current fragments/aliases are not silently followed.
    SourceEdge,
    /// Original parameter lies outside the selected inclusive source bounds.
    SourcePosition,
}
/// Actual admitted owner,exact query value and typed refusal;no candidate or geometric fallback.
pub struct OffsetCandidateQueryError<'a> {
    admitted: &'a AdmittedOffsetCandidates<'a>,
    anchor: EdgeAnchor,
    refusal: OffsetCandidateQueryRefusal,
}
impl<'a> OffsetCandidateQueryError<'a> {
    /// Actual admitted proof supplying the source frame and all candidate roles.
    #[must_use]
    pub const fn admitted(&self) -> &'a AdmittedOffsetCandidates<'a> {
        self.admitted
    }
    /// Exact provided anchor value,owned unchanged by this error.
    #[must_use]
    pub const fn anchor(&self) -> &EdgeAnchor {
        &self.anchor
    }
    /// Actual frame or selected-domain cause.
    #[must_use]
    pub const fn refusal(&self) -> OffsetCandidateQueryRefusal {
        self.refusal
    }
}
impl fmt::Debug for OffsetCandidateQueryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OffsetCandidateQueryError")
            .field("anchor", &self.anchor)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for OffsetCandidateQueryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "offset candidate query: {:?}", self.refusal)
    }
}
impl std::error::Error for OffsetCandidateQueryError<'_> {}

/// Membership outcome,not a resolved geometric position or permission to choose a fragment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetCandidateMatchKind {
    /// Exactly one candidate domain contains the original parameter.
    Unique,
    /// More than one candidate contains the original parameter;none is silently selected.
    Ambiguous,
    /// No candidate contains this in-domain source parameter.
    Gap,
}
/// Complete admitted source query;all matching original candidates remain available without allocation.
/// ```compile_fail
/// fn bypass(raw:&sc_core::construction::OffsetCandidateManifest,anchor:sc_core::ontology::EdgeAnchor) {
///     raw.query_source(anchor);
/// }
/// ```
/// ```compile_fail
/// fn escape(admitted:sc_core::construction::AdmittedOffsetCandidates<'_>,anchor:sc_core::ontology::EdgeAnchor)
///     ->sc_core::construction::OffsetCandidateQuery<'static> {
///     admitted.query_source(anchor).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct OffsetCandidateQuery<'a> {
    admitted: &'a AdmittedOffsetCandidates<'a>,
    anchor: EdgeAnchor,
}
impl<'a> OffsetCandidateQuery<'a> {
    /// Actual admitted manifest/input/policy owner.
    #[must_use]
    pub const fn admitted(&self) -> &'a AdmittedOffsetCandidates<'a> {
        self.admitted
    }
    /// Exact provided anchor value,owned unchanged by this query.
    #[must_use]
    pub const fn anchor(&self) -> &EdgeAnchor {
        &self.anchor
    }
    /// Count all matching source domains;no accepted prefix,default choice or output Param.
    /// This is a linear scan of the admitted population,with no query-path allocation.
    #[must_use]
    pub fn kind(&self) -> OffsetCandidateMatchKind {
        match self.candidates().count() {
            0 => OffsetCandidateMatchKind::Gap,
            1 => OffsetCandidateMatchKind::Unique,
            _ => OffsetCandidateMatchKind::Ambiguous,
        }
    }
    /// Every containing original candidate,in original source order. Shared endpoints retain
    /// both candidates and gaps return none. Reversing this view changes no persistent tag.
    /// Each call scans the admitted population;whole query-batch resources remain separate.
    pub fn candidates(&self) -> impl DoubleEndedIterator<Item = OffsetCandidateMatch<'_>> {
        self.admitted
            .fragments()
            .iter()
            .enumerate()
            .filter_map(move |(index, fragment)| {
                let interval = fragment.source_interval();
                (self.anchor.param >= interval.from() && self.anchor.param <= interval.to())
                    .then_some(OffsetCandidateMatch {
                        query: self,
                        index,
                        fragment,
                    })
            })
    }
}
/// An authentic original occurrence from this complete source query,not a geometric resolution.
/// ```compile_fail
/// fn forge<'a>(query:&'a sc_core::construction::OffsetCandidateQuery<'a>,
///     fragment:&'a sc_core::construction::OffsetCandidateFragment)->sc_core::construction::OffsetCandidateMatch<'a> {
///     sc_core::construction::OffsetCandidateMatch { query,index:0,fragment }
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct OffsetCandidateMatch<'a> {
    query: &'a OffsetCandidateQuery<'a>,
    index: usize,
    fragment: &'a OffsetCandidateFragment,
}
impl<'a> OffsetCandidateMatch<'a> {
    /// Actual validated source query and original anchor field.
    #[must_use]
    pub const fn query(self) -> &'a OffsetCandidateQuery<'a> {
        self.query
    }
    /// Actual original position in the candidate Vec,not a persistent output identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original candidate field,including explicit tag and source interval.
    #[must_use]
    pub const fn fragment(self) -> &'a OffsetCandidateFragment {
        self.fragment
    }
    /// Logical unissued role,not a current edge or a mapped output position.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        EdgeRef::new(
            self.query.admitted.manifest().operation(),
            self.fragment.tag(),
        )
    }
}
impl<'s> AdmittedOffsetCandidates<'s> {
    /// Check exact original edge,then inclusive selected-source bounds. Membership uses original
    /// source parameters even for Reversed traversal;it does not remap to a current fragment.
    /// No curve position,geometric mapping/length/error certificate,ledger repair or authority exists here.
    /// # Errors
    /// Source edge mismatch precedes source position refusal;gap inside the source is a valid query outcome.
    pub fn query_source<'a>(
        &'a self,
        anchor: EdgeAnchor,
    ) -> Result<OffsetCandidateQuery<'a>, OffsetCandidateQueryError<'a>>
    where
        's: 'a,
    {
        let admitted: &AdmittedOffsetCandidates<'a> = self;
        let refuse = |refusal| OffsetCandidateQueryError {
            admitted,
            anchor,
            refusal,
        };
        let source = admitted.bound_input().construction().source().range;
        if anchor.edge != source.edge() {
            return Err(refuse(OffsetCandidateQueryRefusal::SourceEdge));
        }
        if anchor.param < source.from() || anchor.param > source.to() {
            return Err(refuse(OffsetCandidateQueryRefusal::SourcePosition));
        }
        Ok(OffsetCandidateQuery { admitted, anchor })
    }
}
