# Bug Assessment: v0.4.0 QA Fixes

- **Slug**: v040-qa-fixes
- **Created**: 2026-11-22
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> Triage and propose remediation for these 5 critical QA defects found in v0.4.0 testing:
> 1. **PDF Autofit truncates titles (`..`) instead of shrinking font to fit (`src/services/catalog_layout.rs` & `pdf_exporter.rs`)**:
>    - Currently the layout solver only checks vertical row capacity and picks too many columns with an oversized font (e.g., 4 cols at 8pt), truncating almost every title (`02. Ruak Ru..`).
>    - Fix: For each candidate column count `c` (`1..=5`), solve the maximum font size `f` that fits **BOTH vertically** (`rows_per_col * f * line_height <= inner_height`) **AND horizontally** (the actual measured width of the longest track string in each column fits inside `col_width` **with zero truncation**). Pick the `(c, f)` with the largest readable font size `f` where all titles fit completely without `..` (only truncate as an absolute last resort if `f < 1.0 pt`).
> 2. **Playlist Table text is center-aligned (`src/ui/playlist_table.rs`)**:
>    - Change `Export Filename`, `Title`, `Artist`, and `Source File` cells to **left-aligned** (`Layout::left_to_right(Align::Center)`) so track names are easy to scan.
> 3. **Export Catalog child window still clips on resize (`src/ui/export_catalog_dialog.rs`)**:
>    - Enforce a proper `with_min_inner_size([340.0, 440.0])` and comfortable default `with_inner_size([380.0, 560.0])` on the child viewport, and ensure the bottom summary/action bar has clean spacing so nothing clips when resized.
> 4. **Remove square brackets `[` `]` from export filename prefix everywhere (Constitution v1.9.0)**:
>    - Search the entire repo (`grep -rn "XXYY\|AAAA" src/ tests/ README.md`) and update `src/services/naming.rs`, `id3_service.rs`, `export_service.rs`, UI labels/headers (`Export Filename (XXYY-filename.mp3)`), `README.md`, and all unit/integration/GUI tests from `[AAAA-file.mp3]` to `AAAA-file.mp3` (also stripping legacy `[XXXX-]` or `XXXX-` prefixes cleanly on rename).
> 5. **Mandatory Separate Confirmation Dialog Window for Export Media & Reformat Drive (`src/ui/export_panel.rs` & `src/ui/app.rs`)**:
>    - Clicking `Export Playlist` MUST open a dedicated separate confirmation viewport (`ctx.show_viewport_immediate`) showing target device, mount path, file count, and subpath, with `Cancel` (safe default) and `Confirm Export` buttons.
>    - When `Reformat Media` is checked, display a prominent red destructive warning (`ALL DATA ON <DEVICE> WILL BE PERMANENTLY ERASED`) and require explicit confirmation (e.g., checkbox `"I understand all data on this drive will be erased"` before enabling the red `"Erase, Format & Export"` button).
>    - Add `egui_kittest` tests verifying that clicking `Export Playlist` opens the confirmation dialog and never runs export/reformat without explicit confirmation.

## Symptom

1. PDF catalog exporter generates multi-column output with truncated track titles (`..`) instead of dynamically calculating font sizes that fit horizontally and vertically without truncation.
2. The main playlist table renders text in cells (`Export Filename`, `Title`, `Artist`, `Source File`) center-aligned within fixed column allocations, reducing readability.
3. The Export Catalog child viewport clips bottom summary information or action buttons when resized down to small sizes due to insufficient `min_inner_size` bounds (`[280.0, 320.0]`).
4. Export filenames still generate with square brackets (`[AAAA-file.mp3]`) instead of the unbracketed format (`AAAA-file.mp3`) mandated by Constitution v1.9.0 Principle IX for FAT32 / car head unit audio compatibility.
5. Clicking `Export Playlist` in the Removable Media panel starts export immediately without a dedicated confirmation dialog, and selecting `Reformat Media` lacks a mandatory second-gate acknowledgment checkbox before executing destructive disk erasure, violating Constitution v1.9.0 Principle III.

## Reproduction

1. **PDF Title Truncation**: Export a 20-track playlist to a 10x10 cm card. Observe that the solver selects 4 columns with ~8pt font, truncating titles into `01. Someth..`.
2. **Playlist Table Alignment**: Open any playlist in the GUI; observe that cell text in the playlist table is horizontally centered rather than left-aligned.
3. **Export Catalog Sizing**: Open Export Catalog and resize the window to minimum dimensions; observe that inputs and summary items clip.
4. **Export Prefix Format**: Export a track; observe the generated file is named `[AAAA-track.mp3]` instead of `AAAA-track.mp3`.
5. **Missing Confirmation Dialog**: In the Removable Media panel, select a device and click `💾 Export Playlist` with `Reformat Media` unchecked; observe that export begins immediately without confirmation. Check `Reformat Media`; observe that the existing confirmation modal does not enforce an acknowledgment checkbox.

## Suspected Code Paths

- `src/services/catalog_layout.rs:50-80` — `CatalogLayoutSolver::solve()` only considers vertical row packing (`canvas_height / rows_needed`), choosing excess columns and oversized fonts that clip horizontally.
- `src/services/pdf_exporter.rs:188-199` — Hard-coded truncation `approx_char_limit` truncates with `..` instead of checking horizontal constraint during layout solving.
- `src/ui/playlist_table.rs:75-170` — Header and cell labels use default centered alignment in `ui.add_sized()`.
- `src/ui/export_catalog_dialog.rs:14-17` — Viewport configuration sets `min_inner_size([280.0, 320.0])` and `inner_size([380.0, 520.0])`.
- `src/services/naming.rs:88` — `format_export_filename()` outputs `format!("[{}-{}]", xxyy, original_filename)`.
- `src/services/id3_service.rs:98-112` — `apply_xxyy_rename()` only strips `[XXXX-]` and doesn't handle unbracketed `XXXX-`.
- `src/ui/export_panel.rs:85-115` — `💾 Export Playlist` button directly calls `start_export()` when `reformat_enabled` is false, and uses an embedded window without acknowledgment checkbox when true.
- `src/ui/app.rs` — Missing child viewport rendering for the export confirmation modal.

## Root Cause Hypothesis

High confidence. 
1. The catalog layout solver only solved for vertical canvas capacity, disregarding track text lengths;
2. egui's `ui.add_sized()` defaults to centered alignment for selectable labels and labels when width exceeds text length;
3. Viewport size minimums were set too small for the configuration controls;
4. The bracketed format `[XXYY-]` from spec v0.1.0 was never updated to Constitution v1.9.0's `XXYY-` standard;
5. The removable media export action was wired directly to `start_export()` on click rather than routing through an isolated confirmation viewport.

## Proposed Remediation

**Preferred**:

1. **PDF Autofit Layout Solver**:
   - Update `CatalogLayoutSolver` to take candidate track strings (or max characters per track). For each candidate column count `c` in `1..=5`, compute:
     - Vertical limit: `f_vert = canvas_height / (rows_per_col * theme.line_height)`
     - Horizontal limit: `f_horiz = col_width / (max_chars_in_col * 0.52)`
     - Candidate font size: `f_cand = f_vert.min(f_horiz).min(theme.font_size)`
   - Choose candidate `(c, f)` that maximizes `f_cand` such that all strings fit without truncation (only fall back to truncation if `f < 1.0 pt`).
   - In `pdf_exporter.rs`, remove aggressive `..` truncation when font size accommodates the text.

2. **Playlist Table Left-Alignment**:
   - In `src/ui/playlist_table.rs`, wrap headers and table cells in `ui.allocate_ui_with_layout([w, row_height].into(), egui::Layout::left_to_right(egui::Align::Center), |ui| { ... })` so `Export Filename`, `Title`, `Artist`, and `Source File` are cleanly left-aligned.

3. **Export Catalog Viewport Constraints**:
   - In `src/ui/export_catalog_dialog.rs`, set `with_inner_size([380.0, 560.0])` and `with_min_inner_size([340.0, 440.0])`. Add padding to the pinned bottom action bar.

4. **Unbracketed XXYY Export Prefix Format**:
   - Update `src/services/naming.rs::format_export_filename()` to return `format!("{}-{}", xxyy, original_filename)`.
   - Update `src/services/id3_service.rs::apply_xxyy_rename()` to strip both legacy `[XXXX-]` and `XXXX-` prefixes.
   - Update UI headers/labels in `playlist_table.rs` and `export_catalog_dialog.rs` (`Export Filename ([XXYY-filename.mp3])` → `Export Filename (XXYY-filename.mp3)`).
   - Update `README.md` and all test suites across `tests/` (`naming_tests`, `id3_tests`, `export_service_tests`, `compatibility_tests`, `export_integration_tests`, `gui_tests`).

5. **Mandatory Confirmation Viewport for Export & Reformat**:
   - In `src/ui/export_panel.rs`, add `show_export_confirm: bool` and `reformat_ack_checkbox: bool` to `ExportPanelState`.
   - Clicking `💾 Export Playlist` sets `show_export_confirm = true` and `reformat_ack_checkbox = false` (never exports immediately).
   - Implement `render_export_confirm_dialog(app: &mut MusicBoxApp, ctx: &egui::Context)` using `ctx.show_viewport_immediate(...)`:
     - Shows target device name, mount path, track count, and subpath.
     - When `reformat_enabled` is false: `Confirm Export` and `Cancel` buttons.
     - When `reformat_enabled` is true: prominent red warning (`ALL DATA ON <DEVICE> WILL BE PERMANENTLY ERASED`), checkbox `"I understand all data on this drive will be erased"`, and red button `"Erase, Format & Export"` enabled only when checkbox is checked.
   - Wire `render_export_confirm_dialog` in `MusicBoxApp::update()`.
   - Add `egui_kittest` regression tests confirming the confirmation dialog opens, prevents direct export, and requires the acknowledgment checkbox for reformat.

**Alternatives**:
- In-place modal window for export confirmation: Rejected because Constitution v1.9.0 Principle III mandates a dedicated separate confirmation dialog window.

**Files likely to change**:
- `src/services/catalog_layout.rs`
- `src/services/pdf_exporter.rs`
- `src/ui/playlist_table.rs`
- `src/ui/export_catalog_dialog.rs`
- `src/services/naming.rs`
- `src/services/id3_service.rs`
- `src/services/export_service.rs`
- `src/ui/export_panel.rs`
- `src/ui/app.rs`
- `src/ui/mod.rs`
- `README.md`
- `tests/unit/naming_tests.rs`
- `tests/unit/id3_tests.rs`
- `tests/unit/export_service_tests.rs`
- `tests/unit/compatibility_tests.rs`
- `tests/unit/catalog_layout_test.rs`
- `tests/integration/export_integration_tests.rs`
- `tests/gui/export_panel_tests.rs`
- `tests/gui/export_catalog_dialog_test.rs`

**Tests to add or update**:
- `tests/unit/catalog_layout_test.rs`: Test horizontal string fit without truncation across column candidates.
- `tests/gui/export_panel_tests.rs`: Test `Export Playlist` opens confirmation dialog, test reformat gate requires checkbox before button is enabled.
- Update existing tests expecting bracketed filenames `[AAAA-...]` to unbracketed `AAAA-...`.

## Risks & Considerations

- Global prefix rename impact: Any tests matching exact filenames will need updating from `[AAAA-...]` to `AAAA-...`.
- Backward compatibility: Renamer must strip both `[XXXX-]` and `XXXX-` to prevent duplicate prefixes.

## Open Questions

- None. All requirements directly align with Constitution v1.9.0.
