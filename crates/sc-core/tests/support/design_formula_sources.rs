//! Actual source ownership joins at the existing complete Design input fixture boundary.
use super::{
    id,
    input_composition::{definition, Markers},
};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{
        charge_design_input_resources, DesignHostEntity, DesignHostEntityKind, DesignInputs,
        DesignInputsError as Error, DesignInputsPosition as Position, DesignResourceDemand,
        DesignResourceLimitError, DesignResourceLimits, DesignResourceLocation,
        DesignResourceMeter, DesignResourceScope, DesignResourceSink, DesignResourceSource,
        DesignStructure, DesignStructurePosition,
    },
    recipe::{
        FormulaExpressionDocument, FormulaSource, FormulaSourceInputRefusal, FormulaSourceInputs,
        FormulaSourceLimits, FormulaSources,
    },
    value::{
        CanonicalInputPosition, CanonicalInputTarget, CanonicalInputs, LengthDeclaration,
        LengthState, LengthValueError, ScalarInputDeclaration, ScalarInputState,
    },
};
fn registry(rows: Vec<FormulaSource>) -> FormulaSources {
    let limits = FormulaSourceLimits::new(
        rows.len(),
        rows.iter().map(|s| s.expression().source().len()).sum(),
        rows.iter()
            .map(|s| s.expression().canonical().as_str().len())
            .sum(),
    );
    FormulaSources::new(rows, limits).expect("exact actual owned source data")
}
fn source(identity: u128, body: &str) -> FormulaSource {
    FormulaSource::new(
        id(identity),
        FormulaExpressionDocument::new(body.into()).expect("explicit actual source body"),
    )
}
#[test]
fn actual_owned_sources_and_unused_original_bodies_survive_root_snapshot_without_values() {
    let mut supplied = definition();
    let mut rows = supplied.formula_sources.sources().to_vec();
    rows.push(source(9004, " tan(90 deg) "));
    supplied.formula_sources = registry(rows);
    let before = supplied.clone();
    let state = DesignInputs::new(supplied).expect("actual complete owned source references");
    assert_eq!(state.definition(), &before);
    assert_eq!(state.clone(), state);
    assert!(std::ptr::eq(
        state.formula_sources(),
        &state.definition().formula_sources
    ));
    let owned: Vec<_> = state
        .identities()
        .into_iter()
        .filter(|(_, p)| matches!(p, Position::FormulaSource { .. }))
        .collect();
    assert_eq!(
        owned,
        vec![
            (id(9003), Position::FormulaSource { index: 0 }),
            (id(9004), Position::FormulaSource { index: 1 })
        ]
    );
    let joined = FormulaSourceInputs::new(state.inputs(), state.formula_sources())
        .expect("actual containing owners");
    assert!(std::ptr::eq(joined.inputs(), state.inputs()));
    assert!(std::ptr::eq(joined.sources(), state.formula_sources()));
    let derived = joined
        .derived_inputs()
        .next()
        .expect("real canonical Derived target");
    assert!(std::ptr::eq(
        derived.source(),
        state
            .formula_sources()
            .source(id(9003))
            .expect("real source")
    ));
    assert_eq!(
        state
            .inputs()
            .length(id(40))
            .expect("actual canonical")
            .authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(40),
            formula: id(9003)
        })
    );
    assert_eq!(
        state
            .formula_sources()
            .source(id(9004))
            .expect("unused source")
            .expression()
            .source(),
        " tan(90 deg) "
    );
}
#[test]
fn every_actual_global_role_refuses_an_unused_source_alias_before_current_source_joins() {
    for (identity, first, duplicate) in [
        (
            1,
            Position::Structure(DesignStructurePosition::Design),
            Position::FormulaSource { index: 1 },
        ),
        (
            40,
            Position::Structure(DesignStructurePosition::Length { index: 0 }),
            Position::FormulaSource { index: 1 },
        ),
        (
            50,
            Position::Structure(DesignStructurePosition::Scalar { index: 0 }),
            Position::FormulaSource { index: 1 },
        ),
        (
            10,
            Position::Structure(DesignStructurePosition::Piece { index: 1 }),
            Position::FormulaSource { index: 1 },
        ),
        (
            30,
            Position::Structure(DesignStructurePosition::CutCopy { index: 0 }),
            Position::FormulaSource { index: 1 },
        ),
        (
            1000,
            Position::Structure(DesignStructurePosition::Operation { index: 0 }),
            Position::FormulaSource { index: 1 },
        ),
        (
            8003,
            Position::Parameter { index: 0 },
            Position::FormulaSource { index: 1 },
        ),
        (
            8001,
            Position::FormulaSource { index: 1 },
            Position::Material { index: 0 },
        ),
        (
            8002,
            Position::FormulaSource { index: 1 },
            Position::Source(DesignHostEntity {
                id: id(8002),
                kind: DesignHostEntityKind::Landmark,
                index: 0,
            }),
        ),
    ] {
        let mut supplied = definition();
        let mut rows = supplied.formula_sources.sources().to_vec();
        rows.push(source(identity, "1 mm"));
        supplied.formula_sources = registry(rows);
        assert_eq!(
            DesignInputs::new(supplied).expect_err("actual whole-owned identity collision"),
            Error::DuplicateIdentity {
                identity: id(identity),
                first,
                duplicate
            }
        );
    }
}
#[test]
fn complete_required_derived_joins_keep_original_family_position_and_never_use_provenance_as_formula(
) {
    let mut supplied = definition();
    supplied.formula_sources = registry(vec![]);
    let expected = Error::FormulaSources {
        position: CanonicalInputPosition::Length { index: 0 },
        target: CanonicalInputTarget::Length(id(40)),
        refusal: FormulaSourceInputRefusal::MissingSource { formula: id(9003) },
    };
    assert_eq!(
        DesignInputs::new(supplied).expect_err("required actual dedicated source absent"),
        expected
    );
    let mut supplied = definition();
    let mut structure = supplied.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    let mut authored = scalars
        .first()
        .expect("actual general Length")
        .definition()
        .clone();
    authored.state = ScalarInputState::Derived { formula: id(9004) };
    *scalars.first_mut().expect("actual original scalar slot") =
        ScalarInputDeclaration::new(authored).expect("actual Derived declaration");
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("actual replacement inventory");
    supplied.structure = DesignStructure::new(structure).expect("actual unchanged structure");
    assert_eq!(
        DesignInputs::new(supplied).expect_err("unused general Derived still needs actual owner"),
        Error::FormulaSources {
            position: CanonicalInputPosition::Scalar { index: 0 },
            target: CanonicalInputTarget::Scalar {
                declaration: id(50),
                kind: sc_core::value::ScalarInputKind::Length
            },
            refusal: FormulaSourceInputRefusal::MissingSource { formula: id(9004) }
        }
    );
    let mut supplied = definition();
    let mut structure = supplied.structure.definition().clone();
    let mut length = structure
        .inputs
        .length(id(40))
        .expect("actual canonical target")
        .definition()
        .clone();
    assert!(matches!(length.state,LengthState::Derived {formula} if formula==id(9003)));
    length.source = id(9004);
    structure.inputs = CanonicalInputs::new(
        vec![LengthDeclaration::new(length).expect("actual provenance reference")],
        structure.inputs.scalars().to_vec(),
    )
    .expect("actual inventory");
    supplied.structure = DesignStructure::new(structure).expect("actual structure");
    supplied.formula_sources = registry(vec![source(9004, "1 mm")]);
    assert_eq!(
        DesignInputs::new(supplied).expect_err("provenance cannot substitute sole formula id"),
        expected
    );
}
impl DesignResourceSource for Markers {
    fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        sink.charge_entries(
            self.0.len(),
            DesignResourceLocation {
                scope: DesignResourceScope::SourceHost,
                owner: None,
                field: "fixture.markers",
                index: None,
            },
        )
    }
}
fn demand(inputs: &DesignInputs<Markers>) -> DesignResourceDemand {
    let limits = DesignResourceLimits::new(
        NonZeroUsize::new(10000).expect("entry policy"),
        NonZeroUsize::new(10000).expect("text policy"),
        NonZeroU64::new(1000).expect("counter policy"),
    );
    let mut meter = DesignResourceMeter::new(limits);
    charge_design_input_resources(inputs, &mut meter).expect("complete actual held root");
    meter.finish().expect("complete cumulative demand")
}
#[test]
fn complete_root_resources_charge_unused_sources_and_both_actual_buffers_in_the_snapshot() {
    let original = DesignInputs::new(definition()).expect("actual original root");
    let mut supplied = original.definition().clone();
    let unused = source(9004, " tan(90 deg) ");
    let extra = unused.expression().source().len() + unused.expression().canonical().as_str().len();
    let mut rows = supplied.formula_sources.sources().to_vec();
    rows.push(unused);
    supplied.formula_sources = registry(rows);
    let extended = DesignInputs::new(supplied).expect("actual unexecuted owned source");
    let before = demand(&original);
    let after = demand(&extended);
    assert_eq!(after.collection_entries(), before.collection_entries() + 2);
    assert_eq!(after.text_bytes(), before.text_bytes() + extra);
    assert_eq!(after.counter_bits(), before.counter_bits());
    assert_eq!(
        demand(&extended.clone()),
        after,
        "clone retains complete honest source costs"
    );
    assert_eq!(original.formula_sources().sources().len(), 1);
}
