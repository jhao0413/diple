# Demo recording for Diple. Regenerate with `cargo build --release && vhs assets/demo.tape`.
# Builds a throwaway git repo, then drives this checkout's release binary through the review loop.
# Needs `git` and `vhs`.

Require git

Output assets/demo.gif

Set Shell "bash"
Set FontSize 13
Set Width 1320
Set Height 820
Set Theme "Catppuccin Mocha"
Set Padding 16
Set TypingSpeed 45ms

# --- Hidden setup: a temp repo with a committed baseline + an uncommitted edit to review ---
Hide
Type "BIN=$(bash assets/demo-bin.sh) && bash assets/demo-fixture.sh /tmp/diple-demo && cd /tmp/diple-demo && $BIN || exit 1" Enter
Show

# --- The review loop ---
Sleep 2s

# Focus the diff and move into the changed region.
Tab
Sleep 700ms
Type "jjjj"
Sleep 900ms

# Select a line range...
Type "v"
Sleep 400ms
Type "jj"
Sleep 700ms

# ...and comment on it.
Type "c"
Sleep 600ms
Type "guard the empty value here too?"
Sleep 800ms
Enter
Sleep 1200ms

# Confirm the review, restore the terminal, and print the clean payload.
Type "s"
Sleep 900ms
Enter
Sleep 1800ms
