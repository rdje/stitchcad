"""Maintained-book verification and guarded exact change recording for LIVE-DOC-SIZE.

The fixed schema-1 profile preserves existing part/index bounds. Current authority binds exact
content and counts to a native Git baseline and a newly completed task, never to spare capacity.
Only project-local candidate directories are supported; no Git repository is created or amended.
"""
from __future__ import annotations

import argparse
from dataclasses import asdict, dataclass
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import subprocess
import threading
import uuid
from typing import Any

ROOT = Path(__file__).resolve().parents[4]
CONTROL = ".doctrine/live_document_size/maintained_reference.jsonl"
REGISTRY = ".doctrine/live_document_size/surfaces.tsv"
SOURCE = "docs/book/src"
INDEX = SOURCE + "/SUMMARY.md"
PART_BOUNDS = (700, 40960, 440)
INDEX_BOUNDS = (120, 8192, 240)
CONTROL_BYTES = 8192
RECORD_BYTES = 6144
REGISTRY_BYTES = 65536
TASK_BYTES = 98304
CENSUS_ENTRIES = 8192
LISTING_BYTES = 1048576
GIT_SECONDS = 10
FRAME = b"StitchCAD-maintained-book-v1\0"


class Refusal(ValueError):
    """Stable refusal class plus bounded context; no partial authority or content is published."""
    def __init__(self, code: str, detail: str):
        self.code = code
        super().__init__(f"maintained-reference: {code}: {detail}")


def need(condition: bool, code: str, detail: str) -> None:
    if not condition:
        raise Refusal(code, detail)


def keys(value: Any, expected: set[str], where: str) -> dict:
    need(type(value) is dict and set(value) == expected, "schema", where)
    return value


def integer(value: Any, lower: int, upper: int, where: str) -> int:
    need(type(value) is int and lower <= value <= upper, "schema", where)
    return value


def text(value: Any, maximum: int, where: str) -> str:
    need(type(value) is str and bool(value.strip()), "schema", where)
    try:
        size = len(value.encode("utf-8"))
    except UnicodeError as issue:
        raise Refusal("schema", where) from issue
    need(size <= maximum, "schema", where)
    need(not any(ord(c) < 32 or ord(c) == 127 for c in value), "schema", where)
    return value


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


def read_file(root: Path, relative: str, maximum: int) -> bytes:
    before = metadata(root, relative)
    need(stat.S_ISREG(before.st_mode) and before.st_nlink == 1, "path", f"nonregular/shared {relative}")
    need(before.st_size <= maximum, "budget", f"oversized {relative}")
    flags = os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0) | getattr(os, "O_NONBLOCK", 0)
    try:
        descriptor = os.open(root / relative, flags)
        with os.fdopen(descriptor, "rb") as stream:
            opened = os.fstat(stream.fileno())
            need((before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns) ==
                 (opened.st_dev, opened.st_ino, opened.st_size, opened.st_mtime_ns),
                 "changed", f"changed before reading {relative}")
            data = stream.read(maximum + 1)
            after = os.fstat(stream.fileno())
    except OSError as issue:
        raise Refusal("path", f"cannot read {relative}") from issue
    need(len(data) <= maximum, "budget", f"oversized {relative}")
    need((opened.st_dev, opened.st_ino, opened.st_size, opened.st_mtime_ns) ==
         (after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns) and len(data) == after.st_size,
         "changed", f"changed while reading {relative}")
    return data


def git(root: Path, arguments: list[str], maximum: int, *, literal: bool = True) -> bytes:
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
        output = process.stdout.read(maximum + 1)
        need(len(output) <= maximum, "budget", "native Git output exceeds its contract")
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


def blob(root: Path, revision: str, relative: str, maximum: int, *, absent: bool = False):
    portable(relative)
    need(re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", revision) is not None,
         "git", "full native revision required")
    if absent:
        listing = git(root, ["ls-tree", "-z", revision, "--", relative], 8192)
        if not listing:
            return None
        entries = git_entries(listing)
        need(entries == [relative], "git", "ambiguous optional native blob")
    return git(root, ["show", f"{revision}:{relative}"], maximum)


def reject_duplicates(pairs):
    result = {}
    for name, value in pairs:
        need(name not in result, "schema", "duplicate JSON field")
        result[name] = value
    return result


def no_constant(_value):
    raise Refusal("schema", "nonfinite JSON number")


@dataclass(frozen=True)
class Snapshot:
    files: int
    lines: int
    bytes: int
    tree_sha256: str


def snapshot(value: Any) -> Snapshot:
    keys(value, {"files", "lines", "bytes", "tree_sha256"}, "snapshot fields")
    # Complete SUMMARY, not a separate aggregate-growth allowance, limits the Markdown population.
    count = integer(value["files"], 2, INDEX_BOUNDS[0] + 1, "snapshot files")
    lines = integer(value["lines"], 0, (count - 1) * PART_BOUNDS[0] + INDEX_BOUNDS[0], "snapshot lines")
    size = integer(value["bytes"], 0, (count - 1) * PART_BOUNDS[1] + INDEX_BOUNDS[1], "snapshot bytes")
    digest = text(value["tree_sha256"], 64, "snapshot digest")
    need(re.fullmatch("[0-9a-f]{64}", digest) is not None, "schema", "snapshot digest")
    return Snapshot(count, lines, size, digest)


def parse_control(data: bytes) -> dict:
    need(len(data) <= CONTROL_BYTES, "budget", "control bytes")
    lines = data.split(b"\n")
    need(len(lines) == 3 and lines[-1] == b"", "schema", "one JSONL header and one reference required")
    need(len(lines[0]) <= CONTROL_BYTES - RECORD_BYTES and len(lines[1]) <= RECORD_BYTES,
         "budget", "raw JSONL metadata/record bytes")
    try:
        document, record = [json.loads(line.decode("utf-8"), object_pairs_hook=reject_duplicates,
                                       parse_constant=no_constant) for line in lines[:2]]
    except (ValueError, UnicodeError, RecursionError) as issue:
        if isinstance(issue, Refusal):
            raise
        raise Refusal("schema", "invalid UTF-8 control JSONL") from issue
    keys(document, {"schema_version", "max_records", "max_bytes", "max_record_bytes"}, "control fields")
    for name, expected in (("schema_version", 1), ("max_records", 1), ("max_bytes", CONTROL_BYTES),
                           ("max_record_bytes", RECORD_BYTES)):
        need(type(document[name]) is int and document[name] == expected, "schema", name)
    keys(record, {"surface", "audience", "rationale", "navigation_depth", "authority"}, "reference fields")
    need(record["surface"] == "book_collection", "schema", "only book_collection is adopted")
    text(record["audience"], 256, "audience")
    text(record["rationale"], 1024, "rationale")
    need(type(record["navigation_depth"]) is int and record["navigation_depth"] == 3,
         "schema", "schema-1 navigation depth")
    authority = keys(record["authority"], {"task", "reason", "base_commit", "baseline", "target", "delta"},
                     "authority fields")
    task = text(authority["task"], 128, "task")
    need(re.fullmatch(r"[A-Z][A-Z0-9-]*\.[A-Za-z0-9][A-Za-z0-9_-]*(?:\.[A-Za-z0-9][A-Za-z0-9_-]*)*", task) is not None,
         "schema", "owning task id")
    text(authority["reason"], 512, "reason")
    revision = text(authority["base_commit"], 64, "base revision")
    need(re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", revision) is not None, "schema", "base revision")
    before, after = snapshot(authority["baseline"]), snapshot(authority["target"])
    delta = keys(authority["delta"], {"files", "lines", "bytes"}, "signed delta fields")
    for name in delta:
        value = integer(delta[name], -5000000, 5000000, f"delta {name}")
        need(value == getattr(after, name) - getattr(before, name), "delta", f"inexact {name}")
    return record


def registry_profile(tree: Path) -> None:
    try:
        source = read_file(tree, REGISTRY, REGISTRY_BYTES).decode("utf-8")
    except UnicodeError as issue:
        raise Refusal("schema", "registry encoding") from issue
    rows = {}
    for line in source.splitlines():
        if not line or line.startswith("#"):
            continue
        fields = line.split("\t")
        need(len(fields) == 21 and fields[0] not in rows, "schema", "registry row shape/identity")
        rows[fields[0]] = fields
        need(len(rows) <= 64, "budget", "registry row census")
    need("book_collection" in rows and "book_summary" in rows, "profile", "book/index rows missing")
    book, index = rows["book_collection"], rows["book_summary"]
    need(book[1:4] == [SOURCE + "/*.md", "collection", "maintained_reference"] and
         book[13:19] == ["700", "40960", "440", "-", "-", "-"],
         "profile", "book requires frozen part bounds and explicitly inapplicable aggregate targets")
    need(index[1:4] == [INDEX, "file", "bounded_snapshot"] and index[13:16] == ["120", "8192", "240"],
         "profile", "mandatory index bounds changed")


def content_bounds(data: bytes, bounds: tuple[int, int, int], relative: str) -> None:
    need(data.count(b"\n") <= bounds[0] and len(data) <= bounds[1] and
         max((len(line.removesuffix(b"\r")) for line in data.split(b"\n")), default=0) <= bounds[2],
         "part", f"part/index bounds exceeded: {relative}")
    try:
        data.decode("utf-8")
    except UnicodeError as issue:
        raise Refusal("encoding", f"non-UTF-8 Markdown: {relative}") from issue


def navigation(data: bytes) -> set[str]:
    content_bounds(data, INDEX_BOUNDS, INDEX)
    chapters = set()
    previous = 0
    for line in data.decode("utf-8").splitlines():
        if not line.strip() or re.fullmatch(r"#{1,6} .+", line):
            continue
        match = re.fullmatch(r"( *)(?:- )?\[([^\]\r\n]+)\]\(([^()\r\n]+)\)", line)
        need(match is not None, "navigation", "unsupported mandatory-index syntax")
        indent, label, href = match.groups()
        need(bool(label.strip()) and len(indent) % 2 == 0, "navigation", "label/indentation")
        depth = len(indent) // 2 + 1
        need(depth <= 3 and depth <= previous + 1, "navigation", "depth/jump exceeds contract")
        previous = depth
        portable(href)
        need(href.endswith(".md"), "navigation", "chapter must be a local Markdown path")
        relative = SOURCE + "/" + href
        need(relative != INDEX and relative not in chapters, "navigation", "duplicate/self chapter")
        chapters.add(relative)
    need(bool(chapters), "navigation", "empty mandatory index")
    return chapters


def summarize(parts: dict[str, bytes]) -> Snapshot:
    expected = navigation(parts[INDEX]) | {INDEX}
    need(set(parts) == expected, "navigation", "missing/unindexed Markdown chapter")
    digest = hashlib.sha256(FRAME)
    lines = size = 0
    for relative in sorted(parts):
        portable(relative)
        data = parts[relative]
        content_bounds(data, INDEX_BOUNDS if relative == INDEX else PART_BOUNDS, relative)
        encoded = relative.encode("utf-8")
        digest.update(str(len(encoded)).encode("ascii") + b":" + encoded + b"\0")
        digest.update(str(len(data)).encode("ascii") + b":" + data + b"\0")
        lines += data.count(b"\n")
        size += len(data)
    return Snapshot(len(parts), lines, size, digest.hexdigest())


def filesystem_snapshot(tree: Path) -> Snapshot:
    need(stat.S_ISDIR(metadata(tree, SOURCE).st_mode), "path", "book source is not a directory")
    index = read_file(tree, INDEX, INDEX_BOUNDS[1])
    expected = navigation(index) | {INDEX}
    actual = set()
    pending = [tree / SOURCE]
    seen = 0
    while pending:
        directory = pending.pop()
        relative = directory.relative_to(tree).as_posix()
        need(len(PurePosixPath(relative).parts) <= 16, "budget", "source directory depth")
        try:
            with os.scandir(directory) as entries:
                names = []
                for entry in entries:
                    seen += 1
                    need(seen <= CENSUS_ENTRIES, "budget", "source entry census")
                    names.append(entry.name)
        except OSError as issue:
            raise Refusal("path", "cannot enumerate source") from issue
        for name in sorted(names):
            path = directory / name
            relative = path.relative_to(tree).as_posix()
            info = metadata(tree, relative)
            if stat.S_ISDIR(info.st_mode):
                need(not (path / ".git").exists(), "path", "nested repository")
                pending.append(path)
            else:
                need(stat.S_ISREG(info.st_mode) and info.st_nlink == 1, "path", "nonregular/shared source")
                if name.endswith(".md"):
                    actual.add(relative)
                    need(len(actual) <= INDEX_BOUNDS[0] + 1, "budget", "index cannot cover this population")
    need(actual == expected, "navigation", "missing/unindexed Markdown chapter")
    parts = {relative: read_file(tree, relative, INDEX_BOUNDS[1] if relative == INDEX else PART_BOUNDS[1])
             for relative in sorted(actual)}
    need(parts[INDEX] == index, "changed", "mandatory index changed during census")
    return summarize(parts)


def git_entries(listing: bytes) -> list[str]:
    result = []
    for entry in listing.split(b"\0"):
        if not entry:
            continue
        fields = entry.split(b"\t", 1)
        need(len(fields) == 2, "git", "native tree entry shape")
        header = fields[0].split()
        need(len(header) == 3 and header[0] in (b"100644", b"100755") and header[1] == b"blob",
             "path", "Git links, submodules and nonregular entries are outside the book contract")
        try:
            name = fields[1].decode("utf-8")
        except UnicodeError as issue:
            raise Refusal("path", "Git source path encoding") from issue
        result.append(portable(name))
    need(len(result) == len(set(result)), "git", "duplicate native source path")
    return result


def git_snapshot(root: Path, revision: str) -> Snapshot:
    index = blob(root, revision, INDEX, INDEX_BOUNDS[1])
    expected = navigation(index) | {INDEX}
    listing = git(root, ["ls-tree", "-rz", revision, "--", SOURCE], LISTING_BYTES)
    names = git_entries(listing)
    need(len(names) <= CENSUS_ENTRIES, "budget", "Git source census")
    actual = set()
    for name in names:
        portable(name)
        if name.endswith(".md"):
            actual.add(name)
    need(actual == expected, "navigation", "Git baseline has missing/unindexed Markdown")
    return summarize({name: blob(root, revision, name, INDEX_BOUNDS[1] if name == INDEX else PART_BOUNDS[1])
                      for name in sorted(actual)})


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


@dataclass(frozen=True)
class Verification:
    mode: str
    snapshot: Snapshot
    owner: str


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


def verify_record(root: Path, tree: Path, raw: bytes) -> Verification:
    record = parse_control(raw)
    registry_profile(tree)
    authority = record["authority"]
    current = filesystem_snapshot(tree)
    need(current == snapshot(authority["target"]), "target", "current book differs from exact authority")
    head = git(root, ["rev-parse", "HEAD"], 128).decode("ascii").strip()
    at_head = blob(root, head, CONTROL, CONTROL_BYTES, absent=True)
    base = authority["base_commit"]
    if raw != at_head:
        mode = "candidate"
        need(base == head, "freshness", "candidate authority must use exact HEAD baseline")
    else:
        mode = "committed"
        commit = git(root, ["log", "--first-parent", "-1", "--format=%H", "HEAD", "--", CONTROL], 128)
        commit = commit.decode("ascii").strip()
        need(bool(commit), "freshness", "authority change commit absent")
        parent = git(root, ["rev-parse", "--verify", commit + "^1"], 128).decode("ascii").strip()
        need(parent == base, "freshness", "authority is stale or reused")
    baseline = git_snapshot(root, base)
    need(baseline == snapshot(authority["baseline"]), "baseline", "native Git baseline differs")
    task = authority["task"]
    task_path = "docs/tasks/" + task.split(".", 1)[0] + ".md"
    before = task_status(blob(root, base, task_path, TASK_BYTES, absent=True), task)
    after = task_status(read_file(tree, task_path, TASK_BYTES), task)
    need(before in (None, "proposed", "pending", "active", "in_progress") and after == "done",
         "task", "authority owner must be newly completed, never a reused closed task")
    return Verification(mode, current, task)


def check(root: Path = ROOT, tree: Path | None = None) -> Verification:
    root, tree = selected_tree(root, tree)
    return verify_record(root, tree, read_file(tree, CONTROL, CONTROL_BYTES))


def encode_record(record: dict) -> bytes:
    header = dict(schema_version=1, max_records=1, max_bytes=CONTROL_BYTES, max_record_bytes=RECORD_BYTES)
    raw = (json.dumps(header, sort_keys=True) + "\n" +
           json.dumps(record, ensure_ascii=False, sort_keys=True) + "\n").encode("utf-8")
    parse_control(raw)
    return raw


def record_change(task: str, reason: str, root: Path = ROOT, tree: Path | None = None) -> Verification:
    """Record one exact current proposal; all validation precedes the sole canonical replacement.

    The control is authoritative tracked metadata. Regenerable temporary bytes live only in target/;
    the shared producer planner guards its complete source/output plan before the first write.
    This records numeric change authority, never a human approval or an executable command grant.
    """
    root, tree = selected_tree(root, tree)
    need(root == ROOT.resolve(strict=True), "path", "authoring baseline must be this repository")
    need(tree == root or tree.is_relative_to(root / "target"), "path", "authoring candidate must be under target")
    text(task, 128, "task")
    text(reason, 512, "reason")
    registry_profile(tree)
    head = git(root, ["rev-parse", "HEAD"], 128).decode("ascii").strip()
    baseline = git_snapshot(root, head)
    current = filesystem_snapshot(tree)
    destination = tree / CONTROL
    old = read_file(tree, CONTROL, CONTROL_BYTES) if os.path.lexists(destination) else None
    at_head = blob(root, head, CONTROL, CONTROL_BYTES, absent=True)
    if old is not None:
        previous = parse_control(old)
        if old != at_head:
            need(previous["authority"]["task"] == task and previous["authority"]["base_commit"] == head,
                 "conflict", "uncommitted authority belongs to another task or baseline")
    record = {
        "surface": "book_collection", "audience": "StitchCAD book readers and maintainers",
        "rationale": "Unique maintained product/specification prose; bounded parts and complete direct navigation",
        "navigation_depth": 3,
        "authority": {"task": task, "reason": reason, "base_commit": head, "baseline": asdict(baseline),
                      "target": asdict(current),
                      "delta": {key: getattr(current, key) - getattr(baseline, key)
                                for key in ("files", "lines", "bytes")}},
    }
    raw = encode_record(record)
    result = verify_record(root, tree, raw)
    need(result.mode == "candidate", "freshness", "recording requires a fresh candidate authority")
    task_path = "docs/tasks/" + task.split(".", 1)[0] + ".md"
    chapters = navigation(read_file(tree, INDEX, INDEX_BOUNDS[1])) | {INDEX}
    sources = {tree / name for name in chapters | {REGISTRY, task_path}}
    sources.update((Path(__file__).resolve(strict=True), root / "scripts/local_environment.py"))
    if old is not None:
        sources.add(destination)
    # No imported book or JSON content is executable. This is the project's trusted planner module.
    profile_source = read_file(root, "scripts/local_environment.py", 65536)
    profile = dict(__file__=str(root / "scripts/local_environment.py"), __name__="maintained_book_profile")
    exec(compile(profile_source, profile["__file__"], "exec"), profile)
    relative_base = (tree.relative_to(root) / "target/maintained-reference-authoring").as_posix()
    temporary = root / relative_base / ("authority-" + uuid.uuid4().hex + ".jsonl")
    options = dict(sources=tuple(sorted(sources)), mutable_sources=(destination,) if old is not None else (),
                   directories=(root / relative_base,), output_files=(temporary,))
    def plan():
        try:
            profile["plan_producer"](root, **options)
        except ValueError as issue:
            raise Refusal("plan", str(issue)) from issue

    plan()
    frozen = {path: read_file(root, path.relative_to(root).as_posix(), max(TASK_BYTES, 65536))
              for path in sources if path != destination}
    created = None
    try:
        profile["directory"](root, relative_base, create=True)
        descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | getattr(os, "O_NOFOLLOW", 0), 0o600)
        created = os.fstat(descriptor)
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        # Native HEAD, full content/ownership and every producer source are fresh at publication.
        need(git(root, ["rev-parse", "HEAD"], 128).decode("ascii").strip() == head,
             "changed", "HEAD changed during recording")
        for path, data in frozen.items():
            need(read_file(root, path.relative_to(root).as_posix(), max(TASK_BYTES, 65536)) == data,
                 "changed", "authoring input changed before publication")
        verify_record(root, tree, raw)
        present = read_file(tree, CONTROL, CONTROL_BYTES) if os.path.lexists(destination) else None
        need(present == old, "conflict", "authority destination changed before publication")
        plan()
        # Source/destination were separately guarded on this volume; replace never follows a leaf link.
        os.replace(temporary, destination)
        created = None
        return result
    except Refusal:
        raise
    except ValueError as issue:
        raise Refusal("plan", str(issue)) from issue
    except OSError as issue:
        raise Refusal("write", "authority publication failed; previous canonical record retained") from issue
    finally:
        if created is not None:
            try:
                remaining = temporary.lstat()
            except FileNotFoundError:
                pass
            else:
                need((remaining.st_dev, remaining.st_ino) == (created.st_dev, created.st_ino) and
                     stat.S_ISREG(remaining.st_mode) and remaining.st_nlink == 1,
                     "changed", "temporary identity changed; refusing cleanup")
                temporary.unlink()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate", help="repository-relative ordinary candidate directory")
    actions = parser.add_subparsers(dest="action")
    author = actions.add_parser("record-change", help="atomically record exact fresh book-change authority")
    author.add_argument("--task", required=True)
    author.add_argument("--reason", required=True)
    arguments = parser.parse_args()
    try:
        tree = ROOT / portable(arguments.candidate) if arguments.candidate else ROOT
        result = (record_change(arguments.task, arguments.reason, ROOT, tree)
                  if arguments.action == "record-change" else check(ROOT, tree))
    except (Refusal, OSError, UnicodeError) as issue:
        print(str(issue))
        return 2
    print(f"maintained-reference: OK — {result.mode}; {result.snapshot.files} files; "
          f"{result.snapshot.lines} lines; {result.snapshot.bytes} bytes; owner {result.owner}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
