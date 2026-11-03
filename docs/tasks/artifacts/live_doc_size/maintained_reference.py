"""Book structure and shared local input helpers; book size containment retired by director2026-11-04.

No chapter, index, aggregate, file-count, line-width or navigation-depth limit and no growth receipt.
Working and staged checks retain complete local Markdown indexing and regular source/object checks.
Shared task/measurement helpers keep their independent non-book input limits.
"""
from __future__ import annotations
import argparse
from dataclasses import dataclass
import os
from pathlib import Path, PurePosixPath
import re
import stat
import subprocess
import threading

ROOT = Path(__file__).resolve().parents[4]
SOURCE = "docs/book/src"
INDEX = SOURCE + "/SUMMARY.md"
ATTRIBUTES = ".gitattributes"
TASK_BYTES = 98304
TASK_FILES = 32
CENSUS_ENTRIES = 8192
LISTING_BYTES = 1048576
GIT_SECONDS = 10

class Refusal(ValueError):
    """A structural/path/input refusal, never book size or growth authority."""
    def __init__(self, code: str, detail: str):
        self.code = code
        super().__init__(f"book-integrity: {code}: {detail}")

def need(condition: bool, code: str, detail: str) -> None:
    if not condition:
        raise Refusal(code, detail)

def portable(value: str) -> str:
    need(type(value) is str, "path", "path must be text")
    try:
        size = len(value.encode("utf-8"))
    except UnicodeError as issue:
        raise Refusal("path", "path encoding") from issue
    need(0 < size <= 4096, "path", "invalid path size")
    need(not any(ord(c) < 32 or c in "\\:#?%" or ord(c) == 127 for c in value),
         "path", "ambiguous/nonlocal path")
    path = PurePosixPath(value)
    need(not path.is_absolute() and str(path) == value and all(p not in (".", "..") for p in path.parts),
         "path", "path must be canonical and repository-relative")
    return value

def metadata(root: Path, relative: str):
    """Reject links, foreign devices, nested repositories and special files before content reads."""
    portable(relative)
    device = root.stat().st_dev
    current = root
    for component in PurePosixPath(relative).parts:
        need(component != ".git", "path", "nested repository boundary")
        current = current / component
        try:
            info = current.lstat()
        except OSError as issue:
            raise Refusal("path", f"missing/unreadable {relative}") from issue
        need(not stat.S_ISLNK(info.st_mode) and info.st_dev == device,
             "path", f"link/foreign device at {relative}")
        if current != root / relative:
            need(stat.S_ISDIR(info.st_mode), "path", f"non-directory parent of {relative}")
            need(not (current / ".git").exists(), "path", f"nested repository at {relative}")
    return info

def read_file(root: Path, relative: str, maximum: int | None) -> bytes:
    before = metadata(root, relative)
    need(stat.S_ISREG(before.st_mode) and before.st_nlink == 1, "path", f"nonregular/shared {relative}")
    need(maximum is None or before.st_size <= maximum, "budget", f"oversized {relative}")
    flags = os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0) | getattr(os, "O_NONBLOCK", 0)
    try:
        descriptor = os.open(root / relative, flags)
        with os.fdopen(descriptor, "rb") as stream:
            opened = os.fstat(stream.fileno())
            need((before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns) ==
                 (opened.st_dev, opened.st_ino, opened.st_size, opened.st_mtime_ns),
                 "changed", f"changed before reading {relative}")
            data = stream.read() if maximum is None else stream.read(maximum + 1)
            after = os.fstat(stream.fileno())
    except OSError as issue:
        raise Refusal("path", f"cannot read {relative}") from issue
    need(maximum is None or len(data) <= maximum, "budget", f"oversized {relative}")
    need((opened.st_dev, opened.st_ino, opened.st_size, opened.st_mtime_ns) ==
         (after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns) and len(data) == after.st_size,
         "changed", f"changed while reading {relative}")
    return data

def git(root: Path, arguments: list[str], maximum: int | None, *, literal: bool = True) -> bytes:
    """Bound native output before allocation; always reap the child, also on refusal/interruption."""
    # A caller's alternate Git directory, index, object store, namespace or injected config cannot
    # select the authority. Preserve the parent environment; native commands remain read-only.
    environment = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
    environment.update(GIT_OPTIONAL_LOCKS="0", GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull,
                       GIT_CONFIG_SYSTEM=os.devnull, GIT_NO_REPLACE_OBJECTS="1", GIT_TERMINAL_PROMPT="0")
    try:
        options = ["--literal-pathspecs"] if literal else []
        process = subprocess.Popen(["git", "--no-pager", "-c", "core.fsmonitor=false",
                                    "-c", "submodule.recurse=false", *options, "-C", str(root), *arguments],
                                   stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env=environment)
    except OSError as issue:
        raise Refusal("git", "native Git unavailable") from issue
    timer = threading.Timer(GIT_SECONDS, process.kill)
    timer.daemon = True
    started = False
    try:
        try:
            timer.start()
            started = True
        except (RuntimeError, OSError) as issue:
            raise Refusal("git", "native timeout watchdog unavailable") from issue
        need(process.stdout is not None, "git", "native output pipe missing")
        output = process.stdout.read() if maximum is None else process.stdout.read(maximum + 1)
        need(maximum is None or len(output) <= maximum, "budget", "native Git output exceeds its contract")
        result = process.wait()
        if result != 0:
            raise Refusal("git", "native Git refused or timed out; full baseline history is required")
        return output
    finally:
        timer.cancel()
        if started:
            timer.join()
        if process.poll() is None:
            process.kill()
        process.wait()
        if process.stdout is not None:
            process.stdout.close()

def storage_profile(root: Path, paths: set[str]) -> None:
    """Verify explicitly governed Git storage attributes for non-size integrity controls.

    Check effective native attributes, including nested and info attributes, rather than assuming
    the root declarations won. Ordinary candidates use this repository's real storage policy.
    """
    read_file(root, ATTRIBUTES, 65536)
    names = sorted(paths)
    attributes = ("text", "filter", "ident", "working-tree-encoding")
    expected = b"".join(portable(name).encode("utf-8") + b"\0" + attribute.encode("ascii") + b"\0unset\0"
                        for name in names for attribute in attributes)
    actual = git(root, ["check-attr", "-z", *attributes, "--", *names], LISTING_BYTES)
    need(actual == expected, "storage", "governed Git clean transformations must be explicitly unset")

def task_status(data: bytes | None, task: str) -> str | None:
    if data is None:
        return None
    need(len(data) <= TASK_BYTES, "budget", "task tree bytes")
    try:
        source = data.decode("utf-8")
    except UnicodeError as issue:
        raise Refusal("task", "task encoding") from issue
    entries = re.findall(r"^- ID: `([^`]+)`\r?\n  Status: `([^`]+)`", source, re.MULTILINE)
    matches = [status for identity, status in entries if identity == task]
    need(len(matches) <= 1, "task", "duplicate authority owner")
    return matches[0] if matches else None

def task_file_name(name: str, task: str) -> bool:
    tree = re.escape(task.split(".", 1)[0])
    return re.fullmatch(tree + r"(?:-[A-Za-z0-9][A-Za-z0-9_.-]*)?\.md", name) is not None

def task_paths(tree: Path, task: str) -> set[str]:
    """Bound the direct task family before reading bodies; never descend into artifact directories."""
    folder = "docs/tasks"
    need(stat.S_ISDIR(metadata(tree, folder).st_mode) and not os.path.lexists(tree / folder / ".git"),
         "path", "task directory is not an ordinary project directory")
    paths = set()
    try:
        with os.scandir(tree / folder) as entries:
            for count, entry in enumerate(entries, 1):
                need(count <= CENSUS_ENTRIES, "budget", "task directory entry census")
                if task_file_name(entry.name, task):
                    paths.add(folder + "/" + entry.name)
                    need(len(paths) <= TASK_FILES, "budget", "task-family file census")
    except OSError as issue:
        raise Refusal("path", "cannot enumerate task family") from issue
    for relative in sorted(paths):
        info = metadata(tree, relative)
        need(stat.S_ISREG(info.st_mode) and info.st_nlink == 1,
             "path", f"task source must be regular and unshared: {relative}")
        need(info.st_size <= TASK_BYTES, "budget", f"oversized task source: {relative}")
    return paths

def current_tasks(tree: Path, task: str) -> dict[str, bytes]:
    sources = {name: read_file(tree, name, TASK_BYTES) for name in sorted(task_paths(tree, task))}
    root = "docs/tasks/" + task.split(".", 1)[0] + ".md"
    need(root in sources, "task", "canonical task root missing")
    # Canonical siblings form a bounded linked graph. Basename-only local links retain the existing
    # task-tree convention; fragments are navigation, never another source or an escaping path.
    link = re.compile(rb"\]\(([A-Za-z0-9_.-]+\.md)(?:#[^()\r\n]*)?\)")
    reached = set()
    pending = {root}
    while pending:
        relative = min(pending)
        pending.remove(relative)
        if relative in reached:
            continue
        reached.add(relative)
        for raw in link.findall(sources[relative]):
            name = raw.decode("ascii")
            if task_file_name(name, task):
                target = "docs/tasks/" + name
                portable(target)
                need(target in sources, "task", f"linked task sibling missing: {target}")
                if target not in reached:
                    pending.add(target)
    need(reached == set(sources), "task", "unlinked task sibling")
    return sources

def owner_status(sources: dict[str, bytes], task: str) -> str | None:
    states = [state for data in sources.values() if (state := task_status(data, task)) is not None]
    need(len(states) <= 1, "task", "duplicate authority owner across task sources")
    return states[0] if states else None

def selected_tree(root: Path, tree: Path | None) -> tuple[Path, Path]:
    root = root.resolve(strict=True)
    tree = root if tree is None else tree
    try:
        candidate = tree.relative_to(root).as_posix()
    except ValueError as issue:
        raise Refusal("path", "candidate must stay in this repository") from issue
    if candidate != ".":
        need(stat.S_ISDIR(metadata(root, candidate).st_mode) and not os.path.lexists(tree / ".git"),
             "path", "candidate directory must not be another repository")
    return root, tree

@dataclass(frozen=True)
class Snapshot:
    files: int
    lines: int
    bytes: int


def markdown(data: bytes, relative: str) -> str:
    try:
        return data.decode("utf-8")
    except UnicodeError as issue:
        raise Refusal("encoding", f"non-UTF-8 Markdown: {relative}") from issue


def navigation(data: bytes) -> set[str]:
    chapters = set()
    previous = 0
    for line in markdown(data, INDEX).splitlines():
        if not line.strip() or re.fullmatch(r"#{1,6} .+", line):
            continue
        match = re.fullmatch(r"( *)(?:- )?\[([^\]\r\n]+)\]\(([^()\r\n]+)\)", line)
        need(match is not None, "navigation", "unsupported mandatory-index syntax")
        indent, label, href = match.groups()
        need(bool(label.strip()) and len(indent) % 2 == 0, "navigation", "label/indentation")
        depth = len(indent) // 2 + 1
        need(depth <= previous + 1, "navigation", "skipped parent level")
        previous = depth
        portable(href)
        need(href.endswith(".md"), "navigation", "chapter must be a local Markdown path")
        relative = SOURCE + "/" + href
        need(relative != INDEX and relative not in chapters, "navigation", "duplicate/self chapter")
        chapters.add(relative)
    need(bool(chapters), "navigation", "empty mandatory index")
    return chapters


def summarize(parts: dict[str, bytes]) -> Snapshot:
    need(INDEX in parts, "navigation", "mandatory index missing")
    need(set(parts) == navigation(parts[INDEX]) | {INDEX}, "navigation", "missing/unindexed Markdown chapter")
    for relative, data in parts.items():
        portable(relative)
        markdown(data, relative)
    return Snapshot(len(parts), sum(data.count(b"\n") for data in parts.values()),
                    sum(len(data) for data in parts.values()))


def check(root: Path = ROOT, tree: Path | None = None) -> Snapshot:
    _root, tree = selected_tree(root, tree)
    need(stat.S_ISDIR(metadata(tree, SOURCE).st_mode), "path", "book source is not a directory")
    index = read_file(tree, INDEX, None)
    actual = set()
    pending = [tree / SOURCE]
    while pending:
        directory = pending.pop()
        with os.scandir(directory) as entries:
            for entry in entries:
                relative = (directory / entry.name).relative_to(tree).as_posix()
                info = metadata(tree, relative)
                if stat.S_ISDIR(info.st_mode):
                    need(not os.path.lexists(tree / relative / ".git"), "path", "nested repository")
                    pending.append(tree / relative)
                else:
                    need(stat.S_ISREG(info.st_mode) and info.st_nlink == 1,
                         "path", "nonregular/shared source")
                    if entry.name.endswith(".md"):
                        actual.add(relative)
    need(actual == navigation(index) | {INDEX}, "navigation", "missing/unindexed Markdown chapter")
    parts = {name: read_file(tree, name, None) for name in sorted(actual)}
    need(parts[INDEX] == index, "changed", "mandatory index changed during census")
    return summarize(parts)


def staged_entries(listing: bytes) -> dict[str, str]:
    records = listing.split(b"\0")
    need(records[-1] == b"", "index", "incomplete native listing")
    result = {}
    for record in records[:-1]:
        header, separator, encoded = record.partition(b"\t")
        need(bool(separator), "index", "staged entry shape")
        try:
            name = portable(encoded.decode("utf-8"))
        except UnicodeError as issue:
            raise Refusal("index", "staged path encoding") from issue
        need(name.startswith(SOURCE + "/"), "index", "unexpected staged source")
        fields = header.split()
        need(len(fields) == 3 and fields[0] in (b"100644", b"100755") and fields[2] == b"0",
             "index", "book requires regular stage-zero entries")
        need(re.fullmatch(rb"[0-9a-f]{40}|[0-9a-f]{64}", fields[1]) is not None,
             "index", "invalid staged object identity")
        need(name not in result, "index", "duplicate staged path")
        result[name] = fields[1].decode("ascii")
    return result


def check_index(root: Path = ROOT) -> Snapshot:
    root, _tree = selected_tree(root, None)
    command = ["ls-files", "--stage", "-z", "--", SOURCE]
    listing = git(root, command, None)
    entries = staged_entries(listing)
    parts = {name: git(root, ["cat-file", "blob", oid], None)
             for name, oid in entries.items() if name.endswith(".md")}
    result = summarize(parts)
    need(git(root, command, None) == listing, "changed", "index changed during verification")
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--index", action="store_true", help="check exact staged book objects")
    parser.add_argument("--tree", type=Path, help="ordinary project-local fixture tree")
    args = parser.parse_args()
    try:
        need(not (args.index and args.tree), "input", "index mode uses the canonical repository")
        tree = ROOT / args.tree if args.tree is not None and not args.tree.is_absolute() else args.tree
        snapshot = check_index() if args.index else check(tree=tree)
        print(f"book-integrity: OK — {'index' if args.index else 'working'}; {snapshot.files} Markdown files")
        return 0
    except (Refusal, OSError, UnicodeError) as issue:
        print(str(issue))
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
