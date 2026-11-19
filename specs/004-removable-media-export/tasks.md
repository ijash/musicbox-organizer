# Tasks: Removable Media Export (004)

**Input**: Design documents from `/specs/004-removable-media-export/`

**Prerequisites**: plan.md (tech stack, structure), spec.md (3 user stories), data-model.md (entities, constraints), contracts/ (UI spec), research.md (tech decisions)

**Tests**: Integration tests required per quickstart.md validation scenarios; unit tests for naming algorithm and field validation

**Organization**: Tasks grouped by user story to enable independent implementation and testing

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Parallelizable (different files, no dependencies)
- **[Story]**: User story tag (US1, US2, US3)
- **File paths**: Exact locations per plan.md structure (src/models/, src/services/, src/platform/, src/ui/, tests/)

---

## Phase 1: Setup (Project Initialization)

**Purpose**: Cargo workspace, dependencies, base structure

- [ ] T001 Initialize Cargo project with base dependencies (windows, cocoa, objc, gtk, serde, uuid, metaflac) in Cargo.toml
- [ ] T002 [P] Create directory structure: src/{models,services,platform,ui}/, tests/{unit,integration,contract}/
- [ ] T003 [P] Create src/lib.rs with module declarations (models, services, platform, ui)
- [ ] T004 [P] Create src/main.rs entry point with app initialization placeholder
- [ ] T005 Setup linting and formatting (rustfmt, clippy configs)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types, error handling, utilities that all user stories depend on

⚠️ **CRITICAL**: No user story work can begin until this phase completes

- [ ] T006 [P] Create common error types in src/error.rs (DeviceError, ExportError, FormatError variants)
- [ ] T007 [P] Create FileSystem enum (FAT32, exFAT, Unknown) in src/models/mod.rs
- [ ] T008 [P] Create ExportStatus enum (PENDING, FORMATTING, IN_PROGRESS, COMPLETED, FAILED, ABORTED) in src/models/mod.rs
- [ ] T009 Implement base-26 naming algorithm in src/services/naming.rs with track_index_to_xxyy() and xxyy_to_track_index() functions; cite data-model.md naming algorithm; include constraint: index <= 17575 (base-26 limit for 4 chars)
- [ ] T010 [P] Create unit tests for naming algorithm (AAAA/AAAB/AAAZ/ABAA/ZZZZ cases) in tests/unit/naming_tests.rs
- [ ] T011 Create Device struct in src/models/device.rs with validation: is_removable=true, is_write_protected=false, capacity_bytes > 0, available_bytes <= capacity_bytes (cite data-model.md Device constraints)
- [ ] T012 [P] Create Track struct in src/models/track.rs with fields per data-model.md (index, source_path, filename, xxyy_name, file_size_bytes); validate index 0-based sequential within playlist
- [ ] T013 [P] Create ProjectFile struct in src/models/project.rs with version="1.0", metadata, tracks (cite data-model.md ProjectFile format)
- [ ] T014 [P] Create ExportJob struct in src/models/export_job.rs with state transitions (cite data-model.md ExportStatus state machine); validate bytes_written <= total_bytes
- [ ] T015 Implement ProjectFile .txt parser/serializer in src/services/project_file.rs (line-delimited format per data-model.md); handle comments, sections, key-value pairs
- [ ] T016 [P] Create platform abstraction trait DeviceManager in src/services/device_manager.rs with enumerate_devices() and detect_disconnection() methods
- [ ] T017 Create Windows device enumeration backend in src/platform/windows.rs using Win32 SetupAPI (SetupDiGetClassDevs, SetupDiEnumDeviceInfo) per research.md
- [ ] T018 Create macOS device enumeration backend in src/platform/macos.rs using DADiskArbitration via objc crate per research.md
- [ ] T019 Create Linux device enumeration backend in src/platform/linux.rs using udisks2 DBus (or /sys/block fallback) per research.md
- [ ] T020 Implement DeviceManager factory in src/services/device_manager.rs to select platform-specific backend at compile time

**Checkpoint**: Foundation complete. All user stories can now proceed in parallel or sequence.

---

## Phase 3: User Story 1 - Device Enumeration & List Display (Priority: P1) 🎯 MVP

**Goal**: Users can see connected USB/SD devices and select one for export.

**Independent Test**: Connect USB drive, open Export dialog, verify device appears in list with correct name, capacity, filesystem, available space.

### Tests for User Story 1

- [ ] T021 [US1] Create device list mock in tests/unit/device_mock.rs with fake devices (Kingston USB, SanDisk SD) for testing enumeration without live hardware
- [ ] T022 [P] [US1] Unit test Device struct validation in tests/unit/device_tests.rs (capacity > 0, available <= capacity, is_removable=true, is_write_protected=false)
- [ ] T023 [P] [US1] Unit test device list filtering (reject internal drives, accept only removable) in tests/unit/device_tests.rs
- [ ] T024 [US1] Contract test for export dialog in tests/contract/export_dialog_tests.rs: device list renders, device properties (name, capacity, filesystem) display correctly
- [ ] T025 [US1] Integration test device enumeration in tests/integration/device_enumeration_tests.rs: enumerate() returns >=1 device if any connected, empty list if none

### Implementation for User Story 1

- [ ] T026 [US1] Implement DeviceManager::enumerate_devices() in platform backends to return Vec<Device> per research.md device detection pattern (polling 500ms on Linux/Win, event-driven on macOS)
- [ ] T027 [US1] Create export_service.rs stub in src/services/export_service.rs with validate_device() method: reject internal drives, check is_removable=true and is_write_protected=false (cite spec.md FR-006, SC-004)
- [ ] T028 [P] [US1] Create export dialog UI in src/ui/export_dialog.rs: device list widget, device name/capacity/filesystem/available space display per contracts/export-dialog-ui.md (cite spec.md FR-002)
- [ ] T029 [P] [US1] Implement device list refresh button: call DeviceManager::enumerate_devices() on click, update UI list
- [ ] T030 [US1] Implement device selection (user clicks device): set selected_device_id, display export options panel (cite contracts/export-dialog-ui.md)
- [ ] T031 [US1] Implement "No devices" empty state in export dialog: show message "No removable devices detected. Connect a USB drive or SD card to proceed." and disable Export button (cite spec.md FR-001)
- [ ] T032 [US1] Implement device disconnection detection: call DeviceManager::detect_disconnection() periodically (500ms poll or event-driven per platform); if device missing >5s, show alert (cite spec.md FR-009, SC-006)
- [ ] T033 [US1] Add logging for device enumeration (device count, names) for debugging

**Checkpoint**: User Story 1 complete. Users can open Export dialog, see connected devices, select one. Export button ready (disabled until device selected).

---

## Phase 4: User Story 2 - Reformat & Export with Warning (Priority: P1)

**Goal**: Users can reformat device before export with safety warning; or export directly without reformat.

**Independent Test**: Connect USB drive, enable "Reformat & Export" option, verify warning dialog appears, cancel → device unchanged, confirm → device reformatted, files written.

### Tests for User Story 2

- [ ] T034 [P] [US2] Unit test reformat warning acceptance: user must explicitly confirm reformat (not just dismiss dialog) in tests/unit/export_tests.rs
- [ ] T035 [P] [US2] Unit test space estimation: calculate required_space = sum(track sizes), compare with device.available_bytes, warn if insufficient (cite spec.md FR-005)
- [ ] T036 [US2] Unit test file naming: exported files follow [XXYY-filename.mp3] convention (cite spec.md FR-008, data-model.md naming); XXYY sequence correct
- [ ] T037 [P] [US2] Unit test track copying to device: mock file I/O, verify files copied with correct xxyy_name, file count matches (cite spec.md FR-003, SC-003)
- [ ] T038 [US2] Contract test reformat warning dialog in tests/contract/reformat_warning_tests.rs: warning shows device name, data-loss message, requires explicit confirmation
- [ ] T039 [P] [US2] Integration test export without reformat in tests/integration/export_no_reformat_tests.rs: copy files to device, verify files present, correct naming, playable (cite quickstart.md scenario 3)
- [ ] T040 [US2] Integration test reformat + export in tests/integration/reformat_export_tests.rs: reformat to FAT32, copy files, verify device empty except playlist (cite quickstart.md scenario 4)

### Implementation for User Story 2

- [ ] T041 [P] [US2] Create reformat warning dialog in src/ui/warnings.rs: show device name, "All data will be PERMANENTLY DELETED", require explicit confirmation button (cite spec.md FR-004, SC-002, contracts/export-dialog-ui.md)
- [ ] T042 [US2] Add reformat option checkbox in export dialog src/ui/export_dialog.rs; if checked, show filesystem selector (FAT32/exFAT) (cite spec.md FR-007, contracts/export-dialog-ui.md)
- [ ] T043 [US2] Implement space estimation in src/services/export_service.rs: calculate playlist total_bytes, compare with device available_bytes; warn if insufficient (cite spec.md FR-005)
- [ ] T044 [US2] Implement format_device() method in src/services/format_service.rs: call platform-native tools (format.exe on Win, diskutil on macOS, mkfs.fat on Linux) per research.md (cite spec.md FR-003)
- [ ] T045 [US2] Implement progress panel in src/ui/export_dialog.rs: show files written/total, bytes written/total, elapsed time, estimated remaining, 500ms update interval (cite spec.md FR-012, SC-008)
- [ ] T046 [P] [US2] Implement file copy logic in src/services/export_service.rs: copy Track.source_path to device with xxyy_name (AAAA, AAAB, ...), handle errors, track bytes_written (cite data-model.md ExportJob.bytes_written constraint: <= total_bytes)
- [ ] T047 [US2] Implement write-protect detection in src/services/export_service.rs: check Device.is_write_protected before write attempt; display error if true (cite spec.md FR-010)
- [ ] T048 [US2] Implement export completion dialog in src/ui/export_dialog.rs: show success/failure, files written count, time taken, space used (cite contracts/export-dialog-ui.md)
- [ ] T049 [US2] Add "Cancel Export" button: abort mid-operation, leave partial files on device (cite spec.md FR-009, edge cases)
- [ ] T050 [US2] Implement disconnection handling during export: detect mid-operation (via DeviceManager::detect_disconnection()), abort, show "Device Disconnected" alert with reconnect option (cite spec.md FR-009, quickstart.md scenario 5)

**Checkpoint**: User Stories 1 and 2 complete. MVP feature: users can enumerate devices, export with or without reformat, see progress, handle errors.

---

## Phase 5: User Story 3 - Save Project vs Export (Priority: P2)

**Goal**: Users can save local project file independently of export; clear UI distinction prevents confusion.

**Independent Test**: Save project to current dir as .txt, export to USB without affecting saved file; reopen app, project loads from saved file independent of export.

### Tests for User Story 3

- [ ] T051 [P] [US3] Unit test ProjectFile serialization in tests/unit/project_file_tests.rs: .txt format, metadata (name, artist, album), tracks with index/path/artist/title/duration; deserialize and verify roundtrip
- [ ] T052 [P] [US3] Unit test project file path handling: save to current working dir by default, allow File > Save As override (cite spec.md FR-011)
- [ ] T053 [US3] Unit test save/export independence: save project → export to device → save project again → verify local file unchanged by export (cite spec.md FR-011, SC-007)
- [ ] T054 [P] [US3] Unit test project state restoration: close app, reopen, load saved project file from disk, verify playlist state restored (cite spec.md FR-011)
- [ ] T055 [US3] Contract test Save button in tests/contract/save_project_tests.rs: separate from Export, saves .txt file to current dir with metadata
- [ ] T056 [US3] Integration test save/export workflow in tests/integration/save_export_workflow_tests.rs: organize playlist, save locally, export to USB, verify device has only .mp3 files and local project file is .txt with metadata (cite quickstart.md scenario 3, spec.md FR-011)

### Implementation for User Story 3

- [ ] T057 [US3] Create Save button in src/ui/main_window.rs: File > Save, File > Save As dialogs (cite spec.md FR-011, contracts/export-dialog-ui.md)
- [ ] T058 [US3] Implement save_project() in src/services/project_file.rs: write ProjectFile struct to .txt in current dir (or user-specified path), format per data-model.md (cite spec.md FR-011)
- [ ] T059 [US3] Implement load_project() in src/services/project_file.rs: read .txt file, deserialize to ProjectFile struct, restore playlist state in UI (cite spec.md FR-011)
- [ ] T060 [US3] Update export_service.rs to ensure Save and Export are independent: export populates ExportJob and writes .mp3 files only to device, does NOT modify local project file (cite spec.md FR-011, SC-007)
- [ ] T061 [US3] Add UI labels to distinguish Save ("Save project locally for editing") vs Export ("Write playlist to USB/SD") in main menu and dialogs (cite spec.md FR-011, contracts/export-dialog-ui.md)
- [ ] T062 [US3] Implement File > Save As dialog: allow user to override project file path (cite spec.md assumptions)

**Checkpoint**: All three user stories complete. Full feature: enumerate → select device → export with/without reformat → save project locally independently.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements, validation, edge cases

- [ ] T063 [P] Run quickstart.md validation scenario 1 (device enumeration): connect 1 device, verify lists correctly
- [ ] T064 [P] Run quickstart.md validation scenario 6 (write-protect / full disk): verify errors detected and blocked
- [ ] T065 [P] Run quickstart.md validation scenario 7 (multi-device): connect 2 devices, export to one, verify other untouched
- [ ] T066 Implement "Eject Device" button after export: safe OS-specific eject command (cite spec.md FR-013)
- [ ] T067 [P] Performance test: export 100 MB on USB 2.0, measure time, target <3 min for 30s user workflow (cite spec.md SC-001, plan.md performance goals)
- [ ] T068 [P] Code cleanup: extract reusable utilities, apply clippy suggestions, refactor duplicates
- [ ] T069 [P] Documentation: add doc comments to public functions, update README with feature overview
- [ ] T070 Security audit: validate file paths (no directory traversal), reject non-removable devices, verify write-protect checks
- [ ] T071 Cross-platform validation: test on Windows, macOS, Linux with >=3 device types: USB 2.0, USB 3.0, SD card reader (cite spec.md SC-005, constitution II: Cross-Platform Native)
- [ ] T072 Accessibility audit: export dialog keyboard navigation (Tab, Enter, Esc), screen reader support for device list and progress

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - **BLOCKS all user stories**
- **User Stories (Phase 3–5)**: All depend on Foundational phase completion
  - US1 (device enumeration) → US2 (export/reformat) → US3 (save project)
  - Can run sequentially (MVP: US1+US2, then US3)
  - Or in parallel if team capacity allows (all use separate UI modules, platform backends already abstracted)
- **Polish (Phase 6)**: Depends on all user stories complete

### User Story Dependencies

- **US1 (Device List)**: No story dependencies; depends on Phase 2 (Foundational)
- **US2 (Export/Reformat)**: Depends on US1 (device already selected); shares export_service
- **US3 (Save Project)**: Independent of US1/US2; can run in parallel; shares ProjectFile model

### Parallel Opportunities

**Within Phase 2 (Foundational)**:
- T007, T008, T010, T012, T013, T016, T021, T023, T035 can run in parallel (different modules)

**Within Phase 3 (US1)**:
- T026 (backend enumeration), T028 (UI dialog), T029 (refresh logic) can be done in parallel by different devs

**Within Phase 4 (US2)**:
- T041 (warning dialog), T043 (space estimation), T046 (file copy) can start in parallel (different files)

**Within Phase 5 (US3)**:
- T057 (Save button), T058 (serialize), T059 (deserialize) can start in parallel

**Between Stories**:
- Once Foundational (Phase 2) completes, Developer A can work on US1, Developer B on US3 (US2 depends on US1 complete), while Developer C starts cross-platform testing

### Parallel Example: Phase 3 (US1)

```bash
# After Phase 2 (Foundational) completes:
Task T021: Create device list mock
Task T028: Create export dialog UI
Task T026: Implement platform backends
# All run in parallel; UI ready when backends done, tests mock devices for CI
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2)

1. Complete Phase 1: Setup ✓
2. Complete Phase 2: Foundational (CRITICAL) ✓
3. Complete Phase 3: User Story 1 (device list) ✓
4. Complete Phase 4: User Story 2 (export/reformat) ✓
5. **STOP and VALIDATE**: Run quickstart scenarios 1–5 (device enum, export, reformat, disconnection, multi-device)
6. **Deploy MVP**: Users can export playlists to USB/SD with reformat and disconnection handling

### Incremental Delivery

After MVP (US1+US2) validated:
7. Complete Phase 5: User Story 3 (save project)
8. Complete Phase 6: Polish & testing
9. Deploy full feature

### Parallel Team Strategy (If 3+ Developers)

- **Dev A** (Phase 3): Device enumeration + UI
- **Dev B** (Phase 5): Save project (independent of export)
- **Dev C** (Phase 6): Testing, cross-platform validation, documentation
- **All**: Phase 2 (Foundational) together first, then split

---

## Notes

- All file paths exact; match plan.md structure (src/models/, src/services/, src/platform/, src/ui/, tests/unit/contract/integration/)
- Constraints from data-model.md cited explicitly in tasks (e.g., "capacity_bytes > 0")
- Tests marked [P] can run in parallel
- Export/Reformat phases intentionally combined as US2 (users do both together per spec)
- Save/Export separated (US3 independent) to allow parallel development
- Disconnection detection relies on Phase 2 DeviceManager foundation; US1/US2 use it directly
- No feature-creep: v1 scope = FAT32/exFAT only, local project files, no NTFS/cloud (per research.md and constitution)
- Validation requires real USB/SD devices or mocks (Phase 6); CI can use mocks from T021

---

## Constrained-Field Citations (data-model.md)

| Field | Task | Location |
|-------|------|----------|
| Device.capacity_bytes > 0 | T011 | src/models/device.rs validation |
| Device.available_bytes <= capacity_bytes | T011 | src/models/device.rs validation |
| Device.is_removable = true | T027 | src/services/export_service.rs validate_device() |
| Device.is_write_protected = false | T027, T047 | export_service.rs checks |
| ExportJob.bytes_written <= total_bytes | T046 | src/services/export_service.rs file copy loop |
| Track.index 0-based sequential | T012 | src/models/track.rs validation |
| ProjectFile.version = "1.0" | T013 | src/models/project.rs |
| FileSystem enum (FAT32, exFAT, Unknown) | T007 | src/models/mod.rs |
| Naming: base-26 AAAA–ZZZZ | T009, T036 | src/services/naming.rs + tests |

---

## Total Task Count

- **Setup (Phase 1)**: 5 tasks
- **Foundational (Phase 2)**: 15 tasks
- **User Story 1 (Phase 3)**: 12 tasks
- **User Story 2 (Phase 4)**: 18 tasks
- **User Story 3 (Phase 5)**: 6 tasks
- **Polish (Phase 6)**: 10 tasks
- **TOTAL**: 66 tasks

## Coverage

- All 3 user stories (US1, US2, US3) covered with dedicated phases
- All entities from data-model.md have implementing tasks
- All constrained fields explicitly cited
- All quickstart scenarios mapped to test tasks (scenarios 1–7 in phases 3–6)
- All FR/SC from spec.md mapped to tasks

## MVP Scope

**Minimum Viable Product = Phase 1 + Phase 2 + Phase 3 + Phase 4** (46 tasks)
- Users can: enumerate USB/SD devices, export playlists, reformat with warning, handle disconnection
- Not included in MVP: local project save (US3), polish, full cross-platform testing
- Suggested MVP stop point: After T050 validation (Phase 4 complete)
