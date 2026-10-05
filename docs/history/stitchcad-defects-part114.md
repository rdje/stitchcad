# Sealed archive — D191 concurrent directory creation

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup2` on `2026-10-05` (UTC).

- **Sealed identity:** 15 lines, 1634 bytes, `sha256:e304fc75afb829a3f3c04238540daef7c47f7d0ab648e94568b62fe835a31d93`
- **Coverage:** whole new-over7172b5329a802457ee924a99a47006a10f4f5d0f report/closure; docs/tasks/PLANNING.md authored bytes 79446–80091.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D191** — concurrent cold local-environment activation races at directory creation. Repro: two producers activate after cleanup removes target/scratch; one measured native preflight raises FileExistsError from directory() while peers create that directory. Impact: valid local producers fail before dispatch. Owner .f.cleanup2; priority current blocker.
Barrier-controlled real filesystem reproduction, idempotent creation/post-create boundary validation and standing profile controls required before closure. Barrier-controlled real filesystem reproduction also fails in scripts/ci_environment.py; both helper repairs belong to this leaf.

D191 closes in .f.cleanup2: both actual helpers fail barrier-controlled competing
creation before repair. Idempotent mkdir and immediate post-create symlink validation
preserve type/volume/Git checks before descendant creation. Eight real filesystem
cases and four actual idempotence/post-validation body faults pass in the standing
local profile suite, alongside56 runtime/13 helper/two Make/one launcher reds, real
exec and18 review children. CI19/6 and shared31/609/44/230 pass; source exact.
Native full7 and actual three-crate WASM rebuild pass after failed cold activation.
Failed native preflight is excluded. A reused test variable briefly printed3 instead
of13 compiled local reds; fresh variable separation and complete rerun print13.
No Rust/grammar or diagnostic contract changed; D156/broader D34 remain open.
Preflight full sealed bounds/one terminal newline, inventory503 and public retrieval
precede live report retirement. All prior500 immutable identities remain exact.
