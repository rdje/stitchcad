//! Actual original source graph cycles,independent curated oracle and bounded nonrecursive scratch.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBindingKind, FormulaCanonicalInputAdapter as Adapter, FormulaDeclaration,
        FormulaExpressionDocument, FormulaInitialDeclaration, FormulaNamespace,
        FormulaNormalizedSources, FormulaScalarInputOrigin, FormulaSource,
        FormulaSourceCheckLimits, FormulaSourceCycleLimits as Limits,
        FormulaSourceCycleRefusal as Refusal, FormulaSourceDependencyLimits, FormulaSourceInputs,
        FormulaSourceLimits, FormulaSourceNormalizationLimits, FormulaSources,
    },
    value::{
        CanonicalInputs, ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState,
    },
};
const FIXTURE: &str =
    include_str!("../../../docs/tasks/artifacts/formula_structure/source_cycle_cases.tsv");
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
struct Fixture {
    canonical: CanonicalInputs,
    sources: FormulaSources,
    names: Vec<MachineToken>,
}
impl Fixture {
    fn new(size: usize, edges: &[(usize, usize)], untaken: bool) -> Self {
        let canonical = CanonicalInputs::new(
            vec![],
            (0..size)
                .map(|index| {
                    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                        id: id(1000 + index as u128),
                        source: id(4000),
                        kind: ScalarInputKind::Count,
                        state: ScalarInputState::Derived {
                            formula: id(9000 + index as u128),
                        },
                    })
                    .expect("actual canonical Derived Count")
                })
                .collect(),
        )
        .expect("actual unique owners");
        let names: Vec<_> = (0..size)
            .map(|index| MachineToken::new(format!("v{index}")).expect("actual name"))
            .collect();
        let sources: Vec<_> = (0..size)
            .map(|consumer| {
                let uses: Vec<_> = edges
                    .iter()
                    .filter(|(source, _)| *source == consumer)
                    .map(|(_, supplier)| format!("v{supplier}"))
                    .collect();
                let source = if uses.is_empty() {
                    "1".to_owned()
                } else if untaken && consumer == 0 {
                    format!("if(1==2,{},1)", uses.join("+"))
                } else {
                    uses.join("+")
                };
                FormulaSource::new(
                    id(9000 + consumer as u128),
                    FormulaExpressionDocument::new(source).expect("actual syntax"),
                )
            })
            .collect();
        let limits = FormulaSourceLimits::new(
            size,
            sources.iter().map(|s| s.expression().source().len()).sum(),
            sources
                .iter()
                .map(|s| s.expression().canonical().as_str().len())
                .sum(),
        );
        Self {
            canonical,
            sources: FormulaSources::new(sources, limits).expect("actual complete sources"),
            names,
        }
    }
    fn normalized(&self) -> FormulaNormalizedSources<'_> {
        let size = self.sources.sources().len();
        let nodes: usize = self
            .sources
            .sources()
            .iter()
            .map(|s| s.expression().node_count())
            .sum();
        FormulaSourceInputs::new(&self.canonical, &self.sources)
            .expect("actual joins")
            .normalize_sources(FormulaSourceNormalizationLimits::new(
                size,
                nodes,
                size + 2 * nodes,
            ))
            .expect("complete normalization")
    }
    fn namespace(&self) -> FormulaNamespace<'_> {
        FormulaNamespace::new(self.names.iter().enumerate().map(|(index, name)| {
            FormulaInitialDeclaration::try_from(FormulaDeclaration::input(
                name,
                FormulaScalarInputOrigin::Parameter,
                id(5000 + index as u128),
                id(1000 + index as u128),
                FormulaBindingKind::Count,
            ))
            .expect("actual initial metadata")
        }))
        .expect("actual namespace")
    }
}
fn parse_edges(text: &str) -> Vec<(usize, usize)> {
    if text == "-" {
        return vec![];
    }
    text.split(',')
        .map(|pair| {
            let (consumer, supplier) = pair.split_once('>').expect("authored edge");
            (
                consumer.parse().expect("consumer"),
                supplier.parse().expect("supplier"),
            )
        })
        .collect()
}

#[test]
fn curated_independent_graphs_match_actual_cycle_verdicts_and_every_original_witness_edge() {
    let mut rows = 0;
    for row in FIXTURE
        .lines()
        .filter(|row| !row.is_empty() && !row.starts_with('#'))
    {
        let fields: Vec<_> = row.split('\t').collect();
        let size: usize = fields.get(1).expect("vertices").parse().expect("size");
        let edges = parse_edges(fields.get(2).expect("original directed edges"));
        let expected = *fields.get(3).expect("independent verdict") == "true";
        let fixture = Fixture::new(size, &edges, false);
        let normalized = fixture.normalized();
        let checked = normalized
            .check_sources(
                &fixture.namespace(),
                FormulaSourceCheckLimits::new(
                    size,
                    normalized.node_count(),
                    size,
                    size + 2 * normalized.node_count(),
                ),
            )
            .expect("actual complete expression/target proofs");
        let deps = checked
            .prepare_dependencies(FormulaSourceDependencyLimits::new(
                size,
                edges.len(),
                size + edges.len() + 1,
            ))
            .expect("complete original relation");
        let limits = Limits::new(size, 2 * size);
        assert_eq!(limits.sources(), size);
        assert_eq!(limits.workspace_records(), 2 * size);
        match deps.admit_acyclic(limits) {
            Ok(proof) => {
                assert!(expected, "unexpected acyclic row:{row}");
                assert!(std::ptr::eq(proof.dependencies(), &deps));
                assert_eq!(proof.limits(), limits);
                assert_eq!(proof.workspace_records(), 2 * size);
            }
            Err(error) => {
                assert!(!expected, "unexpected cycle row:{row};{error:?}");
                assert!(std::ptr::eq(error.dependencies(), &deps));
                assert_eq!(error.limits(), limits);
                let Refusal::Cycle(cycle) = error.refusal() else {
                    unreachable!("actual cycle cause");
                };
                assert!(std::ptr::eq(cycle.dependencies(), &deps));
                let steps: Vec<_> = cycle.steps().collect();
                assert!(!steps.is_empty() && steps.len() <= size);
                assert_eq!(cycle.step_count(), steps.len());
                let endpoints: Vec<_> = steps
                    .iter()
                    .map(|step| {
                        (
                            step.consumer().normalized().source().id().as_bits() - 9000,
                            step.supplier().normalized().source().id().as_bits() - 9000,
                        )
                    })
                    .collect();
                for (step, (consumer, supplier)) in steps.iter().zip(&endpoints) {
                    assert!(edges
                        .iter()
                        .any(|(a, b)| *a as u128 == *consumer && *b as u128 == *supplier));
                    let span = step.name_use().span();
                    let actual = step
                        .consumer()
                        .normalized()
                        .source()
                        .expression()
                        .source()
                        .get(span.start()..span.end());
                    assert_eq!(actual, Some(format!("v{supplier}").as_str()));
                    let Adapter::Scalar(canonical) = step.canonical_input().adapter() else {
                        unreachable!("actual general Count fixture");
                    };
                    assert_eq!(
                        canonical.state(),
                        &ScalarInputState::Derived {
                            formula: id(9000 + *supplier)
                        }
                    );
                }
                for pair in endpoints.windows(2) {
                    let [left, right] = pair else {
                        unreachable!("two adjacent original edges");
                    };
                    assert_eq!(left.1, right.0);
                }
                assert_eq!(
                    endpoints.first().expect("first").0,
                    endpoints.last().expect("last").1
                );
                assert_eq!(cycle.steps().rev().count(), cycle.step_count());
                assert!(!format!("{error:?}").contains("v0"));
            }
        }
        rows += 1;
    }
    assert_eq!(rows, 9);
}

#[test]
fn independent_standard_library_oracle_watches_every_tracked_curated_graph_verdict() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/source_cycle_reference.py",
        ])
        .current_dir(root)
        .output()
        .expect("independent read-only graphlib oracle");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        b"source cycle reference: 9 curated graphlib cases pass\n"
    );
}

#[test]
fn complete_quotas_precede_walk_and_untaken_cycles_are_never_hidden() {
    let fixture = Fixture::new(2, &[(0, 1), (1, 0)], true);
    let normalized = fixture.normalized();
    let checked = normalized
        .check_sources(
            &fixture.namespace(),
            FormulaSourceCheckLimits::new(
                2,
                normalized.node_count(),
                2,
                2 + 2 * normalized.node_count(),
            ),
        )
        .expect("actual whole proofs");
    let deps = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(2, 2, 5))
        .expect("actual whole relation");
    let sources = deps
        .admit_acyclic(Limits::new(1, 0))
        .expect_err("complete source count first");
    assert!(matches!(
        sources.refusal(),
        Refusal::Sources {
            allowed: 1,
            required: 2
        }
    ));
    let scratch = deps
        .admit_acyclic(Limits::new(2, 3))
        .expect_err("full scratch before traversal");
    assert!(matches!(
        scratch.refusal(),
        Refusal::WorkspaceRecords {
            allowed: 3,
            required: 4
        }
    ));
    assert!(std::error::Error::source(&scratch).is_none());
    assert!(matches!(
        deps.admit_acyclic(Limits::new(2, 4))
            .expect_err("actual untaken cycle remains")
            .refusal(),
        Refusal::Cycle(_)
    ));
}

#[test]
fn long_actual_chain_checks_on_small_stack_without_recursion_or_changing_original_edges() {
    let size = 512;
    let edges: Vec<_> = (0..size - 1).map(|source| (source, source + 1)).collect();
    let fixture = Fixture::new(size, &edges, false);
    let normalized = fixture.normalized();
    let checked = normalized
        .check_sources(
            &fixture.namespace(),
            FormulaSourceCheckLimits::new(
                size,
                normalized.node_count(),
                size,
                size + 2 * normalized.node_count(),
            ),
        )
        .expect("actual complete proofs");
    let deps = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(
            size,
            edges.len(),
            size + edges.len() + 1,
        ))
        .expect("actual complete chain");
    let before: Vec<_> = deps
        .edges()
        .map(|edge| {
            (
                edge.consumer().normalized().source().id(),
                edge.supplier().normalized().source().id(),
                edge.name_use().span(),
            )
        })
        .collect();
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(64 * 1024)
            .spawn_scoped(scope, || {
                let proof = deps
                    .admit_acyclic(Limits::new(size, 2 * size))
                    .expect("bounded iterative full chain");
                assert!(std::ptr::eq(proof.dependencies(), &deps));
            })
            .expect("small-stack worker")
            .join()
            .expect("no recursive traversal overflow");
    });
    let after: Vec<_> = deps
        .edges()
        .map(|edge| {
            (
                edge.consumer().normalized().source().id(),
                edge.supplier().normalized().source().id(),
                edge.name_use().span(),
            )
        })
        .collect();
    assert_eq!(after, before);
}
