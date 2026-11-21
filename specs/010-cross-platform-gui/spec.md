# Feature Specification: Unified Cross-Platform Rust GUI (v0.3.0)

**Feature Branch**: `010-cross-platform-gui`

**Created**: 2026-11-22

**Status**: Draft

**Input**: User description: "Implement Feature 010: Unified Cross-Platform Rust GUI (`v0.3.0`) replacing the Linux-only GTK3 UI with a single pure-Rust `eframe`/`egui` (`glow` renderer) + `rfd` desktop frontend for Linux, Windows, and macOS, reusing 100% of existing `src/services/` and `src/models/`."

## Clarifications

### Session 2026-11-22
- Q: Should Windows builds conditionally attach the parent console when CLI arguments are passed so CLI subcommands display output in PowerShell and Command Prompt? (FR-002) → A: Yes, conditionally attach parent console (`AttachConsole(ATTACH_PARENT_PROCESS)`) on Windows when CLI arguments are present to keep GUI double-click launch silent while showing CLI output in terminals.
- Q: Should the project retain Rust `edition = "2021"` with pinned compatible `eframe` and `egui_kittest` versions, or upgrade `Cargo.toml` to Rust `edition = "2024"`? → A: Upgrade `Cargo.toml` to Rust `edition = "2024"` adopting the modern resolver and latest `eframe` (`glow`) + `egui_kittest` releases.
- Q: Should the Linux file dialog implementation use `rfd`'s `xdg-portal` feature to eliminate GTK3 entirely from build and runtime dependencies? (FR-004) → A: Yes, configure `rfd` with `default-features = false, features = ["xdg-portal"]` to eliminate GTK3 entirely from Linux build dependencies and runtime.
- Q: How should toolbar and audio icons render in `egui` to guarantee high contrast across Dark and Light themes? (FR-018) → A: Use standard Unicode / text symbol glyphs (e.g. ▶, ⏸, ⏹, ⬆, ⬇, ➕, 🗑, ⚙) directly on buttons for minimal binary size and automatic theme-inherited text color contrast.
- Q: Should Feature 010 keep `src/platform/windows.rs` and `macos.rs` stubs returning clean empty-device lists, focusing scope strictly on the unified `eframe` GUI migration? (FR-014) → A: Implement basic Windows removable drive enumeration (`GetLogicalDrives` / `GetDriveTypeW`) in `src/platform/windows.rs` to detect USB/SD drives on Windows, while keeping macOS stubbed.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Cross-Platform Native Desktop Launch & Playlist Organization (Priority: P1)

Users on Linux, Windows, and macOS want a single desktop application that launches immediately upon opening or double-clicking, presenting a clean graphical interface without unwanted terminal windows or complex platform setup. Within this interface, users organize audio tracks by scanning music folders, selecting individual files via native system dialogs, reordering playlist sequences, editing metadata inline, and using undo/redo to correct errors.

**Why this priority**: Core value proposition of the desktop application. Without a reliable, platform-native window and playlist manipulation surface, no secondary audio or export features are accessible.

**Independent Test**: Can be tested by launching the executable with no arguments across Linux, Windows, and macOS, populating a playlist via file/folder dialogs, modifying track ordering, inline-editing titles/artists, and performing undo/redo cycles.

**Acceptance Scenarios**:

1. **Given** the application binary on Windows, macOS, or Linux, **When** the user launches it without CLI parameters (e.g. double-clicking), **Then** the application opens the graphical window directly; on Windows, no terminal or console window flashes.
2. **Given** an open graphical window, **When** the user clicks "Scan Folder..." or "Add MP3", **Then** the system presents the native operating system file or directory chooser dialog and populates selected MP3 tracks into the playlist table.
3. **Given** tracks in the playlist table, **When** the user edits the "Title" or "Artist" cell inline, **Then** the track metadata updates immediately in memory and is reflected in the active session.
4. **Given** one or more selected tracks in the table, **When** the user clicks "Up", "Down", or "Remove", **Then** the playlist sequence updates and re-indexes track ordering numbers accordingly.
5. **Given** any playlist modification (add, reorder, edit, remove), **When** the user clicks "Undo", **Then** the playlist reverts to its previous state; clicking "Redo" reapplies the modification.

---

### User Story 2 - Audio Preview & Non-Blocking Audio DSP Processing (Priority: P2)

Users organizing audio playlists want to preview audio playback directly within the application and apply audio enhancements (volume normalization and silence trimming) to selected tracks. Audio processing must execute asynchronously in the background so the user interface remains smooth and responsive, with visual progress feedback and automatic playback safety.

**Why this priority**: Ensures audio consistency and cleanliness across tracks before exporting to portable hardware, while guaranteeing that audio processing never freezes or crashes the desktop experience.

**Independent Test**: Can be tested by playing an MP3 track using the preview controls, opening the Audio Tools dialog, setting volume normalization and silence trimming thresholds, confirming active playback stops automatically when processing begins, and watching progress complete without UI stutter.

**Acceptance Scenarios**:

1. **Given** an audio track in the playlist, **When** the user selects it and clicks "Play" in the audio preview bar, **Then** audio playback commences and the preview status displays the active track name.
2. **Given** an active preview playback, **When** the user clicks "Pause" or "Stop", **Then** playback pauses or halts immediately.
3. **Given** selected playlist tracks, **When** the user clicks "Audio Tools...", **Then** a modal dialog opens presenting configurable toggles and numeric inputs for Volume Normalization (default -14.0 dB) and Silence Trimming (default -45.0 dB).
4. **Given** the Audio Tools dialog with configured parameters, **When** the user confirms processing, **Then** active playback of any processed track stops, audio processing executes on a background thread with live progress updates, and the user interface remains responsive at 60 FPS without freezing.
5. **Given** completed audio processing, **When** the background task finishes, **Then** the dialog closes, updated tracks reflect their new state, and status reporting confirms successful batch processing.

---

### User Story 3 - Removable Storage Detection & Lexicographical Export (Priority: P3)

Users preparing media for portable music boxes, car stereos, or microcontroller-based players want to detect connected USB/SD drives, optionally reformat media with two-step safety guards, configure target directories, and export ordered playlists using strict lexicographical naming conventions so devices play tracks in exact sequence.

**Why this priority**: Fulfills the core mission of MusicBox Organizer: producing portable physical media that primitive hardware players sort and play deterministically without complex playlist files.

**Independent Test**: Can be tested by plugging in removable storage (or simulating storage mounts), scanning drives, configuring an export subpath, testing the two-step reformat confirmation guard, executing export, and safely unmounting the device.

**Acceptance Scenarios**:

1. **Given** connected removable storage, **When** the user navigates to the export panel and clicks "Scan", **Then** available removable storage drives populate the device selection dropdown.
2. **Given** a selected target device, **When** the user checks "Reformat before export" and initiates export, **Then** the application requires an explicit secondary confirmation step before any destructive filesystem action can occur.
3. **Given** a validated export configuration, **When** the user clicks "Export Playlist to Media", **Then** playlist tracks copy to the target location with formatted lexicographical filenames (`[XXYY-filename.mp3]`) and progress is displayed.
4. **Given** a completed export, **When** the user clicks "Eject Device", **Then** the application triggers safe unmounting/ejection of the physical drive and updates status text.

---

### User Story 4 - Theme Accessibility, Text/RTF Export & Headless CI Verification (Priority: P4)

Users and maintainers require accessible visual themes (Dark and Light modes with clear contrast), playlist summary documentation export (Text/RTF), and headless cross-platform verification so that automated tests validate user interactions without requiring physical display hardware.

**Why this priority**: Delivers visual comfort in varying lighting conditions, provides printable/shareable tracklists, and guarantees continuous quality control across all three desktop operating systems in CI environments.

**Independent Test**: Can be tested by toggling between Dark and Light visual themes, exporting playlist summaries to RTF/Text, and executing automated GUI test suites headlessly across Linux, Windows, and macOS runners.

**Acceptance Scenarios**:

1. **Given** the application running, **When** the user toggles the theme button in the header, **Then** the UI switches between dark and light themes with high-contrast text and icons.
2. **Given** an organized playlist, **When** the user clicks "Export Text/RTF...", **Then** the application prompts for output format and destination, exporting formatted tracklist text.
3. **Given** an automated test run in a continuous integration environment lacking display servers, **When** the GUI test suite runs, **Then** all headless widget interactions, dialog triggers, and state assertions execute and pass without requiring display emulator software.

---

### Edge Cases

- **No Audio Output Hardware**: In headless test environments or machines lacking sound cards, preview playback commands fall back cleanly without panic or UI error.
- **Concurrent Preview and DSP Modification**: If a track currently playing in the preview bar is targeted for silence trimming or normalization, the system automatically terminates playback before modifying the underlying file.
- **Cancelled System File Dialogs**: When a user dismisses a native folder scan or file selection dialog without selecting files, the playlist and application state remain unchanged with no error toasts.
- **Sudden Device Disconnection**: If a removable storage device is unplugged during an active export operation, the process halts gracefully, logs the error in the status bar, and avoids corrupting application state.
- **Large Playlist Navigation**: Handling playlists exceeding 500 items must maintain consistent scrolling and responsive inline editing without UI latency or frame dropping.
- **Accidental Media Reformatting**: Attempting to check and execute drive reformatting requires explicit two-step confirmation to prevent irreversible data loss.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST launch the graphical user interface directly when started with no command-line arguments on Linux, Windows, and macOS.
- **FR-002**: System MUST suppress console/terminal window flash when launched on Windows desktop environments, and MUST conditionally attach the parent console (`AttachConsole(ATTACH_PARENT_PROCESS)`) when CLI arguments are present so CLI subcommand output is visible in PowerShell and Command Prompt.
- **FR-003**: System MUST preserve all existing command-line subcommands (`status`, `scan`, `tag`, `save`, `export`, `export-text`, `check-compat`, `help`) when arguments are provided.
- **FR-004**: Users MUST be able to open native operating system file and folder choosers to scan directories and add MP3 files to the playlist (using `rfd` with `xdg-portal` on Linux to achieve zero GTK3 dependency).
- **FR-005**: System MUST display playlists in a structured table showing sequence number, generated lexicographical export filename (`[XXYY-filename.mp3]`), inline-editable title, inline-editable artist, and source file path.
- **FR-006**: Users MUST be able to reorder tracks upward and downward in the playlist.
- **FR-007**: Users MUST be able to remove one or more selected tracks from the playlist.
- **FR-008**: Users MUST be able to undo and redo playlist actions, including track addition, reordering, metadata edits, and removals.
- **FR-009**: Users MUST be able to select multiple playlist rows for bulk removal or batch audio processing.
- **FR-010**: Users MUST be able to control audio preview playback (play, pause, stop) for selected tracks with an active playback status indicator.
- **FR-011**: System MUST stop active audio preview playback before initiating any audio modification on that track.
- **FR-012**: System MUST provide an Audio Tools modal dialog with configurable toggles and numeric inputs for Volume Normalization (default -14.0 dB) and Silence Trimming (default -45.0 dB).
- **FR-013**: System MUST execute audio processing operations asynchronously on background threads with live progress reporting, maintaining an interactive and responsive user interface.
- **FR-014**: System MUST scan and list available removable media storage devices in a dropdown selector, supporting Linux (via mount/udisks) and Windows (via `GetLogicalDrives` / `GetDriveTypeW`), while displaying a graceful empty state on macOS.
- **FR-015**: System MUST require a two-step explicit confirmation before executing media reformatting prior to playlist export.
- **FR-016**: System MUST export playlist tracks to target removable media with sequence-preserving lexicographical filenames and progress feedback.
- **FR-017**: Users MUST be able to request safe unmounting/ejection of selected removable media devices.
- **FR-018**: Users MUST be able to toggle between Dark and Light visual themes, maintaining high contrast and readability across widgets, text, and compact toolbar buttons using standard text/Unicode symbol glyphs that automatically inherit the active theme text color.
- **FR-019**: Users MUST be able to export playlist summaries to Text and RTF formats via a dedicated export dialog.
- **FR-020**: System MUST display application branding, track counts, live status messages, and project repository links.
- **FR-021**: System MUST support automated headless interaction testing for all user interface components and state transitions across Linux, Windows, and macOS without requiring a physical or virtual display server.

### Key Entities

- **Playlist Session**: Encapsulates the active ordered list of tracks, selection state, undo/redo command history stacks, and active project file path.
- **Playlist Track Item**: Represents a single track entry containing track index, source filesystem path, metadata attributes (title, artist, duration), formatted export filename, and dirty/modified status.
- **Audio DSP Task**: Encapsulates processing parameters (normalization target dB, trimming threshold dB, active track list) and channel-based progress/completion events.
- **Removable Media Target**: Represents a detected storage device including volume mount point, total and available capacity, filesystem identifier, and unmount capability.
- **Application Appearance**: Represents current visual theme configuration (Dark mode vs Light mode) and UI sizing metrics.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Standalone release executable size remains under 20 megabytes across Linux, Windows, and macOS platforms.
- **SC-002**: Graphical interface initializes and displays the main window in under 500 milliseconds on standard desktop systems.
- **SC-003**: Launching the desktop executable on Windows displays zero visible command prompt or terminal console windows.
- **SC-004**: 100% of existing CLI subcommands continue to function without regression when invoked from the command line.
- **SC-005**: 100% of automated GUI interaction tests execute and pass in headless environments without requiring virtual display servers (e.g. `xvfb`).
- **SC-006**: User interface maintains smooth responsiveness (at or above 60 frames per second) during background audio DSP processing.
- **SC-007**: Exported playlist files on storage media sort in exact intended playback order on primitive hardware players adhering to standard ASCII byte ordering.

## Assumptions

- Rust toolchain: `Cargo.toml` upgrades to `edition = "2024"` supporting latest `eframe` (`glow`) and `egui_kittest` releases.
- Operating system targets are modern 64-bit platforms: Linux (x86_64, glibc 2.31+), Windows 10/11 (x86_64), and macOS (arm64, macOS 12+).
- Core domain logic, models, and file operations in `src/services/` and `src/models/` are fully decoupled and reused as the authoritative backend.
- Desktop environments provide standard window manager facilities for mouse, keyboard, and native file dialog interactions.
- In test environments lacking audio output drivers, audio playback functions safely degrade to silent headless operation without panic.
- Release workflow packages single standalone executables per platform without external runtime dependencies or installer requirements.
