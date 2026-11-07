#!/usr/bin/env python3
"""Independent reader: ASTM D6673 .rul + DXF -> graded boundary points TSV."""
import os, re, sys

def die(msg):
    sys.stderr.write("REFUSED: %s\n" % msg); sys.exit(1)

NUM = re.compile(r'^[+-]?\d+(\.\d+)?$')

def parse_rul(path):
    hdr = {}
    lines = open(path, encoding='ascii').read().splitlines()
    i = 0
    while i < len(lines):
        s = lines[i].strip()
        if s.upper().startswith('RULE'):
            break
        i += 1
        if not s: continue
        if ':' not in s: die("rul header line without colon: %r" % s)
        k, v = s.split(':', 1)
        hdr.setdefault(k.strip().upper(), v.strip())
    for k in ('UNITS', 'NUMBER OF SIZES', 'SIZE LIST', 'SAMPLE SIZE'):
        if k not in hdr: die("rul missing header %s" % k)
    if hdr['UNITS'].upper() != 'METRIC': die("only METRIC supported, got %r" % hdr['UNITS'])
    if not hdr['NUMBER OF SIZES'].isdigit(): die("bad NUMBER OF SIZES")
    n = int(hdr['NUMBER OF SIZES'])
    sizes = hdr['SIZE LIST'].replace(',', ' ').split()
    if len(sizes) != n: die("NUMBER OF SIZES %d != %d names in SIZE LIST" % (n, len(sizes)))
    if len(set(sizes)) != n: die("duplicate size names")
    sample = hdr['SAMPLE SIZE']
    if sample not in sizes: die("sample size %r not in size list" % sample)
    toks = []
    for ln in lines[i:]:
        toks += ln.replace(',', ' ').split()
    rules = {}
    p = 0
    ended = False
    while p < len(toks):
        t = toks[p]
        if t.upper() == 'END':
            ended = True
            if p != len(toks) - 1: die("content after END")
            break
        if t.upper() == 'RULE:' or t.upper() == 'RULE':
            p += 1
            if t.upper() == 'RULE' and p < len(toks) and toks[p] == ':': p += 1
        else:
            die("expected RULE:, got %r" % t)
        if p >= len(toks) or toks[p].upper() != 'DELTA': die("rule type is not DELTA near token %d" % p)
        p += 1
        if p >= len(toks) or not re.fullmatch(r'[+-]?\d+', toks[p]): die("bad rule identifier")
        rid = int(toks[p]); p += 1
        if rid in rules: die("duplicate rule %d" % rid)
        vals = []
        while p < len(toks) and toks[p].upper() not in ('RULE:', 'RULE', 'END'):
            if not NUM.match(toks[p]): die("rule %d: non-numeric value %r" % (rid, toks[p]))
            vals.append(float(toks[p])); p += 1
        if len(vals) != 2 * n: die("rule %d has %d values, expected %d" % (rid, len(vals), 2 * n))
        d = [(vals[2*k], vals[2*k+1]) for k in range(n)]
        si = sizes.index(sample)
        if d[si] != (0.0, 0.0): die("rule %d: nonzero delta at sample size %s: %r" % (rid, sample, d[si]))
        rules[rid] = d
    if not ended: die("missing END")
    return sizes, sample, rules

def parse_dxf(path):
    raw = open(path, encoding='ascii').read().split('\n')
    if raw and raw[-1] == '': raw.pop()
    if len(raw) % 2: die("DXF has odd number of lines")
    pairs = []
    for k in range(0, len(raw), 2):
        c = raw[k].strip()
        if not re.fullmatch(r'-?\d+', c): die("DXF line %d: bad group code %r" % (k+1, raw[k]))
        pairs.append((int(c), raw[k+1].strip(), k+2))
    # entities
    ents = []  # (type, [(code,val)])
    cur = None
    for c, v, ln in pairs:
        if c == 0:
            cur = [v, []]; ents.append(cur)
        elif cur is not None:
            cur[1].append((c, v))
    blocks = {}
    block = None
    section = None
    for idx, (t, data) in enumerate(ents):
        if t == 'SECTION':
            section = next((v for c, v in data if c == 2), None)
        elif t == 'ENDSEC':
            section = None
        elif t == 'BLOCK' and section == 'BLOCKS':
            name = next((v for c, v in data if c == 2), None)
            if name is None: die("BLOCK without name")
            if name in blocks: die("duplicate BLOCK %s" % name)
            block = {'verts': [], 'texts': [], 'poly_layers': []}
            blocks[name] = block
        elif t == 'ENDBLK':
            block = None
        elif block is not None:
            d = {}
            for c, v in data: d.setdefault(c, v)
            layer = d.get(8)
            if t == 'VERTEX':
                block['verts'].append((layer, float(d[10]), float(d[20])))
            elif t == 'TEXT':
                block['texts'].append((layer, float(d[10]), float(d[20]), d.get(1, '')))
            elif t == 'POLYLINE':
                block['poly_layers'].append(layer)
    return blocks

def main():
    if len(sys.argv) != 3: die("usage: read_rul_independently.py RUL DXF")
    sizes, sample, rules = parse_rul(sys.argv[1])
    blocks = parse_dxf(sys.argv[2])
    if not blocks: die("no BLOCKs")
    TOL = 0.005
    rows = []
    for bname, b in blocks.items():
        l1v = [(x, y) for L, x, y in b['verts'] if L == '1']
        if not l1v: die("block %s: no layer 1 vertices" % bname)
        seen = {}
        for L, x, y, s in b['texts']:
            if L != '1' or not s.startswith('#'): continue
            body = s[1:].split(',')[0].strip()   # "# n[, alt]" -> n
            if not re.fullmatch(r'[+-]?\d+', body): die("block %s: bad identifier text %r" % (bname, s))
            n = int(body)
            hits = [v for v in l1v if abs(v[0]-x) <= TOL and abs(v[1]-y) <= TOL]
            if not hits: die("block %s: '# %d' at (%.2f,%.2f) not on a layer 1 vertex" % (bname, n, x, y))
            if n not in rules: die("block %s: rule %d has no entry in rule table" % (bname, n))
            if n in seen: die("block %s: identifier %d used twice" % (bname, n))
            seen[n] = (x, y)
            ex = [h for h in hits if h != (x, y)]
            if ex: sys.stderr.write("note: block %s rule %d matched with nonzero offset\n" % (bname, n))
            if len(hits) > 1: sys.stderr.write("note: block %s rule %d matches %d vertices\n" % (bname, n, len(hits)))
            # point location: the vertex's coordinates
            vx, vy = hits[0]
            for si, sz in enumerate(sizes):
                dx, dy = rules[n][si]
                rows.append((bname, n, si, sz, vx + dx, vy + dy))
    rows.sort(key=lambda r: (r[0], r[1], r[2]))
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'independent_reader_output.tsv')
    with open(out, 'w') as f:
        f.write("block\trule\tsize\tx_mm\ty_mm\n")
        for b, n, si, sz, x, y in rows:
            f.write("%s\t%d\t%s\t%.2f\t%.2f\n" % (b, n, sz, x, y))
    used = {r[1] for r in rows}
    unused = sorted(set(rules) - used)
    print("OK: %d blocks, %d rules in table, %d rows -> %s" % (len(blocks), len(rules), len(rows), out))
    print("rules unused by any layer-1 text: %s" % unused)

main()
