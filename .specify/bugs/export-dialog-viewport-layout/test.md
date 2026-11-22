# Bug Verification: Export Dialog Viewport and Responsive Layout

- **Slug**: export-dialog-viewport-layout
- **Tested**: 2026-11-22
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

The export catalog dialog child viewport and responsive layout bug is verified fixed. `ExportCatalogDialog` now spawns as a native OS child viewport via `ctx.show_viewport_immediate(...)`, pins primary action buttons and layout warnings to `TopBottomPanel::bottom` so they are never clipped at minimum size (`280×320px`), and scrolls form content within `CentralPanel`. All 84 workspace tests, headless GUI tests, clippy checks, and formatting checks pass with zero regressions.

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Pinned action buttons at minimum resolution | `cargo test --test gui_tests -- test_export_catalog_dialog_narrow_pinned_buttons_280x320` | pass | At 280×320px, `Export...` and `Cancel` remain queryable, visible, enabled, and functional |
| Full-app child viewport integration | `cargo test --test gui_tests -- test_export_catalog_toolbar_button_opens_dialog_full_app` | pass | Boots root `MusicBoxApp`, clicks toolbar button, opens viewport, queries dialog action buttons |
| Unified format and fields toggles | `cargo test --test gui_tests -- test_unified_export_dialog_format_and_field_toggles` | pass | Toggles PDF/TXT/RTF formats and fields inside extracted content renderer |
| Full GUI test suite | `cargo test --test gui_tests` | pass | All 17 GUI tests pass |
| Full test suite | `cargo test` | pass | 21 lib + 17 gui + 8 integration + 38 unit = 84 tests passed |
| Lint / type-check | `cargo clippy --all-targets -- -D warnings` | pass | Clean (0 warnings) |
| Formatting check | `cargo fmt --check` | pass | Clean |
| Release build & size gate | `cargo build --release` | pass | Standalone binary size 12 MB (< 20 MB ceiling) |

## Output Excerpts

```text
running 17 tests
test gui::export_catalog_dialog_test::test_unified_export_dialog_format_and_field_toggles ... ok
test gui::export_catalog_dialog_test::test_export_catalog_dialog_narrow_pinned_buttons_280x320 ... ok
...
test gui::export_catalog_dialog_test::test_export_catalog_toolbar_button_opens_dialog_full_app ... ok
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

test result: ok. 84 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.04s
```

## Residual Risks

None. In headless test environments without an OS window compositor, `show_viewport_immediate` falls back cleanly to embedded viewport rendering.

## Recommendation

Close the bug — verified end-to-end.
