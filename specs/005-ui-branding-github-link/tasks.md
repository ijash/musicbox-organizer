# Tasks: UI Branding & GitHub Link Integration

**Input**: Design documents from `/specs/005-ui-branding-github-link/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)

---

## Phase 1: User Story 1 - Application Window Favicon & Header Branding (Priority: P1) 🎯 MVP

**Goal**: Display custom favicon icon in window title bar / taskbar and render a compact logo in the header box beside title.

**Independent Test**: Run `cargo run -- gui`, verify taskbar icon displays MusicBox logo, and check 26px logo rendered beside title.

- [X] T001 [US1] Set GTK application window icon from `img/logo/png/logo_mbo_128.png` with fallback in `src/ui/gtk_ui.rs` (FR-001)
- [X] T002 [US1] Add compact 26px logo image widget to top `header_box` beside `project_label` with fallback in `src/ui/gtk_ui.rs` (FR-002, FR-005)

---

## Phase 2: User Story 2 - Clickable GitHub Link in Header (Priority: P1)

**Goal**: Provide a discrete `GitHub ↗` button in top-right header toolbar that opens the repository URL in the system browser.

**Independent Test**: Click `GitHub ↗` button in header; verify default web browser launches `https://github.com/ijash/musicbox-organizer`.

- [X] T003 [US2] Add `GitHub ↗` button to `header_box` in `src/ui/gtk_ui.rs` (FR-003)
- [X] T004 [US2] Implement click handler calling `gtk::show_uri_on_window` for repository URL in `src/ui/gtk_ui.rs` (FR-004)

---

## Phase 3: Polish & Binary Rebuild

**Purpose**: Validate visual layout in both dark and light modes and rebuild binary per Constitution Principle VII.

- [X] T005 Verify layout in Dark Mode and Light Mode, ensuring logo and GitHub button render cleanly
- [X] T006 Run `cargo test` and rebuild release binary (`cargo build --release`) per Constitution Principle VII
