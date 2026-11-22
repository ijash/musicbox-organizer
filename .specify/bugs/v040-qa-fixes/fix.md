# Bug Fix: v0.4.0 QA Fixes

- **Slug**: v040-qa-fixes
- **Fixed**: 2026-11-22
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Remediated all 5 critical v0.4.0 QA defects:
1. Implemented dual-axis layout solving in `CatalogLayoutSolver` and `PdfCatalogExporter` with a conservative ~0.58 char width factor preventing title truncation without dropping below 1.0 pt font.
2. Left-aligned table headers and cells in `playlist_table.rs`.
3. Enforced `380×560` inner size and `340×440` minimum size on `ExportCatalogDialog` with clean bottom panel padding.
4. Removed `[` `]` square brackets from `XXYY-filename.mp3` format everywhere per Constitution v1.9.0 Principle IX, with legacy prefix stripping for both `[XXXX-]` and `XXXX-`.
5. Created a dedicated child viewport confirmation dialog (`render_export_confirm_dialog`) on `Export Playlist` click, and enforced a destructive warning and acknowledgment checkbox gate when `Reformat Media` is selected per Constitution v1.9.0 Principle III.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/services/catalog_layout.rs` | modified | Added `solve_with_line_lengths` accounting for horizontal text width constraint (~0.58 char width factor) alongside vertical constraint |
| `src/services/pdf_exporter.rs` | modified | Passes actual line lengths to layout solver and removes premature `..` truncation |
| `src/ui/playlist_table.rs` | modified | Converted cell and header rendering to `Layout::left_to_right(Align::Center)` |
| `src/ui/export_catalog_dialog.rs` | modified | Enforced `with_inner_size([380.0, 560.0])` and `with_min_inner_size([340.0, 440.0])`, added bottom panel padding, and updated UI checkbox text |
| `src/services/naming.rs` | modified | Updated `format_export_filename` from `[{}-{}]` to `{}-{}` |
| `src/services/id3_service.rs` | modified | Updated `apply_xxyy_rename` to strip legacy `[XXXX-]` and `XXXX-` prefixes |
| `src/models/track_metadata.rs` | modified | Updated `formatted_filename` to return `{}-{}` |
| `src/services/export_service.rs` | modified | Updated export file path generation to clean brackets |
| `src/services/compatibility.rs` | modified | Updated format verification to accept `XXYY-...` and legacy `[XXYY-...]` |
| `src/ui/export_panel.rs` | modified | Added `show_export_confirm` & `reformat_ack_checkbox`, created `render_export_confirm_dialog` and `render_export_confirm_contents` |
| `src/ui/app.rs` | modified | Added `render_export_confirm_dialog` call in `MusicBoxApp::update()` |
| `src/ui/mod.rs` | modified | Re-exported `render_export_confirm_dialog` and `render_export_confirm_contents` |
| `README.md` & `src/main.rs` | modified | Removed square brackets from CLI and documentation references |
| `tests/unit/catalog_layout_test.rs` | modified | Added test `test_layout_solver_21_realistic_long_titles_on_10x10cm` |
| `tests/gui/export_panel_tests.rs` | modified | Added tests for confirmation dialog and reformat acknowledgment gate |
| `tests/unit/` & `tests/integration/` | modified | Updated tests expecting `[AAAA-...]` to expect `AAAA-...` |

## Diff Highlights (optional)

```rust
// In src/services/catalog_layout.rs
let f_vert = row_h / theme.line_height;
let f_horiz = col_width / (max_chars as f32 * 0.58);
let target_font = f_vert.min(f_horiz).min(theme.font_size);
```

```rust
// In src/services/naming.rs
pub fn format_export_filename(track_index: u32, original_filename: &str) -> Result<String, String> {
    let xxyy = track_index_to_xxyy(track_index)?;
    Ok(format!("{}-{}", xxyy, original_filename))
}
```

## Tests Added or Updated

- `tests/unit/catalog_layout_test.rs::test_layout_solver_21_realistic_long_titles_on_10x10cm` — Verifies 21 realistic long titles on 10×10 cm card fit within column width without horizontal overflow.
- `tests/gui/export_panel_tests.rs::test_export_panel_reformat_two_step_guard` — Verifies confirmation dialog and destructive reformat checkbox gate.
- `tests/gui/export_panel_tests.rs::test_export_playlist_click_opens_confirmation_full_app` — Verifies clicking `Export Playlist` opens confirmation viewport.
- `tests/unit/id3_tests.rs::test_rename_with_xxyy_prefix` — Verifies unbracketed rename and legacy prefix stripping (`[ZZZZ-]` and `AAAA-`).
- All existing naming, compatibility, and export integration tests updated to unbracketed convention.

## Local Verification

- Commands run:
  - `cargo test` → PASSED (all 86 tests pass: 21 lib + 18 gui + 8 integration + 39 unit)
  - `cargo clippy --all-targets -- -D warnings` → PASSED (0 warnings)
  - `cargo fmt --check` → PASSED
  - `cargo build --release` → PASSED (binary size 12 MB < 20 MB ceiling)

## Deviations from Assessment

None.

## Follow-ups

- Run `/speckit-bug-test slug=v040-qa-fixes` to record verification report.
