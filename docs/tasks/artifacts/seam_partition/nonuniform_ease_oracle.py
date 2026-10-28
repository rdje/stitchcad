"""Independent exact nonuniform allocation diagnostic; no project arithmetic imports.

Default output is the frozen native-input TSV. --domains emits original-domain query rows.
--check compares both fixtures without writing files. Native consumers check actual source
admission/refusals and original weighted quantities against the independently derived results.
The workspace bound is a constructive arithmetic bound, not an allocator/CPU certificate.
"""
from fractions import Fraction as F
from math import prod
from pathlib import Path
import sys


def quantize(value):
    whole, remainder = divmod(abs(value.numerator), value.denominator)
    rounded = whole + (2 * remainder >= value.denominator)
    return -rounded if value < 0 else rounded


def cumulative(regions, at):
    total = sum((end - start) * weight for start, end, weight in regions)
    prefix = sum(max(F(0), min(at, end) - start) * weight
                 for start, end, weight in regions)
    return prefix / total


def workspace_bound(regions, at):
    # Each stored endpoint/positive weight has <=63-bit positive fields. A mass a_i/d_i
    # has unreduced fields <2^189. D=product(d_i), T=sum(a_i*D/d_i) <N*2^(189N).
    # With query denominator Q, prefix=P/(D*Q), 0<=P<=T*Q; CDF=P/(T*Q).
    count = len(regions)
    bound = 189 * count + (count - 1).bit_length() + at.denominator.bit_length()
    exact = cumulative(regions, at)
    # Independent common-denominator construction checks the bound's algebra, rather than
    # inferring feasibility solely from Fraction's reduced final result.
    masses = [((end.numerator * start.denominator - start.numerator * end.denominator)
               * weight.numerator, end.denominator * start.denominator * weight.denominator)
              for start, end, weight in regions]
    denominator = prod(d for _, d in masses)
    total = sum(n * (denominator // d) for n, d in masses)
    prefix = 0
    for (start, end, weight), (n, d) in zip(regions, masses):
        if at >= end:
            prefix += at.denominator * n * (denominator // d)
        elif at > start:
            prefix += ((at.numerator * start.denominator - start.numerator * at.denominator)
                       * weight.numerator * (denominator // (start.denominator * weight.denominator)))
    assert F(prefix, at.denominator * total) == exact
    assert 0 <= prefix <= at.denominator * total
    assert total.bit_length() <= 189 * count + (count - 1).bit_length()
    assert (at.denominator * total).bit_length() <= bound
    assert max(exact.numerator.bit_length(), exact.denominator.bit_length()) <= bound
    return bound


def render():
    maximum = (1 << 63) - 1
    wide = tuple((F(i, 4), F(i + 1, 4), F(1, maximum - k))
                 for i, k in enumerate((2, 4, 6, 8)))
    exact = cumulative(wide, F(1, 2))
    assert exact.denominator.bit_length() == 188 and exact < F(1, 2)
    binary64 = sum(float((end - start) * weight) for start, end, weight in wide[:2]) / sum(
        float((end - start) * weight) for start, end, weight in wide)
    assert binary64 == 0.5 and quantize(exact) == 0 and quantize(F(binary64)) == 1
    # Apply direction before quantization: parent-round(parent*F_B) spends the tie on
    # the wrong A-ordered child, including the signed mirror of the same example.
    for parent in (-1, 1):
        assert quantize(parent * (1 - F(1, 2))) == parent
        assert parent - quantize(parent * F(1, 2)) == 0
    # Illustrative lengths, not measured geometry: equal normalized cuts would allocate
    # 250um but create a 500um physical differential. Required B progress is21/40.
    asymmetric = ((F(0), F(1, 2), F(1)), (F(1, 2), F(1), F(3)))
    allocated = 1000 * cumulative(asymmetric, F(1, 2))
    matching_b = (11000 * F(1, 2) - allocated) / 10000
    assert allocated == 250 and matching_b == F(21, 40) and matching_b != F(1, 2)
    queries = [("wide", wide, F(1, 2)),
               ("density", asymmetric, F(1, 2)),
               ("uniform-weight", ((F(0), F(1), F(7, 3)),), F(1, 2))]
    gap = ((F(1, 4), F(3, 4), F(3, 2)),)
    queries += [("gap", gap, at) for at in (F(0), F(1, 4), F(1, 2), F(3, 4), F(1))]
    lines = ['# case\tparent_um\tside\tdirection\tat_n\tat_d\tregions\tcdf_n\tcdf_d\tfirst_um\tsecond_um']
    for name, regions, at in queries:
        workspace_bound(regions, at)
        encoded = ';'.join(f'{start}:{end}:{weight}' for start, end, weight in regions)
        for side, direction in (("A", "Same"), ("A", "Opposite"),
                                ("B", "Same"), ("B", "Opposite")):
            progress = cumulative(regions, at)
            if side == "B" and direction == "Opposite":
                progress = 1 - progress
            for parent in (-1_000_000_000, -1, 0, 1, 1_000_000_000):
                first = quantize(parent * progress)
                second = parent - first
                assert first + second == parent and first == -quantize(-parent * progress)
                lines.append('\t'.join(map(str, (name, parent, side, direction, at.numerator,
                             at.denominator, encoded, progress.numerator, progress.denominator,
                             first, second))))
    return '\n'.join(lines) + '\n'


def render_domains():
    root = Path(__file__).resolve().parents[4]
    selected = {'partial-a', 'partial-b', 'outside-low', 'outside-high',
                'cancelled-small-frame', 'cancelled-near-one',
                'final-denominator-overflow', 'final-two-component-overflow'}
    cases = [row.split('\t') for row in
             (root / 'crates/sc-core/tests/fixtures/range_progress.tsv').read_text().splitlines()
             if row and not row.startswith('#') and row.split('\t')[0] in selected]
    assert {row[0] for row in cases} == selected
    maximum = (1 << 63) - 1
    for name, numerator in (('wide-below-half', (maximum - 1) // 2),
                            ('wide-above-half', (maximum + 1) // 2)):
        cases.append([name, '1', str(maximum - 2), str(numerator), str(maximum),
                      str(maximum - 2), str(maximum - 1), 'independent'])
    lines = ['# case\tparent_um\tfrom_n\tfrom_d\tat_n\tat_d\tto_n\tto_d\tcdf_n\tcdf_d\tamount_or_outside']
    for name, fn, fd, an, ad, tn, td, _ in cases:
        start, at, end = F(int(fn), int(fd)), F(int(an), int(ad)), F(int(tn), int(td))
        progress = (at - start) / (end - start)
        assert max(progress.numerator.bit_length(), progress.denominator.bit_length()) <= 189
        if name.startswith('wide-'):
            assert progress.denominator.bit_length() == 189 and float(progress) == 0.5
            assert (progress < F(1, 2)) == (name == 'wide-below-half')
        for parent in (-1_000_000_000, -1, 0, 1, 1_000_000_000):
            result = quantize(parent * progress) if start <= at <= end else 'outside'
            lines.append('\t'.join(map(str, (name, parent, fn, fd, an, ad, tn, td,
                                           progress.numerator, progress.denominator, result))))
    return '\n'.join(lines) + '\n'


if __name__ == '__main__':
    output = render()
    if sys.argv[1:] == ['--check']:
        root = Path(__file__).resolve().parents[4]
        assert (root / 'crates/sc-core/tests/fixtures/nonuniform_ease.tsv').read_bytes() == output.encode()
        assert (root / 'crates/sc-core/tests/fixtures/weighted_domain_ease.tsv').read_bytes() == render_domains().encode()
        print('nonuniform oracle:160 rows;Fraction/common-denominator agreement;'
              '188-bit density denominator;50 original-domain rows/189-bit progress;'
              'float/orientation/equal-cut controls;rc=0')
    elif sys.argv[1:] == ['--domains']:
        print(render_domains(), end='')
    elif not sys.argv[1:]:
        print(output, end='')
    else:
        raise SystemExit('usage: nonuniform_ease_oracle.py [--check|--domains]')
