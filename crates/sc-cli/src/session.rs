//! Replays a session: every change goes through the command bus, with revision checks and undo.
use crate::design::{build, declaration, Identities, Input};
use crate::outline::svg;
use core::cell::RefCell;
use core::fmt;
use sc_constraints::{
    Binding, Catalog, Domain, Expr, Limits, Outcome, Problem, Rule, TemplatePart, Value as Fact,
    Variable, AST_VERSION,
};
use sc_core::{
    command::{
        ChangeDrafting, ChangeMeasurement, CommandActorClaim, CommandAuditSequence,
        CommandAuthority, CommandAuthoritySnapshot, CommandAuthoritySource, CommandBus,
        CommandGrant, CommandGrantStatus, CommandGroupId, CommandHistoryDirection,
        CommandHistoryRequest, CommandId, CommandIdempotencyKey, CommandMutation,
        CommandMutationContext, CommandMutationGroup, CommandOperationLimits, CommandScope,
        DesignRevision, MutationPayload,
    },
    design::{DesignInputs, DesignResourceLimits},
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
    // What fits came to, by exactly what they depend on, so that it is invisible except in how long it takes.
    fits_solved: RefCell<BTreeMap<String, Vec<Input>>>,
    drawing: Option<String>,
    pattern: Option<sc_artifacts::Pattern>,
    authorities: Vec<CommandAuthority>,
    // Every step that changed anything, and who took it.
    trace: Vec<Value>,
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

    fn record_trace(&mut self, what: &str, step: usize) {
        let mut record = BTreeMap::new();
        record.insert("do".to_owned(), Value::Text(what.to_owned()));
        record.insert(
            "step".to_owned(),
            Value::Int(i64::try_from(step).unwrap_or(i64::MAX)),
        );
        record.insert("revision".to_owned(), Value::Text(self.revision_text()));
        record.insert(
            "initiator".to_owned(),
            Value::Text(self.actor.initiator().to_string()),
        );
        record.insert(
            "principal".to_owned(),
            Value::Text(self.actor.principal().to_string()),
        );
        let kind = match self.actor {
            CommandActorClaim::Human(_) => "human",
            CommandActorClaim::Agent { .. } => "agent",
        };
        record.insert("actor".to_owned(), Value::Text(kind.to_owned()));
        self.trace.push(Value::Object(record));
    }

    // What closing each closure of the drawing does, from the drawing itself.
    fn closures(&self, step: usize) -> Result<Value, Failure> {
        let pattern = self.pattern.as_ref().ok_or_else(|| {
            Failure::at(step, "closures", "closures needs a draft first".to_owned())
        })?;
        Ok(Value::List(
            sc_artifacts::closure_reports(pattern)
                .iter()
                .map(sc_artifacts::ClosureReport::to_value)
                .collect(),
        ))
    }

    fn walk(&self, step: usize) -> Result<Value, Failure> {
        let pattern = self
            .pattern
            .as_ref()
            .ok_or_else(|| Failure::at(step, "walk", "a walk needs a draft first".to_owned()))?;
        let reports = sc_artifacts::walk(pattern).map_err(|error| {
            Failure::at(
                step,
                "walk",
                format!("the seam {}: {}", error.seam, error.message),
            )
        })?;
        Ok(Value::List(
            reports
                .iter()
                .map(sc_artifacts::WalkReport::to_value)
                .collect(),
        ))
    }

    // Adds an entry to one of the document's lists, as one edit.
    fn document_with(
        &self,
        step: usize,
        list: &str,
        entry: Value,
    ) -> Result<BTreeMap<String, Value>, Failure> {
        let mut document = self.document().ok_or_else(|| {
            Failure::at(
                step,
                "draft",
                "there is nothing drafted to edit yet".to_owned(),
            )
        })?;
        let mut items = document
            .get(list)
            .and_then(Value::as_list)
            .map(<[Value]>::to_vec)
            .unwrap_or_default();
        items.push(entry);
        document.insert(list.to_owned(), Value::List(items));
        Ok(document)
    }

    fn plant_balance_notch(
        &mut self,
        step: usize,
        fields: &BTreeMap<String, Value>,
    ) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "plant_balance_notch", message);
        let mut entry = fields.clone();
        entry.remove("do");
        let document = self.document_with(step, "balance_notches", Value::Object(entry))?;
        let known = self.lengths_for(step, Some(&document))?;
        crate::draft::draft(&known, &document, &self.design_text, &self.revision_text())
            .map_err(fail)?;
        self.commit_document(step, Value::Object(document).to_text())?;
        Ok(Value::Text(self.revision_text()))
    }

    // Truing never moves a thing it was not asked to. The caller says which side and how far it will accept, the differential before and after are reported, and anything left is
    // what the seam declares as ease.
    fn true_seam(
        &mut self,
        step: usize,
        fields: &BTreeMap<String, Value>,
    ) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "true_seam", message);
        let name = fields
            .get("seam")
            .and_then(Value::as_text)
            .ok_or_else(|| fail("a true names its seam".to_owned()))?
            .to_owned();
        let limit = fields
            .get("limit_um")
            .and_then(Value::as_int)
            .ok_or_else(|| fail("a true says how far it may move, as limit_um".to_owned()))?;
        let known = self.lengths(step)?;
        let before_document = self
            .document()
            .ok_or_else(|| fail("there is nothing drafted to true yet".to_owned()))?;
        let before = crate::draft::draft(
            &known,
            &before_document,
            &self.design_text,
            &self.revision_text(),
        )
        .map_err(fail)?;
        let mut entry = fields.clone();
        entry.remove("do");
        entry.remove("limit_um");
        let document = self.document_with(step, "trues", Value::Object(entry))?;
        let after =
            crate::draft::draft(&known, &document, &self.design_text, &self.revision_text())
                .map_err(fail)?;
        let report =
            |pattern: &sc_artifacts::Pattern| -> Result<sc_artifacts::WalkReport, Failure> {
                sc_artifacts::walk(pattern)
                    .map_err(|error| fail(format!("the seam {}: {}", error.seam, error.message)))?
                    .into_iter()
                    .find(|item| item.seam == name)
                    .ok_or_else(|| fail(format!("no seam is named {name}")))
            };
        let (was, now) = (report(&before)?, report(&after)?);
        let moved = was.unexplained.abs();
        if moved > limit {
            return Err(fail(format!(
                "truing {name} would move an end by {moved}, past the limit of {limit}"
            )));
        }
        // An end in the middle of an edge, or on a mirror, is not a corner this piece can move, and a true that changed nothing is not a true.
        if now.finding == "defect" {
            return Err(fail(format!(
                "truing {name} leaves it out by {}, because the end of that side is not a corner of a piece that can move",
                now.unexplained
            )));
        }
        self.commit_document(step, Value::Object(document).to_text())?;
        let mut result = BTreeMap::new();
        result.insert("moved_um".to_owned(), Value::Int(moved));
        result.insert("before".to_owned(), was.to_value());
        result.insert("after".to_owned(), now.to_value());
        Ok(Value::Object(result))
    }

    // A changed document, drawn first so that what cannot be drawn changes nothing, then committed as one edit.
    fn commit_edited(
        &mut self,
        step: usize,
        word: &'static str,
        document: BTreeMap<String, Value>,
    ) -> Result<Value, Failure> {
        let known = self.lengths_for(step, Some(&document))?;
        let pattern =
            crate::draft::draft(&known, &document, &self.design_text, &self.revision_text())
                .map_err(|message| Failure::at(step, word, message))?;
        self.commit_document(step, Value::Object(document).to_text())?;
        let mut result = BTreeMap::new();
        result.insert("revision".to_owned(), Value::Text(self.revision_text()));
        result.insert(
            "repairs".to_owned(),
            Value::List(
                pattern
                    .repairs
                    .iter()
                    .map(sc_artifacts::Repair::to_value)
                    .collect(),
            ),
        );
        Ok(Value::Object(result))
    }

    // An edit to an edge is an entry in its piece's description, done again at every draft, so it follows a changed measurement. What it orphans is reported, and it changes nothing else.
    fn edit_edge(
        &mut self,
        step: usize,
        fields: &BTreeMap<String, Value>,
    ) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "edit_edge", message);
        let mut entry = fields.clone();
        entry.remove("do");
        let name = entry
            .remove("piece")
            .and_then(|value| value.as_text().map(str::to_owned))
            .ok_or_else(|| fail("an edit names its piece".to_owned()))?;
        let mut document = self
            .document()
            .ok_or_else(|| fail("there is nothing drafted to edit yet".to_owned()))?;
        let piece = piece_in(&mut document, &name)
            .ok_or_else(|| fail(format!("no piece is named {name}")))?;
        if piece.contains_key("mirror_of") {
            return Err(fail(format!(
                "{name} is a mirror, so an edit to it is made on the piece it mirrors"
            )));
        }
        let mut edits = piece
            .get("edits")
            .and_then(Value::as_list)
            .map(<[Value]>::to_vec)
            .unwrap_or_default();
        edits.push(Value::Object(entry));
        piece.insert("edits".to_owned(), Value::List(edits));
        self.commit_edited(step, "edit_edge", document)
    }

    fn references(&self, step: usize) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "references", message);
        let known = self.lengths(step)?;
        let document = self
            .document()
            .ok_or_else(|| fail("there is nothing drafted to look at yet".to_owned()))?;
        let (references, repairs) =
            crate::draft::references(&known, &document, &self.design_text, &self.revision_text())
                .map_err(fail)?;
        let mut result = BTreeMap::new();
        result.insert("references".to_owned(), Value::List(references));
        result.insert(
            "repairs".to_owned(),
            Value::List(repairs.iter().map(sc_artifacts::Repair::to_value).collect()),
        );
        Ok(Value::Object(result))
    }

    // Every size of the set, each a fresh evaluation of the recipe with nothing inherited from another. It changes nothing in the Design.
    fn regenerate(
        &mut self,
        step: usize,
        fields: &BTreeMap<String, Value>,
    ) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "regenerate", message);
        let document = self
            .document()
            .ok_or_else(|| fail("there is nothing drafted to size yet".to_owned()))?;
        let sizes = crate::family::sizes_of(&document).map_err(fail)?;
        let mut reports = Vec::with_capacity(sizes.list.len());
        let mut family = Vec::with_capacity(sizes.list.len());
        for size in &sizes.list {
            let at_size = |message: String| fail(format!("size {}: {message}", size.name));
            let inputs = crate::family::inputs_at(&self.inputs, size).map_err(fail)?;
            let mut described = document.clone();
            described.insert("size".to_owned(), Value::Text(size.name.clone()));
            let solved = self
                .solved(step, &inputs, Some(&described))
                .map_err(|failure| at_size(failure.message))?;
            let evaluated = self
                .evaluated(step, &solved)
                .map_err(|failure| at_size(failure.message))?;
            let known = known_lengths(&solved, &evaluated);
            let pattern =
                crate::draft::draft(&known, &described, &self.design_text, &self.revision_text())
                    .map_err(at_size)?;
            let walked = sc_artifacts::walk(&pattern)
                .map_err(|error| at_size(format!("the seam {}: {}", error.seam, error.message)))?;
            let closures = sc_artifacts::closure_reports(&pattern);
            let text = sc_artifacts::to_canonical_text(&pattern);
            let mut report = BTreeMap::new();
            report.insert("size".to_owned(), Value::Text(size.name.clone()));
            report.insert(
                "canonical_sha256".to_owned(),
                Value::Text(sc_store::sha256_hex(text.as_bytes())),
            );
            report.insert(
                "walk".to_owned(),
                Value::List(
                    walked
                        .iter()
                        .map(sc_artifacts::WalkReport::to_value)
                        .collect(),
                ),
            );
            report.insert(
                "closures".to_owned(),
                Value::List(
                    closures
                        .iter()
                        .map(sc_artifacts::ClosureReport::to_value)
                        .collect(),
                ),
            );
            report.insert(
                "fits".to_owned(),
                Value::List(
                    pattern
                        .fits
                        .iter()
                        .map(sc_artifacts::Fit::to_value)
                        .collect(),
                ),
            );
            let formulas = crate::family::formulas(&self.recipe, &solved);
            report.insert(
                "points".to_owned(),
                Value::List(crate::family::points_of(&described, &known, &formulas)),
            );
            reports.push(Value::Object(report));
            let mut member = BTreeMap::new();
            member.insert("size".to_owned(), Value::Text(size.name.clone()));
            member.insert(
                "pattern".to_owned(),
                sc_store::parse(&text).map_err(|error| at_size(error.to_string()))?,
            );
            family.push(Value::Object(member));
        }
        if let Some(target) = fields.get("path").and_then(Value::as_text) {
            let mut whole = BTreeMap::new();
            whole.insert("sample".to_owned(), Value::Text(sizes.sample.clone()));
            whole.insert("sizes".to_owned(), Value::List(family));
            let path = sc_store::FilePath::new(target).map_err(|error| fail(error.to_string()))?;
            self.base
                .set_file(path, Value::Object(whole).to_text())
                .map_err(|error| fail(error.to_string()))?;
        }
        let mut result = BTreeMap::new();
        result.insert("sample".to_owned(), Value::Text(sizes.sample));
        result.insert("sizes".to_owned(), Value::List(reports));
        Ok(Value::Object(result))
    }

    // Every length a session can name, at each size, each evaluated from the recipe with its closures holding.
    fn lengths_at_sizes(
        &self,
        step: usize,
        document: &BTreeMap<String, Value>,
        sizes: &crate::family::Sizes,
    ) -> Result<Vec<SizedLengths>, Failure> {
        let fail = |message: String| Failure::at(step, "grade", message);
        let mut at_sizes = Vec::with_capacity(sizes.list.len());
        for size in &sizes.list {
            let at_size = |message: String| fail(format!("size {}: {message}", size.name));
            let inputs = crate::family::inputs_at(&self.inputs, size).map_err(fail)?;
            let mut described = document.clone();
            described.insert("size".to_owned(), Value::Text(size.name.clone()));
            let solved = self
                .solved(step, &inputs, Some(&described))
                .map_err(|failure| at_size(failure.message))?;
            let evaluated = self
                .evaluated(step, &solved)
                .map_err(|failure| at_size(failure.message))?;
            at_sizes.push((size.name.clone(), known_lengths(&solved, &evaluated)));
        }
        Ok(at_sizes)
    }

    // What a table worked out from each size's own drawing says, or one a project holds. It changes nothing in the Design.
    fn grade_table(
        &self,
        step: usize,
        document: &BTreeMap<String, Value>,
        fields: &BTreeMap<String, Value>,
    ) -> Result<sc_grading::GradeTable, Failure> {
        let fail = |message: String| Failure::at(step, "grade", message);
        let read = |text: &str| -> Result<sc_grading::GradeTable, Failure> {
            let value = sc_store::parse(text).map_err(|error| fail(error.to_string()))?;
            sc_grading::GradeTable::from_value(&value).map_err(|error| fail(error.to_string()))
        };
        let table = if let Some(text) = fields.get("table_text").and_then(Value::as_text) {
            read(text)?
        } else if let Some(target) = fields.get("table").and_then(Value::as_text) {
            let path = sc_store::FilePath::new(target).map_err(|error| fail(error.to_string()))?;
            let bytes = self
                .base
                .file(&path)
                .ok_or_else(|| fail(format!("this project has no table at {target}")))?;
            read(core::str::from_utf8(bytes).map_err(|_| fail(format!("{target} is not text")))?)?
        } else {
            let sizes = crate::family::sizes_of(document).map_err(fail)?;
            let at_sizes = self.lengths_at_sizes(step, document, &sizes)?;
            crate::draft::derive_table(document, &at_sizes, &sizes.sample).map_err(fail)?
        };
        match fields.get("convention").and_then(Value::as_text) {
            Some(name) => sc_grading::Convention::named(name)
                .and_then(|convention| table.in_convention(convention))
                .map_err(|error| fail(error.to_string())),
            None => Ok(table),
        }
    }

    // Every size of a table, moved from the sample and rebuilt, with the recipe left out of it. Files are only written when nothing a table promised has been broken.
    fn grade(&mut self, step: usize, fields: &BTreeMap<String, Value>) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "grade", message);
        let document = self
            .document()
            .ok_or_else(|| fail("there is nothing drafted to grade yet".to_owned()))?;
        let table = self.grade_table(step, &document, fields)?;
        let drawn_at = document.get("size").and_then(Value::as_text).unwrap_or("");
        if table.sample() != drawn_at {
            return Err(fail(format!(
                "the table's sample size is {}, and this draft is drawn at {drawn_at}",
                table.sample()
            )));
        }
        let known = self.lengths_for(step, Some(&document))?;
        let mut reports = Vec::with_capacity(table.sizes().len());
        let mut family = Vec::with_capacity(table.sizes().len());
        let mut conflicts = Vec::new();
        // What the recipe itself draws at each size, when the draft says what its sizes are.
        let by_recipe: BTreeMap<String, BTreeMap<String, i64>> =
            match crate::family::sizes_of(&document) {
                Ok(sizes) => self
                    .lengths_at_sizes(step, &document, &sizes)?
                    .into_iter()
                    .collect(),
                Err(_) => BTreeMap::new(),
            };
        let (mut quantities, mut findings, mut not_compared) = (Vec::new(), Vec::new(), Vec::new());
        for size in table.sizes() {
            let at_size = |message: String| fail(format!("size {size}: {message}"));
            let rebuilt = crate::draft::reconstruct(
                &known,
                &document,
                &table,
                size,
                (&self.design_text, &self.revision_text()),
            )
            .map_err(at_size)?;
            let walked = sc_artifacts::walk(&rebuilt.pattern)
                .map_err(|error| at_size(format!("the seam {}: {}", error.seam, error.message)))?;
            if let Some(at_size_lengths) = by_recipe.get(size) {
                let mut described = document.clone();
                described.insert("size".to_owned(), Value::Text(size.clone()));
                let regenerated = crate::draft::draft(
                    at_size_lengths,
                    &described,
                    &self.design_text,
                    &self.revision_text(),
                )
                .map_err(at_size)?;
                let (all, found) = crate::draft::compare_size(
                    size,
                    &table,
                    &document,
                    (&known, at_size_lengths),
                    &regenerated,
                    &rebuilt,
                    crate::draft::NUMERICAL,
                )
                .map_err(at_size)?;
                quantities.extend(all);
                findings.extend(found);
            } else {
                not_compared.push(Value::Text(size.clone()));
            }
            let text = sc_artifacts::to_canonical_text(&rebuilt.pattern);
            let words =
                |listed: &[String]| Value::List(listed.iter().cloned().map(Value::Text).collect());
            let mut report = BTreeMap::new();
            report.insert("size".to_owned(), Value::Text(size.clone()));
            report.insert(
                "canonical_sha256".to_owned(),
                Value::Text(sc_store::sha256_hex(text.as_bytes())),
            );
            report.insert(
                "walk".to_owned(),
                Value::List(
                    walked
                        .iter()
                        .map(sc_artifacts::WalkReport::to_value)
                        .collect(),
                ),
            );
            report.insert("unmentioned".to_owned(), words(&rebuilt.unmentioned));
            report.insert("unmatched".to_owned(), words(&rebuilt.unmatched));
            report.insert("notes".to_owned(), words(&rebuilt.notes));
            report.insert(
                "conflicts".to_owned(),
                Value::List(
                    rebuilt
                        .conflicts
                        .iter()
                        .map(crate::draft::Conflict::to_value)
                        .collect(),
                ),
            );
            conflicts.extend(rebuilt.conflicts.iter().cloned());
            reports.push(Value::Object(report));
            let mut member = BTreeMap::new();
            member.insert("size".to_owned(), Value::Text(size.clone()));
            member.insert(
                "pattern".to_owned(),
                sc_store::parse(&text).map_err(|error| at_size(error.to_string()))?,
            );
            family.push(Value::Object(member));
        }
        let asked_for_files = ["path", "rules_path", "rul"]
            .iter()
            .any(|name| fields.contains_key(*name));
        let count = |listed: &[Value]| Value::Int(i64::try_from(listed.len()).unwrap_or(i64::MAX));
        let mut equivalence = BTreeMap::new();
        equivalence.insert(
            "provenance".to_owned(),
            Value::Text(table.provenance().token().to_owned()),
        );
        equivalence.insert("classes".to_owned(), crate::draft::classes());
        equivalence.insert("compared".to_owned(), count(&quantities));
        equivalence.insert("not_compared".to_owned(), Value::List(not_compared));
        equivalence.insert("findings".to_owned(), Value::List(findings));
        if asked_for_files {
            if let Some(first) = conflicts.first() {
                return Err(fail(format!(
                    "the contour {} is declared fixed and comes to {} at size {}, against {} at the sample, so these files are not written",
                    first.contour, first.here, first.size, first.at_sample
                )));
            }
        }
        let write = |project: &mut sc_store::Project, target: &str, bytes: String| {
            let path = sc_store::FilePath::new(target).map_err(|error| fail(error.to_string()))?;
            project
                .set_file(path, bytes)
                .map_err(|error| fail(error.to_string()))
        };
        if let Some(target) = fields.get("path").and_then(Value::as_text) {
            let mut whole = BTreeMap::new();
            whole.insert("sample".to_owned(), Value::Text(table.sample().to_owned()));
            whole.insert("sizes".to_owned(), Value::List(family));
            write(&mut self.base, target, Value::Object(whole).to_text())?;
        }
        // Written whether or not anything differs, since a report that only appears when something is wrong says nothing when it is absent.
        if let Some(target) = fields.get("equivalence_path").and_then(Value::as_text) {
            let mut whole = equivalence.clone();
            whole.insert("quantities".to_owned(), Value::List(quantities));
            write(&mut self.base, target, Value::Object(whole).to_text())?;
        }
        if let Some(target) = fields.get("rules_path").and_then(Value::as_text) {
            write(&mut self.base, target, table.to_value().to_text())?;
        }
        if let Some(target) = fields.get("rul").and_then(Value::as_text) {
            let artifact = sc_grading::to_rul(&table).map_err(|error| fail(error.to_string()))?;
            write(&mut self.base, target, artifact.text)?;
            write(
                &mut self.base,
                &format!("{target}.record.json"),
                artifact.record,
            )?;
        }
        let mut result = BTreeMap::new();
        result.insert("sample".to_owned(), Value::Text(table.sample().to_owned()));
        result.insert(
            "convention".to_owned(),
            Value::Text(table.convention().token().to_owned()),
        );
        result.insert(
            "provenance".to_owned(),
            Value::Text(table.provenance().token().to_owned()),
        );
        result.insert(
            "rules".to_owned(),
            Value::Int(i64::try_from(table.rules().len()).unwrap_or(i64::MAX)),
        );
        result.insert("sizes".to_owned(), Value::List(reports));
        result.insert("equivalence".to_owned(), Value::Object(equivalence));
        Ok(Value::Object(result))
    }

    // The smallest and largest sizes, rebuilt from a written file, and each checked with its own diagnostic rather than a clamped value. What reads the file here is this project's own engine, and the report says so.
    fn check_extremes(&mut self, step: usize) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "check_extremes", message);
        let document = self
            .document()
            .ok_or_else(|| fail("there is nothing drafted to check yet".to_owned()))?;
        let sizes = crate::family::sizes_of(&document).map_err(fail)?;
        let at_sizes = self.lengths_at_sizes(step, &document, &sizes)?;
        let table =
            crate::draft::derive_table(&document, &at_sizes, &sizes.sample).map_err(fail)?;
        let written = sc_grading::to_rul(&table).map_err(|error| fail(error.to_string()))?;
        let file = sc_grading::from_rul(&written.text).map_err(|error| fail(error.to_string()))?;
        let received = table
            .as_read_from(&file)
            .map_err(|error| fail(error.to_string()))?;
        let known = self.lengths_for(step, Some(&document))?;
        let mut checks = Vec::new();
        let (mut quantities, mut findings) = (Vec::new(), Vec::new());
        let ends: Vec<&(String, BTreeMap<String, i64>)> = match (at_sizes.first(), at_sizes.last())
        {
            (Some(first), Some(last)) if first.0 != last.0 => vec![first, last],
            (Some(only), _) => vec![only],
            _ => Vec::new(),
        };
        for (size, at_size) in ends {
            let at_this = |message: String| fail(format!("size {size}: {message}"));
            let rebuilt = match crate::draft::reconstruct(
                &known,
                &document,
                &received,
                size,
                (&self.design_text, &self.revision_text()),
            ) {
                Ok(rebuilt) => rebuilt,
                // It could not be drawn at all, which is the first thing wrong with it, so nothing later can be asked of it.
                Err(message) => {
                    checks.push(crate::draft::Check {
                        check: "offset_validity",
                        size: size.clone(),
                        class: "geometric",
                        verdict: "fails",
                        detail: message,
                    });
                    continue;
                }
            };
            checks.extend(
                crate::draft::checks_on(size, &document, at_size, &rebuilt.pattern)
                    .map_err(at_this)?,
            );
            let mut described = document.clone();
            described.insert("size".to_owned(), Value::Text(size.clone()));
            let regenerated = crate::draft::draft(
                at_size,
                &described,
                &self.design_text,
                &self.revision_text(),
            )
            .map_err(at_this)?;
            let (all, found) = crate::draft::compare_size(
                size,
                &received,
                &document,
                (&known, at_size),
                &regenerated,
                &rebuilt,
                crate::draft::FORMAT,
            )
            .map_err(at_this)?;
            quantities.extend(all);
            findings.extend(found);
        }
        let mut equivalence = BTreeMap::new();
        equivalence.insert("provenance".to_owned(), Value::Text("received".to_owned()));
        equivalence.insert("classes".to_owned(), crate::draft::classes());
        equivalence.insert(
            "compared".to_owned(),
            Value::Int(i64::try_from(quantities.len()).unwrap_or(i64::MAX)),
        );
        equivalence.insert("findings".to_owned(), Value::List(findings));
        let mut result = BTreeMap::new();
        result.insert(
            "through".to_owned(),
            Value::Text("a .rul file, read back by this project's own engine".to_owned()),
        );
        result.insert(
            "checks".to_owned(),
            Value::List(checks.iter().map(crate::draft::Check::to_value).collect()),
        );
        result.insert("equivalence".to_owned(), Value::Object(equivalence));
        Ok(Value::Object(result))
    }

    // One of the three ways grading travels, each its own file with its own record. A rule moves a net corner, so these put the net line on the layer a receiver reads, and say so.
    #[allow(clippy::too_many_lines)]
    fn export_graded(
        &mut self,
        step: usize,
        fields: &BTreeMap<String, Value>,
    ) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "export_graded", message);
        let document = self
            .document()
            .ok_or_else(|| fail("there is nothing drafted to export yet".to_owned()))?;
        let mode = match fields.get("mode").and_then(Value::as_text) {
            Some("base_plus_rules") => sc_artifacts::GradingMode::BasePlusRules,
            Some("embedded_rules") => sc_artifacts::GradingMode::EmbeddedRules,
            Some("all_contours") => sc_artifacts::GradingMode::AllContours,
            Some(other) => {
                return Err(fail(format!(
                    "no grading mode is named {other}: it is base_plus_rules, embedded_rules or all_contours, and there is no universal one"
                )))
            }
            None => {
                return Err(fail(
                    "export_graded names its mode: base_plus_rules, embedded_rules or all_contours"
                        .to_owned(),
                ))
            }
        };
        match fields.get("semantics").and_then(Value::as_text) {
            Some("sew_as_1") => {}
            Some("cut_as_1") => {
                return Err(fail(
                    "a rule moves a net corner, so a graded file needs the net line on the layer a receiver reads, which is sew_as_1. Allowances are generated downstream, and this says so in its record".to_owned(),
                ))
            }
            _ => {
                return Err(fail(
                    "a graded export names its semantics, and only sew_as_1 exists".to_owned(),
                ))
            }
        }
        let target = fields
            .get("path")
            .and_then(Value::as_text)
            .ok_or_else(|| fail("export_graded needs a path, without its extension".to_owned()))?
            .to_owned();
        let sizes = crate::family::sizes_of(&document).map_err(fail)?;
        let at_sizes = self.lengths_at_sizes(step, &document, &sizes)?;
        let table =
            crate::draft::derive_table(&document, &at_sizes, &sizes.sample).map_err(fail)?;
        let known = self.lengths_for(step, Some(&document))?;
        let drawn =
            crate::draft::draft(&known, &document, &self.design_text, &self.revision_text())
                .map_err(fail)?;
        if !drawn.repairs.is_empty() {
            let open: Vec<&str> = drawn
                .repairs
                .iter()
                .map(|repair| repair.carrier.as_str())
                .collect();
            return Err(fail(format!(
                "this drawing has references that need repair before it can be released: {}",
                open.join("; ")
            )));
        }
        for size in table.sizes() {
            let rebuilt = crate::draft::reconstruct(
                &known,
                &document,
                &table,
                size,
                (&self.design_text, &self.revision_text()),
            )
            .map_err(|message| fail(format!("size {size}: {message}")))?;
            if let Some(first) = rebuilt.conflicts.first() {
                return Err(fail(format!(
                    "the contour {} is declared fixed and comes to {} at size {}, against {} at the sample, so these files are not written",
                    first.contour, first.here, first.size, first.at_sample
                )));
            }
        }
        let flattening = sc_artifacts::Flattening {
            bound: 100,
            achieved: crate::draft::TOLERANCE + 1,
        };
        let (sample, paired) = crate::draft::without_mirrors(&drawn, &document).map_err(fail)?;
        let points = crate::draft::rule_points(&known, &document, &table).map_err(fail)?;
        let artifact = match mode {
            sc_artifacts::GradingMode::BasePlusRules => {
                sc_artifacts::write_base_plus_rules(&sample, &paired, &points, flattening)
            }
            sc_artifacts::GradingMode::EmbeddedRules => sc_artifacts::write_embedded_rules(
                &sample,
                &paired,
                &points,
                (table.sample(), table.name()),
                flattening,
            ),
            sc_artifacts::GradingMode::AllContours => {
                let counts = crate::draft::curve_counts(&document, &at_sizes).map_err(fail)?;
                let mut nests = Vec::with_capacity(at_sizes.len());
                for (size, lengths) in &at_sizes {
                    let mut described = document.clone();
                    described.insert("size".to_owned(), Value::Text(size.clone()));
                    let drawn_here = crate::draft::draft(
                        lengths,
                        &described,
                        &self.design_text,
                        &self.revision_text(),
                    )
                    .map_err(|message| fail(format!("size {size}: {message}")))?;
                    nests.push((
                        size.clone(),
                        crate::draft::nest_at(lengths, &document, &drawn_here, &counts, &table)
                            .map_err(|message| fail(format!("size {size}: {message}")))?,
                    ));
                }
                sc_artifacts::write_all_contours(&sample, &paired, &nests, flattening)
            }
        }
        .map_err(|error| fail(error.to_string()))?;
        let write = |project: &mut sc_store::Project, name: String, bytes: String| {
            let path = sc_store::FilePath::new(&name).map_err(|error| fail(error.to_string()))?;
            project
                .set_file(path, bytes)
                .map_err(|error| fail(error.to_string()))
        };
        let mut written = vec![format!("{target}.dxf"), format!("{target}.dxf.record.json")];
        let digest = sc_store::sha256_hex(artifact.text.as_bytes());
        write(&mut self.base, format!("{target}.dxf"), artifact.text)?;
        write(
            &mut self.base,
            format!("{target}.dxf.record.json"),
            artifact.record,
        )?;
        // All contours carries no rules, so it has no table to go with it.
        if mode != sc_artifacts::GradingMode::AllContours {
            let rules = sc_grading::to_rul(&table).map_err(|error| fail(error.to_string()))?;
            written.push(format!("{target}.rul"));
            written.push(format!("{target}.rul.record.json"));
            write(&mut self.base, format!("{target}.rul"), rules.text)?;
            write(
                &mut self.base,
                format!("{target}.rul.record.json"),
                rules.record,
            )?;
        }
        let mut result = BTreeMap::new();
        result.insert("mode".to_owned(), Value::Text(mode.token().to_owned()));
        result.insert("target".to_owned(), Value::Text(mode.target().to_owned()));
        result.insert("dxf_sha256".to_owned(), Value::Text(digest));
        result.insert(
            "files".to_owned(),
            Value::List(written.into_iter().map(Value::Text).collect()),
        );
        Ok(Value::Object(result))
    }

    // A person or an agent says where an orphaned reference goes, or that it is gone. Nothing else ever does.
    fn repair_reference(
        &mut self,
        step: usize,
        fields: &BTreeMap<String, Value>,
    ) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "repair_reference", message);
        let mut document = self
            .document()
            .ok_or_else(|| fail("there is nothing drafted to repair yet".to_owned()))?;
        let retire = matches!(fields.get("retire"), Some(Value::Bool(true)));
        if let Some(seam) = fields.get("seam").and_then(Value::as_text) {
            let Some(Value::List(seams)) = document.get_mut("seams") else {
                return Err(fail(format!("no seam is named {seam}")));
            };
            let position = seams
                .iter()
                .position(|item| {
                    item.as_object()
                        .and_then(|item| item.get("name"))
                        .and_then(Value::as_text)
                        == Some(seam)
                })
                .ok_or_else(|| fail(format!("no seam is named {seam}")))?;
            if retire {
                seams.remove(position);
            } else {
                let side = fields.get("side").and_then(Value::as_text);
                let end = fields.get("end").and_then(Value::as_text);
                let (Some(side @ ("a" | "b")), Some(end @ ("from" | "to"))) = (side, end) else {
                    return Err(fail(
                        "repairing a seam says which side, a or b, and which end, from or to"
                            .to_owned(),
                    ));
                };
                let target = seams
                    .get_mut(position)
                    .and_then(object_of)
                    .and_then(|item| item.get_mut(side))
                    .and_then(object_of)
                    .ok_or_else(|| fail(format!("{seam} has no side {side}")))?;
                let held = match target.get_mut(end) {
                    Some(Value::Object(held)) => held,
                    Some(other) => {
                        *other = Value::Object(BTreeMap::new());
                        object_of(other).ok_or_else(|| fail("no such end".to_owned()))?
                    }
                    None => return Err(fail(format!("{seam} has no end {end}"))),
                };
                repoint(held, fields);
            }
        } else {
            let name = fields
                .get("piece")
                .and_then(Value::as_text)
                .ok_or_else(|| {
                    fail("a repair names a seam, or a piece and what is on it".to_owned())
                })?;
            let (list, carrier) = match (
                fields.get("notch").and_then(Value::as_text),
                fields.get("grade_point").and_then(Value::as_text),
            ) {
                (Some(notch), None) => ("notches", notch),
                (None, Some(point)) => ("grade_points", point),
                _ => {
                    return Err(fail(
                        "a repair on a piece names one notch or one grade_point".to_owned(),
                    ))
                }
            };
            let piece = piece_in(&mut document, name)
                .ok_or_else(|| fail(format!("no piece is named {name}")))?;
            let Some(Value::List(items)) = piece.get_mut(list) else {
                return Err(fail(format!("{name} has nothing named {carrier}")));
            };
            let position = items
                .iter()
                .position(|item| {
                    item.as_object()
                        .and_then(|item| item.get("name"))
                        .and_then(Value::as_text)
                        == Some(carrier)
                })
                .ok_or_else(|| fail(format!("{name} has nothing named {carrier}")))?;
            if retire {
                items.remove(position);
            } else {
                let held = items
                    .get_mut(position)
                    .and_then(object_of)
                    .ok_or_else(|| fail(format!("{name} has nothing named {carrier}")))?;
                repoint(held, fields);
            }
        }
        self.commit_edited(step, "repair_reference", document)
    }

    // What a change would do, computed on a copy, leaving the Design, its revision and its history exactly as they are.
    fn preview(&self, step: usize, fields: &BTreeMap<String, Value>) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "preview", message);
        let token = fields
            .get("token")
            .and_then(Value::as_text)
            .ok_or_else(|| fail("preview needs a token".to_owned()))?;
        let value = fields
            .get("um")
            .and_then(Value::as_int)
            .ok_or_else(|| fail("preview needs um".to_owned()))?;
        let value = Length::from_micrometres(value)
            .map_err(|error| Failure::at(step, "invalid_value", error.to_string()))?;
        let mut inputs = self.inputs.clone();
        let input = inputs
            .iter_mut()
            .find(|input| input.token == token)
            .ok_or_else(|| {
                Failure::at(step, "unknown_input", format!("no input is named {token}"))
            })?;
        input.value = value;
        let document = self.document();
        let inputs = self.solved(step, &inputs, document.as_ref())?;
        let garment =
            build(self.design, self.revision(), &inputs, &self.recipe, "").map_err(fail)?;
        let before = self.evaluate(step)?;
        let after = evaluate_design(garment.inputs(), step)?;
        let (Value::List(old), Value::List(new)) = (&before, &after) else {
            return Err(fail("a recipe evaluates to a list".to_owned()));
        };
        let changed: Vec<Value> = new
            .iter()
            .filter(|binding| !old.contains(binding))
            .cloned()
            .collect();
        let mut result = BTreeMap::new();
        result.insert("changed".to_owned(), Value::List(changed));
        result.insert("committed".to_owned(), Value::Bool(false));
        Ok(Value::Object(result))
    }

    fn evaluate(&self, step: usize) -> Result<Value, Failure> {
        let document = self.document();
        let inputs = self.solved(step, &self.inputs, document.as_ref())?;
        self.evaluated(step, &inputs)
    }

    // The bus's own Design for the inputs it holds, and any other values through a copy of it that nothing else sees.
    fn evaluated(&self, step: usize, inputs: &[Input]) -> Result<Value, Failure> {
        if inputs == self.inputs {
            return evaluate_design(self.bus.state().inputs(), step);
        }
        self.tried(step, inputs, &self.recipe)
    }

    fn tried(&self, step: usize, inputs: &[Input], recipe: &str) -> Result<Value, Failure> {
        let garment = build(self.design, self.revision(), inputs, recipe, "")
            .map_err(|message| Failure::at(step, "recipe", message))?;
        evaluate_design(garment.inputs(), step)
    }

    // What the inputs a description's fits vary come to. Solved again whenever it is needed, because it depends on nothing but the inputs, the recipe and the description, and the Design holds those.
    fn solved(
        &self,
        step: usize,
        from: &[Input],
        description: Option<&BTreeMap<String, Value>>,
    ) -> Result<Vec<Input>, Failure> {
        let mut inputs = from.to_vec();
        let Some(description) = description else {
            return Ok(inputs);
        };
        let Some(listed) = description.get("fits").and_then(Value::as_list) else {
            return Ok(inputs);
        };
        // Few answers are in use at once: the design as it is, and what a preview would make it.
        const KEPT: usize = 8;
        let key = format!(
            "{from:?}|{}|{}",
            self.recipe,
            Value::Object(description.clone()).to_text()
        );
        if let Some(answer) = self.fits_solved.borrow().get(&key) {
            return Ok(answer.clone());
        }
        let fail = |message: String| Failure::at(step, "fit", message);
        // A closure has to hold at the answer. It need not hold at every value tried on the way there.
        let recipe = without_asserts(&self.recipe);
        for item in listed {
            let item = item
                .as_object()
                .ok_or_else(|| fail("a fit must be an object".to_owned()))?;
            let text = |name: &str| {
                item.get(name)
                    .and_then(Value::as_text)
                    .ok_or_else(|| fail(format!("a fit names its {name}")))
            };
            let (vary, seam) = (text("vary")?, text("seam")?);
            let at = inputs
                .iter()
                .position(|input| input.token == vary)
                .ok_or_else(|| fail(format!("a fit varies {vary}, and no input has that name")))?;
            let known = known_lengths(&inputs, &self.tried(step, &inputs, &recipe)?);
            let Some([low, high]) = item.get("between").and_then(Value::as_list) else {
                return Err(fail(format!(
                    "a fit of {vary} says between which two lengths"
                )));
            };
            let bound = |value: &Value| {
                let name = value
                    .as_text()
                    .ok_or_else(|| fail(format!("a fit of {vary} names its two lengths")))?;
                known
                    .get(name)
                    .copied()
                    .ok_or_else(|| fail(format!("no length is named {name}")))
            };
            let range = (bound(low)?, bound(high)?);
            let base = inputs.clone();
            let mut gap = |value: i64| -> Result<Option<i64>, String> {
                let mut trial = base.clone();
                if let Some(input) = trial.get_mut(at) {
                    input.value =
                        Length::from_micrometres(value).map_err(|error| error.to_string())?;
                }
                let evaluated = self
                    .tried(step, &trial, &recipe)
                    .map_err(|failure| failure.message)?;
                crate::draft::seam_gap(&known_lengths(&trial, &evaluated), description, seam)
            };
            let found = crate::draft::solve_fit((vary, seam), range, &mut gap).map_err(fail)?;
            if let (Some(value), Some(input)) = (found, inputs.get_mut(at)) {
                input.value =
                    Length::from_micrometres(value).map_err(|error| fail(error.to_string()))?;
            }
        }
        let mut kept = self.fits_solved.borrow_mut();
        if kept.len() >= KEPT {
            kept.clear();
        }
        kept.insert(key, inputs.clone());
        Ok(inputs)
    }

    // Every length a session can name, as the Design is drawn now.
    fn lengths(&self, step: usize) -> Result<BTreeMap<String, i64>, Failure> {
        self.lengths_for(step, self.document().as_ref())
    }

    // Every length a session can name for this description: the inputs, the recipe's bindings, and `zero`.
    fn lengths_for(
        &self,
        step: usize,
        description: Option<&BTreeMap<String, Value>>,
    ) -> Result<BTreeMap<String, i64>, Failure> {
        let inputs = self.solved(step, &self.inputs, description)?;
        Ok(known_lengths(&inputs, &self.evaluated(step, &inputs)?))
    }

    // How to draw it, held by the Design, so an edit to it has a revision, an actor and an undo. The drawing itself is derived and is never saved.
    fn document(&self) -> Option<BTreeMap<String, Value>> {
        let text = &self.bus.state().definition().drafting;
        if text.is_empty() {
            return None;
        }
        sc_store::parse(text).ok()?.as_object().cloned()
    }

    // The first drawing starts the design's own history: it is part of creating the design, so it takes no revision, and what came before it cannot be undone past it.
    fn install_document(&mut self, step: usize, text: &str) -> Result<(), Failure> {
        let fail = |message: String| Failure::at(step, "draft", message);
        let maximum = self
            .bus
            .maximum_issued_revision()
            .ok_or_else(|| fail("this bus has no journal".to_owned()))?
            .clone();
        let sequence = self
            .bus
            .audit_sequence()
            .ok_or_else(|| fail("this bus has no journal".to_owned()))?
            .clone();
        let garment = build(
            self.design,
            self.revision(),
            &self.inputs,
            &self.recipe,
            text,
        )
        .map_err(fail)?;
        let authorities = self.authorities.clone();
        self.bus = new_bus(
            garment,
            self.design,
            self.actor,
            self.identities,
            &authorities,
        )
        .map_err(fail)?;
        self.bus
            .resume_issuance(maximum, sequence)
            .map_err(|error| fail(error.to_string()))?;
        self.undo.clear();
        self.redo.clear();
        Ok(())
    }

    // A later edit to how the garment is drawn is a command like any other.
    fn commit_document(&mut self, step: usize, text: String) -> Result<(), Failure> {
        let group_id = CommandGroupId::from_entity_id(self.fresh_id());
        let revision = self.revision();
        let context = self.context(group_id, revision);
        let payload = MutationPayload::ChangeDrafting(ChangeDrafting::new(text));
        let group =
            CommandMutationGroup::new(group_id, vec![CommandMutation::new(context, payload)])
                .map_err(|error| Failure::at(step, "refused", error.to_string()))?;
        self.bus
            .commit_group(&group)
            .map_err(|error| Failure::at(step, "refused", error.to_string()))?;
        self.undo.push(group);
        self.redo.clear();
        self.pattern = None;
        Ok(())
    }

    // A step naming its pieces draws those, and they become how the design is drawn. One naming none draws what the design holds.
    fn draft(&mut self, step: usize, fields: &BTreeMap<String, Value>) -> Result<Value, Failure> {
        let fail = |message: String| Failure::at(step, "draft", message);
        let description = if fields.contains_key("pieces") {
            let mut own = fields.clone();
            own.remove("do");
            own
        } else {
            self.document().ok_or_else(|| {
                fail("a draft names its pieces, and this project has saved none".to_owned())
            })?
        };
        let known = self.lengths_for(step, Some(&description))?;
        // Refused here, before anything changes, if it cannot be drawn.
        crate::draft::draft(
            &known,
            &description,
            &self.design_text,
            &self.revision_text(),
        )
        .map_err(fail)?;
        let text = Value::Object(description.clone()).to_text();
        if self.bus.state().definition().drafting.is_empty() {
            self.install_document(step, &text)?;
        } else if self.bus.state().definition().drafting != text {
            self.commit_document(step, text)?;
        }
        let pattern = crate::draft::draft(
            &known,
            &description,
            &self.design_text,
            &self.revision_text(),
        )
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
        if let Some(drawn) = &self.pattern {
            if !drawn.fits.is_empty() {
                record.insert(
                    "fits".to_owned(),
                    Value::List(drawn.fits.iter().map(sc_artifacts::Fit::to_value).collect()),
                );
            }
        }
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
        // The canonical form is how a drawing is inspected, and it lists what is open. A cutting file is a release, and a notch nobody placed would be missing from it without a word.
        if format != "canonical" && !pattern.repairs.is_empty() {
            let open: Vec<&str> = pattern
                .repairs
                .iter()
                .map(|repair| repair.carrier.as_str())
                .collect();
            return Err(fail(format!(
                "this drawing has references that need repair before it can be released: {}",
                open.join("; ")
            )));
        }
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
        let drafting = &self.bus.state().definition().drafting;
        if !drafting.is_empty() {
            project
                .set_file(path("draft.json")?, drafting.clone())
                .map_err(|error| error.to_string())?;
        }
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
        // A project saved before anything was drafted has no description, and a draft then says so.
        let drafting = match saved.project.file(&path("draft.json").map_err(fail)?) {
            Some(bytes) => {
                let text = String::from_utf8_lossy(bytes).into_owned();
                let parsed = sc_store::parse(&text).map_err(|error| fail(error.to_string()))?;
                if parsed.as_object().is_none() {
                    return Err(fail("draft.json must hold an object".to_owned()));
                }
                text
            }
            None => String::new(),
        };
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
        let garment =
            build(self.design, revision, &self.inputs, &self.recipe, &drafting).map_err(fail)?;
        let authorities = self.authorities.clone();
        self.bus = new_bus(
            garment,
            self.design,
            self.actor,
            self.identities,
            &authorities,
        )
        .map_err(fail)?;
        self.bus
            .resume_issuance(maximum, CommandAuditSequence::from_revision(sequence))
            .map_err(|error| fail(error.to_string()))?;
        self.undo.clear();
        self.redo.clear();
        Ok(Value::Text(self.revision_text()))
    }
}

// A size, and every length a session can name there.
type SizedLengths = (String, BTreeMap<String, i64>);

// The recipe with its asserts taken out. A statement ends where the next one starts.
fn without_asserts(recipe: &str) -> String {
    let mut out = String::with_capacity(recipe.len());
    let mut skipping = false;
    for line in recipe.lines() {
        let start = line.trim_start();
        if start.starts_with("assert ") {
            skipping = true;
        } else if start.starts_with("let ") {
            skipping = false;
        }
        if !skipping {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn known_lengths(inputs: &[Input], evaluated: &Value) -> BTreeMap<String, i64> {
    let mut known: BTreeMap<String, i64> = inputs
        .iter()
        .map(|input| (input.token.clone(), input.value.as_micrometres()))
        .collect();
    known.insert("zero".to_owned(), 0);
    if let Value::List(bindings) = evaluated {
        for binding in bindings {
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
    known
}

// Runs the recipe against one Design's inputs, whichever Design that is.
fn evaluate_design(root: &DesignInputs<DesignSources>, step: usize) -> Result<Value, Failure> {
    let fail = |code, error: String| Failure::at(step, code, error);
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

fn limits() -> DesignResourceLimits {
    DesignResourceLimits::new(NonZeroUsize::MAX, NonZeroUsize::MAX, NonZeroU64::MAX)
}

fn new_bus(
    garment: sc_core::design::DesignGarment<DesignSources>,
    design: EntityId,
    actor: CommandActorClaim,
    identities: Identities,
    authorities: &[CommandAuthority],
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
        authorities.to_vec(),
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
pub const STEPS: [&str; 23] = [
    "evaluate",
    "preview",
    "closures",
    "walk",
    "plant_balance_notch",
    "true_seam",
    "edit_edge",
    "references",
    "repair_reference",
    "regenerate",
    "grade",
    "check_extremes",
    "export_graded",
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

/// Who is driving a session, and what they may do. `replay` is a local human holding what a recorded session needs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Caller {
    /// Whether this is an agent acting for its human principal, which the bus records on every command.
    pub agent: bool,
    /// What this caller may do. An agent can never hold `Approve`: the core refuses to build that grant.
    pub authorities: Vec<CommandAuthority>,
}

impl Caller {
    /// A local human, as the command line runs: everything a session's steps can need.
    #[must_use]
    pub fn local_human() -> Self {
        Self {
            agent: false,
            authorities: vec![
                CommandAuthority::Inspect,
                CommandAuthority::Propose,
                CommandAuthority::Commit,
                CommandAuthority::Generate,
            ],
        }
    }
}

fn object_of(value: &mut Value) -> Option<&mut BTreeMap<String, Value>> {
    match value {
        Value::Object(fields) => Some(fields),
        _ => None,
    }
}

fn piece_in<'a>(
    document: &'a mut BTreeMap<String, Value>,
    name: &str,
) -> Option<&'a mut BTreeMap<String, Value>> {
    let Some(Value::List(pieces)) = document.get_mut("pieces") else {
        return None;
    };
    pieces
        .iter_mut()
        .filter_map(object_of)
        .find(|piece| piece.get("name").and_then(Value::as_text) == Some(name))
}

// A choice at a split keeps what the reference holds. Anything else replaces it with what the caller stated.
fn repoint(held: &mut BTreeMap<String, Value>, fields: &BTreeMap<String, Value>) {
    if let Some(choose) = fields.get("choose") {
        held.insert("choose".to_owned(), choose.clone());
        return;
    }
    held.remove("choose");
    for name in ["at", "along", "before_end"] {
        held.remove(name);
    }
    for name in ["edge", "at", "along", "before_end"] {
        if let Some(value) = fields.get(name) {
            held.insert(name.to_owned(), value.clone());
        }
    }
}

// What each step needs. `change`, `undo` and `redo` are also refused by the bus itself, whose grant is built from the same list.
fn required(step: &str) -> Option<CommandAuthority> {
    match step {
        // Loading writes nothing. It discards unsaved work, and only a caller who may commit can have made any.
        "evaluate" | "solve" | "reopen" | "closures" | "walk" | "references" => {
            Some(CommandAuthority::Inspect)
        }
        "preview" => Some(CommandAuthority::Propose),
        "save" => Some(CommandAuthority::Commit),
        "draft" | "export_pattern" | "render" | "export" | "regenerate" | "grade"
        | "check_extremes" => Some(CommandAuthority::Generate),
        _ => None,
    }
}

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
        Self::open_as(session, repository, &Caller::local_human())
    }

    /// Opens a session for this caller, whose authority every later step is held to.
    ///
    /// # Errors
    /// The session is malformed, the project cannot be read, or the caller asks for what it cannot hold.
    pub fn open_as(session: &Value, repository: R, caller: &Caller) -> Result<Self, Failure> {
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
        let actor = if caller.agent {
            CommandActorClaim::Agent {
                identity: identities.nth(22),
                principal: identities.nth(21),
            }
        } else {
            CommandActorClaim::Human(identities.nth(21))
        };
        let garment = build(design, DesignRevision::initial(), &inputs, &recipe, "")
            .map_err(Failure::session)?;
        let bus = new_bus(garment, design, actor, identities, &caller.authorities)
            .map_err(Failure::session)?;
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
            fits_solved: RefCell::new(BTreeMap::new()),
            authorities: caller.authorities.clone(),
            trace: Vec::new(),
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
        let verb = item
            .as_object()
            .and_then(|fields| fields.get("do"))
            .and_then(Value::as_text)
            .unwrap_or("");
        if let Some(needed) = required(verb) {
            if !state.authorities.contains(&needed) {
                return Err(Failure::at(
                    step,
                    "unauthorized",
                    format!("{verb} needs the {} authority", needed.token()),
                ));
            }
        }
        let item = item
            .as_object()
            .ok_or_else(|| Failure::at(step, "session", "a step must be an object"))?;
        let what = item
            .get("do")
            .and_then(Value::as_text)
            .ok_or_else(|| Failure::at(step, "session", "a step needs a do field"))?;
        let result = match what {
            "evaluate" => state.evaluate(step)?,
            "preview" => state.preview(step, item)?,
            "closures" => state.closures(step)?,
            "walk" => state.walk(step)?,
            "plant_balance_notch" => state.plant_balance_notch(step, item)?,
            "true_seam" => state.true_seam(step, item)?,
            "edit_edge" => state.edit_edge(step, item)?,
            "references" => state.references(step)?,
            "regenerate" => state.regenerate(step, item)?,
            "grade" => state.grade(step, item)?,
            "check_extremes" => state.check_extremes(step)?,
            "export_graded" => state.export_graded(step, item)?,
            "repair_reference" => state.repair_reference(step, item)?,
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
        if matches!(
            what,
            "change" | "undo" | "redo" | "save" | "reopen" | "draft" | "export_pattern"
        ) {
            self.state.record_trace(what, step);
        }
        Ok(Value::Object(record))
    }

    /// Every step that changed anything so far, and who took it.
    #[must_use]
    pub fn trace(&self) -> Value {
        Value::List(self.state.trace.clone())
    }

    /// Whether the project holds a file at this path, saved or waiting for the next save.
    #[must_use]
    pub fn has_file(&self, path: &str) -> bool {
        sc_store::FilePath::new(path).is_ok_and(|path| self.state.base.file(&path).is_some())
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
        let garment =
            build(design, DesignRevision::from_counter(5), &inputs, "", "").expect("builds");
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
        let mut bus = new_bus(
            garment,
            design,
            actor,
            identities,
            &[CommandAuthority::Inspect, CommandAuthority::Commit],
        )
        .expect("a bus");
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
