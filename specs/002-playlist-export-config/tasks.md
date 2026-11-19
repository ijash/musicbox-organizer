# Tasks: Playlist Export Config & Theme Persistence

**Input**: Design documents from `/specs/002-playlist-export-config/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)

---

## Phase 1: Foundational (Preferences & Models)

**Purpose**: Core preference storage and format models

- [X] T001 Implement UserPreferences model with load and save in `src/models/preferences.rs`
- [X] T002 [P] Implement ExportFormatSelection in `src/models/preferences.rs`
- [X] T003 Export preferences in `src/models/mod.rs` and `src/lib.rs`
- [X] T004 [P] Unit test for UserPreferences serialization in `tests/unit/preferences_tests.rs`

---

## Phase 2: User Story 1 - Dark Theme by Default & Persistence (Priority: P1) 🎯 MVP

**Goal**: App defaults to dark mode and restores user preference across restarts.

**Independent Test**: Save preference as light mode; restart app; verify light mode restores.

- [X] T005 [US1] Load UserPreferences on startup in `src/ui/gtk_ui.rs`
- [X] T006 [US1] Persist active theme preference on toggle in `src/ui/gtk_ui.rs`

---

## Phase 3: User Story 2 - Multi-Format Clear-All Playlist Export (Priority: P1)

**Goal**: Export `.txt` and `.rtf` formats to `playlist-text/` folder with clear-all behavior.

**Independent Test**: Place a dummy file in `playlist-text/`; run export; verify dummy file deleted and `playlist.txt`/`playlist.rtf` generated.

- [X] T007 [P] [US2] Unit test for clear-all multi-format export in `tests/unit/export_config_tests.rs`
- [X] T008 [US2] Implement ExportConfigService in `src/services/export_config.rs`
- [X] T009 [US2] Export ExportConfigService in `src/services/mod.rs`

---

## Phase 4: User Story 3 - Audio Editing Framework Placeholder (Priority: P3)

**Goal**: Reserve architectural module and UI slot for future audio processing.

**Independent Test**: Verify `AudioProcessor` trait exists and returns disabled/coming soon status.

- [X] T010 [P] [US3] Implement AudioProcessor trait and stub in `src/services/audio_editing.rs`
- [X] T011 [US3] Export AudioProcessor in `src/services/mod.rs`

---

## Phase 5: Polish & UI / CLI Integration

**Purpose**: Connect export dialog format selection to GTK UI and add CLI subcommands.

- [X] T012 [US2] Add Multi-Format Export dialog with checkboxes in `src/ui/gtk_ui.rs`
- [X] T013 [US3] Add disabled "Audio Tools (Coming Soon)" item to GTK3 toolbar in `src/ui/gtk_ui.rs`
- [X] T014 Add `musicbox export-text [--clear] [dir]` CLI subcommand in `src/main.rs`
- [X] T015 Run full test suite and rebuild release binary per Constitution Principle VII
