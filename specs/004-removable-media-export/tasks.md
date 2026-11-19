# Tasks: Removable Media Export (Feature 004)

**Input**: Design documents from `/specs/004-removable-media-export/`

**Prerequisites**: plan.md (complete), spec.md (complete with clarifications), data-model.md (complete), research.md (complete)

**Organization**: Tasks grouped by user story (US1, US2, US3) for independent implementation and testing.

**Format**: `[TaskID] [P?] [Story?] Description with file path`
- **[P]**: Parallelizable (different files, no dependencies)
- **[Story]**: Which user story (US1, US2, US3)

---

## Phase 1: Setup (Project Initialization)

**Purpose**: Project structure, Cargo dependencies, platform detection

- [x] T001 Create project structure per plan.md: `src/{models,services,platform,ui}`, `tests/{unit,integration}`, update Cargo.toml
- [x] T002 [P] Add Cargo.toml dependencies: `uuid`, `chrono`, `serde`, platform-specific device enumeration (research.md recommendations)
- [x] T003 [P] Create lib.rs with module structure: mod models, mod services, mod ui, mod platform
- [x] T004 [P] Create .gitignore entries for target/, Cargo.lock, test artifacts

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure MUST complete before user stories can start

**⚠️ CRITICAL**: No user story work begins until this phase finishes.

### Data Model & Type Definitions

- [x] T005 [P] Implement Device struct in `src/models/device.rs` with all fields from data-model.md: id, name, capacity_bytes (>0), available_bytes, filesystem, mount_point, is_removable (=true), is_write_protected (=false), last_seen. Add validation: capacity_bytes > 0, is_removable=true, is_write_protected=false (cite: data-model.md Device)
- [x] T006 [P] Implement ExportJob struct in `src/models/export_job.rs` with ExportStatus enum (PENDING, FORMATTING, IN_PROGRESS, COMPLETED, FAILED, ABORTED). Add validation: reformat_enabled=true requires reformat_confirmed=true before formatting starts (cite: data-model.md ExportJob, ExportStatus)
- [x] T007 [P] Implement Track struct in `src/models/export_job.rs` with index (unique, sequential, 0-based), source_path, filename, xxyy_name, file_size_bytes (cite: data-model.md Track)
- [x] T008 [P] Implement ProjectFile struct in `src/models/project.rs` with PlaylistMetadata, tracks list, version field (must be "1.0"), parsing logic for plain-text .txt format (cite: data-model.md ProjectFile, Serialization)

### Naming Algorithm (XXYY)

- [x] T009 Implement XXYY base-26 naming logic in `src/services/naming.rs`: `track_index_to_xxyy(index)` → AAAA (0), AAAB (1), ..., ZZZZ (17575). Enforce max 17,576 tracks (cite: data-model.md Naming Sequence)
- [x] T010 [P] Implement inverse function `xxyy_to_track_index(xxyy)` for round-trip validation (cite: data-model.md Naming Sequence)
- [x] T011 Unit test XXYY naming: T011 [P] Add naming tests in `tests/unit/naming_tests.rs` with test cases from data-model.md (AAAA→0, AAAB→1, AAAZ→25, ABAA→26, ZZZZ→17575)

### Device Manager Abstraction

- [x] T012 Create platform abstraction layer in `src/services/device_manager.rs`: DeviceManager trait with methods enumerate_devices(), detect_write_protected(), validate_removable() (cite: data-model.md Device validation rules)
- [x] T013 [P] Implement Windows device enumeration in `src/services/platform/windows.rs` using Win32 APIs (research.md guide)
- [x] T014 [P] Implement macOS device enumeration in `src/services/platform/macos.rs` using iokit/DADiskArbitration (research.md guide)
- [x] T015 [P] Implement Linux device enumeration in `src/services/platform/linux.rs` using udisks2 or /sys/block (research.md guide)
- [x] T016 Create platform selector in `src/services/platform/mod.rs` with platform-specific conditional compilation

### File System Operations

- [x] T017 Implement file copy logic in `src/services/export_service.rs` with error handling for device disconnection, insufficient space, permission denied (cite: data-model.md Error Codes & State Recovery)
- [x] T018 Implement reformat logic in `src/services/format_service.rs` for FAT32 and exFAT (platform-native commands or lib per research.md) (cite: plan.md Constraints, spec.md FR-003)

**Checkpoint**: Foundation ready - user story implementation can begin in parallel

---

## Phase 3: User Story 1 - Select and Export Playlist to USB/SD Card (Priority: P1) 🎯 MVP

**Goal**: Users can select a connected USB/SD device and export organized playlist files with correct naming.

**Independent Test**: Connect USB device, use app to select device, trigger export, verify files appear with [XXYY-filename.mp3] naming in correct sequence.

### Tests for User Story 1

- [x] T019 [P] [US1] Unit test Device detection in `tests/unit/device_tests.rs`: mock enumerate_devices(), verify Device struct fields populated (id, name, capacity_bytes, is_removable, is_write_protected)
- [x] T020 [P] [US1] Unit test export service in `tests/unit/export_service_tests.rs`: test file copy, naming, error handling for partial exports (cite: data-model.md Error Codes & State Recovery)
- [x] T021 [US1] Integration test export workflow in `tests/integration/export_integration_tests.rs`: connect test USB, enumerate device, export 5-track playlist, verify [XXYY-filename.mp3] naming on device, check files are playable

### Implementation for User Story 1

- [x] T022 [P] [US1] Create export dialog UI in `src/ui/export_dialog.rs`: device list widget with name, capacity, filesystem, available space. Show message "No removable devices detected. Connect a USB drive or SD card to proceed." when list empty. Disable Export button if no devices (cite: spec.md FR-001, Edge Cases)
- [x] T023 [P] [US1] Implement device enumeration trigger in UI (on dialog open, on-demand refresh). Poll for device disconnection every 500ms with 5-second timeout for detection (cite: spec.md FR-009, SC-006)
- [x] T024 [US1] Implement export action in `src/services/export_service.rs`: read playlist tracks, validate device (capacity_bytes > 0, is_removable=true, is_write_protected=false), compute required space, warn if insufficient (cite: data-model.md Device validation, spec.md FR-005)
- [x] T025 [US1] Integrate device validation error messages: display "Device is internal (not removable). Select a USB or SD card instead." for non-removable, "Device is write-protected. Check the physical write-protect switch on the device." for write-protected, "Insufficient space. Device has X GB available, need Y GB for full playlist." for too small (cite: spec.md FR-006, FR-010, FR-005)
- [x] T026 [P] [US1] Implement file copy loop: for each track, generate XXYY name, copy source MP3 to device with [XXYY-filename.mp3] naming. On success, update ExportJob bytes_written, files_written (cite: data-model.md Track, File Layout)
- [x] T027 [US1] Implement progress display: update UI every 500ms (2 Hz minimum) with files written, percentage, elapsed time. Ensure max UI freeze ≤100ms (cite: spec.md SC-008, FR-012)
- [x] T028 [P] [US1] Implement disconnection detection: monitor device every 500ms, abort gracefully if device disappears within 5-second window. Display error message and allow retry (cite: spec.md FR-009, SC-006)
- [x] T029 [US1] Implement partial failure handling: if export fails mid-operation, leave partial files on device, return ExportJob to PENDING state, allow user to manually delete or reformat and retry (cite: data-model.md ExportJob state transitions, spec.md Edge Cases)
- [x] T030 [P] [US1] Implement state machine transitions for export: PENDING → IN_PROGRESS (user confirms export), IN_PROGRESS → COMPLETED (all files copied), any state → FAILED (device error), any state → ABORTED (user cancels) (cite: data-model.md ExportJob state transitions)
- [x] T031 [US1] Add error logging for export operations: log device info, file count, bytes written, errors with timestamps (cite: plan.md Performance Goals)
- [x] T032 [P] [US1] Implement eject button after successful export: call platform-native eject command (Win32/iokit/udisks2) to safely unmount device (cite: spec.md FR-013)

**Checkpoint**: User Story 1 complete and independently testable - device selection and export workflow fully functional

---

## Phase 4: User Story 2 - Reformat Device Before Export with Warning (Priority: P1)

**Goal**: Users can optionally reformat USB/SD device before export with explicit safety confirmation.

**Independent Test**: Select device, check "Reformat device" checkbox, see modal warning with Cancel/Confirm buttons only (no X), click Confirm, verify device reformatted to FAT32/exFAT and files exported cleanly.

### Tests for User Story 2

- [x] T033 [P] [US2] Unit test reformat logic in `tests/unit/export_service_tests.rs`: test format_service.rs with mock device, verify reformat succeeds, fails gracefully on errors (cite: data-model.md ExportJob state machine FORMATTING state)
- [x] T034 [P] [US2] Unit test reformat prerequisite validation: test ExportJob requires reformat_confirmed=true when reformat_enabled=true before format starts (cite: data-model.md ExportJob validation)
- [x] T035 [US2] Integration test reformat workflow: select device, check reformat checkbox, confirm warning, verify device emptied and export proceeds, verify no orphaned data remains (cite: spec.md FR-003, User Story 2 Acceptance Scenarios)

### Implementation for User Story 2

- [x] T036 [P] [US2] Add "Reformat device" checkbox to export dialog in `src/ui/export_dialog.rs`. Unchecked by default. When checked, enables warning dialog trigger (cite: spec.md FR-003, two-step confirmation)
- [x] T037 [US2] Implement reformat warning modal dialog in `src/ui/warnings.rs`: modal blocks all app interaction, displays device name and data-loss message. Include Cancel and Confirm buttons only (NO X close button). User must click one button to proceed (cite: spec.md FR-004, SC-002)
- [x] T038 [US2] Implement state machine transitions for reformat: PENDING → FORMATTING (user confirms reformat warning), FORMATTING → IN_PROGRESS (reformat succeeds), FORMATTING → FAILED (reformat fails). On cancel: return to PENDING (cite: data-model.md ExportJob state transitions, spec.md Clarifications Q3)
- [x] T039 [P] [US2] Implement reformat prerequisite logic: ExportJob.reformat_confirmed must be true before starting format. Add validation: reformat_enabled=true requires reformat_confirmed=true (cite: data-model.md ExportJob validation)
- [x] T040 [US2] Call format_service.rs with device ID and target filesystem (FAT32 or exFAT per plan.md). Wrap in error handling: if format fails, display error, return ExportJob to PENDING, allow retry (cite: spec.md FR-003, data-model.md Error Codes)
- [x] T041 [P] [US2] Implement two-step confirmation UX: (1) User checks checkbox, (2) User clicks Export, (3) Modal warning appears, (4) User clicks Confirm or Cancel. Prevents accidental reformat (cite: spec.md FR-003, clarification Q3)
- [x] T042 [US2] After reformat completes successfully, automatically transition to export (FORMATTING → IN_PROGRESS). Copy files to freshly formatted device (cite: spec.md User Story 2 Acceptance Scenario 3)

**Checkpoint**: User Story 2 complete - reformat with two-step confirmation + warning modal working independently

---

## Phase 5: User Story 3 - Distinguish "Save" (Local Project) vs "Export" (to Media) (Priority: P2)

**Goal**: Users understand the difference between saving a project file locally and exporting to removable media. UI clearly separates workflows.

**Independent Test**: Save project to local .txt file, export playlist to USB, verify project file persists locally and is independent of export; return to app, verify project reloads and export did not modify it.

### Tests for User Story 3

- [x] T043 [P] [US3] Unit test ProjectFile parsing in `tests/unit/project_tests.rs`: test load .txt file with [Metadata] and [Tracks] sections, verify version "1.0", tracks sequential 0-based indices, paths parseable (cite: data-model.md ProjectFile, Parsing)
- [x] T044 [P] [US3] Unit test ProjectFile serialization: test write ProjectFile to .txt, verify format matches spec, round-trip (load, save, load again), verify data unchanged (cite: data-model.md ProjectFile, Serialization)
- [x] T045 [US3] Integration test Save vs Export workflows: save project locally, export to USB, verify project file exists locally after export, verify USB contains only MP3 files (no project metadata), return to app and reload project, verify project intact and independent of export result (cite: spec.md FR-011, User Story 3)

### Implementation for User Story 3

- [x] T046 [P] [US3] Create separate Save button in main UI window (not export dialog). Label clearly as "Save Project" vs "Export Playlist" (cite: spec.md FR-011, User Story 3)
- [x] T047 [P] [US3] Implement ProjectFile.load() in `src/models/project.rs`: parse .txt file from current working directory (or user-specified path), populate PlaylistMetadata, load tracks from [Tracks] section (cite: data-model.md ProjectFile, Parsing)
- [x] T048 [P] [US3] Implement ProjectFile.save() in `src/models/project.rs`: serialize to .txt format with [Metadata] and [Tracks] sections, version "1.0", human-readable. Save to current working directory by default or File > Save As override (cite: data-model.md ProjectFile, Serialization, spec.md Assumptions)
- [x] T049 [US3] Add "File > Save Project As" menu item in main window for user-specified path override (cite: spec.md Assumptions)
- [x] T050 [US3] Ensure Save workflow does NOT affect export state: saving project does not clear device selection, export mode, or any export UI state (cite: spec.md FR-011, SC-007)
- [x] T051 [US3] Ensure Export workflow does NOT affect project state: exporting playlist does not modify project file, does not change project file location, does not mark project as "exported" (cite: spec.md FR-011, SC-007)
- [x] T052 [P] [US3] On app startup, auto-load project file from current working directory if it exists. Display project name in window title. If no project exists, allow new project creation (cite: spec.md User Story 3 Acceptance Scenario 3)
- [x] T053 [US3] Add UI documentation/help: explain difference between "Save Project" (persists locally for future editing) vs "Export" (one-way delivery to portable media). Prevent user confusion (cite: spec.md User Story 3 why priority)

**Checkpoint**: User Story 3 complete - Save and Export are clearly separated workflows, independent of each other

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final integration, documentation, validation, performance tuning

- [x] T054 [P] Run all unit tests in `tests/unit/`: naming_tests.rs, device_tests.rs, export_service_tests.rs, project_tests.rs. Verify 100% pass
- [x] T055 [P] Run all integration tests in `tests/integration/export_integration_tests.rs`. Requires physical test device or mock. Verify full workflows pass
- [x] T056 [P] Code cleanup: remove debug print statements, ensure error messages are user-facing (not debug symbols), apply rustfmt, run clippy lints
- [x] T057 Documentation: update README.md with user guide (Select Device, Export, Reformat, Save Project workflows). Include screenshot mockups and step-by-step instructions (cite: spec.md User Stories 1–3)
- [x] T058 [P] Platform validation: test on Windows, macOS, Linux (or at least one of each). Verify device enumeration works, export succeeds, progress updates visible (cite: spec.md SC-005 cross-platform)
- [x] T059 Run quickstart.md validation: execute manual test scenarios (connect device, enumerate, export, verify files, disconnect). Verify all pass (cite: plan.md)
- [x] T060 Performance tuning: profile export loop, ensure progress updates every 500ms, max UI freeze ≤100ms (cite: spec.md SC-008)
- [x] T061 [P] Binary size check: verify compiled binary <50MB (cite: plan.md Constraints, Principle I Lightweight First)
- [x] T062 Git commits: ensure all commits follow Conventional Commits v1.0.0 format with Co-authored-by trailer. Final commit: `feat(004): complete removable media export feature` (cite: plan.md Principle VI)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3–5)**: All depend on Foundational completion
  - US1, US2 can run in parallel (both P1)
  - US3 starts after US1/US2 (P2)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### Within Each User Story

- Tests (T019–T021 for US1) should be written and fail BEFORE implementation
- Models → Services → UI → Integration
- Each story independently testable before moving to next

### Parallel Opportunities

**Phase 1**: T002, T003, T004 can run in parallel (different files)

**Phase 2**: T005–T008 (models), T013–T015 (platform backends) can run in parallel

**Phase 2.5** (after models complete): T009–T010 (naming), T017–T018 (file ops) can run in parallel

**Phase 3 (US1)**: T019–T021 (tests), T022–T023 (UI), T024–T026 (export logic), T028 (disconnection), T032 (eject) can run in parallel as long as they don't conflict on files

**Phase 4 (US2)**: Once US1 complete, T033–T035 (tests), T036–T037 (UI), T038–T040 (state machine) can run in parallel

**Phase 5 (US3)**: Once US1/US2 complete, T043–T045 (tests), T046–T053 (save/load logic) can run in parallel

**Phase 6**: T054, T055, T056, T058, T061 can run in parallel (independent validation tasks)

---

## Parallel Example: Phase 2 Foundational

```bash
# Launch all model tasks together:
Task T005: Device struct
Task T006: ExportJob struct
Task T007: Track struct
Task T008: ProjectFile struct

# Wait for models → launch all platform backends together:
Task T013: Windows enumeration
Task T014: macOS enumeration
Task T015: Linux enumeration

# Launch naming + file ops together:
Task T009: XXYY to_index
Task T010: index_to_XXYY
Task T017: File copy logic
Task T018: Reformat logic
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks everything)
3. Complete Phase 3: User Story 1 (Select & Export)
4. **STOP and VALIDATE**: Test export workflow independently
5. Deploy/demo MVP

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add US1 → Test independently → Deploy (MVP!)
3. Add US2 → Test independently → Deploy (Reformat)
4. Add US3 → Test independently → Deploy (Save vs Export distinction)
5. Each story adds value without breaking previous stories

---

## Task Summary

- **Total Tasks**: 62 (T001–T062)
- **Setup**: 4 tasks
- **Foundational**: 14 tasks
- **User Story 1**: 14 tasks (3 tests + 11 implementation)
- **User Story 2**: 7 tasks (3 tests + 4 implementation)
- **User Story 3**: 8 tasks (3 tests + 5 implementation)
- **Polish**: 9 tasks

**Parallelizable Tasks [P]**: ~28 tasks can run in parallel (different files, no shared dependencies within phases)

**MVP Scope**: Phase 1 + Phase 2 + Phase 3 (Select & Export) = 32 tasks for minimum viable product

---

## Phase 7: Convergence

- [x] T063 Implement is_sufficient_space on WindowsDeviceManager and MacOSDeviceManager per Constitution II (contradicts) - CRITICAL
- [x] T064 Add missing unit and integration test suites under tests/unit/ and tests/integration/ per Constitution V (missing) - CRITICAL
- [x] T065 Create export dialog UI in src/ui/export_dialog.rs per FR-001, FR-002, FR-012, SC-001, SC-008 (missing)
- [x] T066 Implement reformat warning modal dialog in src/ui/warnings.rs per FR-004, SC-002 (missing)
- [x] T067 Enforce [XXYY-filename.mp3] naming convention in src/services/export_service.rs per FR-008, SC-003 (partial)
- [x] T068 Add mid-operation device disconnection polling to src/services/export_service.rs per FR-009, SC-006 (missing)
- [x] T069 Implement device ejection trigger and UI action per FR-013 (missing)
- [x] T070 Connect format_service to export_service for reformat-then-export pipeline per FR-003, US2/AC3 (missing)
- [x] T071 Implement ProjectFile::load and ProjectFile::save filesystem methods in src/models/project.rs per US3/AC1, T047, T048 (missing)
- [x] T072 Implement main window Save Project vs Export Playlist distinction in src/main.rs and UI per FR-011, US3 (missing)
- [x] T073 Allow ExportJob::fail state transition from PENDING in src/models/export_job.rs per FR-005, FR-006, data-model.md (contradicts)

