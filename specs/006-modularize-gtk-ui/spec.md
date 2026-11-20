# Feature Specification: Modularize GTK UI Components (Tech Debt D3)

**Feature Branch**: `006-modularize-gtk-ui`

**Created**: 2026-11-20

**Status**: Draft

**Input**: User description: "tech debt d3."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Maintain Identical UI Appearance & Interactions (Priority: P1)

Users continue using the desktop application exactly as before. The application window layout, header buttons, playlist table, audio controls, device export panel, theme toggle, and footer link look and behave identically, with zero visual or behavioral regressions.

**Why this priority**: Refactoring must never degrade user experience or alter existing working workflows.

**Independent Test**: Can be tested by launching `cargo run` and exercising all existing workflows: adding tracks, editing tags, reordering up/down, playing audio preview, toggling theme, saving project, exporting media, and opening the footer link.

**Acceptance Scenarios**:

1. **Given** application launches, **When** examining window elements, **Then** visual layout, geometry (740x520), and colors (dark/light themes) match the pre-refactoring interface 100%.
2. **Given** user interacts with any UI control (inline edit, reorder, play, pause, theme toggle, export, footer link), **When** clicked or activated, **Then** all underlying actions execute with identical results.

---

### User Story 2 - Modular Component Separation for Maintainability (Priority: P1)

Developers maintaining or extending the UI work with clean, focused, isolated component files under 250 lines each instead of navigating a monolithic 1,150+ line file.

**Why this priority**: Reduces cognitive load, eliminates reader/writer merge conflicts, and isolates component lifecycle management.

**Independent Test**: Can be tested by inspecting the file structure under `src/ui/components/` and verifying that each module compiles with dedicated, clear responsibilities.

**Acceptance Scenarios**:

1. **Given** `src/ui/` codebase, **When** reviewing files, **Then** `src/ui/gtk_ui.rs` acts as a coordinator, delegating header, table, audio preview, export panel, and footer to modular subcomponents in `src/ui/components/`.
2. **Given** all components modularized, **When** running `cargo test`, **Then** 100% of existing tests pass without regressions.

---

### Edge Cases

- How does state synchronization work between components without circular references? Components borrow shared state via `Rc<RefCell<...>>` or event callbacks.
- How do theme CSS stylesheets apply across modular components? Stylesheets remain application-wide via `StyleContext::add_provider_for_screen`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST preserve 100% visual layout, styling, and geometry of the GTK3 desktop interface.
- **FR-002**: System MUST modularize monolithic `gtk_ui.rs` into clean subcomponents under `src/ui/components/`:
  - `header.rs`: App logo, title, undo, redo, and theme toggle.
  - `playlist_table.rs`: TreeView, inline editing handlers, row selection, and move up/down reordering.
  - `audio_bar.rs`: Playback controls (Play, Pause, Stop) and preview player status.
  - `export_panel.rs`: Device dropdown, scan button, reformat warning modal, and media export execution.
  - `footer.rs`: Attribution label and GitHub repository hyperlink.
- **FR-003**: System MUST preserve all existing state synchronization (undo/redo snapshotting, project file loading/saving, and device polling).
- **FR-004**: System MUST maintain clean compilation without introducing new compiler warnings or external runtime dependencies.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `src/ui/gtk_ui.rs` line count reduced from >1,150 lines to <200 lines.
- **SC-002**: Every subcomponent file in `src/ui/components/` contains <300 lines of code.
- **SC-003**: 100% of existing unit and integration tests (49/49) pass without modification.
- **SC-004**: Zero visual or functional regressions upon launching the application.

## Assumptions

- This is a pure architectural refactoring (tech debt reduction); no new user-facing features or schema changes are introduced.
- Existing Rust 2021 edition and GTK 0.17 bindings are retained.
