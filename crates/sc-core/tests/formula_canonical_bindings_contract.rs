//! Independently authored binding bytes and actual fresh replay, without a project format claim.
#![allow(clippy::expect_used)]
use sc_core::{
    recipe::{
        FormulaCanonicalBindings, FormulaCanonicalBindingsError as Error, FormulaNamespace,
        FormulaRecipe, FormulaRecipeExecutionLimits, FormulaScalarExecutionContext,
        FormulaSizeContext, FormulaTangentLimits, FormulaToleranceContext,
    },
    value::CanonicalInputs,
};

fn output(text: &str) -> FormulaCanonicalBindings {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let recipe = FormulaRecipe::parse(text)
        .expect("syntax")
        .normalize_literals()
        .expect("input");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("complete proof");
    let plan = proof
        .admit_execution(FormulaRecipeExecutionLimits::new(16, 256, 1024, 16))
        .expect("explicit logical allowances")
        .prepare_scalar_with_tangent_limits(&inputs, FormulaTangentLimits::new(2073, 512))
        .expect("complete original sources/policy");
    let context = FormulaScalarExecutionContext::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    );
    let first = plan.replay(context).expect("actual complete execution");
    let second = first
        .plan()
        .replay(context)
        .expect("fresh statement-one replay");
    let bytes = first
        .canonical_bindings(8192)
        .expect("explicit output allowance");
    assert_eq!(
        bytes,
        second.canonical_bindings(8192).expect("replayed output")
    );
    assert_eq!(
        bytes,
        first
            .canonical_bindings(bytes.as_str().len())
            .expect("exact allowance")
    );
    bytes
}

#[test]
fn exact_authored_bytes_preserve_all_six_stored_kinds_and_declaration_order() {
    let source="let l:length=-1 um/2\nlet a:angle=450 deg\nlet area:area=2 um*3 um\nlet r:ratio=-0.000001/2\nlet count:count=2\nlet flag:boolean=1==2";
    let expected="(bindings (bind l length:-1 (sources)) (bind a angle:450000000 (sources)) (bind area area:6 (sources)) (bind r ratio:-1 (sources)) (bind count count:2 (sources)) (bind flag boolean:false (sources)))";
    let actual = output(source);
    assert_eq!(actual.as_str(), expected);
    assert!(actual.as_str().is_ascii());
    assert_eq!(actual.as_str(), actual.as_str().trim());
    assert!(!actual.as_str().contains('\n'));
    assert_ne!(
        output("let first:count=1\nlet second:count=2"),
        output("let second:count=2\nlet first:count=1")
    );
    assert_ne!(output("let a:count=1"), output("let b:count=1"));
    assert_ne!(output("let a:count=1"), output("let a:length=1 um"));
}

#[test]
fn approximation_sources_are_lexical_and_survive_boolean_and_zero_with_lazy_exclusion() {
    let actual=output("let a:ratio=sin(90 deg)\nlet b:ratio=cos(0 deg)\nlet flag:boolean=a==b\nlet zero:ratio=(a-a)+(b-b)\nlet selected:ratio=if(1==1,sin(90 deg),cos(0 deg))");
    assert_eq!(actual.as_str(),"(bindings (bind a ratio:1000000 (sources sin)) (bind b ratio:1000000 (sources cos)) (bind flag boolean:true (sources cos sin)) (bind zero ratio:0 (sources cos sin)) (bind selected ratio:1000000 (sources sin)))");
    assert_ne!(output("let x:ratio=1.0"), output("let x:ratio=sin(90 deg)"));
    // These bytes identify stored bindings only; the fractional RHS and true assertion are omitted.
    assert_eq!(
        output("let x:length=1 um/3"),
        output("let x:length=0 um\nassert ok:eps_num=x==0 um")
    );
}

#[test]
fn complete_byte_quota_refuses_every_short_allowance_and_preserves_accepted_result() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let recipe = FormulaRecipe::parse("let x:ratio=sin(90 deg)\nlet y:length=1 um/3")
        .expect("syntax")
        .normalize_literals()
        .expect("input");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("proof");
    let result = proof
        .admit_execution(FormulaRecipeExecutionLimits::new(2, 16, 64, 2))
        .expect("limits")
        .prepare_scalar(&inputs)
        .expect("whole sources")
        .evaluate(FormulaScalarExecutionContext::new(
            FormulaToleranceContext::empty(),
            FormulaSizeContext::empty(),
        ))
        .expect("accepted execution");
    let expected = "(bindings (bind x ratio:1000000 (sources sin)) (bind y length:0 (sources)))";
    for allowed in 0..expected.len() {
        let error = result
            .canonical_bindings(allowed)
            .expect_err("complete output exceeds selected quota");
        assert!(
            matches!(error,Error::Bytes { allowed:a,required } if a==allowed && required==expected.len())
        );
        assert_eq!(result.statements().len(), 2);
    }
    assert_eq!(
        result
            .canonical_bindings(expected.len())
            .expect("exact bytes")
            .as_str(),
        expected
    );
    assert_eq!(
        result
            .canonical_bindings(usize::MAX)
            .expect("larger allowance")
            .as_str(),
        expected
    );
}

#[test]
fn empty_output_and_owned_private_bytes_outlive_source_without_debug_content() {
    assert_eq!(output("").as_str(), "(bindings)");
    assert_eq!(
        output("assert ok:eps_num=1 um==1 um").as_str(),
        "(bindings)"
    );
    let actual = output("let customer_secret:length=123456 um");
    let debug = format!("{actual:?}");
    assert!(!debug.contains("customer_secret"));
    assert!(!debug.contains("123456"));
    let mut edited = actual.clone().into_string();
    edited.clear();
    assert_eq!(
        actual.as_str(),
        "(bindings (bind customer_secret length:123456 (sources)))"
    );
}
