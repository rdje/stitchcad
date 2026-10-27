"""D278: scope numeric-check workspaces, registries and parts before writes/content measurements."""
from pathlib import Path
import argparse
import runpy
import stat
import sys

ROOT = Path(__file__).resolve().parents[4]
READER = runpy.run_path(str(Path(__file__).with_name("maintained_reference.py")))
PROFILE = runpy.run_path(str(ROOT / "scripts/local_environment.py"))
LIMIT = 1048576


def local(value, root):
    path = Path(value)
    if not path.is_absolute():
        path = root / path
    # The shared contract rejects escapes before looking at any outside destination.
    relative = PROFILE["producer_path"](ROOT, path)
    return path, relative


def source(value, root, maximum):
    path, relative = local(value, root)
    return path, READER["read_file"](ROOT, relative, maximum)


def select(root_value, registry, tracked):
    root = Path(root_value)
    if not root.is_absolute():
        root = ROOT / root
    PROFILE["workspace"](root)
    scratch = root / "target/doctrine_scratch"
    PROFILE["directory"](ROOT, scratch.relative_to(ROOT).as_posix(), missing_ok=True)
    directory = Path(registry)
    if not directory.is_absolute():
        directory = root / directory
    for name in ("surfaces.tsv", "routes.tsv"):
        source(directory / name, root, 65536)
    if tracked:
        source(tracked, root, LIMIT)
    return root


def parts(root_value, registry, output):
    root = Path(root_value)
    PROFILE["workspace"](root)
    registry_path, data = source(registry, root, 65536)
    output_path, relative_output = local(output, root)
    PROFILE["producer_output_path"](ROOT, output_path)
    READER["need"](output_path != registry_path, "path", "measurement output aliases registry")
    entries = []
    rows = 0
    for line in data.decode("utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        fields = line.split("\t")
        READER["need"](len(fields) >= 3 and fields[0], "schema", "measurement registry fields")
        rows += 1
        READER["need"](rows <= 64, "budget", "measurement registry population")
        identity, value, kind = fields[:3]
        if kind == "external":
            names = []
        elif kind == "file":
            names = [value]
        elif kind == "list":
            names = value.split(";")
        elif kind in ("collection", "binary_collection"):
            listing = READER["git"](root, ["ls-files", "-z", "--", value], LIMIT, literal=False)
            names = [item.decode("utf-8") for item in listing.split(b"\0") if item]
        else:
            raise READER["Refusal"]("schema", "unknown measurement kind")
        for name in names:
            READER["portable"](name)
            path = root / name
            relative = PROFILE["producer_path"](ROOT, path)
            info = READER["metadata"](ROOT, relative)
            READER["need"](stat.S_ISREG(info.st_mode) and info.st_nlink == 1,
                           "path", "measurement source must be an owned regular file")
            READER["need"](path != output_path, "path", "measurement output aliases a part")
            entries.append((identity, name))
            READER["need"](len(entries) <= 8192, "budget", "measurement part census")
    payload = "".join(identity + "\t" + name + "\n" for identity, name in entries).encode("utf-8")
    READER["need"](len(payload) <= LIMIT, "budget", "measurement manifest bytes")
    PROFILE["plan_producer"](ROOT, sources=(registry_path, Path(__file__),
        Path(__file__).with_name("maintained_reference.py"), ROOT / "scripts/local_environment.py"),
        output_files=(output_path,))
    # No part body has been opened. The native wc phase receives only this fully validated manifest.
    output_path.write_bytes(payload)


def tracked(root_value, output):
    root = Path(root_value)
    PROFILE["workspace"](root)
    path, _relative = local(output, root)
    PROFILE["producer_output_path"](ROOT, path)
    listing = READER["git"](root, ["ls-files", "-z", "--", "*.md"], LIMIT, literal=False)
    names = [READER["portable"](item.decode("utf-8")) for item in listing.split(b"\0") if item]
    READER["need"](len(names) <= 8192, "budget", "tracked Markdown population")
    PROFILE["plan_producer"](ROOT, sources=(Path(__file__),
        Path(__file__).with_name("maintained_reference.py"), ROOT / "scripts/local_environment.py"),
        output_files=(path,))
    path.write_text("".join(name + "\n" for name in names), encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    actions = parser.add_subparsers(dest="action", required=True)
    choose = actions.add_parser("root")
    choose.add_argument("workspace")
    choose.add_argument("registry")
    choose.add_argument("tracked", nargs="?", default="")
    measure = actions.add_parser("parts")
    measure.add_argument("workspace")
    measure.add_argument("registry")
    measure.add_argument("output")
    inventory = actions.add_parser("tracked")
    inventory.add_argument("workspace")
    inventory.add_argument("output")
    arguments = parser.parse_args()
    try:
        if arguments.action == "root":
            print(select(arguments.workspace, arguments.registry, arguments.tracked))
        elif arguments.action == "parts":
            parts(arguments.workspace, arguments.registry, arguments.output)
        else:
            tracked(arguments.workspace, arguments.output)
    except (ValueError, OSError, UnicodeError) as issue:
        print("LIVE-DOC-SIZE: REFUSED input scope — " + str(issue), file=sys.stderr)
        return 2 if arguments.action == "root" else 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
