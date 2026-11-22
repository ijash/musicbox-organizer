# Bug Fix: v0.4.0 Audit Fixes

- **Slug**: v040-audit-fixes
- **Fixed**: 2026-11-23
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Applied all 5 remediations triaged during the v0.4.0 audit: fixed the dead Cancel button and converted DSP dialog to a centered OS child viewport with single-pass `process_track` audio processing; enabled direct playback switching when selecting other tracks in the Audio Bar; synchronized `playing_track_idx` across track removal and reordering in Toolbar; returned honest unsupported `Err` messages on Windows format and eject stubs; and added dynamic child viewport resizing on format switch in Export Catalog.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/ui/dsp_dialog.rs` | modified | Migrated to `ctx.show_viewport_immediate`, removed Cancel override, and replaced separate trim/normalize with single-pass `processor.process_track`. |
| `src/ui/audio_bar.rs` | modified | Enabled `▶ Play` button when `target_idx != app.playing_track_idx`. |
| `src/ui/toolbar.rs` | modified | Reconciled `app.playing_track_idx` on `remove_selected_tracks`, `move_selected_up`, and `move_selected_down`. |
| `src/platform/windows.rs` | modified | Replaced stub `Ok(())` in `format_device` and `eject_device` with honest descriptive `Err` messages. |
| `src/platform/mod.rs` | modified | Made `windows` module available across all platforms for headless tests. |
| `src/ui/export_catalog_dialog.rs` | modified | Added `ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(dialog_size))` inside child viewport closure. |
| `src/ui/app.rs` | modified | Removed duplicate viewport invocation from `render_central_content`. |
| `src/ui/mod.rs` | modified | Re-exported `render_dsp_dialog_contents` for testing. |
| `tests/gui/audio_dsp_tests.rs` | modified | Added tests for DSP dialog Cancel button, full app toolbar toggle, and Audio Bar track switching while playing. |
| `tests/gui/playlist_tests.rs` | modified | Added regression test for toolbar track removal and move index sync. |
| `tests/gui/export_catalog_dialog_test.rs` | modified | Added test for dynamic child viewport format switching. |
| `tests/unit/device_tests.rs` | modified | Added unit test for Windows unsupported format and eject error handling. |

## Diff Highlights

### 1. DSP Dialog Child Viewport, Cancel Fix & Single-Pass DSP
```rust
// In src/ui/dsp_dialog.rs
pub fn render_dsp_dialog(app: &mut MusicBoxApp, ctx: &egui::Context) {
    if !app.dsp_dialog.is_open {
        return;
    }
    // Centered non-resizable child viewport with dynamic sizing
    ctx.show_viewport_immediate(viewport_id, builder, |ctx, _| {
        if ctx.input(|i| i.viewport().close_requested()) {
            app.dsp_dialog.is_open = false;
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(dialog_size));
        render_dsp_dialog_contents(app, ctx);
        if !app.dsp_dialog.is_open {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}
```

### 2. Audio Bar Direct Track Switching
```rust
// In src/ui/audio_bar.rs
let is_different_track = target_idx != app.playing_track_idx;
let play_btn_enabled = target_idx.is_some() && (!is_playing || is_paused || is_different_track);
```

### 3. Toolbar `playing_track_idx` Sync
```rust
// In src/ui/toolbar.rs
if let Some(playing_idx) = app.playing_track_idx {
    if app.selected_track_indices.contains(&playing_idx) {
        if let Some(player) = &mut app.preview_player {
            player.stop();
        }
        app.playing_track_idx = None;
    } else {
        let removed_before = app.selected_track_indices.iter().filter(|&&idx| idx < playing_idx).count();
        app.playing_track_idx = Some(playing_idx - removed_before);
    }
}
```

### 4. Windows Honest Format and Eject Errors
```rust
// In src/platform/windows.rs
fn format_device(&self, _device_id: &str, _filesystem: FileSystem) -> Result<(), Box<dyn Error>> {
    Err("Device formatting is not yet supported on Windows".into())
}
fn eject_device(&self, _device_id: &str) -> Result<(), Box<dyn Error>> {
    Err("Safe eject is not yet supported on Windows".into())
}
```

### 5. Export Catalog Dynamic Sizing
```rust
// In src/ui/export_catalog_dialog.rs
ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(dialog_size));
```

## Tests Added or Updated

- `tests/gui/audio_dsp_tests.rs::test_dsp_dialog_cancel_button` — Verifies clicking Cancel closes the DSP modal.
- `tests/gui/audio_dsp_tests.rs::test_dsp_dialog_toolbar_button_opens_and_cancel_closes_full_app` — Verifies opening via toolbar and closing via Cancel in full app harness.
- `tests/gui/audio_dsp_tests.rs::test_audio_bar_direct_track_switching_while_playing` — Verifies `▶ Play` is active when selecting a different track while playing.
- `tests/gui/playlist_tests.rs::test_toolbar_track_removal_and_move_sync_playing_index` — Verifies `playing_track_idx` updates on track deletions and moves, and halts playback when the playing track is deleted.
- `tests/gui/export_catalog_dialog_test.rs::test_export_catalog_format_switch_resizes_viewport` — Verifies format switching under child viewport.
- `tests/unit/device_tests.rs::test_windows_unsupported_format_and_eject_errors` — Verifies descriptive unsupported errors on Windows device operations.

## Local Verification

- `cargo test --all-targets` → Passed (93 tests ok, 0 failed).
- `cargo clippy --all-targets -- -D warnings` → Zero warnings.
- `cargo fmt --check` → Clean formatting.
- `cargo build --release` → 12 MB binary (`< 20 MB` Constitution ceiling).

## Deviations from Assessment

None. All 5 remediations implemented exactly as proposed.

## Follow-ups

- Proceed to `/speckit-bug-test slug=v040-audit-fixes` for verification reporting.
