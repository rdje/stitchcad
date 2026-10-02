"""Check publication topology, local navigation and a scoped current-library status map.

This checks references/status markers, not correctness of all prose or sewing semantics. The public
API map is deliberately scoped; contract tests and later gates retain execution/certification proof.
Eight copied-fixture mutations prove the named refusals. No tracked source or other Git repo is edited.
"""
from __future__ import annotations

import csv
from html.parser import HTMLParser
from pathlib import Path
import re
import shutil
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


def markdown_links(text):
    # The book uses inline local Markdown links; external links and non-chapter assets are excluded.
    return re.findall(r"\]\(([^\s)]+)", text)


def chapters(text):
    return [href.split("#", 1)[0] for href in markdown_links(text)
            if href.split("#", 1)[0].endswith(".md")]


def rows(root):
    with (root / MAP).open(newline="") as stream:
        return list(csv.reader((line for line in stream if not line.startswith("#")), delimiter="\t"))


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
    return len(registered), len(current_rows), source_links, rendered_links


def fixture(root, destination):
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
    for file in [Path("ROADMAP.md"), MAP] + [Path(row[2]) for row in rows(root)]:
        target = destination / file
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(root / file, target)


def replace(file, old, new):
    text = file.read_text()
    require(text.count(old) == 1, "FIXTURE_ANCHOR", old)
    file.write_text(text.replace(old, new, 1))


def mutate(root, name):
    source = root / SOURCE
    if name == "unregistered chapter":
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
    print(f"publication probes: {len(cases) + 1} pass / 0 fail")


if __name__ == "__main__":
    main()
