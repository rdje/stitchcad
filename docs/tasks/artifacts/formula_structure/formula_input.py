"""D76: independently constructed machine-input controls for the actual reference parser."""
from pathlib import Path
from fractions import Fraction
import os
import re
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/formula_input'
STRUCTURE = ROOT / 'docs/tasks/artifacts/formula_structure/formula_structure.py'
TOOL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'


def load_context():
    namespace, reference = runpy.run_path(str(STRUCTURE))['load_reference']()
    grammar = namespace['sections'](str(ROOT / 'docs/book/src/spec/formula-language/grammar.md'))
    units = {}
    for row in namespace['table_in'](grammar['2.1'], 'Unit token')[1]:
        token = namespace['debacktick'](row[0])
        match = re.fullmatch(r'([×÷])\s*([0-9 ]+)', row[2].strip())
        assert match is not None, ('canonical unit ratio changed', row)
        amount = Fraction(int(match.group(2).replace(" ", "")))
        factor = amount if match.group(1) == '×' else 1 / amount
        units[token] = (row[1], factor * (1000000 if row[1] == 'ratio' else 1))
    assert set(units) == {'um', 'mm', 'cm', 'm', 'in', 'deg', 'pct'}
    reference.units = units
    # Minimal authored context for valid statement-position controls, not a product environment.
    reference.bindable = ['length', 'angle', 'area', 'ratio', 'count', 'boolean']
    reference.reserved = {'eps_num': ('length', True, Fraction(1))}
    return namespace, reference


def load_arithmetic_context():
    """Table-driven numeric fixture context only; never execute a contract as an import."""
    namespace, reference = load_context()
    grammar = namespace['sections'](str(ROOT / 'docs/book/src/spec/formula-language/grammar.md'))
    # Load only published dimension signatures. This is test context, not another evaluator/parser.
    for left, right, product, quotient in namespace['table_in'](grammar['5.1'], 'Left')[1]:
        left, right = namespace['debacktick'](left), namespace['debacktick'](right)
        product, quotient = namespace['debacktick'](product), namespace['debacktick'](quotient)
        product = None if product == '—' else product
        quotient = None if quotient == '—' else quotient
        reference.pairs[left, right] = {'*': product, '/': quotient}
        if product:
            reference.pairs.setdefault((right, left), {})['*'] = product
    reference.sigs = {'param_at': [(['edge', 'length'], False, 'ratio')],
                      'round_to': [(['T', 'T'], False, 'T')],
                      'sqrt': [(['area'], False, 'length'), (['ratio'], False, 'ratio')],
                      'hypot': [(['length', 'length'], False, 'length')]}
    return namespace, reference


def contracts():
    namespace, reference = load_context()
    error_type = namespace['FErr']
    checks = 0

    def accepts(source, expected, statement=False):
        nonlocal checks
        error = None
        result = None
        try:
            result = reference.statement(source, {}) if statement else reference.parse(source)
        except error_type as refused:
            error = refused
        assert error is None, ('refused valid machine form', source, error)
        actual = result[:3] if statement else result
        assert actual == expected, (source, actual, expected)
        checks += 1

    def refuses(source, signature, statement=False):
        nonlocal checks
        error = None
        try:
            if statement:
                reference.statement(source, {})
            else:
                reference.parse(source)
        except error_type as refused:
            error = refused
        assert error is not None, ('accepted invalid machine form', source)
        assert error.token == 'formula_parse' and signature in error.msg, (source, error)
        checks += 1

    for name in ['a', 'waist_girth', 'point_2_x', 'a_2', 'x2', 'letter', 'assertion',
                 'if_else', 'let_value', 'eps_num', 'size_index', 'is_base_size']:
        accepts(name, ('name', name))
    for name in ['Upper', 'waistGirth', '_a', 'a_', 'a__b', 'waist_Girth']:
        refuses(name, 'violates lower-snake spelling')
    for keyword in ['let', 'assert', 'if']:
        refuses(keyword, 'is not an identifier here')
        refuses('let ' + keyword + ': count = 4', 'is not an identifier here', statement=True)
        refuses('assert ' + keyword + ': eps_num = 1 cm == 1 cm',
                'is not an identifier here', statement=True)
    for keyword in ['let', 'assert']:
        refuses(keyword + '(1)', 'is not an identifier here')
    accepts('if(a, b, c)', ('if', ('name', 'a'), ('name', 'b'), ('name', 'c')))
    accepts('if_else(a)', ('call', 'if_else', [('name', 'a')]))
    for source in ['if(a)', 'if(a, b)', 'if(a, b, c, d)']:
        refuses(source, 'conditional takes three parts')
    for source in ['abs()', 'f()', 'if()']:
        refuses(source, 'call arguments require at least one expression')
    for source in ['abs(1,)', 'f(,1)', 'if(a,,c)']:
        refuses(source, 'unexpected')
    accepts('abs(1)', ('call', 'abs', [('lit', 'count', Fraction(1))]))
    accepts('max(1, 2)', ('call', 'max', [('lit', 'count', Fraction(1)),
                                       ('lit', 'count', Fraction(2))]))
    for source in ['1 cm é', '١', '１', 'waist_２', 'a\u00a0b', '√(a)', '1 °']:
        refuses(source, 'machine syntax must be ASCII')
    for space in [' ', '\t', '\n', '\r', '\f', '\v']:
        accepts(space + 'a' + space + '+' + space + 'b' + space,
                ('bin', '+', ('name', 'a'), ('name', 'b')))
        accepts('let' + space + 'letter: count = 4', ('let', 'letter', 'count'), statement=True)
    for unit, (kind, factor) in reference.units.items():
        accepts('1 ' + unit, ('lit', kind, factor))
        for gap in ['', '  ', '\t', '\n', '\r', '\f', '\v']:
            refuses('1' + gap + unit, 'unit follows it with exactly one space')
    accepts('0004', ('lit', 'count', Fraction(4)))
    accepts('01.0500', ('lit', 'ratio', Fraction(1050000)))
    for source in ['.5', '1.', '1..2', '1e3', '0x10', '1 alien_unit', '1 2']:
        refuses(source, 'grammar' if '.' in source else 'trailing')
    accepts('assert seam_check: eps_num = 1 cm == 1 cm',
            ('assert', 'seam_check', True), statement=True)
    for source in ['assert seam_check: eps_num = 1cm == 1 cm',
                   'assert seam_check: eps_num = 1 cm == 1\tcm']:
        refuses(source, 'unit follows it with exactly one space', statement=True)
    # Tokenizer positions refer to the original ASCII source, including all skipped whitespace.
    tokens, starts = reference.tokenize(' \tlet\nletter: count = 1 cm ')
    assert tokens == [('id', 'let'), ('id', 'letter'), ('op', ':'), ('id', 'count'),
                      ('op', '='), ('num', '1'), ('id', 'cm')]
    assert starts == [2, 6, 12, 14, 20, 22, 24]
    checks += 1
    print('formula input contracts: %d pass / 0 fail' % checks)


def end_to_end():
    WORK.mkdir(exist_ok=True)
    for name, original, replacement, signature in [
        ('missing unit gap', '`if(dart_intake > dart_intake_max, 2, 1)`',
         '`if(dart_intake > 5cm, 2, 1)`', 'unit follows it with exactly one space'),
        ('malformed binding', '| `garment_waist` | length |',
         '| `garment__waist` | length |', 'violates lower-snake spelling'),
        ('keyword binding', '| `garment_waist` | length |',
         '| `if` | length |', 'is not an identifier here'),
    ]:
        book = WORK / name.replace(' ', '_') / 'src'
        if book.exists():
            shutil.rmtree(book)
        shutil.copytree(ROOT / 'docs/book/src', book)
        examples = book / 'spec/formula-language/examples.md'
        text = examples.read_text()
        assert text.count(original) == 1, ('mutation anchor changed', name)
        examples.write_text(text.replace(original, replacement))
        result = subprocess.run(['bash', str(TOOL)], cwd=ROOT,
                                env=dict(os.environ, FORMULA_BOOK=str(book)),
                                capture_output=True, text=True)
        output = result.stdout + result.stderr
        (WORK / (name.replace(' ', '_') + '.log')).write_text(output)
        assert result.returncode == 1 and 'formula_parse' in output and signature in output, (name, result.returncode, output)
        print('  input end-to-end:', name, 'rc=1, named grammar refusal')
    print('formula input end-to-end: 3 pass / 0 fail')


if __name__ == '__main__':
    contracts()
    if '--contracts-only' not in sys.argv:
        end_to_end()
