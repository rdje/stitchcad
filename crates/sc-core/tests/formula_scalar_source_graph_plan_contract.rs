//! Complete scalar source plans,prospective allowances and unchanged actual unbound/state boundary.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaCanonicalInputAdapter, FormulaDeclaration, FormulaExpressionDocument,
        FormulaInitialDeclaration, FormulaInputOrigin, FormulaNamespace, FormulaNormalizedSources,
        FormulaScalarExecutionSourceKind, FormulaScalarSourceGraphPlanRefusal as Refusal,
        FormulaSource, FormulaSourceCheckLimits, FormulaSourceCycleLimits,
        FormulaSourceDependencyLimits, FormulaSourceGraphExecutionLimits as Limits,
        FormulaSourceInputs, FormulaSourceLimits, FormulaSourceNormalizationLimits, FormulaSources,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        LengthValueError,
    },
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
struct Fixture {
    canonical: CanonicalInputs,
    sources: FormulaSources,
    name: MachineToken,
}
impl Fixture {
    fn new(tangent: bool) -> Self {
        let canonical = CanonicalInputs::new(
            vec![(10, 9001), (11, 9002)]
                .into_iter()
                .map(|(target, formula)| {
                    LengthDeclaration::new(LengthDeclarationDefinition {
                        id: id(target),
                        source: id(4000),
                        state: LengthState::Derived {
                            formula: id(formula),
                        },
                    })
                    .expect("actual canonical state")
                })
                .collect(),
            vec![],
        )
        .expect("actual canonical owners");
        let mut rows = vec![(9002, "a*3"), (9001, "1 mm/3")];
        if tangent {
            rows.push((9003, "tan(1 deg)"));
        }
        let sources: Vec<_> = rows
            .into_iter()
            .map(|(identity, source)| {
                FormulaSource::new(
                    id(identity),
                    FormulaExpressionDocument::new(source.into()).expect("actual source syntax"),
                )
            })
            .collect();
        let limits = FormulaSourceLimits::new(
            sources.len(),
            sources.iter().map(|s| s.expression().source().len()).sum(),
            sources
                .iter()
                .map(|s| s.expression().canonical().as_str().len())
                .sum(),
        );
        Self {
            canonical,
            sources: FormulaSources::new(sources, limits).expect("actual source entities"),
            name: MachineToken::new("a").expect("actual name"),
        }
    }
    fn normalized(&self) -> FormulaNormalizedSources<'_> {
        let s = self.sources.sources().len();
        let n: usize = self
            .sources
            .sources()
            .iter()
            .map(|s| s.expression().node_count())
            .sum();
        FormulaSourceInputs::new(&self.canonical, &self.sources)
            .expect("actual target/source joins")
            .normalize_sources(FormulaSourceNormalizationLimits::new(s, n, s + 2 * n))
            .expect("complete normalized owners")
    }
    fn namespace(&self) -> FormulaNamespace<'_> {
        FormulaNamespace::new([FormulaInitialDeclaration::try_from(
            FormulaDeclaration::length_input(
                &self.name,
                FormulaInputOrigin::Parameter,
                id(5001),
                self.canonical.length(id(10)).expect("actual source"),
            ),
        )
        .expect("actual initial declaration")])
        .expect("actual namespace")
    }
}

#[test]
fn complete_actual_scalar_sources_are_prepared_without_values_binding_or_canonical_state_changes() {
    let fixture = Fixture::new(false);
    let before = fixture.canonical.clone();
    let normalized = fixture.normalized();
    let checked = normalized
        .check_sources(
            &fixture.namespace(),
            FormulaSourceCheckLimits::new(2, 6, 2, 14),
        )
        .expect("complete actual static sources");
    let deps = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(2, 1, 4))
        .expect("actual edge");
    let graph = deps
        .admit_acyclic(FormulaSourceCycleLimits::new(2, 4))
        .expect("actual acyclic relation");
    let limits = Limits::new(2, 6, 1, 17, 2);
    assert_eq!(
        (
            limits.sources(),
            limits.nodes(),
            limits.name_uses(),
            limits.workspace_records(),
            limits.output_records()
        ),
        (2, 6, 1, 17, 2)
    );
    let plan = graph
        .prepare_scalar_sources(limits)
        .expect("complete available source plans");
    assert!(std::ptr::eq(plan.graph(), &graph));
    assert_eq!(plan.limits(), limits);
    assert_eq!(
        (
            plan.sources().len(),
            plan.node_count(),
            plan.name_use_count(),
            plan.workspace_records(),
            plan.output_records()
        ),
        (2, 6, 1, 17, 2)
    );
    for (actual, original) in plan.sources().iter().zip(checked.sources()) {
        assert!(std::ptr::eq(actual.source(), original));
        assert!(std::ptr::eq(
            actual.plan().admission().root().expression(),
            original.proof().expression()
        ));
        assert!(std::ptr::eq(actual.plan().inputs(), &fixture.canonical));
    }
    let source = plan.sources().first().expect("actual first source:a*3");
    let name = source
        .plan()
        .sources()
        .next()
        .expect("actual Derived name occurrence");
    let FormulaScalarExecutionSourceKind::Canonical(joined) = name.kind() else {
        unreachable!("actual canonical input");
    };
    let FormulaCanonicalInputAdapter::Length(input) = joined.adapter() else {
        unreachable!("actual Length family");
    };
    assert!(std::ptr::eq(
        input.canonical_declaration(),
        fixture
            .canonical
            .length(id(10))
            .expect("same actual record")
    ));
    assert_eq!(
        input.canonical_declaration().authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(10),
            formula: id(9001)
        })
    );
    assert_eq!(fixture.canonical, before);
}

#[test]
fn complete_source_node_name_output_and_workspace_quotas_precede_source_preparation() {
    let fixture = Fixture::new(false);
    let normalized = fixture.normalized();
    let checked = normalized
        .check_sources(
            &fixture.namespace(),
            FormulaSourceCheckLimits::new(2, 6, 2, 14),
        )
        .expect("static");
    let deps = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(2, 1, 4))
        .expect("original relation");
    let graph = deps
        .admit_acyclic(FormulaSourceCycleLimits::new(2, 4))
        .expect("acyclic");
    for (limits, category) in [
        (Limits::new(1, 0, 0, 0, 0), "sources"),
        (Limits::new(2, 5, 0, 0, 0), "nodes"),
        (Limits::new(2, 6, 0, 0, 0), "names"),
        (Limits::new(2, 6, 1, 0, 1), "outputs"),
        (Limits::new(2, 6, 1, 16, 2), "workspace"),
    ] {
        let error = graph
            .prepare_scalar_sources(limits)
            .expect_err("complete resource refusal");
        assert!(std::ptr::eq(error.graph(), &graph));
        assert_eq!(error.limits(), limits);
        assert!(matches!(
            (category, error.refusal()),
            (
                "sources",
                Refusal::SourceCount {
                    allowed: 1,
                    required: 2
                }
            ) | (
                "nodes",
                Refusal::Nodes {
                    allowed: 5,
                    required: 6
                }
            ) | (
                "names",
                Refusal::NameUses {
                    allowed: 0,
                    required: 1
                }
            ) | (
                "outputs",
                Refusal::OutputRecords {
                    allowed: 1,
                    required: 2
                }
            ) | (
                "workspace",
                Refusal::WorkspaceRecords {
                    allowed: 16,
                    required: 17
                }
            )
        ));
        assert!(std::error::Error::source(&error).is_none());
    }
}

#[test]
fn late_unused_unavailable_scalar_family_keeps_original_source_call_instead_of_language_error() {
    let fixture = Fixture::new(true);
    let normalized = fixture.normalized();
    let checked = normalized
        .check_sources(
            &fixture.namespace(),
            FormulaSourceCheckLimits::new(3, 8, 2, 19),
        )
        .expect("all static kinds");
    let deps = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(3, 1, 5))
        .expect("actual relation");
    let graph = deps
        .admit_acyclic(FormulaSourceCycleLimits::new(3, 6))
        .expect("all vertices acyclic");
    let error = graph
        .prepare_scalar_sources(Limits::new(3, 8, 1, 23, 3))
        .expect_err("actual unavailable catalog family");
    let Refusal::MissingTangentPolicy { source_index, node } = error.refusal() else {
        unreachable!("real unavailable call");
    };
    assert_eq!(*source_index, 2);
    let source = checked
        .sources()
        .get(2)
        .expect("actual source with tangent");
    assert!(std::ptr::eq(node.expression(), source.proof().expression()));
    let span = node.span();
    assert_eq!(
        source
            .normalized()
            .source()
            .expression()
            .source()
            .get(span.start()..span.end()),
        Some("tan(1 deg)")
    );
}

#[test]
fn empty_source_collection_needs_no_prospective_scalar_runtime_or_outputs() {
    let canonical = CanonicalInputs::new(vec![], vec![]).expect("empty actual owner");
    let sources = FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))
        .expect("empty source entities");
    let normalized = FormulaSourceInputs::new(&canonical, &sources)
        .expect("actual empty joins")
        .normalize_sources(FormulaSourceNormalizationLimits::new(0, 0, 0))
        .expect("empty normalized owners");
    let checked = normalized
        .check_sources(
            &FormulaNamespace::new([]).expect("initial"),
            FormulaSourceCheckLimits::new(0, 0, 0, 0),
        )
        .expect("empty actual proofs");
    let deps = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(0, 0, 1))
        .expect("terminal offset");
    let graph = deps
        .admit_acyclic(FormulaSourceCycleLimits::new(0, 0))
        .expect("empty acyclic relation");
    let plan = graph
        .prepare_scalar_sources(Limits::new(0, 0, 0, 0, 0))
        .expect("zero empty scalar allowance");
    assert!(plan.sources().is_empty());
    assert_eq!(
        (
            plan.node_count(),
            plan.name_use_count(),
            plan.workspace_records(),
            plan.output_records()
        ),
        (0, 0, 0, 0)
    );
}
