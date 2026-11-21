---
name: egui-gui-testing
description: Run and write fast, headless cross-platform GUI and service tests for musicbox-organizer using egui_kittest and cargo test.
---

# MusicBox Organizer Testing Guide (`egui_kittest` + Rust)

## Rules
- Never modify original files in `mp3-sample/`; always copy fixtures into `tempfile::tempdir()`.
- Prefer `egui_kittest::Harness` (AccessKit widget queries + state assertions) over brittle pixel snapshot PNGs so tests run fast and deterministically on Linux, Windows, and macOS without `xvfb-run`.
- Stop `AudioPreviewPlayer` in tests/handlers before overwriting audio files.

## Test Coverage Checklist
1. **Services (`cargo test --lib --test unit_tests --test integration_tests`)**: naming `[AAAA-]`, ID3 preservation, `NativeAudioProcessor` (trim + LUFS normalize + atomic write with zero temp leftovers), export & compatibility.
2. **Headless GUI (`egui_kittest`)**:
   - Build `Harness::new_ui` / `Harness::new_state` around the root app state.
   - Query widgets by label/role (`harness.get_by_label(...)`), simulate `.click()`, call `harness.run()` / `harness.step()`.
   - Verify: playlist add/reorder/undo/redo, Dark/Light `Visuals` toggle, Audio Bar Play/Pause/Stop state, and DSP modal dialog open/run/cancel.
3. **Final Gates**:
   - `cargo test` passes 100% with zero warnings.
   - `cargo build --release` binary size stays `< 20 MB`.