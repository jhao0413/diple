# Diple standalone product: Plan

Delivers the sibling `spec.md`.

Status: Complete
Verified: 2026-09-09

## Goal

Cut one final baseline from upstream, retain the fork's staged/unstaged review
workflow, and ship a host-independent `diple` binary whose review handoff is
plain stdout.

## Ticket map

1. **Port staged/unstaged review.** Bring the fork's index-side model and UI to
   the current codebase, revise the write invariant, and retain continuity.
2. **Replace the host boundary.** Remove last-turn and herdr runtime calls, move
   config to Diple's directory, and export confirmed comments to stdout.
3. **Rebrand and package.** Rename the crate/binary/assets, update release CI and
   documentation, preserve attribution, and remove plugin packaging.

## 1. Port staged/unstaged review

### Acceptance criteria

- [x] The uncommitted file navigator has independent staged and working groups.
- [x] A both-sides path occurs once in each group and opens that side's diff.
- [x] Per-row and header statistics describe the visible side.
- [x] The stage key stages only a working file and unstages only a staged file.
- [x] Refresh and stage transitions preserve section/path identity.
- [x] Branch and commits scopes remain ungrouped and read-only.

### Verification

- Focused git parser and repository tests.
- App-flow and render tests for groups, side diffs, staging, and continuity.

## 2. Replace the host boundary

### Acceptance criteria

- [x] No production module shells out to `herdr`.
- [x] Last-turn is absent from the model, keymap, config, worker, UI, and docs.
- [x] Config resolves through the platform config directory as
  `diple/config.toml`.
- [x] Send confirms, exits, restores terminal modes, and writes comments to
  stdout with one trailing newline.
- [x] Captured stdout stays free of TUI bytes by routing the backend to
  `/dev/tty` on Unix.
- [x] Plain quit writes nothing.

### Verification

- Config unit tests with an isolated config root.
- PTY test that separately captures terminal bytes and stdout payload.
- App-flow tests for Send confirmation and cancellation.

## 3. Rebrand and package

### Acceptance criteria

- [x] Package, crate imports, binary, release assets, cache/log paths, UI, and
  current documentation use Diple.
- [x] Cargo metadata points at `jhao0413/diple` and credits both authors.
- [x] The original MIT copyright remains and a derivative notice is present.
- [x] Plugin manifest and herdr installation/actions are not in the product.
- [x] Release CI builds Diple binaries for supported macOS and Linux targets.
- [x] README leads with standalone installation and the agent handoff workflow.

### Verification

- Repository-wide search for inherited product/host identifiers, with only
  attribution and historical migration notes allowed.
- Format, clippy, tests, and release build.

## Replan

- If the staged feature cannot be replayed cleanly on the current upstream,
  port its behavior and tests by invariant instead of preserving patch shape.
- If `/dev/tty` is unavailable while stdout is captured, return a clear startup
  error rather than corrupting stdout with terminal control bytes.

## Verification record

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`: 724 passed, 1 live-network test ignored
- `cargo build --release`
- `cargo package --allow-dirty --no-verify`: 78 files, 455.6 KiB compressed
- `git diff --check`
- Repository scan: inherited names remain only in attribution, migration history,
  and this implementation spec
