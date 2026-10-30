//! Complete original source shape admission and real source-borrowed normalized owners.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    recipe::{
        FormulaExpressionDocument as Document, FormulaNamespace, FormulaSource,
        FormulaSourceInputs as Inputs, FormulaSourceLimits,
        FormulaSourceNormalizationLimits as Limits, FormulaSourceNormalizationRefusal as Refusal,
        FormulaSources,
    },
    value::CanonicalInputs,
};

fn sources(texts: &[&str]) -> FormulaSources {
    let sources: Vec<_> = texts
        .iter()
        .enumerate()
        .map(|(index, source)| {
            FormulaSource::new(
                EntityId::from_bits(9000 + index as u128),
                Document::new((*source).into()).expect("syntax"),
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
    FormulaSources::new(sources, limits).expect("actual complete source owners")
}

#[test]
fn immutable_factory_counts_match_existing_independent_reference_shape_rows_before_reparse() {
    let rows = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/normalized_expression_shapes.tsv"
    );
    let mut count = 0;
    for row in rows
        .lines()
        .filter(|row| !row.is_empty() && !row.starts_with('#'))
    {
        let fields: Vec<_> = row.split('\t').collect();
        let source = fields.first().expect("original source");
        let expected: usize = fields
            .get(2)
            .expect("independent node count")
            .parse()
            .expect("count");
        let document = Document::new((*source).into()).expect("actual original document");
        assert_eq!(document.node_count(), expected, "{source}");
        assert_eq!(document.clone().node_count(), expected);
        let originals = sources(&[source]);
        let canonical = CanonicalInputs::new(vec![], vec![]).expect("empty canonical owner");
        let joined = Inputs::new(&canonical, &originals).expect("complete actual admission");
        let normalized = joined
            .normalize_sources(Limits::new(1, expected, 1 + 2 * expected))
            .expect("exact actual retained allowances");
        assert_eq!(normalized.node_count(), expected);
        assert_eq!(
            normalized
                .sources()
                .first()
                .expect("actual source")
                .expression()
                .node_count(),
            expected
        );
        count += 1;
    }
    assert_eq!(count, 24);
}

#[test]
fn actual_original_sources_spans_and_input_context_survive_complete_normalization_in_order() {
    let originals = sources(&[" (2.5 cm + eps_num) ", "if(1==1,25 mm,missing)"]);
    let canonical = CanonicalInputs::new(vec![], vec![]).expect("empty canonical inventory");
    let inputs = Inputs::new(&canonical, &originals).expect("actual owners");
    // Three nodes plus six nodes, including the untaken missing name; neither is evaluated.
    let limits = Limits::new(2, 9, 20);
    assert_eq!(limits.sources(), 2);
    assert_eq!(limits.nodes(), 9);
    assert_eq!(limits.retained_records(), 20);
    let normalized = inputs
        .normalize_sources(limits)
        .expect("complete admitted owners");
    assert_eq!(normalized.limits(), limits);
    assert_eq!(normalized.node_count(), 9);
    assert_eq!(normalized.retained_records(), 20);
    assert!(std::ptr::eq(normalized.inputs().inputs(), &canonical));
    assert!(std::ptr::eq(normalized.inputs().sources(), &originals));
    for (actual, source) in normalized.sources().iter().zip(originals.sources()) {
        assert!(std::ptr::eq(actual.source(), source));
        assert_eq!(
            &actual.expression().canonical_form(),
            source.expression().canonical()
        );
    }
    let first = normalized.sources().first().expect("actual first source");
    let span = first.expression().root().span();
    assert_eq!(
        first
            .source()
            .expression()
            .source()
            .get(span.start()..span.end()),
        Some("(2.5 cm + eps_num)")
    );
    let last = normalized.sources().last().expect("actual second source");
    assert_eq!(
        last.expression()
            .check_kinds(&FormulaNamespace::new([]).expect("namespace"))
            .expect_err("complete static name check still required")
            .token(),
        "formula_unbound_name"
    );
    assert!(!format!("{normalized:?}").contains("missing"));
}

#[test]
fn complete_quotas_include_unused_sources_all_branches_and_no_accepted_prefix() {
    let originals = sources(&["1 mm", "if(1==1,1 mm,missing)"]);
    let canonical =
        CanonicalInputs::new(vec![], vec![]).expect("no consumers; sources still count");
    let inputs = Inputs::new(&canonical, &originals).expect("actual owners");
    let source_error = inputs
        .normalize_sources(Limits::new(1, 0, 0))
        .expect_err("count first");
    assert!(matches!(
        source_error.refusal(),
        Refusal::Sources {
            allowed: 1,
            required: 2
        }
    ));
    assert!(std::ptr::eq(source_error.inputs().sources(), &originals));
    assert_eq!(source_error.limits(), Limits::new(1, 0, 0));
    let node_error = inputs
        .normalize_sources(Limits::new(2, 6, 0))
        .expect_err("all seven nodes before allocation");
    assert!(matches!(
        node_error.refusal(),
        Refusal::Nodes {
            allowed: 6,
            required: 7
        }
    ));
    let retained_error = inputs
        .normalize_sources(Limits::new(2, 7, 15))
        .expect_err("separate retained allowance");
    assert!(matches!(
        retained_error.refusal(),
        Refusal::RetainedRecords {
            allowed: 15,
            required: 16
        }
    ));
    assert!(std::error::Error::source(&retained_error).is_none());
    assert!(inputs.normalize_sources(Limits::new(2, 7, 16)).is_ok());
}

#[test]
fn maximum_dynamic_call_arguments_and_empty_sources_obey_actual_complete_populations() {
    let text = format!("missing_call({})", vec!["1"; 255].join(","));
    let originals = sources(&[&text]);
    let canonical = CanonicalInputs::new(vec![], vec![]).expect("actual owner");
    let inputs = Inputs::new(&canonical, &originals).expect("complete join");
    let normalized = inputs
        .normalize_sources(Limits::new(1, 256, 513))
        .expect("language maximum");
    assert_eq!(normalized.node_count(), 256);
    assert_eq!(normalized.retained_records(), 513);
    assert_eq!(
        normalized
            .sources()
            .first()
            .expect("actual source")
            .expression()
            .node_count(),
        256
    );
    let empty = sources(&[]);
    let inputs = Inputs::new(&canonical, &empty).expect("empty actual join");
    let normalized = inputs
        .normalize_sources(Limits::new(0, 0, 0))
        .expect("zero empty allowances");
    assert!(normalized.sources().is_empty());
    assert_eq!(normalized.node_count(), 0);
    assert_eq!(normalized.retained_records(), 0);
}
