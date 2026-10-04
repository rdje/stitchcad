"""Check publication topology, local navigation and a scoped current-library status map.

This checks references/status markers, not correctness of all prose or sewing semantics. The public
API map is deliberately scoped; contract tests and later gates retain execution/certification proof.
Copied-fixture mutations prove the named refusals. No tracked source or other Git repo is edited.
"""
from __future__ import annotations

import csv
from html.parser import HTMLParser
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
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
    "static-validation": (),
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


def build(root):
    """Capture the actual builder; source/link topology cannot certify warning-bearing output."""
    environment = dict(os.environ, MDBOOK_LOG="warn", RUST_LOG="warn")
    result = subprocess.run(["mdbook", "build", "docs/book"], cwd=root, env=environment,
                            capture_output=True, text=True)
    diagnostic = result.stdout + result.stderr
    require(result.returncode == 0, "BOOK_BUILD", diagnostic.strip())
    plain = re.sub(r"\x1b\[[0-9;]*m", "", diagnostic)
    warnings = re.findall(r"(?m)^\s*WARN\b.*$", plain)
    require(not warnings, "BOOK_BUILD_WARNING", "\n".join(warnings))
    return diagnostic


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
    (destination / "docs/book").mkdir(parents=True)
    shutil.copy2(root / "docs/book/book.toml", destination / "docs/book/book.toml")
    for directory in (SOURCE, Path("docs/tasks")):
        # Artifacts are copied separately; evidence/tree Markdown is sufficient for ownership lookup.
        if directory == Path("docs/tasks"):
            target = destination / directory
            target.mkdir(parents=True)
            for file in (root / directory).glob("*.md"):
                shutil.copy2(file, target / file.name)
        else:
            shutil.copytree(root / directory, destination / directory)
    (destination / HTML).mkdir(parents=True)
    for file in (root / HTML).rglob("*.html"):
        target = destination / HTML / file.relative_to(root / HTML)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(file, target)
    for file in [Path("ROADMAP.md"), MAP, Path("crates/sc-core/tests/formula_statement_contract.rs")] + [Path(row[2]) for row in rows(root)]:
        target = destination / file
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(root / file, target)


def replace(file, old, new):
    text = file.read_text()
    require(text.count(old) == 1, "FIXTURE_ANCHOR", old)
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
    source = root / SOURCE
    if name.startswith("retired formula claim:"):
        chapter, ordinal = name.removeprefix("retired formula claim:").split(":")
        path = source / "annexes" / f"formula-{chapter}.md"
        text = path.read_text()
        intro, rest = text.split("\n## ", 1)
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
        path.write_text(intro.replace("formula-checked-recipes.md", "formula-recipe-inputs.md") + "\n## " + rest)
    elif name == "stale statement contract count":
        replace(source / "annexes/formula-statements.md", "10 public contracts verify", "9 public contracts verify")
    elif name == "missing statement contract marker":
        path = root / "crates/sc-core/tests/formula_statement_contract.rs"
        text = path.read_text()
        require("#[test]\n" in text, "FIXTURE_ANCHOR", name)
        path.write_text(text.replace("#[test]\n", "// removed test marker\n", 1))
    elif name == "unregistered chapter":
        (source / "unregistered.md").write_text("# Stranded reader chapter\n")
    elif name == "missing indexed chapter":
        replace(source / "topic-index.md", "- [From an idea to a pattern](learn/design-to-pattern.md)\n", "")
    elif name == "missing source target":
        with (source / "introduction.md").open("a") as stream:
            stream.write("\n[Missing](missing-chapter.md)\n")
    elif name == "missing source anchor":
        with (source / "introduction.md").open("a") as stream:
            stream.write("\n[Missing](availability.md#absent-section)\n")
    elif name == "missing rendered chapter":
        (root / HTML / "learn/design-to-pattern.html").unlink()
    elif name == "missing rendered anchor":
        with (root / HTML / "introduction.html").open("a") as stream:
            stream.write('<a href="availability.html#absent-section">Missing</a>')
    elif name == "stale G0 status":
        replace(source / "introduction.md", "G1 executable foundations are in progress.", "The project is in gate G0.")
    elif name == "missing public MTM API":
        replace(root / "crates/sc-measure/src/mtm_chart.rs", "pub struct MtmChart {", "pub struct RetiredChart {")
    else:
        raise PublicationError(f"unknown fixture mutation: {name}")


def main():
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
    scratch.mkdir(parents=True, exist_ok=True)
    for name, expected in cases.items():
        with tempfile.TemporaryDirectory(prefix="book-publication-", dir=scratch) as directory:
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
    with tempfile.TemporaryDirectory(prefix="formula-currency-", dir=scratch) as directory:
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
    with tempfile.TemporaryDirectory(prefix="book-warning-", dir=scratch) as directory:
        root = Path(directory)
        fixture(ROOT, root)
        chapter = root / SOURCE / "introduction.md"
        marker = "\nPublication control: Checked<UnclosedType>\n"
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
    print(f"publication probes: {len(cases) + 4} pass / 0 fail")


if __name__ == "__main__":
    main()
