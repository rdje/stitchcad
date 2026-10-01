# Length declarations retain authored state and provenance without a numeric unknown fallback

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.4a.1`; ontology §2.1/§2.2/§5, formula-language §2/§3 and roadmap §2.3.

answers: "where do measurement state and source live?" · "can unknown inputs default to zero?" · "does known prove evidence?" · "how are core dependency cycles avoided?" · "how do shared machine tokens preserve spelling?" · "where is procedure documentation stored?" · "can body and garment references interchange?" · "how does a measurement table refuse token or scalar reassignment?" · "does close fit authorize compression?" · "how are per-POM ease mappings tied to current tables?"

## The fact / decision

The shared immutable length declaration lives in sc-core and holds identity, source reference and
exactly one authored state. sc-measure consumes this contract and core identity; sc-core does not
acquire a dependency on the higher measurement crate. Recipe evaluation consumes a core input
contract through its host context. Measurement metadata references/borrows the canonical declaration,
without a second numeric/state cache. Ease and size-chart inputs can share the same length contract.

Known holds an authored Length and nonempty distinct evidence references; Assumed holds an authored
Length and a recorded-assumption reference; Unknown holds the observation reference needed to obtain
a value; Preference holds an authored Length and provenance reference; Derived holds a formula
reference without an authored/cached result. Mandatory references are not optional strings. Empty or
duplicate known evidence is structurally refused. A declared known state claims evidence; it does
not independently establish its existence, scope, truth or validity.

The input exposes its authored value only for known/assumed/preference. Unknown and derived return
structured unresolved-source refusals naming declaration and observation/formula, never zero or a
preference substituted for observation. Querying an authored preference is not applying it over an
unknown: these are distinct states, and profile composition remains G4's rule. Explicit signed values
and zero retain their units; measurement-specific domains/physical repeatability require their named
procedure and constraints rather than an invented positivity rule in the general length declaration.

## Boundary and later obligations

- G1-SLICE.4a.2/.3 validates MeasurementTable metadata and current references. State/source truth
  is distinct from structural input and cannot be established by a display label or an id alone.
- G1-SLICE.5/.6 resolves declaration/formula dependencies, rejects missing/kind-mismatched references
  and cycles, and propagates derived input state. A derived reference has no independently editable result.
- G4-PROFILES.6/.7 validates scoped evidence and artifact-specific uncertainty closure/policy; G4
  composition resolves preference precedence. No generic declaration query grants export permission.
- Known/assumed/preference authored values are not immutable evaluation certificates: Design revision
  and current source/registry checks remain necessary before computation, approval and release.

This slice introduces only the length-valued contract required by measurement/ease/chart work. It
does not introduce text-valued formula parameters, new physical measurement vocabulary, standards
mapping data, solver assignment to unknown facts or an artifact policy default.

## Shared machine tokens — G1-SLICE.4a.2a

Measurement tokens and recipe identifiers share a core MachineToken. This avoids a higher-crate
lexical dependency or divergent validators. The grammar's ASCII lower-snake rule is made exact:
a lowercase letter starts the token; letters/digits continue each nonempty segment; one underscore
separates segments. Keywords let/assert/if refuse at token construction. Exact authored bytes remain
unchanged, with no normalization or display-name inference. Built-in reserved names such as eps_geo
are valid references; measurement/recipe namespace binding rejects attempts to redeclare them.
MachineToken is neither a scalar text value nor a localized label. Its private representation needs
validated reconstruction to replace a token; a cloned replacement never mutates earlier metadata.

## Measurement metadata — G1-SLICE.4a.2b

sc-measure borrows core declarations and tokens. A measurement retains name, token, entered unit,
body/garment kind, two landmark identities, procedure identity and declaration identity. Landmarks
have explicit kind and source; the canonical procedure record holds nonblank documentation, kind
and source. Metadata never copies procedure prose, value or state. Requiring canonical documentation
here avoids a dangling document-id claim while still treating physical repeatability and source
truth as separately unproved. Caller-authored fixture procedures are not sourced standard content.

Current context rejects duplicate identities across its declaration/landmark/procedure inventories;
a measurement id cannot reuse one of those record identities. Landmark/procedure kind mismatches
refuse rather than interchange body and garment. Same landmark at both ends is legal for girth-level
intent; no unproved straight-line or distinct-endpoint rule is invented. Target queries validate only
the requested reference; complete current validation checks all references. Same-id record replacement
is read as current content, with no automatic retargeting. Design still validates global registries,
source provenance and evidence validity. Metadata cannot rebind the eight reserved formula inputs.

## Verification

The core contract tests must cover all five states, required provenance, nonempty/distinct known
evidence, explicit signed/zero lengths, structured unknown/derived refusals and immutable replacement.
Independent guard/query mutations must fail real regressions, with restored strict checks and WASM.
The book documents authored-state limits; later consumers retain their owned registry/evidence proofs.

## Measurement tables retain bindings, not duplicate scalar records

G1-SLICE.4a.3 gives each named table stable identity and ordered bindings that capture measurement
identity, exact machine token, body/POM kind and canonical declaration identity. These are the
binding's expected targets; queries borrow the supplied current records. Same-id changes to canonical
state/source remain visible without numeric caching. A missing measurement never transfers a token
to a surviving peer; changed token/kind/declaration requires an explicit validated table replacement.
This mirrors the ontology's no-silent-reassignment rule, rather than inferring identity from a label.

Unique measurement identities and tokens are enforced per table. Distinct tables may use the same
spelling for distinct canonical measurements; recipe composition later enforces its one flat
namespace. Shared declarations, repeated display labels and mixed body/POM entries are not invented
ambiguities. Empty draft tables are legal; the normative contract declares no minimum cardinality.
Context rejects duplicate/cross-kind canonical identities before lookup. Table validation checks all
current metadata and its required targets; a targeted query proves just the selected entry. It does
not certify a caller's Design revision, evidence truth, physical repeatability or export permission.

## Individual Ease — G1-SLICE.4b.1

The immutable mapping retains expected body/POM id/token/kind/scalar bindings and a distinct canonical
signed amount declaration. Amount state/source/provenance is borrowed, never copied. Mapping provenance
owns correspondence/fit intent; explicit Declared compression carries its own provenance identity.
Close < Semi < Loose is an ordered vocabulary, not numerical thresholds or compression authorization.
Current same-id amount edits re-run negative permission; unknown/derived drafts retain no numeric
fallback. Evaluators must check signed result permission explicitly, then separately prove current
inputs and provenance. Individual mappings do not yet certify selected table membership (.4b.2).
Structural compression declarations do not expand the v1 garment envelope; G3 owns its refusal.

## Per-POM Ease sets — G1-SLICE.4b.2

The ordered immutable set owns unique Ease-id/token/POM bindings plus explicit body/garment table
identities; one mixed table is legal. Shared body and amount declarations are not ambiguity. Bindings
pin each mapping's body/POM metadata expectations and amount identity, never its fit or state/provenance.
Current lookup borrows the canonical mapping, refuses retargeting or missing identities, validates both
selected table memberships and current Ease. Same-id intent/permission and scalar-state edits stay
visible. Empty drafts and targeted queries do not certify complete chart/Design coverage; full set
validation checks all entries. Identity, scope, factual evidence and evaluation remain distinct.
