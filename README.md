# Diple

**A focused terminal workspace for reviewing code before it leaves your machine.**

The name comes from the ancient *diple* (`>`), a margin mark used to call attention to a
noteworthy passage. Diple brings that idea to source code: inspect a diff, mark exact lines,
write a review, and hand the result to whatever tool or agent you choose.

Diple is a standalone Rust TUI. It does not require an editor extension, an agent host, or a
particular AI workflow.

## What it does

- Reviews uncommitted changes as separate **Staged Changes** and **Changes** sections.
- Stages or unstages the selected file with `a` without modifying the worktree.
- Reviews a branch against an automatically resolved or manually selected base.
- Reviews one commit or a contiguous run of commits.
- Browses a compact, all-branch commit graph with decorations, authors, and relative dates.
- Adapts wide Changes panes into old/new source columns and keeps narrow panes unified.
- Adds comments to exact old-side or new-side line ranges.
- Browses and searches the whole repository, with syntax highlighting and Markdown preview.
- Copies a review to the clipboard, or confirms and writes it to stdout for another command.

## Install

With Homebrew on macOS or Linux:

```sh
brew install jhao0413/tap/diple
```

With WinGet on Windows (available after the initial community manifest is indexed):

```powershell
winget install --id Jhao0413.Diple -e
```

With Cargo:

```sh
cargo install diple
```

Prebuilt binaries and SHA-256 checksums are also available from the
[latest GitHub release](https://github.com/jhao0413/diple/releases/latest) for Apple Silicon,
Intel macOS, x86-64 Linux, ARM64 Linux, and x86-64 Windows.

## Use

Open the repository containing the current directory:

```sh
diple
```

Or name one explicitly:

```sh
diple ~/src/my-project
```

Write comments in the TUI and press `s`. Diple asks for confirmation, restores the terminal,
then writes the formatted review to stdout and exits. This makes it composable without coupling
the UI to a specific recipient:

```sh
review="$(diple ~/src/my-project)"
printf '%s\n' "$review"
```

When stdout is captured, Diple draws the TUI through the controlling terminal (`/dev/tty` on
Unix or the console on Windows) so terminal escape sequences never enter the captured review.

Press `q` to quit without emitting anything. Press `y` to copy comments without exiting.

## Essential keys

| Key | Action |
| --- | --- |
| `j` / `k`, `↑` / `↓` | Move |
| `tab` | Switch between navigator and reader |
| `1` / `2` / `3` | Changes / all files / history graph |
| `u` / `b` / `g` | Uncommitted / branch / commits scope |
| `B` / `G` | Pick a branch base / commit range |
| `a` | Stage or unstage the selected file |
| `v` | Start or finish a line selection |
| `c` | Comment on the selected lines |
| `l` | Open the comments list |
| `s` | Confirm and emit the review to stdout |
| `y` | Copy the review to the clipboard |
| `/` / `ctrl+f` | Repository search / find in file |
| `m` | Toggle Markdown preview |
| `e` | Open the selected location in an editor |
| `?` | Show the complete contextual key guide |
| `q` | Quit without output |

Keys can be rebound in the configuration file.

## Scopes

`uncommitted` compares the repository in two steps:

- **Staged Changes:** `HEAD` → index
- **Changes:** index → worktree, including untracked files

A path edited on both sides appears once in each section, and each row opens only that side's
diff. `a` moves the selected path across the index boundary.

`branch` compares the worktree with the merge base of a selected base branch. Diple tries the
explicit `--base`, a saved per-worktree choice, the configured upstream, `origin/HEAD`, and common
default branch names.

`commits` lets you pick one commit or a contiguous range and compares the first commit's parent
with the newest selected commit.

## Review output

Comments are sorted by path and line. Each block contains the location, the selected diff lines,
and the comment text:

```text
src/parser.rs:40-42
-    parse_legacy(input)
+    parse(input)?
Should this preserve the legacy fallback for old files?
```

Blocks are separated by one blank line. Output is produced only after the send confirmation;
ordinary quit, errors, logs, and terminal rendering do not write to stdout.

## Configuration

Diple reads `config.toml` from the platform configuration directory under `diple/`. Set
`DIPLE_CONFIG_DIR` to use a specific directory. For example:

```toml
theme = "catppuccin-mocha"
default_scope = "uncommitted"
navigator_position = "left"
editor = "code -g {file}:{line}"

[keybindings]
toggle-stage = ["a"]
send = ["s"]
```

CLI flags override their corresponding settings:

```text
diple [--poll MILLISECONDS] [--base REF] [--theme NAME] [--wrap on|off] [REPO]
```

Set `DIPLE_LOG=/path/to/log` for an optional diagnostic event log.

## Safety

Browsing, searching, commenting, and exporting are read-only. Diple writes to Git only for two
explicit actions:

- `a` updates the selected path in the index (`git add` or `git restore --staged`).
- choosing a branch base records that spelling under `refs/worktree/diple/base-pick`.

Diple never commits, pushes, resets, checks out a branch, or modifies worktree file content.

## Attribution

Diple began as an independent continuation of
[herdr-reviewr](https://github.com/persiyanov/herdr-reviewr). The original MIT license and
copyright notice are retained. See [NOTICE](NOTICE) for details.

## License

MIT
