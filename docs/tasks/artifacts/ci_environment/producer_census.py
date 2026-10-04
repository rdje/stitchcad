"""Read-only D156 source inventory; candidates are inspection work, never runtime proof."""
from pathlib import Path
import argparse
import ast
import csv
import hashlib
import json
import re
import sys

ROOT = Path(__file__).resolve().parents[4]
TREES = ('scripts', 'knowledge-map/scripts', '.githooks', 'docs/tasks/artifacts',
         '.github/workflows', 'crates')
FILES = ('Makefile', 'Cargo.toml', 'rust-toolchain.toml', 'docs/book/book.toml')
MANIFEST = Path(__file__).with_name('producer_entries.tsv')
KINDS = {'profile', 'native', 'output', 'reference', 'delegate', 'platform'}
EFFECTS = {
    'run', 'check_call', 'check_output', 'Popen', 'execv', 'execve', 'execvp', 'execvpe',
    'system', 'popen', 'spawn', 'exec', 'eval', 'compile', 'run_path', '__import__', 'import_module',
    'TemporaryDirectory', 'NamedTemporaryFile', 'mkdtemp', 'mkstemp', 'gettempdir',
    'mkdir', 'makedirs', 'write_text', 'write_bytes', 'open', 'write', 'writelines',
    'unlink', 'rmdir', 'rmtree', 'rename', 'replace', 'copy', 'copy2', 'copytree',
    'copyfile', 'move', 'symlink', 'symlink_to', 'touch', 'chmod', 'truncate', 'extractall',
}
HEREDOC = re.compile(r"(?<!<)<<(?!=|<)(-?)[ \t]*(?:'([^']+)'|\"([^\"]+)\"|([A-Za-z_][A-Za-z_0-9]*))")


class Refusal(ValueError):
    """Inventory cannot be complete within the declared source boundary."""


def require(condition, message):
    if not condition:
        raise Refusal(message)


def inventory(root):
    """No Git invocation, link traversal, external implementation or generated output scan."""
    device = root.stat().st_dev
    paths = []

    def parents(path):
        current = root
        for part in path.relative_to(root).parts[:-1]:
            current = current / part
            require(not current.is_symlink(), 'source symlink ancestor')
            require(current.is_dir() and current.stat().st_dev == device, 'invalid source ancestor')
            marker = current / '.git'
            require(not (marker.exists() or marker.is_symlink()), 'nested Git source boundary')

    def visit(path):
        require(not path.is_symlink(), 'source symlink: ' + path.relative_to(root).as_posix())
        require(path.stat().st_dev == device, 'source on another volume')
        if path.is_dir():
            marker = path / '.git'
            require(not (marker.exists() or marker.is_symlink()), 'nested Git source boundary')
            for child in sorted(path.iterdir()):
                visit(child)
        else:
            require(path.is_file(), 'source is not a regular file')
            relative = path.relative_to(root).as_posix()
            if (path.suffix in {'.py', '.sh', '.yml', '.yaml'} or
                    relative.startswith('.githooks/') or path.name in {'Cargo.toml', 'build.rs'}):
                paths.append(relative)

    for name in TREES:
        parents(root / name)
        visit(root / name)
    for name in FILES:
        path = root / name
        parents(path)
        require(not path.is_symlink() and path.is_file() and path.stat().st_dev == device,
                'invalid declared source file: ' + name)
        paths.append(name)
    require(len(paths) == len(set(paths)), 'duplicate source path')
    return sorted(paths)


def python_calls(text, filename, offset=0):
    """Every AST call is retained, including aliases and unresolved dynamic callees."""
    try:
        tree = ast.parse(text, filename=filename)
    except SyntaxError as error:
        raise Refusal('unparsed Python: ' + filename) from error
    result = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Call):
            callee = ast.get_source_segment(text, node.func)
            name = node.func.attr if isinstance(node.func, ast.Attribute) else (
                node.func.id if isinstance(node.func, ast.Name) else None)
            result.append({'line': offset + node.lineno, 'column': node.col_offset,
                           'callee': callee, 'candidate': name in EFFECTS or name is None,
                           'source': ast.get_source_segment(text, node)})
    return sorted(result, key=lambda row: (row['line'], row['column']))


def shell_sections(text, filename):
    """Retain the entire shell text and literal heredoc boundaries; do not claim a shell parser.

    PY-labelled/explicit Python heredocs get an AST inventory. Other heredocs are kept verbatim
    for review (including generated fixture programs). A body is never scanned as shell commands.
    """
    lines = text.splitlines(keepends=True)
    result, index = [], 0
    while index < len(lines):
        opener = lines[index]
        matches = [] if opener.lstrip().startswith('#') else list(HEREDOC.finditer(opener))
        index += 1
        for match in matches:
            delimiter = next(value for value in match.groups()[1:] if value is not None)
            first, body = index, []
            while index < len(lines):
                closing = lines[index].rstrip('\r\n')
                if match[1]:
                    closing = closing.lstrip('\t')
                if closing == delimiter:
                    break
                body.append(lines[index])
                index += 1
            require(index < len(lines), 'unclosed literal heredoc: ' + filename)
            source = ''.join(body)
            python = delimiter in {'PY', 'PYTHON'} or bool(re.search(r'\bpython3?\b', opener))
            result.append({'opener_line': first, 'first_line': first + 1,
                           'last_line': index, 'delimiter': delimiter, 'source': source,
                           'python_calls': python_calls(source, filename, first) if python else None})
            index += 1
    return result


def entries(path):
    with path.open(newline='', encoding='utf-8') as stream:
        rows = list(csv.DictReader(stream, delimiter='\t'))
    require(rows and set(rows[0]) == {'path', 'kind', 'owner', 'review'}, 'invalid inventory columns')
    require(all(row['kind'] in KINDS and row['owner'] and row['review'] for row in rows),
            'unclassified source entry')
    require(len(rows) == len({row['path'] for row in rows}), 'duplicate inventory entry')
    return {row['path']: row for row in rows}


def census(root, manifest):
    paths, dispositions = inventory(root), entries(manifest)
    require(set(paths) == set(dispositions), 'source/entry inventory differs: ' +
            ', '.join(sorted(set(paths) ^ set(dispositions))))
    result = []
    for relative in paths:
        data = (root / relative).read_bytes()
        text = data.decode('utf-8')
        row = dict(dispositions[relative], sha256=hashlib.sha256(data).hexdigest(),
                   bytes=len(data), lines=len(text.splitlines()), source=text)
        if relative.endswith('.py'):
            row['calls'] = python_calls(text, relative)
        else:
            # Entire non-Python text is retained. YAML/Make/TOML and Rust build scripts require
            # manual public-contract review; regex matches are never reported as command counts.
            if relative.endswith('.sh') or relative.startswith('.githooks/'):
                row['heredocs'] = shell_sections(text, relative)
        result.append(row)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--json', action='store_true', help='full source/call inventory to stdout')
    args = parser.parse_args()
    rows = census(ROOT, MANIFEST)
    if args.json:
        print(json.dumps(rows, ensure_ascii=False, indent=2))
    else:
        calls = sum(len(row.get('calls', [])) for row in rows)
        bodies = [body for row in rows for body in row.get('heredocs', [])]
        embedded = sum(len(body['python_calls'] or []) for body in bodies)
        print('producer-census: ' + str(len(rows)) + ' source entries; ' + str(calls) +
              ' Python AST calls; ' + str(len(bodies)) + ' literal heredocs; ' + str(embedded) +
              ' embedded Python calls; inspection inventory only, no locality signoff')


if __name__ == '__main__':
    try:
        main()
    except (Refusal, OSError, UnicodeError) as error:
        print('producer-census: REFUSED — ' + str(error), file=sys.stderr)
        sys.exit(2)
