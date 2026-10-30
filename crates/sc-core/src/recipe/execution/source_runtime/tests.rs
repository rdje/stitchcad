use super::*;
use crate::{name::MachineToken, ontology::EntityId, recipe::*, value::*};

#[test]
#[allow(clippy::expect_used)]
fn actual_multiple_demands_share_earned_unbound_roots_and_poison_after_failure() {
    let id = EntityId::from_bits;
    let inputs = CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(40),
            source: id(4000),
            state: LengthState::Derived { formula: id(9000) },
        })
        .expect("actual canonical Derived")],
        vec![],
    )
    .expect("actual inventory");
    let name = MachineToken::new("a").expect("actual parameter name");
    let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(
        FormulaDeclaration::length_input(
            &name,
            FormulaInputOrigin::Parameter,
            id(8000),
            inputs.length(id(40)).expect("actual canonical target"),
        ),
    )
    .expect("initial source")])
    .expect("actual initial namespace");
    let sources: Vec<_> = [
        (9000, "1 mm/3"),
        (9001, "a*3"),
        (9002, "a+a"),
        (9003, "1 mm/0"),
        (9004, "1 mm"),
    ]
    .into_iter()
    .map(|(identity, body)| {
        FormulaSource::new(
            id(identity),
            FormulaExpressionDocument::new(body.into()).expect("original source body"),
        )
    })
    .collect();
    let s = sources.len();
    let n = sources.iter().map(|s| s.expression().node_count()).sum();
    let limits = FormulaSourceLimits::new(
        s,
        sources.iter().map(|s| s.expression().source().len()).sum(),
        sources
            .iter()
            .map(|s| s.expression().canonical().as_str().len())
            .sum(),
    );
    let sources = FormulaSources::new(sources, limits).expect("actual original source registry");
    let normalized = FormulaSourceInputs::new(&inputs, &sources)
        .expect("all actual source joins")
        .normalize_sources(FormulaSourceNormalizationLimits::new(s, n, s + 2 * n))
        .expect("complete original arenas");
    let checked = normalized
        .check_sources(
            &namespace,
            FormulaSourceCheckLimits::new(s, n, 1, s + 2 * n),
        )
        .expect("complete actual source proofs");
    let dependencies = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(s, 3, s + 4))
        .expect("actual complete Derived edges");
    let graph = dependencies
        .admit_acyclic(FormulaSourceCycleLimits::new(s, 2 * s))
        .expect("actual acyclic sources");
    let plan = graph
        .prepare_scalar_sources(FormulaSourceGraphExecutionLimits::new(
            s,
            n,
            3,
            2 * s + 2 * n + 3,
            s,
        ))
        .expect("complete actual source plan");
    let context = Context::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    );
    let mut runtime = SourceRuntime::new(plan, context).expect("all runtime buffers before reads");
    assert_eq!(runtime.machines.len(), s);
    assert!(runtime.completed.iter().all(Option::is_none));
    assert!(runtime.frames.is_empty());
    assert_eq!(runtime.ordinal, 0);
    assert!(runtime
        .plan
        .sources()
        .iter()
        .flat_map(|s| s.plan().sources.iter())
        .all(|s| s.read.is_none()));
    let machines = runtime.machines.as_ptr();
    let frames = runtime.frames.as_ptr();
    let outputs = runtime.completed.as_ptr();
    let original = |identity| source_index(&checked, id(identity)).expect("actual source index");
    let first = runtime.demand(original(9001)).expect("actual a*3 root");
    assert_eq!(
        first.numeric().expect("Length").rational(),
        FormulaRational::new(1000, 1, false).expect("unbound exact result")
    );
    assert_eq!(runtime.ordinal, 2);
    let a = runtime
        .completed
        .get(original(9000))
        .copied()
        .flatten()
        .expect("actual computed supplier");
    assert_eq!(a.ordinal, 1);
    assert_eq!(
        a.value.numeric().expect("Length").rational(),
        FormulaRational::new(1000, 3, false).expect("bare root never stored")
    );
    assert_eq!(
        runtime.demand(original(9001)).expect("actual cached root"),
        first
    );
    assert_eq!(
        runtime.ordinal, 2,
        "cached demand cannot re-enter a successful source"
    );
    let second = runtime
        .demand(original(9002))
        .expect("another actual root shares a");
    assert_eq!(
        second.numeric().expect("Length").rational(),
        FormulaRational::new(2000, 3, false).expect("unbound shared result")
    );
    assert_eq!(runtime.ordinal, 3);
    assert_eq!(
        runtime
            .completed
            .get(original(9000))
            .copied()
            .flatten()
            .expect("same completed source")
            .ordinal,
        1
    );
    assert!(runtime
        .completed
        .get(original(9004))
        .expect("unused actual source")
        .is_none());
    assert_eq!(runtime.machines.as_ptr(), machines);
    assert_eq!(runtime.frames.as_ptr(), frames);
    assert_eq!(runtime.completed.as_ptr(), outputs);
    assert_eq!(
        inputs
            .length(id(40))
            .expect("unchanged actual state")
            .authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(40),
            formula: id(9000)
        })
    );
    let error = runtime
        .demand(original(9003))
        .expect_err("actual original division failure");
    assert!(matches!(
        error,
        Cause::Evaluation {
            source_index: 3,
            ..
        }
    ));
    assert!(runtime.failed);
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || runtime.demand(original(9000))
        ))
        .is_err(),
        "failure cannot return an earlier cached value"
    );
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| runtime.finish())).is_err(),
        "failure cannot publish a successful prefix"
    );
}
