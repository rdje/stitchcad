//! Original additional names and actual output metadata sources,before complete name admission.
use super::NormalizedDraftingProgram;
use crate::{
    construction::{
        ActiveEdgeOutputRole, ActivePointOutputRole, BezierConstruction, CenterArcConstruction,
        DraftingContextSource, DraftingGeometryReference, LineConstruction, PointConstruction,
    },
    design::DesignMetadataHost,
    name::MachineToken,
    ontology::{EdgeRef, PointRef},
    recipe::{FormulaDeclaration, FormulaInitialDeclaration},
};
use core::fmt;

/// Actual original source of a planned point/edge name,not an anonymous supplied output reference.
/// Fixed schemas use their own published output method. Variable roles retain authentic typed
/// active catalogue membership and its actual definition/tag;local keys are not global names.
/// This is metadata,not program association,canonical issuance,frame,currentness or live geometry.
/// ```compile_fail
/// fn escape(source:sc_core::construction::DraftingGeometryNameSource<'_>)
///     ->sc_core::construction::DraftingGeometryNameSource<'static> {source}
/// ```
#[derive(Clone, Copy)]
pub enum DraftingGeometryNameSource<'a> {
    /// Original point declaration and its actual fixed output role.
    Point(&'a PointConstruction),
    /// Original line declaration and its actual fixed edge output role.
    Line(&'a LineConstruction),
    /// Original center-arc declaration and its actual fixed edge output role.
    CenterArc(&'a CenterArcConstruction),
    /// Original quadratic/cubic declaration and its actual fixed edge output role.
    Bezier(&'a BezierConstruction),
    /// Original typed point-role catalogue,active membership and definition.
    PointRole(ActivePointOutputRole<'a>),
    /// Original typed edge-role catalogue,active membership and definition.
    EdgeRole(ActiveEdgeOutputRole<'a>),
}
impl DraftingGeometryNameSource<'_> {
    /// Exact persistent reference from the actual fixed schema or original active tag/operation.
    /// This issues no identity,constructs no geometry and proves no creating-step association.
    #[must_use]
    pub fn reference(self) -> DraftingGeometryReference {
        use DraftingGeometryReference as R;
        match self {
            Self::Point(p) => R::Point(p.output()),
            Self::Line(l) => R::Edge(l.output()),
            Self::CenterArc(a) => R::Edge(a.output()),
            Self::Bezier(b) => R::Edge(b.output()),
            Self::PointRole(role) => R::Point(PointRef::new(
                role.catalog().catalog().operation(),
                *role.tag(),
            )),
            Self::EdgeRole(role) => R::Edge(EdgeRef::new(
                role.catalog().catalog().operation(),
                *role.tag(),
            )),
        }
    }
}
impl fmt::Debug for DraftingGeometryNameSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let family = match self {
            Self::Point(_) => "Point",
            Self::Line(_) => "Line",
            Self::CenterArc(_) => "CenterArc",
            Self::Bezier(_) => "Bezier",
            Self::PointRole(_) => "PointRole",
            Self::EdgeRole(_) => "EdgeRole",
        };
        f.debug_tuple("DraftingGeometryNameSource")
            .field(&family)
            .finish()
    }
}

/// Explicit original global output name,requested creating position and actual metadata source.
/// No default name/tag/position is inferred. Zero,out-of-range or mismatched requests survive
/// raw construction for later complete content/association admission;this is not a bound output.
pub struct DraftingOutputName<'a> {
    position: usize,
    name: MachineToken,
    source: DraftingGeometryNameSource<'a>,
}
impl<'a> DraftingOutputName<'a> {
    /// Own the actual authored name beside the requested position and original source handle.
    #[must_use]
    pub const fn new(
        position: usize,
        name: MachineToken,
        source: DraftingGeometryNameSource<'a>,
    ) -> Self {
        Self {
            position,
            name,
            source,
        }
    }
    /// Exact requested one-based combined position,including invalid input before admission.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }
    /// Actual explicit global machine name,independent of a catalogue's local role key.
    #[must_use]
    pub const fn name(&self) -> &MachineToken {
        &self.name
    }
    /// Actual original fixed declaration or typed active catalogue member.
    #[must_use]
    pub const fn source(&self) -> DraftingGeometryNameSource<'a> {
        self.source
    }
    /// Original borrowed name with the exact source's typed persistent reference,as raw metadata.
    /// This creates no source span,identity,current geometry or accepted namespace/program.
    #[must_use]
    pub fn declaration(&self) -> FormulaDeclaration<'_> {
        match self.source.reference() {
            DraftingGeometryReference::Point(p) => FormulaDeclaration::point(&self.name, p),
            DraftingGeometryReference::Edge(e) => FormulaDeclaration::edge(&self.name, e),
        }
    }
}
impl fmt::Debug for DraftingOutputName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingOutputName")
            .field("position", &self.position)
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}

/// Raw actual-program loan plus complete original extra initial and named-output vectors.
/// Initial locators retain original validated names and source records;output names own their
/// actual machine tokens and borrow original schema/catalogue owners. Empty,repeated,colliding
/// or mismatched input is retained unchanged for complete later admission,without callbacks,
/// values,normalization,source/position validation,namespace construction or accepted prefix.
/// Earlier vector/token/catalogue allocations are separate from the later explicit policy.
/// ```
/// use sc_core::{construction::{DraftingContextSource, DraftingGeometryNameSource, DraftingOutputName,
///     DraftingProgramNameInputs, NormalizedDraftingProgram, PointConstruction},
///     design::DesignMetadataHost, name::{MachineToken, MachineTokenError}};
/// fn raw_names<'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
///     program:&'n NormalizedDraftingProgram<'p,'a,H,S>,point:&'n PointConstruction)
///     -> Result<DraftingProgramNameInputs<'n,'p,'a,H,S>,MachineTokenError> {
///     let output=DraftingOutputName::new(2,MachineToken::new("front_waist")?,
///         DraftingGeometryNameSource::Point(point));
///     let names=DraftingProgramNameInputs::new(program,vec![],vec![output]);
///     assert!(std::ptr::eq(names.program(),program));
///     Ok(names)
/// }
/// ```
/// ```compile_fail,E0616
/// use sc_core::{construction::{DraftingProgramNameInputs,DraftingContextSource},design::DesignMetadataHost};
/// fn rewrite<H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     names:&mut DraftingProgramNameInputs<'_, '_, '_,H,S>) {names.outputs.clear();}
/// ```
pub struct DraftingProgramNameInputs<
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    program: &'n NormalizedDraftingProgram<'p, 'a, H, S>,
    initial: Vec<FormulaInitialDeclaration<'n>>,
    outputs: Vec<DraftingOutputName<'n>>,
}
impl<'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramNameInputs<'n, 'p, 'a, H, S>
{
    /// Retain the actual normalized program and both original input vectors without cloning.
    #[must_use]
    pub const fn new(
        program: &'n NormalizedDraftingProgram<'p, 'a, H, S>,
        initial: Vec<FormulaInitialDeclaration<'n>>,
        outputs: Vec<DraftingOutputName<'n>>,
    ) -> Self {
        Self {
            program,
            initial,
            outputs,
        }
    }
    /// Actual original normalized program,including its actual Root/context and full source owner.
    #[must_use]
    pub const fn program(&self) -> &'n NormalizedDraftingProgram<'p, 'a, H, S> {
        self.program
    }
    /// Complete original extra initial locators;Root-derived names and reserved sources compose later.
    #[must_use]
    pub fn initial(&self) -> &[FormulaInitialDeclaration<'n>] {
        &self.initial
    }
    /// Complete original named output occurrences,with original requested positions and sources.
    #[must_use]
    pub fn outputs(&self) -> &[DraftingOutputName<'n>] {
        &self.outputs
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramNameInputs<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramNameInputs")
            .field("initial_count", &self.initial.len())
            .field("output_count", &self.outputs.len())
            .finish_non_exhaustive()
    }
}
