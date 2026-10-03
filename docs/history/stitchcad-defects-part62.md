# Sealed archive — D144 original source-location report

Immutable historical segment, sealed by leaf `G1-SLICE.5f.3a.t1` on `2026-10-03` (UTC).

- **Sealed identity:** 9 lines, 898 bytes, `sha256:dd44096f972e480e0a821060ec8b9df60abdc14014d03e90694a485280100967`
- **Coverage:** D144 original source-location report; original payload unchanged from dc8d7888bc46476d25b6f9ef1c0a96ca89de5e20.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D144** — reference loader strips85 shell-header lines before compiling with the original shell
  filename. Tool comparison finds resolve_geometry.__code__.co_firstlineno=690 while its real source
  begins at775 (offset85); actual D140 failure traceback displayed unrelated source text at that line.
  Reproduce load_reference/code-object line versus original def line; rc=0. Impact: misleading
  source-location diagnostics during reference failures, not changed numerical or static semantics.
  Root: static_signature_contract.py compiles the extracted prefix without preserving source offset.
  Owner G1-SLICE.5f.3a.t1, P1 immediately after D140 before the product expression checker. Preserve
  original line positions for the normal reference and identify in-memory fault sources honestly;
  independent source-location assertions, original byte integrity and full reference regressions.
