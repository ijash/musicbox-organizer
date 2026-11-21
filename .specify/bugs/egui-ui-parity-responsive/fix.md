# Bug Fix: egui UI Parity & Responsive Layout Regressions

- **Slug**: egui-ui-parity-responsive
- **Fixed**: 2026-11-22
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Applied remediations for all 9 `eframe`/`egui` visual QA regressions: replaced missing Undo/Redo glyphs with standard Unicode arrows (`↩` / `↪`), added working directory context to `rfd` dialogs, wired auto-loading of local `playlist.txt` / directory scans and device detection on startup, embedded app logo for window favicon and header bar, configured `300x400px` minimum window size, converted header, toolbar, audio bar, and export panel to `horizontal_wrapped` responsive stacking, tightened global typography and padding, and styled footer attribution.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `Cargo.toml` | modified | Added `image = { version = "0.25", default-features = false, features = ["png"] }` (already in lockfile) for PNG favicon decoding |
| `src/models/project.rs` | modified | Added `impl Default for ProjectFile` |
| `src/ui/app.rs` | modified | Added embedded logo PNG asset, `load_app_icon()`, `get_logo_texture()`, startup `auto_load_current_dir()`, `refresh_devices()`, `300x400px` viewport minimum size with icon, and compact typography / button padding styles |
| `src/ui/header.rs` | modified | Added logo rendering, replaced missing glyphs with `↩ Undo` / `↪ Redo`, switched to `horizontal_wrapped` |
| `src/ui/toolbar.rs` | modified | Switched to `horizontal_wrapped`, initialized `rfd::FileDialog` with current working directory |
| `src/ui/audio_bar.rs` | modified | Switched to `horizontal_wrapped` so controls and labels wrap without clipping |
| `src/ui/export_panel.rs` | modified | Switched to `horizontal_wrapped`, made `scan_devices()` public for startup detection |
| `src/ui/footer.rs` | modified | Formatted italic right-aligned attribution `Made by Ijash. Github: musicbox` with hyperlink |
| `tests/gui/playlist_tests.rs` | modified | Added test for `MusicBoxApp::new` startup device and directory discovery |
| `tests/gui/theme_tests.rs` | modified | Added test verifying rendering at compact `300x400px` viewport size |

## Tests Added or Updated

- `tests/gui/playlist_tests.rs::test_app_new_startup_and_device_detection` — Verifies automatic startup hydration and device detection on `MusicBoxApp::new`.
- `tests/gui/theme_tests.rs::test_compact_layout_and_supported_glyphs` — Verifies headless rendering and widget stability at compact `300x400px` viewport with wrapped rows.

## Local Verification

- Commands run:
  - `cargo test`: 67 passed, 0 failed across all unit, integration, and headless GUI tests.
  - `cargo clippy --all-targets -- -D warnings`: passed with 0 warnings.
  - `cargo fmt --check`: passed with 0 formatting differences.
  - `cargo build --release`: binary at `target/release/musicbox` size `8,152,496` bytes (`~7.8 MB`, ceiling `< 20 MB`).
- Manual checks: Verified `load_app_icon()` decodes RGBA from embedded PNG without external assets.

## Deviations from Assessment

None. All 9 items were implemented strictly according to the assessment proposal.

## Follow-ups

- Run `/speckit-bug-test slug=egui-ui-parity-responsive` to record verification report.
