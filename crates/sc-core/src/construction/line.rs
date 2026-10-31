//! Ordered line references and authentic declaration dependencies; no geometric edge result.
use crate::ontology::{EdgeRef, EntityId, LocalTag, PointRef};
use core::fmt;

/// Actual ordered endpoint role in a line declaration, independent of identity sort order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LineEndpoint {
    /// First point; establishes the line's declared direction.
    Start,
    /// Second point in that same declared direction.
    End,
}

/// A definite local self-dependency in the actual supplied line declaration.
/// No global prior-order, current registry or geometric diagnostic is fabricated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineConstructionError {
    operation: EntityId,
    endpoint: LineEndpoint,
    point: PointRef,
}
impl LineConstructionError {
    /// Original supplied creating-operation identity.
    #[must_use]
    pub const fn operation(self) -> EntityId {
        self.operation
    }
    /// Actual first refused source role, start before end.
    #[must_use]
    pub const fn endpoint(self) -> LineEndpoint {
        self.endpoint
    }
    /// Original point whose creator is this same operation, retaining its persistent local tag.
    #[must_use]
    pub const fn point(self) -> PointRef {
        self.point
    }
}
impl fmt::Display for LineConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "line {:?} point depends on its own creating operation {}",
            self.endpoint, self.operation
        )
    }
}
impl std::error::Error for LineConstructionError {}

/// Authored directed line declaration with two actual point-reference fields and one edge role.
/// Constructor excludes only definite local self-supply. It does not issue identities, resolve
/// references, prove prior order/frame compatibility or classify geometric degeneracy. Repeated
/// point uses remain distinct source occurrences; no coordinates, length or edge geometry exist.
/// ```compile_fail
/// use sc_core::{construction::LineConstruction,ontology::{EntityId,EdgeRef,LocalTag}};
/// let edge=EdgeRef::new(EntityId::from_bits(2),LocalTag::FIRST);
/// let invalid=LineConstruction::new(EntityId::from_bits(1),edge,edge);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineConstruction {
    operation: EntityId,
    start: PointRef,
    end: PointRef,
}
impl LineConstruction {
    /// Retain ordered endpoint references, checking a definite local self-dependency first.
    /// Numeric/ULID order says nothing about operation order. Actual containing-program checks
    /// must validate both suppliers and the output manifest before any geometry or publication.
    /// ```
    /// use sc_core::{construction::*,ontology::{EntityId,PointRef,LocalTag}};
    /// let start=PointRef::new(EntityId::from_bits(99),LocalTag::FIRST);
    /// let end=PointRef::new(EntityId::from_bits(80),LocalTag::new(7));
    /// let line=LineConstruction::new(EntityId::from_bits(10),start,end)?;
    /// assert_eq!(line.dependencies().map(|use_|use_.point()).collect::<Vec<_>>(),vec![start,end]);
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Returns the original operation,first self-dependent role and point reference;no partial
    /// accepted line or invented formula-language name/span is returned.
    pub fn new(
        operation: EntityId,
        start: PointRef,
        end: PointRef,
    ) -> Result<Self, LineConstructionError> {
        for (endpoint, point) in [(LineEndpoint::Start, start), (LineEndpoint::End, end)] {
            if point.creator() == operation {
                return Err(LineConstructionError {
                    operation,
                    endpoint,
                    point,
                });
            }
        }
        Ok(Self {
            operation,
            start,
            end,
        })
    }
    /// Original creating-operation identity, not an issuance/currentness certificate.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Stable logical edge role, without an evaluated or current edge.
    #[must_use]
    pub const fn output(&self) -> EdgeRef {
        EdgeRef::new(self.operation, LocalTag::FIRST)
    }
    /// Exact point field for this actual role, without reordering or reference remapping.
    #[must_use]
    pub const fn endpoint(&self, endpoint: LineEndpoint) -> PointRef {
        match endpoint {
            LineEndpoint::Start => self.start,
            LineEndpoint::End => self.end,
        }
    }
    /// Both actual source occurrences in start/end order, including a repeated point.
    /// Each private view remains tied to this exact declaration and its original field.
    pub fn dependencies(
        &self,
    ) -> impl ExactSizeIterator<Item = LinePointDependency<'_>> + DoubleEndedIterator + '_ {
        [LineEndpoint::Start, LineEndpoint::End]
            .into_iter()
            .map(move |endpoint| LinePointDependency {
                line: self,
                endpoint,
            })
    }
}

/// One actual point field's dependency, constructed only through its original line declaration.
/// This is a declared dependency, not a successful current-source or executed read witness.
/// ```compile_fail
/// fn forge(line:&sc_core::construction::LineConstruction)->sc_core::construction::LinePointDependency<'_> {
///     sc_core::construction::LinePointDependency { line,endpoint:sc_core::construction::LineEndpoint::End }
/// }
/// ```
/// ```compile_fail
/// fn escape(line:sc_core::construction::LineConstruction)->sc_core::construction::LinePointDependency<'static> {
///     line.dependencies().next().unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct LinePointDependency<'a> {
    line: &'a LineConstruction,
    endpoint: LineEndpoint,
}
impl<'a> LinePointDependency<'a> {
    /// Actual source declaration; no equal-looking foreign line can replace this borrow.
    #[must_use]
    pub const fn line(self) -> &'a LineConstruction {
        self.line
    }
    /// Actual ordered source role in that declaration.
    #[must_use]
    pub const fn endpoint(self) -> LineEndpoint {
        self.endpoint
    }
    /// Exact original point field, reconstructed only from its same immutable owner and role.
    #[must_use]
    pub const fn point(self) -> PointRef {
        self.line.endpoint(self.endpoint)
    }
}
