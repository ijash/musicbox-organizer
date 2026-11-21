# Tasks: Automated GUI Test Harness and Relm4 Icons (Feature 007)

**Input**: Design documents from `/specs/007-gui-test-harness-relm4-icons/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)

---

## Phase 1: Setup & Dependency Configuration

**Purpose**: Add in-use icon definitions and test harness infrastructure

- [X] T001 [P] Implement `src/ui/icons.rs` with embedded SVG data for in-use icons (FR-003, FR-004)
- [X] T002 [P] Create `tests/gui/mod.rs` and `tests/gui_tests.rs` test harness target
- [X] T003 Create executable script `tests/gui/run_gui_tests.sh` with `xvfb-run -a` wrapper (FR-001)

---

## Phase 2: User Story 2 - Standardized Embedded Relm4 Icons (Priority: P1)

**Goal**: Replace ad-hoc unicode symbols across toolbars with crisp, theme-aware Relm4 symbolic icons.

**Independent Test**: Launch GUI, verify standard vector icons render on all buttons across dark and light themes.

- [X] T004 [US2] Update `src/ui/components/playlist_table.rs` toolbar buttons (Scan, Add, Remove, Up, Down, Save, Open) to use `src/ui/icons.rs` icons (FR-005)
- [X] T005 [US2] Update `src/ui/components/audio_bar.rs` buttons (Play, Pause, Stop) to use `src/ui/icons.rs` icons (FR-005)
- [X] T006 [US2] Update `src/ui/components/header.rs` theme button to use `src/ui/icons.rs` icons (FR-005)

---

## Phase 3: User Story 1 - Headless & Automated GUI Test Harness (Priority: P1) 🎯 MVP

**Goal**: Programmatically drive the GTK interface under `xvfb-run` to exercise interactive click flows.

**Independent Test**: Run `./tests/gui/run_gui_tests.sh`, confirm all interactive simulation steps pass in <10s.

- [X] T007 [US1] Implement `tests/gui/harness.rs` programmatic driver with event pump and button click simulation
- [X] T008 [US1] Implement automated test suite in `tests/gui_tests.rs` testing track addition, inline edit, reorder up/down, remove, audio preview, and theme toggle (FR-002)

---

## Phase 4: Polish & Binary Size Audit

**Purpose**: Verify binary size ceiling and Constitution compliance

- [X] T009 Run `./tests/gui/run_gui_tests.sh` under `xvfb-run` and verify 0 failures
- [X] T010 Run `cargo test` (all unit and integration tests pass) and rebuild release binary (`cargo build --release`) per Constitution Principle VII
- [X] T011 Audit release binary size (`ls -lh target/release/musicbox`) ensuring it remains <2.5 MB per Constitution Principle I
