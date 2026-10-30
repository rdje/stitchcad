//! Shared actual input/source/namespace/dependency/cycle factories;no computed value seeds.
use sc_core::{name::MachineToken, ontology::EntityId, recipe::*, value::*};
fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
pub(super) struct Fixture {
    pub(super) inputs: CanonicalInputs,
    sources: FormulaSources,
    names: Vec<(MachineToken, EntityId)>,
}
impl Fixture {
    pub(super) fn new(rows: Vec<(u128, ScalarInputKind, String, Option<&str>)>) -> Self {
        let mut scalars = Vec::new();
        let mut sources = Vec::new();
        let mut names = Vec::new();
        for (index, (formula, kind, body, name)) in rows.into_iter().enumerate() {
            let target = id(1000 + index as u128);
            scalars.push(
                ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                    id: target,
                    source: id(4001),
                    kind,
                    state: ScalarInputState::Derived {
                        formula: id(formula),
                    },
                })
                .expect("actual Derived scalar"),
            );
            sources.push(FormulaSource::new(
                id(formula),
                FormulaExpressionDocument::new(body).expect("actual source"),
            ));
            if let Some(name) = name {
                names.push((MachineToken::new(name).expect("actual name"), target));
            }
        }
        let limits = FormulaSourceLimits::new(
            sources.len(),
            sources.iter().map(|s| s.expression().source().len()).sum(),
            sources
                .iter()
                .map(|s| s.expression().canonical().as_str().len())
                .sum(),
        );
        Self {
            inputs: CanonicalInputs::new(vec![], scalars).expect("actual inventory"),
            sources: FormulaSources::new(sources, limits).expect("actual registry"),
            names,
        }
    }
    pub(super) fn namespace(&self) -> FormulaNamespace<'_> {
        FormulaNamespace::new(
            self.names
                .iter()
                .enumerate()
                .map(|(index, (name, target))| {
                    let record = self
                        .inputs
                        .scalar(*target)
                        .expect("actual canonical target");
                    FormulaInitialDeclaration::try_from(FormulaDeclaration::input(
                        name,
                        FormulaScalarInputOrigin::Parameter,
                        id(5000 + index as u128),
                        *target,
                        record.kind().into(),
                    ))
                    .expect("actual name metadata")
                }),
        )
        .expect("actual namespace")
    }
    pub(super) fn with_graph(
        &self,
        inspect: impl FnOnce(&FormulaAcyclicSourceDependencies<'_>, FormulaSourceGraphExecutionLimits),
    ) {
        let s = self.sources.sources().len();
        let n = self
            .sources
            .sources()
            .iter()
            .map(|s| s.expression().node_count())
            .sum();
        let normalized = FormulaSourceInputs::new(&self.inputs, &self.sources)
            .expect("actual source joins")
            .normalize_sources(FormulaSourceNormalizationLimits::new(s, n, s + 2 * n))
            .expect("all actual arenas");
        let namespace = self.namespace();
        let checked = normalized
            .check_sources(
                &namespace,
                FormulaSourceCheckLimits::new(s, n, s, s + 2 * n),
            )
            .expect("actual complete source kinds");
        let d = checked
            .sources()
            .iter()
            .map(|s| s.proof().dependencies().len())
            .sum();
        let dependencies = checked
            .prepare_dependencies(FormulaSourceDependencyLimits::new(s, d, s + d + 1))
            .expect("actual canonical edges");
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(s, 2 * s))
            .expect("whole acyclic relation");
        inspect(
            &graph,
            FormulaSourceGraphExecutionLimits::new(s, n, d, 2 * s + 2 * n + d, s),
        );
    }
}
