# The conformance matrix

Every feature this project claims is listed here beside the check that would fail if the claim were false. A claim with no such check is listed as unclaimed, so nothing is implied by leaving it out.

```text
$ cargo test -p sc-cli --test conformance_matrix
```

That command holds this page to the repository: each check named here exists, and every clause the [gauge](../availability.md) measures through G2 has a row or is listed as unclaimed.

## How much a check can prove

Roadmap §13 ranks evidence, and a higher layer cannot be replaced by more of a lower one.

| Layer | What it is |
| --- | --- |
| 1 | Native invariants and analytic geometry: exact answers worked out independently of the code |
| 2 | Independently produced fixtures with a documented meaning |
| 3 | Differential inspection, by a separate reader or renderer |
| 4 | Import through the actual target product, with recorded options |
| 5 | Printed, cut and sewn checks on real material |

Nothing in this project has reached layer 4 or 5 yet.

## What is claimed

| Claim | Clause | Layer | The oracle | Check |
| --- | --- | --- | --- | --- |
| Lengths and angles are exact, and the same on every platform | — | 1 | Frozen bits, and a census of forbidden calls | `cargo test -p sc-units --test determinism` |
| Three runtime profiles compile | `G1-01` | 1 | Real builds, the browser's for real | `bash scripts/check_profiles.sh` |
| A browser gives the native program's bytes | `G1-02` | 3 | Chrome across two processes, compared with the native program | `bash docs/tasks/artifacts/browser_run/run_browser_run.sh` |
| The canvas is decided by measurement | `G1-03` | 3 | The rule written before the measurement | `bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh` |
| Every edit goes through the command bus | `G1-04` | 1 | The real program, with a refused stale revision | `cargo test -p sc-cli --test session` |
| A save survives a crash | `G1-05` | 1 | Crash at every step, and real processes killed | `cargo test -p sc-store --test project_kill` |
| Licenses are compatible | `G1-06` | 1 | Each dependency's own stated license | `bash scripts/check_licenses.sh` |
| The browser matrix matches a real run | `G1-07` | 3 | The recording of that run | `bash docs/tasks/artifacts/browser_run/run_capability_matrix_check.sh` |
| The reference skirt drafts to the book's numbers | `G2-01` | 2 | [The chapter's own numbers](reference-skirt.md), derived separately, and frozen goldens | `cargo test -p sc-cli --test skirt` |
| A viewer and editor use the same commands | `G2-02` | 3 | What the page records, replayed by the native program to the same bytes | `bash docs/tasks/artifacts/dev_shell/run_dev_shell_check.sh` |
| The DXF holds what it should | `G2-03` | 3 | A reader in the test that shares no code with the writer. No other reader has seen it | `cargo test -p sc-artifacts --test dxf` |
| The PDF is dimensionally correct | `G2-03` | 3 | xpdf and macOS CoreGraphics, measured in pixels | `bash docs/tasks/artifacts/printed_square/run_printed_square.sh` |
| Offsets pass the pathology corpus | `G2-05` | 1 | Checks sharing no code with the engine | `cargo test -p sc-geometry --test offset_corpus` |
| Moving, turning and mirroring a piece does what it should | `G2-06` | 1 | The same offset, transformed | `cargo test -p sc-geometry --test metamorphic` |
| A project reopens to the same bytes, with its intent | `G2-06` | 1 | The frozen goldens, and two designs that diverge | `cargo test -p sc-cli --test save_load` |
| A scoped caller cannot exceed its authority, and an agent cannot approve | — | 1 | The real server through real pipes, where the bus itself refuses | `cargo test -p sc-mcp --test admission` |
| An agent with only MCP and the docs drafts the skirt | `G2-08` | 3 | Two recorded sessions, replayed by the command line, read by the judge, and compared with the golden | `cargo test -p sc-mcp --test agent_gate` |
| A cap meets its armhole by a declared ease, and the same difference undeclared is a defect | `G3-01` | 1 | The bodice and sleeve at five sizes, with each curve also measured from its control points | `cargo test -p sc-cli --test bodice` |
| A reference stays on the same cloth through an edit, or says it cannot | `G3-03` | 1 | Every edit against every carrier, with a stated seed, comparing where each one lands | `cargo test -p sc-cli --test references` |
| Darts, folds and seams work as operations, with undo | `G3-02` | 1 | Lengths worked out by hand, a closed dart landing on itself, and a seam re-walked after truing | `cargo test -p sc-cli --test walk` |
| A fault is caught for the right reason | `G2-07` | 1 | Each injected fault's named code | `cargo test -p sc-cli --test mutants` |

## What is unclaimed

| Claim | Clause | Why it has no check yet | What would give it one |
| --- | --- | --- | --- |
| A printed scale square measures true on paper | `G2-04` | Only a person with a ruler can do it, and this measures pixels so far | [Five minutes with a ruler](physical-evaluation.md) |
| A CAD system opens these DXF files | — | No seat exists, and the director has parked this until StitchCAD is feature complete | A real receiver, at G6 |
| The garment fits and sews | — | Nothing has been cut. Nobody who sews has reviewed the corner shapes or the waist curve | [A first toile](physical-evaluation.md) and a domain reviewer |
| The canvas verdict holds on Windows and Linux | — | Only macOS was measured | A machine with a display |
| All of this on Windows | — | The determinism job below has not run, and no Windows machine exists here | The first hosted run |

## The determinism policy

The same design gives the same bytes on every platform, compiler, locale and time zone. A golden file is only useful if this holds, because otherwise it fails on the platform nobody tested. Each rule below has what enforces it.

| Rule | Why | Enforced by |
| --- | --- | --- |
| No platform math on any product path | `hypot`, `sin`, `cos` and `atan2` come from each platform's library, and IEEE 754 does not pin them down | A census of every crate's source, in `cargo test -p sc-units --test determinism` |
| Geometry uses only `sc_units::pure` for those | It is built only from operations IEEE 754 requires to be correctly rounded | Frozen bits of its answers, checked on each operating system |
| Numbers are written from integers, never by formatting a float | A float's text form differs by library | The writers' own arithmetic, and the frozen goldens |
| Nothing on the output path reads a clock, chance, the environment or a hash order | Any of those makes two runs differ | The same census |
| One written order | Pieces by name, marks sorted, a closed line from its lowest point | A test rotating a line's start and shuffling pieces, for each writer |
| No date, producer or file identifier in a file | Two runs must be one byte string | A test forbidding each, for each writer |
| Locale and time zone change nothing | A number written with a comma would be a different file | The real program under foreign settings |
| A checkout changes no golden file | Windows would otherwise rewrite the line endings | `.gitattributes` for everything in `conformance/` |
| A local build reflects the local source | A harness that copied the workspace left cargo watching the copy (D400) | `bash scripts/check_build_fingerprints.sh`, before every run of the gauge's checks |

**Where it runs.** `.github/workflows/determinism.yml` runs these on Linux, macOS and Windows. **It has not run yet,** because it first runs at the next push, and only this Mac has run any of it. Real Chrome running the WebAssembly build is the second platform that has: [the dev shell's check](dev-shell.md) requires the same bytes as the native program.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `sc_units::pure` | Trigonometry and lengths from IEEE-exact operations |
| `hypot` | A vector's length, from a platform's math library |
| `sin` | A sine, from a platform's math library |
| `cos` | A cosine, from a platform's math library |
| `atan2` | An angle, from a platform's math library |
| `.gitattributes` | Git's per-file settings |
| `.github/workflows/determinism.yml` | The job that runs this on three operating systems |
| `G1-01` | A row of [the gauge](../availability.md) |
| `G1-02` | A row of [the gauge](../availability.md) |
| `G1-03` | A row of [the gauge](../availability.md) |
| `G1-04` | A row of [the gauge](../availability.md) |
| `G1-05` | A row of [the gauge](../availability.md) |
| `G1-06` | A row of [the gauge](../availability.md) |
| `G1-07` | A row of [the gauge](../availability.md) |
| `G2-01` | A row of [the gauge](../availability.md) |
| `G2-02` | A row of [the gauge](../availability.md) |
| `G2-03` | A row of [the gauge](../availability.md) |
| `G2-04` | A row of [the gauge](../availability.md) |
| `G2-05` | A row of [the gauge](../availability.md) |
| `G2-06` | A row of [the gauge](../availability.md) |
| `G2-07` | A row of [the gauge](../availability.md) |
| `G2-08` | A row of [the gauge](../availability.md) |
| `G3-01` | A row of [the gauge](../availability.md) |
| `G3-02` | A row of [the gauge](../availability.md) |
| `G3-03` | A row of [the gauge](../availability.md) |
