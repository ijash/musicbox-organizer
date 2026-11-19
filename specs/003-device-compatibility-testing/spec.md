# Feature Specification: Device Compatibility Testing

**Feature Branch**: `003-device-compatibility-testing`

**Created**: 2026-11-19

**Status**: Draft

**Input**: User description: "Ensure playlist compatibility with primitive audio devices (portable speakers, car head units, microcontrollers) that use lexicographical sorting without custom algorithms."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Verify Lexicographic Sort Playback on Portable Device (Priority: P1)

User exports a playlist from the app, writes it to USB/SD card, and plays it on a portable MP3 player. The player sorts files lexicographically and plays them in the intended sequence without app intervention.

**Why this priority**: Core use case. If playlists don't play in correct order on target devices, the app fails its primary mission. This is a blocker for release.

**Independent Test**: Can be tested by exporting playlist, writing to USB/SD card, inserting into portable MP3 player, and verifying playback order matches intended sequence.

**Acceptance Scenarios**:

1. **Given** user exports 10-50 MP3s with naming convention [XXYY-filename.mp3], **When** files are written to USB/SD card, **Then** files can be copied to device without errors
2. **Given** playlist files are on a portable MP3 player, **When** player sorts files lexicographically (ASCII order), **Then** playback order matches the intended sequence (no out-of-order playback)
3. **Given** multiple groups exist (e.g., aa, ab, ac groups), **When** device plays files, **Then** files from group aa play first, then ab, then ac, etc., within each group by sequence number (00, 01, 02...)
4. **Given** playlist contains filenames with special characters (hyphens, underscores), **When** device sorts and plays, **Then** playback order remains correct (no sort conflicts from special chars)

---

### User Story 2 - Test on Car Head Unit with Lexicographic Sorting (Priority: P1)

User burns playlist to CD or writes to USB in car head unit format. Car head unit uses lexicographic sorting without custom algorithms. Playlist plays in correct sequence without requiring manual track selection.

**Why this priority**: Common real-world use case (hunting, announcements in vehicles). Device must work out-of-the-box without user workarounds.

**Independent Test**: Can be tested by burning CD or writing USB, inserting into car head unit, and verifying playback order. Delivers reliable in-car audio experience.

**Acceptance Scenarios**:

1. **Given** playlist is exported and burned to CD or written to USB in FAT32/exFAT format, **When** CD/USB is inserted into car head unit, **Then** device recognizes files without errors
2. **Given** car head unit reads files and sorts lexicographically, **When** user presses Play, **Then** tracks play in intended order (not alphabetical by title, by filename sequence)
3. **Given** user presses Next/Previous during playback, **When** navigation occurs, **Then** next/previous tracks follow the lexicographic sequence (no jumps or skips)

---

### User Story 3 - Validate Naming Convention Prevents Sort Conflicts (Priority: P2)

Test that the [XXYY-filename.mp3] naming convention produces correct ASCII sort order. Verify edge cases (leading zeros, char boundaries, unicode edge cases) don't break playback order on primitive devices.

**Why this priority**: Validation of core design. Ensures naming scheme works across all target devices without unicode or encoding issues.

**Independent Test**: Can be tested by generating test playlists with various naming patterns and verifying ASCII sort produces intended order. Prevents future compatibility issues.

**Acceptance Scenarios**:

1. **Given** playlist has 100 files with names like [aa00-file.mp3], [aa01-file.mp3], ..., [ab00-file.mp3], **When** ASCII sort is applied, **Then** order is: aa00, aa01, ..., aa99, ab00, ab01, ... (not alphabetical jumbles)
2. **Given** filenames contain only ASCII characters (0-9, a-z, A-Z, hyphen), **When** sort occurs on any device, **Then** playback order is predictable and matches intended sequence
3. **Given** files use lowercase in group/sequence code (e.g., [aa00]), **When** device sorts, **Then** ordering is consistent (no case sensitivity issues)

---

### User Story 4 - Test Microcontroller/Embedded Device Compatibility (Priority: P3)

Test playlist compatibility with simple microcontroller-based MP3 players or embedded systems that read files from SD card and iterate lexicographically. Verify files can be detected and played on minimal-capability devices.

**Why this priority**: Future extensibility. Some users may use very basic audio hardware. Testing ahead prevents discovery of incompatibilities later.

**Independent Test**: Can be tested by writing playlist to SD card and testing on simple embedded MP3 player (Arduino-based, simple MP3 shields, etc.). Validates robustness.

**Acceptance Scenarios**:

1. **Given** SD card contains playlist files in [XXYY-filename.mp3] format, **When** microcontroller reads SD card and enumerates files lexicographically, **Then** files are detected without errors
2. **Given** embedded device plays files in lexicographic order, **When** playback begins, **Then** intended sequence is maintained (device doesn't require metadata or custom sort)

---

### Edge Cases

- What happens if user creates filenames with uppercase vs lowercase group codes (e.g., [Aa00] vs [aa00])? (Sorting may differ based on ASCII table; lowercase convention must be enforced)
- How does device handle files with unicode filenames or non-ASCII characters? (Should test and document that unicode breaks compatibility; only ASCII allowed)
- What if user manually renames files after export, breaking the naming convention? (Should document that custom names may not sort correctly; recommend using app for re-organization)
- What if SD card is formatted in NTFS? (v1 targets FAT32/exFAT; NTFS compatibility deferred to v2; app should document this limitation clearly)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: App MUST generate filenames in format [XXYY-filename.mp3] where XX = group code (aa-zz), YY = sequence (00-99), using only lowercase ASCII characters
- **FR-002**: App MUST verify exported filenames contain only ASCII characters (no unicode); reject or warn if unicode detected in title/artist metadata that could create non-ASCII filenames
- **FR-003**: Testing framework MUST validate that files generated by app sort correctly under ASCII/lexicographic ordering (not alphabetical by title)
- **FR-004**: Testing MUST confirm playback on at least 3 real-world device categories: portable MP3 player, car head unit, basic embedded/microcontroller device
- **FR-005**: App MUST document compatibility with device filesystem formats for v1: FAT32, exFAT supported; NTFS compatibility deferred to v2
- **FR-006**: App MUST provide export option targeting USB/SD card format (no special metadata that prevents device compatibility)
- **FR-007**: App MUST verify and warn if filename + metadata combination would create non-ASCII characters that break device sorting
- **FR-008**: Testing MUST confirm all special characters in filenames (hyphens, underscores, spaces) sort consistently across device types

### Key Entities

- **Compatibility Test Suite**: Automated validation of filename sorting and device playback scenarios
- **Device Compatibility Matrix**: Mapping of tested devices, filesystems, and confirmed compatibility status
- **Naming Convention Validator**: Logic to ensure [XXYY-filename.mp3] format and ASCII-only character constraints
- **Test Playlist**: Pre-built 20-100 file playlist with varied naming patterns for device testing

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Playlist files sort correctly under ASCII/lexicographic ordering 100% of the time (validated programmatically before export)
- **SC-002**: Exported playlist plays in intended sequence on 100% of tested portable MP3 players (tested on at least 3 different models)
- **SC-003**: Exported playlist plays in intended sequence on 100% of tested car head units (tested on at least 2 different models)
- **SC-004**: Exported playlist plays in intended sequence on 100% of tested embedded/microcontroller devices (tested on at least 1 basic model)
- **SC-005**: No non-ASCII characters appear in exported filenames when titles/artists contain only ASCII input
- **SC-006**: App warns or blocks export if user inputs non-ASCII metadata that would create incompatible filenames
- **SC-007**: App's naming validation catches 100% of violations to [XXYY-filename.mp3] format before export
- **SC-008**: Compatibility testing completes without manual device reconfiguration or special drivers

## Assumptions

- **Device Sorting**: All target devices use standard lexicographic (ASCII byte-order) sorting without custom algorithms; device firmware cannot be modified
- **Filesystem Format**: v1 targets USB/SD cards formatted as FAT32 or exFAT; NTFS compatibility deferred to v2+
- **ASCII-Only Input**: User input (titles, artists) is ASCII-only; non-ASCII input is rejected or sanitized at app level
- **Filename Length**: Filenames with [XXYY-filename.mp3] convention fit within filesystem limits (255 chars typical); app enforces reasonable length limits
- **Test Device Access**: Testing team has physical access to at least 3 portable MP3 players, 2 car head units, and 1 basic microcontroller device for validation
- **Playback Order Verification**: Manual playback testing or device debug output confirms file order; automated verification not expected on primitive devices
- **No Metadata Dependencies**: Device playback order depends only on filename sort, not ID3 tags or metadata; metadata is informational only
