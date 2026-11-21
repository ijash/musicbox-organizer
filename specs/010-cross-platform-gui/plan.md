# Implementation Plan: Unified Cross-Platform Rust GUI (Feature 010)

**Branch**: `010-cross-platform-gui` | **Date**: 2026-11-22 | **Spec**: [specs/010-cross-platform-gui/spec.md](spec.md)

**Input**: Feature specification from `specs/010-cross-platform-gui/spec.md`

## Summary

Migrate MusicBox Organizer from the Linux-only GTK3 frontend to a unified pure-Rust cross-platform GUI built with `eframe`/`egui` (`glow` renderer) and `rfd` native OS file dialogs for Linux, Windows, and macOS (`v0.3.0`). Upgrade crate to Rust `edition = "2024"`. Eliminate all GTK3, Cocoa, and Objective-C dependencies while preserving 100% of underlying domain services (`src/services/`) and models (`src/models/`). Enforce Constitution Principle I (< 20 MB standalone release binary), add Windows silent double-click launch with conditional CLI console attachment (`AttachConsole`), implement Windows removable drive detection, and replace the Linux-only `xvfb-run` test harness with cross-platform headless `egui_kittest` tests under standard `cargo test` across all 3 operating systems.

## Technical Context

**Language/Version**: Rust 1.85+ (Edition 2024)

**Primary Dependencies**:
- GUI: `eframe = { version = "0.31", default-features = false, features = ["default_fonts", "glow"] }`, `egui = "0.31"`, `egui_extras = "0.31"`
- File Dialogs: `rfd = { version = "0.15", default-features = false, features = ["xdg-portal"] }` (zero GTK3 dependency on Linux)
- Windows Platform: `windows = { version = "0.51", features = ["Win32_Foundation", "Win32_UI_WindowsAndMessaging", "Win32_System_Memory", "Win32_System_Console", "Win32_Storage_FileSystem"] }`
- Audio/DSP Core (reused): `rodio = "0.19"`, `id3 = "1.14"`, `ebur128 = "0.1"`, `mp3lame-encoder = "0.1"`, `metaflac = "0.2"`
- Concurrency: `std::thread`, `std::sync::mpsc` (zero `tokio` per Constitution Principle I)
- Testing: `egui_kittest = { version = "0.31", default-features = false, features = ["winit"] }` (headless GUI testing under `cargo test`)

**Storage**: Local filesystem (MP3 files, `playlist.txt`, removable USB/SD media volumes)

**Testing**: Standard `cargo test` (unit tests, service integration tests, and headless `egui_kittest` GUI tests on all platforms)

**Target Platform**: Linux (`x86_64`), Windows 10/11 (`x86_64`), macOS (`arm64`)

**Project Type**: Standalone desktop GUI & CLI application

**Performance Goals**:
- GUI initialization in < 500 ms
- Smooth 60 FPS rendering during background DSP and file export operations
- Standalone release binary size strictly `< 20 MB` across all target OSes

**Constraints**:
- Zero GTK3, Cocoa, Web technologies, or external runtime dependencies
- No `tokio` or heavy async runtimes (threads + channels only)
- Windows double-click must never flash an empty console window

**Scale/Scope**:
- Full rewrite of `src/ui/` into modular immediate-mode components
- Update `src/main.rs` entry point and Windows subsystem handling
- Implementation of Windows removable drive detection in `src/platform/windows.rs`
- CI workflow update in `.github/workflows/release.yml`

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I: Lightweight First**: PASS. Uses `eframe` with `glow` (no heavy `wgpu` shaders, no `tokio`, no Electron/WebView). Release profile enforces `lto = true`, `codegen-units = 1`, `strip = true` ensuring binary size stays well under `< 20 MB`.
- **Principle II: Cross-Platform Native**: PASS. Replaces OS-divergent GUI toolkits with a single unified pure-Rust `eframe`/`egui` + `rfd` frontend that compiles natively for Linux, Windows, and macOS.
- **Principle III: User-Centric Simplicity**: PASS. Delivers inline metadata editing, drag/button reordering, single-click undo/redo, audio preview playback, and dark/light theme toggle.
- **Principle IV: Offline-First & Portable**: PASS. Pure standalone executable with zero network calls, zero temp file leaks, and zero runtime installation prerequisites.
- **Principle V: Testable Core Logic & GUI Interaction Verification**: PASS. Headless GUI interaction tests with `egui_kittest` run via `cargo test` on all 3 OSes without `xvfb-run` or X11/Wayland servers.
- **Principle VI: Git-First Workflow & Conventional Commits**: PASS. Follows atomic Conventional Commits with Co-authored-by trailers.
- **Principle VII: Rapid Development & Continuous Rebuild**: PASS. Fast rebuild times with `glow` backend; binary rebuilt and verified after implementation.
- **Principle VIII: DRY & KISS**: PASS. Reuses 100% of domain services and models; UI directly models state without complex reactive abstractions.
- **Principle IX: Device Compatibility & Lexicographical Sorting**: PASS. Maintains strict `[XXYY-filename.mp3]` byte-order export sequencing.

## Project Structure

### Documentation (this feature)

```text
specs/010-cross-platform-gui/
├── plan.md              # Implementation plan (this file)
├── research.md          # Technical research and architecture decisions
├── data-model.md        # Domain and UI state models and message flows
├── quickstart.md        # Local validation guide and smoke testing procedures
├── contracts/
│   └── gui-components-and-cli-contract.md # Entry point, UI modularization, and platform contracts
└── checklists/
    └── requirements.md  # Specification quality checklist
```

### Source Code (repository root)

```text
Cargo.toml               # Edition 2024, eframe/glow, rfd/xdg-portal, egui_kittest, windows features
src/
├── main.rs              # Entry point with Windows subsystem & AttachConsole interop
├── lib.rs               # Library root re-exporting services, models, and ui
├── models/              # (Unchanged) project, session_state, track_metadata, device
├── services/            # (Unchanged) audio_editing, audio_player, export_service, device_manager
├── platform/
│   ├── mod.rs           # Platform dispatch
│   ├── linux.rs         # (Unchanged) Linux udisks/mount detection
│   ├── windows.rs       # Real Win32 GetLogicalDrives/GetDriveTypeW removable drive detection
│   └── macos.rs         # Safe empty-device stub
└── ui/
    ├── mod.rs           # UI submodule exports & MusicBoxApp initialization
    ├── app.rs           # eframe::App trait implementation & frame orchestration
    ├── header.rs        # App title, track count, Undo/Redo buttons, Dark/Light toggle
    ├── toolbar.rs       # Scan Folder, Add MP3, Remove, Up, Down, Audio Tools, Export Text, Save
    ├── playlist_table.rs# Scrollable table with inline Title/Artist editing and selection
    ├── audio_bar.rs     # Audio preview controls (Play, Pause, Stop) & status
    ├── dsp_dialog.rs    # Modal dialog for volume normalization & silence trimming
    ├── export_panel.rs  # Removable device selector, reformat guard, export trigger
    └── footer.rs        # Status message line & clickable GitHub link

tests/
├── gui_tests.rs         # egui_kittest headless GUI interaction tests
├── unit_tests.rs        # Core unit tests
└── integration_tests.rs # Audio processing & export integration tests

.github/
└── workflows/
    └── release.yml      # CI workflow: removed libgtk-3-dev/xvfb; cargo test on 3 OSes; < 20 MB gate
```

**Structure Decision**: Clean modular split of `src/ui/` into single-responsibility immediate-mode components. Heavy GTK-specific modules (`src/ui/gtk_ui.rs`, `src/ui/export_dialog.rs`) are fully replaced by pure `eframe` submodules.

## Complexity Tracking

> No constitution violations detected. Standard immediate-mode architecture fully adheres to Principles I-IX.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
