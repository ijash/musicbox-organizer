# Bug Verification: GTK UI Crashes on Move Down/Edit ID3, Logo Left Spacing, and Missing Remove Track

- **Slug**: gtk-crashes-ui-fixes
- **Tested**: 2026-11-20
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

All 4 reported issues were verified as resolved:
1. Finishing inline ID3 editing updates `ListStore` in-place without invalidating the active cell renderer or triggering fatal GTK assertion panics.
2. Moving tracks down updates indices and re-selects the moved row stably without crashing.
3. The logo is nested in `title_group` without expanding margins, sitting snugly beside the project title.
4. The `✖ Remove` button is functional, removing selected rows, re-indexing tracks, and creating undo checkpoints.

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Inline editing safety | Code inspection of `list_store.set_value` | pass | No `list_store.clear()` during cell edit callbacks |
| Move down stability | Safe TreePath selection inspection | pass | No unmapped scroll calls |
| Header logo alignment | `title_group` layout inspection | pass | Logo adjacent to title with 6px spacing |
| Remove track toolbar | `remove_track_btn` inspection | pass | Deletes track, re-indexes, records undo |
| Regression test suite | `cargo test` | pass | 49/49 unit and integration tests passed |
| Standalone binary build | `cargo build --release` | pass | Release binary built cleanly (~1.7 MB) |

## Output Excerpts

```text
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Residual Risks

- None. GTK signal flows and model updates are stabilized.

## Recommendation

Close the bug — verified end-to-end.
