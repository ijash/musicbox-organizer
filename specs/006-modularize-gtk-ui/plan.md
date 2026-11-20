# Implementation Plan: Modularize GTK UI Components (Tech Debt D3)

**Branch**: `006-modularize-gtk-ui` | **Date**: 2026-11-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/006-modularize-gtk-ui/spec.md`

## Summary
Decompose monolithic `src/ui/gtk_ui.rs` (>1,150 lines) into dedicated, single-responsibility UI submodules under `src/ui/components/` and `src/ui/theme.rs`. Preserve 100% exact appearance, geometry, state coordination, and all test behaviors.

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.75+)
**Primary Dependencies**: `gtk = "0.17"` (Linux native UI)
**Storage**: N/A (UI layer refactoring)
**Testing**: `cargo test` unit and integration test suite
**Target Platform**: Linux (GTK3 desktop)
**Project Type**: Desktop UI Architecture
**Performance Goals**: Zero startup latency regression; identical rendering speed
**Constraints**: Zero behavioral or visual regressions; files <300 lines each

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Lightweight First)**: PASS. Zero new dependencies; code cleanup only.
- **Principle II (Cross-Platform Native)**: PASS. Cleans platform GTK3 code.
- **Principle V (Testable Core Logic)**: PASS. Decoupled subcomponents increase testability.
- **Principle VII (Rapid Development & Continuous Rebuild)**: PASS. Rebuild binary on completion.
- **Principle VIII (DRY & KISS)**: PASS. Directly eliminates monolithic anti-pattern.

## Project Structure

### Documentation

```text
specs/006-modularize-gtk-ui/
├── plan.md              # This file
├── research.md          # Architecture & state sharing design
├── data-model.md        # UI component hierarchy
├── quickstart.md        # Validation scenarios
├── contracts/           # Component contracts
└── tasks.md             # Tasks
```

### Source Code

```text
src/ui/
├── mod.rs               # Exports UI modules
├── export_dialog.rs     # Controller logic
├── warnings.rs          # Warning dialog controller
├── theme.rs             # CSS stylesheets (LIGHT_CSS, DARK_CSS, apply_theme)
├── gtk_ui.rs            # Top-level window assembler (<200 lines)
└── components/
    ├── mod.rs           # Module declarations
    ├── header.rs        # App title, logo, undo/redo, theme toggle
    ├── playlist_table.rs# TreeView, inline editing, toolbar, reorder
    ├── audio_bar.rs     # Audio preview playback controls
    ├── export_panel.rs  # Removable media selection & export
    └── footer.rs        # Italic attribution and GitHub link
```

**Structure Decision**: Single project architecture. Refactors `src/ui/` into modular subcomponents under `src/ui/components/` and `src/ui/theme.rs`.

## Complexity Tracking
*No violations. All design choices strictly align with Constitution.*
