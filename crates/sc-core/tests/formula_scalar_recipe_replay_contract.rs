//! Public fresh replay: real stored values, current reads and failure isolation.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    recipe::{
        FormulaExecutedScalarRecipe, FormulaNamespace, FormulaRational, FormulaRecipe,
        FormulaRecipeExecutionLimits, FormulaScalarExecutionContext as Context,
        FormulaScalarRecipeErrorCause, FormulaScalarRecipeFailure,
        FormulaScalarRecipeOutcome as Outcome, FormulaScalarRecipeReplayError as ReplayError,
        FormulaSizeContext, FormulaTangentLimits, FormulaToleranceContext,
        FormulaToleranceContextDefinition, FormulaToleranceProvider,
    },
    value::CanonicalInputs,
};
use sc_units::{Length, Tolerance, ToleranceClass};

const LIMITS: FormulaRecipeExecutionLimits = FormulaRecipeExecutionLimits::new(8, 64, 192, 8);
fn context() -> Context<'static> {
    Context::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
fn physical(value: i64) -> Tolerance {
    Tolerance::new(
        ToleranceClass::PhysicalAcceptance,
        Length::from_micrometres(value).expect("positive fixture length"),
        "replay fixture factory",
    )
    .expect("explicit physical tolerance")
}
fn with_physical(value: &Tolerance) -> Context<'_> {
    Context::new(
        FormulaToleranceContext::new(FormulaToleranceContextDefinition {
            label: Some("replay_fixture"),
            format: None,
            importer: None,
            physical: Some(FormulaToleranceProvider {
                source: EntityId::from_bits(90),
                tolerance: value,
            }),
        })
        .expect("actual checked provider"),
        FormulaSizeContext::empty(),
    )
}
// Test projection only: this is not a persistence format or a complete result identity.
fn stored(result: &FormulaExecutedScalarRecipe<'_>) -> Vec<i64> {
    result
        .statements()
        .iter()
        .filter_map(|statement| match statement.outcome() {
            Outcome::Binding { binding, .. } => Some(
                binding
                    .value()
                    .numeric_integer()
                    .expect("numeric binding fixture"),
            ),
            Outcome::Assertion(_) => None,
        })
        .collect()
}
fn reads<'a>(result: &FormulaExecutedScalarRecipe<'a>) -> Vec<&'a str> {
    result
        .reads()
        .map(|read| read.dependency().declaration().name())
        .collect()
}

#[test]
fn repeated_replay_matches_fresh_execution_and_independently_expected_stored_values() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let before = inputs.clone();
    let recipe = FormulaRecipe::parse(
        "let first:length=1 um/3\nlet second:length=first+1 um\nassert check:eps_num=second==1 um",
    )
    .expect("syntax")
    .normalize_literals()
    .expect("normalization");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let plan = proof
        .admit_execution(LIMITS)
        .expect("explicit limits")
        .prepare_scalar(&inputs)
        .expect("whole preparation");
    let first = plan.replay(context()).expect("first fresh run");
    let second = first
        .plan()
        .replay(context())
        .expect("accepted plan replay");
    let fresh = plan
        .evaluate(context())
        .expect("ordinary from-statement-one run");
    for result in [&first, &second, &fresh] {
        // 1/3 um stores as 0 um; the second statement reads that stored zero.
        assert_eq!(stored(result), [0, 1]);
        assert_eq!(reads(result), ["first", "second", "eps_num"]);
        assert_eq!(result.statements().len(), 3);
        assert!(std::ptr::eq(result.plan().admission().proof(), &proof));
        assert!(std::ptr::eq(result.plan().inputs(), &inputs));
        assert_eq!(result.plan().admission().limits(), LIMITS);
        assert_eq!(result.plan().tangent_limits(), None);
        let Outcome::Binding { expression, .. } = result
            .statements()
            .first()
            .expect("actual first statement")
            .outcome()
        else {
            unreachable!("binding fixture");
        };
        assert_eq!(
            expression.numeric().expect("Length").rational(),
            FormulaRational::new(1, 3, false).expect("exact rational")
        );
    }
    assert_eq!(inputs, before);
}

#[test]
fn changed_context_replaces_read_trace_and_late_failure_preserves_prior_accepted_result() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let recipe = FormulaRecipe::parse("let prior:length=1 um/3\nlet chosen:length=if(eps_phys<5 um,prior,9 um)\nassert check:eps_phys=chosen==0 um")
        .expect("syntax").normalize_literals().expect("normalization");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let plan = proof
        .admit_execution(LIMITS)
        .expect("explicit limits")
        .prepare_scalar(&inputs)
        .expect("whole preparation");
    let small = physical(1);
    let large = physical(10);
    let refuses = physical(6);
    let accepted = plan.replay(with_physical(&small)).expect("first result");
    let changed = accepted
        .plan()
        .replay(with_physical(&large))
        .expect("fresh other branch");
    assert_eq!(stored(&accepted), [0, 0]);
    assert_eq!(stored(&changed), [0, 9]);
    assert_eq!(
        reads(&accepted),
        ["eps_phys", "prior", "chosen", "eps_phys"]
    );
    assert_eq!(reads(&changed), ["eps_phys", "chosen", "eps_phys"]);
    // A read from the earlier selected branch cannot survive into this unselected branch.
    for read in changed
        .reads()
        .filter(|read| read.dependency().declaration().name() == "eps_phys")
    {
        assert_eq!(
            read.value().numeric().expect("Length").rational(),
            FormulaRational::from_signed_integer(10)
        );
    }
    let ReplayError::Execution(error) = accepted
        .plan()
        .replay(with_physical(&refuses))
        .expect_err("late assertion must refuse the whole new attempt")
    else {
        unreachable!("actual execution failure");
    };
    assert!(std::ptr::eq(error.proof(), &proof));
    assert_eq!(
        error
            .statement()
            .expect("real failed assertion")
            .statement_index(),
        3
    );
    assert!(
        matches!(error.cause(), FormulaScalarRecipeErrorCause::Evaluation(cause)
        if matches!(cause.as_ref(), FormulaScalarRecipeFailure::Assertion(_)))
    );
    assert_eq!(stored(&accepted), [0, 0]);
    assert_eq!(
        reads(&accepted),
        ["eps_phys", "prior", "chosen", "eps_phys"]
    );
    let again = changed
        .plan()
        .replay(with_physical(&small))
        .expect("fresh selected branch again");
    assert_eq!(stored(&again), [0, 0]);
    assert_eq!(reads(&again), reads(&accepted));
}

#[test]
fn replay_retains_explicit_tangent_policy_in_success_and_failure_and_accepts_empty_recipe() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let recipe = FormulaRecipe::parse("let prior:count=1\nlet t:ratio=tan(30 deg)")
        .expect("syntax")
        .normalize_literals()
        .expect("normalization");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    for policy in [
        FormulaTangentLimits::new(2073, 512),
        FormulaTangentLimits::new(0, 0),
    ] {
        let plan = proof
            .admit_execution(LIMITS)
            .expect("limits")
            .prepare_scalar_with_tangent_limits(&inputs, policy)
            .expect("explicit policy");
        match plan.replay(context()) {
            Ok(result) => {
                assert_ne!(policy, FormulaTangentLimits::new(0, 0));
                assert_eq!(stored(&result), [1, 577350]);
                assert_eq!(result.plan().tangent_limits(), Some(policy));
                let replay = result
                    .plan()
                    .replay(context())
                    .expect("same explicit policy");
                assert_eq!(stored(&replay), stored(&result));
                assert_eq!(replay.plan().tangent_limits(), Some(policy));
            }
            Err(ReplayError::Execution(error)) => {
                assert_eq!(policy, FormulaTangentLimits::new(0, 0));
                assert_eq!(error.tangent_limits(), Some(policy));
                assert_eq!(
                    error.statement().expect("Tan statement").statement_index(),
                    2
                );
            }
            Err(error) => unreachable!("unexpected preparation failure: {error}"),
        }
    }
    let empty = FormulaRecipe::parse("")
        .expect("empty syntax")
        .normalize_literals()
        .expect("empty normalization");
    let proof = empty
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("empty proof");
    let plan = proof
        .admit_execution(FormulaRecipeExecutionLimits::new(0, 0, 0, 0))
        .expect("zero allowances")
        .prepare_scalar(&inputs)
        .expect("empty preparation");
    let result = plan.replay(context()).expect("empty replay");
    assert!(result.statements().is_empty());
    assert_eq!(result.reads().count(), 0);
}
