# Feature Specification: MP3 File Organizer

**Feature Branch**: `001-mp3-file-organizer`

**Created**: 2026-11-19

**Status**: Draft

**Input**: User description: "Lightweight Rust desktop app that organizes MP3s using sequence-based naming convention [XXYY-filename.mp3], with ID3 tag editing, drag-drop sorting, audio preview, and export to txt/rtf/docx formats."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Load and Display MP3s (Priority: P1)

User opens the app and needs to organize their MP3 collection. The app scans the current directory and displays all MP3 files for management.

**Why this priority**: Core MVP feature. Without loading and displaying files, the app cannot function. Users must see their library before taking any action.

**Independent Test**: Can be tested by loading a directory with MP3s, verifying files appear in a list, and no errors occur. Delivers immediate value: visibility of music library.

**Acceptance Scenarios**:

1. **Given** app is opened, **When** current directory contains MP3 files, **Then** all MP3 files in the directory are listed (not including subdirectories)
2. **Given** directory is empty, **When** app loads, **Then** user sees empty state message
3. **Given** directory contains non-MP3 files, **When** app loads, **Then** only MP3 files are displayed, other files are ignored
4. **Given** files are already displayed, **When** new MP3 file is added to directory, **Then** app detects and displays the new file (or requires manual refresh)

---

### User Story 2 - Edit ID3 Tags and Rename Files (Priority: P1)

User needs to organize MP3s by modifying ID3 tags (track number, track name, album/group name, artist) and renaming files according to the sequence convention [XXYY-filename.mp3], where first 2 chars = group, next 2 = sequence.

**Why this priority**: Core organizational function. The entire purpose of the app is to manage MP3s through ID3 editing and file naming. This is non-negotiable for MVP.

**Independent Test**: Can be tested by selecting a file, editing its tags and sequence position, saving, and verifying the file is renamed and tags are updated in the saved MP3. Delivers complete organization workflow.

**Acceptance Scenarios**:

1. **Given** user double-clicks on a file in the list, **When** double-click action occurs on a tag column, **Then** inline edit mode activates for that field
2. **Given** user edits track name, album name, artist, or track number, **When** user confirms edit, **Then** changes are stored in app's working session (not written to file yet)
3. **Given** user has modified files and clicks Save, **When** save is triggered, **Then** app writes ID3 tags to MP3 files and renames files to [XXYY-filename.mp3] format based on group/sequence assignments
4. **Given** files are renamed, **When** save completes, **Then** session cache is deleted and app shows confirmation

---

### User Story 3 - Drag & Drop Grouping and Sorting (Priority: P2)

User needs to organize files into groups and set sequence order via drag-and-drop interaction, without manual text entry of group/sequence codes.

**Why this priority**: Improves UX efficiency. While users can manually edit each file's group/sequence, drag-drop is faster for bulk organization and supports the "quick development" principle.

**Independent Test**: Can be tested by dragging files into groups on-screen, dragging within a group to reorder, and verifying visual feedback. Delivers intuitive organization without coding sequence numbers manually.

**Acceptance Scenarios**:

1. **Given** multiple files are displayed, **When** user drags a file to a different group area, **Then** file is visually moved and assigned to the new group
2. **Given** files are in a group, **When** user drags a file up/down within the group, **Then** file position/sequence number updates and is reflected in the list
3. **Given** user performs drag operations, **When** drag occurs, **Then** visual feedback (highlight, drag preview) is shown

---

### User Story 4 - Audio Preview with Playback Controls (Priority: P2)

User wants to preview MP3s before finalizing organization. App displays audio player at bottom with waveform visualization and basic playback controls.

**Why this priority**: Validation feature. Users need to verify they're organizing the correct audio before saving. Enables confidence in organization decisions.

**Independent Test**: Can be tested by clicking a file, verifying audio preview loads, and testing each playback control (play, pause, stop, next, prev, loop). Delivers audio confirmation of file identity.

**Acceptance Scenarios**:

1. **Given** user selects or double-clicks an MP3 file, **When** selection occurs, **Then** audio preview loads at bottom of app with waveform visualization
2. **Given** preview is loaded, **When** user clicks Play, **Then** audio plays and waveform shows playback position
3. **Given** audio is playing, **When** user clicks Next/Prev, **Then** audio switches to next/previous file in the list
4. **Given** audio is playing, **When** user adjusts seek bar, **Then** playback jumps to that position
5. **Given** Loop button is toggled, **When** audio finishes, **Then** it restarts from beginning (or plays next file if loop is off)

---

### User Story 5 - Undo/Redo Operations (Priority: P2)

User makes mistakes during organization and needs to revert changes without losing data. Undo/redo allows reversing the last N operations before save.

**Why this priority**: Safety net for users. Reduces fear of making mistakes and improves confidence in experimenting with organization.

**Independent Test**: Can be tested by making changes (edit tags, drag files), clicking Undo, verifying reversion, clicking Redo, verifying restoration. Delivers confidence through reversibility.

**Acceptance Scenarios**:

1. **Given** user makes changes (edit, drag, sort), **When** user clicks Undo, **Then** the last change is reverted to previous state
2. **Given** user has undone actions, **When** user clicks Redo, **Then** the action is reapplied
3. **Given** app is open, **When** user saves, **Then** undo/redo history is cleared and cannot be undone across save boundaries
4. **Given** user closes app without saving, **When** app reopens, **Then** no changes persist (session cache ensures clean state)

---

### User Story 6 - Export Playlist (Priority: P3)

User needs to export the organized playlist to external formats (txt, rtf, docx) for sharing or documentation. Export creates a compact view suitable for printing or sharing.

**Why this priority**: Secondary feature. Users can organize without exporting, but export adds utility for documentation/sharing workflows.

**Independent Test**: Can be tested by exporting to each format, verifying files are created, and checking that content is properly formatted and compact. Delivers portability of playlist data.

**Acceptance Scenarios**:

1. **Given** files are organized, **When** user clicks Export button and selects txt, **Then** txt file is created with file names and metadata in plain text format
2. **Given** files are organized, **When** user clicks Export and selects rtf, **Then** file is created with compact layout (minimal spacing, small fonts) suitable for small-area printing (DOCX export deferred to v2+ per Constitution Principle I)
3. **Given** export completes, **When** export finishes, **Then** user sees confirmation and file location

---

### User Story 7 - Dark/Light Mode Toggle (Priority: P3)

User prefers dark or light theme based on environment and personal preference. Toggle is accessible in top-right corner of the app.

**Why this priority**: UX enhancement. Improves usability in different lighting conditions but doesn't block core workflows.

**Independent Test**: Can be tested by clicking toggle, verifying theme switches, and checking that preference persists across session. Delivers accessibility and comfort.

**Acceptance Scenarios**:

1. **Given** app is open, **When** user clicks theme toggle (top-right corner), **Then** UI switches between dark and light mode
2. **Given** app is closed in dark mode, **When** app reopens, **Then** dark mode is remembered and restored

---

### Edge Cases

- What happens when user attempts to rename two files to the same [XXYY] sequence? (Should show conflict warning or prevent overwrite)
- How does app handle corrupted MP3 files that lack ID3 tags? (Should load file but show placeholder values in tags)
- What happens if user deletes MP3 file from directory while app is open? (Should detect on next refresh or show warning if file no longer exists at save time)
- What if user drags a file outside all group boundaries? (Should clarify group assignment or place in unassigned state)
- How does app handle extremely large MP3 files or directories with 1000+ files? (Performance expectations not specified)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST scan current directory on app load and identify all MP3 files (excluding subdirectories)
- **FR-002**: System MUST display MP3 files in a list with current filename, track number, track name, album name, and artist (read from ID3 or placeholders if missing)
- **FR-003**: System MUST allow double-click on any tag column (track number, track name, album name, artist) to enter inline edit mode
- **FR-004**: System MUST store all edits and sequence changes in an in-memory session cache during the app session
- **FR-005**: System MUST support drag-and-drop to assign files to groups (first 2 chars of naming convention: aa-zz range) and set sequence order within groups (third-fourth chars: 00-zz range)
- **FR-006**: System MUST validate naming convention format [XXYY-filename.mp3] before writing to disk
- **FR-007**: System MUST write validated ID3 tags (track number, track name, album name, artist) to MP3 files upon save
- **FR-008**: System MUST rename MP3 files to [XXYY-filename.mp3] format upon save, preserving original filename after hyphen
- **FR-009**: System MUST display audio preview player at bottom of window with waveform visualization when user selects an MP3
- **FR-010**: System MUST support playback controls: Play, Pause, Stop, Next, Previous, Loop toggle
- **FR-011**: System MUST allow seek bar interaction to jump to specific position in audio
- **FR-012**: System MUST implement Undo functionality to revert the last operation (max depth: 10 undo steps per session)
- **FR-013**: System MUST implement Redo functionality to reapply undone operations
- **FR-014**: System MUST support export to txt format with file list and metadata
- **FR-015**: System MUST support export to rtf format with compact layout
- **FR-016**: System MUST support playlist export to compact printable format (RTF and TXT in v1; DOCX deferred to v2+ per Principle I)
- **FR-017**: System MUST provide theme toggle (dark/light mode) in top-right corner of UI
- **FR-018**: System MUST persist theme preference across sessions
- **FR-019**: System MUST maintain in-memory session cache during app session tracking edits and dirty state without leaving temporary files on disk (per Principle IV)
- **FR-020**: System MUST reset session cache dirty state upon successful save with zero orphaned files
- **FR-021**: System MUST support window resizing from compact to maximized view, with audio preview bar scaling accordingly

### Key Entities

- **MP3 File**: Represents an audio file with filename, file path, ID3 metadata (track number, track name, album, artist), and sequence assignment (group code XX + sequence code YY)
- **Session State**: In-memory representation of all file edits, tag changes, sequence assignments, and undo/redo history for current app session
- **Group**: Logical collection of files assigned the same group code (first 2 chars of naming convention)
- **Sequence**: Order position within a group (second 2 chars of naming convention, 00-99 or aa-zz range)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can load and organize 100+ MP3 files from directory without performance degradation (app remains responsive during load, drag, edit, and save operations)
- **SC-002**: MP3 files are correctly renamed to [XXYY-filename.mp3] format 100% of the time with zero data loss or corruption
- **SC-003**: ID3 tags (track number, track name, album name, artist) are correctly written to MP3 files 100% of the time
- **SC-004**: Users can complete a full organization workflow (load → edit → drag → save) in under 5 minutes for a typical 50-file playlist
- **SC-005**: Audio preview starts playing within 500ms of file selection
- **SC-006**: Waveform visualization renders smoothly without frame stuttering
- **SC-007**: Exported playlist files (txt/rtf) open correctly in standard viewers and contain all expected metadata
- **SC-008**: App starts up in under 1 second on initial load
- **SC-009**: All Undo operations revert changes with 100% accuracy
- **SC-010**: In-memory session cache resets upon successful save with zero orphaned temporary files left on disk

## Assumptions

- **User Environment**: Users have a local directory of MP3 files accessible to the app at runtime
- **MP3 Compatibility**: All MP3 files in the directory have valid audio format (corrupted files may be skipped or handled gracefully)
- **ID3 Tag Support**: App supports ID3v2.3 or later tag format; older ID3v1 may be upgraded on write
- **File Naming**: After save, files are expected to follow [XXYY-filename.mp3] convention; any existing files with different naming are subject to renaming
- **Session Cache**: Session cache is stored as JSON or binary file in current directory and is non-critical (safe to delete manually)
- **Audio Preview**: Audio preview is for verification only; app does not modify audio stream or perform DSP operations
- **Group/Sequence Range**: Group codes range aa-zz (26 groups), sequence codes range 00-99 (100 sequences per group) — sufficient for typical playlists
- **Persistence**: User preferences (theme, window size) are persisted to a config file in app's local data directory or current directory
- **Single Directory**: App operates on a single directory per session; multi-directory support is out of scope for v1
