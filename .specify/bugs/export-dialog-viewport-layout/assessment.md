# Bug Assessment: Export Dialog Viewport and Responsive Layout

- **Slug**: export-dialog-viewport-layout
- **Created**: 2026-11-22
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> Bug report on `ExportCatalogDialog` (`src/ui/export_catalog_dialog.rs`):
> 1. **Cannot move outside parent window**: Currently rendered as an embedded `egui::Window` trapped inside the main viewport. Switch to a native OS child viewport via `ctx.show_viewport_immediate(...)` (title `"Export Catalog"`, default `380×520`, `min_inner_size([280.0, 320.0])`, resizable, closes cleanly on OS window close button `ViewportCommand::Close` / `close_requested()`).
> 2. **Bottom summary & Export/Cancel buttons get clipped on resize / small resolutions**:
>    - Split the dialog into a **pinned bottom panel** (`egui::TopBottomPanel::bottom`) holding `Live Layout Summary`, the red `< 5.0 pt` warning, and `Cancel` / `Export...` buttons so actions are NEVER clipped even at `280×320px`.
>    - Wrap the form body (`Format`, `Fields to Include`, `Page Sizing & Layout`) inside `egui::CentralPanel` + `egui::ScrollArea::vertical().auto_shrink([false, false])`.
> 3. **Visual polish**:
>    - Remove the duplicate `"Export Catalog"` heading inside the body (keep only the compact playlist subtitle).
>    - Make each group frame stretch cleanly to `ui.available_width()` and use `horizontal_wrapped` for radio/input rows so nothing clips or misaligns when resized narrow.
> 4. **Headless testing (`egui_kittest`)**: Extract `render_export_catalog_contents(...)` so `egui_kittest` can test both narrow (`280×320px`) scrolling/pinned buttons and full-app dialog open/close cleanly without requiring an OS multi-window compositor.

## Symptom

When opening `ExportCatalogDialog`, the dialog is rendered as an `egui::Window` confined within the bounds of the main application window and cannot be moved outside it to another monitor or workspace. Furthermore, on smaller display resolutions or when resizing the dialog down towards its minimum size (`280×320px`), the bottom action buttons (`Export...` and `Cancel`) and the live layout summary / warning at the bottom of the single scroll area can be obscured or pushed below the fold, preventing the user from triggering export without scrolling. In addition, an extraneous duplicate heading `"Export Catalog"` is rendered inside the body, and radio button rows use rigid `ui.horizontal` layouts that overflow on narrow widths.

## Reproduction

1. Launch `./target/release/musicbox` with a playlist loaded.
2. Click `📄 Export Catalog...` on the toolbar.
3. Attempt to drag the modal window outside of the main application window boundary: observe that it is clamped/trapped within the main window viewport.
4. Resize the window or dialog to a narrow height/width (e.g. 280×320px): observe that the action buttons (`Export...` and `Cancel`) and live summary get scrolled out of view and can be clipped, requiring scrolling to perform primary actions.
5. Notice the redundant `"Export Catalog"` heading at the top of the body despite the window title already displaying `"Export Catalog"`.

## Suspected Code Paths

- `src/ui/export_catalog_dialog.rs:64-306` — `render_export_catalog_dialog()` creates an embedded `egui::Window::new("Export Catalog")` inside the primary `ctx`, rather than a separate native child viewport via `ctx.show_viewport_immediate(...)`.
- `src/ui/export_catalog_dialog.rs:72-305` — The entire dialog body (title, form inputs, live layout summary, error message, and action buttons) is placed in a single `egui::ScrollArea::vertical()`. The action buttons and summary are not pinned to the bottom.
- `src/ui/export_catalog_dialog.rs:73` — Redundant `ui.heading("Export Catalog");` displayed inside the dialog body.
- `src/ui/export_catalog_dialog.rs:85-103, 148-155, 163-186` — Form groups and control rows use `ui.horizontal` without wrapping, causing overflow at narrow widths.
- `tests/gui/export_catalog_dialog_test.rs:136-169` — Existing headless tests rely on `render_central_content` and lack a standalone content harness for `280×320px` pinned panel assertions.

## Root Cause Hypothesis

High confidence. The dialog was originally implemented using `egui::Window::new(...)` inside the primary viewport. `egui::Window` is an immediate-mode sub-surface drawn within the parent canvas, inherently unable to cross native window boundaries. Moreover, putting action buttons at the bottom of a monolithic vertical scroll area violates responsive dialog UX standards, causing essential controls to be clipped when the viewport is resized.

## Proposed Remediation

**Preferred**:
1. **OS Child Viewport**:
   In `src/ui/export_catalog_dialog.rs`, update `render_export_catalog_dialog()` to invoke `ctx.show_viewport_immediate()` using `ViewportId::from_hash_of("export_catalog_viewport")` with `ViewportBuilder::default().with_title("Export Catalog").with_inner_size([380.0, 520.0]).with_min_inner_size([280.0, 320.0]).with_resizable(true)`. Check `ctx.input(|i| i.viewport().close_requested())` and dispatch `app.export_catalog_dialog.close()` cleanly.
2. **Pinned Bottom Panel + Central Scroll Area**:
   Extract `pub fn render_export_catalog_contents(app: &mut MusicBoxApp, ctx: &egui::Context)`:
   - Render `egui::TopBottomPanel::bottom("export_catalog_bottom_panel")` holding:
     - Live layout summary (`Columns: ... | Font Size: ... | Sheet: ...`)
     - Red readability warning (`layout.readability_warning`)
     - Optional error message (`app.export_catalog_dialog.error_message`)
     - Pinned action buttons (`Export...` and `Cancel`)
   - Render `egui::CentralPanel::default()` with `egui::ScrollArea::vertical().auto_shrink([false, false])` holding the configuration form.
3. **Visual Polish & Narrow Width Resilience**:
   - Remove `ui.heading("Export Catalog");`, keeping the compact playlist subtitle `ui.label(...)`.
   - Set group frames to fill `ui.available_width()`.
   - Use `ui.horizontal_wrapped` for format radio buttons, unit selectors, and numeric inputs to ensure clean wrapping down to 280px width.
4. **Headless Testing Support**:
   - Update `tests/gui/export_catalog_dialog_test.rs` to test `render_export_catalog_contents` directly at minimum size `280×320px`, verifying that `Export...` and `Cancel` buttons remain visible and enabled regardless of scrolling.
   - Verify that full-app toolbar integration continues to pass via `Harness::builder().build_eframe(...)`.

**Alternatives**:
- Retaining `egui::Window` inside the main viewport: Rejected because it does not satisfy the requirement to move the dialog outside the parent window.

**Files likely to change**:
- `src/ui/export_catalog_dialog.rs`
- `src/ui/app.rs` (if any adjustments to `render_central_content` are needed)
- `tests/gui/export_catalog_dialog_test.rs`

**Tests to add or update**:
- `tests/gui/export_catalog_dialog_test.rs::test_export_catalog_dialog_narrow_pinned_buttons_280x320`: Verifies pinned bottom panel and action buttons at minimum `280×320px` viewport.
- `tests/gui/export_catalog_dialog_test.rs::test_export_catalog_toolbar_button_opens_dialog_full_app`: Confirms dialog opening and action button accessibility under `eframe::App`.

## Risks & Considerations

- Viewport embedding in headless tests: In headless/test environments where multi-viewport support is not present in the backend, `show_viewport_immediate` falls back to `ViewportClass::Embedded`, which `egui_kittest` handles cleanly.
- Panel naming: Distinct panel IDs (`"export_catalog_bottom_panel"`, `"export_catalog_central_panel"`) prevent ID clashes with root app panels.

## Open Questions

- None. Requirements, UI layout structure, and testing strategy are well-defined.
