---
description: "Task list for Feature 010: Unified Cross-Platform Rust GUI (v0.3.0)"
---

# Tasks: Unified Cross-Platform Rust GUI (Feature 010)

**Input**: Design documents from `specs/010-cross-platform-gui/` (`spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/gui-components-and-cli-contract.md`, `quickstart.md`)

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/gui-components-and-cli-contract.md`

**Tests**: Cross-platform headless GUI tests using `egui_kittest` under standard `cargo test` are mandatory per Constitution Principle V and feature requirements.

**Organization**: Tasks are grouped by user story (US1–US4) to enable independent implementation and testing of each story.

## Format: `- [ ] [TaskID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g. `[US1]`, `[US2]`, `[US3]`, `[US4]`)
- Exact file paths included in every task description

---

## Phase 1: Setup (Dependency & Build Migration)

**Purpose**: Cargo.toml dependency overhaul, edition upgrade, and build configuration

- [X] T001 Migrate `Cargo.toml` to `edition = "2024"`, replace `gtk`/`cocoa`/`objc` with `eframe` (features `default_fonts`, `glow`), `egui`, `egui_extras`, `rfd` (features `xdg-portal`), add `egui_kittest` under `[dev-dependencies]`, configure Windows platform features (`Win32_System_Console`, `Win32_Storage_FileSystem`), and configure release profile (`lto = true`, `codegen-units = 1`, `strip = true`) in `Cargo.toml`
- [X] T002 Run early `cargo check` to verify dependency resolution and identify compile breakages in `src/main.rs` and `src/ui/`

---

## Phase 2: Foundational (Entrypoint & Core UI Scaffold)

**Purpose**: Core application shell, Windows console interop, and platform driver stubs that MUST complete before user stories begin

**⚠️ CRITICAL**: Blocking prerequisite for all user stories

- [X] T003 Update Windows subsystem attribute `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` and implement conditional `AttachConsole(ATTACH_PARENT_PROCESS)` for CLI subcommands in `src/main.rs`
- [X] T004 [P] Implement Win32 removable drive enumeration (`GetLogicalDrives`, `GetDriveTypeW`, `GetDiskFreeSpaceExW`, `GetVolumeInformationW`) in `src/platform/windows.rs`
- [X] T005 [P] Update UI module declarations, remove legacy GTK references, and define public `MusicBoxApp` struct and messaging types in `src/ui/mod.rs` citing `data-model.md: MusicBoxApp.dsp_rx` and `data-model.md: MusicBoxApp.export_rx`
- [X] T006 Implement `eframe::App` trait shell and central layout orchestration in `src/ui/app.rs` citing `data-model.md: MusicBoxApp.project` and `data-model.md: MusicBoxApp.session`

**Checkpoint**: Foundation ready — user story implementation can now proceed in priority order

---

## Phase 3: User Story 1 - Cross-Platform Native Desktop Launch & Playlist Organization (Priority: P1) 🎯 MVP

**Goal**: Deliver a standalone native GUI window on Linux, Windows, and macOS with native file/folder scanning, scrollable playlist table, inline Title/Artist editing, Up/Down reordering, row deletion, and Undo/Redo.

**Independent Test**: Launch executable with no CLI arguments; scan a directory or add individual MP3 files via `rfd` dialogs; inline-edit metadata; reorder tracks; undo/redo actions.

### Tests for User Story 1

- [X] T007 [P] [US1] Create headless GUI test for playlist table display, track additions, and reordering in `tests/gui/playlist_tests.rs`

### Implementation for User Story 1

- [X] T008 [P] [US1] Implement header component with app branding, track count indicator, and undo/redo buttons in `src/ui/header.rs`
- [X] T009 [P] [US1] Implement compact toolbar with `Scan Folder...`, `Add MP3` (via `rfd`), `Remove`, `Up`, `Down`, `Save`, and `Open` using theme-aware Unicode glyphs in `src/ui/toolbar.rs`
- [X] T010 [US1] Implement scrollable playlist table with `#`, `Export Filename`, inline-editable `Title` and `Artist`, and `Source File` columns supporting multi-row selection in `src/ui/playlist_table.rs` citing `data-model.md: MusicBoxApp.selected_track_indices` and `data-model.md: MusicBoxApp.editing_cell`
- [X] T011 [US1] Integrate toolbar, header, and playlist table into `src/ui/app.rs` and verify `cargo check --lib`

**Checkpoint**: User Story 1 fully functional and testable independently as the minimal viable product (MVP)

---

## Phase 4: User Story 2 - Audio Preview & Non-Blocking Audio DSP Processing (Priority: P2)

**Goal**: Provide in-app audio preview playback controls and an asynchronous modal dialog for volume normalization and silence trimming without freezing the UI.

**Independent Test**: Select a track; click Play in the audio preview bar; open the Audio Tools dialog; adjust thresholds; click Apply; verify active playback stops and background thread processes audio with live progress reporting.

### Tests for User Story 2

- [X] T012 [P] [US2] Create headless GUI test for audio preview controls and DSP modal parameter toggles in `tests/gui/audio_dsp_tests.rs`

### Implementation for User Story 2

- [X] T013 [P] [US2] Implement audio preview dock with `Play`, `Pause`, `Stop`, and active track status label in `src/ui/audio_bar.rs` citing `data-model.md: MusicBoxApp.preview_player` and `data-model.md: MusicBoxApp.playing_track_idx`
- [X] T014 [US2] Implement Audio Tools modal dialog with normalize and silence trimming checkboxes, numeric inputs, and cancel/run buttons in `src/ui/dsp_dialog.rs` citing `data-model.md: DspDialogState.normalize_target_db` and `data-model.md: DspDialogState.trim_threshold_db`
- [X] T015 [US2] Wire DSP batch execution to background `std::thread::spawn` with `std::sync::mpsc` progress channel, stopping `preview_player` prior to file overwrite and calling `ctx.request_repaint()` in `src/ui/app.rs`

**Checkpoint**: User Stories 1 and 2 both functional and testable independently

---

## Phase 5: User Story 3 - Removable Storage Detection & Lexicographical Export (Priority: P3)

**Goal**: Detect connected removable USB/SD drives, provide a two-step confirmation guard for media reformatting, configure target subpaths, and export playlist tracks using strict `[XXYY-filename.mp3]` lexicographical ordering.

**Independent Test**: Connect/simulate a removable drive; click Scan in export panel; verify drive appears in dropdown; configure subpath; test 2-step reformat guard; execute export and verify progress and status reporting.

### Tests for User Story 3

- [X] T016 [P] [US3] Create headless GUI test for removable media export panel and two-step reformat guard in `tests/gui/export_panel_tests.rs`

### Implementation for User Story 3

- [X] T017 [US3] Implement export panel with device selector dropdown, `Scan`, `Subpath`, two-step confirmation dialog for `Reformat before export`, `Export Playlist to Media`, and `Eject Device` in `src/ui/export_panel.rs` citing `data-model.md: ExportPanelState.selected_device_id` and `data-model.md: ExportPanelState.show_reformat_confirm`
- [X] T018 [US3] Wire export operation to background `std::thread::spawn` with `ExportProgressMessage` channel and `ctx.request_repaint()` in `src/ui/app.rs`

**Checkpoint**: User Stories 1, 2, and 3 functional and testable independently

---

## Phase 6: User Story 4 - Theme Accessibility, Text/RTF Export & Headless CI Verification (Priority: P4)

**Goal**: Deliver accessible Dark/Light theme switching with high-contrast text and icons, playlist summary export (Text and RTF), consolidated headless `egui_kittest` tests under `cargo test`, and updated CI release workflow.

**Independent Test**: Toggle between Dark and Light visual themes; export tracklist to Text and RTF; execute `cargo test` without display servers; verify `.github/workflows/release.yml` builds and enforces the < 20 MB size gate.

### Tests for User Story 4

- [X] T019 [P] [US4] Create headless GUI test verifying Dark/Light theme switching and text color contrast in `tests/gui/theme_tests.rs`

### Implementation for User Story 4

- [X] T020 [P] [US4] Implement Dark/Light theme toggle wired to `egui::Visuals::dark()` and `egui::Visuals::light()` in `src/ui/header.rs` citing `data-model.md: MusicBoxApp.dark_mode`
- [X] T021 [P] [US4] Implement export text/RTF dialog triggering `RtfExporter` and TXT summary export in `src/ui/export_dialog.rs`
- [X] T022 [P] [US4] Implement footer status line and clickable GitHub project hyperlink in `src/ui/footer.rs`
- [X] T023 [US4] Replace GTK `xvfb-run` test runner with consolidated `egui_kittest` integration harness in `tests/gui_tests.rs`
- [X] T024 [US4] Update `.github/workflows/release.yml` to remove `libgtk-3-dev`/`xvfb`, run cross-platform `cargo test` across Linux, Windows, and macOS, and assert release binary size `< 20 MB`

**Checkpoint**: All user stories functional, tested, and automated in CI

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Cleanup of legacy artifacts, whole-project validation, binary size verification, and documentation updates

- [X] T025 [P] Clean up obsolete GTK files (`src/ui/gtk_ui.rs`, `tests/gui/run_gui_tests.sh`) and unused assets
- [X] T026 Run full validation suite per `quickstart.md`: `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo fmt --check`
- [X] T027 Build standalone release binary `cargo build --release` and verify binary size `< 20 MB` (`20,971,520` bytes) per Constitution Principle I & VII
- [X] T028 [P] Update `README.md` changelog for `v0.3.0` and cross-platform installation/usage instructions per Constitution Development Workflow

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Can start immediately. Overhauls dependencies and sets up edition 2024.
- **Foundational (Phase 2)**: Depends on Phase 1 completion. Blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Phase 2 completion. Forms MVP core.
- **User Story 2 (Phase 4)**: Depends on Phase 3 (extends `MusicBoxApp` with audio preview and DSP modal).
- **User Story 3 (Phase 5)**: Depends on Phase 3 (extends `MusicBoxApp` with export panel).
- **User Story 4 (Phase 6)**: Depends on Phases 3, 4, and 5 (unifies theme, exports, and CI).
- **Polish (Phase 7)**: Depends on all user stories completing.

### Parallel Opportunities

- Within Phase 2: T004 (Windows platform) and T005 (UI mod) can execute in parallel.
- Within Phase 3 (US1): T007 (test), T008 (header), and T009 (toolbar) can execute in parallel.
- Within Phase 4 (US2): T012 (test) and T013 (audio bar) can execute in parallel.
- Within Phase 5 (US3): T016 (test) and T017 (export panel) can execute in parallel.
- Within Phase 6 (US4): T019 (test), T020 (theme toggle), T021 (export text/rtF), and T022 (footer) can execute in parallel.
- Within Phase 7: T025 (cleanup) and T028 (README updates) can execute in parallel.

---

## Parallel Example: User Story 1

```bash
# Launch test and UI component tasks in parallel:
Task: "Create headless GUI test for playlist table display in tests/gui/playlist_tests.rs"
Task: "Implement header component with app branding in src/ui/header.rs"
Task: "Implement compact toolbar with theme-aware glyphs in src/ui/toolbar.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1 (Cargo.toml migration & check)
2. Complete Phase 2 (Foundational entrypoint & core app scaffold)
3. Complete Phase 3 (User Story 1 - Desktop Launch & Playlist Organization)
4. **STOP and VALIDATE**: Verify playlist creation, inline editing, and undo/redo independently.

### Incremental Delivery
1. Phase 1 + Phase 2 → Foundation established.
2. Phase 3 (US1) → Core desktop MVP functional.
3. Phase 4 (US2) → Audio preview and DSP enhancement added.
4. Phase 5 (US3) → Removable storage export added.
5. Phase 6 (US4) → Accessible themes, RTF/Text exports, and automated CI tests added.
6. Phase 7 → Final cleanup, release binary size gate verification, and README release notes.
