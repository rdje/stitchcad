//! Actual bounded initial name map;ordered mixed metadata and kind/dependency proof follow later.
use super::BoundDraftingProgramInitials;
use crate::{
    construction::DraftingInitialInputProvider,
    design::{
        DesignFormulaDeclarationSource, DesignFormulaError, DesignInputsError, DesignMetadataHost,
    },
    recipe::{FormulaInitialDeclaration, FormulaNameScope, FormulaNamespace},
};
use core::fmt;

/// Original current-input,host or namespace cause,with complete prior initial associations.
pub struct DraftingProgramInitialNamespaceError<
    's,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'s BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    cause: DesignFormulaError<'s, H::Error>,
}
impl<
        's,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramInitialNamespaceError<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original complete associations,actual programme/Root and selected name-work limits.
    #[must_use]
    pub const fn inputs(&self) -> &'s BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Existing original current-input/host/invalid-initial/collision diagnostic. Explicit inspection
    /// exposes its original payload;default Debug/Display only identify the refusing phase.
    #[must_use]
    pub const fn cause(&self) -> &DesignFormulaError<'s, H::Error> {
        &self.cause
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramInitialNamespaceError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let phase = match &self.cause {
            DesignFormulaError::Inputs(_) => "Inputs",
            DesignFormulaError::InvalidInitial(_) => "InvalidInitial",
            DesignFormulaError::Namespace(_) => "Namespace",
        };
        f.debug_struct("DraftingProgramInitialNamespaceError")
            .field("phase", &phase)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingProgramInitialNamespaceError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting initial namespace: {self:?}")
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingProgramInitialNamespaceError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.source()
    }
}

/// Private initial map derived from the exact containing Root and complete bound extra inputs.
/// A readonly scope borrows its allocation;no caller namespace,cloned final map or accepted mixed
/// programme can be substituted. Values,whole kinds/dependencies and ordered visibility follow.
/// ```compile_fail,E0616
/// use sc_core::{construction::{DraftingProgramInitialNamespace,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn extract<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     names:DraftingProgramInitialNamespace<'_, '_, '_, '_, '_, '_, '_,H,S>) {let _=names.namespace;}
/// ```
/// ```compile_fail
/// use sc_core::{construction::{DraftingProgramInitialNamespace,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn escape<'s,'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     names:DraftingProgramInitialNamespace<'s,'i,'r,'b,'n,'p,'a,H,S>)
///     ->DraftingProgramInitialNamespace<'static,'i,'r,'b,'n,'p,'a,H,S> {names}
/// ```
pub struct DraftingProgramInitialNamespace<
    's,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'s BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    namespace: FormulaNamespace<'s>,
}
impl<
        's,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact original complete initial associations and their original role/work/input owners.
    #[must_use]
    pub const fn inputs(&self) -> &'s BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Borrow only actual initial declarations and reserved metadata;no authored programme output
    /// or scalar statement is visible yet. This view cannot outlive the private map allocation.
    #[must_use]
    pub const fn visible_names(&self) -> FormulaNameScope<'_, 's> {
        self.namespace.visible_names()
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramInitialNamespace<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramInitialNamespace")
            .finish_non_exhaustive()
    }
}
impl<
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignFormulaDeclarationSource,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Recheck actual current Root inputs after complete held-content/prospective name bounds,
    /// then build the initial map from actual parameters,selected host rows and bound extras.
    /// The allocation-free host iterator replaces the older allocating declaration-vector factory.
    /// Existing collision rules retain original source pairs in that order;no prefix escapes.
    /// No new currentness/geometry query is made to the held immutable drafting source.
    ///
    /// ```
    /// use sc_core::{construction::{BoundDraftingProgramInitials,DraftingInitialInputProvider},
    ///     design::DesignFormulaDeclarationSource,name::MachineToken,recipe::FormulaKind};
    /// fn initial_kind<H:DesignFormulaDeclarationSource,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     inputs:&BoundDraftingProgramInitials<'_, '_, '_, '_, '_, '_,H,S>,name:&MachineToken)
    ///     ->Result<FormulaKind,String> {
    ///     let names=inputs.bind_initial_namespace().map_err(|error|error.to_string())?;
    ///     names.visible_names().resolve(name).map(|declaration|declaration.kind())
    ///         .map_err(|error|error.to_string())
    /// }
    /// ```
    /// # Errors
    /// Existing actual current-input/host cause,invalid initial declaration or first collision.
    /// Complete original input/source witnesses remain available;no values or readiness grant.
    pub fn bind_initial_namespace<'s>(
        &'s self,
    ) -> Result<
        DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramInitialNamespaceError<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    > {
        let refuse = |cause| DraftingProgramInitialNamespaceError {
            inputs: self,
            cause,
        };
        let names = self.roles().positions().work().inputs();
        let root = names.program().admitted().inputs().context().scope().root();
        root.validate_current()
            .map_err(|cause| refuse(DesignFormulaError::Inputs(Box::new(cause))))?;
        let mut namespace = FormulaNamespace::new([])
            .map_err(|cause| refuse(DesignFormulaError::Namespace(cause)))?;
        for parameter in root.parameters().parameters() {
            let declaration = parameter.declaration(root.inputs()).map_err(|cause| {
                refuse(DesignFormulaError::Inputs(Box::new(
                    DesignInputsError::Parameters(Box::new(cause)),
                )))
            })?;
            let initial = FormulaInitialDeclaration::try_from(declaration)
                .map_err(|cause| refuse(DesignFormulaError::InvalidInitial(cause)))?;
            namespace = namespace
                .with_initial([initial])
                .map_err(|cause| refuse(DesignFormulaError::Namespace(cause)))?;
        }
        for declaration in root
            .sources()
            .host()
            .formula_declaration_sources(root.references(), root.inputs())
        {
            let declaration = declaration.map_err(|cause| {
                refuse(DesignFormulaError::Inputs(Box::new(
                    DesignInputsError::References(Box::new(cause)),
                )))
            })?;
            let initial = FormulaInitialDeclaration::try_from(declaration)
                .map_err(|cause| refuse(DesignFormulaError::InvalidInitial(cause)))?;
            namespace = namespace
                .with_initial([initial])
                .map_err(|cause| refuse(DesignFormulaError::Namespace(cause)))?;
        }
        namespace = namespace
            .with_initial(names.initial().iter().copied())
            .map_err(|cause| refuse(DesignFormulaError::Namespace(cause)))?;
        Ok(DraftingProgramInitialNamespace {
            inputs: self,
            namespace,
        })
    }
}
