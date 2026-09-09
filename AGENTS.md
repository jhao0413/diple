# Diple contributor guide

## Product

Diple is a standalone Rust/ratatui code-review TUI. It reviews uncommitted changes, branches,
and commit ranges; lets reviewers write line comments; and emits a confirmed review to stdout.
It has no host-agent or editor-extension dependency.

## Commands

- `just ci` — formatting, clippy, tests, and release build.
- `just run` — run Diple in the current repository.
- `cargo test <name>` — focused test while iterating.
- `python3 scripts/bench_tui.py --binary target/release/diple --fixture` — latency benchmark.

## Workflow

Behavior changes are spec-first. Add `docs/specs/YYYY-MM-DD-slug/spec.md` and `plan.md`, define
observable invariants, then implement and record verification in the plan.

Use `cargo fmt`; keep clippy warning-free. Tests should assert behavior and safety boundaries,
not implementation trivia.

## Architecture

- `src/app.rs` — session state and terminal-free interaction logic.
- `src/world.rs` — git-derived background snapshots and reconciliation inputs.
- `src/git.rs` — git queries plus the two explicit index actions, stage and unstage.
- `src/diff.rs` / `src/file_list.rs` — diff and navigator models.
- `src/ui.rs` — ratatui rendering and hit testing.
- `src/lib.rs` — terminal lifecycle, workers, input dispatch, and stdout delivery.
- `src/export.rs` — deterministic comment formatting and clipboard export.
- `src/config.rs` — CLI flags and `diple/config.toml` validation.
- `src/forge.rs`, `src/gitlab.rs`, `src/azure_devops.rs` — optional read-only forge view.

## Invariants

- Normal browsing never writes the worktree, commits, branches, or remotes.
- Only the explicit `toggle-stage` action mutates the index.
- The saved base choice writes only `refs/worktree/diple/base-pick`.
- An uncommitted staged row displays `HEAD → index`; a working row displays `index → worktree`.
- A path present in both sections is identified by path plus section everywhere.
- Refresh preserves comments, drafts, selection, open-file identity, and scroll where possible.
- Sending requires confirmation, restores the terminal first, emits only the formatted review to
  stdout, and consumes comments only at that confirmed boundary.
- `q` and error exits produce no stdout payload.
- Diple never invokes `herdr` and never depends on host-specific environment variables.

## Attribution

Diple is derived from herdr-reviewr. Preserve `LICENSE`, `NOTICE`, and relevant Git history.
