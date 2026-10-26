"""Historical D239 transition witness for the 22ab074 contract; no product approval or hash choice.

All objects are synthetic fixtures. This illustrates post-hash mutation, detached approval metadata,
and metadata fixed before the final hash. It does not validate or generate a release package.
"""
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = 'docs/tasks/artifacts/release_contract/approval_identity_probe.py'
LEGACY_FIELDS = (
    'design_revision', 'inputs', 'resolved_profile_versions', 'sizes_and_pieces',
    'engine_and_exporter_config', 'artifact_hashes', 'validation_results',
    'unresolved_assumptions', 'approver_scope_date',
)


def encoded(value):
    """A fixed diagnostic serialization, not the product canonicalizer."""
    return json.dumps(value, sort_keys=True, separators=(',', ':')).encode('utf-8')


def digest(value):
    return hashlib.sha256(encoded(value)).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def witness():
    candidate = {field: {'synthetic_fixture': field} for field in LEGACY_FIELDS}
    candidate['approver_scope_date'] = {'state': 'unresolved'}
    human_metadata = {'human': 'fixture_human', 'scope': 'fixture_scope', 'date': '2000-01-01'}
    before = encoded(candidate)
    before_digest = digest(candidate)

    filled = deepcopy(candidate)
    filled['approver_scope_date'] = human_metadata
    require(encoded(filled) != before, 'fixture did not change manifest bytes')
    require(digest(filled) != before_digest, 'fixture did not change diagnostic digest')
    require(all(filled[field] == candidate[field] for field in LEGACY_FIELDS
                if field != 'approver_scope_date'), 'more than the approval field changed')

    # Proposal A uses a stable external-binding descriptor; no self digest is put inside its subject.
    detached_subject = deepcopy(candidate)
    detached_subject['approver_scope_date'] = {'binding': 'detached_records_for_this_package'}
    detached_before = encoded(detached_subject)
    detached_digest = digest(detached_subject)
    detached_record = {'subject_digest': detached_digest, 'metadata': human_metadata}
    require(b'fixture_human' not in detached_before, 'human metadata leaked into detached subject')
    require(encoded(detached_subject) == detached_before, 'detached proposal mutated its subject')
    require(detached_record['subject_digest'] == digest(detached_subject), 'detached subject mismatch')

    # Proposal B fixes prospective metadata before its final candidate hash and later confirmation.
    prebound_subject = deepcopy(filled)
    prebound_before = encoded(prebound_subject)
    prebound_digest = digest(prebound_subject)
    confirmation = {'subject_digest': prebound_digest, 'metadata': deepcopy(human_metadata)}
    require(encoded(prebound_subject) == prebound_before, 'prebound proposal mutated its subject')
    require(confirmation['subject_digest'] == digest(prebound_subject), 'prebound subject mismatch')

    return {
        'classification': 'synthetic_transition_witness_only',
        'legacy_source_revision': '22ab074',
        'manifest_fields': len(LEGACY_FIELDS),
        'fixture_hash': 'sha256_not_a_product_algorithm_decision',
        'real_approval_performed': False,
        'proposal_adopted_by_probe': False,
        'post_hash_fill': {'bytes_changed': True, 'digest_changed': True,
                           'before_digest': before_digest, 'after_digest': digest(filled)},
        'detached_proposal': {'subject_unchanged': True, 'record_binds_exact_subject': True,
                              'manifest_contains_binding_not_human_values': True},
        'prebound_proposal': {'subject_unchanged_at_confirmation': True,
                             'prospective_metadata_must_be_fixed_before_final_hash': True},
    }


def main():
    profile = runpy.run_path(str(ROOT / 'scripts/local_environment.py'))
    profile['enter_producer'](ROOT, sources=(SOURCE,))
    if len(sys.argv) != 1:
        print('usage: python3 -I -B ' + SOURCE, file=sys.stderr)
        return 2
    print(json.dumps(witness(), indent=2, sort_keys=True))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
