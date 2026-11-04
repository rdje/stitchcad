//! Complete original named signature and actual canonical Derived-output relation,without values.
use super::{BoundDraftingRuleSource, DraftingRuleBindingShape, DraftingRuleProvider};
use crate::{
    design::{
        fixed_value, DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError,
        DesignResourceSink, DesignResourceSource,
    },
    recipe::SemanticOperation,
    value::{LogicalInputBinding, LogicalInputLookupError, LogicalInputState},
};
use core::fmt;
mod check;
mod error;
mod matches;
pub use error::{
    DraftingRuleSignatureError, DraftingRuleSignatureIssue, DraftingRuleSignatureRefusal,
};
pub use matches::{DraftingRuleInputMatch, DraftingRuleInputPair};
type Admission = (DesignResourceDemand, DraftingRuleBindingShape);

/// Complete original local role/kind/result signature and actual logical output Derived-operation
/// relation under its exact original rule source parent. All domains/states remain in their owners.
/// It proves no actual canonical input/Profile parameter binding,selected values,body conformance,
/// source/evidence truth,physical result,dependency graph or executable program.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingRuleProvider<H>>(bound:&mut BoundDraftingRuleSignature<'_,'_,'_,H,S>) {
///     bound.output=None;
/// }
/// ```
/// The signature proof cannot outlive its original source parent:
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignResourceSource};
/// fn escape<'b,'w,'a,H:DesignResourceSource,S:DraftingRuleProvider<H>>(parent:&'b BoundDraftingRuleSource<'w,'a,H,S>)
///     ->Result<BoundDraftingRuleSignature<'static,'w,'a,H,S>,String> {
///     parent.bind_signature().map_err(|e|e.to_string())
/// }
/// ```
pub struct BoundDraftingRuleSignature<
    'b,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    parent: &'b BoundDraftingRuleSource<'w, 'a, H, S>,
    output: Option<LogicalInputBinding<'a>>,
    demand: DesignResourceDemand,
    shape: DraftingRuleBindingShape,
}
impl<'b, 'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    BoundDraftingRuleSignature<'b, 'w, 'a, H, S>
{
    /// Exact original source parent,including original work,operation,rule body and origin.
    #[must_use]
    pub const fn parent(&self) -> &'b BoundDraftingRuleSource<'w, 'a, H, S> {
        self.parent
    }
    /// Actual canonical Root logical output member when logical;Buttonhole has no authored length.
    /// Domain/state/provenance are original data,not evaluated values or evidence acceptance.
    #[must_use]
    pub const fn output(&self) -> Option<LogicalInputBinding<'a>> {
        self.output
    }
    /// Complete original named input count;mandatory button is separate from policy requirements.
    #[must_use]
    pub fn input_count(&self) -> usize {
        matches::input_count(self.parent.work().operation().operation())
    }
    /// One exact original role/kind match,with both original owners,loans and zero-based indices.
    /// Invalid operation-input index returns None. This is one allocation-free requirement scan;
    /// repeated caller inspections need their own aggregate allowance and bind no actual input.
    #[must_use]
    pub fn input_match(&self, index: usize) -> Option<DraftingRuleInputMatch<'a>> {
        matches::input_match(
            self.parent.work().operation().operation(),
            self.parent.source().declaration,
            index,
        )
    }
    /// Fresh complete Root/provider/request-pin demand from this signature admission.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Fresh complete conservative matching bound,without source-parent cached credit.
    #[must_use]
    pub const fn shape(&self) -> DraftingRuleBindingShape {
        self.shape
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingRuleSignature<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingRuleSignature")
            .field("input_count", &self.input_count())
            .finish_non_exhaustive()
    }
}
impl<H: DesignResourceSource, S: DraftingRuleProvider<H> + ?Sized>
    BoundDraftingRuleSignature<'_, '_, '_, H, S>
{
    /// Freshly rewalk every actual parent/Root/provider/request field;output is an original borrowed
    /// canonical member. Stored metadata grants no content credit. No matching,source or value query.
    /// # Errors
    /// Original first cumulative resource cause,including earlier or ignored sink failures.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            parent,
            output,
            demand,
            shape,
        } = self;
        fixed_value(output);
        fixed_value(demand);
        fixed_value(shape);
        parent.charge_resources(sink)
    }
}
impl<'w, 'a, H: DesignResourceSource, S: DraftingRuleProvider<H> + ?Sized>
    BoundDraftingRuleSource<'w, 'a, H, S>
{
    /// Repeat complete fresh content/W/member admission before result/count/named role-kind checks.
    /// Both immutable constructors already prove local role uniqueness;equal counts and complete
    /// supplied-role membership establish a bijection without positional pairing or a reverse scan.
    /// Then resolve actual Root logical output/kind and require Derived to name this exact operation.
    /// No new source/currentness/value query,buffer cloning,selection or result-vector allocation.
    /// # Errors
    /// Original fresh work,first complete signature,actual lending output or Derived-link refusal.
    pub fn bind_signature(
        &self,
    ) -> Result<
        BoundDraftingRuleSignature<'_, 'w, 'a, H, S>,
        DraftingRuleSignatureError<'_, 'w, 'a, H, S>,
    > {
        let work = self.work();
        let fresh = work
            .context()
            .prepare_rule_binding(
                work.operation().operation().id(),
                work.operation().operation().kind(),
                work.selection(),
                work.limits(),
            )
            .map_err(|cause| {
                DraftingRuleSignatureError::new(
                    self,
                    None,
                    None,
                    DraftingRuleSignatureRefusal::Work(cause),
                )
            })?;
        let admission = Some((fresh.demand(), fresh.shape()));
        let operation = work.operation().operation();
        check::signature(operation, self.source().declaration).map_err(|issue| {
            DraftingRuleSignatureError::new(
                self,
                admission,
                None,
                DraftingRuleSignatureRefusal::Mismatch(issue),
            )
        })?;
        let output = match operation {
            SemanticOperation::Buttonhole(_) => None,
            SemanticOperation::LogicalDerivation(logical) => {
                let target = logical.definition().output;
                let binding = match work
                    .context()
                    .scope()
                    .root()
                    .inputs()
                    .logicals()
                    .resolve(target.declaration, target.kind)
                {
                    Ok(binding) => binding,
                    Err(cause) => {
                        let actual = match cause {
                            LogicalInputLookupError::Kind { actual, .. } => Some(actual),
                            LogicalInputLookupError::Missing { .. } => None,
                        };
                        return Err(DraftingRuleSignatureError::new(
                            self,
                            admission,
                            actual,
                            DraftingRuleSignatureRefusal::Output(cause),
                        ));
                    }
                };
                let actual = binding.declaration();
                match &actual.definition().state {
                    LogicalInputState::Derived { operation: derived }
                        if *derived == operation.id() => {}
                    LogicalInputState::Derived { operation: derived } => {
                        return Err(DraftingRuleSignatureError::new(
                            self,
                            admission,
                            Some(actual),
                            DraftingRuleSignatureRefusal::DerivedOperation {
                                expected: operation.id(),
                                actual: *derived,
                            },
                        ))
                    }
                    _ => {
                        return Err(DraftingRuleSignatureError::new(
                            self,
                            admission,
                            Some(actual),
                            DraftingRuleSignatureRefusal::OutputNotDerived,
                        ))
                    }
                }
                Some(binding)
            }
        };
        Ok(BoundDraftingRuleSignature {
            parent: self,
            output,
            demand: fresh.demand(),
            shape: fresh.shape(),
        })
    }
}
