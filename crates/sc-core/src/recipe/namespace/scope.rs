//! Borrowed metadata/checking capability,not a cloneable namespace or whole-program certificate.
use super::{
    FormulaDeclaration, FormulaDeclarationSource, FormulaNamespace, FormulaNamespaceError,
    FormulaUnboundName,
};
use crate::{
    name::MachineToken,
    recipe::{FormulaCheckedExpression, FormulaExpressionCheckError, FormulaNormalizedExpression},
};
use core::fmt;

/// Point/edge-only metadata locator;no registry,frame,manifest,operation order or value proof.
/// Private construction excludes scalar,recipe and reserved declarations from geometry insertion.
#[derive(Clone, Copy, Debug)]
pub struct FormulaGeometryDeclaration<'a>(FormulaDeclaration<'a>);
impl<'a> FormulaGeometryDeclaration<'a> {
    /// Exact original declaration,including borrowed name and persistent typed reference.
    #[must_use]
    pub const fn declaration(self) -> FormulaDeclaration<'a> {
        self.0
    }
}
impl<'a> TryFrom<FormulaDeclaration<'a>> for FormulaGeometryDeclaration<'a> {
    type Error = FormulaDeclaration<'a>;
    fn try_from(declaration: FormulaDeclaration<'a>) -> Result<Self, Self::Error> {
        match declaration.source() {
            FormulaDeclarationSource::Point(_) | FormulaDeclarationSource::Edge(_) => {
                Ok(Self(declaration))
            }
            FormulaDeclarationSource::Input { .. }
            | FormulaDeclarationSource::LengthInput { .. }
            | FormulaDeclarationSource::Recipe { .. }
            | FormulaDeclarationSource::Reserved(_) => Err(declaration),
        }
    }
}
impl<'a> From<FormulaGeometryDeclaration<'a>> for super::FormulaInitialDeclaration<'a> {
    fn from(declaration: FormulaGeometryDeclaration<'a>) -> Self {
        Self(declaration.0)
    }
}

/// Readonly initial/prior metadata loan. Copying this view copies only the map borrow.
/// Expression proofs retain actual expression/declaration owners,not this map allocation.
/// The view certifies no complete program,source/currentness,kinds or values by itself.
/// ```
/// use sc_core::{construction::{PointConstruction, PointConstructionNormalizationLimits, PointCoordinate},
///     name::MachineToken, ontology::{EntityId, LocalTag, PointRef}, recipe::{FormulaDeclaration,
///     FormulaExpressionDocument, FormulaGeometryDeclaration, FormulaKind, FormulaNameCursor,
///     FormulaNamespace, FormulaRecipe}};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let recipe=FormulaRecipe::parse("let width:length=25 mm")?.normalize_literals()?;
/// let name=MachineToken::new("p")?;
/// let mut cursor=FormulaNameCursor::new(FormulaNamespace::new([])?,&recipe);
/// cursor.advance_metadata().map_err(|e|e.to_string())?;
/// // This explicitly supplied reference is metadata, not a live geometry/source certificate.
/// let geometry=FormulaGeometryDeclaration::try_from(FormulaDeclaration::point(&name,
///     PointRef::new(EntityId::from_bits(80),LocalTag::FIRST))).map_err(|_|"point metadata")?;
/// cursor.declare_geometry(geometry).map_err(|e|e.to_string())?;
/// let input=PointConstruction::new(EntityId::from_bits(100),
///     FormulaExpressionDocument::new("width".into())?,
///     FormulaExpressionDocument::new("1 mm/0".into())?);
/// let normalized=input.normalized(PointConstructionNormalizationLimits::new(4,10))
///     .map_err(|e|e.to_string())?;
/// let proof=normalized.check_arguments_in_scope(cursor.visible_names()).map_err(|e|e.to_string())?;
/// drop(cursor);
/// assert_eq!(proof.coordinate(PointCoordinate::X).kind(),FormulaKind::Length);
/// # Ok(())
/// # }
/// ```
/// ```compile_fail,E0616
/// fn extract(scope:sc_core::recipe::FormulaNameScope<'_, '_>) {let _=scope.namespace.clone();}
/// ```
/// A held view cannot survive advancing its original cursor:
/// ```compile_fail,E0502
/// use sc_core::{name::MachineToken,recipe::{FormulaNameCursor,FormulaNamespace,FormulaRecipe}};
/// let recipe=FormulaRecipe::parse("let width:length=1 mm").unwrap().normalize_literals().unwrap();
/// let mut cursor=FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(),&recipe);
/// let scope=cursor.visible_names();
/// cursor.advance_metadata().unwrap();
/// let _=scope.resolve(&MachineToken::new("width").unwrap());
/// ```
#[derive(Clone, Copy)]
pub struct FormulaNameScope<'n, 'a> {
    namespace: &'n FormulaNamespace<'a>,
}
impl<'n, 'a> FormulaNameScope<'n, 'a> {
    // The mixed controller preflights authentic output headers without making self names visible.
    pub(crate) fn check_geometry(
        self,
        declaration: FormulaGeometryDeclaration<'a>,
    ) -> Result<(), FormulaNamespaceError<'a>> {
        self.namespace.check_declaration(declaration.declaration())
    }
    pub(super) const fn new(namespace: &'n FormulaNamespace<'a>) -> Self {
        Self { namespace }
    }
    pub(crate) const fn namespace(self) -> &'n FormulaNamespace<'a> {
        self.namespace
    }
    /// Exact visible declaration lookup,without canonical state,context availability or values.
    /// # Errors
    /// Original unbound-name refusal,borrowing the actual validated query.
    pub fn resolve<'q>(
        self,
        name: &'q MachineToken,
    ) -> Result<FormulaDeclaration<'a>, FormulaUnboundName<'q>> {
        self.namespace.resolve(name)
    }
    /// Invoke the existing complete expression checker against these actual visible declarations.
    /// No lookup/signature algorithm or proof owner is replaced;no numeric provider is queried.
    /// # Errors
    /// Original expression/node/span/name/call/dimension cause,without an accepted proof prefix.
    pub fn check_expression<'e, 's>(
        self,
        expression: &'e FormulaNormalizedExpression<'s>,
    ) -> Result<FormulaCheckedExpression<'e, 's, 'a>, FormulaExpressionCheckError<'e, 's>> {
        expression.check_kinds(self.namespace)
    }
}
impl fmt::Debug for FormulaNameScope<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNameScope").finish_non_exhaustive()
    }
}
