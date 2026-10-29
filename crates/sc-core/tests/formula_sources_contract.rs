//! Original entity/source ownership and complete admission; no graph or value certificate.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    recipe::{
        FormulaExpressionDocument as Document, FormulaSource as Source,
        FormulaSourceLimits as Limits, FormulaSourceResource as Resource,
        FormulaSources as Sources, FormulaSourcesError as Error,
    },
};

fn source(id: u128, text: &str) -> Source {
    Source::new(
        EntityId::from_bits(id),
        Document::new(text.into()).expect("syntax only"),
    )
}
fn limits(sources: &[Source]) -> Limits {
    Limits::new(
        sources.len(),
        sources.iter().map(|s| s.expression().source().len()).sum(),
        sources
            .iter()
            .map(|s| s.expression().canonical().as_str().len())
            .sum(),
    )
}

#[test]
fn exact_owned_buffers_authored_order_and_ids_survive_indexing_without_text_deduplication() {
    let original = vec![
        source(u128::MAX, " (2.5 cm + eps_num) "),
        source(0, "25 mm+eps_num"),
        source(9001, "26 mm+eps_num"),
    ];
    let addresses: Vec<_> = original
        .iter()
        .map(|s| s.expression().source().as_ptr())
        .collect();
    let expected = original.clone();
    let registry = Sources::new(original, limits(&expected)).expect("exact complete allowances");
    assert_eq!(registry.sources(), expected);
    for (position, actual) in registry.sources().iter().enumerate() {
        let found = registry.source(actual.id()).expect("exact id");
        assert!(std::ptr::eq(actual, found));
        assert_eq!(
            Some(&found.expression().source().as_ptr()),
            addresses.get(position)
        );
        let normalized = found
            .expression()
            .normalized()
            .expect("actual original source");
        assert_eq!(&normalized.canonical_form(), found.expression().canonical());
    }
    let mut actual = registry.sources().iter();
    let first = actual.next().expect("first");
    let alias = actual.next().expect("second");
    assert_eq!(
        first.expression().canonical(),
        alias.expression().canonical()
    );
    assert_ne!(first.id(), alias.id());
    assert_ne!(first.expression(), alias.expression());
    assert_eq!(registry.source(EntityId::from_bits(25)), None);
    let normalized = first.expression().normalized().expect("view");
    let span = normalized.root().span();
    assert_eq!(
        first.expression().source().get(span.start()..span.end()),
        Some("(2.5 cm + eps_num)")
    );
    assert!(!format!("{registry:?}").contains("eps_num"));
}

#[test]
fn duplicate_refusals_follow_original_order_even_when_sorted_identity_order_disagrees() {
    // An independent authored-prefix scan, including triples and competing duplicate identities.
    for encoded in 0u16..256 {
        let ids: Vec<_> = (0..4)
            .map(|shift| u128::from((encoded >> (shift * 2)) & 3))
            .collect();
        let expected = ids.iter().enumerate().find_map(|(duplicate, id)| {
            ids.get(..duplicate)
                .and_then(|prefix| prefix.iter().position(|earlier| earlier == id))
                .map(|first| (*id, first, duplicate))
        });
        let originals: Vec<_> = ids.iter().map(|id| source(*id, "1 mm")).collect();
        let allowance = limits(&originals);
        if let Some((id, expected_first, expected_duplicate)) = expected {
            let actual = Sources::new(originals, allowance).expect_err("actual duplicate");
            assert!(
                matches!(actual, Error::DuplicateIdentity { identity, first, duplicate }
                if identity.as_bits() == id && first == expected_first && duplicate == expected_duplicate)
            );
        } else {
            let registry = Sources::new(originals, allowance).expect("unique identities");
            assert_eq!(registry.sources().len(), 4);
        }
    }
    let same_id_different_body = vec![source(9, "1 mm"), source(9, "2 mm")];
    let allowance = limits(&same_id_different_body);
    assert!(matches!(
        Sources::new(same_id_different_body, allowance),
        Err(Error::DuplicateIdentity {
            first: 0,
            duplicate: 1,
            ..
        })
    ));
}

#[test]
fn complete_content_limits_are_exact_separate_and_precede_duplicate_admission() {
    let original = vec![source(7, " 1 mm "), source(8, "2 mm")];
    let exact = limits(&original);
    assert_eq!(exact.sources(), 2);
    assert_eq!(exact.original_bytes(), 10);
    assert!(exact.canonical_bytes() > 0);
    for (resource, allowance, allowed, required) in [
        (
            Resource::Sources,
            Limits::new(1, exact.original_bytes(), exact.canonical_bytes()),
            1,
            2,
        ),
        (
            Resource::OriginalBytes,
            Limits::new(2, 9, exact.canonical_bytes()),
            9,
            10,
        ),
        (
            Resource::CanonicalBytes,
            Limits::new(2, 10, exact.canonical_bytes() - 1),
            exact.canonical_bytes() - 1,
            exact.canonical_bytes(),
        ),
    ] {
        let refusal = Sources::new(original.clone(), allowance).expect_err("complete refusal");
        assert!(
            matches!(refusal, Error::Limit { resource: actual, allowed: a, required: r }
            if actual == resource && a == allowed && r == required)
        );
    }
    assert!(Sources::new(original, exact).is_ok());
    let duplicate = vec![source(7, "1 mm"), source(7, "1 mm")];
    assert!(matches!(
        Sources::new(duplicate, Limits::new(0, 0, 0)),
        Err(Error::Limit {
            resource: Resource::Sources,
            required: 2,
            ..
        })
    ));
    let empty = Sources::new(Vec::new(), Limits::new(0, 0, 0)).expect("explicit empty draft");
    assert!(empty.sources().is_empty());
    assert!(empty.source(EntityId::from_bits(0)).is_none());
}

#[test]
fn original_expression_ownership_does_not_supply_namespace_kind_graph_or_evaluated_state() {
    let original = vec![
        source(9001, "missing+1 mm/0"),
        source(9002, "1 mm+1 deg"),
        source(9003, "missing_call(1)"),
    ];
    let allowance = limits(&original);
    let registry = Sources::new(original, allowance).expect("source-only ownership");
    assert_eq!(registry.sources().len(), 3);
    let real = registry
        .source(EntityId::from_bits(9001))
        .expect("actual source");
    assert_eq!(real.expression().source(), "missing+1 mm/0");
    let normalized = real.expression().normalized().expect("syntax");
    let namespace = sc_core::recipe::FormulaNamespace::new([]).expect("empty namespace");
    assert_eq!(
        normalized
            .check_kinds(&namespace)
            .expect_err("actual missing name")
            .token(),
        "formula_unbound_name"
    );
}
