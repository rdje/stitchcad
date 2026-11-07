# The API and MCP server

An agent can now do what the command line and [the dev shell](dev-shell.md) do, through tools it can discover: read this book, start a design, evaluate it, preview a change, draw it, export it. `sc-api` holds those workflows with no transport in them, and `sc-mcp` offers them over MCP. Both go through the same session steps on the same command bus, so everything an agent does can be replayed by the command-line program.

```text
$ stitchcad-mcp --project DIR --book docs/book/src
```

**What works today.** The `local-stdio` profile, where a host launches this program and speaks over its pipes: 22 tools, four authorities a caller can hold, an actor trace, and its admission tests. **What does not exist yet:** the `local-http`, `remote-http` and `private-bridge` profiles, each of which waits for its own threat model; resources and prompts, since tools alone cover this; and a release to approve.

**The roadmap names `jsonrpsee` and `rmcp` for this.** Neither is used. The stdio transport is one JSON message to a line, which needs no library, and a network profile is where they would earn their place. This contract is experimental, and its semver commitment comes at G7.

## The tools, and what each needs

| Tool | Needs | What it does |
| --- | --- | --- |
| `server_info` | `inspect` | Who you are acting as, and what you may do |
| `list_book` | `inspect` | The chapters of this book |
| `read_book` | `inspect` | One chapter |
| `start_design` | `commit` | A new design from named lengths and a recipe |
| `open_project` | `inspect` | Loads the design saved in this project |
| `evaluate` | `inspect` | Every binding, by name, kind and value |
| `preview_change` | `propose` | What a change would change, changing nothing |
| `change_measurement` | `commit` | Changes one input through the bus |
| `undo` | `commit` | Undoes the last change |
| `redo` | `commit` | Redoes the last change undone |
| `check_constraint` | `inspect` | Checks an input against a limit |
| `draft_pattern` | `generate` | Draws the pieces, as [the command line](command-line.md) describes them |
| `describe_pattern` | `inspect` | Each piece's name, extent, notches and labels |
| `describe_closures` | `inspect` | Each dart, tuck, pleat and gather, and what closing it removes |
| `walk_seams` | `inspect` | Each seam's sides, differential and what explains it |
| `plant_balance_notch` | `commit` | A notch on both sides of a seam, as one edit |
| `true_seam` | `commit` | Makes a seam's sides match, as one edit, within a stated limit |
| `export_pattern` | `generate` | Writes canonical text, DXF or PDF into the project |
| `save_project` | `commit` | One crash-consistent commit |
| `reopen_project` | `commit` | Returns to what was saved |
| `trace` | `inspect` | Every step that changed anything, and who took it |
| `approve_release` | `approve` | Says that approval is a person's |

```rust
use sc_api::Api;
use sc_cli::{Caller, CommandAuthority};
use sc_store::parse;

let project = std::env::temp_dir().join(format!("stitchcad-api-example-{}", std::process::id()));
std::fs::create_dir_all(&project).expect("creates");

// An agent that may look and propose, and nothing else.
let caller = Caller { agent: true, authorities: vec![CommandAuthority::Inspect, CommandAuthority::Propose] };
let mut api = Api::new(caller, &project, None).expect("a caller");

// It may not start a design, and the refusal says what it would need.
let design = parse(r#"{"inputs":[{"token":"width","um":200000}],"recipe":"let double: length = 2 * width\n"}"#).expect("valid");
let refused = api.call("start_design", &design).expect_err("needs commit");
assert_eq!(refused.code, "unauthorized");
assert!(refused.message.contains("commit"));

// Asking an agent to hold approval fails before anything starts.
let approving = Caller { agent: true, authorities: vec![CommandAuthority::Approve] };
assert_eq!(Api::new(approving, &project, None).expect_err("human only").code, "human_only");
std::fs::remove_dir_all(&project).expect("removes");
```

`render` and `export` are the two steps with no tool, because they draw the schematic that predates drafting. A test compares every tool's step with [the bus's own list](command-line.md), so a new step must either gain a tool or be named here.

## Authority

An agent holds what it was started with, and can do nothing more. The grant that holds this is built in `sc-core`, so `change_measurement` from a caller without `commit` is refused by the bus itself and not by a check in front of it. The tools with no bus command behind them are held to the same list by the session.

**Approval is a person's.** The core refuses to build a grant that gives an agent `approve`, so the server does not start with one, and `approve_release` tells an agent that it cannot. A mutation can never manufacture approval.

**Every mutation names who made it.** `trace` lists each step with the agent that took it and the person it acts for, and the bus records both on every command.

## What the local-stdio profile admits

| Roadmap §7.8 asks about | What holds | The test |
| --- | --- | --- |
| Caller identity | Whoever launched the process, with no socket. It is an agent unless started as a person | `an_agent_cannot_be_given_approval_at_all` |
| Unauthorized access | An `inspect` or `propose` caller cannot change anything, and the bus says so | `a_scoped_caller_cannot_exceed_its_authority` |
| Scope escape | Only the project and the book's directories. A chapter must be one the listing names, and an export path must be plainly inside the project | `nothing_outside_the_project_and_the_book_can_be_reached` |
| Human-only approval | Refused by name, and impossible to grant | `an_agent_cannot_be_given_approval_at_all` |
| Injected data | A label reading "IGNORE PREVIOUS INSTRUCTIONS" is a label, and nothing acts on it | `text_in_the_data_stays_data` |
| Artifact overwrites | An existing file is refused unless the caller says `replace` | `an_existing_artifact_is_not_overwritten_unless_asked` |
| Disconnect and retry | Killed mid-edit, only unsaved work is gone. A revision is not issued twice, and a stale edit is refused | `a_disconnect_loses_only_what_was_not_saved_and_a_retry_carries_on` |
| Command-state parity | What a caller did replays through the command line to the same bytes | `what_a_caller_did_replays_through_the_command_line_to_the_same_files` |

```text
$ cargo test -p sc-mcp --test admission
```

**What this does not show.** That a text-based instruction cannot influence an LLM: "stays data" holds for this server, which acts on nothing it reads, and what an agent does with a sentence it read is the agent's. Resource limits: a host can send as much as it likes. A file edited on disk outside a commit is not detected, as [the store](project-store.md) says.

## The agent gate

G2 asks whether an LLM agent given only MCP and the docs can produce a semantically equal instance of [the reference skirt](reference-skirt.md), in two independent sessions with pinned models. So a session was started with no built-in tool, no settings, no instruction file and an empty working directory, and given this server, this book, and one short request. What it was really given is read from the session's own first record, because a model asked what tools it has will sometimes guess. One did.

An agent's report that it succeeded counts for nothing. `cargo test -p sc-mcp --test agent_gate` judges what it left: the command-line path replays its recording to the bytes it was handed, [the judge](mutation-tests.md) reads the drawing against the fixture's declaration, and the result is compared with the frozen skirt within T2, 10 µm, with a label's place and punctuation left free and its words judged. That test needs no model, so a model's nondeterminism cannot block anything, and it runs anywhere.

| Session | Model | Calls | Cost | Refused calls | Verdict |
| --- | --- | --- | --- | --- | --- |
| The first reading, kept in `conformance/agent-gate-history/` | `claude-sonnet-5-5` | 16 | $0.62 | 1 | Failed: it drew every size right, and different darts and notch depths |
| `sonnet-1` | `claude-sonnet-5-5` | 15 | $0.49 | 0 | Passed |
| `haiku-1` | `claude-haiku-5-5` | 16 | $0.11 | 0 | Passed |

Those three are every session run. **The first failed, and that was the book's fault.** The chapter promises that a reader can draft the garment "without asking a question", and the reader had to decide nine things. Most importantly, it cut each dart out of the boundary, where this project's own session marks it with internal lines, and nothing in the book said which. It also pointed out that the book asks for a colorway on every label and this project's own session had none. So [the skirt's chapter](reference-skirt.md) now pins the drawing, with a test holding each position against it, and then the two sessions above ran on that book, with its small remaining gaps fixed between them.

**What this shows.** The tools can be discovered and used with nothing but what they say and what this book says. **What it does not show.** That an agent can design a garment: the chapter now pins every line, so this tests reading, tools and care. That agents would agree on anything the book leaves open. Two sessions are few, and each ran on a book I had just improved from what the first one found. That the skirt is right: all three inherit the same unreviewed constants, as the second session itself pointed out.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `sc-api` | The workflows, with no transport |
| `sc-mcp` | The MCP server |
| `sc-core` | The core model, where a grant is built |
| `stitchcad-mcp` | The program a host launches |
| `local-stdio` | A local host launches the server over process pipes |
| `local-http` | MCP over HTTP on a loopback listener, not built |
| `remote-http` | MCP over authenticated TLS, not built |
| `private-bridge` | An authenticated relay, not built |
| `jsonrpsee` | A JSON-RPC library the roadmap names |
| `rmcp` | An MCP library the roadmap names |
| `inspect` | May read and evaluate |
| `propose` | May preview a change |
| `commit` | May change the design and save it |
| `generate` | May draw and export |
| `approve` | A person's alone |
| `replace` | Says an export may write over an existing file |
| `render` | The step that draws the old schematic |
| `export` | The step that writes the old schematic |
| `a_disconnect_loses_only_what_was_not_saved_and_a_retry_carries_on` | A test in `sc-mcp` |
| `a_scoped_caller_cannot_exceed_its_authority` | A test in `sc-mcp` |
| `an_agent_cannot_be_given_approval_at_all` | A test in `sc-mcp` |
| `an_existing_artifact_is_not_overwritten_unless_asked` | A test in `sc-mcp` |
| `nothing_outside_the_project_and_the_book_can_be_reached` | A test in `sc-mcp` |
| `text_in_the_data_stays_data` | A test in `sc-mcp` |
| `what_a_caller_did_replays_through_the_command_line_to_the_same_files` | A test in `sc-mcp` |
| `sonnet-1` | The first passing session |
| `haiku-1` | The second passing session |
| `claude-sonnet-5-5` | A pinned model id |
| `claude-haiku-5-5` | A pinned model id |
