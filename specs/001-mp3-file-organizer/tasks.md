# Tasks: MP3 File Organizer & ID3 Editor

**Input**: Design documents from `/specs/001-mp3-file-organizer/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add required crates and project structure

- [X] T001 Add `id3 = "1.14"` and `rodio = "0.19"` to Cargo.toml
- [X] T002 [P] Verify `cargo check` and clean build with new audio dependencies

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data models and traits that all user stories depend on

- [X] T003 Create TrackMetadata and OrganizerTrack entities in `src/models/track_metadata.rs` (citing data-model.md TrackMetadata and OrganizerTrack)
- [X] T004 Create SessionState and UndoAction in `src/models/session_state.rs` (citing data-model.md SessionState)
- [X] T005 Export new models in `src/models/mod.rs` and `src/lib.rs`

**Checkpoint**: Foundation ready - models compile and are available to services.

---

## Phase 3: User Story 1 - Load and Display MP3s (Priority: P1) 🎯 MVP

**Goal**: Non-recursively scan directory for `.mp3` files and load them into playlist.

**Independent Test**: Load `mp3-sample/` folder; verify only `sample-3s.mp3` and `sample-5s.mp3` are listed.

- [X] T006 [P] [US1] Unit test for AudioScanner in `tests/unit/scanner_tests.rs`
- [X] T007 [US1] Implement DirectoryScanner in `src/services/scanner.rs` per contracts/id3-scanner-contract.md
- [X] T008 [US1] Integrate directory scanning with project tracks in `src/models/project.rs`

---

## Phase 4: User Story 2 - Edit ID3 Tags and Rename Files (Priority: P1)

**Goal**: Read and write ID3 tags (Title, Artist, Album, Track number) and apply `[XXYY-filename.mp3]` renaming.

**Independent Test**: Read ID3 from `mp3-sample/sample-3s.mp3`, edit title, write back to file, and verify ID3 tags update.

- [X] T009 [P] [US2] Unit test for ID3 read/write in `tests/unit/id3_tests.rs`
- [X] T010 [US2] Implement Id3Service in `src/services/id3_service.rs` per TagEditor contract
- [X] T011 [US2] Implement batch rename and tag commit logic on save in `src/services/id3_service.rs`

---

## Phase 5: User Story 3 - Reordering and Group Sorting (Priority: P2)

**Goal**: Reorder tracks in session and auto-update base-26 `[XXYY]` sequence positions.

**Independent Test**: Move track 1 before track 0; verify XXYY prefixes recalculate correctly (0 -> `AAAA`, 1 -> `AAAB`).

- [X] T012 [P] [US3] Unit test for playlist track reordering in `tests/unit/naming_tests.rs`
- [X] T013 [US3] Implement track reordering and sequence recalculation in `src/models/session_state.rs`

---

## Phase 6: User Story 4 - Audio Preview with Playback Controls (Priority: P2)

**Goal**: Audio preview player with Play, Pause, Stop, and Seek.

**Independent Test**: Load `mp3-sample/sample-3s.mp3`, trigger play, verify playback starts, trigger pause/stop.

- [X] T014 [P] [US4] Implement AudioPreviewPlayer in `src/services/audio_player.rs` using `rodio`
- [X] T015 [US4] Connect audio preview playback controls to UI/CLI runner

---

## Phase 7: User Story 5 - Undo/Redo Operations (Priority: P2)

**Goal**: In-memory undo/redo stack bounded to 10 operations.

**Independent Test**: Perform 3 tag edits, call undo twice, verify reversion; call redo, verify restoration.

- [X] T016 [P] [US5] Unit test for UndoStack in `tests/unit/undo_tests.rs`
- [X] T017 [US5] Implement Undo/Redo dispatch and stack limiting in `src/models/session_state.rs`

---

## Phase 8: User Story 6 - Export Playlist (Priority: P3)

**Goal**: Export playlist summary to `.txt` and compact `.rtf`.

**Independent Test**: Export sample playlist to `.rtf`; verify valid RTF header and table markup.

- [X] T018 [P] [US6] Implement RTF playlist exporter in `src/services/rtf_exporter.rs`
- [X] T019 [US6] Unit test for RTF and TXT export formatting in `tests/unit/project_tests.rs`

---

## Phase 9: Polish & GTK3 UI Integration

**Purpose**: Wire all services into the GTK3 desktop interface and CLI.

- [X] T020 [US1] [US2] Connect directory scanner and inline ID3 tag editing to GTK3 table in `src/ui/gtk_ui.rs`
- [X] T021 [US3] Add Move Up / Move Down reordering buttons to GTK3 table in `src/ui/gtk_ui.rs`
- [X] T022 [US4] Add Audio Preview playback bar (Play/Pause/Stop) at bottom of GTK3 window in `src/ui/gtk_ui.rs`
- [X] T023 [US5] Add Undo / Redo buttons and shortcuts to GTK3 window in `src/ui/gtk_ui.rs`
- [X] T024 Add `musicbox scan [dir]` and `musicbox tag` CLI subcommands in `src/main.rs`
- [X] T025 [US7] Add Dark/Light mode theme toggle in GTK3 header bar and persist preference in `src/ui/gtk_ui.rs`
- [X] T026 Run end-to-end integration test with sample files in `mp3-sample/`
