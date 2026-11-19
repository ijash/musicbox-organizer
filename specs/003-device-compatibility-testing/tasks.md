# Tasks: Device Compatibility Testing

**Input**: Design documents from `/specs/003-device-compatibility-testing/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)

---

## Phase 1: Foundational (Entities & Data Model)

**Purpose**: Core report data structures

- [ ] T001 Implement CompatibilityReport and DeviceProfile in `src/models/compatibility.rs`
- [ ] T002 Export compatibility models in `src/models/mod.rs` and `src/lib.rs`

---

## Phase 2: User Story 1 - Lexicographical Byte-Order Sort Verification (Priority: P1) 🎯 MVP

**Goal**: Verify generated filenames strictly match raw ASCII byte order for any sequence length up to 17,576.

**Independent Test**: Generate 1,000 sequential `[XXYY-filename.mp3]` names; verify `sort()` preserves index order.

- [ ] T003 [P] [US1] Unit test for ASCII byte-order sequence sorting in `tests/unit/compatibility_tests.rs`
- [ ] T004 [US1] Implement sequence order verifier in `src/services/compatibility.rs`

---

## Phase 3: User Story 3 - ASCII-Only Validation & Sanitization (Priority: P2)

**Goal**: Catch non-ASCII/multibyte unicode characters and sanitize them for primitive players.

**Independent Test**: Provide unicode filenames (Cyrillic, emoji, smart quotes); verify detection and sanitization to ASCII.

- [ ] T005 [P] [US3] Unit test for ASCII character guard and sanitization in `tests/unit/compatibility_tests.rs`
- [ ] T006 [US3] Implement ASCII validation and sanitize_filename in `src/services/compatibility.rs`
- [ ] T007 [US3] Implement FAT32 255-character path length validator in `src/services/compatibility.rs`
- [ ] T008 Export CompatibilityService in `src/services/mod.rs`

---

## Phase 4: Polish & CLI Integration

**Purpose**: Provide standalone compatibility audit command and rebuild binary.

- [ ] T009 Add `musicbox check-compat <dir>` CLI subcommand in `src/main.rs`
- [ ] T010 Run full test suite and rebuild release binary per Constitution Principle VII
