//! Executed-call source metadata, independently enumerated against the normative source contract.
#![allow(clippy::expect_used)]
use sc_core::recipe::{
    FormulaApproximationSources as Sources, FormulaBuiltin as B, FormulaToleranceName as T,
};

const EXPECTED: [&str; 11] = [
    "arc_length",
    "atan",
    "atan2",
    "cos",
    "dir",
    "dist",
    "hypot",
    "round_to",
    "sin",
    "sqrt",
    "tan",
];

fn names(sources: Sources) -> Vec<&'static str> {
    sources.iter().map(B::token).collect()
}

#[test]
fn every_builtin_has_its_actual_introduced_source_or_no_source() {
    for call in B::ALL {
        let actual = Sources::for_call(call);
        let wanted = EXPECTED.contains(&call.token());
        assert_eq!(!actual.is_empty(), wanted, "{}", call.token());
        assert_eq!(actual.contains(call), wanted, "{}", call.token());
        assert_eq!(
            names(actual),
            if wanted { vec![call.token()] } else { vec![] }
        );
        for other in B::ALL {
            assert_eq!(actual.contains(other), wanted && call == other);
        }
    }
    assert_eq!(names(Sources::default()), Vec::<&str>::new());
}

#[test]
fn unions_are_immutable_idempotent_order_independent_and_lexically_reported() {
    let forward = B::ALL.into_iter().fold(Sources::EMPTY, Sources::with_call);
    let reverse = B::ALL
        .into_iter()
        .rev()
        .fold(Sources::EMPTY, Sources::with_call);
    assert_eq!(forward, reverse);
    assert_eq!(names(forward), EXPECTED);
    assert_eq!(forward.union(forward), forward);
    assert_eq!(forward.union(Sources::EMPTY), forward);
    for left in B::ALL {
        for right in B::ALL {
            let a = Sources::for_call(left);
            let b = Sources::for_call(right);
            assert_eq!(a.union(b), b.union(a));
            assert_eq!(a.with_call(right), a.union(b));
            assert_eq!(a, Sources::for_call(left));
            let old = names(a);
            let _combined = a.union(b);
            assert_eq!(names(a), old);
        }
    }
}

#[test]
fn t1_refuses_each_actual_source_and_retains_the_requested_symbol_and_all_sources() {
    for sources in B::ALL
        .into_iter()
        .map(Sources::for_call)
        .chain([B::ALL.into_iter().fold(Sources::EMPTY, Sources::with_call)])
    {
        for tolerance in [
            T::Numerical,
            T::Geometric,
            T::Format,
            T::Importer,
            T::Physical,
        ] {
            let result = sources.check_tolerance(tolerance);
            if tolerance == T::Numerical && !sources.is_empty() {
                let error = result.expect_err("T1 with approximation sources");
                assert_eq!(error.tolerance(), tolerance);
                assert_eq!(error.sources(), sources);
                assert_eq!(error.diagnostic_code(), "formula_domain");
                let rendered = error.to_string();
                assert!(rendered.contains("eps_num"));
                assert!(rendered.contains("T2 or looser"));
                for source in sources.iter() {
                    assert!(rendered.contains(source.token()));
                }
            } else {
                result.expect("no T1 approximation refusal");
            }
        }
    }
}

#[test]
fn explicit_executed_operand_merges_keep_condition_sources_and_exclude_untaken_branch() {
    let condition = Sources::for_call(B::Hypot);
    let taken = Sources::EMPTY;
    let untaken = Sources::for_call(B::Sin);
    let selected = taken.union(condition).with_call(B::If);
    assert_eq!(names(selected), ["hypot"]);
    assert!(!selected.contains(B::Sin));
    assert!(selected.check_tolerance(T::Numerical).is_err());
    assert_eq!(names(untaken), ["sin"]);
    // Ordinary/cancelling operations and selecting an extremum must merge evaluated inputs,
    // rather than discard a source because the result equals an exact integer or one candidate.
    let candidates = Sources::for_call(B::Sqrt).union(untaken);
    assert_eq!(names(candidates.with_call(B::Min)), ["sin", "sqrt"]);
    assert_eq!(names(candidates.with_call(B::Max)), ["sin", "sqrt"]);
    assert_eq!(names(candidates.with_call(B::Clamp)), ["sin", "sqrt"]);
    assert_eq!(names(Sources::for_call(B::RoundTo)), ["round_to"]);
}
