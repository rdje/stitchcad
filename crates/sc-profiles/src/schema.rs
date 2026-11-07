//! The Factory Profile file, schema 2 (roadmap §8): every refusal names its field, and all of them are reported at once.
use crate::exact::{decimal_text, parse_decimal};
use crate::registry::{spec_of, Kind, ParameterSpec, ACCEPTS, NOTCH_CODES, PEN_FUNCTIONS};
use core::fmt;
use sc_constraints::Rational;
use sc_store::{sha256_hex, Value};
use std::collections::{BTreeMap, BTreeSet};

/// The schema this crate reads. A content change never changes it.
pub const SCHEMA_VERSION: &str = "2.0";

const TOP_FIELDS: [&str; 8] = [
    "accepts",
    "composition",
    "constraints_ast",
    "content_version",
    "parameters",
    "profile_id",
    "schema_version",
    "system",
];
const SYSTEM_FIELDS: [&str; 3] = ["product", "vendor", "version_range"];
const PARAMETER_FIELDS: [&str; 11] = [
    "artifact_effect",
    "default",
    "domain",
    "evidence",
    "name",
    "provenance",
    "resolve",
    "state",
    "transformation_stage",
    "type",
    "value",
];

/// Why a profile file is refused. Stable code, the field to fix, and a sentence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    /// The stable code.
    pub code: &'static str,
    /// The field, such as `parameters[units].default`.
    pub field: String,
    /// What is wrong, in words.
    pub message: String,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}: {}", self.code, self.field, self.message)
    }
}

impl std::error::Error for Refusal {}

/// What is known about a parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// A fact.
    Known,
    /// A value taken on a recorded human assumption.
    Assumed,
    /// A fact nobody has observed. It holds no value, and no default ever becomes one.
    Unknown,
    /// A design choice the solver selects.
    Choice,
    /// An overridable default, with where it came from.
    Preference,
}

impl State {
    /// Its name in a profile file.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Known => "known",
            Self::Assumed => "assumed",
            Self::Unknown => "unknown",
            Self::Choice => "choice",
            Self::Preference => "preference",
        }
    }
}

/// How an unknown gets its answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolve {
    /// A person answers.
    AskHuman,
}

impl Resolve {
    /// Its name in a profile file.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::AskHuman => "ask_human",
        }
    }
}

/// What an unknown does to artifacts, beyond what the policy matrix already says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// No production release until it is resolved.
    BlocksRelease,
    /// No artifact at all until it is resolved.
    BlocksAll,
}

impl Effect {
    /// Its name in a profile file.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::BlocksRelease => "blocks_release",
            Self::BlocksAll => "blocks_all",
        }
    }
}

/// When a transformation is applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// After grading and before export.
    PostGradePreExport,
}

impl Stage {
    /// Its name in a profile file.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::PostGradePreExport => "post_grade_pre_export",
        }
    }
}

/// One layer of composition, strongest first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    /// A safety or design limit nothing can override.
    HardRestriction,
    /// What this factory says.
    FactoryOverride,
    /// An overridable preference.
    Preference,
    /// A default.
    Default,
}

impl Layer {
    /// The one order the layers have (roadmap §8).
    pub const ORDER: [Self; 4] = [
        Self::HardRestriction,
        Self::FactoryOverride,
        Self::Preference,
        Self::Default,
    ];

    /// Its name in a profile file.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::HardRestriction => "hard_restriction",
            Self::FactoryOverride => "factory_override",
            Self::Preference => "preference",
            Self::Default => "default",
        }
    }
}

/// A notch's physical shape as a factory states it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotchSpec {
    /// The shape.
    pub code: String,
    /// Its depth in millimetres.
    pub depth_mm: Rational,
    /// The roadmap's `sample_mm`, kept exactly as written. Nothing reads it yet.
    pub sample_mm: Rational,
}

/// A parameter's value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParamValue {
    /// One token.
    Symbol(String),
    /// Yes or no.
    Bool(bool),
    /// One exact number.
    Number(Rational),
    /// Two exact numbers.
    Pair(Rational, Rational),
    /// A notch's shape.
    Notch(NotchSpec),
    /// A pen for each function.
    Pens(BTreeMap<String, i64>),
}

/// One parameter of a profile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    /// Its name, one the schema knows.
    pub name: String,
    /// What it holds.
    pub kind: Kind,
    /// The values allowed, in order. Empty when the kind has none to list.
    pub domain: Vec<ParamValue>,
    /// What it is.
    pub state: State,
    /// Its value, unless its state says it has none.
    pub value: Option<ParamValue>,
    /// What a person is offered for an unknown. It never becomes the value by itself.
    pub default: Option<ParamValue>,
    /// References into the evidence store.
    pub evidence: Vec<String>,
    /// How an unknown gets its answer.
    pub resolve: Option<Resolve>,
    /// What an unknown does to artifacts, beyond the policy matrix.
    pub artifact_effect: Option<Effect>,
    /// When a transformation is applied.
    pub transformation_stage: Option<Stage>,
    /// Where a preference came from.
    pub provenance: Option<String>,
}

/// The system a profile is for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetSystem {
    /// Who makes it.
    pub vendor: String,
    /// What it is.
    pub product: String,
    /// The versions covered: `13.2` is that version, and `13.x` is 13 and everything under it.
    pub version_range: Vec<String>,
}

impl TargetSystem {
    /// Whether this version is covered.
    #[must_use]
    pub fn admits(&self, version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        self.version_range.iter().any(|pattern| {
            let wanted: Vec<&str> = pattern.split('.').collect();
            match wanted.split_last() {
                Some((&"x", head)) => parts.get(..head.len()) == Some(head),
                _ => parts == wanted,
            }
        })
    }
}

/// A content version: three whole numbers that rise when the content changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContentVersion(pub u32, pub u32, pub u32);

impl fmt::Display for ContentVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

/// A Factory Profile that has been read and checked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    /// Which profile this is.
    pub profile_id: String,
    /// Which content of it this is.
    pub content_version: ContentVersion,
    /// The system it is for.
    pub system: TargetSystem,
    /// The artifact targets it accepts.
    pub accepts: Vec<String>,
    /// Its parameters, in order.
    pub parameters: Vec<Parameter>,
}

struct Reader {
    refusals: Vec<Refusal>,
}

impl Reader {
    fn refuse(&mut self, code: &'static str, field: &str, message: String) {
        self.refusals.push(Refusal {
            code,
            field: field.to_owned(),
            message,
        });
    }

    fn malformed(&mut self, field: &str, wanted: &str) {
        self.refuse(
            "profile_field_malformed",
            field,
            format!("this should be {wanted}"),
        );
    }

    fn fields<'v>(
        &mut self,
        value: &'v Value,
        field: &str,
        known: &[&str],
    ) -> Option<&'v BTreeMap<String, Value>> {
        let Some(fields) = value.as_object() else {
            self.malformed(field, "an object");
            return None;
        };
        for key in fields.keys() {
            if !known.contains(&key.as_str()) {
                let at = if field.is_empty() {
                    key.clone()
                } else {
                    format!("{field}.{key}")
                };
                self.refuse(
                    "profile_field_unknown",
                    &at,
                    format!(
                        "this schema has no field {key}: it has {}",
                        known.join(", ")
                    ),
                );
            }
        }
        Some(fields)
    }

    fn required<'v>(
        &mut self,
        fields: &'v BTreeMap<String, Value>,
        name: &str,
        field: &str,
    ) -> Option<&'v Value> {
        let found = fields.get(name);
        if found.is_none() {
            self.refuse(
                "profile_field_missing",
                field,
                format!("a profile needs {name} here"),
            );
        }
        found
    }

    fn text(
        &mut self,
        fields: &BTreeMap<String, Value>,
        name: &str,
        field: &str,
    ) -> Option<String> {
        let value = self.required(fields, name, field)?;
        match value.as_text() {
            Some(text) if !text.trim().is_empty() => Some(text.to_owned()),
            _ => {
                self.malformed(field, "text that is not empty");
                None
            }
        }
    }

    fn texts(&mut self, value: &Value, field: &str) -> Vec<String> {
        let Some(items) = value.as_list() else {
            self.malformed(field, "a list");
            return Vec::new();
        };
        let mut out = Vec::new();
        for (at, item) in items.iter().enumerate() {
            match item.as_text() {
                Some(text) if !text.trim().is_empty() => out.push(text.to_owned()),
                _ => self.malformed(&format!("{field}[{at}]"), "text that is not empty"),
            }
        }
        out
    }

    fn number(&mut self, value: &Value, field: &str) -> Option<Rational> {
        let exact = match value {
            Value::Int(number) => Some(Rational::int(*number)),
            Value::Text(text) => parse_decimal(text),
            _ => None,
        };
        if exact.is_none() {
            self.malformed(
                field,
                "a whole number, or decimal text of up to 18 digits such as \"0.1\"",
            );
        }
        exact
    }

    fn system(&mut self, value: Option<&Value>) -> Option<TargetSystem> {
        let fields = self.fields(value?, "system", &SYSTEM_FIELDS)?;
        let vendor = self.text(fields, "vendor", "system.vendor");
        let product = self.text(fields, "product", "system.product");
        let range = self.required(fields, "version_range", "system.version_range");
        let version_range = range
            .map(|range| self.texts(range, "system.version_range"))
            .unwrap_or_default();
        if range.is_some() && version_range.is_empty() {
            self.malformed("system.version_range", "a list of at least one version");
        }
        for (at, pattern) in version_range.iter().enumerate() {
            if !version_pattern_is_good(pattern) {
                self.malformed(
                    &format!("system.version_range[{at}]"),
                    "a version such as 13.2, or one ending in .x such as 13.x",
                );
            }
        }
        Some(TargetSystem {
            vendor: vendor?,
            product: product?,
            version_range,
        })
    }

    fn accepts(&mut self, value: Option<&Value>) -> Vec<String> {
        let tokens = value
            .map(|value| self.texts(value, "accepts"))
            .unwrap_or_default();
        let mut seen = BTreeSet::new();
        for (at, token) in tokens.iter().enumerate() {
            let field = format!("accepts[{at}]");
            if !ACCEPTS.contains(&token.as_str()) {
                self.refuse(
                    "profile_token_unknown",
                    &field,
                    format!("no target is named {token}: {}", ACCEPTS.join(", ")),
                );
            }
            if !seen.insert(token.clone()) {
                self.refuse(
                    "profile_field_malformed",
                    &field,
                    format!("{token} is already listed"),
                );
            }
        }
        tokens
    }

    fn composition(&mut self, value: Option<&Value>) {
        let Some(fields) =
            value.and_then(|value| self.fields(value, "composition", &["precedence"]))
        else {
            return;
        };
        let Some(precedence) = self.required(fields, "precedence", "composition.precedence") else {
            return;
        };
        let order = self.texts(precedence, "composition.precedence");
        let wanted = Layer::ORDER.map(Layer::token);
        if order != wanted {
            self.refuse(
                "profile_precedence_wrong",
                "composition.precedence",
                format!("the order is fixed: {}", wanted.join(", ")),
            );
        }
    }

    fn value_of(&mut self, spec: &ParameterSpec, value: &Value, field: &str) -> Option<ParamValue> {
        match spec.kind {
            Kind::Enum => match value.as_text() {
                Some(token) => Some(ParamValue::Symbol(token.to_owned())),
                None => {
                    self.malformed(field, "a token");
                    None
                }
            },
            Kind::Bool => match value {
                Value::Bool(answer) => Some(ParamValue::Bool(*answer)),
                _ => {
                    self.malformed(field, "true or false");
                    None
                }
            },
            Kind::F32 => {
                let number = self.number(value, field)?;
                if spec.positive && number.sign() != core::cmp::Ordering::Greater {
                    self.malformed(field, "above zero");
                    return None;
                }
                Some(ParamValue::Number(number))
            }
            Kind::Vec2f => {
                let [first, second] = value.as_list().unwrap_or_default() else {
                    self.malformed(field, "a list of two numbers");
                    return None;
                };
                let first = self.number(first, &format!("{field}[0]"));
                let second = self.number(second, &format!("{field}[1]"));
                Some(ParamValue::Pair(first?, second?))
            }
            Kind::NotchSpec => {
                let fields = self.fields(value, field, &["code", "depth_mm", "sample_mm"])?;
                let code = self.text(fields, "code", &format!("{field}.code"));
                if let Some(code) = &code {
                    if !NOTCH_CODES.contains(&code.as_str()) {
                        self.refuse(
                            "profile_token_unknown",
                            &format!("{field}.code"),
                            format!("no notch is coded {code}: {}", NOTCH_CODES.join(", ")),
                        );
                    }
                }
                let mut length = |name: &str| {
                    let at = format!("{field}.{name}");
                    let number = self.required(fields, name, &at)?;
                    self.number(number, &at)
                };
                let depth_mm = length("depth_mm");
                let sample_mm = length("sample_mm");
                Some(ParamValue::Notch(NotchSpec {
                    code: code?,
                    depth_mm: depth_mm?,
                    sample_mm: sample_mm?,
                }))
            }
            Kind::PenMap => {
                let fields = self.fields(value, field, &PEN_FUNCTIONS)?;
                let mut pens = BTreeMap::new();
                for (function, pen) in fields {
                    match pen.as_int() {
                        Some(pen) if pen >= 1 => {
                            pens.insert(function.clone(), pen);
                        }
                        _ => self.malformed(&format!("{field}.{function}"), "a pen number from 1"),
                    }
                }
                if pens.is_empty() {
                    self.malformed(field, "at least one function with a pen");
                    return None;
                }
                Some(ParamValue::Pens(pens))
            }
        }
    }

    fn domain(
        &mut self,
        spec: &ParameterSpec,
        fields: &BTreeMap<String, Value>,
        label: &str,
    ) -> Vec<ParamValue> {
        let at = format!("{label}.domain");
        let Some(domain) = fields.get("domain") else {
            if spec.kind == Kind::Enum {
                self.refuse(
                    "profile_field_missing",
                    &at,
                    "an enum lists the values it can take".to_owned(),
                );
            }
            return Vec::new();
        };
        if !matches!(spec.kind, Kind::Enum | Kind::F32) {
            self.refuse(
                "profile_field_malformed",
                &at,
                format!("a {} has no list of values", spec.kind.token()),
            );
            return Vec::new();
        }
        let Some(items) = domain.as_list().filter(|items| !items.is_empty()) else {
            self.malformed(&at, "a list of at least one value");
            return Vec::new();
        };
        let mut out: Vec<ParamValue> = Vec::new();
        for (index, item) in items.iter().enumerate() {
            let field = format!("{at}[{index}]");
            let Some(value) = self.value_of(spec, item, &field) else {
                continue;
            };
            if out.contains(&value) {
                self.refuse(
                    "profile_field_malformed",
                    &field,
                    "this value is already listed".to_owned(),
                );
                continue;
            }
            if let ParamValue::Symbol(token) = &value {
                if !spec.tokens.contains(&token.as_str()) {
                    self.refuse(
                        "profile_token_unknown",
                        &field,
                        format!(
                            "nothing here understands {token} for {}: {}",
                            spec.name,
                            spec.tokens.join(", ")
                        ),
                    );
                    continue;
                }
            }
            out.push(value);
        }
        out
    }

    fn parameter(
        &mut self,
        value: &Value,
        at: usize,
        seen: &mut BTreeSet<String>,
    ) -> Option<Parameter> {
        let index = format!("parameters[{at}]");
        let Some(fields) = value.as_object() else {
            self.malformed(&index, "an object");
            return None;
        };
        let name = self.text(fields, "name", &format!("{index}.name"))?;
        let Some(spec) = spec_of(&name) else {
            self.refuse(
                "profile_parameter_unknown",
                &format!("{index}.name"),
                format!(
                    "no parameter is named {name}, and one that nothing reads would do nothing: {}",
                    crate::registry::PARAMETERS.map(|spec| spec.name).join(", ")
                ),
            );
            return None;
        };
        let label = format!("parameters[{name}]");
        if !seen.insert(name.clone()) {
            self.refuse(
                "profile_parameter_repeated",
                &format!("{index}.name"),
                format!("{name} is already in this profile"),
            );
            return None;
        }
        self.fields(value, &label, &PARAMETER_FIELDS);
        let declared = self.text(fields, "type", &format!("{label}.type"))?;
        if declared != spec.kind.token() {
            self.refuse(
                "profile_type_mismatch",
                &format!("{label}.type"),
                format!("{name} holds a {}, not a {declared}", spec.kind.token()),
            );
            return None;
        }
        let domain = self.domain(spec, fields, &label);
        let read = |reader: &mut Self, part: &str| {
            let at = format!("{label}.{part}");
            let value = fields.get(part)?;
            let value = reader.value_of(spec, value, &at)?;
            let inside = domain.is_empty() || domain.contains(&value);
            if !inside {
                reader.refuse(
                    "profile_value_outside_domain",
                    &at,
                    format!("this is not one of the values {name} lists"),
                );
            }
            Some(value)
        };
        let parameter_value = read(self, "value");
        let default = read(self, "default");
        let state = self.state(fields, &label)?;
        self.state_rules(spec, fields, state, &domain, &label);
        let evidence = fields
            .get("evidence")
            .map(|items| self.texts(items, &format!("{label}.evidence")))
            .unwrap_or_default();
        let resolve = self.resolve(fields, &label);
        let artifact_effect = self.effect(fields, &label);
        let transformation_stage = self.stage(spec, fields, &label);
        Some(Parameter {
            name,
            kind: spec.kind,
            domain,
            state,
            value: parameter_value,
            default,
            evidence,
            resolve,
            artifact_effect,
            transformation_stage,
            provenance: fields
                .get("provenance")
                .and_then(Value::as_text)
                .map(str::to_owned),
        })
    }

    fn state(&mut self, fields: &BTreeMap<String, Value>, label: &str) -> Option<State> {
        let at = format!("{label}.state");
        let token = self.text(fields, "state", &at)?;
        match token.as_str() {
            "known" => Some(State::Known),
            "assumed" => Some(State::Assumed),
            "unknown" => Some(State::Unknown),
            "choice" => Some(State::Choice),
            "preference" => Some(State::Preference),
            "derived" => {
                self.refuse(
                    "profile_state_unavailable",
                    &at,
                    "a profile has no formulas, so nothing in it can be derived".to_owned(),
                );
                None
            }
            other => {
                self.refuse(
                    "profile_token_unknown",
                    &at,
                    format!(
                        "no state is named {other}: known, assumed, unknown, choice, preference"
                    ),
                );
                None
            }
        }
    }

    fn state_rules(
        &mut self,
        spec: &ParameterSpec,
        fields: &BTreeMap<String, Value>,
        state: State,
        domain: &[ParamValue],
        label: &str,
    ) {
        let has = |name: &str| fields.contains_key(name);
        let token = state.token();
        let wrong = |reader: &mut Self, part: &str, why: String| {
            reader.refuse("profile_state_conflict", &format!("{label}.{part}"), why);
        };
        match state {
            State::Known | State::Assumed | State::Preference => {
                if !has("value") {
                    wrong(self, "value", format!("a {token} parameter holds a value"));
                }
                if has("default") {
                    wrong(
                        self,
                        "default",
                        format!("a {token} parameter holds a value and needs no default"),
                    );
                }
            }
            State::Unknown => {
                if has("value") {
                    wrong(
                        self,
                        "value",
                        "an unknown holds no value: no value is invented for it".to_owned(),
                    );
                }
                if !has("resolve") {
                    wrong(
                        self,
                        "resolve",
                        "an unknown says how it gets its answer, and a default never answers it"
                            .to_owned(),
                    );
                }
            }
            State::Choice => {
                if has("value") || has("default") {
                    let part = if has("value") { "value" } else { "default" };
                    wrong(
                        self,
                        part,
                        "the solver selects a choice, in the order its values are listed"
                            .to_owned(),
                    );
                }
                if spec.kind != Kind::Bool && domain.is_empty() {
                    wrong(
                        self,
                        "domain",
                        "a choice lists what it can choose".to_owned(),
                    );
                }
            }
        }
        if state != State::Unknown {
            for part in ["resolve", "artifact_effect"] {
                if has(part) {
                    wrong(
                        self,
                        part,
                        format!("{part} is for an unknown, and this is {token}"),
                    );
                }
            }
        }
        if state == State::Preference {
            if !fields
                .get("provenance")
                .is_some_and(|text| text.as_text().is_some())
            {
                wrong(
                    self,
                    "provenance",
                    "a preference says where it came from".to_owned(),
                );
            }
        } else if has("provenance") {
            wrong(
                self,
                "provenance",
                format!("provenance is for a preference, and this is {token}"),
            );
        }
    }

    fn resolve(&mut self, fields: &BTreeMap<String, Value>, label: &str) -> Option<Resolve> {
        let at = format!("{label}.resolve");
        match fields.get("resolve")?.as_text() {
            Some("ask_human") => Some(Resolve::AskHuman),
            _ => {
                self.refuse(
                    "profile_token_unknown",
                    &at,
                    "an unknown is resolved by ask_human: a person answers".to_owned(),
                );
                None
            }
        }
    }

    fn effect(&mut self, fields: &BTreeMap<String, Value>, label: &str) -> Option<Effect> {
        let at = format!("{label}.artifact_effect");
        match fields.get("artifact_effect")?.as_text() {
            Some("blocks_release") => Some(Effect::BlocksRelease),
            Some("blocks_all") => Some(Effect::BlocksAll),
            _ => {
                self.refuse(
                    "profile_token_unknown",
                    &at,
                    "an effect is blocks_release or blocks_all".to_owned(),
                );
                None
            }
        }
    }

    fn stage(
        &mut self,
        spec: &ParameterSpec,
        fields: &BTreeMap<String, Value>,
        label: &str,
    ) -> Option<Stage> {
        let at = format!("{label}.transformation_stage");
        let Some(stage) = fields.get("transformation_stage") else {
            if spec.transformation {
                self.refuse(
                    "profile_stage_wrong",
                    &at,
                    format!("{} is applied at a declared stage", spec.name),
                );
            }
            return None;
        };
        if !spec.transformation {
            self.refuse(
                "profile_stage_wrong",
                &at,
                format!("{} is not a transformation", spec.name),
            );
            return None;
        }
        match stage.as_text() {
            Some("post_grade_pre_export") => Some(Stage::PostGradePreExport),
            _ => {
                self.refuse(
                    "profile_stage_wrong",
                    &at,
                    format!("{} is applied at post_grade_pre_export", spec.name),
                );
                None
            }
        }
    }
}

fn version_pattern_is_good(pattern: &str) -> bool {
    let parts: Vec<&str> = pattern.split('.').collect();
    let Some((last, head)) = parts.split_last() else {
        return false;
    };
    let component = |part: &&str| {
        !part.is_empty() && *part != "x" && part.bytes().all(|byte| byte.is_ascii_alphanumeric())
    };
    head.iter().all(component) && (component(last) || (*last == "x" && !head.is_empty()))
}

fn id_is_good(id: &str) -> bool {
    id.bytes()
        .next()
        .is_some_and(|byte| byte.is_ascii_lowercase())
        && id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn content_version(text: &str) -> Option<ContentVersion> {
    let mut parts = text.split('.').map(|part| {
        if part.len() > 1 && part.starts_with('0') {
            None
        } else {
            part.parse::<u32>().ok()
        }
    });
    let version = ContentVersion(parts.next()??, parts.next()??, parts.next()??);
    parts.next().is_none().then_some(version)
}

fn number_value(number: Rational) -> Value {
    if number.denominator() == 1 {
        if let Ok(whole) = i64::try_from(number.numerator()) {
            return Value::Int(whole);
        }
    }
    Value::Text(decimal_text(number).unwrap_or_else(|| number.to_string()))
}

impl ParamValue {
    /// The one way this value is written.
    #[must_use]
    pub fn to_value(&self) -> Value {
        match self {
            Self::Symbol(token) => Value::Text(token.clone()),
            Self::Bool(answer) => Value::Bool(*answer),
            Self::Number(number) => number_value(*number),
            Self::Pair(first, second) => {
                Value::List(vec![number_value(*first), number_value(*second)])
            }
            Self::Notch(notch) => Value::Object(BTreeMap::from([
                ("code".to_owned(), Value::Text(notch.code.clone())),
                ("depth_mm".to_owned(), number_value(notch.depth_mm)),
                ("sample_mm".to_owned(), number_value(notch.sample_mm)),
            ])),
            Self::Pens(pens) => Value::Object(
                pens.iter()
                    .map(|(function, pen)| (function.clone(), Value::Int(*pen)))
                    .collect(),
            ),
        }
    }
}

fn texts_value(items: &[String]) -> Value {
    Value::List(items.iter().cloned().map(Value::Text).collect())
}

fn entry(name: &str, value: Value) -> (String, Value) {
    (name.to_owned(), value)
}

impl Parameter {
    /// The one way this parameter is written. A field with nothing to say is left out.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = BTreeMap::from([
            entry("name", Value::Text(self.name.clone())),
            entry("type", Value::Text(self.kind.token().to_owned())),
            entry("state", Value::Text(self.state.token().to_owned())),
        ]);
        if !self.domain.is_empty() {
            let domain = self.domain.iter().map(ParamValue::to_value).collect();
            fields.insert("domain".to_owned(), Value::List(domain));
        }
        if let Some(value) = &self.value {
            fields.insert("value".to_owned(), value.to_value());
        }
        if let Some(default) = &self.default {
            fields.insert("default".to_owned(), default.to_value());
        }
        if !self.evidence.is_empty() {
            fields.insert("evidence".to_owned(), texts_value(&self.evidence));
        }
        let mut token = |name: &str, token: Option<&'static str>| {
            if let Some(token) = token {
                fields.insert(name.to_owned(), Value::Text(token.to_owned()));
            }
        };
        token("resolve", self.resolve.map(Resolve::token));
        token("artifact_effect", self.artifact_effect.map(Effect::token));
        token(
            "transformation_stage",
            self.transformation_stage.map(Stage::token),
        );
        if let Some(provenance) = &self.provenance {
            fields.insert("provenance".to_owned(), Value::Text(provenance.clone()));
        }
        Value::Object(fields)
    }
}

impl Profile {
    /// Reads and checks a profile file, except that evidence references are not followed. See [`Profile::read`].
    ///
    /// # Errors
    ///
    /// Every refusal the file earns, each naming its field.
    pub fn from_value(value: &Value) -> Result<Self, Vec<Refusal>> {
        let mut reader = Reader {
            refusals: Vec::new(),
        };
        let Some(fields) = reader.fields(value, "", &TOP_FIELDS) else {
            return Err(reader.refusals);
        };
        let profile_id = reader.text(fields, "profile_id", "profile_id");
        if let Some(id) = &profile_id {
            if !id_is_good(id) {
                reader.malformed("profile_id", "lowercase letters, digits and dashes");
            }
        }
        if let Some(schema) = reader.text(fields, "schema_version", "schema_version") {
            if schema != SCHEMA_VERSION {
                reader.refuse(
                    "profile_schema_version",
                    "schema_version",
                    format!("this reads schema {SCHEMA_VERSION}, and the file is {schema}"),
                );
            }
        }
        let version = reader.text(fields, "content_version", "content_version");
        let version = version.and_then(|text| {
            let parsed = content_version(&text);
            if parsed.is_none() {
                reader.malformed("content_version", "three whole numbers such as 1.3.0");
            }
            parsed
        });
        let system = reader.required(fields, "system", "system");
        let system = reader.system(system);
        let accepts = reader.required(fields, "accepts", "accepts");
        let accepts = reader.accepts(accepts);
        let composition = reader.required(fields, "composition", "composition");
        reader.composition(composition);
        let mut parameters = Vec::new();
        if let Some(items) = reader.required(fields, "parameters", "parameters") {
            match items.as_list() {
                Some(items) => {
                    let mut seen = BTreeSet::new();
                    for (at, item) in items.iter().enumerate() {
                        if let Some(parameter) = reader.parameter(item, at, &mut seen) {
                            parameters.push(parameter);
                        }
                    }
                }
                None => reader.malformed("parameters", "a list"),
            }
        }
        if let Some(rules) = reader.required(fields, "constraints_ast", "constraints_ast") {
            if rules.as_list().is_none_or(|rules| !rules.is_empty()) {
                reader.refuse(
                    "profile_constraints_unread",
                    "constraints_ast",
                    "this reads an empty list so far: the typed rules are leaf G4-PROFILES.2"
                        .to_owned(),
                );
            }
        }
        match (profile_id, version, system, reader.refusals.is_empty()) {
            (Some(profile_id), Some(content_version), Some(system), true) => Ok(Self {
                profile_id,
                content_version,
                system,
                accepts,
                parameters,
            }),
            _ => Err(reader.refusals),
        }
    }

    /// Reads and checks a profile file, following every evidence reference into what the store holds.
    ///
    /// # Errors
    ///
    /// Every refusal the file earns, each naming its field.
    pub fn read(value: &Value, evidence: &BTreeSet<String>) -> Result<Self, Vec<Refusal>> {
        let profile = Self::from_value(value)?;
        let dangling = profile.dangling_evidence(evidence);
        if dangling.is_empty() {
            Ok(profile)
        } else {
            Err(dangling)
        }
    }

    /// Each evidence reference that points at nothing.
    #[must_use]
    pub fn dangling_evidence(&self, evidence: &BTreeSet<String>) -> Vec<Refusal> {
        let mut out = Vec::new();
        for parameter in &self.parameters {
            for (at, reference) in parameter.evidence.iter().enumerate() {
                if !evidence.contains(reference) {
                    out.push(Refusal {
                        code: "profile_evidence_dangling",
                        field: format!("parameters[{}].evidence[{at}]", parameter.name),
                        message: format!("no evidence record is {reference}"),
                    });
                }
            }
        }
        out
    }

    /// Each parameter that says it is known and cites nothing. The roadmap's own example has six.
    #[must_use]
    pub fn known_without_evidence(&self) -> Vec<&str> {
        self.parameters
            .iter()
            .filter(|parameter| parameter.state == State::Known && parameter.evidence.is_empty())
            .map(|parameter| parameter.name.as_str())
            .collect()
    }

    /// The one way this profile is written.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = self.content();
        fields.insert(
            "content_version".to_owned(),
            Value::Text(self.content_version.to_string()),
        );
        fields.insert(
            "schema_version".to_owned(),
            Value::Text(SCHEMA_VERSION.to_owned()),
        );
        Value::Object(fields)
    }

    fn content(&self) -> BTreeMap<String, Value> {
        BTreeMap::from([
            entry("profile_id", Value::Text(self.profile_id.clone())),
            entry(
                "system",
                Value::Object(BTreeMap::from([
                    entry("vendor", Value::Text(self.system.vendor.clone())),
                    entry("product", Value::Text(self.system.product.clone())),
                    entry("version_range", texts_value(&self.system.version_range)),
                ])),
            ),
            entry("accepts", texts_value(&self.accepts)),
            entry(
                "parameters",
                Value::List(self.parameters.iter().map(Parameter::to_value).collect()),
            ),
            entry("constraints_ast", Value::List(Vec::new())),
            entry(
                "composition",
                Value::Object(BTreeMap::from([entry(
                    "precedence",
                    Value::List(
                        Layer::ORDER
                            .iter()
                            .map(|layer| Value::Text(layer.token().to_owned()))
                            .collect(),
                    ),
                )])),
            ),
        ])
    }

    /// The digest of what this profile says, leaving out its own version labels.
    #[must_use]
    pub fn content_digest(&self) -> String {
        sha256_hex(Value::Object(self.content()).to_text().as_bytes())
    }
}

/// Whether `later` may follow `earlier`: the content version moves when the content does, and only then.
///
/// # Errors
///
/// They are different profiles, the content changed under the same version or a lower one, or a version moved over identical content.
pub fn check_succession(earlier: &Profile, later: &Profile) -> Result<(), Refusal> {
    let refuse = |code, message: String| Refusal {
        code,
        field: "content_version".to_owned(),
        message,
    };
    if earlier.profile_id != later.profile_id {
        return Err(Refusal {
            code: "profile_identity_changed",
            field: "profile_id".to_owned(),
            message: format!(
                "{} cannot follow {}: they are different profiles",
                later.profile_id, earlier.profile_id
            ),
        });
    }
    let same = earlier.content_digest() == later.content_digest();
    let (from, to) = (earlier.content_version, later.content_version);
    if same && from != to {
        return Err(refuse(
            "profile_version_without_change",
            format!("{from} became {to}, and the content is identical"),
        ));
    }
    if !same && to <= from {
        return Err(refuse(
            "profile_change_without_version",
            format!("the content changed, and {to} does not rise above {from}"),
        ));
    }
    Ok(())
}
