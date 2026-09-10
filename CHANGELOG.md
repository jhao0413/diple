# Changelog

All notable changes to Diple are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-09-10

### Added

- Read-only History workspace on `3`, with a bounded all-ref commit graph, decorations, authors,
  relative dates, keyboard navigation, mouse navigation, and refresh-in-place.

### Changed

- Refined the interface with neutral pane borders, background-free header and footer chrome, and
  clearer file-tree hierarchy.
- Replaced the muddy neutral active-row fill with a clean, theme-aware cool-blue highlight while
  keeping text selection visually distinct.
- Compact tab labels automatically on narrow terminals so scope and commit identity remain visible.

### Removed

- Pull-request and merge-request workspace from the public tab bar, keymap, and footer hints.

## [0.2.1] - 2026-09-09

### Added

- Responsive old/new source columns for wide Changes diffs, with unified rendering retained on
  narrower panes.

### Changed

- The file navigator now defaults to the left of the read pane.

## [0.2.0] - 2026-09-09

### Added

- Native x86-64 Windows support with continuous testing on a Windows runner.
- Portable Windows release ZIP for installation through WinGet without Rust or Cargo.
- Unicode clipboard export and default-browser opening on Windows.

### Changed

- Executable lookup now respects the host PATH separator and Windows `PATHEXT`.

## [0.1.0] - 2026-09-09

### Added

- Independent Diple product, crate, binary, configuration, documentation, and release identity.
- Separate `Staged Changes` and `Changes` sections for the uncommitted scope.
- `a` stages or unstages the selected path while preserving the worktree.
- Explicit send confirmation followed by a clean stdout review payload.
- TUI output through the controlling terminal when stdout is captured.
- Non-interactive `--help` and `--version` commands for package-manager verification.
- Homebrew installation through `jhao0413/tap/diple`.

### Removed

- Herdr runtime integration, plugin packaging, agent picker, and last-turn scope.

## Project history

Diple began from herdr-reviewr 0.36.2. Earlier release history remains available in Git and in
the upstream project. See [NOTICE](NOTICE).
