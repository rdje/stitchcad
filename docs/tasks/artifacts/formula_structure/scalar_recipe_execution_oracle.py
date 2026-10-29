"""Finite whole scalar recipe rows from the existing independently parsed book evaluator.

Trace real value reads and pre-storage operands, then invoke the unchanged reference methods.
No Rust implementation, native results or frozen expected file is read. Pure guarded stdout.
Missing-family/Derived evaluation, physical geometry, load/replay and platform proof stay outside.
"""
from fractions import Fraction
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
HELPER = ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), HELPER, PROFILE_SOURCE))
CONTRACT = runpy.run_path(str(HELPER))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), *CONTRACT['REFERENCE_INPUTS']))
CASES = (
    ('', None),
    ('let a:length=1 mm/3\nlet b:length=a+a\nassert check:eps_num=b==666 um', None),
    ('let first:length=1 um/3\nlet half:ratio=0.000003/2\nlet flag:boolean=half<0.0\n'
     'assert prior:eps_num=first==0 um\nlet second:length=if(flag,eps_phys,first+1 um)\n'
     'assert final:eps_num=second==1 um', None),
    ('let sweep:angle=-720 deg\nlet scaled:angle=sweep/3\nassert check:eps_num=scaled==-240 deg', None),
    ('let count:count=1/2.0\nlet next:count=count+1\nassert check:eps_num=next==2', None),
    ('let a:length=min(0 um,round_to(1 um,1 um))\n'
     'let b:length=clamp(a,0 um,round_to(1 um,1 um))\nassert check:eps_geo=b==0 um', None),
    ('let a:ratio=sin(90 deg)\nlet flag:boolean=a>0.0\nlet output:length=if(flag,1 um,eps_phys)\n'
     'let zero:length=output-output\nassert check:eps_geo=zero==0 um', None),
    ('let angle:angle=atan(1.0)\nlet length:length=arc_length(angle,0 um)\nassert check:eps_geo=length==0 um', None),
    ('let a:length=if(1==1,1 um,1 um/0)\nlet b:length=if(a==1 um,a,eps_phys)', None),
    ('let inside:boolean=within(1 um,2 um,eps_num)\nlet value:length=if(inside,1 um,eps_phys)', None),
    ('let first:count=1\nassert prior:eps_num=first==1\nlet last:count=first+1\nassert check:eps_num=last==2', None),
    ('let a:length=1 um\nassert named:eps_num=a==3 um', 'formula_assertion'),
    ('let prior:length=1 um\nlet huge:count=9223372036854775807*2\nlet later:length=eps_phys', 'formula_domain'),
    ('let a:ratio=sin(90 deg)\nassert named:eps_num=a==1.0', 'formula_domain'),
    ('let a:length=1 mm\nassert named:eps_phys=a/0==2 mm', 'formula_division'),
    ('let a:length=1 mm\nassert named:eps_phys=a==2 mm', 'formula_tolerance_unbound'),
    ('let a:count=1\nlet b:length=if(a==1,eps_fmt,1 um)', 'formula_tolerance_unbound'),
    ('let a:length=1 um\nlet b:length=sqrt(-(a^2))', 'formula_domain'),
)


def scalar(value):
    if value.kind == 'boolean':
        payload = 'true' if value.v else 'false'
    else:
        exact = Fraction(value.v)
        payload = '%d/%d' % (exact.numerator, exact.denominator)
    return ':'.join((value.kind, payload, ','.join(sorted(value.sources)) or '-'))


def fixture_rows():
    namespace, reference = CONTRACT['load_reference']()
    original_read, original_stored = reference.value_of_name, reference.stored
    reads, storage_inputs = [], []
    def traced_read(name, environment):
        value = original_read(name, environment)
        reads.append(name + ':' + scalar(value))
        return value
    def traced_storage(value, operation):
        storage_inputs.append(value)
        return original_stored(value, operation)
    reference.value_of_name, reference.stored = traced_read, traced_storage
    yield '# recipe with literal \\n separators\tcomplete outcomes or actual refusal\texecuted reads'
    for source, expected in CASES:
        plan = reference.preflight(source, ())
        environment, outcomes = {}, []
        reads.clear(); storage_inputs.clear()
        error = None
        for ordinal, (start, end, checked) in enumerate(plan, 1):
            try:
                returned = reference.statement(source[start:end], environment)
            except namespace['FErr'] as refusal:
                assert refusal.token == expected, (source, expected, refusal)
                error = 'error:%d:%s:%s' % (ordinal, checked[1], refusal.token)
                break
            if checked[0] == 'let':
                _, name, kind, value, _ = returned
                outcomes.append('%d:let:%s:%s>%s' % (ordinal, name, scalar(storage_inputs[-1]), scalar(value)))
                environment[name] = dict(kind=kind, origin='recipe', state='derived', value=value.v, sources=value.sources)
            else:
                _, name, holds, left, right = returned
                assert holds is True and reads[-1].startswith(checked[2] + ':'), 'actual assertion header read last'
                outcomes.append('%d:assert:%s:%s:%s==%s' % (ordinal, name, checked[2], scalar(left), scalar(right)))
        if error is not None:
            yield '\t'.join((source.replace('\n', r'\n'), error, '-'))
        else:
            assert expected is None, (source, 'intended refusal missing', expected)
            yield '\t'.join((source.replace('\n', r'\n'), ';'.join(['ok', *outcomes]), ';'.join(reads) or '-'))


if __name__ == '__main__':
    print('\n'.join(fixture_rows()))
