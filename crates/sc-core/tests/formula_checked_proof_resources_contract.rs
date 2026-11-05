//! Independent exact proof populations and cumulative boundary/refusal checks through public APIs.
#![allow(clippy::expect_used)]
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{
        DesignResourceCount, DesignResourceLimitError, DesignResourceLimits,
        DesignResourceLocation, DesignResourceMeter, DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
    recipe::{FormulaExpression, FormulaNamespace},
};
fn limits(entries: usize) -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::new(entries).expect("positive entries"),
        NonZeroUsize::new(1).expect("positive text"),
        NonZeroU64::new(1).expect("positive bits"),
    )
}
fn at(field: &'static str) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(EntityId::from_bits(801)),
        field,
        index: Some(3),
    }
}
#[test]
fn complete_checked_proof_buffers_charge_actual_counts_and_preserve_first_refusals() {
    let namespace = FormulaNamespace::new([]).expect("original empty explicit namespace");
    // Counts are authored from the grammar trees, including both conditional branches. Static
    // proof/accounting must not evaluate the zero divisor or discard the untaken branch.
    for (source, nodes, dependencies) in [
        ("1 mm", 1, 0),
        ("-(2 mm)", 2, 0),
        ("if(is_base_size,1 mm/0,max(2 mm,3 mm))", 8, 1),
        ("if(is_base_size,if(is_base_size,1 mm,2 mm),3 mm)", 7, 2),
        ("within(1 mm,2 mm,eps_num)", 4, 1),
    ] {
        let expression = FormulaExpression::parse(source)
            .expect("original syntax")
            .normalize_literals()
            .expect("bounded exact literals");
        let proof = expression
            .check_kinds(&namespace)
            .expect("actual original proof");
        assert_eq!(proof.node_count(), nodes);
        assert_eq!(proof.dependencies().len(), dependencies);
        let certificates_at = at("proof.certificates");
        let dependencies_at = at("proof.dependencies");
        let mut exact = DesignResourceMeter::new(limits(nodes + dependencies));
        proof
            .charge_retained_resources(&mut exact, certificates_at, dependencies_at)
            .expect("exact allowance");
        let demand = exact.finish().expect("complete actual proof content");
        assert_eq!(
            (
                demand.collection_entries(),
                demand.text_bytes(),
                demand.counter_bits()
            ),
            (nodes + dependencies, 0, 0)
        );
        // Repeated composition charges both original logical occurrences, without cached credit.
        let mut repeated = DesignResourceMeter::new(limits(2 * (nodes + dependencies)));
        for _ in 0..2 {
            proof
                .charge_retained_resources(&mut repeated, certificates_at, dependencies_at)
                .expect("fresh repeated walk");
        }
        assert_eq!(
            repeated.finish().expect("twice").collection_entries(),
            2 * (nodes + dependencies)
        );
        let (maximum, consumed) = if nodes == 1 { (1, 1) } else { (nodes - 1, 0) };
        let mut short = DesignResourceMeter::new(limits(maximum));
        short
            .charge_entries(consumed, at("prefix"))
            .expect("owned prefix");
        let error = proof
            .charge_retained_resources(&mut short, certificates_at, dependencies_at)
            .expect_err("precharge complete certificates");
        assert_eq!(
            error,
            DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum,
                consumed,
                requested: nodes,
                location: certificates_at,
            }
        );
        assert_eq!(short.finish(), Err(error));
        if dependencies != 0 {
            let mut short = DesignResourceMeter::new(limits(nodes + dependencies - 1));
            let error = proof
                .charge_retained_resources(&mut short, certificates_at, dependencies_at)
                .expect_err("complete ordered dependencies");
            assert_eq!(
                error,
                DesignResourceLimitError::Count {
                    unit: DesignResourceCount::CollectionEntries,
                    maximum: nodes + dependencies - 1,
                    consumed: nodes,
                    requested: dependencies,
                    location: dependencies_at,
                }
            );
            assert_eq!(short.finish(), Err(error));
        }
        let mut latched = DesignResourceMeter::new(limits(nodes + dependencies));
        let original = latched
            .charge_text("ab", at("earlier.text"))
            .expect_err("original earlier text refusal");
        assert_eq!(
            proof.charge_retained_resources(&mut latched, certificates_at, dependencies_at),
            Err(original)
        );
        assert_eq!(latched.finish(), Err(original));
        assert!(core::ptr::eq(proof.expression(), &expression));
        assert_eq!(proof.node_count(), nodes);
        assert_eq!(proof.dependencies().len(), dependencies);
    }
}

fn component(
    records: usize,
    failures: &[(usize, usize, usize, DesignResourceLocation)],
    charge: impl Fn(&mut dyn DesignResourceSink) -> Result<(), DesignResourceLimitError>,
) {
    let mut exact = DesignResourceMeter::new(limits(records.max(1)));
    charge(&mut exact).expect("complete component");
    let demand = exact.finish().expect("whole component demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (records, 0, 0)
    );
    let mut repeated = DesignResourceMeter::new(limits((2 * records).max(1)));
    charge(&mut repeated).expect("first logical occurrence");
    charge(&mut repeated).expect("second logical occurrence");
    assert_eq!(
        repeated
            .finish()
            .expect("fresh repeated component")
            .collection_entries(),
        2 * records
    );
    for &(maximum, prefix, requested, location) in failures {
        let mut short = DesignResourceMeter::new(limits(maximum));
        short
            .charge_entries(prefix, at("prefix"))
            .expect("permitted prefix");
        let error = charge(&mut short).expect_err("complete actual field shortage");
        assert_eq!(
            error,
            DesignResourceLimitError::Count {
                unit: DesignResourceCount::CollectionEntries,
                maximum,
                consumed: if prefix == 0 {
                    maximum + 1 - requested
                } else {
                    prefix
                },
                requested,
                location,
            }
        );
        assert_eq!(short.finish(), Err(error));
    }
    let mut latched = DesignResourceMeter::new(limits(records.max(1)));
    let original = latched
        .charge_text("ab", at("earlier"))
        .expect_err("original text latch");
    assert_eq!(charge(&mut latched), Err(original));
    assert_eq!(latched.finish(), Err(original));
}
fn field(field: &'static str, owner: Option<u128>, index: Option<usize>) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: owner.map(EntityId::from_bits),
        field,
        index,
    }
}
#[test]
fn checked_statement_components_preserve_real_ordinals_and_left_right_field_order() {
    use sc_core::recipe::{FormulaCheckedStatementKind, FormulaRecipe};
    let recipe = FormulaRecipe::parse("let n:count=1\nassert equal:eps_num=n==n")
        .expect("original statement syntax")
        .normalize_literals()
        .expect("original normalized statements");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("initial namespace"))
        .expect("actual ordered statement proofs");
    assert_eq!(proof.statements().len(), 2);
    let binding = proof
        .statements()
        .first()
        .expect("first original statement");
    let assertion = proof
        .statements()
        .get(1)
        .expect("second original statement");
    component(
        1,
        &[(
            1,
            1,
            1,
            field("checked.statement.let.certificates", Some(801), Some(1)),
        )],
        |sink| {
            binding.charge_retained_resources(
                sink,
                DesignResourceScope::CommandState,
                Some(EntityId::from_bits(801)),
            )
        },
    );
    component(
        4,
        &[
            (
                1,
                1,
                1,
                field(
                    "checked.statement.assert.left.certificates",
                    Some(801),
                    Some(2),
                ),
            ),
            (
                1,
                0,
                1,
                field(
                    "checked.statement.assert.left.dependencies",
                    Some(801),
                    Some(2),
                ),
            ),
            (
                2,
                0,
                1,
                field(
                    "checked.statement.assert.right.certificates",
                    Some(801),
                    Some(2),
                ),
            ),
            (
                3,
                0,
                1,
                field(
                    "checked.statement.assert.right.dependencies",
                    Some(801),
                    Some(2),
                ),
            ),
        ],
        |sink| {
            assertion.charge_retained_resources(
                sink,
                DesignResourceScope::CommandState,
                Some(EntityId::from_bits(801)),
            )
        },
    );
    assert_eq!(binding.statement_index(), 1);
    assert_eq!(assertion.statement_index(), 2);
    assert!(matches!(
        binding.kind(),
        FormulaCheckedStatementKind::Let { .. }
    ));
    assert!(matches!(
        assertion.kind(),
        FormulaCheckedStatementKind::Assert { .. }
    ));
}
#[test]
fn checked_source_and_graph_components_count_unused_proofs_edges_and_terminal_offsets() {
    use sc_core::{
        name::MachineToken,
        recipe::{
            FormulaDeclaration, FormulaExpressionDocument, FormulaInitialDeclaration,
            FormulaInputOrigin, FormulaSource, FormulaSourceCheckLimits, FormulaSourceCycleLimits,
            FormulaSourceDependencyLimits, FormulaSourceInputs, FormulaSourceLimits,
            FormulaSourceNormalizationLimits, FormulaSources,
        },
        value::{CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState},
    };
    let id = EntityId::from_bits;
    let canonical = CanonicalInputs::new(
        vec![
            LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(1),
                source: id(51),
                state: LengthState::Derived { formula: id(100) },
            })
            .expect("original Derived length"),
            LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(2),
                source: id(52),
                state: LengthState::Unknown {
                    observation: id(700),
                },
            })
            .expect("original Unknown length"),
        ],
        vec![],
    )
    .expect("original canonical inventory");
    let raw = FormulaSources::new(
        [(100, "base_length+base_length"), (101, "derived_length")]
            .into_iter()
            .map(|(identity, text)| {
                FormulaSource::new(
                    id(identity),
                    FormulaExpressionDocument::new(text.into()).expect("original source"),
                )
            })
            .collect(),
        FormulaSourceLimits::new(2, 100, 100),
    )
    .expect("whole original source set");
    let names = [
        MachineToken::new("derived_length").expect("Derived name"),
        MachineToken::new("base_length").expect("Unknown name"),
    ];
    let namespace = FormulaNamespace::new(names.iter().zip([1, 2]).map(|(name, identity)| {
        FormulaInitialDeclaration::try_from(FormulaDeclaration::length_input(
            name,
            FormulaInputOrigin::Parameter,
            id(800 + identity),
            canonical
                .length(id(identity))
                .expect("actual original length"),
        ))
        .expect("original initial metadata")
    }))
    .expect("actual initial namespace");
    let normalized = FormulaSourceInputs::new(&canonical, &raw)
        .expect("original actual source inputs")
        .normalize_sources(FormulaSourceNormalizationLimits::new(2, 4, 10))
        .expect("actual normalized source owners");
    let checked = normalized
        .check_sources(&namespace, FormulaSourceCheckLimits::new(2, 4, 1, 10))
        .expect("all actual source proofs");
    let dependencies = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(2, 1, 4))
        .expect("actual complete relation");
    let acyclic = dependencies
        .admit_acyclic(FormulaSourceCycleLimits::new(2, 4))
        .expect("actual complete acyclic proof");
    assert_eq!(checked.sources().len(), 2);
    assert_eq!(dependencies.edges().len(), 1);
    component(
        9,
        &[
            (1, 0, 2, field("checked.sources", Some(801), None)),
            (
                4,
                0,
                3,
                field("checked.sources.certificates", Some(100), Some(0)),
            ),
            (
                6,
                0,
                2,
                field("checked.sources.dependencies", Some(100), Some(0)),
            ),
            (
                7,
                0,
                1,
                field("checked.sources.certificates", Some(101), Some(1)),
            ),
            (
                8,
                0,
                1,
                field("checked.sources.dependencies", Some(101), Some(1)),
            ),
        ],
        |sink| {
            checked.charge_retained_resources(
                sink,
                DesignResourceScope::CommandState,
                Some(id(801)),
            )
        },
    );
    let failures = [
        (
            1,
            1,
            1,
            field("checked.source_dependencies.edges", Some(801), None),
        ),
        (
            3,
            0,
            3,
            field("checked.source_dependencies.offsets", Some(801), None),
        ),
    ];
    component(4, &failures, |sink| {
        dependencies.charge_retained_resources(
            sink,
            DesignResourceScope::CommandState,
            Some(id(801)),
        )
    });
    component(4, &failures, |sink| {
        acyclic.charge_retained_resources(sink, DesignResourceScope::CommandState, Some(id(801)))
    });
    let mut whole = DesignResourceMeter::new(limits(13));
    checked
        .charge_retained_resources(&mut whole, DesignResourceScope::CommandState, Some(id(801)))
        .expect("source proof arrays");
    acyclic
        .charge_retained_resources(&mut whole, DesignResourceScope::CommandState, Some(id(801)))
        .expect("relation arrays once, without old scratch");
    assert_eq!(
        whole
            .finish()
            .expect("complete two components")
            .collection_entries(),
        13
    );
    assert!(matches!(
        canonical
            .length(id(2))
            .expect("unchanged canonical loan")
            .state(),
        LengthState::Unknown { .. }
    ));
    assert!(core::ptr::eq(acyclic.dependencies(), &dependencies));
    let empty_canonical = CanonicalInputs::new(vec![], vec![]).expect("empty canonical owner");
    let empty_raw = FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))
        .expect("empty raw source owner");
    let empty_namespace = FormulaNamespace::new([]).expect("empty namespace");
    let empty_normalized = FormulaSourceInputs::new(&empty_canonical, &empty_raw)
        .expect("empty source association")
        .normalize_sources(FormulaSourceNormalizationLimits::new(0, 0, 0))
        .expect("empty normalized owner");
    let empty_checked = empty_normalized
        .check_sources(&empty_namespace, FormulaSourceCheckLimits::new(0, 0, 0, 0))
        .expect("empty checked sources");
    let empty_dependencies = empty_checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(0, 0, 1))
        .expect("terminal offset");
    let empty_acyclic = empty_dependencies
        .admit_acyclic(FormulaSourceCycleLimits::new(0, 0))
        .expect("zero scratch");
    component(0, &[], |sink| {
        empty_checked.charge_retained_resources(sink, DesignResourceScope::CommandState, None)
    });
    component(1, &[], |sink| {
        empty_acyclic.charge_retained_resources(sink, DesignResourceScope::CommandState, None)
    });
}
