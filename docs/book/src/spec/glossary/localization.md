# Glossary: localization

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Every term here is specified by the
> [internationalization](../i18n-architecture.md) chapter.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| externalization lint | the CI rule that fails the build on an inline user-facing string, with a closed list of exemptions each carrying a reason | [i18n §5](../i18n-architecture.md) | string lint, hardcoded-copy check | — |
| locale-independent file | a canonical file whose meaning does not depend on the locale that reads it: internal integers, no localized separator, no locale tag inside a value | [i18n §6](../i18n-architecture.md) | canonical file, locale-free file | — |
| message id | the identifier a catalogue entry is looked up by, which is the diagnostic's own stable token and never a number or an English slug | [i18n §3](../i18n-architecture.md) | message key, l10n id | — |
| message system | the one format and library both ends use for localized text; chosen at G0 so no bridge is needed | [i18n §2](../i18n-architecture.md) | l10n framework, i18n library | — |
| pseudolocalization ⚠ | a build mode rendering every message expanded and accented, so a hardcoded string, a truncation or a concatenation shows up before a translator is paid | [i18n §7](../i18n-architecture.md) | pseudo-locale, accented build, *Pseudolokalisierung* | — |
| review tier | how much review a message needs before a language pack may ship: `strict`, `safety` or `standard` | [i18n §9](../i18n-architecture.md) | importance class, review threshold | — |
| termbase | the glossary projected into one language: term, translation, ⚠ mark, machine token and the reviewer who approved it | [i18n §4](../i18n-architecture.md) | terminology base, translation glossary, *Terminologieliste* | — |
