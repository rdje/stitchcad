//! Original signature work,first local issue and actual lending canonical output causes.
use super::{Admission, BoundDraftingRuleSource, DraftingRuleBindingShape, DraftingRuleProvider};
use crate::{
    construction::DraftingRuleWorkError,
    design::{DesignMetadataHost, DesignResourceDemand},
    ontology::EntityId,
    recipe::SemanticRuleInputKind,
    value::{LogicalInputDeclaration, LogicalInputKind, LogicalInputLookupError},
};
use core::fmt;
/// First original complete signature issue;original records stay in the error's source parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingRuleSignatureIssue {
    /// Actual operation/signature families disagree;private source construction normally excludes it.
    Family,
    /// Logical rule result disagrees with the operation's explicit logical output kind.
    ResultKind {
        /// Original expected operation output kind.
        expected: LogicalInputKind,
        /// Actual original rule result kind.
        actual: LogicalInputKind,
    },
    /// Complete counts disagree,including missing or extra named requirements.
    InputCount {
        /// Complete original supplied input population.
        inputs: usize,
        /// Complete original source requirement population.
        requirements: usize,
    },
    /// First original operation role absent from the complete source signature.
    MissingRole {
        /// Original zero-based operation input position.
        input: usize,
    },
    /// Original named pair disagrees in exact logical/numeric representation or kind.
    InputKind {
        /// Original zero-based operation input position.
        input: usize,
        /// Original zero-based source requirement position.
        requirement: usize,
        /// Actual original source requirement.
        expected: SemanticRuleInputKind,
        /// Exact kind represented by the original supplied reference,not a bound canonical value.
        actual: SemanticRuleInputKind,
    },
}
/// Original fresh work,first signature,actual output-member or Derived-link refusal.
pub enum DraftingRuleSignatureRefusal<
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    /// Original complete fresh work cause before signature/output inspection.
    Work(DraftingRuleWorkError<'a, H, S>),
    /// First complete metadata signature issue,without a successful local prefix.
    Mismatch(DraftingRuleSignatureIssue),
    /// Original actual Root logical output lookup/kind cause,including the actual member loan.
    Output(LogicalInputLookupError<'a>),
    /// Actual output has an authored/unknown state instead of a Derived computation reference.
    OutputNotDerived,
    /// Actual output's Derived reference names another operation.
    DerivedOperation {
        /// Original requested semantic operation identity.
        expected: EntityId,
        /// Original actual canonical declaration's Derived operation reference.
        actual: EntityId,
    },
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingRuleSignatureRefusal<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Work(cause) => f.debug_tuple("Work").field(cause).finish(),
            Self::Mismatch(issue) => f.debug_tuple("Mismatch").field(issue).finish(),
            Self::Output(cause) => f.debug_tuple("Output").field(cause).finish(),
            Self::OutputNotDerived => f.write_str("OutputNotDerived"),
            Self::DerivedOperation { expected, actual } => f
                .debug_struct("DerivedOperation")
                .field("expected", expected)
                .field("actual", actual)
                .finish(),
        }
    }
}
/// Exact original source parent,fresh work metadata,original output loan and first typed refusal.
pub struct DraftingRuleSignatureError<
    'b,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    details: Box<Failure<'b, 'w, 'a, H, S>>,
}
struct Failure<'b, 'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> {
    parent: &'b BoundDraftingRuleSource<'w, 'a, H, S>,
    fresh: Option<Admission>,
    output: Option<&'a LogicalInputDeclaration>,
    refusal: DraftingRuleSignatureRefusal<'a, H, S>,
}
impl<'b, 'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingRuleSignatureError<'b, 'w, 'a, H, S>
{
    pub(super) fn new(
        parent: &'b BoundDraftingRuleSource<'w, 'a, H, S>,
        fresh: Option<Admission>,
        output: Option<&'a LogicalInputDeclaration>,
        refusal: DraftingRuleSignatureRefusal<'a, H, S>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                parent,
                fresh,
                output,
                refusal,
            }),
        }
    }
    /// Exact original rule source parent and complete original operation/signature records.
    #[must_use]
    pub const fn parent(&self) -> &'b BoundDraftingRuleSource<'w, 'a, H, S> {
        self.details.parent
    }
    /// Fresh complete content demand when fresh work succeeded.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        match self.details.fresh {
            Some((demand, _)) => Some(demand),
            None => None,
        }
    }
    /// Fresh complete matching shape when fresh work succeeded.
    #[must_use]
    pub const fn shape(&self) -> Option<DraftingRuleBindingShape> {
        match self.details.fresh {
            Some((_, shape)) => Some(shape),
            None => None,
        }
    }
    /// Original actual canonical output record when encountered,without a partial signature proof.
    #[must_use]
    pub const fn output(&self) -> Option<&'a LogicalInputDeclaration> {
        self.details.output
    }
    /// Original nested work,signature,lending output or Derived-link cause.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingRuleSignatureRefusal<'a, H, S> {
        &self.details.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingRuleSignatureError<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingRuleSignatureError")
            .field("refusal", self.refusal())
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Display
    for DraftingRuleSignatureError<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting rule signature: {:?}", self.refusal())
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> std::error::Error
    for DraftingRuleSignatureError<'_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal() {
            DraftingRuleSignatureRefusal::Work(cause) => std::error::Error::source(cause),
            _ => None,
        }
    }
}
