# Feature Specification: Playlist Export Config & Theme

**Feature Branch**: `002-playlist-export-config`

**Created**: 2026-11-19

**Status**: Draft

**Input**: User description: "Dark mode by default, clear-all export behavior with multiple format selection (txt/rtf/docx) stored in playlist-text directory, architecture support for future audio editing features (not implemented yet)."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Dark Theme by Default (Priority: P1)

User opens the app and experiences dark mode as the default visual theme, matching modern user expectations and reducing eye strain.

**Why this priority**: Immediate UX improvement. Matches constitution principle (user-centric simplicity). Quick win with high user satisfaction.

**Independent Test**: Can be tested by launching app and verifying UI renders in dark colors. Delivers comfortable viewing experience from first use.

**Acceptance Scenarios**:

1. **Given** app launches for first time, **When** app initializes, **Then** dark mode is active (not light mode)
2. **Given** user is in dark mode, **When** user navigates to any screen, **Then** all UI elements use dark color scheme (dark backgrounds, light text)
3. **Given** user toggles theme to light, **When** user closes and reopens app, **Then** light mode is remembered (user preference persists)

---

### User Story 2 - Playlist Export with Format Selection (Priority: P1)

User organizes their playlist and needs to export it to multiple formats. Export dialog allows user to select which formats to generate (txt, rtf, docx) before exporting. Clear-all behavior overwrites previous exports.

**Why this priority**: Core feature from v1 spec. Users must be able to export and share playlists. Format selection improves flexibility.

**Independent Test**: Can be tested by opening export dialog, selecting formats, confirming, and verifying files are created in playlist-text directory with correct content. Delivers playlist portability.

**Acceptance Scenarios**:

1. **Given** user clicks Export button, **When** export dialog opens, **Then** user sees checkboxes for txt, rtf, and docx formats with all checked by default
2. **Given** dialog is open, **When** user unchecks specific formats, **Then** only selected formats will be exported
3. **Given** user clicks Confirm, **When** export begins, **Then** all files in playlist-text directory are deleted first (clear-all behavior)
4. **Given** clear-all completes, **When** export proceeds, **Then** new files are written for each selected format: playlist.txt, playlist.rtf, playlist.docx
5. **Given** export finishes, **When** user checks the playlist-text directory, **Then** only the selected format files exist (no orphaned files from previous exports)
6. **Given** user exports again with different format selection, **When** second export runs, **Then** playlist-text directory is cleared first and repopulated with new selection

---

### User Story 3 - Audio Editing Framework Placeholder (Priority: P3)

App reserves architectural space for future audio editing features (looping, volume, compression, noise reduction, EQ, etc.) without implementing these features in v1.

**Why this priority**: Future-proofing. Allows modular expansion without major rewrites. Constitution principle (rapid development) prioritizes core org features first; audio editing deferred.

**Independent Test**: Can be tested by verifying app structure allows adding audio editing module later (extensibility hook exists, documented, not yet populated).

**Acceptance Scenarios**:

1. **Given** app is architected, **When** structure is reviewed, **Then** a documented hook/module slot for audio editing features exists
2. **Given** framework is in place, **When** future audio editing feature is added, **Then** integration point is clear and no core reorganization logic needs modification
3. **Given** v1 is released, **When** user opens audio editing menu/panel, **Then** placeholder message or disabled controls indicate feature is coming (not crash or error)

---

### Edge Cases

- What happens if user unchecks all export formats? (Should show warning or re-enable at least one format)
- What if playlist-text directory is locked or read-only? (Should show error message and allow user to choose alternate location or retry)
- What if export partially fails (e.g., rtf fails but txt succeeds)? (Should report which formats succeeded/failed and allow retry)
- How are existing user preferences (theme choice, export format selection) stored if app crashes before save? (Should auto-save preferences on change)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST launch with dark mode as the default UI theme
- **FR-002**: System MUST allow user to toggle between dark and light themes via UI control
- **FR-003**: System MUST persist theme preference so that when app relaunches, the last selected theme is restored
- **FR-004**: System MUST provide Export button accessible from main UI
- **FR-005**: System MUST display export dialog showing format selection checkboxes for txt, rtf, and docx
- **FR-006**: System MUST allow user to check/uncheck any combination of formats in the export dialog
- **FR-007**: System MUST validate that at least one format is selected before allowing export to proceed by disabling the Confirm/Export button when no format is checked
- **FR-008**: System MUST implement clear-all behavior: delete all existing files in playlist-text directory before writing new exports
- **FR-009**: System MUST create playlist-text directory if it does not exist in current directory
- **FR-010**: System MUST write exported files with standard names: playlist.txt, playlist.rtf, playlist.docx (only for selected formats)
- **FR-011**: System MUST include audio editing module hook in codebase with documentation for future expansion
- **FR-012**: System MUST show placeholder UI element (disabled menu/button/panel) indicating audio editing features are planned but not yet available
- **FR-013**: System MUST not crash or show errors when audio editing placeholder is visible; it should gracefully indicate "Coming Soon" or similar

### Key Entities

- **Theme Preference**: User's choice of dark or light mode, persisted to local config
- **Export Configuration**: Set of selected file formats (txt, rtf, docx) and target directory (playlist-text)
- **Export Output**: Three possible files (playlist.txt, playlist.rtf, playlist.docx) created based on format selection
- **Audio Editing Framework**: Extensibility hook/interface for future audio feature modules (not populated in v1)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Dark mode is active on app first launch with 100% reliability
- **SC-002**: User theme preference is remembered with 100% accuracy across app restarts
- **SC-003**: Export dialog displays all three format options and allows any combination to be selected
- **SC-004**: Exported files (txt/rtf/docx) are created with correct names in playlist-text directory within current directory
- **SC-005**: Clear-all behavior removes 100% of existing files in playlist-text directory before new export
- **SC-006**: Export completes for 100+ item playlists without performance degradation (under 2 seconds for all three formats)
- **SC-007**: Users understand audio editing features are planned (placeholder UI provides clear "Coming Soon" indication)
- **SC-008**: Audio editing framework documentation allows developers to add features without core rewrite (no architectural changes required to extend)

## Assumptions

- **Theme Storage**: User theme preference is stored in local config file (e.g., app settings directory or current directory config file) and is readable/writable without elevated permissions
- **Export Directory**: playlist-text directory is created in the app's current working directory (where MP3s are stored)
- **Format Implementation**: Export formats (txt/rtf/docx) use standard libraries or simple formatting (no complex styling); compactness is priority per spec v1
- **Audio Editing Scope**: Audio editing features (looping, volume, compression, etc.) are deferred to v2 or later; v1 app is for organization/export only
- **Platform Consistency**: Theme preference and export behavior are consistent across Windows, macOS, and Linux; no platform-specific variations expected
- **Persistence**: User preferences (theme, last export format selection) are auto-saved on change; no explicit "Save Preferences" step required
