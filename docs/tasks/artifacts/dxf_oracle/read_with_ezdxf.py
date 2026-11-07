#!/usr/bin/env python3
"""G2-2D.4.third-party-reader: every committed DXF read by ezdxf, a library nobody in this repository wrote.

WHY: until now each DXF here was read only by readers written in this repository, so a mistake shared by the
writer and its readers could not show. ezdxf is a widely used DXF library (MIT). It runs on the hosted runner
from the operating system's signed package, `python3-ezdxf`, and it is never shipped or linked.

WHAT IT CHECKS, for each file under conformance/ ending in .dxf:
  1. ezdxf opens it.
  2. ezdxf's auditor finds no error. Repairs it makes are printed, and each one is a defect to log.
  3. Every entity ezdxf reads, in each block and in model space, equals what a plain reading of the group codes
     says, with the same type, layer, coordinates, closed flag, text and block name. So ezdxf understands the
     entities the way they are written, and none is lost, added or reinterpreted.

FALSIFIED ON EVERY RUN, with two controls written under target/dxf_oracle/:
  C1. A copy with a group code that is not a number. ezdxf has to refuse it.
  C2. A copy with one coordinate moved by 1 mm. ezdxf's reading of it has to disagree with the original's
      plain reading, which proves the comparison is not empty.

Usage:  python3 -I -B docs/tasks/artifacts/dxf_oracle/read_with_ezdxf.py [--plain-only]
Exit:   0 every file passes and both controls fail as they should; 1 a failure; 2 ezdxf is not installed, which
        is never a pass. --plain-only runs the plain reading alone, as a self-test where ezdxf is absent.
"""
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True,
                                   check=True).stdout.strip())
STORE = ROOT / "target" / "dxf_oracle"
KNOWN = ("LINE", "POINT", "TEXT", "POLYLINE", "LWPOLYLINE", "INSERT", "CIRCLE", "ARC")


def committed():
    listed = subprocess.run(["git", "ls-files", "conformance"], cwd=ROOT, capture_output=True, text=True,
                            check=True).stdout.split()
    return [ROOT / name for name in listed if name.lower().endswith(".dxf")]


def pairs_of(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) % 2:
        raise ValueError("an odd number of lines")
    out = []
    for at in range(0, len(lines), 2):
        out.append((int(lines[at].strip()), lines[at + 1].rstrip("\r")))
    return out


def first(fields, code, default=None):
    for found, value in fields:
        if found == code:
            return value
    return default


def number(fields, code):
    value = first(fields, code)
    return None if value is None else float(value)


def plain_entity(kind, fields, vertices):
    layer = first(fields, 8, "0")
    if kind == "LINE":
        return (kind, layer, number(fields, 10), number(fields, 20), number(fields, 11), number(fields, 21))
    if kind == "POINT":
        return (kind, layer, number(fields, 10), number(fields, 20))
    if kind == "TEXT":
        return (kind, layer, number(fields, 10), number(fields, 20), first(fields, 1, ""))
    if kind == "POLYLINE":
        closed = bool(int(first(fields, 70, "0")) & 1)
        return (kind, layer, closed, tuple(vertices))
    if kind == "LWPOLYLINE":
        xs = [float(value) for code, value in fields if code == 10]
        ys = [float(value) for code, value in fields if code == 20]
        closed = bool(int(first(fields, 70, "0")) & 1)
        return (kind, layer, closed, tuple(zip(xs, ys)))
    if kind == "INSERT":
        return (kind, layer, first(fields, 2), number(fields, 10), number(fields, 20))
    if kind == "CIRCLE":
        return (kind, layer, number(fields, 10), number(fields, 20), number(fields, 40))
    if kind == "ARC":
        return (kind, layer, number(fields, 10), number(fields, 20), number(fields, 40), number(fields, 50),
                number(fields, 51))
    return ("UNREAD", layer, kind)


def plain_reading(text):
    """Containers by name, each a sorted list of entity tuples, read straight from the group codes."""
    records = []
    current = None
    for code, value in pairs_of(text):
        if code == 999:
            continue
        if code == 0:
            current = [value, []]
            records.append(current)
        elif current is not None:
            current[1].append((code, value))
    containers = {}
    section = None
    block = None
    polyline = None
    for kind, fields in records:
        if kind == "SECTION":
            section = first(fields, 2)
            continue
        if kind == "ENDSEC":
            section = None
            continue
        if section == "BLOCKS" and kind == "BLOCK":
            block = first(fields, 2)
            continue
        if section == "BLOCKS" and kind == "ENDBLK":
            block = None
            continue
        if section not in ("BLOCKS", "ENTITIES"):
            continue
        name = "*Model_Space" if section == "ENTITIES" else block
        if name is None or (section == "BLOCKS" and name.startswith("*")):
            continue
        if kind == "VERTEX" and polyline is not None:
            polyline[2].append((number(fields, 10), number(fields, 20)))
            continue
        if kind == "SEQEND" and polyline is not None:
            containers.setdefault(polyline[0], []).append(plain_entity("POLYLINE", polyline[1], polyline[2]))
            polyline = None
            continue
        if kind == "POLYLINE":
            polyline = [name, fields, []]
            continue
        containers.setdefault(name, []).append(plain_entity(kind, fields, []))
    return {name: sorted(items, key=repr) for name, items in containers.items()}


def ezdxf_entity(entity):
    kind = entity.dxftype()
    layer = entity.dxf.get("layer", "0")
    if kind == "LINE":
        start, end = entity.dxf.start, entity.dxf.end
        return (kind, layer, start.x, start.y, end.x, end.y)
    if kind == "POINT":
        at = entity.dxf.location
        return (kind, layer, at.x, at.y)
    if kind == "TEXT":
        at = entity.dxf.insert
        return (kind, layer, at.x, at.y, entity.dxf.get("text", ""))
    if kind == "POLYLINE":
        vertices = tuple((vertex.dxf.location.x, vertex.dxf.location.y) for vertex in entity.vertices)
        return (kind, layer, bool(entity.is_closed), vertices)
    if kind == "LWPOLYLINE":
        points = tuple((x, y) for x, y in entity.get_points("xy"))
        return (kind, layer, bool(entity.closed), points)
    if kind == "INSERT":
        at = entity.dxf.insert
        return (kind, layer, entity.dxf.name, at.x, at.y)
    if kind == "CIRCLE":
        at = entity.dxf.center
        return (kind, layer, at.x, at.y, entity.dxf.radius)
    if kind == "ARC":
        at = entity.dxf.center
        return (kind, layer, at.x, at.y, entity.dxf.radius, entity.dxf.start_angle, entity.dxf.end_angle)
    return ("UNREAD", layer, kind)


def ezdxf_reading(ezdxf, path):
    document = ezdxf.readfile(str(path))
    auditor = document.audit()
    containers = {"*Model_Space": [ezdxf_entity(entity) for entity in document.modelspace()]}
    for block in document.blocks:
        if block.name.startswith("*"):
            continue
        containers[block.name] = [ezdxf_entity(entity) for entity in block]
    containers = {name: sorted(items, key=repr) for name, items in containers.items() if items}
    return containers, auditor


def differences(ours, theirs):
    out = []
    for name in sorted(set(ours) | set(theirs)):
        mine, other = ours.get(name, []), theirs.get(name, [])
        if mine != other:
            missing = [item for item in mine if item not in other]
            extra = [item for item in other if item not in mine]
            out.append("%s: %d plain, %d ezdxf; first plain-only %r; first ezdxf-only %r"
                       % (name, len(mine), len(other), missing[:1], extra[:1]))
    for name, items in ours.items():
        for item in items:
            if item[0] == "UNREAD":
                out.append("%s: an entity this check does not read: %s" % (name, item[2]))
    return out


def main():
    plain_only = "--plain-only" in sys.argv[1:]
    files = committed()
    if not files:
        print("dxf-oracle: no DXF is committed under conformance/")
        return 1
    STORE.mkdir(parents=True, exist_ok=True)
    if plain_only:
        for path in files:
            reading = plain_reading(path.read_text(encoding="ascii"))
            count = sum(len(items) for items in reading.values())
            unread = sum(1 for items in reading.values() for item in items if item[0] == "UNREAD")
            print("plain\t%s\t%d containers\t%d entities\t%d unread" % (path.relative_to(ROOT), len(reading), count,
                                                                         unread))
            if count == 0 or unread:
                return 1
        print("dxf-oracle: plain reading only; ezdxf was not run, so nothing here is a third-party verdict")
        return 0
    try:
        import ezdxf
    except ImportError:
        print("dxf-oracle: ezdxf is not installed, so nothing was checked. This is never a pass.")
        return 2
    print("dxf-oracle: ezdxf %s" % ezdxf.__version__)
    failed = 0
    for path in files:
        name = path.relative_to(ROOT)
        ours = plain_reading(path.read_text(encoding="ascii"))
        try:
            theirs, auditor = ezdxf_reading(ezdxf, path)
        except Exception as error:  # a refusal by the library is the finding
            print("FAIL\t%s\tezdxf refused it: %s" % (name, error))
            failed += 1
            continue
        found = differences(ours, theirs)
        count = sum(len(items) for items in ours.values())
        errors = list(auditor.errors)
        fixes = list(auditor.fixes)
        for fix in fixes:
            print("REPAIRED\t%s\t%s" % (name, getattr(fix, "message", fix)))
        for error in errors:
            print("AUDIT\t%s\t%s" % (name, getattr(error, "message", error)))
        if count == 0 or found or errors:
            failed += 1
            for line in found:
                print("FAIL\t%s\t%s" % (name, line))
            if count == 0:
                print("FAIL\t%s\tno entity was read, so nothing was compared" % name)
        else:
            print("ok\t%s\t%d entities agree in %d containers; %d repairs" % (name, count, len(ours), len(fixes)))
    sample = files[0]
    text = sample.read_text(encoding="ascii")
    pairs = pairs_of(text)
    written = lambda items, first_code=None: "".join(
        "%s\n%s\n" % (first_code if at == 0 and first_code else "%3d" % code, value)
        for at, (code, value) in enumerate(items))
    section = next(at for at, pair in enumerate(pairs) if pair == (0, "SECTION"))
    garbled = STORE / "c1_garbled.dxf"
    garbled.write_text(written(pairs[:section]) + written(pairs[section:], "zero"), encoding="ascii")
    try:
        ezdxf.readfile(str(garbled))
        print("FAIL\tC1\tezdxf opened a file with a group code that is not a number, so a refusal here means nothing")
        failed += 1
    except Exception as error:
        print("ok\tC1\tezdxf refused the garbled copy: %s" % type(error).__name__)
    vertex = next(at for at, pair in enumerate(pairs) if pair[0] == 0 and pair[1] in ("VERTEX", "POINT", "LINE"))
    moved = next(at for at in range(vertex + 1, len(pairs)) if pairs[at][0] == 10)
    pairs[moved] = (10, "%.6f" % (float(pairs[moved][1]) + 1.0))
    shifted = STORE / "c2_shifted.dxf"
    shifted.write_text(written(pairs), encoding="ascii")
    try:
        theirs, _ = ezdxf_reading(ezdxf, shifted)
        seen = bool(differences(plain_reading(text), theirs))
    except Exception as error:
        print("FAIL\tC2\tezdxf refused the shifted copy: %s" % error)
        seen = None
    if seen:
        print("ok\tC2\ta coordinate moved by 1 mm is seen")
    elif seen is False:
        print("FAIL\tC2\ta coordinate moved by 1 mm was not seen, so agreement here means nothing")
    if not seen:
        failed += 1
    print("dxf-oracle: %d of %d files failed or a control did not" % (failed, len(files)))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
