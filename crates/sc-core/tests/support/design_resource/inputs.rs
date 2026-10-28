//! Actual held parameter/recipe/Material data, without value/source/binding fabrication.
use super::{limits, Error, Location, Meter, Scope, Unit};
use sc_core::{
    design::{
        charge_material_inventory_resources, charge_parameter_resources,
        charge_recipe_document_resources, DesignParameterDefinition, DesignParameters,
        DesignResourceSink,
    },
    name::MachineToken,
    ontology::{
        EntityId, Material, MaterialDefinition, MaterialInventory, MaterialShrinkage,
        MaterialShrinkageStage,
    },
    recipe::FormulaRecipeDocument,
    value::{
        CanonicalInputTarget as Target, CanonicalInputs, LengthDeclaration,
        LengthDeclarationDefinition, LengthState, ScalarInputDeclaration,
        ScalarInputDeclarationDefinition, ScalarInputKind as Kind, ScalarInputState as State,
    },
};
const SOURCE: &str = " \nlet x: length = 1 mm \t";
const CANONICAL: &str = "(recipe (bind x length length:1000))";
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn scalar(identity: u128, kind: Kind) -> ScalarInputDeclaration {
    let state = if identity == 6 {
        State::Derived { formula: id(950) }
    } else {
        State::Unknown {
            observation: id(951),
        }
    };
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(identity),
        source: id(900),
        kind,
        state,
    })
    .expect("actual explicit unresolved/derived property state")
}
fn fixture() -> (
    CanonicalInputs,
    DesignParameters,
    FormulaRecipeDocument,
    MaterialInventory,
) {
    let canonical = CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(1),
            source: id(900),
            state: LengthState::Unknown {
                observation: id(952),
            },
        })
        .expect("actual unknown width")],
        (2..=7)
            .map(|identity| {
                scalar(
                    identity,
                    if identity >= 6 {
                        Kind::Ratio
                    } else {
                        Kind::Boolean
                    },
                )
            })
            .collect(),
    )
    .expect("actual canonical targets");
    let parameters = DesignParameters::new(
        vec![
            DesignParameterDefinition {
                id: id(200),
                token: MachineToken::new("a").expect("actual token"),
                target: Target::Length(id(1)),
            },
            DesignParameterDefinition {
                id: id(201),
                token: MachineToken::new("bb").expect("actual second token"),
                target: Target::Length(id(1)),
            },
        ],
        &canonical,
    )
    .expect("actual bound immutable metadata");
    let recipe =
        FormulaRecipeDocument::new(SOURCE.into()).expect("actual retained source/canonical syntax");
    assert_eq!(recipe.canonical().as_str(), CANONICAL);
    let target = |identity, kind| Target::Scalar {
        declaration: id(identity),
        kind,
    };
    let first = MaterialDefinition {
        id: id(300),
        source: id(900),
        name: "猫".into(),
        composition: Some(" é ".into()),
        width: Target::Length(id(1)),
        nap: target(2, Kind::Boolean),
        directional_print: target(3, Kind::Boolean),
        striped: target(4, Kind::Boolean),
        checked: target(5, Kind::Boolean),
        shrinkage: Some(MaterialShrinkage {
            x: target(6, Kind::Ratio),
            y: target(7, Kind::Ratio),
            treatment: id(953),
            stage: MaterialShrinkageStage::PostGradePreExport,
        }),
    };
    let mut second = first.clone();
    second.id = id(301);
    second.name = "silk".into();
    second.composition = None;
    second.shrinkage = None;
    let materials = MaterialInventory::new(
        vec![
            Material::new(first, &canonical).expect("actual draft Material"),
            Material::new(second, &canonical).expect("actual unprovided composition/shrinkage"),
        ],
        &canonical,
    )
    .expect("actual current Material inventory");
    (canonical, parameters, recipe, materials)
}
fn location(field: &'static str, owner: Option<u128>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::Inputs,
        owner: owner.map(id),
        field,
        index,
    }
}
#[test]
fn actual_input_metadata_counts_both_recipe_strings_tokens_and_optional_material_text_without_changes(
) {
    let (canonical, parameters, recipe, materials) = fixture();
    assert_eq!((SOURCE.len(), CANONICAL.len()), (24, 36));
    let saved = (
        canonical.clone(),
        parameters.clone(),
        recipe.clone(),
        materials.clone(),
    );
    let mut meter = Meter::new(limits(4, 74, 1));
    charge_parameter_resources(&parameters, &mut meter)
        .expect("two actual parameter tokens:3 bytes");
    charge_recipe_document_resources(&recipe, &mut meter)
        .expect("actual original24/canonical36 bytes");
    charge_material_inventory_resources(&materials, &mut meter)
        .expect("two actual Materials:11 text bytes");
    let demand = meter
        .finish()
        .expect("exact cumulative metadata component demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (4, 74, 0)
    );
    assert_eq!((canonical, parameters, recipe, materials), saved);
}
#[test]
fn actual_parameter_precharge_and_token_refusal_keep_exact_original_owner_and_index() {
    let (_, parameters, _, _) = fixture();
    let mut entries = Meter::new(limits(1, 74, 1));
    let original = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 1,
        consumed: 0,
        requested: 2,
        location: location("parameters", None, None),
    };
    assert_eq!(
        charge_parameter_resources(&parameters, &mut entries),
        Err(original)
    );
    assert_eq!(entries.finish(), Err(original));
    let mut text = Meter::new(limits(4, 2, 1));
    let original = Error::Count {
        unit: Unit::TextBytes,
        maximum: 2,
        consumed: 1,
        requested: 2,
        location: location("parameters.token", Some(201), Some(1)),
    };
    assert_eq!(
        charge_parameter_resources(&parameters, &mut text),
        Err(original)
    );
    assert_eq!(text.finish(), Err(original));
}
#[test]
fn recipe_source_and_canonical_charges_are_separate_original_fields_and_preserve_prior_token_bytes()
{
    let (_, parameters, recipe, _) = fixture();
    let mut source = Meter::new(limits(4, 23, 1));
    let original = Error::Count {
        unit: Unit::TextBytes,
        maximum: 23,
        consumed: 0,
        requested: 24,
        location: location("recipe.source", None, None),
    };
    assert_eq!(
        charge_recipe_document_resources(&recipe, &mut source),
        Err(original)
    );
    assert_eq!(source.finish(), Err(original));
    let mut canonical = Meter::new(limits(4, 62, 1));
    charge_parameter_resources(&parameters, &mut canonical).expect("preceding actual token3");
    let original = Error::Count {
        unit: Unit::TextBytes,
        maximum: 62,
        consumed: 27,
        requested: 36,
        location: location("recipe.canonical", None, None),
    };
    assert_eq!(
        charge_recipe_document_resources(&recipe, &mut canonical),
        Err(original)
    );
    assert_eq!(canonical.finish(), Err(original));
}
#[test]
fn material_composition_name_and_collection_limits_keep_actual_owners_without_property_value_lookup(
) {
    let (_, parameters, recipe, materials) = fixture();
    let mut composition = Meter::new(limits(4, 6, 1));
    let original = Error::Count {
        unit: Unit::TextBytes,
        maximum: 6,
        consumed: 3,
        requested: 4,
        location: location("materials.composition", Some(300), Some(0)),
    };
    assert_eq!(
        charge_material_inventory_resources(&materials, &mut composition),
        Err(original)
    );
    assert_eq!(composition.finish(), Err(original));
    let mut text = Meter::new(limits(4, 73, 1));
    charge_parameter_resources(&parameters, &mut text).expect("preceding parameter text");
    charge_recipe_document_resources(&recipe, &mut text).expect("preceding source/canonical text");
    let original = Error::Count {
        unit: Unit::TextBytes,
        maximum: 73,
        consumed: 70,
        requested: 4,
        location: location("materials.name", Some(301), Some(1)),
    };
    assert_eq!(
        charge_material_inventory_resources(&materials, &mut text),
        Err(original)
    );
    assert_eq!(text.finish(), Err(original));
    let mut entries = Meter::new(limits(3, 74, 1));
    charge_parameter_resources(&parameters, &mut entries).expect("preceding2 entries");
    let original = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 3,
        consumed: 2,
        requested: 2,
        location: location("materials", None, None),
    };
    assert_eq!(
        charge_material_inventory_resources(&materials, &mut entries),
        Err(original)
    );
    assert_eq!(entries.finish(), Err(original));
}
#[test]
fn actual_empty_metadata_recipe_keeps_its_canonical_identity_bytes_and_prior_failure() {
    let canonical = CanonicalInputs::new(vec![], vec![]).expect("actual empty canonical draft");
    let parameters =
        DesignParameters::new(vec![], &canonical).expect("actual empty parameter draft");
    let materials =
        MaterialInventory::new(vec![], &canonical).expect("actual empty Material draft");
    let recipe = FormulaRecipeDocument::new(String::new()).expect("actual empty recipe source");
    assert_eq!(recipe.canonical().as_str(), "(recipe)");
    let mut meter = Meter::new(limits(1, 8, 1));
    charge_parameter_resources(&parameters, &mut meter).expect("empty parameters");
    charge_recipe_document_resources(&recipe, &mut meter).expect("zero source plus canonical8");
    charge_material_inventory_resources(&materials, &mut meter).expect("empty Materials");
    let demand = meter.finish().expect("exact actual empty-draft metadata");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 8, 0)
    );
    let mut refused = Meter::new(limits(1, 1, 1));
    let original = refused
        .charge_text("é", location("preceding", None, None))
        .expect_err("first prior refusal");
    assert_eq!(
        charge_parameter_resources(&parameters, &mut refused),
        Err(original)
    );
    assert_eq!(
        charge_recipe_document_resources(&recipe, &mut refused),
        Err(original)
    );
    assert_eq!(
        charge_material_inventory_resources(&materials, &mut refused),
        Err(original)
    );
    assert_eq!(refused.finish(), Err(original));
}
