//! Complete initial-namespace source kind proofs and actual canonical dependency/target checks.
use super::{
    FormulaBindingKind, FormulaCanonicalInput, FormulaCanonicalInputError,
    FormulaCheckedExpression, FormulaDeclarationSource, FormulaDerivedInput,
    FormulaExpressionCheckError, FormulaKind, FormulaNamespace, FormulaNormalizedSource,
    FormulaNormalizedSources, FormulaSourceSpan,
};
use crate::{ontology::EntityId, value::CanonicalInputTarget};
use core::fmt;
use std::collections::TryReserveError;

/// Explicit complete source/node/Derived-target and conservative retained-proof allowances.
/// `sources + 2 * nodes` bounds source proof records, node certificates and at most one name
/// dependency per node. Existing normalized arenas, checker scratch, allocator bytes/capacity,
/// namespace creation, CPU and whole graph/runtime resources are separate populations.
/// ```compile_fail
/// let limits: sc_core::recipe::FormulaSourceCheckLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSourceCheckLimits {
    sources: usize,
    nodes: usize,
    targets: usize,
    retained_records: usize,
}
impl FormulaSourceCheckLimits {
    /// Select all four allowances explicitly, including zero for an empty source/target scope.
    #[must_use]
    pub const fn new(
        sources: usize,
        nodes: usize,
        targets: usize,
        retained_records: usize,
    ) -> Self {
        Self {
            sources,
            nodes,
            targets,
            retained_records,
        }
    }
    /// Maximum complete source population, including unused sources.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Maximum complete normalized node population, including untaken branches.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Maximum complete canonical Derived target population, including shared/unused consumers.
    #[must_use]
    pub const fn targets(self) -> usize {
        self.targets
    }
    /// Maximum conservative retained-proof reservation `sources + 2 * nodes`.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}

/// Original static/ownership/consumer refusal; no fake statement, value or language error.
#[derive(Debug)]
pub enum FormulaSourceCheckRefusal<'a> {
    /// Complete source quota, checked first.
    Sources {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Complete prospective population or retained reservation cannot be represented.
    PopulationOverflow,
    /// Complete actual normalized node population exceeds its allowance.
    Nodes {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Complete canonical Derived consumer population exceeds its allowance.
    Targets {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Conservative complete retained-proof reservation exceeds its allowance.
    RetainedRecords {
        /// Selected allowance.
        allowed: usize,
        /// Complete prospective reservation.
        required: usize,
    },
    /// Original outer proof-vector reservation failure; inline, with no further allocation.
    Allocation(TryReserveError),
    /// Existing expression factory's original failing source/span and typed static cause.
    Expression {
        /// Original authored source index, not a statement ordinal.
        source_index: usize,
        /// Exact existing borrowed error, boxed only to compact semantic refusals.
        error: Box<FormulaExpressionCheckError<'a, 'a>>,
    },
    /// Existing canonical join refuses an actual name dependency, including untaken occurrences.
    CanonicalInput {
        /// Original authored source index.
        source_index: usize,
        /// Actual original name-use span.
        span: FormulaSourceSpan,
        /// Original canonical inventory/metadata cause; no value has been read.
        error: Box<FormulaCanonicalInputError<'a>>,
    },
    /// Actual source root kind disagrees with an actual canonical Derived consumer's kind.
    /// This is an integration refusal, not a fabricated let-header language diagnostic.
    TargetKind {
        /// Original authored source index.
        source_index: usize,
        /// Actual canonical Derived target/source pair.
        target: FormulaDerivedInput<'a>,
        /// Required kind from that actual target.
        expected: FormulaKind,
        /// Existing successful source proof's actual root kind.
        actual: FormulaKind,
    },
}

/// Refusal pinned to the actual normalized/source/input owners and selected explicit limits.
#[derive(Debug)]
pub struct FormulaSourceCheckError<'a> {
    normalized: &'a FormulaNormalizedSources<'a>,
    limits: FormulaSourceCheckLimits,
    refusal: FormulaSourceCheckRefusal<'a>,
}
impl<'a> FormulaSourceCheckError<'a> {
    /// Actual original normalized collection and complete canonical/source admission.
    #[must_use]
    pub const fn normalized(&self) -> &'a FormulaNormalizedSources<'a> {
        self.normalized
    }
    /// Selected allowances; no fallback/default or canonical content mutation.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceCheckLimits {
        self.limits
    }
    /// Original typed scope/static/consumer cause; borrowed semantic errors remain inspectable.
    #[must_use]
    pub const fn refusal(&self) -> &FormulaSourceCheckRefusal<'a> {
        &self.refusal
    }
}
impl fmt::Display for FormulaSourceCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "complete initial-scope source checking: {:?}",
            self.refusal
        )
    }
}
impl std::error::Error for FormulaSourceCheckError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            FormulaSourceCheckRefusal::Allocation(error) => Some(error),
            // Static/canonical causes borrow actual owners; refusal() preserves those typed causes
            // instead of erasing or pretending they meet Error::source's 'static requirement.
            _ => None,
        }
    }
}

/// One actual normalized source and its complete existing initial-namespace expression proof.
/// Canonical dependencies have matched the supplied actual inventory, without reading values.
#[derive(Debug)]
pub struct FormulaCheckedSource<'a> {
    normalized: &'a FormulaNormalizedSource<'a>,
    proof: FormulaCheckedExpression<'a, 'a, 'a>,
}
impl<'a> FormulaCheckedSource<'a> {
    /// Actual original source and normalized arena, with original local spans.
    #[must_use]
    pub const fn normalized(&self) -> &'a FormulaNormalizedSource<'a> {
        self.normalized
    }
    /// Actual existing factory proof; not a copied annotation or caller-seeded result.
    #[must_use]
    pub const fn proof(&self) -> &FormulaCheckedExpression<'a, 'a, 'a> {
        &self.proof
    }
}

/// Complete initial-scope source proofs, actual canonical dependency joins and target-kind checks.
/// No prior-let/operation graph, cycle, provider/current Design, binding, value or execution proof.
/// Supplied initial namespace metadata stays borrowed; the namespace allocation need not survive.
/// This certifies the supplied scope, not completeness of a containing Design/host namespace.
/// ```compile_fail
/// fn clear(sources: &mut sc_core::recipe::FormulaCheckedSources<'_>) { sources.sources.clear(); }
/// ```
/// ```compile_fail
/// fn escape(source: &sc_core::recipe::FormulaCheckedSource<'_>)
///     -> sc_core::recipe::FormulaCheckedNode<'static, 'static> { source.proof().root() }
/// ```
#[derive(Debug)]
pub struct FormulaCheckedSources<'a> {
    normalized: &'a FormulaNormalizedSources<'a>,
    limits: FormulaSourceCheckLimits,
    target_count: usize,
    retained_records: usize,
    sources: Vec<FormulaCheckedSource<'a>>,
}
impl<'a> FormulaCheckedSources<'a> {
    /// Actual immutable normalized/source/input owners; no replacement inventory is accepted.
    #[must_use]
    pub const fn normalized(&self) -> &'a FormulaNormalizedSources<'a> {
        self.normalized
    }
    /// Selected complete allowances, separate from canonical content.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceCheckLimits {
        self.limits
    }
    /// Complete actual canonical Derived consumer population, without a second target vector.
    #[must_use]
    pub const fn target_count(&self) -> usize {
        self.target_count
    }
    /// Complete conservative proof reservation; not measured allocator bytes or live records.
    #[must_use]
    pub const fn retained_records(&self) -> usize {
        self.retained_records
    }
    /// Actual complete source proofs in original authored order, never a successful prefix.
    #[must_use]
    pub fn sources(&self) -> &[FormulaCheckedSource<'a>] {
        &self.sources
    }
    /// Exact-id proof lookup reusing the original private source index; no copied/sorted index.
    #[must_use]
    pub fn source(&self, id: EntityId) -> Option<&FormulaCheckedSource<'a>> {
        self.sources
            .get(self.normalized.inputs().sources().source_index(id)?)
    }
}
impl<'s> FormulaNormalizedSources<'s> {
    /// Check every original source through the actual initial-namespace expression factory,
    /// then every canonical dependency, then every actual canonical Derived target's root kind.
    /// Every branch/unused source and repeated dependency participates before any accepted result.
    /// Reserved/geometry metadata grants kinds only; real providers/currentness remain separate.
    /// # Errors
    /// Source quota, checked complete arithmetic, node/target/retained quotas precede allocation.
    /// Existing static causes retain their original source/span; canonical errors retain original
    /// metadata/inventory. Target refusals retain actual consumers and actual inferred root kinds.
    /// No partial proof vector, guessed header, value read or new language diagnostic is returned.
    pub fn check_sources<'a, 'd>(
        &'a self,
        namespace: &FormulaNamespace<'d>,
        limits: FormulaSourceCheckLimits,
    ) -> Result<FormulaCheckedSources<'a>, FormulaSourceCheckError<'a>>
    where
        's: 'a,
        'd: 'a,
    {
        use FormulaSourceCheckRefusal as Refusal;
        let normalized: &'a FormulaNormalizedSources<'a> = self;
        let namespace: &FormulaNamespace<'a> = namespace;
        let refuse = |refusal| FormulaSourceCheckError {
            normalized,
            limits,
            refusal,
        };
        let source_count = normalized.sources().len();
        if source_count > limits.sources {
            return Err(refuse(Refusal::Sources {
                allowed: limits.sources,
                required: source_count,
            }));
        }
        let retained_records = normalized
            .node_count()
            .checked_mul(2)
            .and_then(|nodes| nodes.checked_add(source_count))
            .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
        let target_count = normalized
            .inputs()
            .derived_inputs()
            .try_fold(0usize, |total, _| {
                total
                    .checked_add(1)
                    .ok_or_else(|| refuse(Refusal::PopulationOverflow))
            })?;
        if normalized.node_count() > limits.nodes {
            return Err(refuse(Refusal::Nodes {
                allowed: limits.nodes,
                required: normalized.node_count(),
            }));
        }
        if target_count > limits.targets {
            return Err(refuse(Refusal::Targets {
                allowed: limits.targets,
                required: target_count,
            }));
        }
        if retained_records > limits.retained_records {
            return Err(refuse(Refusal::RetainedRecords {
                allowed: limits.retained_records,
                required: retained_records,
            }));
        }
        let mut sources = Vec::new();
        sources
            .try_reserve_exact(source_count)
            .map_err(|error| refuse(Refusal::Allocation(error)))?;
        for (source_index, source) in normalized.sources().iter().enumerate() {
            let proof = source
                .expression()
                .check_kinds(namespace)
                .map_err(|error| {
                    refuse(Refusal::Expression {
                        source_index,
                        error: Box::new(error),
                    })
                })?;
            sources.push(FormulaCheckedSource {
                normalized: source,
                proof,
            });
        }
        for (source_index, source) in sources.iter().enumerate() {
            for dependency in source.proof.dependencies() {
                let declaration = dependency.declaration();
                match declaration.source() {
                    FormulaDeclarationSource::Input { .. }
                    | FormulaDeclarationSource::LengthInput { .. } => {
                        FormulaCanonicalInput::resolve(declaration, normalized.inputs().inputs())
                            .map_err(|error| {
                            refuse(Refusal::CanonicalInput {
                                source_index,
                                span: dependency.span(),
                                error: Box::new(error),
                            })
                        })?;
                    }
                    FormulaDeclarationSource::Reserved(_)
                    | FormulaDeclarationSource::Point(_)
                    | FormulaDeclarationSource::Edge(_) => {}
                    FormulaDeclarationSource::Recipe { .. } => {
                        unreachable!("initial namespace construction prohibits prior-let sources");
                    }
                }
            }
        }
        for target in normalized.inputs().derived_inputs() {
            let Some(source_index) = normalized
                .inputs()
                .sources()
                .source_index(target.source().id())
            else {
                unreachable!(
                    "complete immutable source/input admission retains the exact source id"
                );
            };
            let Some(source) = sources.get(source_index) else {
                unreachable!("complete source proofs retain every original source position");
            };
            let expected = match target.target() {
                CanonicalInputTarget::Length(_) => FormulaKind::Length,
                CanonicalInputTarget::Scalar { kind, .. } => FormulaBindingKind::from(kind).into(),
            };
            let actual = source.proof.kind();
            if actual != expected {
                return Err(refuse(Refusal::TargetKind {
                    source_index,
                    target,
                    expected,
                    actual,
                }));
            }
        }
        Ok(FormulaCheckedSources {
            normalized,
            limits,
            target_count,
            retained_records,
            sources,
        })
    }
}
