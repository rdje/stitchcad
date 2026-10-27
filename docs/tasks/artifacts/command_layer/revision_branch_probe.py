"""Read-only D285 counterexample from pinned native revision/undo contracts, not a runtime undo test."""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[4]
BASELINE = '57767b01b4bffd1456162f6cb708cd1905b341a1'


def original(path):
    return subprocess.check_output(['git', 'show', BASELINE + ':' + path], cwd=ROOT).decode()


def main():
    contract = original('docs/book/src/spec/command-layer.md')
    counter = original('crates/sc-core/src/command/context.rs')
    effects = original('crates/sc-core/src/command/bus/execution.rs')
    binding = original('crates/sc-core/src/command/bus/group.rs')
    assert 'the identity of every entity and the revision counter' in contract
    assert 'Self(&self.0 + 1_u8)' in counter
    assert 'let revision = self.state.inputs().revision().successor();' in effects
    assert 'context.expected_revision() != inputs.revision()' in binding
    initial_revision = 0
    first_snapshot = ('first edit', initial_revision + 1)
    restored_revision = initial_revision
    naive_branch = ('different edit', restored_revision + 1)
    stale_precondition = first_snapshot[1]
    assert first_snapshot != naive_branch and stale_precondition == naive_branch[1]
    maximum_issued = first_snapshot[1]
    fresh_branch_revision = maximum_issued + 1
    assert fresh_branch_revision > maximum_issued and stale_precondition != fresh_branch_revision
    print('D285 baseline57767b0 contract model: distinct snapshots reuse revision1; old precondition matches')
    print('separate maximum-issued counter: branch revision2; old revision1 refuses; rc=0')
    print('scope: counterexample and proposed issuance rule only; native undo/branch acceptance is separate')


if __name__ == '__main__':
    main()
