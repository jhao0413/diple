# Wide side-by-side diff

## Status

Accepted

## Problem

The default review layout places the changed-file navigator on the right and always paints a
unified diff. On a wide terminal this leaves enough horizontal room for two readable source
columns, but the diff does not use it.

## Observable behavior

- A fresh Diple session places the navigator on the left and the read pane on the right.
- An explicit `navigator_position` setting and the existing position-cycle action continue to
  take precedence over the default.
- In the Changes tab's diff view, a read-pane interior at least 120 columns wide automatically
  paints an old column on the left and a new column on the right.
- Deletions paint only in the old column, insertions only in the new column, and context lines in
  both columns. Each source column owns its line-number gutter, and a divider separates them.
- Fold rows continue to span the entire read pane.
- A narrower read pane, the All files view, and PR snippets retain the existing unified rendering.
- Wrapping, horizontal scrolling, find highlights, cursor and selection fills, inline comments,
  gutter comments, and mouse text selection continue to agree with the painted geometry.

## Non-goals

- Pairing an arbitrary deletion row and insertion row onto one display row.
- Adding a user-facing diff-mode setting.
- Changing comment anchors, exported snippets, or the underlying diff model.

