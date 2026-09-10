# Local workspace chrome and clean selection

## Goal

Diple removes the hosted PR workspace from its public chrome and uses a clean cool highlight
instead of a muddy neutral fill for the active row. The later
`2026-09-10-history-graph` change adds History as a third local workspace.

## User contract

- No PR tab is painted or clickable, and `tab-pr` is no longer a configurable action.
- The expanded `tabs` hint contains only visible local workspaces; the follow-up History contract
  owns the `3` binding.
- The focused cursor row uses a low-saturation blue fill derived from the active theme; the
  unfocused cursor row remains a quieter neutral surface.
- Text selection keeps its stronger blue fill and remains visually distinct inside a cursor row.
- Primary text keeps at least 4.5:1 contrast against both cursor and text-selection fills in every
  built-in light and dark theme.
- User documentation describes only local changes, repository files, branches, and commits.

## Non-goals

- Physically deleting the currently isolated forge adapters and PR state machinery in this visual
  pass. They have no header, keybinding, footer hint, or other user entry point.
- Changing diff add/remove fills or search-match highlighting.
