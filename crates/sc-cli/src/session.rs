//! Replays a session: every change goes through the command bus, with revision checks and undo.
use crate::design::{build, declaration, Identities, Input};
use crate::outline::svg;
use core::fmt;
use sc_constraints::{
    Binding, Catalog, Domain, Expr, Limits, Outcome, Problem, Rule, TemplatePart, Value as Fact,
    Variable, AST_VERSION,
};
use sc_core::{
    command::{
        ChangeMeasurement, CommandActorClaim, CommandAuditSequence, CommandAuthority,
        CommandAuthoritySnapshot, CommandAuthoritySource, CommandBus, CommandGrant,
        CommandGrantStatus, CommandGroupId, CommandHistoryDirection, CommandHistoryRequest,
        CommandId, CommandIdempotencyKey, CommandMutation, CommandMutationContext,
        CommandMutationGroup, CommandOperationLimits, CommandScope, DesignRevision,
        MutationPayload,
    },
    design::DesignResourceLimits,
    ontology::EntityId,
    recipe::{
        FormulaCheckedStatementKind, FormulaRecipeExecutionLimits, FormulaScalarExecutionContext,
        FormulaSizeContext, FormulaToleranceContext,
    },
};
use sc_measure::DesignSources;
use sc_store::{Project, ProjectError, ProjectStore, Repository, Value};
use sc_units::Length;
use std::collections::BTreeMap;
use std::num::{NonZeroU64, NonZeroUsize};
use std::time::{Duration, SystemTime};

const FORMAT: i64 = 1;
const UNDO_DEPTH: usize = 256;
const MAX_ACTIONS: usize = 4096;

/// Why a session stopped. It prints as canonical text, and the program exits nonzero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    step: Option<usize>,
    code: &'static str,
    message: String,
}

impl Failure {
    /// The command line itself was wrong.
    #[must_use]
    pub fn usage(message: &str) -> Self {
        Self {
            step: None,
            code: "usage",
            message: message.to_owned(),
        }
    }

    fn session(message: impl Into<String>) -> Self {
        Self {
            step: None,
            code: "session",
            message: message.into(),
        }
    }

    fn at(step: usize, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            step: Some(step),
            code,
            message: message.into(),
        }
    }

    /// The structured diagnostic.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = BTreeMap::new();
        fields.insert("failed".to_owned(), Value::Text(self.code.to_owned()));
        fields.insert("message".to_owned(), Value::Text(self.message.clone()));
        if let Some(step) = self.step {
            fields.insert(
                "step".to_owned(),
                Value::Int(i64::try_from(step).unwrap_or(i64::MAX)),
            );
        }
        Value::Object(fields)
    }
}

// The session's single local operator, holding inspect and commit over this design. The time is fixed: nothing
// about a replay depends on when it runs.
struct LocalOperator {
    actor: CommandActorClaim,
    grants: Vec<CommandGrant>,
}

impl CommandAuthoritySource for LocalOperator {
    fn snapshot(&self) -> CommandAuthoritySnapshot<'_> {
        CommandAuthoritySnapshot {
            authenticated_actor: Some(self.actor),
            observed_at: SystemTime::UNIX_EPOCH + Duration::from_secs(1),
            grants: &self.grants,
        }
    }
}

fn field<'a>(fields: &'a BTreeMap<String, Value>, name: &str) -> Result<&'a Value, Failure> {
    fields
        .get(name)
        .ok_or_else(|| Failure::session(format!("the session has no {name} field")))
}

fn read_inputs(fields: &BTreeMap<String, Value>) -> Result<Vec<Input>, Failure> {
    let list = field(fields, "inputs")?
        .as_list()
        .ok_or_else(|| Failure::session("inputs must be a list"))?;
    let mut inputs = Vec::new();
    for item in list {
        let item = item
            .as_object()
            .ok_or_else(|| Failure::session("an input must be an object"))?;
        let token = field(item, "token")?
            .as_text()
            .ok_or_else(|| Failure::session("an input's token must be text"))?;
        let value = field(item, "um")?.as_int().ok_or_else(|| {
            Failure::session(format!(
                "{token}'s um must be a whole number of micrometres"
            ))
        })?;
        let value = Length::from_micrometres(value)
            .map_err(|error| Failure::session(format!("{token}: {error}")))?;
        inputs.push(Input {
            token: token.to_owned(),
            value,
        });
    }
    Ok(inputs)
}

struct Session<R: Repository> {
    identities: Identities,
    design: EntityId,
    actor: CommandActorClaim,
    bus: CommandBus<DesignSources, LocalOperator>,
    inputs: Vec<Input>,
    undo: Vec<CommandMutationGroup>,
    redo: Vec<CommandMutationGroup>,
    next_id: u128,
    store: ProjectStore<R>,
    // What the project holds beyond what this session authors, so a save keeps unknown fields and files.
    base: Project,
    design_text: String,
    recipe: String,
    drawing: Option<String>,
    pattern: Option<sc_artifacts::Pattern>,
}

impl<R: Repository> Session<R> {
    fn fresh_id(&mut self) -> EntityId {
        self.next_id += 1;
        self.identities.nth(self.next_id)
    }

    fn revision(&self) -> DesignRevision {
        self.bus.state().inputs().revision().clone()
    }

    fn context(
        &mut self,
        group: CommandGroupId,
        revision: DesignRevision,
    ) -> CommandMutationContext {
        let key = CommandIdempotencyKey::from_entity_id(self.fresh_id());
        CommandMutationContext::new(self.design, self.actor, revision, key, Some(group))
    }

    fn change(
        &mut self,
        step: usize,
        token: &str,
        value: Length,
        expected: Option<DesignRevision>,
    ) -> Result<Value, Failure> {
        let at = self
            .inputs
            .iter()
            .position(|input| input.token == token)
            .ok_or_else(|| {
                Failure::at(step, "unknown_input", format!("no input is named {token}"))
            })?;
        let ordinal = u128::try_from(at).unwrap_or(0);
        let replacement = declaration(self.identities, ordinal, value)
            .map_err(|message| Failure::at(step, "invalid_value", message))?;
        let group_id = CommandGroupId::from_entity_id(self.fresh_id());
        let revision = expected.unwrap_or_else(|| self.revision());
        let context = self.context(group_id, revision);
        let payload = MutationPayload::ChangeMeasurement(ChangeMeasurement::new(
            self.identities.measurement(ordinal),
            replacement,
        ));
        let group =
            CommandMutationGroup::new(group_id, vec![CommandMutation::new(context, payload)])
                .map_err(|error| Failure::at(step, "refused", error.to_string()))?;
        self.bus
            .commit_group(&group)
            .map_err(|error| Failure::at(step, "refused", error.to_string()))?;
        if let Some(input) = self.inputs.get_mut(at) {
            input.value = value;
        }
        self.undo.push(group);
        self.redo.clear();
        self.pattern = None;
        Ok(Value::Text(self.revision_text()))
    }

    fn history(
        &mut self,
        step: usize,
        direction: CommandHistoryDirection,
    ) -> Result<Value, Failure> {
        let (name, source) = match direction {
            CommandHistoryDirection::Undo => ("undo", self.undo.pop()),
            CommandHistoryDirection::Redo => ("redo", self.redo.pop()),
        };
        let original = source.ok_or_else(|| {
            Failure::at(
                step,
                "nothing_to_change",
                format!("there is nothing to {name}"),
            )
        })?;
        let key = CommandIdempotencyKey::from_entity_id(self.fresh_id());
        let context = CommandMutationContext::new(
            self.design,
            self.actor,
            self.revision(),
            key,
            Some(original.id()),
        );
        let request = CommandHistoryRequest::new(context, original.clone(), direction)
            .map_err(|error| Failure::at(step, "refused", error.to_string()))?;
        self.bus
            .apply_history(&request)
            .map_err(|error| Failure::at(step, "refused", error.to_string()))?;
        match direction {
            CommandHistoryDirection::Undo => self.redo.push(original),
            CommandHistoryDirection::Redo => self.undo.push(original),
        }
        self.sync_inputs();
        self.pattern = None;
        Ok(Value::Text(self.revision_text()))
    }

    // After history moves, the Design is the authority on each value.
    fn sync_inputs(&mut self) {
        let lengths = self.bus.state().inputs().inputs().lengths().to_vec();
        for (input, length) in self.inputs.iter_mut().zip(lengths) {
            if let sc_core::value::LengthState::Known { value, .. } = length.definition().state {
                input.value = value;
            }
        }
    }

    fn revision_text(&self) -> String {
        self.revision().to_string()
    }

    fn evaluate(&self, step: usize) -> Result<Value, Failure> {
        let fail = |code, error: String| Failure::at(step, code, error);
        let root = self.bus.state().inputs();
        let scope = root
            .prepare_formula_scope(limits())
            .map_err(|error| fail("recipe", error.to_string()))?;
        let normalized = scope
            .normalize_recipe()
            .map_err(|error| fail("recipe", error.to_string()))?;
        let checked = normalized
            .check_kinds()
            .map_err(|error| fail("recipe", error.to_string()))?;
        let proof = checked.proof();
        let statements = proof.statements().len();
        let dependencies = proof.dependencies().count();
        let nodes: usize = proof
            .statements()
            .iter()
            .map(|statement| match statement.kind() {
                FormulaCheckedStatementKind::Let { expression } => expression.node_count(),
                FormulaCheckedStatementKind::Assert { left, right } => {
                    left.node_count() + right.node_count()
                }
            })
            .sum();
        let plan = proof
            .admit_execution(FormulaRecipeExecutionLimits::new(
                statements,
                nodes,
                2 * nodes + dependencies,
                statements,
            ))
            .map_err(|error| fail("recipe", error.to_string()))?
            .prepare_scalar(root.inputs())
            .map_err(|error| fail("recipe", error.to_string()))?;
        let context = FormulaScalarExecutionContext::new(
            FormulaToleranceContext::empty(),
            FormulaSizeContext::empty(),
        );
        let result = plan
            .replay(context)
            .map_err(|error| fail("recipe", error.to_string()))?;
        let canonical = result
            .canonical_bindings(1 << 20)
            .map_err(|error| fail("recipe", error.to_string()))?;
        Ok(Value::List(bindings(canonical.as_str())))
    }

    // Every length a session can name: the inputs, the recipe's bindings, and `zero`.
    fn lengths(&self, step: usize) -> Result<BTreeMap<String, i64>, Failure> {
        let mut known: BTreeMap<String, i64> = self
            .inputs
            .iter()
            .map(|input| (input.token.clone(), input.value.as_micrometres()))
            .collect();
        known.insert("zero".to_owned(), 0);
        if let Value::List(bindings) = self.evaluate(step)? {
            for binding in &bindings {
                let Some(binding) = binding.as_object() else {
                    continue;
                };
                let (Some(name), Some(kind), Some(value)) = (
                    binding.get("name").and_then(Value::as_text),
                    binding.get("kind").and_then(Value::as_text),
                    binding.get("value").and_then(Value::as_text),
                ) else {
                    continue;
                };
                if let ("length", Ok(number)) = (kind, value.parse::<i64>()) {
                    known.insert(name.to_owned(), number);
                }
            }
        }
        Ok(known)
    }

    fn draft(&mut self, step: usize, fields: &BTreeMap<String, Value>) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "draft", message);
        let known = self.lengths(step)?;
        let pattern = crate::draft::draft(&known, fields, &self.design_text, &self.revision_text())
            .map_err(fail)?;
        let text = sc_artifacts::to_canonical_text(&pattern);
        let digest = sc_store::sha256_hex(text.as_bytes());
        let pieces = pattern
            .pieces
            .iter()
            .map(|piece| Value::Text(piece.name.clone()))
            .collect();
        self.pattern = Some(pattern);
        let mut record = BTreeMap::new();
        record.insert("canonical_sha256".to_owned(), Value::Text(digest));
        record.insert("pieces".to_owned(), Value::List(pieces));
        Ok(Value::Object(record))
    }

    // Writes one artifact of the drafted pattern into the project, with its record beside it. The next save lands them.
    fn export_pattern(
        &mut self,
        step: usize,
        fields: &BTreeMap<String, Value>,
    ) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "export_pattern", message);
        let pattern = self
            .pattern
            .as_ref()
            .ok_or_else(|| fail("export_pattern needs a draft first".to_owned()))?;
        let format = fields
            .get("format")
            .and_then(Value::as_text)
            .ok_or_else(|| fail("export_pattern needs a format".to_owned()))?;
        let target = fields
            .get("path")
            .and_then(Value::as_text)
            .ok_or_else(|| fail("export_pattern needs a path".to_owned()))?;
        let (bytes, record): (Vec<u8>, Option<String>) = match format {
            "canonical" => (sc_artifacts::to_canonical_text(pattern).into_bytes(), None),
            "dxf" => {
                let semantics = match fields.get("semantics").and_then(Value::as_text) {
                    Some("cut_as_1") => sc_artifacts::LineSemantics::CutAsCut,
                    Some("sew_as_1") => sc_artifacts::LineSemantics::SewAsCut,
                    // A profile chooses this, and there is no default.
                    _ => {
                        return Err(fail(
                            "a dxf export names its semantics: cut_as_1 or sew_as_1".to_owned(),
                        ))
                    }
                };
                let flattening = sc_artifacts::Flattening {
                    bound: 100,
                    achieved: crate::draft::TOLERANCE + 1,
                };
                let artifact = sc_artifacts::write_dxf(pattern, semantics, flattening)
                    .map_err(|error| fail(error.to_string()))?;
                (artifact.text.into_bytes(), Some(artifact.record))
            }
            "pdf" => {
                let layout = match fields.get("layout").and_then(Value::as_text) {
                    Some("a0") => sc_artifacts::PageLayout::Single(sc_artifacts::Sheet::A0),
                    Some("a0_tiled") => sc_artifacts::PageLayout::Tiled {
                        sheet: sc_artifacts::Sheet::A0,
                        overlap: 10_000,
                    },
                    Some("a3_tiled") => sc_artifacts::PageLayout::Tiled {
                        sheet: sc_artifacts::Sheet::A3,
                        overlap: 10_000,
                    },
                    Some("a4_tiled") => sc_artifacts::PageLayout::Tiled {
                        sheet: sc_artifacts::Sheet::A4,
                        overlap: 10_000,
                    },
                    _ => {
                        return Err(fail(
                            "a pdf export names its layout: a0, a0_tiled, a3_tiled or a4_tiled"
                                .to_owned(),
                        ))
                    }
                };
                let artifact = sc_artifacts::write_pdf(pattern, layout)
                    .map_err(|error| fail(error.to_string()))?;
                (artifact.bytes, Some(artifact.record))
            }
            other => return Err(fail(format!("no format is named {other}"))),
        };
        let digest = sc_store::sha256_hex(&bytes);
        let path = sc_store::FilePath::new(target).map_err(|error| fail(error.to_string()))?;
        self.base
            .set_file(path, bytes)
            .map_err(|error| fail(error.to_string()))?;
        if let Some(record) = record {
            let beside = sc_store::FilePath::new(&format!("{target}.record.json"))
                .map_err(|error| fail(error.to_string()))?;
            self.base
                .set_file(beside, record)
                .map_err(|error| fail(error.to_string()))?;
        }
        Ok(Value::Text(digest))
    }

    // Each point names two lengths, a recipe binding or an input, or `zero`.
    fn render(&mut self, step: usize, fields: &BTreeMap<String, Value>) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "render", message);
        let known = self.lengths(step)?;
        let listed = fields
            .get("outline")
            .and_then(Value::as_list)
            .ok_or_else(|| fail("render needs an outline".to_owned()))?;
        let mut points = Vec::new();
        for point in listed {
            let names = point.as_list().unwrap_or(&[]);
            let [x, y] = names else {
                return Err(fail("each outline point names two lengths".to_owned()));
            };
            let read = |name: &Value| -> Result<i64, Failure> {
                let name = name
                    .as_text()
                    .ok_or_else(|| fail("a point's lengths are named as text".to_owned()))?;
                known
                    .get(name)
                    .copied()
                    .ok_or_else(|| fail(format!("no length is named {name}")))
            };
            points.push((read(x)?, read(y)?));
        }
        let drawing = svg(&points).map_err(fail)?;
        self.drawing = Some(drawing.clone());
        Ok(Value::Text(drawing))
    }

    fn export(&mut self, step: usize, fields: &BTreeMap<String, Value>) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "export", message);
        let target = fields
            .get("path")
            .and_then(Value::as_text)
            .ok_or_else(|| fail("export needs a path".to_owned()))?;
        let path = sc_store::FilePath::new(target).map_err(|error| fail(error.to_string()))?;
        let drawing = self
            .drawing
            .clone()
            .ok_or_else(|| fail("export needs a render first".to_owned()))?;
        let digest = sc_store::sha256_hex(drawing.as_bytes());
        self.base
            .set_file(path, drawing)
            .map_err(|error| fail(error.to_string()))?;
        Ok(Value::Text(digest))
    }

    fn save(&mut self, step: usize) -> Result<Value, Failure> {
        let project = self
            .project()
            .map_err(|message| Failure::at(step, "save", message))?;
        let commit = self
            .store
            .save(&project)
            .map_err(|error| Failure::at(step, "save", error.to_string()))?;
        self.base = project;
        Ok(Value::Text(format!("{commit:?}").to_lowercase()))
    }

    // What a project holds today: the inputs and the recipe, the Design's authored content so far.
    fn project(&self) -> Result<Project, String> {
        let mut project = self.base.clone();
        project.set_field("design", Value::Text(self.design_text.clone()));
        project.set_field("revision", Value::Text(self.revision_text()));
        // Saved separately from the revision: undo can move the Design below the highest one issued.
        let maximum = self
            .bus
            .maximum_issued_revision()
            .ok_or("this bus has no journal")?;
        project.set_field("maximum_issued_revision", Value::Text(maximum.to_string()));
        let sequence = self.bus.audit_sequence().ok_or("this bus has no journal")?;
        project.set_field(
            "audit_sequence",
            Value::Text(sequence.as_revision().to_string()),
        );
        project.set_field("identities_issued", Value::Text(self.next_id.to_string()));
        let list = self.inputs.iter().map(|input| {
            let mut fields = BTreeMap::new();
            fields.insert("token".to_owned(), Value::Text(input.token.clone()));
            fields.insert("um".to_owned(), Value::Int(input.value.as_micrometres()));
            Value::Object(fields)
        });
        let path = |text: &str| sc_store::FilePath::new(text).map_err(|error| error.to_string());
        project
            .set_file(path("inputs.json")?, Value::List(list.collect()).to_text())
            .map_err(|error| error.to_string())?;
        project
            .set_file(path("recipe.formula")?, self.recipe.clone())
            .map_err(|error| error.to_string())?;
        Ok(project)
    }

    fn reopen(&mut self, step: usize) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "reopen", message);
        let saved = self.store.load().map_err(|error| fail(error.to_string()))?;
        self.pattern = None;
        let path = |text: &str| sc_store::FilePath::new(text).map_err(|error| error.to_string());
        let inputs = saved
            .project
            .file(&path("inputs.json").map_err(fail)?)
            .ok_or_else(|| fail("the project has no inputs.json".to_owned()))?;
        let text = String::from_utf8_lossy(inputs).into_owned();
        let parsed = sc_store::parse(&text).map_err(|error| fail(error.to_string()))?;
        let mut holder = BTreeMap::new();
        holder.insert("inputs".to_owned(), parsed);
        self.inputs = read_inputs(&holder).map_err(|failure| fail(failure.message))?;
        let recipe = saved
            .project
            .file(&path("recipe.formula").map_err(fail)?)
            .ok_or_else(|| fail("the project has no recipe.formula".to_owned()))?;
        self.recipe = String::from_utf8_lossy(recipe).into_owned();
        let counter = |name: &str| -> Result<u128, String> {
            saved
                .project
                .field(name)
                .and_then(Value::as_text)
                .and_then(|text| text.parse().ok())
                .ok_or_else(|| format!("the project has no whole number {name}"))
        };
        let revision = DesignRevision::from_counter(counter("revision").map_err(fail)?);
        let maximum =
            DesignRevision::from_counter(counter("maximum_issued_revision").map_err(fail)?);
        let sequence = DesignRevision::from_counter(counter("audit_sequence").map_err(fail)?);
        self.next_id = counter("identities_issued").map_err(fail)?;
        self.base = saved.project.clone();
        // Undo does not cross a reopen yet, but revisions and audit numbers carry on and are never reissued.
        let garment = build(self.design, revision, &self.inputs, &self.recipe).map_err(fail)?;
        self.bus = new_bus(garment, self.design, self.actor, self.identities).map_err(fail)?;
        self.bus
            .resume_issuance(maximum, CommandAuditSequence::from_revision(sequence))
            .map_err(|error| fail(error.to_string()))?;
        self.undo.clear();
        self.redo.clear();
        Ok(Value::Text(self.revision_text()))
    }
}

fn limits() -> DesignResourceLimits {
    DesignResourceLimits::new(NonZeroUsize::MAX, NonZeroUsize::MAX, NonZeroU64::MAX)
}

fn new_bus(
    garment: sc_core::design::DesignGarment<DesignSources>,
    design: EntityId,
    actor: CommandActorClaim,
    identities: Identities,
) -> Result<CommandBus<DesignSources, LocalOperator>, String> {
    let scope = CommandScope::new(
        design,
        CommandId::ALL.to_vec(),
        SystemTime::UNIX_EPOCH,
        SystemTime::UNIX_EPOCH + Duration::from_secs(2),
    )
    .map_err(|error| error.to_string())?;
    let grant = CommandGrant::new(
        identities.nth(20),
        actor,
        vec![CommandAuthority::Inspect, CommandAuthority::Commit],
        scope,
        CommandGrantStatus::Active,
    )
    .map_err(|error| error.to_string())?;
    let capacity = |value: usize| NonZeroUsize::new(value).unwrap_or(NonZeroUsize::MIN);
    let mut bus = CommandBus::with_edit_history(
        garment,
        LocalOperator {
            actor,
            grants: vec![grant],
        },
        vec![],
        capacity(MAX_ACTIONS),
        capacity(UNDO_DEPTH),
        capacity(MAX_ACTIONS),
    )
    .map_err(|error| error.to_string())?;
    let any = NonZeroUsize::MAX;
    bus.install_operation_limits(CommandOperationLimits::new(any, any, any, any))
        .map_err(|error| error.to_string())?;
    bus.install_model_limits(limits())
        .map_err(|error| error.to_string())?;
    bus.install_retained_limits(limits())
        .map_err(|error| error.to_string())?;
    Ok(bus)
}

// `(bindings (bind name kind:value (sources ...))...)` as a list of records.
fn bindings(canonical: &str) -> Vec<Value> {
    canonical
        .split("(bind ")
        .skip(1)
        .filter_map(|part| {
            let mut words = part.split_whitespace();
            let (name, typed) = (words.next()?, words.next()?);
            let (kind, value) = typed.split_once(':')?;
            let mut fields = BTreeMap::new();
            fields.insert("name".to_owned(), Value::Text(name.to_owned()));
            fields.insert("kind".to_owned(), Value::Text(kind.to_owned()));
            fields.insert("value".to_owned(), Value::Text(value.to_owned()));
            Some(Value::Object(fields))
        })
        .collect()
}

// The one constraint the session's profile states: a value of a chosen input must be one of some allowed values.
fn solve(
    step: usize,
    inputs: &[Input],
    fields: &BTreeMap<String, Value>,
) -> Result<Value, Failure> {
    let fail = |message: String| Failure::at(step, "solve", message);
    let token = field(fields, "token")
        .map_err(|failure| fail(failure.message))?
        .as_text()
        .ok_or_else(|| fail("token must be text".to_owned()))?;
    let at_most = field(fields, "at_most_um")
        .map_err(|failure| fail(failure.message))?
        .as_int()
        .ok_or_else(|| fail("at_most_um must be a whole number".to_owned()))?;
    let input = inputs
        .iter()
        .find(|input| input.token == token)
        .ok_or_else(|| Failure::at(step, "unknown_input", format!("no input is named {token}")))?;
    let current = sc_constraints::Rational::int(input.value.as_micrometres());
    let limit = sc_constraints::Rational::int(at_most);
    let domain = Domain::Numbers {
        values: vec![current],
        unit: Some("um".to_owned()),
    };
    let variable = Variable::new(token, token, domain, Binding::Known(Fact::Number(current)));
    let rule = Rule::hard(
        "input-limit",
        vec![
            TemplatePart::Label(token.to_owned()),
            TemplatePart::Text(format!(" must be at most {at_most} micrometres")),
        ],
        Expr::Compare(
            sc_constraints::Comparison::AtMost,
            sc_constraints::Linear::var(token),
            sc_constraints::Linear::constant(limit, Some("um")),
        ),
    );
    let problem = Problem::new(AST_VERSION, vec![variable], vec![rule])
        .map_err(|error| fail(error.to_string()))?;
    let outcome = problem.solve(Limits::default());
    let verdict = match &outcome {
        Outcome::Satisfied(_) => "satisfied",
        Outcome::ProvenUnsatisfiable(_) => "proven_unsatisfiable",
        Outcome::Unknown { .. } => "unknown",
        Outcome::NumericallyFailed { .. } => "numerically_failed",
    };
    let sentences = problem
        .explain(&outcome, &Catalog::english())
        .into_iter()
        .map(Value::Text)
        .collect();
    let mut record = BTreeMap::new();
    record.insert("outcome".to_owned(), Value::Text(verdict.to_owned()));
    record.insert("explanation".to_owned(), Value::List(sentences));
    Ok(Value::Object(record))
}

/// The steps a session can take. Anything else is refused by name.
pub const STEPS: [&str; 11] = [
    "evaluate",
    "change",
    "undo",
    "redo",
    "solve",
    "render",
    "draft",
    "export_pattern",
    "export",
    "save",
    "reopen",
];

/// A session opened to be driven one step at a time, which is what `replay` does with a file's steps. A front end
/// drives this and nothing else, so it has no way to change a Design that a session file does not also have.
pub struct Running<R: Repository> {
    state: Session<R>,
    next: usize,
}

impl<R: Repository> fmt::Debug for Running<R> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Running")
            .field("next", &self.next)
            .finish_non_exhaustive()
    }
}

impl<R: Repository> Running<R> {
    /// Opens a session against a project, reading its format, design, inputs and recipe. Its steps are the caller's.
    ///
    /// # Errors
    /// The session is malformed, or the project cannot be read.
    pub fn open(session: &Value, repository: R) -> Result<Self, Failure> {
        let fields = session
            .as_object()
            .ok_or_else(|| Failure::session("a session must be an object"))?;
        if field(fields, "format")?.as_int() != Some(FORMAT) {
            return Err(Failure::session(format!(
                "this version reads session format {FORMAT}"
            )));
        }
        let design_text = field(fields, "design")?
            .as_text()
            .ok_or_else(|| Failure::session("design must be text"))?;
        let design: EntityId = design_text
            .parse()
            .map_err(|error| Failure::session(format!("design: {error}")))?;
        let inputs = read_inputs(fields)?;
        let recipe = field(fields, "recipe")?
            .as_text()
            .ok_or_else(|| Failure::session("recipe must be text"))?
            .to_owned();
        let identities = Identities::new(design);
        let actor = CommandActorClaim::Human(identities.nth(21));
        let garment =
            build(design, DesignRevision::initial(), &inputs, &recipe).map_err(Failure::session)?;
        let bus = new_bus(garment, design, actor, identities).map_err(Failure::session)?;
        let (mut store, _) =
            ProjectStore::open(repository).map_err(|error| Failure::session(error.to_string()))?;
        // A project already here keeps whatever this session does not author.
        let base = match store.load() {
            Ok(loaded) => loaded.project,
            Err(ProjectError::NoProject) => Project::new(),
            Err(error) => return Err(Failure::session(error.to_string())),
        };
        let state = Session {
            identities,
            design,
            actor,
            bus,
            inputs,
            undo: Vec::new(),
            redo: Vec::new(),
            next_id: 1 << 32,
            store,
            base,
            design_text: design_text.to_owned(),
            recipe,
            drawing: None,
            pattern: None,
        };
        Ok(Self { state, next: 0 })
    }

    /// Takes one step, returning its transcript record. A step that failed changed nothing.
    ///
    /// # Errors
    /// The step failed, with a structured diagnostic.
    pub fn step(&mut self, item: &Value) -> Result<Value, Failure> {
        let step = self.next;
        let state = &mut self.state;
        let item = item
            .as_object()
            .ok_or_else(|| Failure::at(step, "session", "a step must be an object"))?;
        let what = item
            .get("do")
            .and_then(Value::as_text)
            .ok_or_else(|| Failure::at(step, "session", "a step needs a do field"))?;
        let result = match what {
            "evaluate" => state.evaluate(step)?,
            "change" => {
                let token = item
                    .get("token")
                    .and_then(Value::as_text)
                    .ok_or_else(|| Failure::at(step, "session", "change needs a token"))?;
                let value = item
                    .get("um")
                    .and_then(Value::as_int)
                    .ok_or_else(|| Failure::at(step, "session", "change needs um"))?;
                let value = Length::from_micrometres(value)
                    .map_err(|error| Failure::at(step, "invalid_value", error.to_string()))?;
                // A script may state the revision it edited, and the bus then checks that.
                let expected = match item.get("expect_revision").and_then(Value::as_text) {
                    None => None,
                    Some(text) => {
                        let counter: u128 = text.parse().map_err(|_| {
                            Failure::at(step, "session", "expect_revision must be a whole number")
                        })?;
                        Some(DesignRevision::from_counter(counter))
                    }
                };
                state.change(step, token, value, expected)?
            }
            "undo" => state.history(step, CommandHistoryDirection::Undo)?,
            "redo" => state.history(step, CommandHistoryDirection::Redo)?,
            "solve" => solve(step, &state.inputs, item)?,
            "render" => state.render(step, item)?,
            "draft" => state.draft(step, item)?,
            "export_pattern" => state.export_pattern(step, item)?,
            "export" => state.export(step, item)?,
            "save" => state.save(step)?,
            "reopen" => state.reopen(step)?,
            other => {
                return Err(Failure::at(
                    step,
                    "session",
                    format!("no step is named {other}"),
                ))
            }
        };
        let mut record = BTreeMap::new();
        record.insert("do".to_owned(), Value::Text(what.to_owned()));
        record.insert("result".to_owned(), result);
        record.insert("revision".to_owned(), Value::Text(state.revision_text()));
        record.insert(
            "step".to_owned(),
            Value::Int(i64::try_from(step).unwrap_or(i64::MAX)),
        );
        self.next += 1;
        Ok(Value::Object(record))
    }

    /// The drafted pattern, or `None` when nothing has been drafted since the Design last changed.
    #[must_use]
    pub const fn pattern(&self) -> Option<&sc_artifacts::Pattern> {
        self.state.pattern.as_ref()
    }

    /// The Design's revision.
    #[must_use]
    pub fn revision(&self) -> String {
        self.state.revision_text()
    }

    /// Each input's name and value in micrometres, in the session's order.
    #[must_use]
    pub fn inputs(&self) -> Vec<(String, i64)> {
        self.state
            .inputs
            .iter()
            .map(|input| (input.token.clone(), input.value.as_micrometres()))
            .collect()
    }

    /// Whether there is a change to undo.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.state.undo.is_empty()
    }

    /// Whether there is a change to redo.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.state.redo.is_empty()
    }
}

/// Run a session against a project directory, and return the transcript.
///
/// # Errors
/// The first failure stops the session with a structured diagnostic. A step that failed changed nothing.
pub fn replay<R: Repository>(session: &Value, repository: R) -> Result<Value, Failure> {
    let mut running = Running::open(session, repository)?;
    let fields = session
        .as_object()
        .ok_or_else(|| Failure::session("a session must be an object"))?;
    let steps = field(fields, "steps")?
        .as_list()
        .ok_or_else(|| Failure::session("steps must be a list"))?;
    let mut transcript = Vec::new();
    for item in steps {
        transcript.push(running.step(item)?);
    }
    Ok(Value::List(transcript))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use super::*;
    use sc_core::command::CommandResumeError;

    fn parts() -> (
        sc_core::design::DesignGarment<DesignSources>,
        EntityId,
        CommandActorClaim,
        Identities,
    ) {
        let design = EntityId::from_bits(1 << 80);
        let identities = Identities::new(design);
        let inputs = [Input {
            token: "a".to_owned(),
            value: Length::ZERO,
        }];
        let garment = build(design, DesignRevision::from_counter(5), &inputs, "").expect("builds");
        (
            garment,
            design,
            CommandActorClaim::Human(identities.nth(21)),
            identities,
        )
    }

    fn sequence(count: u128) -> CommandAuditSequence {
        CommandAuditSequence::from_revision(DesignRevision::from_counter(count))
    }

    #[test]
    fn resuming_refuses_what_would_rewind_or_disagree() {
        let (garment, design, actor, identities) = parts();
        let mut bus = new_bus(garment, design, actor, identities).expect("a bus");
        assert_eq!(
            bus.maximum_issued_revision(),
            Some(&DesignRevision::from_counter(5))
        );
        assert_eq!(
            bus.resume_issuance(DesignRevision::from_counter(4), sequence(1)),
            Err(CommandResumeError::BehindCurrent {
                current: DesignRevision::from_counter(5),
                maximum: DesignRevision::from_counter(4)
            })
        );
        assert_eq!(
            bus.maximum_issued_revision(),
            Some(&DesignRevision::from_counter(5)),
            "a refusal changes nothing"
        );

        bus.resume_issuance(DesignRevision::from_counter(9), sequence(3))
            .expect("resumes");
        assert_eq!(
            bus.maximum_issued_revision(),
            Some(&DesignRevision::from_counter(9))
        );
        assert_eq!(bus.audit_sequence(), Some(&sequence(3)));
        assert_eq!(
            bus.resume_issuance(DesignRevision::from_counter(20), sequence(4)),
            Err(CommandResumeError::AlreadyIssuing)
        );

        let (garment, _, _, _) = parts();
        let operator = LocalOperator {
            actor,
            grants: vec![],
        };
        let mut plain = CommandBus::new(garment, operator);
        assert_eq!(
            plain.resume_issuance(DesignRevision::from_counter(9), sequence(3)),
            Err(CommandResumeError::NoJournal)
        );
    }
}
