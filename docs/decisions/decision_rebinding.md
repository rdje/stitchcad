# Binding refusals retain real sources and available recipe locations

- **Type:** `decision`
- **Date:** `2026-10-03` (UTC)
- **Status:** `active`; engineering-verified, independent approval unclaimed
- **Owner / source:** `G1-SLICE.5b.2c.1b`, director's explicit delegation for D131; canonical
  `docs/book/src/spec/formula-language.md` §3.1/5.2.1 and governance §6.1.

answers: "what arguments describe a reserved-name refusal?" · "which recipe indices can a binding diagnostic carry?" · "how does detached checking differ from whole preflight?"

## D131 reserved-name diagnostic proposal — resolved under delegation 2026-10-03 (UTC)

Owner: G1-SLICE.5b.2c.1a/.1b. Contract3.1 forbids every origin from rebinding eight reserved
names; the static annex and actual reference use formula_rebinding. Contract5.2 only assigns
that token to a repeated recipe let and requires both statement indices. An initial authored
declaration has no recipe statement, and a reserved declaration has no prior recipe statement.
The common diagnostic-context rule explicitly forbids fabricated indices.

Actual namespace([('eps_num', {'kind':'length', 'origin':'measurement'})]) refuses with
formula_rebinding, message reserved `eps_num` cannot be declared by measurement, arguments={}.
Actual static_statement('let eps_num:length=1', {}) uses the same token, also with arguments={}.
reserved_diagnostic_review.py reproduces120 reserved refusals and one ordinary recipe rebinding;
three actual compiled assertion controls detect token changes or a fabricated zero index. This
documents the mismatch; it does not accept the empty argument dictionaries as complete diagnostics.

Recommended: extend formula_rebinding's raised-when clause to reserved-name binding attempts.
For this case require the name, reserved metadata source (fixed kind/origin/required context)
and attempted binding source/origin. Retain actual ordinal and source spans when the attempt is
a recipe let; carry no statement index for an initial authored input or the reserved source.
Source arguments identify records/references; they do not copy numerical values or state.
Ordinary repeated lets retain the name and both actual recipe statement indices. Tokens,
identifier grammar, collision refusal and reserved-name population remain unchanged.

Alternative: use formula_ambiguous_name for reserved collisions and revise the reference/static
annex accordingly. That changes the established reserved-refusal token and treats a reserved
binding as an origin collision. The director delegated this choice and application to the engineer
on2026-10-03, requiring signoff and production quality; the decision follows.

### D131 delegated decision

Author and applier: the same repo engineer, under the director's explicit delegation. Adopt the
recommendation: formula_rebinding distinguishes reserved_name and recipe_name source cases.
Canonical contract5.2.1 defines truthful case-specific sources and actual available locations.
Preserving the token retains the established reserved-binding prohibition; treating it as ordinary
ambiguity would obscure that the language owns the reserved source and no authored origin can win.
An explicit case distinguishes those two causes without competing tokens or fabricated indices.

The reference carries static reserved metadata and actual metadata-pair/recipe locators; it does
not invent canonical record identities. Whole preflight retains real prior/current indices and
global whole/name spans, including assertions in statement order. Detached checking keeps its
available local spans without a fictitious recipe ordinal. No numeric state/value enters sources.
Product namespace errors will retain the existing typed source declarations and opaque default
diagnostic formatting; canonical registry validation and localization remain their own owners.

Consequences are executable: reserved_diagnostic_review.py checks3624 independent argument cases
and19 actual compiled assertion reds; existing signature/namespace/whole controls must stay green.
Engineering verification is not independent approval; governance6.1 leaves approval unclaimed
until the designated independent reviewer evaluates the decision, implementation and evidence.
The director or that reviewer may reopen this decision for a concrete diagnostic or integration
counterexample. Reversal changes contract5.2.1/reference/product variant together and replays the
same independent matrices; grammar, reserved population and collision refusal must stay aligned.
