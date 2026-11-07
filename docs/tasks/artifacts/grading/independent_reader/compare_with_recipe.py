#!/usr/bin/env python3
"""What a separately written reader made of a written .rul and DXF, against what the recipe draws at each size.

argv: the reader's TSV, the table this project wrote (rules.json), and a transcript with a regenerate step.
A base coordinate and a delta are each rounded to 0.01 mm in the files, so the two can differ by 10 um. Anything more is a finding, and the exit status says so."""
import csv
import json
import sys

BOUND_UM = 10

rows = list(csv.DictReader(open(sys.argv[1], encoding='ascii'), delimiter='\t'))
table = json.load(open(sys.argv[2], encoding='utf-8'))
transcript = json.load(open(sys.argv[3], encoding='utf-8'))
by_id = {rule['id']: rule['point'] for rule in table['rules']}
report = next(step['result'] for step in transcript if step['do'] == 'regenerate')
sample = report['sample']
places = {}
for size in report['sizes']:
    for item in size['points']:
        if item['point'] == 'to':
            key = f"{item['piece']}/{item['edge']}/to"
            places[(key, size['size'])] = (item['x']['value'], item['y']['value'])
anchor = {}
for stack in table['stack']:
    anchor[stack.split('/')[0]] = stack
largest = 0
compared = 0
findings = []
for row in rows:
    point = by_id.get(int(row['rule']))
    if point is None or not point.endswith('/to'):
        findings.append((row['block'], row['rule'], row['size'], 'the reader gave a rule that is not a corner'))
        continue
    piece = point.split('/')[0]
    if row['block'] != f'{piece}_{sample}':
        findings.append((row['block'], row['rule'], row['size'], f'this rule is for {piece}'))
        continue
    size = row['size']
    here = places[(point, size)]
    stack_here = places[(anchor[piece], size)]
    stack_sample = places[(anchor[piece], sample)]
    wanted = (here[0] - (stack_here[0] - stack_sample[0]), here[1] - (stack_here[1] - stack_sample[1]))
    # The reader writes two places of a millimetre, so this is exact in hundredths.
    read = (round(float(row['x_mm']) * 100) * 10, round(float(row['y_mm']) * 100) * 10)
    apart = max(abs(read[0] - wanted[0]), abs(read[1] - wanted[1]))
    largest = max(largest, apart)
    compared += 1
    if apart > BOUND_UM:
        findings.append((row['block'], row['rule'], size, f'{apart} um from the recipe'))
print(f'compared {compared} corners at every size; the largest difference is {largest} um, against a bound of {BOUND_UM} um')
used = {int(row['rule']) for row in rows}
print(f"{len(by_id) - len(used)} rules in the table have no corner in this reader's output")
for finding in findings:
    print('FINDING', *finding)
sys.exit(1 if findings or compared == 0 else 0)
