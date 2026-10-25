# Ledger pointer fixtures

The changelog retains older records through links. A label that names a retained
record is insufficient: the link must resolve to that record, either directly or
through its catalog anchor. The pointer fixtures execute the public ledger verdict
function extracted from `docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh`.
They preserve its native Bash, archive reader and pointer helper behavior.

| Tooling name | Role |
| --- | --- |
| `python3` | Host Python interpreter command |
| `HISTORY_ARCHIVE_ROOT` | Local archive authority selected for the fixture |
| `AssertionError` | Python assertion failure; its presence alone does not prove a POINTER fault |

Run the thirteen independent examples from the repository root:

```bash
python3 -I -B docs/tasks/artifacts/changelog/ledger_pointer_contract.py
python3 -I -B docs/tasks/artifacts/changelog/ledger_pointer_contract.py --mask-first
```

The masked run moves the wrong-target example first. It uses the same cases and
expected verdicts; this helps expose a checker that stops after one successful
pointer. Both modes print `13 independent actual verdicts / 0 fail` on success.

Examples include a raw record link with an arbitrary label, a catalog link with a
short or incorrect label, and a catalog link with the full record name. Those pass
when the target is valid. Label-only prose, a filename in a code span, a wrong
target, missing catalog or anchor, and a missing raw record produce one named
`POINTER FAIL`. A valid pointer beside an invalid target also fails. Fenced or
commented pseudo-indexes do not establish a pointer.

## Fixture authority and lifecycle

Before reading the shell function or activating stores, the entry checks all six
entry/helper/profile/reader sources and the complete physical archive backing
inputs. The archive reader validates its whole backing store before returning
logical part24, so declaring only that logical record would miss dependencies.
An existing `HISTORY_ARCHIVE_ROOT` is preserved when it satisfies the same local
authority contract. An archive authority inside the fixture workspace is refused.

The full destination population is known before effects: the scratch parent and
workspace, thirteen numbered fixture directories with `docs/history`, and four
files per fixture: the retained record, `CHANGELOG.md`, `order.txt`, and
`verdict.log`. The workspace is `target/scratch/ledger_pointer_contract`.
Existing entries must belong to this finite population. Unknown caller files,
symlink destinations, hard-linked output files, wrong kinds, foreign Git markers
and invalid store settings are refused before activation or old-tree removal.

Each creation, write, child invocation and removal rechecks its relevant authority.
The archive read and native verdict children inherit prepared project-local stores
and caller settings. Read-only source declarations grant no source mutation.
Workspace setup, the archive child and all fixture cases share protected teardown.
An archive-child failure therefore removes the owned workspace before propagating
the failure. Removal revalidates the complete bounded tree and requires the
runtime's symlink-safe tree remover.

## Independent evidence

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_ledger_fixture_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_ledger_fixture_contract.py --real
```

The default controls execute the original pre-repair failure in a private mirror,
capture complete plans and effects, inject seven actual omissions, and exercise
twelve physical CLI refusals. Captured child responses establish dispatch and
environment handling. `--real` separately runs both modes through native children,
checks all source and archive bytes, and requires zero fixture residue. The
complete original case table, permutation, extraction, arguments, logs and
assertions are also compared through the declared guard and teardown adapters.
These controls pass on Python 3.9 and 3.14.

## Source mutation evidence

```bash
python3 -I -B docs/tasks/artifacts/changelog/ledger_pointer_mutations.py
```

Run this producer exclusively: it temporarily changes the ledger shell and pointer
helper, then restores both sources. It validates the complete seven sources and
physical archive backing inputs, both mutable source authorities, its workspace
and four log destinations before reading snapshots or activating stores. Fresh
plans precede source writes, syntax checks, contract children, log writes, source
restoration, restored reads and the final baseline. Child arguments retain the
original `bash -n` and `python3 -I -B` interfaces and inherit prepared stores.

The four original faults require these specific contract failures:

| Fault | Required case | Required mismatch |
| --- | --- | --- |
| Filename text replaces link targets | Catalog target, short label | A valid pointer fails |
| An unregistered catalog route is accepted | Missing catalog | An invalid pointer passes |
| Invalid extra target status is ignored | Valid pointer plus invalid target | An invalid pointer passes |
| A code fence is mistaken for an index | Fenced pseudo-index | An invalid pointer passes |

An arbitrary assertion is insufficient. The response must identify the original
`assert (not failures) == expected` site, the expected named case and polarity,
and its five-field evidence tuple. Failure lines must agree with captured stdout,
belong to the POINTER family and contradict the expected verdict. This repairs
D216: the original classifier accepted unrelated assertions containing a filename
and `AssertionError` token. The entry preserves its original exit/status/source
checks and adds this stricter evidence check.

The loop's protected restoration runs after a syntax-child, write or classifier
failure. Each restoration write rechecks mutable authority. Four logs remain at
`target/ledger_pointer_mutations/mutation-1.log` through `mutation-4.log`. A
successful run restores exact source bytes and finishes with all thirteen native
baseline verdicts; the contract workspace is absent afterward.

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_ledger_mutation_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_ledger_mutation_contract.py --real
```

Default controls capture 27 complete plans, 15 effect calls and nine prepared
children, reproduce the original false admissions, and verify two interrupted
restorations, six unrelated-red refusals, ten actual omission reds and thirteen
physical CLI refusals. Their simulated child responses prove dispatch and
classification. `--real` independently runs all four native faults exclusively,
checks the expected assertion evidence, and verifies restored baseline/source and
archive bytes. Both caller versions, Python 3.9 and 3.14, pass; the native child
command remains `python3`. Full original bodies, edits, anchors, logs and arguments
remain exact through declared plan, environment and classifier adapters.

See [Governance](../governance.md) for the historical-reader and retention contract,
and [Build and checks](../build-and-checks.md) for the broader project gates.

## Diagnostic record continuity

The current handoff suite also verifies the complete retained D217 diagnostic records.
It obtains logical devnotes parts303/304 and changelog part208 through the public archive
reader. A packing window may remove their physical Markdown files; their logical names
and full bytes remain the interface. D222 repaired the former raw-file dependency.

```bash
python3 -I -B docs/tasks/artifacts/handoff/d217_probe.py retained
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_client_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_client_contract.py --real
```

Before each child, the client declares the complete physical archive backing population,
the required master control, its own source, the local profile and both reader sources.
Symlink inputs, wrong source kinds, nested Git boundaries, escaping authorities and invalid
store settings refuse before dispatch. A valid local `HISTORY_ARCHIVE_ROOT` and prepared
stores survive; children have Git context bound to this checkout. This read-only check
creates no stores, fixture directories or output files.

The proof still compares five complete task records, the complete diagnostic followup
and eight whole ledger entries with their original Git sources. It also runs the original
window11 source comparison. Those provenance checks require the referenced Git objects;
ordinary archive retrieval remains independent of historical Git.

The independent controls first pack the three records in a private valid archive and
remove their raw copies. The original client refuses that fixture; the repaired client
passes with exact original source assertions, extraction, child arguments and result fields.
Seven physical refusals, three same-length content changes and seven actual compiled body
faults test complete input plans, fresh checks, Git scope, logical reads and provenance.
The native mode separately executes all eight children against the packed fixture and
requires exact source/archive/fixture bytes and no workspace residue. Python3.9 and3.14
pass. The controls create no production archive window and modify no committed window.
