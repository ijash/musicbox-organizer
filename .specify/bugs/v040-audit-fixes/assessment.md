# Bug Assessment: v0.4.0 Audit Fixes

- **Slug**: v040-audit-fixes
- **Created**: 2026-11-23
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> Follow Constitution v1.9.0 (create branch `fix/v040-audit-fixes` off `fix/v040-qa-fixes` when fixing, do NOT auto-merge) and skills `rust-gui` + `egui-gui-testing`. Triage and propose remediation for these 5 verified bugs in the current codebase:
>
> 1. **`src/ui/dsp_dialog.rs` (Dead Cancel button + trapped window + double MP3 re-encode)**:
>    - Line 122 (`app.dsp_dialog.is_open = is_open`) overwrites `app.dsp_dialog.is_open = false` when `Cancel` is clicked, making `Cancel` completely non-functional.
>    - Migrate `Audio Tools - Batch DSP` from embedded `egui::Window` to a centered, compact, non-resizable OS child viewport (`ctx.show_viewport_immediate`), consistent with `Export Catalog` and `Confirm Export`.
>    - In `start_dsp_processing` (lines 200–212), replace separate `trim_silence()` + `normalize_volume()` calls with a single-pass `processor.process_track(&path, trim_enabled, trim_threshold, normalize_enabled, normalize_target)` to avoid double MP3 generation loss.
> 2. **`src/ui/audio_bar.rs:26–32` (Cannot switch tracks directly via `▶ Play`)**:
>    - Allow `▶ Play` whenever `target_idx.is_some() && (!is_playing || is_paused || target_idx != app.playing_track_idx)` so users can select another track and click Play directly without clicking Stop first.
> 3. **`src/ui/toolbar.rs:108–175` (`playing_track_idx` desync on Remove / Up / Down)**:
>    - On `remove_selected_tracks`: stop `preview_player` and clear `playing_track_idx` if the playing track is deleted, or remap its index if tracks above it are deleted.
>    - On `move_selected_up` / `move_selected_down`: keep `playing_track_idx` tracking the moved audio file accurately.
> 4. **`src/platform/windows.rs:120–130` (Silent fake `Ok(())` for Windows format/eject)**:
>    - Return honest errors (`Err("Device formatting is not yet supported on Windows".into())` and `Err("Safe eject is not yet supported on Windows".into())`) instead of silent `Ok(())`.
> 5. **`src/ui/export_catalog_dialog.rs:93–115` (Dynamic child viewport resize on format switch)**:
>    - Send `ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(dialog_size))` when `dialog_size` changes while the dialog is open, so switching between PDF (`485px`) and Plain Text/Rich Text (`285px`) resizes the open window cleanly with zero dead space.

## Symptom

1. **DSP Dialog**: Clicking `Cancel` does not close the dialog because `app.dsp_dialog.is_open` is overwritten with `is_open` immediately after closure exit. The modal is also trapped inside the primary viewport bounds. Running batch DSP executes two sequential passes (`trim_silence` then `normalize_volume`), causing double lossy MP3 re-encoding.
2. **Audio Bar Track Switching**: When a track is playing and the user selects a different track in the playlist table, the `▶ Play` button remains disabled. The user is forced to click `⏹ Stop` before playing the new track.
3. **Playing Track State Desync**: Removing tracks from the playlist leaves `app.playing_track_idx` pointing to an invalid or shifted index without stopping playback if deleted. Moving tracks up or down shifts playlist positions without updating `app.playing_track_idx`, causing the audio bar to display or control the wrong track.
4. **Windows Device Operations**: `format_device` and `eject_device` in `src/platform/windows.rs` return `Ok(())` unconditionally without performing any operation, misleading calling services and users into believing formatting/ejection succeeded.
5. **Export Catalog Child Window Sizing**: Switching between PDF (`485px`) and Plain Text/Rich Text (`285px`) leaves empty space or clipped content in the child viewport because `ViewportBuilder::with_inner_size` is only applied upon initial viewport creation.

## Reproduction

1. **DSP Dialog**:
   - Open `Audio Tools - Batch DSP` dialog. Click `Cancel`. Observe that the dialog remains open.
   - Run DSP with both Normalize and Trim enabled on an MP3 track. Observe two separate file writes and audio re-encodes.
2. **Audio Bar Play Switching**:
   - Add two tracks to playlist. Click `▶ Play` on track 1. While track 1 is playing, click track 2 in playlist table. Observe `▶ Play` button is disabled.
3. **Playing Track Desync**:
   - Play track 1 (`playing_track_idx = Some(1)`). Select track 0 and click `Remove`. Observe track 1 moves to index 0, but `playing_track_idx` remains `Some(1)`.
   - Play track 1. Select track 1 and click `Move Up`. Observe track moves to index 0, but `playing_track_idx` remains `Some(1)`.
4. **Windows Format / Eject**:
   - Invoke `format_device` or `eject_device` on `WindowsDeviceManager`. Observe immediate `Ok(())` return despite no action taken.
5. **Export Catalog Sizing**:
   - Open Export Catalog dialog in PDF mode (`485px` height). Switch format dropdown to Plain Text (`285px`). Observe that the OS window retains `485px` height with large empty whitespace.

## Suspected Code Paths

- `src/ui/dsp_dialog.rs:25–130` — Embedded `egui::Window` usage, and line 122 `app.dsp_dialog.is_open = is_open;` overriding the Cancel button click.
- `src/ui/dsp_dialog.rs:200–215` — `start_dsp_processing` calls `processor.trim_silence()` and then `processor.normalize_volume()`, triggering two decode/encode cycles.
- `src/ui/audio_bar.rs:26–34` — `play_btn_enabled` condition ignores `target_idx != app.playing_track_idx`.
- `src/ui/toolbar.rs:108–180` — `remove_selected_tracks`, `move_selected_up`, and `move_selected_down` mutate `app.project.tracks` without reconciling `app.playing_track_idx` or stopping playback.
- `src/platform/windows.rs:120–132` — `format_device` and `eject_device` return `Ok(())` stub values.
- `src/ui/export_catalog_dialog.rs:93–120` — Child viewport creation does not issue `egui::ViewportCommand::InnerSize(dialog_size)` during subsequent frames.

## Root Cause Hypothesis

High confidence.
1. `src/ui/dsp_dialog.rs` captures `is_open = app.dsp_dialog.is_open` before the window closure and unconditionally assigns `app.dsp_dialog.is_open = is_open` after the closure, overwriting the Cancel handler. The DSP worker loop sequentially invokes `trim_silence` and `normalize_volume` instead of the single-pass `process_track` method in `NativeAudioProcessor`.
2. `src/ui/audio_bar.rs` requires `!is_playing || is_paused` for `play_btn_enabled`, not accounting for the case where the user has selected a different track (`target_idx != app.playing_track_idx`).
3. `src/ui/toolbar.rs` operates directly on `app.project.tracks` and `app.selected_track_indices`, failing to update `app.playing_track_idx` or halt `preview_player` when tracks are removed or reordered.
4. `src/platform/windows.rs` implements `DeviceManager` with placeholder stubs returning `Ok(())` rather than honest error messages.
5. `src/ui/export_catalog_dialog.rs` computes dynamic `dialog_size` but relies solely on `ViewportBuilder::with_inner_size(dialog_size)` which only applies on window creation, omitting `ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(dialog_size))`.

## Proposed Remediation

**Preferred**:

1. **DSP Dialog Viewport, Cancel Fix & Single-Pass DSP**:
   - Migrate `render_dsp_dialog` to use `ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("dsp_dialog_viewport"), ...)` with centered pos and non-resizable bounds, matching `export_catalog_dialog.rs`.
   - Separate dialog body into `render_dsp_dialog_contents(app, ctx)` for testability under `egui_kittest`.
   - On `Cancel` button click, set `app.dsp_dialog.is_open = false` and close viewport with `ctx.send_viewport_cmd(egui::ViewportCommand::Close)`.
   - In `start_dsp_processing`, replace separate calls to `trim_silence` and `normalize_volume` with `processor.process_track(&path, trim_enabled, trim_threshold, normalize_enabled, normalize_target)`.
2. **Direct Track Switching in Audio Bar**:
   - In `src/ui/audio_bar.rs`, update `play_btn_enabled`:
     ```rust
     let is_different_track = target_idx != app.playing_track_idx;
     let play_btn_enabled = target_idx.is_some() && (!is_playing || is_paused || is_different_track);
     ```
3. **Synchronize `playing_track_idx` in Toolbar Operations**:
   - In `remove_selected_tracks`:
     - If `playing_track_idx` is in `selected_track_indices`: stop `preview_player` and set `playing_track_idx = None`.
     - Else if `Some(p)`: subtract the count of removed tracks where `idx < p`.
   - In `move_selected_up` and `move_selected_down`:
     - Track the playing track's `file_path` before moving, and find its new position in `app.project.tracks` after swapping and reindexing.
4. **Honest Errors on Windows Format & Eject**:
   - In `src/platform/windows.rs`, change `format_device` to return `Err("Device formatting is not yet supported on Windows".into())`.
   - Change `eject_device` to return `Err("Safe eject is not yet supported on Windows".into())`.
5. **Dynamic Child Viewport Resizing in Export Catalog Dialog**:
   - In `src/ui/export_catalog_dialog.rs`, inside the viewport closure:
     ```rust
     ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(dialog_size));
     ```
     This forces the OS child window to resize smoothly when switching between PDF (`485px`) and Plain Text / Rich Text (`285px`).

**Files likely to change**:
- `src/ui/dsp_dialog.rs`
- `src/ui/audio_bar.rs`
- `src/ui/toolbar.rs`
- `src/platform/windows.rs`
- `src/ui/export_catalog_dialog.rs`
- `src/ui/mod.rs`
- `tests/gui/audio_dsp_tests.rs`
- `tests/gui/playlist_tests.rs`
- `tests/gui/export_catalog_dialog_test.rs`
- `tests/unit/device_tests.rs`

**Tests to add or update**:
- `test_dsp_dialog_cancel_button`: Verify clicking Cancel sets `is_open = false`.
- `test_audio_bar_direct_switch_track`: Verify `play_btn_enabled` is true when selecting a different track while playing.
- `test_toolbar_track_removal_stops_or_shifts_playback`: Verify removing the playing track stops player and resets index; removing preceding track decrements index.
- `test_toolbar_track_move_updates_playing_index`: Verify moving tracks up/down preserves accurate `playing_track_idx`.
- `test_windows_format_eject_returns_unsupported_error`: Verify Windows stubs return explicit descriptive errors.
- `test_export_catalog_viewport_resize_command`: Verify `ViewportCommand::InnerSize` is emitted with the appropriate dimensions on format switch.

## Risks & Considerations

- **Child Viewport Platform Differences**: On platforms or windowing backends where multiple viewports are unsupported or disabled, `show_viewport_immediate` falls back to inline rendering. Maintaining `render_dsp_dialog_contents` ensures compatibility with headless `egui_kittest`.
- **Atomic File Writing in DSP**: Single-pass `process_track` writes an atomic temporary file before replacing the target MP3; verify that permissions and ID3 metadata are retained.
- **Undo Integration**: Toolbar operations push an undo state before mutating tracks; ensuring `playing_track_idx` updates cleanly does not corrupt undo history.

## Open Questions

None. All 5 bugs are verified in code with exact reproduction paths and clear remediations.
