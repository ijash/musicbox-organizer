# Tasks: Modularize GTK UI Components (006-modularize-gtk-ui)

**Input**: Design documents from `/specs/006-modularize-gtk-ui/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)

---

## Phase 1: Setup & Theme Extraction

**Purpose**: Create submodules and extract theme stylesheet constants

- [X] T001 [P] Create `src/ui/theme.rs` with `LIGHT_CSS`, `DARK_CSS`, and `apply_theme` helper
- [X] T002 [P] Create `src/ui/components/mod.rs` declaring component submodules
- [X] T003 Export theme and components in `src/ui/mod.rs`

---

## Phase 2: Foundational Components (US2)

**Purpose**: Extract isolated leaf components

- [X] T004 [P] [US2] Implement `HeaderComponent` in `src/ui/components/header.rs` (logo, title, undo, redo, theme toggle)
- [X] T005 [P] [US2] Implement `FooterComponent` in `src/ui/components/footer.rs` (italic attribution and GitHub link)
- [X] T006 [P] [US2] Implement `AudioBarComponent` in `src/ui/components/audio_bar.rs` (play, pause, stop, status label)

---

## Phase 3: Core Complex Components (US1, US2)

**Purpose**: Extract table and export panel with state coordination

- [X] T007 [US2] Implement `PlaylistTableComponent` in `src/ui/components/playlist_table.rs` (TreeView, inline edit, toolbar, reorder, scan)
- [X] T008 [US2] Implement `ExportPanelComponent` in `src/ui/components/export_panel.rs` (device selection, refresh, reformat warning, export media)

---

## Phase 4: Assembler & Integration (US1, US2)

**Purpose**: Assemble window in `gtk_ui.rs` and verify line counts

- [X] T009 [US1] [US2] Refactor `src/ui/gtk_ui.rs` to assemble window by calling component builders (<200 lines)
- [X] T010 [US1] Verify visual layout, inline edits, reorder, audio preview, and export workflows
- [X] T011 Run `cargo test` (all 49 tests pass) and rebuild release binary (`cargo build --release`) per Constitution Principle VII
