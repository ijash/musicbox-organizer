# Feature Specification: UI Branding and GitHub Link Integration

**Feature Branch**: `005-ui-branding-github-link`

**Created**: 2026-11-20

**Status**: Draft

**Input**: User description: "- put the logo in the app. also favicon. you decide where the best placement for it. - add github link for it. make it at the top right"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Application Window Favicon & Header Branding (Priority: P1)

Users open the desktop application and immediately see official branding. The window title bar and taskbar display the custom icon (favicon/app icon), and the main application header displays a compact, sharp app logo beside the project title.

**Why this priority**: Core identity. Makes the application recognizable on desktop environments, window managers, and taskbars.

**Independent Test**: Can be tested by launching `cargo run` and verifying the window icon in the OS taskbar/title bar and the logo image rendered in the header box.

**Acceptance Scenarios**:

1. **Given** application launches on desktop, **When** window is displayed, **Then** the window manager and taskbar show the application icon (using `img/logo/ico/favicon.ico` or `img/logo/png/logo_mbo_128.png`).
2. **Given** main application window, **When** examining top header area, **Then** a 24x24 or 28x28 logo image is displayed to the left of "MusicBox Organizer" title.

---

### User Story 2 - Clickable GitHub Link in Header (Priority: P1)

Users can easily access the source repository and documentation from the app. A discrete GitHub button/link is located at the top-right corner of the application window. Clicking it opens the repository URL in the user's default web browser.

**Why this priority**: Connects user to documentation, issue tracker, and project updates directly from the running app.

**Independent Test**: Can be tested by clicking the top-right GitHub button/link and confirming the system browser opens `https://github.com/ijash/musicbox-organizer`.

**Acceptance Scenarios**:

1. **Given** main window header, **When** viewing top-right controls, **Then** a GitHub link/button is positioned adjacent to theme and undo/redo controls.
2. **Given** GitHub link button, **When** user clicks it, **Then** system opens `https://github.com/ijash/musicbox-organizer` via the default web browser without blocking or freezing the application UI.

---

### Edge Cases

- What happens if the logo asset path cannot be found at runtime (e.g. running outside repo root)? The app should gracefully fall back to title text without crashing or breaking layout.
- What happens if the desktop environment has no default browser configured or URI handler fails? The app should log/display a non-blocking notification with the URL instead of crashing.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Application window MUST set its window/favicon icon from available icon assets (`img/logo/ico/favicon.ico` or `img/logo/png/logo_mbo_128.png`).
- **FR-002**: Application top header MUST display a compact branding logo alongside the application title.
- **FR-003**: Application header MUST provide a GitHub repository button/link in the top-right toolbar area.
- **FR-004**: Clicking GitHub button MUST invoke system default browser for `https://github.com/ijash/musicbox-organizer`.
- **FR-005**: If branding assets cannot be read from disk, system MUST fallback gracefully and render UI normally without error popup.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Window icon displays on system window manager/taskbar on app launch.
- **SC-002**: Logo renders sharply in header with zero noticeable startup latency increase (<10ms).
- **SC-003**: Clicking GitHub link triggers default browser navigation in <1 second.

## Assumptions

- Target repository URL is `https://github.com/ijash/musicbox-organizer` as defined in `Cargo.toml`.
- Logo assets are loaded from `img/logo/` relative to working directory or bundled binary location.
