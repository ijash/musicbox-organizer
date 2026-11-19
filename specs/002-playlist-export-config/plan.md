# Implementation Plan: Playlist Export Config & Theme Persistence

**Branch**: `002-playlist-export-config` | **Date**: 2026-11-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/002-playlist-export-config/spec.md`

## Summary
Implement dark mode default with persistent preferences in `.musicbox_config.json`, multi-format export to `playlist-text/` folder with clear-all overwrite behavior, format selection checkboxes, and architectural placeholder for future audio DSP editing.

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.75+)
**Primary Dependencies**: `serde`, `serde_json`, `gtk = "0.17"`
**Storage**: `.musicbox_config.json` + `playlist-text/` folder
**Testing**: `cargo test` unit and integration tests
**Target Platform**: Linux, Windows, macOS
**Project Type**: Desktop app
**Performance Goals**: Export completes <100ms; preference load <10ms
**Constraints**: Zero temporary files left behind; offline-first; pure Rust

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Lightweight First)**: PASS. Uses existing `serde_json` and built-in formats.
- **Principle III (User-Centric Simplicity)**: PASS. Clear format selection modal, dark mode default.
- **Principle IV (Offline-First & Portable)**: PASS. Transparent local config file.
- **Principle VI (Git-First Workflow & Conventional Commits)**: PASS. Conventional commits.
- **Principle VII (Rapid Development & Continuous Rebuild)**: PASS. Rebuild binary on completion.

## Project Structure

### Documentation

```text
specs/002-playlist-export-config/
├── plan.md              # This file
├── research.md          # Preferences and clear-all research
├── data-model.md        # UserPreferences, ExportFormatSelection
├── quickstart.md        # Test scenarios
├── contracts/           # PlaylistTextExporter contract
└── tasks.md             # Generated tasks
```

### Source Code

```text
src/
├── models/
│   └── preferences.rs      # UserPreferences load/save
├── services/
│   ├── export_config.rs    # Multi-format clear-all exporter
│   └── audio_editing.rs    # AudioProcessor placeholder trait
└── ui/
    └── gtk_ui.rs           # Connect theme persistence & export dialog

tests/
├── unit/
│   └── export_config_tests.rs
```
**Structure Decision**: Single project architecture. Add preferences model, export config service, audio editing placeholder, and connect to GTK UI.

## Complexity Tracking
*No violations. All design choices strictly align with Constitution.*
