//! Actual authored size membership supplies values and pinned sources to the core scalar worker.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    recipe::{
        FormulaCheckedNodeForm as Form, FormulaExecutionLimits, FormulaExpression,
        FormulaNamespace, FormulaRational, FormulaScalarExecutionContext,
        FormulaScalarExecutionReadKind as Read, FormulaToleranceContext,
    },
    value::CanonicalInputs,
};
use sc_measure::{
    SizeLabel, SizeMember, SizeMembership, SizeMembershipDefinition, SizeSetReference, SizeSystem,
};
use sc_units::Count;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}

#[test]
fn real_pinned_authored_membership_drives_only_selected_size_values_and_preserves_sources() {
    let reference = SizeSetReference {
        id: id(1),
        revision: Count::new(7),
    };
    let membership = SizeMembership::new(SizeMembershipDefinition {
        reference,
        system: SizeSystem::Custom,
        members: [(11, "42"), (12, "0012"), (13, "2XL")]
            .map(|(identity, label)| SizeMember {
                id: id(identity),
                label: SizeLabel::new(label.to_owned()).expect("actual authored label"),
            })
            .into(),
        base: id(12),
    })
    .expect("actual canonical authored order/base");
    let before = membership.clone();
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("actual empty scalar inventory");
    let expression = FormulaExpression::parse("if(is_base_size,size_index*1 mm,size_count*1 mm)")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = expression
        .check_kinds(&FormulaNamespace::new([]).expect("namespace"))
        .expect("complete static expression");
    for selected in [11, 12, 13] {
        let actual = membership
            .formula_context(reference, id(selected))
            .expect("actual pin/member");
        assert!(std::ptr::eq(actual.membership(), &membership));
        assert!(std::ptr::eq(
            actual.member(),
            membership.member(id(selected)).expect("actual member")
        ));
        let result = proof
            .admit_execution(FormulaExecutionLimits::new(256, 768))
            .expect("explicit bounds")
            .prepare_scalar(&inputs)
            .expect("all reserved source roles")
            .evaluate(FormulaScalarExecutionContext::new(
                FormulaToleranceContext::empty(),
                actual.context(),
            ))
            .expect("real membership-derived execution");
        assert_eq!(
            result.value().numeric().expect("Length").rational(),
            FormulaRational::from_signed_integer(if selected == 12 { 2000 } else { 3000 })
        );
        assert_eq!(result.plan().sources().len(), 3);
        let mut names = Vec::new();
        for read in result.reads() {
            let Form::Name { name, .. } = read.node().form() else {
                unreachable!("actual reserved name");
            };
            names.push(name);
            let Read::Size(value) = read.source() else {
                unreachable!("actual membership size read");
            };
            assert_eq!(value.source().set, reference.id);
            assert_eq!(value.source().revision, reference.revision);
            assert_eq!(value.source().member, id(selected));
            assert_eq!(value.source().base, id(12));
        }
        assert_eq!(
            names,
            [
                "is_base_size",
                if selected == 12 {
                    "size_index"
                } else {
                    "size_count"
                }
            ]
        );
    }
    assert_eq!(membership, before);
}
