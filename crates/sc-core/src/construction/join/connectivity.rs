//! Required original source-path gluing under authored endpoint orientation,not actual geometry.
use super::{
    DeclaredJoinCutContours, JoinCutContourUseLocation, JoinParameterDirection, JoinPlacedCutUse,
    JoinSourceOccurrence, JoinSourceOccurrenceLocation, JoinSpliceEnd,
};
use crate::ontology::{Direction, Param, SeamSideId};
use core::fmt;

/// Complete declared target hole/use allowances before source-port scans;no defaults.
/// ```compile_fail
/// let limits:sc_core::construction::JoinCutConnectivityLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCutConnectivityLimits {
    holes: usize,
    uses: usize,
}
impl JoinCutConnectivityLimits {
    /// Admit all target H/U first. Earlier complete immutable input/source/candidate/direction/
    /// cut-coverage/use policies remain required;this stage allocates no new heap index.
    #[must_use]
    pub const fn new(holes: usize, uses: usize) -> Self {
        Self { holes, uses }
    }
    /// Whole original proposed result hole-group allowance.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// Whole original proposed result cut-use allowance.
    #[must_use]
    pub const fn uses(self) -> usize {
        self.uses
    }
}
/// First whole quota,unexpected native-source invariant or declared adjacency failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCutConnectivityRefusal {
    /// Complete target hole headers exceed policy before any source-port read.
    Holes {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete target uses exceed policy before any source-port read.
    Uses {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Authentic bank no longer lends its actual original cut field/loop. This is an internal
    /// invariant failure requiring diagnosis,not an unsupported caller variant.
    BankSourceInvariant {
        /// Original A/B role.
        side: SeamSideId,
    },
    /// Authentic cut use no longer lends its actual original cut field/loop;diagnose the invariant.
    UseSourceInvariant {
        /// Original declared result use.
        location: JoinCutContourUseLocation,
    },
    /// A privately admitted nonempty loop is unexpectedly empty;diagnose rather than skip it.
    DeclaredLoopInvariant {
        /// None for boundary,Some for original result hole index.
        hole: Option<usize>,
    },
    /// Consecutive or closing uses violate required original source-vertex/bank-endpoint gluing.
    Adjacency {
        /// Actual original preceding result use.
        from: JoinCutContourUseLocation,
        /// Actual original next result use,including loop closure.
        to: JoinCutContourUseLocation,
    },
}
/// Whole authentic cut-use parent,policy,cause and actual use views survive every refusal.
pub struct JoinCutConnectivityError<'a> {
    contours: &'a DeclaredJoinCutContours<'a>,
    limits: JoinCutConnectivityLimits,
    refusal: JoinCutConnectivityRefusal,
    from: Option<JoinPlacedCutUse<'a>>,
    to: Option<JoinPlacedCutUse<'a>>,
}
impl<'a> JoinCutConnectivityError<'a> {
    /// Actual complete original cut-use/direction/coverage/identity/source parents.
    #[must_use]
    pub const fn contours(&self) -> &'a DeclaredJoinCutContours<'a> {
        self.contours
    }
    /// Original whole H/U policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCutConnectivityLimits {
        self.limits
    }
    /// First complete quota,unexpected native invariant or original adjacency cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinCutConnectivityRefusal {
        self.refusal
    }
    /// Actual preceding use for Adjacency,or failing use for UseSourceInvariant;otherwise None.
    #[must_use]
    pub const fn from_use(&self) -> Option<JoinPlacedCutUse<'a>> {
        self.from
    }
    /// Actual next use only for Adjacency;other causes return None.
    #[must_use]
    pub const fn to_use(&self) -> Option<JoinPlacedCutUse<'a>> {
        self.to
    }
}
impl fmt::Debug for JoinCutConnectivityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCutConnectivityError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCutConnectivityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join required cut connectivity: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCutConnectivityError<'_> {}
// Transient positions in one authentic source context,never persisted/issued entity identities.
// Distinct A/B roles and original cut loops remain distinct even for byte-equal EdgeRefs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SourceLoop {
    Boundary,
    Hole(usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Position {
    Vertex(usize),
    Interior { field: usize, param: Param },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Port {
    side: SeamSideId,
    source_loop: SourceLoop,
    position: Position,
}
fn port(source: JoinSourceOccurrence<'_>, param: Param) -> Option<Port> {
    let definition = source.piece().definition();
    let (source_loop, index, fields) = match source.location() {
        JoinSourceOccurrenceLocation::Boundary(index) => {
            (SourceLoop::Boundary, index, definition.boundary.as_slice())
        }
        JoinSourceOccurrenceLocation::Hole { hole, index } => (
            SourceLoop::Hole(hole),
            index,
            definition.holes.get(hole)?.as_slice(),
        ),
        JoinSourceOccurrenceLocation::Construction(_) | JoinSourceOccurrenceLocation::Fold(_) => {
            return None
        }
    };
    let field = fields.get(index)?;
    if !core::ptr::eq(&field.edge, source.field().edge()) {
        return None;
    }
    let position = if param == Param::START || param == Param::END {
        let entry = match field.direction {
            Direction::Original => Param::START,
            Direction::Reversed => Param::END,
        };
        let next = index.checked_add(1)?;
        Position::Vertex(if param == entry {
            index
        } else if next == fields.len() {
            0
        } else {
            next
        })
    } else {
        Position::Interior {
            field: index,
            param,
        }
    };
    Some(Port {
        side: source.side(),
        source_loop,
        position,
    })
}
fn ports(use_: JoinPlacedCutUse<'_>) -> Option<(Port, Port)> {
    let output = use_.output();
    let source = output.source();
    let range = source.source();
    let reverse = matches!(
        (output.parameter_direction(), *use_.direction()),
        (JoinParameterDirection::SameAsSource, Direction::Reversed)
            | (JoinParameterDirection::ReverseSource, Direction::Original)
    );
    let (entry, exit) = if reverse {
        (range.to(), range.from())
    } else {
        (range.from(), range.to())
    };
    Some((
        port(source.occurrence(), entry)?,
        port(source.occurrence(), exit)?,
    ))
}
type Links = [(Port, Port); 2];
fn equivalent(p: Port, q: Port, [(a, b), (c, d)]: Links) -> bool {
    let contains = |x, left, right| x == left || x == right;
    let shared = a == c || a == d || b == c || b == d;
    p == q
        || (contains(p, a, b) && contains(q, a, b))
        || (contains(p, c, d) && contains(q, c, d))
        || (shared
            && (contains(p, a, b) || contains(p, c, d))
            && (contains(q, a, b) || contains(q, c, d)))
}
fn check_loop<'a>(
    contours: &'a DeclaredJoinCutContours<'a>,
    limits: JoinCutConnectivityLimits,
    mut uses: impl Iterator<Item = JoinPlacedCutUse<'a>>,
    hole: Option<usize>,
    links: Links,
) -> Result<(), JoinCutConnectivityError<'a>> {
    use JoinCutConnectivityRefusal as R;
    let error = |refusal, from, to| JoinCutConnectivityError {
        contours,
        limits,
        refusal,
        from,
        to,
    };
    let first = uses
        .next()
        .ok_or_else(|| error(R::DeclaredLoopInvariant { hole }, None, None))?;
    let (first_entry, mut exit) = ports(first).ok_or_else(|| {
        error(
            R::UseSourceInvariant {
                location: first.location(),
            },
            Some(first),
            None,
        )
    })?;
    let mut previous = first;
    for next in uses {
        let (entry, next_exit) = ports(next).ok_or_else(|| {
            error(
                R::UseSourceInvariant {
                    location: next.location(),
                },
                Some(next),
                None,
            )
        })?;
        if !equivalent(exit, entry, links) {
            return Err(error(
                R::Adjacency {
                    from: previous.location(),
                    to: next.location(),
                },
                Some(previous),
                Some(next),
            ));
        }
        previous = next;
        exit = next_exit;
    }
    if !equivalent(exit, first_entry, links) {
        return Err(error(
            R::Adjacency {
                from: previous.location(),
                to: first.location(),
            },
            Some(previous),
            Some(first),
        ));
    }
    Ok(())
}
impl<'s> DeclaredJoinCutContours<'s> {
    /// Check requested original source-path adjacency and both bank endpoint gluing requirements.
    /// Whole target H/U precedes every source-port read/scan. Original cyclic directed-loop vertices
    /// normalize0/1;interior source field/Param ports stay exact. A/B and source-loop identity remain
    /// distinct. Compose explicit output traversal with requested parameter orientation;actual
    /// canonical Same/Opposite seam pairs supply exactly two required equivalence links,including
    /// transitivity when their endpoints share an original vertex. No numeric interior map follows.
    /// Boundary then original hole order,consecutive then closing use determines first failure.
    /// Work O(H+U+1),constant four bank ports/loop cursor,no allocation,registry/provider/geometry.
    /// Earlier complete immutable input/source/candidate/direction/cut-use policies remain required.
    /// This proves required source-path gluing only,not achieved endpoint coincidence,geometric
    /// closure,CCW/simplicity,hole classification/containment,currentness,noncut semantics or authority.
    /// # Errors
    /// Whole H/U quotas,unexpected original bank/source/view invariants,first requested adjacency.
    /// An invariant failure is an implementation fault to diagnose,not unsupported input/repair.
    pub fn require_cut_connectivity<'a>(
        &'a self,
        limits: JoinCutConnectivityLimits,
    ) -> Result<RequiredJoinCutConnectivity<'a>, JoinCutConnectivityError<'a>>
    where
        's: 'a,
    {
        use JoinCutConnectivityRefusal as R;
        let error = |refusal| JoinCutConnectivityError {
            contours: self,
            limits,
            refusal,
            from: None,
            to: None,
        };
        let h = self.hole_count();
        let u = self.use_count();
        if h > limits.holes {
            return Err(error(R::Holes {
                required: h,
                allowed: limits.holes,
            }));
        }
        if u > limits.uses {
            return Err(error(R::Uses {
                required: u,
                allowed: limits.uses,
            }));
        }
        let banks = self
            .directions()
            .coverage()
            .edges()
            .identity()
            .roles()
            .banks();
        let splice = banks.sources().population().metadata().splice();
        let make_port = |side, end| {
            let anchor = splice.endpoint_pair(end).anchor(side);
            port(banks.bank(side).occurrence(), anchor.param)
                .ok_or_else(|| error(R::BankSourceInvariant { side }))
        };
        let links = [
            (
                make_port(SeamSideId::A, JoinSpliceEnd::AFrom)?,
                make_port(SeamSideId::B, JoinSpliceEnd::AFrom)?,
            ),
            (
                make_port(SeamSideId::A, JoinSpliceEnd::ATo)?,
                make_port(SeamSideId::B, JoinSpliceEnd::ATo)?,
            ),
        ];
        check_loop(self, limits, self.boundary(), None, links)?;
        for hole in self.holes() {
            check_loop(self, limits, hole.uses(), Some(hole.index()), links)?;
        }
        Ok(RequiredJoinCutConnectivity {
            contours: self,
            limits,
            connections: u,
        })
    }
}
/// Authentic required source-path gluing,not geometric correspondence/closure or a valid Piece.
/// ```compile_fail
/// fn forge<'a>(mut connectivity:sc_core::construction::RequiredJoinCutConnectivity<'a>,other:&'a sc_core::construction::DeclaredJoinCutContours<'a>) {connectivity.contours=other;}
/// ```
/// ```compile_fail
/// fn escape(contours:sc_core::construction::DeclaredJoinCutContours<'_>)->sc_core::construction::RequiredJoinCutConnectivity<'static> {
///     contours.require_cut_connectivity(sc_core::construction::JoinCutConnectivityLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct RequiredJoinCutConnectivity<'a> {
    contours: &'a DeclaredJoinCutContours<'a>,
    limits: JoinCutConnectivityLimits,
    connections: usize,
}
impl<'a> RequiredJoinCutConnectivity<'a> {
    /// Actual whole original cut-use/direction/coverage/identity/source parents.
    #[must_use]
    pub const fn contours(&self) -> &'a DeclaredJoinCutContours<'a> {
        self.contours
    }
    /// Original complete target H/U policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCutConnectivityLimits {
        self.limits
    }
    /// Complete consecutive-plus-closing connection count,one per actual declared cut use.
    #[must_use]
    pub const fn connection_count(&self) -> usize {
        self.connections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_link_equivalence_matches_independent_reachability_for_shared_disjoint_and_self_links() {
        let a = Port {
            side: SeamSideId::A,
            source_loop: SourceLoop::Boundary,
            position: Position::Vertex(0),
        };
        let b = Port {
            side: SeamSideId::B,
            ..a
        };
        let c = Port {
            source_loop: SourceLoop::Hole(0),
            ..b
        };
        let d = Port {
            position: Position::Vertex(1),
            ..a
        };
        for (start, target, links) in [
            (a, c, [(a, b), (b, c)]),
            (c, a, [(a, b), (b, c)]),
            (a, d, [(a, b), (b, c)]),
            (a, c, [(a, b), (c, d)]),
            (a, b, [(a, a), (b, c)]),
            (a, a, [(a, a), (b, c)]),
        ] {
            // Independent graph traversal oracle for the declared two undirected links.
            // These are source-position keys,not fabricated geometric evidence.
            let mut reached = vec![start];
            let mut cursor = 0;
            while let Some(current) = reached.get(cursor).copied() {
                cursor += 1;
                for (left, right) in links {
                    let neighbor = if current == left {
                        Some(right)
                    } else if current == right {
                        Some(left)
                    } else {
                        None
                    };
                    if let Some(next) = neighbor.filter(|next| !reached.contains(next)) {
                        reached.push(next);
                    }
                }
            }
            assert_eq!(equivalent(start, target, links), reached.contains(&target));
        }
    }
}
