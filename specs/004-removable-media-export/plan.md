# Implementation Plan: Removable Media Export

**Branch**: `004-removable-media-export` | **Date**: 2026-11-19 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/004-removable-media-export/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command; its definition describes the execution workflow.

## Summary

Enable users to export organized MP3 playlists to connected USB/SD devices with automatic renaming ([XXYY-filename.mp3] lexicographic sort convention), optional reformat-before-export, and real-time progress tracking. Support FAT32/exFAT filesystems in v1. Device detection live-polls for disconnects (5s tolerance). Project metadata saved as plain-text .txt files locally; export files to removable media independent of project saves.

## Technical Context

**Language/Version**: Rust 1.75+ (core cross-platform logic)

**Primary Dependencies**: NEEDS CLARIFICATION (USB/SD enumeration library: `udisks2` on Linux, `iokit`/`DADiskArbitration` on macOS, `WinAPI` on Windows; formatting: platform-native tools or pure Rust impl)

**Storage**: Filesystem (local project files as .txt; MP3 files + playlist metadata on target device)

**Testing**: `cargo test` (unit tests for file renaming, filesystem ops)

**Target Platform**: Cross-platform desktop (Windows, macOS, Linux)

**Project Type**: desktop-app

**Performance Goals**: Device detection <5s (poll + timeout), progress updates 500ms intervals, export throughput limited by device I/O

**Constraints**: <50MB binary (Lightweight First), offline-capable, no network calls, platform-native UI (Win32/Cocoa/GTK3+)

**Scale/Scope**: Single user, 1-100 tracks per playlist, devices up to 2TB (FAT32/exFAT limits)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Lightweight First | ✓ | USB/SD export is minimal feature; native APIs only, no bloat |
| II. Cross-Platform Native | ✓ | Requires platform-native device enumeration (Win32/iokit/udisks2) |
| III. User-Centric Simplicity | ✓ | Device selection dialog + Export button; clear Save vs Export UI distinction |
| IV. Offline-First & Portable | ✓ | No network calls; all local FS operations; .txt project files portable |
| V. Testable Core Logic | ✓ | Renaming logic (AAAA→ZZZZ sequence), file ops, reformat detection all unit-testable |
| VI. Git-First Workflow & Conventional Commits | ✓ | All commits follow v1.0.0 format; auto-commit on feature completion |
| VII. Rapid Development | ✓ | Straightforward feature scope; no blocking unknowns after clarifications |
| VIII. DRY & KISS | ⚠️ | Platform abstraction layer needed for device enumeration (3 native backends); justified by Principle II |
| IX. Device Compatibility & Lexicographic Sorting | ✓ | XXYY naming (AAAA, AAAB, ...) ensures byte-order ASCII sort on primitive devices |

**Gate Result**: PASS. Principle VIII complexity justified by native-first requirement.

## Project Structure

### Documentation (this feature)

```text
specs/004-removable-media-export/
├── plan.md              # This file
├── research.md          # Phase 0 output (device enumeration libraries, FAT32/exFAT formatting options)
├── data-model.md        # Phase 1 output (Device, ExportJob, ProjectFile entities)
├── quickstart.md        # Phase 1 output (manual validation scenarios: connect device, export, verify)
├── contracts/           # Phase 1 output (UI contract for device list, export action)
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code (repository root)

```text
src/
├── models/
│   ├── device.rs       # Device struct + platform-specific enums
│   ├── export_job.rs   # ExportJob tracking (progress, state)
│   └── project.rs      # ProjectFile (.txt format) parsing
├── services/
│   ├── device_manager.rs        # Platform-agnostic device enumeration facade
│   ├── platform/
│   │   ├── mod.rs               # Platform selector
│   │   ├── windows.rs           # Win32 device enumeration
│   │   ├── macos.rs             # iokit/DADiskArbitration
│   │   └── linux.rs             # udisks2 or /sys/block enumeration
│   ├── export_service.rs        # File copy + renaming logic
│   ├── format_service.rs        # FAT32/exFAT reformat (platform-native or lib)
│   └── naming.rs                # XXYY sequence generator (AAAA → ZZZZ)
├── ui/
│   ├── export_dialog.rs         # Device list widget, export action UI
│   └── warnings.rs              # Reformat warning dialog
└── lib.rs

tests/
├── unit/
│   ├── naming_tests.rs          # Test AAAA → AAAB → ABAA sequences
│   ├── export_service_tests.rs  # File copy, error handling (partial files)
│   └── device_tests.rs          # Device detection mock/stub
└── integration/
    └── export_integration_tests.rs  # End-to-end export workflow (requires test device/mount)
```

**Structure Decision**: Single monolithic project with platform-specific modules under `src/platform/`. Device enumeration abstracted behind `DeviceManager` trait to support platform-specific backends. Naming logic isolated in `src/services/naming.rs` for independent unit testing. UI split into export dialog and warning components per platform.
