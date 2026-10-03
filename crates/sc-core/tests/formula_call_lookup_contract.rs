//! Independently authored closed calls and exact typed refusal payloads; no value execution.
use sc_core::name::MachineToken;
use sc_core::recipe::{
    FormulaBuiltin as B, FormulaCallAlternative as A, FormulaCallLookupSource as S,
    FormulaCallRefusal, FormulaCallRefusalKind as R, FormulaNamespace,
};

const CALLS: [(&str, B); 21] = [
    ("sqrt", B::Sqrt),
    ("hypot", B::Hypot),
    ("abs", B::Abs),
    ("min", B::Min),
    ("max", B::Max),
    ("clamp", B::Clamp),
    ("round_to", B::RoundTo),
    ("sin", B::Sin),
    ("cos", B::Cos),
    ("tan", B::Tan),
    ("atan", B::Atan),
    ("atan2", B::Atan2),
    ("arc_length", B::ArcLength),
    ("within", B::Within),
    ("x", B::X),
    ("y", B::Y),
    ("dist", B::Dist),
    ("dir", B::Dir),
    ("len", B::Len),
    ("param_at", B::ParamAt),
    ("point_at", B::PointAt),
];

fn refuse(name: &MachineToken, reason: R, token: &str, sources: &[S], alternatives: &[A]) {
    let result = B::resolve_call(name);
    assert!(
        result.is_err(),
        "callee must refuse before argument checking"
    );
    let Err(error) = result else { return };
    assert_eq!(error.name(), name.as_str());
    assert!(core::ptr::eq(error.name().as_ptr(), name.as_str().as_ptr()));
    assert_eq!(error.kind(), reason);
    assert_eq!(error.token(), token);
    assert_eq!(error.lookup_scope(), "formula_call");
    assert_eq!(error.origins_searched(), sources);
    assert_eq!(error.alternatives(), alternatives);
    assert_eq!(error.to_string(), token);
    assert!(!format!("{error:?}").contains(name.as_str()));
    let copied = error;
    assert_eq!(copied.name(), name.as_str());
    let _: &dyn std::error::Error = &error;
}

#[test]
fn exact_closed_ordinary_calls_leave_if_as_the_existing_special_form() {
    for (name, expected) in CALLS {
        let name = MachineToken::new(name).expect("normative ordinary callee");
        assert_eq!(B::resolve_call(&name).expect("declared call"), expected);
    }
    let ordinary: Vec<_> = B::ALL.into_iter().filter(|item| *item != B::If).collect();
    assert_eq!(ordinary, CALLS.map(|(_, item)| item));
    for keyword in ["let", "assert", "if"] {
        assert!(MachineToken::new(keyword).is_err());
    }
    for invalid in ["Sin", "sin ", " sin", "sin__x", "é", "sin()", ""] {
        assert!(MachineToken::new(invalid).is_err());
    }
}

#[test]
fn six_envelope_aliases_retain_exact_request_and_only_the_searched_envelope() {
    for alias in ["nurbs", "spline", "bspline"] {
        let name = MachineToken::new(alias).expect("envelope identifier");
        refuse(
            &name,
            R::Nurbs,
            "env_nurbs",
            &[S::Envelope],
            &[A::LineSegment, A::CircularArc, A::CubicBezier],
        );
    }
    for alias in ["solve", "constraint", "fixpoint"] {
        let name = MachineToken::new(alias).expect("envelope identifier");
        refuse(
            &name,
            R::SketchConstraints,
            "env_sketch_constraints",
            &[S::Envelope],
            &[A::OrderedConstructionRecipe],
        );
    }
}

#[test]
fn undeclared_and_reserved_data_names_are_not_callables_or_envelope_aliases() {
    for spelling in [
        "loop",
        "unlisted_call",
        "sin_missing",
        "nurbs_extra",
        "solve_extra",
        "waist",
        "macro",
        "fn",
        "function",
        "while",
        "repeat",
        "eps_num",
        "eps_geo",
        "eps_fmt",
        "eps_imp",
        "eps_phys",
        "size_index",
        "size_count",
        "is_base_size",
        "line_segment",
        "circular_arc",
        "cubic_bezier",
        "ordered_construction_recipe",
    ] {
        let name = MachineToken::new(spelling).expect("ordinary identifier remains grammar-valid");
        refuse(
            &name,
            R::Unbound,
            "formula_unbound_name",
            &[S::Envelope, S::BuiltinCatalog],
            &[],
        );
    }
}

#[test]
fn data_reads_and_calls_have_distinct_domains_without_namespace_authority() {
    let namespace = FormulaNamespace::new([]).expect("empty authored namespace");
    let sin = MachineToken::new("sin").expect("ordinary identifier");
    assert!(namespace.resolve(&sin).is_err());
    assert_eq!(B::resolve_call(&sin).expect("catalog callable"), B::Sin);
    for name in ["eps_num", "size_count", "is_base_size"] {
        let name = MachineToken::new(name).expect("reserved data identifier");
        assert!(namespace.resolve(&name).is_ok());
        assert!(B::resolve_call(&name).is_err());
    }
    // The lookup API accepts no namespace, argument, state, geometry or execution provider.
    fn metadata_only<'a>(name: &'a MachineToken) -> Result<B, FormulaCallRefusal<'a>> {
        B::resolve_call(name)
    }
    assert_eq!(metadata_only(&sin).expect("no provider needed"), B::Sin);
}

#[test]
fn diagnostic_source_and_alternative_tags_match_the_declared_contract() {
    for (source, tag) in [
        (S::Envelope, "envelope"),
        (S::BuiltinCatalog, "builtin_catalog"),
    ] {
        assert_eq!(source.token(), tag);
    }
    for (alternative, tag) in [
        (A::LineSegment, "line_segment"),
        (A::CircularArc, "circular_arc"),
        (A::CubicBezier, "cubic_bezier"),
        (A::OrderedConstructionRecipe, "ordered_construction_recipe"),
    ] {
        assert_eq!(alternative.token(), tag);
    }
}
