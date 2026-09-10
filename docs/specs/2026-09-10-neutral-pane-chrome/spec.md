# Neutral pane chrome

## Goal

Diple's main review workspace uses quiet, neutral structure like Lumen: pane boundaries organize
the screen without competing with the diff, while crisp text hierarchy keeps focus immediately
visible.

## User contract

- Main-pane borders always use a neutral gray, whether or not the pane is focused. Dark themes use
  the terminal's neutral dark gray; light themes use their own light surface gray.
- The focused pane title uses bold primary text; an unfocused pane title uses secondary text.
- Moving focus between the navigator and read pane changes the title emphasis without flashing
  or recoloring the full pane outline.
- Navigator directory names use primary text and section labels use secondary text, preserving a
  clear hierarchy without making ordinary content look disabled.
- The top tab strip and bottom action bar use the terminal background. Their hierarchy comes from
  text color, weight, and spacing rather than full-width color bands.
- The treatment applies consistently to Changes and All files in both side-by-side and stacked
  layouts, and follows the active light or dark theme.

## Non-goals

- Changing pane order, proportions, divider hit targets, or responsive breakpoints.
- Restyling comment composers, pickers, or confirmation dialogs, whose borders communicate an
  active task rather than workspace focus.
- Replacing the existing Catppuccin-derived palettes.
