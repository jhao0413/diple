# History graph

## Goal

Diple's third workspace shows repository history as a compact, read-only commit graph instead of
the removed pull-request view.

## User contract

- The header exposes `3 History` after `1 Changes` and `2 Files`; the label and its configured key
  are clickable and keyboard-accessible like the other tabs.
- History lists the most recent 200 commits reachable from all local and remote refs, in git's
  date-ordered graph topology.
- Commit rows show the graph lanes, abbreviated object id, decorations, subject, author, and age.
  Topology-only connector rows stay visible so branches and merges remain legible.
- `j`/`k`, the arrow keys, page keys, and half-page keys move between commit rows while skipping
  connector-only rows. The selected commit uses the focused-row highlight.
- Mouse click selects a commit row; the wheel scrolls through the history without changing the
  selected commit.
- `r` reloads the graph while preserving the selected commit by object id when it still exists.
- An unborn repository shows `no commits yet` without failing.
- History is read-only: it does not offer staging, editing, commenting, sending, scope, file-pane,
  or pull-request actions.
- No PR tab or PR keybinding is restored.

## Non-goals

- Opening a commit diff from the history row.
- Searching or filtering history.
- Loading more than the bounded 200-commit window.
- Removing the isolated forge adapters and their internal test coverage.
