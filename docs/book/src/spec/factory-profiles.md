# Factory profiles

A factory's cutter takes centimetres or millimetres, wants the allowance in the outline or not, and reads one DXF version and not another. Get any of those wrong and the pattern looks fine until it is cut. A Factory Profile is where those facts live, with what is known about each one, including that nobody knows yet.

**What works today.** The file: read, checked and written back in one form, with every mistake in it reported at once by the field to fix. The rule that a content version moves when the content does, and only then. **What does not exist yet:** rules in a profile (`G4-PROFILES.2`) and solving over them (`G4-PROFILES.3`). The evidence records a reference points at (`G4-PROFILES.6`), so a caller passes the identifiers that exist. What an unknown does to an export (`G4-PROFILES.7`). Layering one profile over another (`G4-PROFILES.8`). An editor, so for now a profile is written by hand (`G4-PROFILES.9`). **Nothing reads a profile yet:** no export changes because of one.

```rust
use sc_profiles::{Profile, State};
use sc_store::parse;
use std::collections::BTreeSet;

let file = parse(r#"{
  "profile_id": "acme-cmt-pt",
  "schema_version": "2.0",
  "content_version": "1.3.0",
  "system": {"vendor": "lectra", "product": "modaris", "version_range": ["13.x"]},
  "accepts": ["dxf-aama-named", "plt"],
  "parameters": [
    {"name": "seam_allowance_policy", "type": "enum", "domain": ["included", "excluded"],
     "value": "included", "state": "known", "evidence": ["ev:2026-07-14"]},
    {"name": "units", "type": "enum", "domain": ["mm", "cm"], "state": "unknown",
     "default": "cm", "resolve": "ask_human", "artifact_effect": "blocks_release"},
    {"name": "max_cutting_width_mm", "type": "f32", "value": "1520.5", "state": "known"}
  ],
  "constraints_ast": [],
  "composition": {"precedence": ["hard_restriction", "factory_override", "preference", "default"]}
}"#).expect("the text is well formed");

let evidence = BTreeSet::from(["ev:2026-07-14".to_owned()]);
let profile = Profile::read(&file, &evidence).expect("it validates");

// The target system's versions mean something definite.
assert!(profile.system.admits("13.2.1"));
assert!(!profile.system.admits("14.0"));

// Nobody has told us the units. The default is there to offer a person, and it is not the value.
let units = &profile.parameters[1];
assert_eq!(units.state, State::Unknown);
assert_eq!(units.value, None);

// It says it is known and cites nothing. That is allowed, and it is listed rather than hidden.
assert_eq!(profile.known_without_evidence(), ["max_cutting_width_mm"]);
```

## A mistake names its field

Every mistake is reported at once, in the order of the file, with a stable code. A person fixing a file sees the whole list, and an editor can point at each field.

```rust
use sc_profiles::Profile;
use sc_store::parse;
use std::collections::BTreeSet;

let file = parse(r#"{
  "profile_id": "acme-cmt-pt", "schema_version": "2.0", "content_version": "1.3.0",
  "system": {"vendor": "lectra", "product": "modaris", "version_range": ["13.x"]},
  "accepts": ["dxf-aama-named"],
  "parameters": [
    {"name": "unit", "type": "enum", "domain": ["mm", "cm"], "value": "mm", "state": "known"},
    {"name": "cut_sew_swap", "type": "bool", "value": false, "state": "known", "evidence": ["ev:lost"]},
    {"name": "spline_policy", "type": "enum", "domain": ["explode_bulge"], "value": "flatten_chord_0.1mm", "state": "known"}
  ],
  "constraints_ast": [],
  "composition": {"precedence": ["hard_restriction", "factory_override", "preference", "default"]}
}"#).expect("the text is well formed");

let refusals = Profile::from_value(&file).expect_err("two mistakes in the file itself");
let found: Vec<(&str, &str)> = refusals.iter().map(|r| (r.code, r.field.as_str())).collect();
assert_eq!(found, [
    ("profile_parameter_unknown", "parameters[0].name"),
    ("profile_value_outside_domain", "parameters[spline_policy].value"),
]);
```

A reference to evidence that does not exist is the same kind of mistake, and `Profile::read` finds it.

```rust
use sc_profiles::Profile;
use sc_store::parse;
use std::collections::BTreeSet;

let file = parse(r#"{
  "profile_id": "acme-cmt-pt", "schema_version": "2.0", "content_version": "1.3.0",
  "system": {"vendor": "lectra", "product": "modaris", "version_range": ["13.x"]},
  "accepts": ["dxf-aama-named"],
  "parameters": [{"name": "cut_sew_swap", "type": "bool", "value": false, "state": "known", "evidence": ["ev:lost"]}],
  "constraints_ast": [],
  "composition": {"precedence": ["hard_restriction", "factory_override", "preference", "default"]}
}"#).expect("the text is well formed");

let refusals = Profile::read(&file, &BTreeSet::new()).expect_err("it points at nothing");
assert_eq!(refusals[0].code, "profile_evidence_dangling");
assert_eq!(refusals[0].field, "parameters[cut_sew_swap].evidence[0]");
```

## The file

| Field | What it holds |
| --- | --- |
| `profile_id` | Which profile this is: lowercase letters, digits and dashes, starting with a letter |
| `schema_version` | The form of the file. Always `2.0` here, and a content change never moves it |
| `content_version` | What this version of the profile says, as three whole numbers |
| `system` | The system it is for: `vendor`, `product`, `version_range` |
| `accepts` | The artifact targets it takes |
| `parameters` | What is known about this factory, in order |
| `constraints_ast` | Its rules. An empty list until `G4-PROFILES.2`, and rules are refused until then rather than ignored |
| `composition` | Its `precedence`, which has one permitted order: `hard_restriction`, `factory_override`, `preference`, `default` |

A field this schema does not have is refused. A field that does nothing should not look as if it did something.

| Target | What it is |
| --- | --- |
| `dxf-aama-named` | DXF with named layers |
| `dxf-astm-num-r13` | DXF with numbered layers |
| `plt` | A plotter file |
| `pdf-a0` | A printable PDF, at A0 or tiled |

### What a version range covers

| Written | Covers | Does not cover |
| --- | --- | --- |
| `13.x` | `13`, `13.0`, `13.2`, `13.2.1` | `14`, `130` |
| `13.2` | `13.2` | `13`, `13.2.1` |

A list covers whatever any of its entries covers. An `x` can only be last, and only after at least one number. Parts compare as written, so `13.0` and `13` are different versions unless a range covers both.

## A parameter

| Field | What it holds |
| --- | --- |
| `name` | One the schema knows, from the table below |
| `type` | What kind of value it is. It has to be that parameter's own type |
| `domain` | The values it can take, in the order a search would try them. An enum needs one |
| `value` | Its value. What may be here depends on `state` |
| `default` | What a person would be offered for an unknown |
| `state` | What is known about it |
| `evidence` | References to the records behind it |
| `resolve` | How an unknown gets its answer. `ask_human` is the only way so far: a person answers |
| `artifact_effect` | What an unknown does to artifacts: `blocks_release`, or `blocks_all` |
| `transformation_stage` | When a transformation is applied. `post_grade_pre_export` is the only stage so far |
| `provenance` | Where a preference came from, as a sentence |

**The name has to be one this schema knows.** A parameter that nothing reads would change nothing while looking as if it did, which is worse than being told. A profile's domain can narrow the values the table gives, and it cannot add to them.

| Name | Type | Values |
| --- | --- | --- |
| `seam_allowance_policy` | `enum` | `included`, `excluded` |
| `notch_geometry` | `notch_spec` | A shape from `slit`, `V`, `T`, `castle`, `check`, `U`, with `depth_mm` and `sample_mm`. A profile's `code` holds the shape |
| `units` | `enum` | `mm`, `cm` |
| `dxf_version_target` | `enum` | `r12`, `r13`, `r2000` |
| `spline_policy` | `enum` | `explode_bulge`, `flatten_chord_0.1mm` |
| `shrinkage_xy_pct` | `vec2f` | Two numbers |
| `max_cutting_width_mm` | `f32` | A number above zero |
| `piece_block_required` | `bool` | Yes or no |
| `cut_sew_swap` | `bool` | Yes or no |
| `hpgl_pens` | `pen_map` | A pen number from 1 for each of `cut`, `draw`, `notch`, `drill`, and not all four are needed |

These are the ten in the roadmap's example. **Nothing on this list is read by an export yet,** so `r2000`, which no writer here produces, is accepted as a token and promises nothing. `sample_mm` is in the roadmap's example with no definition: it is kept exactly and nothing uses it.

### The types say no floating point

The roadmap calls two types `f32` and `vec2f`, and those names are kept so that a file matches it. What is behind them is exact. A whole number is written as a number, anything else as decimal text such as `"1520.5"`, up to 18 digits, because this project's text files have [no floating point](project-store.md). `1520`, `"1520"` and `"1520.0"` are one value with one spelling, so they cannot look like a change in content.

### What each state allows

| State | `value` | `default` | `resolve` | Also |
| --- | --- | --- | --- | --- |
| `known` | Needed | Refused | Refused | Should cite `evidence` |
| `assumed` | Needed | Refused | Refused | A person's recorded assumption, and not evidence |
| `unknown` | **Refused** | Allowed | Needed | May carry `artifact_effect` |
| `choice` | Refused | Refused | Refused | Needs a `domain`, or to be a `bool` |
| `preference` | Needed | Refused | Refused | Needs `provenance` |

**An unknown never holds a value, and a default never becomes one.** A plausible unit can still produce a pattern nobody can use. An unknown may have no default at all, because inventing one would be the very thing this is against. The roadmap's sentence "value OR default" is read here as "not both".

`derived`, the sixth word in [the state vocabulary](ontology.md), is refused here. A derived value is computed from others, and a profile has no formulas.

`known_without_evidence` lists what says it is known and cites nothing. The roadmap's own example has six of these, so it cannot be a refusal, but "known" means a fact with evidence behind it, so it is listed and not hidden.

## When the content version moves

`content_digest` is the digest of what a profile says, leaving out its two version labels. `check_succession` compares two versions of one profile.

```rust
use sc_profiles::{check_succession, Profile};
use sc_store::parse;

let at = |version: &str, swap: bool| {
    let text = format!(r#"{{
      "profile_id": "acme-cmt-pt", "schema_version": "2.0", "content_version": "{version}",
      "system": {{"vendor": "lectra", "product": "modaris", "version_range": ["13.x"]}},
      "accepts": ["plt"],
      "parameters": [{{"name": "cut_sew_swap", "type": "bool", "value": {swap}, "state": "known"}}],
      "constraints_ast": [],
      "composition": {{"precedence": ["hard_restriction", "factory_override", "preference", "default"]}}
    }}"#);
    Profile::from_value(&parse(&text).expect("well formed")).expect("it validates")
};

assert!(check_succession(&at("1.3.0", false), &at("1.4.0", true)).is_ok());
let code = |later| check_succession(&at("1.3.0", false), &later).expect_err("refused").code;
assert_eq!(code(at("1.3.1", false)), "profile_version_without_change");
assert_eq!(code(at("1.3.0", true)), "profile_change_without_version");
```

| Refusal | Means |
| --- | --- |
| `profile_version_without_change` | The label moved and the content is identical |
| `profile_change_without_version` | The content changed, and the version did not rise |
| `profile_identity_changed` | These are two different profiles |

## Every refusal

| Code | Raised when |
| --- | --- |
| `profile_field_unknown` | A field this schema does not have |
| `profile_field_missing` | A field it needs is not there |
| `profile_field_malformed` | A field's form is wrong: a list where a number goes, a width at zero |
| `profile_schema_version` | The file is another schema |
| `profile_parameter_unknown` | A parameter this schema does not know |
| `profile_parameter_repeated` | A parameter given twice |
| `profile_type_mismatch` | A parameter declared as a type that is not its own |
| `profile_token_unknown` | A token nothing here understands |
| `profile_value_outside_domain` | A value or default its own domain does not list |
| `profile_state_conflict` | What a parameter holds does not go with its state |
| `profile_state_unavailable` | The state `derived`, which a profile cannot hold |
| `profile_stage_wrong` | A transformation's stage is missing or wrong, or a stage on what is not one |
| `profile_precedence_wrong` | The layers are not in their one order |
| `profile_constraints_unread` | Rules, which are not read until `G4-PROFILES.2` |
| `profile_evidence_dangling` | An evidence reference points at nothing |

## A profile on the shelf

`conformance/profiles/acme-cmt-pt/profile.json` is the roadmap's example in this form. The test reads the example out of the roadmap itself and compares it with those bytes, so the roadmap and this schema cannot drift apart without a test failing.

```text
$ cargo test -p sc-profiles --test schema
```

**What this does not show.** That any factory's profile looks like this: the example is the roadmap's, invented for illustration, and no factory has been asked. That the ten parameters are enough, which only real factories can say. That `V`, `slit` and the others are the shapes a factory means by those words: they come from a reading of the layer numbers in a [sample of the standard](grade-rules.md), and no sewing expert has reviewed them.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `G4-PROFILES.2` | The leaf for rules in a profile |
| `G4-PROFILES.3` | The leaf for solving over them |
| `G4-PROFILES.6` | The leaf for evidence records |
| `G4-PROFILES.7` | The leaf for what an unknown does to an export |
| `G4-PROFILES.8` | The leaf for layering profiles |
| `G4-PROFILES.9` | The leaf for an editor |
| `Profile::read` | Reads a file and follows its evidence references |
| `2.0` | The schema this reads |
| `vendor` | Who makes a target system |
| `product` | What that system is |
| `version_range` | Which of its versions a profile is for |
| `precedence` | The order layers win in |
| `hard_restriction` | A limit nothing overrides |
| `factory_override` | What this factory says |
| `preference` | An overridable default. Also a parameter's state |
| `default` | The weakest layer. Also a parameter's field |
| `13.x` | 13 and everything under it |
| `13.2` | That version only |
| `13` | A version |
| `13.0` | A version |
| `13.2.1` | A version |
| `14` | A version |
| `130` | A version |
| `x` | The last part of a range that covers what is under it |
| `ask_human` | A person answers |
| `blocks_release` | No production release until it is answered |
| `blocks_all` | No artifact until it is answered |
| `post_grade_pre_export` | After grading and before export |
| `included` | The allowance is in the outline |
| `excluded` | The allowance is not in the outline |
| `mm` | Millimetres |
| `cm` | Centimetres |
| `r12` | DXF release 12 |
| `r13` | DXF release 13 |
| `r2000` | DXF release 2000 |
| `explode_bulge` | Curves as arcs |
| `flatten_chord_0.1mm` | Curves as straight runs within a tenth of a millimetre |
| `enum` | One token from a list |
| `notch_spec` | A notch's shape |
| `vec2f` | Two exact numbers |
| `f32` | One exact number |
| `bool` | Yes or no |
| `pen_map` | A pen for each function |
| `slit` | A notch that is a cut |
| `V` | A notch that is a V |
| `T` | A notch that is a T |
| `castle` | A notch that is a castle |
| `check` | A notch that is a check |
| `U` | A notch that is a U |
| `code` | A notch's shape, in a `notch_spec` value |
| `depth_mm` | A notch's depth |
| `sample_mm` | In the roadmap's example, undefined there |
| `cut` | A plotter's cutting function |
| `draw` | A plotter's drawing function |
| `notch` | A plotter's notching function |
| `drill` | A plotter's drilling function |
| `"1520.5"` | A number as text |
| `1520` | A number |
| `"1520"` | A number as text |
| `"1520.0"` | The same number as text |
| `known` | A fact |
| `assumed` | A recorded assumption |
| `unknown` | Nobody has observed it |
| `choice` | The solver selects |
| `derived` | Computed from others |
| `known_without_evidence` | What says it is known and cites nothing |
| `content_digest` | The digest of what a profile says |
| `check_succession` | Compares two versions of a profile |
| `conformance/profiles/acme-cmt-pt/profile.json` | The roadmap's example |
