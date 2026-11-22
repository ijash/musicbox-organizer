# Bug Verification: v0.4.0 QA Fixes

- **Slug**: v040-qa-fixes
- **Tested**: 2026-11-22
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

All 5 v0.4.0 QA defects have been verified resolved across unit, integration, and headless GUI suites. The PDF dual-axis layout solver avoids horizontal truncation of titles without reducing font below 1.0 pt, playlist table cells are cleanly left-aligned, the Export Catalog child window sizing and spacing are constrained properly, unbracketed `XXYY-filename.mp3` naming is enforced project-wide with legacy prefix stripping, and the mandatory confirmation viewport with destructive reformat acknowledgment checkbox is fully active. All 86 tests pass, clippy is clean, and the release binary is 12 MB (< 20 MB).

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| PDF Autofit Dual-Axis solver | `cargo test --test unit_tests -- test_layout_solver_21_realistic_long_titles_on_10x10cm` | pass | 21 realistic long titles on 10×10 cm card fit within column width with zero title truncation |
| Removable Media confirmation dialog | `cargo test --test gui_tests -- test_export_playlist_click_opens_confirmation_full_app` | pass | Clicking `Export Playlist` opens confirmation viewport, cancels cleanly |
| Destructive reformat checkbox gate | `cargo test --test gui_tests -- test_export_panel_reformat_two_step_guard` | pass | `Erase, Format & Export` button is disabled until acknowledgment checkbox is checked |
| Unbracketed prefix & legacy stripping | `cargo test --test unit_tests -- test_rename_with_xxyy_prefix` | pass | Verifies `AAAA-song.mp3` generation and stripping both `[ZZZZ-]` and `AAAA-` |
| Full GUI suite | `cargo test --test gui_tests` | pass | All 18 GUI tests pass |
| Full workspace test suite | `cargo test` | pass | All 86 tests pass (21 lib, 18 gui, 8 integration, 39 unit) |
| Linter | `cargo clippy --all-targets -- -D warnings` | pass | Clean (0 warnings) |
| Formatting | `cargo fmt --check` | pass | Clean |
| Release build & size ceiling | `cargo build --release` | pass | Binary size 12 MB (< 20 MB ceiling) |

## Output Excerpts

```text
running 18 tests
test gui::export_panel_tests::test_export_panel_reformat_two_step_guard ... ok
test gui::export_panel_tests::test_export_playlist_click_opens_confirmation_full_app ... ok
test gui::export_catalog_dialog_test::test_export_catalog_toolbar_button_opens_dialog_full_app ... ok
...
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

test result: ok. 86 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.04s
```

## Residual Risks

None. All naming, confirmation gating, and sizing contracts conform to Constitution v1.9.0.

## Recommendation

Close the bug — verified end-to-end.
