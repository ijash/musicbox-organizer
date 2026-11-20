# Implementation Plan: UI Branding & GitHub Link Integration

**Branch**: `005-ui-branding-github-link` | **Date**: 2026-11-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/005-ui-branding-github-link/spec.md`

## Summary
Add application window favicon icon, render compact branding logo in the top-left header bar next to title, and provide a clickable `GitHub ↗` button in the top-right toolbar opening the repository in the system browser.

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.75+)
**Primary Dependencies**: `gtk = "0.17"` (Linux native UI)
**Storage**: Static image assets in `img/logo/`
**Testing**: Manual GUI validation + `cargo test` suite
**Target Platform**: Linux (GTK3), Windows, macOS
**Project Type**: Desktop Application
**Performance Goals**: <10ms additional startup time; zero memory leak
**Constraints**: Pure native GTK3 methods (`set_icon_from_file`, `show_uri_on_window`); graceful fallback if assets missing

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Lightweight First)**: PASS. Zero new crates; uses existing GTK3 and image files.
- **Principle II (Cross-Platform Native)**: PASS. Uses native GTK3 desktop integration.
- **Principle III (User-Centric Simplicity)**: PASS. Intuitive branding and discoverable repository link.
- **Principle VI (Git-First Workflow & Conventional Commits)**: PASS. Conventional commits with trailer.
- **Principle VII (Rapid Development & Continuous Rebuild)**: PASS. Mandatory release rebuild on completion.

## Project Structure

### Documentation

```text
specs/005-ui-branding-github-link/
├── plan.md              # This file
├── research.md          # Icon loading & URI handler analysis
├── data-model.md        # BrandingConfig & interaction model
├── quickstart.md        # GUI verification guide
├── contracts/           # Header branding contract
└── tasks.md             # Tasks
```

### Source Code

```text
src/
└── ui/
    └── gtk_ui.rs        # Window icon, header logo, and GitHub button
```

**Structure Decision**: Single project architecture. Minimal localized modification in `src/ui/gtk_ui.rs`.

## Complexity Tracking
*No violations. All design choices strictly align with Constitution.*

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
