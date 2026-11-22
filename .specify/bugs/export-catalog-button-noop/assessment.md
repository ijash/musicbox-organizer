# Bug Assessment: Export Catalog Toolbar Button No-Op

- **Slug**: export-catalog-button-noop
- **Created**: 2026-11-22
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> In `./target/release/musicbox` (on branch `feat/011-pdf-catalog-exporter`), clicking the `Export Catalog...` toolbar button does nothing — the Export Catalog modal dialog never opens.
>
> Inspect the full click-to-render path across `src/ui/toolbar.rs`, `src/ui/app.rs`, and `src/ui/export_catalog_dialog.rs`:
> 1. Identify why clicking `Export Catalog...` fails to open/render `ExportCatalogDialog` (e.g., missing wiring from toolbar action to dialog state, `show()` not invoked in `MusicBoxApp::update()`, immediate close on same frame, or silent empty-playlist guard).
> 2. Also check that clicking Export inside the dialog properly triggers `rfd::FileDialog` (starting in `std::env::current_dir()`) and writes PDF/TXT/RTF via `CatalogExportService`.
> 3. Propose an `egui_kittest` regression test driving the **full `MusicBoxApp` root UI** (clicking the toolbar `Export Catalog...` button and asserting the dialog window and controls render visibly).

## Symptom

Clicking the `📄 Export Catalog...` toolbar button in the running application (`./target/release/musicbox`) does nothing visually. The `Export Catalog` modal window never renders on screen, even when tracks are present in the playlist.

## Reproduction

1. Launch `./target/release/musicbox` (or `cargo run --bin musicbox`).
2. Add one or more MP3 tracks to the playlist (or open an existing playlist).
3. Click the `📄 Export Catalog...` button on the top toolbar.
4. Observe that no dialog window appears.

## Suspected Code Paths

- `src/ui/toolbar.rs:49-51` — Clicking `📄 Export Catalog...` calls `app.export_catalog_dialog.open()`, setting `is_open = true`. The toolbar wiring itself functions as intended.
- `src/ui/app.rs:338-340` — In `impl eframe::App for MusicBoxApp::update()`, floating modal dialog rendering only invokes `crate::ui::dsp_dialog::render_dsp_dialog(self, ctx);`. The call `crate::ui::export_catalog_dialog::render_export_catalog_dialog(self, ctx);` was completely omitted from `MusicBoxApp::update()`.
- `src/ui/app.rs:174` — `render_export_catalog_dialog` was previously added to `MusicBoxApp::render_central_content()`, which was called by certain mock test harnesses but is never called in production by `eframe::App::update()`.
- `src/ui/export_catalog_dialog.rs:341-375` — `trigger_catalog_export()` properly triggers `rfd::FileDialog::new().set_directory(start_dir)` starting from `std::env::current_dir()`, opens a native save dialog with format filters (`.pdf`, `.txt`, `.rtf`), converts tracks via `convert_project_tracks()`, and executes `CatalogExportService::export_catalog()`.

## Root Cause Hypothesis

High confidence. The bug is caused by a missing rendering invocation in `MusicBoxApp::update()` in `src/ui/app.rs`. When `MusicBoxApp` runs under `eframe`, each frame executes `eframe::App::update()`. While toolbar button interaction sets `app.export_catalog_dialog.is_open = true`, `MusicBoxApp::update()` only dispatches `render_dsp_dialog(self, ctx)` and never calls `crate::ui::export_catalog_dialog::render_export_catalog_dialog(self, ctx)`. Existing GUI tests did not detect this bug because they were constructed using `Harness::new_ui_state` calling `state.render_central_content(ui)`, bypassing `eframe::App::update()`.

## Proposed Remediation

**Preferred**:
1. In `src/ui/app.rs`, add `crate::ui::export_catalog_dialog::render_export_catalog_dialog(self, ctx);` inside `impl eframe::App for MusicBoxApp::update()` directly alongside `render_dsp_dialog(self, ctx)` under the modal/floating window section.
2. In `tests/gui/export_catalog_dialog_test.rs`, add an `egui_kittest` regression test that tests the root `eframe::App` UI via `Harness::builder().build_eframe(...)` or `Harness::builder().build_state(|ctx, app| { app.update(ctx, &mut eframe::Frame::_new_kittest()); }, app)`. The test clicks `harness.get_by_label("📄 Export Catalog...").click()`, advances the harness with `harness.run()`, and asserts that the dialog window and its controls (e.g., label `"Export Catalog"`, format radio button `"PDF"`, button `"Export..."`) are queried and present in the accessibility hierarchy.

**Alternatives**:
- None needed; placing modal window rendering in `MusicBoxApp::update()` is the standard pattern matching `render_dsp_dialog`.

**Files likely to change**:
- `src/ui/app.rs`
- `tests/gui/export_catalog_dialog_test.rs`

**Tests to add or update**:
- `test_export_catalog_toolbar_button_opens_dialog_full_app`: An `egui_kittest` headless test running `MusicBoxApp::update` via `Harness::builder().build_eframe(|_cc| app)` or `build_state`, clicking `"📄 Export Catalog..."`, calling `harness.run()`, and asserting that `app.export_catalog_dialog.is_open` is `true` and the `"Export Catalog"` window is queryable.

## Risks & Considerations

- Dialog z-ordering: Floating modal windows rendered at the end of `update()` render over top of central and dock panels, which is the desired behavior for a modal dialog.
- Responsive layout: `ExportCatalogDialog` already supports resizing and viewport constraints down to 300x400.

## Open Questions

- None. Root cause and remediation are verified directly against source code and test suite.
