# Diple standalone product

Status: Implemented
Date: 2026-09-09

## Problem

The application is useful outside herdr, but its public identity, configuration,
send path, one diff scope, and release packaging still belong to a herdr plugin.
The current fork also adds explicit index staging while the inherited product
contract says the index is never changed.

Diple is the independent product. It reviews Git changes in any terminal and
hands authored feedback to any coding agent through the shell. No multiplexer,
agent host, or plugin manager is required.

## Product contract

Diple is a terminal code-review workbench:

- `diple [repo]` opens the repository at `repo`, or the current directory.
- Changes, files, search, comments, markdown preview, editor opening, and the
  read-only forge view work without herdr.
- Uncommitted changes are split into `Staged Changes` and `Changes`. A path
  changed on both sides appears in both sections with the diff for that side.
- An explicit stage action stages a working-side file or unstages a staged-side
  file. No refresh, poll, startup, or exit path changes the index.
- Branch and commits remain review scopes. Last turn is removed because a
  portable git application has no agent-turn authority.
- `s` asks for confirmation, restores the terminal, exits, and writes the full
  comment batch to stdout. It does not submit a prompt.
- When stdout is captured, the TUI writes to `/dev/tty` on Unix so stdout
  contains only the exported comment batch. When stdout is a terminal, it is
  also the TUI writer and receives the batch only after terminal restoration.
- `y` copies the same batch without exiting. A failed copy or stdout write keeps
  the comments available whenever the process can continue.
- Configuration lives at the platform config directory under
  `diple/config.toml` (normally `~/.config/diple/config.toml`). No herdr lookup
  participates in configuration.
- The package, library crate, executable, logs, cache directory, release assets,
  documentation, and UI call the product `diple` or `Diple`.

## Safety

Diple never edits worktree files, creates commits, or moves branches.

It performs only these git writes:

1. A user-commanded stage or unstage changes the index for the selected path.
2. A base pick may be stored in Diple's private per-worktree ref namespace.

Both writes happen only after an explicit keypress and surface failure without
pretending that the requested state was applied.

## Agent handoff

The exported text keeps the existing stable format: location, anchored diff
snippet, and comment text, with one blank line between comments. Diple adds no
terminal escapes, status text, prose preamble, or host-specific envelope to
stdout.

Typical agent use is a shell escape or tool call that captures `diple` stdout.
The protocol is deliberately plain text. Agent-specific adapters may wrap it,
but no adapter is part of the core runtime.

## Attribution

Diple remains an MIT-licensed derivative of herdr-reviewr. The original license
and copyright notice stay intact. Diple adds its own attribution without
rewriting authorship or discarding commit history.

## Invariants

| code | Always true |
| --- | --- |
| DPL-NO-HERDR | Starting, browsing, configuring, exporting, and exiting Diple invokes no `herdr` command. |
| DPL-STDOUT-CLEAN | A captured stdout contains only the confirmed export batch and one trailing newline. |
| DPL-SEND-EXPLICIT | Quit never exports; only a confirmed Send exports and exits. |
| DPL-STAGE-EXPLICIT | Only the stage action changes the index, and only for the selected file row. |
| DPL-SIDE-DIFF | Each uncommitted section shows the diff and line counts for its own side of the index. |
| DPL-CONTINUITY | Refresh keeps the selected section/path when that identity survives. |
| DPL-NO-LAST-TURN | No key, config value, header, empty state, worker, or documentation exposes a last-turn scope. |
| DPL-IDENTITY | User-visible and distribution-owned names use Diple, never herdr-reviewr or reviewr. |

## Out of scope

- Posting reviews, resolving comments, merging, or rerunning checks on a forge.
- Automatically submitting the exported text to an agent.
- A herdr plugin bundled in the core repository.
- AI-generated explanations, commits, or review findings.
- Windows support in the first standalone release.
- Detaching or renaming the GitHub repository; that is a separate owner action
  after the migration branch is ready.
