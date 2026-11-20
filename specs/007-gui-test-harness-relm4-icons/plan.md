# Implementation Plan: Automated GUI Test Harness and Relm4 Icons (Feature 007)

**Branch**: `007-gui-test-harness-relm4-icons` | **Date**: 2026-11-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/007-gui-test-harness-relm4-icons/spec.md`

## Summary
Provide automated, headless GUI integration testing using `xvfb-run` that exercises real GTK event loop clicks and row manipulations per Constitution Principle V. Integrate standardized vector icons from `relm4-icons` for in-use toolbar actions, embedded directly into the binary to maintain clean appearance and lightweight binary size (<2.5 MB).

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.75+)
**Primary Dependencies**: `gtk = "0.17"`, `relm4-icons = "0.7"` (or embedded in-use Relm4 SVGs)
**Storage**: N/A (UI and test harness)
**Testing**: `tests/gui/run_gui_tests.sh` with `xvfb-run`, plus `cargo test`
**Target Platform**: Linux (GTK3 desktop & headless CI)
**Project Type**: Desktop UI Automation & Vector Icon Standardization
**Performance Goals**: GUI test suite runs in <10s; binary size ceiling <2.5 MB
**Constraints**: Embed only in-use icons (zero bloat per Constitution Principle I)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Lightweight First)**: PASS. Only in-use icons embedded; binary stays <2.5 MB.
- **Principle V (Testable Core Logic & GUI Interaction Verification)**: PASS. Directly implements mandatory headless GUI verification and automation harness.
- **Principle VII (Rapid Development & Continuous Rebuild)**: PASS. Rebuild binary on completion.
- **Principle VIII (DRY & KISS)**: PASS. Centralized icon mapping and reusable GUI harness.

## Project Structure

### Documentation

```text
specs/007-gui-test-harness-relm4-icons/
├── plan.md              # This file
├── research.md          # Headless GTK test harness and icon embedding research
├── data-model.md        # GuiTestSession and in-use icon inventory
├── quickstart.md        # Headless testing validation guide
├── contracts/           # GuiHarness and AppIcon contracts
└── tasks.md             # Tasks
```

### Source Code

```text
src/ui/
├── icons.rs             # In-use Relm4 symbolic vector icon loader
└── components/
    ├── header.rs        # Standardized theme icon
    ├── playlist_table.rs# Standardized toolbar icons (Scan, Add, Remove, Up, Down, Save, Open)
    └── audio_bar.rs     # Standardized playback icons (Play, Pause, Stop)

tests/
├── gui/
│   ├── mod.rs
│   ├── harness.rs       # Reusable programmatic GTK test driver
│   └── run_gui_tests.sh # xvfb-run headless wrapper script
└── gui_tests.rs         # Integration test target exercising GUI flows
```

**Structure Decision**: Single project architecture. Add `src/ui/icons.rs`, wire components, and create `tests/gui/` harness.

## Complexity Tracking
*No violations. All design choices strictly align with Constitution.*
