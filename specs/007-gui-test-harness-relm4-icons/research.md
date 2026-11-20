# Research: Automated GUI Test Harness and Relm4 Icons

**Feature Branch**: `007-gui-test-harness-relm4-icons`
**Date**: 2026-11-20

## 1. Automated GUI Testing with xvfb & GTK3 Main Loop

### Decision
Implement a reusable test harness in `tests/gui/` and an integration runner script `tests/gui/run_gui_tests.sh` that leverages `xvfb-run -a cargo test --test gui_tests`.

### Rationale
- Enforces Constitution Principle V (Testable Core Logic & GUI Interaction Verification).
- Uses `gtk::main_iteration_do(false)` and `glib::timeout_add_local` or simulated button clicks (`button.clicked()`) to programmatically drive:
  - Adding tracks
  - Moving up / moving down with persistent selection
  - Inline title and artist editing in the `ListStore`
  - Removing selected tracks
  - Audio play, pause, and stop controls
  - Dark/light mode theme toggling
  - Opening the export dialog and verifying format selections
- Runs entirely headless in CI or local terminal without requiring an open X11 window.

---

## 2. Relm4 Standardized Icons Integration

### Decision
Add `relm4-icons` with custom feature flags for only in-use symbolic icons (e.g., `relm4-icons = { version = "0.7", default-features = false }`) or embed the selected official Relm4 SVG vector icons into `src/ui/icons.rs`.

### Rationale
- Standardized, theme-aware symbolic icons across the entire desktop UI:
  - Folder Scan: `folder-symbolic`
  - Add MP3: `list-add-symbolic`
  - Remove MP3: `edit-delete-symbolic` / `list-remove-symbolic`
  - Move Up: `go-up-symbolic`
  - Move Down: `go-down-symbolic`
  - Audio Tools: `audio-volume-high-symbolic`
  - Export Text: `document-save-symbolic`
  - Save: `document-save-symbolic`
  - Open: `document-open-symbolic`
  - Play: `media-playback-start-symbolic`
  - Pause: `media-playback-pause-symbolic`
  - Stop: `media-playback-stop-symbolic`
  - Theme Toggle: `weather-clear-night-symbolic`
- Compiles *only* the icons used into the binary, preserving the strict <2 MB binary size ceiling per Constitution Principle I.
