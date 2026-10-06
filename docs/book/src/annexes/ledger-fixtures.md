# Ledger pointer fixtures

The changelog retains older records through links. A label that names a retained
record is insufficient: the link must resolve to that record, either directly or
through its catalog anchor. The pointer fixtures execute the public ledger verdict
function extracted from `docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh`.
They preserve its native Bash, archive reader and pointer helper behavior.

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
These controls pass on Python 3.9 and 3.14. The separate source-mutation producer
remains owned by the next task; this entry does not run tracked-source faults.

See [Governance](../governance.md) for the historical-reader and retention contract,
and [Build and checks](../build-and-checks.md) for the broader project gates.
