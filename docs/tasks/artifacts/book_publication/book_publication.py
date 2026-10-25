"""Check publication topology, local navigation and a scoped current-library status map.

This checks references/status markers, not correctness of all prose or sewing semantics. The public
API map is deliberately scoped; contract tests and later gates retain execution/certification proof.
Copied-fixture mutations prove the named refusals. No tracked source or other Git repo is edited.
"""
from __future__ import annotations

import csv
from contextlib import contextmanager
import hashlib
from html.parser import HTMLParser
import json
import os
from pathlib import Path
import re
import runpy
import shutil
import subprocess
import uuid
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[4]
SOURCE = Path("docs/book/src")
HTML = Path("docs/book/book")
MAP = Path("docs/tasks/artifacts/book_publication/status.tsv")
LEARNING = [
    "learn/design-to-pattern.md", "learn/measurements-and-fit.md",
    "learn/pieces-and-assembly.md", "learn/sizes-and-grading.md",
    "learn/agents-and-workflows.md", "spec/mtm-input-charts.md", "availability.md",
]
# Finite current-scope contracts, not a general natural-language correctness oracle.
# These retired claims contradicted already published public APIs (D157).
FORMULA_SCOPE = {
    "syntax": ("type/name validation and evaluation remain G1-SLICE.5 work",
               "Product normalization/evaluation",
               "Production normalization/serialization/evaluation",
               "namespace/whole static preflight remain .5b.1b/.1c"),
    "declarations": ("ordered name reads/bindings remain .2d",
                     "A recipe has no product static acceptance",
                     "expression/whole checks remain .3/.4",
                     "metadata for the future checker",
                     "independently exercised in the reference before product namespace implementation",
                     "expression kind/signature checking and complete static dependency graphs remain .3/.4",
                     "Expression type checking and whole acceptance remain .5b.3/.4"),
    "operator-signatures": ("complete expression checking and whole recipe acceptance remain .5b.3b/.3c/.4",
                            "the contextual expression checker remains the next owner",
                            "The contextual checker will use this metadata",
                            "conditional checking, contextual dimensional errors and complete static graphs remain later work"),
    "call-lookup": ("statement integration, whole-recipe validation and execution remain separate work",),
    "literals": ("complete recipe normalization/identity, binding and evaluation remain later work",
                 "Name/type/binding/evaluation"),
    "recipe-inputs": ("name/type checking, binding and evaluation remain G1-SLICE.5 work",
                      "remaining implementation is owned by .5b–.5g"),
    "runtime-validation": (),
    "statements": ("complete recipe validation and evaluation remain G1-SLICE.5 work",
                   "recipe normalization/identity .3f, numerical execution and production approval remain owned future work",
                   "statement serialization, numerical evaluation and production approval remain later",
                   "The recipe has no persistent statement serialization or recipe hash yet; .3f.1 owns that byte contract"),
    "static-validation": ("complete static graph remains .5b.4",),
}
FORMULA_METHODS = {
    "checked.rs": ("check_kinds",),
    "checked_recipe.rs": ("check_kinds",),
    "namespace/ordered.rs": ("check_kinds", "advance_metadata"),
}


class PublicationError(Exception):
    """A named structural/status refusal with its offending target."""


class Page(HTMLParser):
    def __init__(self):
        super().__init__()
        self.ids = set()
        self.links = []

    def handle_starttag(self, tag, attrs):
        attributes = dict(attrs)
        if "id" in attributes:
            self.ids.add(attributes["id"])
        if tag == "a" and "href" in attributes:
            self.links.append(attributes["href"])


def require(condition, code, detail):
    if not condition:
        raise PublicationError(f"{code}: {detail}")


PROFILE_SOURCE = ROOT / "scripts/local_environment.py"


def load_profile():
    parent = PROFILE_SOURCE.parent
    require(not parent.is_symlink() and parent.is_dir() and parent.stat().st_dev == ROOT.stat().st_dev and
            not ((parent / ".git").exists() or (parent / ".git").is_symlink()) and
            not PROFILE_SOURCE.is_symlink() and PROFILE_SOURCE.is_file() and
            PROFILE_SOURCE.stat().st_dev == ROOT.stat().st_dev,
            "FIXTURE_AUTHORITY", "unsafe publication profile source")
    return runpy.run_path(str(PROFILE_SOURCE))


PROFILE = load_profile()
CORE = (Path(__file__).absolute(), PROFILE_SOURCE)


def source_tree(path):
    """Resolve a finite input tree without following aliases or foreign Git directories."""
    folders, files = [Path(path).absolute()], []
    for folder in folders:
        PROFILE["directory"](ROOT, PROFILE["producer_path"](ROOT, folder))
        for child in sorted(folder.iterdir()):
            PROFILE["require"](not child.is_symlink(), "symlink publication input")
            (folders if child.is_dir() else files).append(child)
    return tuple(folders), tuple(files)


def fixture_plan(root, destination):
    """Snapshot the full copied population before the first destination effect."""
    try:
        root, destination = Path(root).absolute(), Path(destination).absolute()
        PROFILE["plan_producer"](ROOT, sources=(*CORE, root / MAP))
        current_rows = rows(root)
        require(all(len(row) == 6 for row in current_rows), "FIXTURE_AUTHORITY", "invalid copied status row")
        source_dirs, source_files = source_tree(root / SOURCE)
        _, rendered = source_tree(root / HTML)
        PROFILE["directory"](ROOT, PROFILE["producer_path"](ROOT, root / "docs/tasks"))
        copied = {root / "docs/book/book.toml", *source_files,
                  *(root / "docs/tasks").glob("*.md"), *(file for file in rendered if file.suffix == ".html"),
                  root / "ROADMAP.md", root / MAP, root / "crates/sc-core/tests/formula_statement_contract.rs",
                  *(root / row[2] for row in current_rows)}
        for file in copied:
            PROFILE["producer_path"](root, file)
        inputs = {*CORE, *copied}
        outputs = {destination / file.relative_to(root) for file in copied}
        directories = {destination, destination / "docs/book", destination / "docs/tasks", destination / HTML,
                       *(destination / SOURCE / folder.relative_to(root / SOURCE) for folder in source_dirs)}
        for output in outputs:
            parent = output.parent
            while parent.is_relative_to(destination):
                directories.add(parent)
                if parent == destination:
                    break
                parent = parent.parent
        require(not any(file.is_relative_to(destination) for file in inputs),
                "FIXTURE_AUTHORITY", "copied destination overlaps inputs")
        require(not destination.is_relative_to(root / SOURCE) and not destination.is_relative_to(root / HTML),
                "FIXTURE_AUTHORITY", "copied destination is inside input traversal")
        PROFILE["plan_producer"](ROOT, sources=tuple(sorted(inputs)), directories=tuple(sorted(directories)),
                                 output_files=tuple(sorted(outputs)))
        return dict(inputs=inputs, outputs=outputs, directories=directories, root=root,
                    destination=destination, source_files=source_files, source_dirs=source_dirs, rows=current_rows)
    except (PROFILE["Refusal"], OSError) as error:
        raise PublicationError("FIXTURE_AUTHORITY: " + str(error)) from None


def fixture_effect(plan, *, sources=(), files=(), directories=(), activate=False):
    """Recheck the selected effect against the complete frozen copy population."""
    try:
        PROFILE["require"](set(sources) <= plan["inputs"] and set(files) <= plan["outputs"] and
                           set(directories) <= plan["directories"], "publication effect outside copied population")
        if activate:
            sources, files, directories = plan["inputs"], plan["outputs"], plan["directories"]
        PROFILE["plan_producer"](ROOT, sources=tuple(sorted({*CORE, plan["root"] / MAP, *sources})),
                                 directories=tuple(sorted(directories)), output_files=tuple(sorted(files)))
        environment = PROFILE["environment_for"](ROOT, os.environ, create=activate)
        if activate:
            os.environ.update(environment)
    except (PROFILE["Refusal"], OSError) as error:
        raise PublicationError("FIXTURE_AUTHORITY: " + str(error)) from None


def mutation_plan(file, *, read=True, activate=False):
    try:
        file = Path(file).absolute()
        sources = (*CORE, file) if read or file.exists() else CORE
        mutable = (file,) if file in sources else ()
        PROFILE["plan_producer"](ROOT, sources=sources, mutable_sources=mutable, output_files=(file,))
        environment = PROFILE["environment_for"](ROOT, os.environ, create=activate)
        if activate:
            os.environ.update(environment)
    except (PROFILE["Refusal"], OSError) as error:
        raise PublicationError("FIXTURE_AUTHORITY: " + str(error)) from None


def mutation_target(root, name):
    if name.startswith("retired formula claim:"):
        chapter, _ = name.removeprefix("retired formula claim:").split(":")
        return root / SOURCE / "annexes" / f"formula-{chapter}.md"
    if name.startswith("missing formula method:"):
        file, _ = name.removeprefix("missing formula method:").split(":")
        return root / "crates/sc-core/src/recipe" / file
    paths = {
        "missing formula proof boundary": SOURCE / "annexes/formula-runtime-validation.md",
        "stale statement contract count": SOURCE / "annexes/formula-statements.md",
        "missing statement contract marker": Path("crates/sc-core/tests/formula_statement_contract.rs"),
        "unregistered chapter": SOURCE / "unregistered.md",
        "missing indexed chapter": SOURCE / "topic-index.md",
        "missing source target": SOURCE / "introduction.md",
        "missing source anchor": SOURCE / "introduction.md",
        "missing rendered chapter": HTML / "learn/design-to-pattern.html",
        "missing rendered anchor": HTML / "introduction.html",
        "stale G0 status": SOURCE / "introduction.md",
        "missing public MTM API": Path("crates/sc-measure/src/mtm_chart.rs"),
    }
    if name not in paths:
        raise PublicationError(f"unknown fixture mutation: {name}")
    return root / paths[name]


@contextmanager
def temporary_root(*, prefix, dir, fixture_root=None):
    """Choose and validate an exclusive owned root before creation; inspect all teardown files."""
    destination = Path(dir).absolute() / (prefix + uuid.uuid4().hex)
    try:
        PROFILE["plan_producer"](ROOT, sources=CORE, directories=(destination.parent, destination))
        require(not destination.exists(), "FIXTURE_AUTHORITY", "temporary root already exists")
        if fixture_root is not None:
            fixture_plan(fixture_root, destination)
        environment = PROFILE["environment_for"](ROOT, os.environ)
        os.environ.update(environment)
        PROFILE["directory"](ROOT, PROFILE["producer_path"](ROOT, destination.parent), create=True)
        destination.mkdir()
    except (PROFILE["Refusal"], OSError) as error:
        raise PublicationError("FIXTURE_AUTHORITY: " + str(error)) from None
    try:
        yield destination
    finally:
        try:
            folders, files = source_tree(destination)
            PROFILE["plan_producer"](ROOT, sources=(*CORE, *files), directories=folders,
                                     output_files=files)
            shutil.rmtree(destination)
        except (PROFILE["Refusal"], OSError) as error:
            raise PublicationError("FIXTURE_AUTHORITY: " + str(error)) from None


def build_inputs(root):
    """Close the supported renderer input/configuration surface without activating stores."""
    root = Path(root).absolute()
    PROFILE["producer_path"](ROOT, root)
    PROFILE["workspace"](root)
    config = root / "docs/book/book.toml"
    PROFILE["plan_producer"](ROOT, sources=(*CORE, config))
    # The current public configuration has metadata and one built-in HTML renderer. New path or
    # command options need their own input/delegation plan before this boundary can accept them.
    section, seen = None, set()
    fields = {"book": {"title", "description", "authors", "language", "src"},
              "output.html": {"default-theme", "git-repository-url"}}
    for line in config.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        if line in ("[book]", "[output.html]"):
            section = line[1:-1]
            require(section not in seen, "BOOK_AUTHORITY", "duplicate configuration section")
            seen.add(section)
            continue
        field, separator, value = line.partition("=")
        field = field.strip()
        require(separator and field in fields.get(section, ()), "BOOK_AUTHORITY", "unplanned book configuration")
        try:
            value = json.loads(value.strip())
        except ValueError:
            raise PublicationError("BOOK_AUTHORITY: unsupported configuration value") from None
        require((isinstance(value, list) and all(isinstance(item, str) for item in value)) if field == "authors"
                else isinstance(value, str), "BOOK_AUTHORITY", "invalid book metadata type")
        if field == "src":
            require(value == "src", "BOOK_AUTHORITY", "unplanned source directory")
    require(seen == set(fields), "BOOK_AUTHORITY", "missing book/HTML configuration")
    require(not any(key.upper().startswith("MDBOOK_") and key != "MDBOOK_LOG" for key in os.environ),
            "BOOK_AUTHORITY", "unplanned mdBook environment override")
    theme = root / "docs/book/theme"
    require(not theme.exists() and not theme.is_symlink(), "BOOK_AUTHORITY", "unplanned theme inputs")
    folders, files = source_tree(root / SOURCE)
    PROFILE["plan_producer"](ROOT, sources=(*CORE, config, *files))
    for file in files:
        if file.suffix == ".md":
            require("{{#" not in file.read_text(), "BOOK_AUTHORITY", "unplanned preprocessor input")
    for chapter in chapters((root / SOURCE / "SUMMARY.md").read_text()):
        PROFILE["producer_path"](ROOT, root / SOURCE / chapter)
        require(root / SOURCE / chapter in files, "BOOK_AUTHORITY", "unplanned or missing summary chapter")
    return folders, (config, *files)


def publication_environment(*, create=False):
    """Keep stores/caller variables while binding child Git observations to this checkout."""
    environment = {key: value for key, value in PROFILE["environment_for"](ROOT, os.environ, create=create).items()
                   if not key.upper().startswith("GIT_")}
    environment.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull, GIT_OPTIONAL_LOCKS="0",
                       GIT_LITERAL_PATHSPECS="1", GIT_TERMINAL_PROMPT="0")
    return environment


def publication_plan(root, stage=None):
    """The sole public-output exception: inspect the entire exact docs/book/book tree."""
    output = root / HTML
    PROFILE["directory"](ROOT, PROFILE["producer_path"](ROOT, output), missing_ok=True)
    folders, files = source_tree(output) if output.exists() else ((), ())
    PROFILE["plan_producer"](ROOT, sources=(*CORE, *files), mutable_sources=files)
    tracked = subprocess.check_output(["git", "ls-files", "-z", "--", PROFILE["producer_path"](ROOT, output)],
                                      cwd=ROOT, env=publication_environment())
    require(not tracked, "BOOK_AUTHORITY", "public output contains tracked data")
    if stage is not None:
        generated_dirs, generated_files = source_tree(stage)
        PROFILE["plan_producer"](ROOT, sources=(*CORE, *generated_files), output_files=generated_files)
        required = {stage / Path(chapter).with_suffix(".html")
                    for chapter in chapters((root / SOURCE / "SUMMARY.md").read_text())}
        require(required <= set(generated_files), "BOOK_BUILD", "renderer omitted a summary chapter")
        owned = {*folders, *files}
        for path in (*generated_dirs, *generated_files):
            target = output / path.relative_to(stage)
            PROFILE["producer_path"](ROOT, target)
            # The old tree is replaced in full: an owned file may become a directory or vice versa.
            require(not target.is_symlink() and (not target.exists() or target in owned),
                    "BOOK_AUTHORITY", "unsafe generated publication destination")
    return folders, files


def build(root):
    """Render privately, enumerate dynamic output, then publish to the existing public destination."""
    try:
        root = Path(root).absolute()
        input_dirs, inputs = build_inputs(root)
        identities = {file: hashlib.sha256(file.read_bytes()).digest() for file in inputs}
        publication_plan(root)
        with temporary_root(prefix="book-builder-", dir=ROOT / "target/scratch") as stage:
            build_inputs(root)
            publication_plan(root)
            environment = dict(publication_environment(create=True), MDBOOK_LOG="warn", RUST_LOG="warn",
                               MDBOOK_BUILD__BUILD_DIR=json.dumps(str(stage)), MDBOOK_BUILD__CREATE_MISSING="false")
            result = subprocess.run(["mdbook", "build", "docs/book"], cwd=root, env=environment,
                                    capture_output=True, text=True)
            diagnostic = result.stdout + result.stderr
            require(result.returncode == 0, "BOOK_BUILD", diagnostic.strip())
            plain = re.sub(r"\x1b\[[0-9;]*m", "", diagnostic)
            warnings = re.findall(r"(?m)^\s*WARN\b.*$", plain)
            require(not warnings, "BOOK_BUILD_WARNING", "\n".join(warnings))
            require(build_inputs(root) == (input_dirs, inputs) and all(hashlib.sha256(file.read_bytes()).digest() == digest
                    for file, digest in identities.items()), "BOOK_AUTHORITY", "builder changed publication inputs")
            publication_plan(root, stage)
            if (root / HTML).exists():
                shutil.rmtree(root / HTML)
            publication_plan(root, stage)
            shutil.copytree(stage, root / HTML)
        return diagnostic
    except (PROFILE["Refusal"], OSError, UnicodeError, subprocess.SubprocessError) as error:
        raise PublicationError("BOOK_AUTHORITY: " + str(error)) from None


def markdown_links(text):
    # The book uses inline local Markdown links; external links and non-chapter assets are excluded.
    return re.findall(r"\]\(([^\s)]+)", text)


def chapters(text):
    return [href.split("#", 1)[0] for href in markdown_links(text)
            if href.split("#", 1)[0].endswith(".md")]


def rows(root):
    with (root / MAP).open(newline="") as stream:
        return list(csv.reader((line for line in stream if not line.startswith("#")), delimiter="\t"))


def method_pattern(method):
    return r"^    pub fn " + re.escape(method) + r"(?:<[^>\n]*>)?\("


def formula_scope(root):
    """Watch known current availability clauses without rejecting scoped runtime limits."""
    for name, retired in FORMULA_SCOPE.items():
        chapter = f"annexes/formula-{name}.md"
        text = (root / SOURCE / chapter).read_text()
        prose = " ".join(re.sub(r"(?m)^>\s*", "", text).split())
        for claim in retired:
            require(claim not in prose, "FORMULA_SCOPE", f"{chapter}: retired claim {claim}")
        introduction = text.split("\n## ", 1)[0]
        require("formula-checked-recipes.md" in markdown_links(introduction),
                "FORMULA_SCOPE", f"{chapter}: missing complete kind-proof boundary")
    for file, methods in FORMULA_METHODS.items():
        source = root / "crates/sc-core/src/recipe" / file
        for method in methods:
            require(re.search(method_pattern(method), source.read_text(), re.M),
                    "FORMULA_API", f"{file}: missing public {method}")
    chapter = (root / SOURCE / "annexes/formula-statements.md").read_text()
    declared = re.search(r"(?m)^(\d+) public contracts verify fifteen independently authored", chapter)
    tests = (root / "crates/sc-core/tests/formula_statement_contract.rs").read_text()
    population = len(re.findall(r"(?m)^#\[test\]$", tests))
    require(declared is not None and int(declared[1]) == population,
            "FORMULA_COUNT", f"statement contracts: declared {declared[1] if declared else 'absent'}, source {population}")


def check(root):
    source = (root / SOURCE).resolve()
    output = (root / HTML).resolve()
    summary = (source / "SUMMARY.md").read_text()
    registered = chapters(summary)
    population = {str(file.relative_to(source)) for file in source.rglob("*.md")
                  if file.name != "SUMMARY.md"}
    require(len(set(registered)) == len(registered), "CHAPTER_COVERAGE", "duplicate registration")
    require(set(registered) == population, "CHAPTER_COVERAGE",
            f"unregistered {sorted(population - set(registered))}; missing {sorted(set(registered) - population)}")
    require(registered[1:8] == LEARNING, "LEARNING_ORDER", "progressive chapters must precede references")
    for part in ("# Learning path", "# Glossary", "# Annexes", "# Index"):
        require(part in summary, "PUBLICATION_PART", part)
    annex_start = summary.index("# Annexes")
    index_start = summary.index("# Index")
    for chapter in registered:
        if chapter.startswith("spec/") and chapter not in ("spec/glossary.md", "spec/mtm-input-charts.md") \
                and not chapter.startswith("spec/glossary/"):
            position = summary.index(f"]({chapter})")
            require(annex_start < position < index_start, "ANNEX_SCOPE", chapter)
    indexed = set(chapters((source / "topic-index.md").read_text()))
    require(set(registered) - {"topic-index.md"} <= indexed, "INDEX_COVERAGE",
            sorted(set(registered) - {"topic-index.md"} - indexed))
    pages = {}
    for chapter in registered:
        path = output / Path(chapter).with_suffix(".html")
        require(path.is_file(), "RENDER_CHAPTER", chapter)
        page = Page()
        page.feed(path.read_text())
        pages[path.resolve()] = page
    source_links = 0
    rendered_links = 0
    for chapter in registered + ["SUMMARY.md"]:
        file = source / chapter
        for href in markdown_links(file.read_text()):
            url = urlsplit(href)
            if url.scheme or url.netloc or not url.path.endswith(".md"):
                continue
            target = (file.parent / unquote(url.path)).resolve()
            require(target.is_relative_to(source) and target.is_file(), "SOURCE_LINK", f"{chapter}: {href}")
            if url.fragment:
                rendered = output / target.relative_to(source).with_suffix(".html")
                require(rendered in pages and unquote(url.fragment) in pages[rendered].ids,
                        "SOURCE_ANCHOR", f"{chapter}: {href}")
            source_links += 1
    for file, page in pages.items():
        for href in page.links:
            url = urlsplit(href)
            if url.scheme or url.netloc or (url.path and not url.path.endswith((".html", ".md"))):
                continue
            target = (file.parent / unquote(url.path)).resolve() if url.path else file
            require(target.is_relative_to(output) and target.is_file(), "RENDER_LINK", f"{file.name}: {href}")
            if url.fragment:
                require(target in pages and unquote(url.fragment) in pages[target].ids,
                        "RENDER_ANCHOR", f"{file.name}: {href}")
            rendered_links += 1
    owners = "\n".join(file.read_text() for file in (root / "docs/tasks").glob("*.md"))
    roadmap = (root / "ROADMAP.md").read_text()
    require("10. **Documentation in lockstep.**" in roadmap, "POLICY", "roadmap publication principle")
    current_rows = rows(root)
    seen = set()
    for row in current_rows:
        require(len(row) == 6, "STATUS_MAP", row)
        family, chapter, code, symbol, owner, section = row
        require(family not in seen, "STATUS_MAP", f"duplicate {family}")
        seen.add(family)
        require(chapter in registered, "STATUS_MAP", chapter)
        path = root / code
        require(path.is_file(), "API_STATUS", code)
        require(re.search(r"^pub (?:struct|enum|trait) " + re.escape(symbol) + r"\b", path.read_text(), re.M),
                "API_STATUS", f"{family}: {code} lacks public {symbol}")
        require(f"- ID: `{owner}`" in owners, "STATUS_OWNER", owner)
        require(re.search(r"^#{2,3} " + re.escape(section) + r"(?:\. | )", roadmap, re.M),
                "STATUS_ROADMAP", section)
    intro = (source / "introduction.md").read_text()
    availability = (source / "availability.md").read_text()
    require("G1 executable foundations are in progress." in intro, "TEXT_STATUS", "intro G1 scope")
    require("not a finished drafting application" in intro and "not a claim that a feature ships today" in availability,
            "TEXT_STATUS", "existing libraries versus future specification")
    formula_scope(root)
    return len(registered), len(current_rows), source_links, rendered_links


def fixture(root, destination):
    root, destination = Path(root).absolute(), Path(destination).absolute()
    plan = fixture_plan(root, destination)
    fixture_effect(plan, directories=(destination / "docs/book",), activate=True)
    (destination / "docs/book").mkdir(parents=True)
    fixture_effect(plan, sources=(root / "docs/book/book.toml",), files=(destination / "docs/book/book.toml",))
    shutil.copy2(root / "docs/book/book.toml", destination / "docs/book/book.toml")
    for directory in (SOURCE, Path("docs/tasks")):
        # Artifacts are copied separately; evidence/tree Markdown is sufficient for ownership lookup.
        if directory == Path("docs/tasks"):
            target = destination / directory
            fixture_effect(plan, directories=(target,))
            target.mkdir(parents=True)
            for file in (root / directory).glob("*.md"):
                fixture_effect(plan, sources=(file,), files=(target / file.name,))
                shutil.copy2(file, target / file.name)
        else:
            current_dirs, current_files = source_tree(root / directory)
            require((current_dirs, current_files) == (plan["source_dirs"], plan["source_files"]),
                    "FIXTURE_AUTHORITY", "copied input traversal changed")
            fixture_effect(plan, sources=current_files,
                           files=tuple(destination / directory / file.relative_to(root / directory) for file in current_files),
                           directories=tuple(destination / directory / folder.relative_to(root / directory) for folder in current_dirs))
            shutil.copytree(root / directory, destination / directory)
    fixture_effect(plan, directories=(destination / HTML,))
    (destination / HTML).mkdir(parents=True)
    for file in (root / HTML).rglob("*.html"):
        target = destination / HTML / file.relative_to(root / HTML)
        fixture_effect(plan, directories=(target.parent,))
        target.parent.mkdir(parents=True, exist_ok=True)
        fixture_effect(plan, sources=(file,), files=(target,))
        shutil.copy2(file, target)
    for file in [Path("ROADMAP.md"), MAP, Path("crates/sc-core/tests/formula_statement_contract.rs")] + [Path(row[2]) for row in plan["rows"]]:
        target = destination / file
        fixture_effect(plan, directories=(target.parent,))
        target.parent.mkdir(parents=True, exist_ok=True)
        fixture_effect(plan, sources=(root / file,), files=(target,))
        shutil.copy2(root / file, target)


def replace(file, old, new):
    mutation_plan(file)
    text = file.read_text()
    require(text.count(old) == 1, "FIXTURE_ANCHOR", old)
    mutation_plan(file, activate=True)
    file.write_text(text.replace(old, new, 1))


def currency_refusal(checker, root):
    """Independent assertion on the actual stale fixture, shared by green and body-fault arms."""
    try:
        checker(root)
    except PublicationError as error:
        require(str(error).startswith("FORMULA_SCOPE:"), "WRONG_REFUSAL", str(error))
    else:
        raise PublicationError("FORMULA_BODY_RED: actual stale current claim was accepted")


def mutate(root, name):
    selected = mutation_target(root, name)
    mutation_plan(selected, read=name != "unregistered chapter")
    source = root / SOURCE
    if name.startswith("retired formula claim:"):
        chapter, ordinal = name.removeprefix("retired formula claim:").split(":")
        path = source / "annexes" / f"formula-{chapter}.md"
        text = path.read_text()
        intro, rest = text.split("\n## ", 1)
        mutation_plan(path, activate=True)
        path.write_text(intro + "\n\n" + FORMULA_SCOPE[chapter][int(ordinal)] + ".\n\n## " + rest)
    elif name.startswith("missing formula method:"):
        file, method = name.removeprefix("missing formula method:").split(":")
        path = root / "crates/sc-core/src/recipe" / file
        matches = re.findall(method_pattern(method), path.read_text(), re.M)
        require(len(matches) == 1, "FIXTURE_ANCHOR", name)
        replace(path, matches[0], matches[0].replace("pub fn", "fn", 1))
    elif name == "missing formula proof boundary":
        path = source / "annexes/formula-runtime-validation.md"
        text = path.read_text()
        intro, rest = text.split("\n## ", 1)
        mutation_plan(path, activate=True)
        path.write_text(intro.replace("formula-checked-recipes.md", "formula-recipe-inputs.md") + "\n## " + rest)
    elif name == "stale statement contract count":
        replace(source / "annexes/formula-statements.md", "10 public contracts verify", "9 public contracts verify")
    elif name == "missing statement contract marker":
        path = root / "crates/sc-core/tests/formula_statement_contract.rs"
        text = path.read_text()
        require("#[test]\n" in text, "FIXTURE_ANCHOR", name)
        mutation_plan(path, activate=True)
        path.write_text(text.replace("#[test]\n", "// removed test marker\n", 1))
    elif name == "unregistered chapter":
        mutation_plan(selected, read=False, activate=True)
        (source / "unregistered.md").write_text("# Stranded reader chapter\n")
    elif name == "missing indexed chapter":
        replace(source / "topic-index.md", "- [From an idea to a pattern](learn/design-to-pattern.md)\n", "")
    elif name == "missing source target":
        mutation_plan(selected, activate=True)
        with (source / "introduction.md").open("a") as stream:
            stream.write("\n[Missing](missing-chapter.md)\n")
    elif name == "missing source anchor":
        mutation_plan(selected, activate=True)
        with (source / "introduction.md").open("a") as stream:
            stream.write("\n[Missing](availability.md#absent-section)\n")
    elif name == "missing rendered chapter":
        mutation_plan(selected, activate=True)
        (root / HTML / "learn/design-to-pattern.html").unlink()
    elif name == "missing rendered anchor":
        mutation_plan(selected, activate=True)
        with (root / HTML / "introduction.html").open("a") as stream:
            stream.write('<a href="availability.html#absent-section">Missing</a>')
    elif name == "stale G0 status":
        replace(source / "introduction.md", "G1 executable foundations are in progress.", "The project is in gate G0.")
    elif name == "missing public MTM API":
        replace(root / "crates/sc-measure/src/mtm_chart.rs", "pub struct MtmChart {", "pub struct RetiredChart {")
    else:
        raise PublicationError(f"unknown fixture mutation: {name}")


def main_inputs(root):
    """Validate every non-rendered reader input before main's first producing call."""
    try:
        root = Path(root).absolute()
        PROFILE["workspace"](root)
        PROFILE["plan_producer"](ROOT, sources=(*CORE, root / MAP))
        current_rows = rows(root)
        require(all(len(row) == 6 for row in current_rows), "BOOK_AUTHORITY", "invalid status row")
        _, source_files = source_tree(root / SOURCE)
        PROFILE["directory"](ROOT, PROFILE["producer_path"](ROOT, root / "docs/tasks"))
        inputs = {*CORE, root / MAP, root / "ROADMAP.md", root / "docs/book/book.toml", *source_files,
                  *(root / "docs/tasks").glob("*.md"), root / "crates/sc-core/tests/formula_statement_contract.rs",
                  *(root / row[2] for row in current_rows),
                  *(root / "crates/sc-core/src/recipe" / file for file in FORMULA_METHODS)}
        PROFILE["plan_producer"](ROOT, sources=tuple(sorted(inputs)))
    except (PROFILE["Refusal"], OSError, UnicodeError, csv.Error) as error:
        raise PublicationError("BOOK_AUTHORITY: " + str(error)) from None


def main():
    main_inputs(ROOT)
    diagnostic = build(ROOT)
    if diagnostic:
        print(diagnostic, end="" if diagnostic.endswith("\n") else "\n")
    result = check(ROOT)
    print(f"publication: {result[0]} chapters / {result[1]} scoped API rows / "
          f"{result[2]} source links / {result[3]} rendered links; all checked")
    cases = {
        "unregistered chapter": "CHAPTER_COVERAGE",
        "missing indexed chapter": "INDEX_COVERAGE",
        "missing source target": "SOURCE_LINK",
        "missing source anchor": "SOURCE_ANCHOR",
        "missing rendered chapter": "RENDER_CHAPTER",
        "missing rendered anchor": "RENDER_ANCHOR",
        "stale G0 status": "TEXT_STATUS",
        "missing public MTM API": "API_STATUS",
    }
    for chapter, claims in FORMULA_SCOPE.items():
        for ordinal in range(len(claims)):
            cases[f"retired formula claim:{chapter}:{ordinal}"] = "FORMULA_SCOPE"
    for file, methods in FORMULA_METHODS.items():
        for method in methods:
            cases[f"missing formula method:{file}:{method}"] = "FORMULA_API"
    cases["missing formula proof boundary"] = "FORMULA_SCOPE"
    cases["stale statement contract count"] = "FORMULA_COUNT"
    cases["missing statement contract marker"] = "FORMULA_COUNT"
    scratch = ROOT / "target/scratch"
    for name, expected in cases.items():
        with temporary_root(prefix="book-publication-", dir=scratch, fixture_root=ROOT) as directory:
            root = Path(directory)
            fixture(ROOT, root)
            mutate(root, name)
            try:
                check(root)
            except PublicationError as error:
                require(str(error).startswith(expected + ":"), "WRONG_REFUSAL", f"{name}: {error}")
            else:
                raise PublicationError(f"MISSING_REFUSAL: {name}")
        print(f"  verified refusal: {name} ({expected})")
    with temporary_root(prefix="formula-currency-", dir=scratch, fixture_root=ROOT) as directory:
        root = Path(directory)
        fixture(ROOT, root)
        chapter = root / SOURCE / "annexes/formula-declarations.md"
        replace(chapter, "[whole recipe kind proofs]", "[complete ordered kind validation]")
        require(check(root) == result, "FORMULA_REWORDING", "link-label edit changed proof scope")
        mutate(root, "retired formula claim:declarations:0")
        currency_refusal(check, root)
        # Compile an actual instrument-body fault: omission must let the stale claim through.
        body = Path(__file__).read_text()
        anchor = "    formula_scope(root)\n"
        require(body.count(anchor) == 1, "BODY_ANCHOR", anchor)
        namespace = {"__file__": __file__, "__name__": "publication_currency_fault"}
        exec(compile(body.replace(anchor, "", 1), __file__, "exec"), namespace)
        try:
            currency_refusal(namespace["check"], root)
        except PublicationError as error:
            require(str(error).startswith("FORMULA_BODY_RED:"), "BODY_CONTROL", str(error))
        else:
            raise PublicationError("BODY_CONTROL: actual guard omission escaped no assertion")
    print("  formula currency: reworded positive / actual compiled guard-omission assertion red")
    # D175's independent stale sentence is deliberately outside the map-driven case population.
    # Removing its catalog entry must still fail this control, including when placed after the intro.
    with temporary_root(prefix="static-currency-", dir=scratch, fixture_root=ROOT) as directory:
        root = Path(directory)
        fixture(ROOT, root)
        chapter = root / SOURCE / "annexes/formula-static-validation.md"
        mutation_plan(chapter, activate=True)
        chapter.write_text(chapter.read_text() + "\nComplete checking: complete static graph remains .5b.4.\n")
        currency_refusal(check, root)
        body = Path(__file__).read_text()
        anchor = '"static-validation": (' + '"complete static graph remains .5b.4",),'
        require(body.count(anchor) == 1, "BODY_ANCHOR", "static scope catalog")
        namespace = {"__file__": __file__, "__name__": "publication_static_currency_fault"}
        exec(compile(body.replace(anchor, '"static-validation": (),', 1), __file__, "exec"), namespace)
        try:
            currency_refusal(namespace["check"], root)
        except PublicationError as error:
            require(str(error).startswith("FORMULA_BODY_RED:"), "BODY_CONTROL", str(error))
        else:
            raise PublicationError("BODY_CONTROL: actual static-scope omission escaped no assertion")
    print("  static currency: independent later-section claim / actual compiled catalog-omission assertion red")
    with temporary_root(prefix="book-warning-", dir=scratch, fixture_root=ROOT) as directory:
        root = Path(directory)
        fixture(ROOT, root)
        chapter = root / SOURCE / "introduction.md"
        marker = "\nPublication control: Checked<UnclosedType>\n"
        mutation_plan(chapter, activate=True)
        chapter.write_text(chapter.read_text() + marker)
        try:
            build(root)
        except PublicationError as error:
            require(str(error).startswith("BOOK_BUILD_WARNING:") and "unclosed" in str(error),
                    "WRONG_REFUSAL", str(error))
        else:
            raise PublicationError("MISSING_REFUSAL: actual malformed generic builder warning")
        replace(chapter, marker, "\nPublication control: `Checked<UnclosedType>`\n")
        build(root)
        require(check(root) == result, "WARNING_REPAIR", "quoted generic changed publication scope")
        rendered = (root / HTML / "introduction.html").read_text()
        require("<code>Checked&lt;UnclosedType&gt;</code>" in rendered,
                "WARNING_REPAIR", "generic parameter missing from repaired rendering")
    print("  verified refusal: actual malformed generic (BOOK_BUILD_WARNING); repaired rendering pass")
    print(f"publication probes: {len(cases) + 5} pass / 0 fail")


if __name__ == "__main__":
    main()
