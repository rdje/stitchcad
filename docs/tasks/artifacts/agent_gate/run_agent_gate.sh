#!/usr/bin/env bash
# docs/tasks/artifacts/agent_gate/run_agent_gate.sh
# G2-2D.13 — one agent session for the agent gate: a pinned model, given only the MCP server and the book, drafts the reference skirt.
#
# WHY: G2's exit asks whether an LLM agent given only MCP and the docs produces a semantically equal instance, in two independent pinned-model sessions. The answer must come from
# something the agent could not touch, so this keeps the session's whole record, and `cargo test -p sc-mcp --test agent_gate` judges it: it replays what the agent did with the command-line
# path and reads the result against the fixture's declaration. An agent's own report that it succeeded counts for nothing.
#
# WHAT THE AGENT HAS: no built-in tool, no settings, no instruction file, no other MCP server, and a working directory with nothing in it. What it was really given is read from
# the session's own first record, because a model asked what tools it has will sometimes guess.
#
# COST: this spends real money on the director's account, capped by BUDGET per session. It is not part of any automatic check, and a failed session blocks nothing: the
# command-line path (`G2-01`) is the gate that must hold, and LLM nondeterminism never blocks it.
#
# Usage:  bash docs/tasks/artifacts/agent_gate/run_agent_gate.sh RUN MODEL
# Output: conformance/agent-gate/RUN/, holding the prompt, the session's record, the agent's recording and what it wrote. Exit 0 when the session ended and left a recording.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
RUN="${1:?a run name}"
MODEL="${2:?a model id}"
BUDGET="${3:-6}"
STORE="$ROOT/target/agent_gate/$RUN"
RECORD="$ROOT/conformance/agent-gate/$RUN"
python3 -I -B scripts/local_environment.py -- cargo build -q -p sc-mcp -p sc-cli || { echo "agent gate: it does not build" >&2; exit 1; }
rm -rf "$STORE" "$RECORD"
mkdir -p "$STORE/work" "$STORE/project" "$RECORD"
cat > "$RECORD/prompt.txt" <<'PROMPT'
Using only the tools of the stitchcad server, and the book those tools let you read, draft the reference skirt exactly as the book's chapter on it specifies, in size M.

Start a new design. When every piece is drawn, export it in canonical form to exports/agent.json, and save the project. Then say what you did, and anything in the book you found ambiguous or had to decide for yourself.
PROMPT
python3 -I -B - "$ROOT" "$STORE" > "$STORE/mcp.json" <<'PY'
import json
import sys

root, store = sys.argv[1:3]
print(json.dumps({"mcpServers": {"stitchcad": {"command": root + "/target/debug/stitchcad-mcp", "args": [
    "--project", store + "/project", "--book", root + "/docs/book/src", "--recording", store + "/recording.json"]}}}))
PY
cd "$STORE/work" || exit 1
env -u CLAUDECODE claude -p "$(cat "$RECORD/prompt.txt")" --model "$MODEL" --setting-sources "" --tools "" --mcp-config "$STORE/mcp.json" --strict-mcp-config \
  --allowedTools mcp__stitchcad --no-session-persistence --max-budget-usd "$BUDGET" --output-format stream-json --verbose > "$STORE/stream.jsonl" 2> "$STORE/stderr.log"
STATUS=$?
cd "$ROOT" || exit 1
exec python3 -I -B - "$STORE" "$RECORD" "$MODEL" "$STATUS" <<'PY'
import hashlib
import json
import pathlib
import shutil
import sys

store, record = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
model, status = sys.argv[3], int(sys.argv[4])
LIMIT = 600
facts = {"asked_for_model": model, "exit_status": status}
lines = []


def shorten(text):
    if len(text) <= LIMIT:
        return text
    return "%s ...[%d characters, sha256 %s]" % (text[:LIMIT], len(text), hashlib.sha256(text.encode()).hexdigest()[:16])


for raw in (store / "stream.jsonl").read_text(encoding="utf-8").splitlines():
    item = json.loads(raw)
    if item.get("type") == "system" and item.get("subtype") == "init":
        facts.update(model=item.get("model"), tools=item.get("tools"), mcp_servers=item.get("mcp_servers"), claude_code=item.get("claude_code_version"))
    elif item.get("type") == "result":
        facts.update({key: item.get(key) for key in ("subtype", "is_error", "num_turns", "total_cost_usd", "duration_ms")})
        # The project's canonical text holds no fractions, so a cost is written as text.
        facts["total_cost_usd"] = str(facts.get("total_cost_usd"))
    elif item.get("type") in ("assistant", "user"):
        content = item["message"]["content"]
        for part in content if isinstance(content, list) else [{"type": "text", "text": content}]:
            if part.get("type") == "text":
                lines.append("%s: %s" % (item["type"], part["text"]))
            elif part.get("type") == "tool_use":
                lines.append("call %s %s" % (part["name"], shorten(json.dumps(part["input"], ensure_ascii=False))))
            elif part.get("type") == "tool_result":
                inner = part.get("content")
                text = inner if isinstance(inner, str) else " ".join(entry.get("text", "") for entry in inner or [])
                lines.append("%s %s" % ("REFUSED" if part.get("is_error") else "result", shorten(text)))
(record / "run.json").write_text(json.dumps(facts, indent=2, sort_keys=True) + "\n", encoding="utf-8")
(record / "transcript.txt").write_text("\n".join(lines) + "\n", encoding="utf-8")
print("agent gate: %s, %s turns, $%s, %s" % (facts.get("model"), facts.get("num_turns"), facts.get("total_cost_usd"), facts.get("subtype")))
print("agent gate: tools " + ", ".join(sorted(facts.get("tools") or []))[:300])
recording = store / "recording.json"
if not recording.is_file():
    print("agent gate: the session left no recording", file=sys.stderr)
    sys.exit(1)
shutil.copy(recording, record / "session.json")
written = store / "project" / "exports" / "agent.json"
if written.is_file():
    shutil.copy(written, record / "canonical.json")
else:
    print("agent gate: the session wrote no exports/agent.json")
PY
