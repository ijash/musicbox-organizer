# Feature Specification: Removable Media Export

**Feature Branch**: `004-removable-media-export`

**Created**: 2026-11-19

**Status**: Draft

**Input**: User description: "it can choose active sd card or usb that plugged in, and choose action to export it. it should display warning before reformat. what i mean by saving the sorting project is in the current dir, not exporting to sd card. so mind the export and save difference."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Select and Export Playlist to USB/SD Card (Priority: P1)

User connects a USB drive or SD card to the computer. From the app, user selects which device to export to, chooses an export action (write playlist, write with reformat), and the app exports the organized playlist files to the selected device.

**Why this priority**: Core workflow. Users need an intuitive way to select their target device and export playlists without manual file copying. This is the primary reason for using the app (get files onto portable media).

**Independent Test**: Can be tested by connecting USB/SD card, using app UI to select device, triggering export, and verifying files appear on media with correct naming.

**Acceptance Scenarios**:

1. **Given** USB drive or SD card is connected to computer, **When** user opens Export dialog, **Then** app displays list of available removable devices with capacity and filesystem info
2. **Given** user selects a device from the list, **When** user confirms export, **Then** playlist files are copied to device root directory (or user-specified folder) with [XXYY-filename.mp3] naming convention
3. **Given** export completes successfully, **When** user ejects device and connects to portable player, **Then** files are present and playable in correct sequence
4. **Given** multiple devices connected, **When** user selects one and exports, **Then** only the selected device receives files (no cross-device contamination)

---

### User Story 2 - Reformat Device Before Export with Warning (Priority: P1)

User wants to clear old content from a USB/SD card before exporting new playlist. User selects "Reformat & Export" option. App displays a prominent warning about data loss. User confirms. App formats the device (to FAT32 or exFAT) and writes the playlist.

**Why this priority**: Common real-world scenario. Users often reuse old USB/SD cards that contain data they no longer need. Reformatting is faster than manual file deletion, but requires safety guards (warning, confirmation).

**Independent Test**: Can be tested by exporting to device with "Reformat" option enabled, verifying warning is shown, confirming action, and checking that device is emptied and files are written cleanly.

**Acceptance Scenarios**:

1. **Given** user selects "Reformat & Export" option, **When** user clicks Confirm, **Then** app displays a prominent warning dialog with data-loss message and device name
2. **Given** warning is displayed, **When** user clicks Cancel, **Then** reformat is aborted and device is left unchanged
3. **Given** warning is displayed, **When** user clicks Confirm, **Then** device is reformatted to FAT32 (or exFAT) and playlist files are written
4. **Given** reformat completes, **When** user checks device, **Then** only playlist files are present (no orphaned data)

---

### User Story 3 - Distinguish "Save" (Local Project) vs "Export" (to Media) (Priority: P2)

App supports two distinct operations: Save (persist project file locally for future editing) and Export (write organized playlist to USB/SD). UI clearly labels and separates these two workflows so users don't confuse saving a project with exporting media.

**Why this priority**: UX clarity. Users need to understand that "Save" creates a reusable project file they can edit again, while "Export" is one-way delivery to portable media. Preventing confusion reduces support burden.

**Independent Test**: Can be tested by verifying UI has separate Save and Export buttons/menu items, each with clear labels and documentation, and that they perform distinct operations with no side effects on each other.

**Acceptance Scenarios**:

1. **Given** user has organized playlist in app, **When** user clicks Save, **Then** project file (.txt or app-specific format) is saved to current working directory for future edits
2. **Given** user clicks Export, **When** export completes, **Then** only playlist media files are written to device; no project metadata is copied
3. **Given** user saves project and later exports playlist, **When** user returns to app, **Then** project state is restored from local file (independent of any prior export)

---

### Edge Cases

- What happens if user connects multiple USB/SD cards? (App should list all; user selects specific device; export targets only that device)
- What if device is write-protected or read-only? (App should detect and display error before attempting write; suggest checking physical write-protect switch)
- What if device becomes disconnected during export? (App should detect mid-operation, abort gracefully, and warn user to reconnect and retry)
- What if device is too small for full playlist? (App should estimate required space, warn user if insufficient, and allow user to select subset of files or cancel)
- What if user selects a system drive or OS partition by mistake? (App should whitelist removable media only, reject internal drives, and warn if user tries to export to non-removable path)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: App MUST detect and list all connected USB drives and SD card readers as removable devices (exclude internal/system drives)
- **FR-002**: App MUST display device info for each device: device name, capacity (GB), filesystem type (FAT32, exFAT, NTFS), available space
- **FR-003**: App MUST provide two export modes: "Export" (write files to existing filesystem) and "Reformat & Export" (erase device, write files)
- **FR-004**: App MUST display a prominent, dismissible warning dialog when user selects "Reformat & Export", showing device name and data-loss message; user must explicitly confirm before proceeding
- **FR-005**: App MUST estimate required storage (based on playlist file count and average size) and warn user if device has insufficient free space; user can retry with subset or cancel
- **FR-006**: App MUST reject export to non-removable devices (internal drives, OS partitions); show error message if user attempts this
- **FR-007**: App MUST support exporting to device root directory or user-specified subdirectory on device
- **FR-008**: App MUST write playlist files with [XXYY-filename.mp3] naming convention to target device, following device compatibility naming standard (spec v003)
- **FR-009**: App MUST handle mid-operation disconnection: if device is unplugged during export, abort gracefully, display error, allow user to reconnect and retry
- **FR-010**: App MUST detect write-protected devices and display error message before attempting write; suggest checking physical write-protect switch
- **FR-011**: App MUST save project file locally (to current working directory or user-specified path) independently of any export operation; project save and device export are separate workflows
- **FR-012**: App MUST display export progress (files written, percentage complete, elapsed time) during export operation
- **FR-013**: App MUST provide "Eject Device" button after successful export to safely unmount device (platform-appropriate eject command)

### Key Entities

- **Removable Device**: USB drive, SD card reader, or similar. Attributes: device name, mount path, filesystem type, total capacity, available space, write-protected status
- **Export Configuration**: target device, export mode (normal/reformat), target directory on device, file subset (all or selected files)
- **Project File**: Local file storing playlist organization state. Attributes: file path (current working directory), format (.txt or app-native), last modified timestamp, playlist metadata

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can select a device and complete export to USB/SD within 30 seconds (after device list is displayed)
- **SC-002**: Reformat warning is displayed in a modal dialog that blocks further action until user confirms or cancels; no accidental reformats possible
- **SC-003**: 100% of exported files appear on target device with correct naming convention and file count matches export selection
- **SC-004**: No files are written to non-removable devices or system partitions; attempts are rejected with clear error messages
- **SC-005**: Export completes successfully on at least 3 different device types (USB 2.0, USB 3.0, SD card reader) without user intervention
- **SC-006**: Device disconnection mid-export is detected within 5 seconds and reported to user with error message; export can be retried
- **SC-007**: Project save and export operations are independent; saving project does not affect device selection or export, and vice versa
- **SC-008**: App remains responsive during export (progress display updates, user can monitor status); no UI freeze

## Assumptions

- **Device Detection**: App uses platform-native APIs to enumerate removable devices (no third-party libraries required for detection)
- **Filesystem Support**: v1 supports FAT32 and exFAT; NTFS deferred to v2+ (per device compatibility spec)
- **Reformat Scope**: Reformat operation clears entire device (not selective folder wipe); user is responsible for backing up data beforehand
- **Project File Location**: Project files saved to current working directory by default; user can override path via File > Save As
- **Device Permissions**: User has permission to write to removable devices (OS-level permissions assumed to be granted by user)
- **Export Target**: Exported files written to device root or user-specified subdirectory; no automatic subfolder creation beyond user-specified path
- **No Network**: All device operations are local; no cloud sync or remote device access in v1
- **Platform-Native**: Device detection and eject operations use OS-specific commands (no cross-platform abstraction library); each platform implements natively
