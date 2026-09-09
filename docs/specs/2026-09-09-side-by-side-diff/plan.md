# Implementation plan

1. Change the navigator default to `left` while retaining every explicit configuration value and
   the existing clockwise position cycle.
2. Add one shared wide-diff geometry calculation used by row measurement, painting, gutters, and
   text-selection hit testing.
3. Paint wide Changes diffs as old/new lanes and keep structural rows full-width; retain unified
   rendering below the threshold and outside Changes diff view.
4. Add render and configuration coverage for the new default, the responsive threshold, lane
   placement, wrapping, and pointer mapping.
5. Run formatting, Clippy, the full test suite, and a release build; record the results here.

## Verification

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features` — 728 passed, 1 live-network test ignored
- `cargo build --release --all-features`
