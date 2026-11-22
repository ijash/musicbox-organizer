# Bug Fix: Export Catalog Toolbar Button No-Op

- **Slug**: export-catalog-button-noop
- **Fixed**: 2026-11-22
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Fixed the no-op behavior of the `📄 Export Catalog...` toolbar button by adding `crate::ui::export_catalog_dialog::render_export_catalog_dialog(self, ctx);` inside `impl eframe::App for MusicBoxApp::update()`. Added an `egui_kittest` regression test driving the root `eframe::App` to ensure the modal dialog and its action buttons render visibly on click.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/ui/app.rs` | modified | Added `render_export_catalog_dialog(self, ctx)` under the floating modal windows section of `MusicBoxApp::update()` |
| `tests/gui/export_catalog_dialog_test.rs` | modified | Added `test_export_catalog_toolbar_button_opens_dialog_full_app` driving full root UI via `build_eframe` |

## Diff Highlights (optional)

```rust
// src/ui/app.rs in impl eframe::App for MusicBoxApp::update()
         // Modals / Floating Windows
         crate::ui::dsp_dialog::render_dsp_dialog(self, ctx);
+        crate::ui::export_catalog_dialog::render_export_catalog_dialog(self, ctx);
```

## Tests Added or Updated

- `tests/gui/export_catalog_dialog_test.rs::test_export_catalog_toolbar_button_opens_dialog_full_app` — Boots full root `MusicBoxApp` via `Harness::builder().build_eframe()`, clicks `"📄 Export Catalog..."`, and verifies that the dialog window opens and `"Export..."` and `"Cancel"` buttons render visibly in the AccessKit tree.

## Local Verification

- Commands run:
  - `cargo test --test gui_tests -- test_export_catalog_toolbar_button_opens_dialog_full_app` → PASSED (confirmed failing when fix was temporarily removed)
  - `cargo test` → PASSED (all 21 lib + 17 gui + 8 integration + 38 unit tests pass)
  - `cargo clippy --all-targets -- -D warnings` → PASSED (0 warnings)
  - `cargo fmt --check` → PASSED
  - `cargo build --release` → PASSED (binary size 12 MB < 20 MB ceiling)

## Deviations from Assessment

None.

## Follow-ups

- Run `/speckit-bug-test slug=export-catalog-button-noop` to record verification report.
