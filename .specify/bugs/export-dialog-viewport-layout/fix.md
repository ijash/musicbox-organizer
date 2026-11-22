# Bug Fix: Export Dialog Viewport and Responsive Layout

- **Slug**: export-dialog-viewport-layout
- **Fixed**: 2026-11-22
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Migrated `ExportCatalogDialog` from an embedded `egui::Window` trapped inside the main window to a native OS child viewport via `ctx.show_viewport_immediate(...)`. Reorganized the dialog into a pinned `TopBottomPanel::bottom` (holding live layout summary, readability warning, and `Export...` / `Cancel` action buttons) and a scrollable `CentralPanel` with full-width groups and `horizontal_wrapped` rows, ensuring actions and warnings are never clipped at `280×320px`. Extracted `render_export_catalog_contents` for robust headless `egui_kittest` coverage.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/ui/export_catalog_dialog.rs` | modified | Added `ctx.show_viewport_immediate` in `render_export_catalog_dialog`, extracted `render_export_catalog_contents` with pinned bottom panel, full-width groups, `horizontal_wrapped` inputs, and removed duplicate heading |
| `src/ui/app.rs` | modified | Removed duplicate dead call `render_export_catalog_dialog` from `render_central_content` |
| `src/ui/mod.rs` | modified | Re-exported `render_export_catalog_contents` and `render_export_catalog_dialog` |
| `tests/gui/export_catalog_dialog_test.rs` | modified | Added `test_export_catalog_dialog_narrow_pinned_buttons_280x320` asserting pinned buttons and cancel flow at 280×320px; updated existing tests to use `render_export_catalog_contents` |

## Diff Highlights (optional)

```rust
// In src/ui/export_catalog_dialog.rs
pub fn render_export_catalog_dialog(app: &mut MusicBoxApp, ctx: &egui::Context) {
    if !app.export_catalog_dialog.is_open {
        return;
    }

    let viewport_id = egui::ViewportId::from_hash_of("export_catalog_viewport");
    ctx.show_viewport_immediate(
        viewport_id,
        egui::ViewportBuilder::default()
            .with_title("Export Catalog")
            .with_inner_size([380.0, 520.0])
            .with_min_inner_size([280.0, 320.0])
            .with_resizable(true),
        |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested()) {
                app.export_catalog_dialog.close();
            }
            render_export_catalog_contents(app, ctx);
            if !app.export_catalog_dialog.is_open {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        },
    );
}
```

## Tests Added or Updated

- `tests/gui/export_catalog_dialog_test.rs::test_export_catalog_dialog_narrow_pinned_buttons_280x320` — Pins down `280×320px` minimum viewport rendering: asserts that `Export...` and `Cancel` buttons remain visible and enabled in the pinned bottom panel and clicking `Cancel` cleanly closes the dialog.
- `tests/gui/export_catalog_dialog_test.rs::test_unified_export_dialog_format_and_field_toggles` — Updated to drive `render_export_catalog_contents` directly.
- `tests/gui/export_catalog_dialog_test.rs::test_export_catalog_toolbar_button_opens_dialog_full_app` — Verifies full-app integration via `build_eframe` opening child viewport.

## Local Verification

- Commands run:
  - `cargo test --test gui_tests` → PASSED (17/17 tests pass)
  - `cargo test` → PASSED (all 84 workspace tests pass)
  - `cargo clippy --all-targets -- -D warnings` → PASSED (0 warnings)
  - `cargo fmt --check` → PASSED
  - `cargo build --release` → PASSED (binary size 12 MB < 20 MB ceiling)

## Deviations from Assessment

None.

## Follow-ups

- Run `/speckit-bug-test slug=export-dialog-viewport-layout` to record verification report.
