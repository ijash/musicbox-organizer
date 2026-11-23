# Tasks: Audio DSP Editing Suite (Feature 008)

**Input**: Design documents from `specs/008-audio-dsp-editing/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `- [ ] [TaskID] [P?] [Story?] Description with file path`

- **[P]**: Parallelizable (distinct files, independent scope)
- **[Story]**: User story identifier ([US1], [US2], [US3], [US4])

---

## Phase 1: Setup & Early Binary-Size Gate

**Purpose**: Dependency configuration and strict early validation of the 2.5 MB binary size ceiling per Constitution Principle I.

- [X] T001 Move `tempfile = "3"` to `[dependencies]` and add `ebur128 = "0.1"` and `mp3lame-encoder = "0.1"` in `Cargo.toml`
- [X] T002 Compile release binary via `cargo build --release` and verify file size is strictly `< 2.5 MB` in `target/release/musicbox`

---

## Phase 2: Foundational (Blocking Infrastructure)

**Purpose**: Data model structures, atomic safe disk persistence, and tag preservation helpers.

- [X] T003 Define `DspJobConfig`, `DspProgress`, and `DspBatchReport` models per `data-model.md` in `src/services/audio_editing.rs`
- [X] T004 Implement atomic file writer helper using `tempfile::NamedTempFile::new_in` with verbatim ID3 tag preservation via `id3` in `src/services/audio_editing.rs`

---

## Phase 3: User Story 1 - Volume Normalization (Priority: P1)

**Goal**: Normalize audio tracks to target loudness (default `-14.0 dB` LUFS) with a peak-safe ceiling (`-1.0 dBTP`).

- [X] T005 [US1] Add unit tests for EBU R128 loudness calculation and true-peak gain clamping on synthetic PCM buffers in `src/services/audio_editing.rs`
- [X] T006 [US1] Implement `normalize_volume` on `NativeAudioProcessor` using `ebur128` and peak-safe scaling in `src/services/audio_editing.rs`

---

## Phase 4: User Story 2 - Silence Trimming (Priority: P1)

**Goal**: Trim leading and trailing silence below threshold (default `-45.0 dB`) with ~20ms RMS window and ~50ms safety padding.

- [X] T007 [US2] Add unit tests for windowed RMS silence detection, channel alignment, and padding retention on synthetic PCM buffers in `src/services/audio_editing.rs`
- [X] T008 [US2] Implement `trim_silence` on `NativeAudioProcessor` with boundary clamping and channel preservation in `src/services/audio_editing.rs`

---

## Phase 5: User Story 3 - Batch Processing & Pipeline Integration (Priority: P2)

**Goal**: Chain trimming and normalization in correct acoustic order (trim first, normalize second) with batch error isolation and zero orphaned temp files.

- [X] T009 [US3] Implement `process_track` pipeline (decode via `rodio::Decoder`, trim, normalize, encode via `mp3lame-encoder`, atomic write) in `src/services/audio_editing.rs`
- [X] T010 [US3] Add integration tests in `tests/integration_tests.rs` using copies of `assets/mp3-sample/` in `tempfile::tempdir()` verifying playable output, ID3 tag retention, and clean temp directories

---

## Phase 6: User Story 4 - Native GTK3 UI & Headless Harness (Priority: P2)

**Goal**: Discoverable DSP toolbar button, non-blocking modal dialog, background thread processing, and headless GUI test coverage.

- [X] T011 [P] [US4] Add `AppIcon::AudioTools` symbolic SVG data and button builder in `src/ui/icons.rs`
- [X] T012 [US4] Replace `Audio Tools (Soon)` placeholder with active DSP button and implement background worker DSP dialog in `src/ui/components/playlist_table.rs`
- [X] T013 [US4] Add headless GUI test for opening and cancelling DSP dialog via `tests/gui/run_gui_tests.sh` in `tests/gui_tests.rs`

---

## Phase 7: Polish & Quality Gates

**Purpose**: Verification gates, binary size confirmation, and documentation updates.

- [X] T014 Run complete test suite (`cargo test` and `./tests/gui/run_gui_tests.sh`) and resolve any regressions
- [X] T015 Run `cargo build --release` and verify final executable size is `< 2.5 MB` in `target/release/musicbox`
- [X] T016 Update `README.md` Changelog and Roadmap for Feature 008 per Constitution v1.7.0

---

## Dependencies & Execution Order

```
[Phase 1: Setup & Early Binary Gate] (T001 -> T002)
       │
       ▼
[Phase 2: Foundational] (T003, T004)
       │
       ├─────────────────────────┐
       ▼                         ▼
[Phase 3: US1 Normalize]    [Phase 4: US2 Trim]
   (T005 -> T006)             (T007 -> T008)
       │                         │
       └───────────┬─────────────┘
                   ▼
[Phase 5: US3 Batch Pipeline & Integration] (T009 -> T010)
                   │
                   ▼
[Phase 6: US4 GTK3 UI & Headless Harness] (T011 -> T012 -> T013)
                   │
                   ▼
[Phase 7: Polish & Quality Gates] (T014 -> T015 -> T016)
```

## Parallel Execution Opportunities

- T003 & T004 can be drafted in tandem.
- US1 (T005, T006) and US2 (T007, T008) test suites can run in parallel once Foundational phase completes.
- T011 (Icon definition in `src/ui/icons.rs`) can be implemented in parallel with DSP logic.
