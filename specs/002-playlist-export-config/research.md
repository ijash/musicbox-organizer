# Research: Playlist Export Config & Theme Persistence

**Feature Branch**: `002-playlist-export-config`
**Date**: 2026-11-20

## 1. Preferences Persistence

### Decision
Store user preferences (e.g. `dark_mode: bool`, `default_formats: Vec<ExportFormat>`) in `.musicbox_config.json` in the working directory (or user config directory) using `serde_json`.

### Rationale
- Pure Rust JSON serialization with already-installed `serde` and `serde_json`.
- Offline-first and transparent (user can inspect or delete).
- Fulfills FR-003 and FR-001 with zero external libraries.

### Alternatives Considered
- SQLite: Heavy overkill for 2 booleans.
- Platform registry / `dconf`: Platform-specific code complexity; `.json` is universal across Linux/macOS/Windows.

---

## 2. Multi-Format Clear-All Exporter

### Decision
Implement `ExportConfigService` managing destination `playlist-text/` folder.

### Rationale
- Cleans existing files in `playlist-text/` before writing selected formats (FR-008).
- Exports selected formats (`.txt` via `ProjectFile`, `.rtf` via `RtfExporter`).
- Disables confirm button if no checkboxes are active (resolving FR-007 clarification).

---

## 3. Audio Editing Framework Placeholder

### Decision
Define `AudioProcessor` trait in `src/services/audio_editing.rs` with stubbed error returns ("Coming soon in v2").

### Rationale
- Clean modular architectural seam per Principle VIII (DRY & KISS).
- UI displays a disabled "Audio DSP (Coming Soon)" item per FR-012/FR-013.
