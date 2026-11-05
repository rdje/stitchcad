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
