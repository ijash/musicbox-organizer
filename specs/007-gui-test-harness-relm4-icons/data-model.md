# Data Model: GUI Test Harness & Icon Set

**Feature Branch**: `007-gui-test-harness-relm4-icons`
**Date**: 2026-11-20

## Entities

### 1. GuiTestSession
Context for driving simulated GTK interactions in headless mode.

| Field | Type | Description |
|---|---|---|
| `window` | `gtk::Window` | Main app window instance |
| `project` | `Rc<RefCell<ProjectFile>>` | Working project state |
| `session_state` | `Rc<RefCell<SessionState>>` | Undo/redo stack state |
| `event_pump_cycles` | `usize` | Iterations pumped through GTK main loop |

### 2. AppIconIdentifier
Enumeration of all in-use symbolic icons in the application.

| Icon Identifier | Purpose | Relm4 SVG Source |
|---|---|---|
| `FolderOpen` | Scan directory | `folder-symbolic` |
| `ListAdd` | Add audio file | `list-add-symbolic` |
| `ListRemove` | Remove selected track | `edit-delete-symbolic` |
| `GoUp` | Move row up | `go-up-symbolic` |
| `GoDown` | Move row down | `go-down-symbolic` |
| `Play` | Start audio preview | `media-playback-start-symbolic` |
| `Pause` | Pause audio preview | `media-playback-pause-symbolic` |
| `Stop` | Stop audio preview | `media-playback-stop-symbolic` |
| `Save` | Save project | `document-save-symbolic` |
| `Open` | Open project | `document-open-symbolic` |
| `ThemeDark` | Dark mode toggle | `weather-clear-night-symbolic` |
| `ThemeLight` | Light mode toggle | `weather-clear-symbolic` |
