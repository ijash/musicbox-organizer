# Feature 008: Audio DSP Editing Suite

## Overview

Replace `PlaceholderAudioProcessor` in `src/services/audio_editing.rs` with a production pure-Rust implementation providing **volume normalization** and **silence trimming** for MP3 files. Add minimal GTK3 UI integration so users can apply DSP operations to selected playlist tracks without leaving the app.

This is the first v0.2.0 feature, evolving beyond the v0.1.0 "preview-only" audio constraint.

## Context

The `AudioProcessor` trait already defines the interface (`normalize_volume`, `trim_silence`, `is_available`). The placeholder returns errors. This feature implements the real DSP pipeline using `rodio::Decoder` for PCM decoding (already in `Cargo.toml`) plus minimal new crates for loudness analysis and MP3 encoding.

## Clarifications

### Session 2026-11-21
- Q: Should the DSP dialog replace the existing "Audio Tools (Soon)" placeholder button and add one `AudioTools` SVG to `AppIcon`? → A: Yes — replace placeholder, add single `AudioTools` variant to `AppIcon` in `src/ui/icons.rs`.
- Q: What MP3 encoding quality and binary size fallback? → A: Match source bitrate (read original MP3 bitrate, re-encode at same rate). Fallback: pure-Rust MP3 frame manipulation if `mp3lame-encoder` pushes binary over 2.5 MB.
- Q: How should DSP edits handle undo/safety? → A: Overwrite in place after modal dialog confirmation. Push pre-edit playlist state into the existing in-memory undo stack (10-step limit) for session rollback. Audio sample modifications overwrite in place atomically with zero disk backup copies (Constitution I & IV).
- Q: Pipeline order when both operations enabled, and batch error behavior? → A: Trim first → Normalize second (silence doesn't skew LUFS). Skip failed files, report per-file errors in completion summary.
- Q: Format scope and tag preservation? → A: MP3 only, gracefully skip non-MP3 tracks. Preserve ALL ID3v1/v2 frames verbatim including APIC album art.

## User Stories

### US1: Volume Normalization
**As a** playlist organizer,
**I want to** normalize the volume of selected MP3 tracks to a consistent loudness level,
**so that** playback volume is uniform across all tracks on portable devices.

**Acceptance Criteria:**
- AC1: User can select one or more tracks and trigger "Normalize Volume" from toolbar DSP button.
- AC2: Default target is -14.0 dB LUFS with -1.0 dBTP true-peak ceiling.
- AC3: User can adjust target dB via input field before applying.
- AC4: Normalization applies peak-safe linear gain without clipping.
- AC5: Silent or empty audio files are safely skipped (no-op) without error.
- AC6: ID3 tags are preserved after processing.
- AC7: Output remains standard CBR/VBR MP3 playable on primitive devices.

### US2: Silence Trimming
**As a** playlist organizer,
**I want to** trim leading and trailing silence from selected MP3 tracks,
**so that** tracks start and end cleanly without dead air.

**Acceptance Criteria:**
- AC1: User can select one or more tracks and trigger "Trim Silence" from toolbar DSP button.
- AC2: Default silence threshold is -45.0 dB with ~20 ms RMS window and ~50 ms safety padding.
- AC3: User can adjust threshold dB via input field before applying.
- AC4: Trimming operates on full channel frame boundaries.
- AC5: Entirely silent tracks are safely skipped (no-op) without error.
- AC6: ID3 tags are preserved after processing.
- AC7: Output remains standard CBR/VBR MP3 playable on primitive devices.

### US3: Batch Processing with Progress
**As a** playlist organizer,
**I want to** apply DSP operations to multiple selected tracks at once,
**so that** I can process an entire playlist efficiently.

**Acceptance Criteria:**
- AC1: Batch operations run on a background thread; GTK main loop never freezes.
- AC2: Progress indication shows per-file status during batch processing.
- AC3: Errors on individual files are reported without aborting the batch.
- AC4: Zero leftover temporary files on disk after success or failure (Constitution IV).
- AC5: Undo support or backup/rollback available per Constitution III.

### US4: DSP Dialog UI
**As a** user,
**I want** a clean, discoverable DSP action entry in the toolbar,
**so that** I can find and use audio processing features easily.

**Acceptance Criteria:**
- AC1: Toolbar has a DSP button with embedded symbolic SVG icon.
- AC2: Clicking opens a compact GTK3 dialog with checkboxes for Normalize and Trim.
- AC3: Each operation has an adjustable dB input field with sensible defaults.
- AC4: Dialog shows Apply and Cancel buttons.
- AC5: Dialog is modal during processing, showing progress.

## Functional Requirements

- **FR-001**: System MUST implement `NativeAudioProcessor` struct implementing existing `AudioProcessor` trait with `is_available() -> true`.
- **FR-002**: `normalize_volume` MUST analyze loudness (LUFS/RMS) and apply peak-safe linear gain toward `target_db` without clipping, using -1.0 dBTP true-peak ceiling.
- **FR-003**: `trim_silence` MUST remove leading and trailing silence below `threshold_db` using ~20 ms RMS window and ~50 ms safety padding, on full channel frame boundaries.
- **FR-004**: Both operations MUST preserve existing ID3 tags using the `id3` crate (v1.14).
- **FR-005**: Both operations MUST produce standard CBR/VBR MP3 output compatible with primitive portable devices (Constitution IX).
- **FR-006**: Processing MUST write edited output atomically (temp file → rename) and clean up temp files on any error (Constitution IV).
- **FR-007**: Batch processing MUST run on background thread (`std::thread::spawn` + `glib` idle callback) so GTK never freezes.
- **FR-008**: DSP toolbar button MUST use an embedded symbolic SVG icon registered in `src/ui/icons.rs`.
- **FR-009**: DSP dialog MUST provide toggle checkboxes for Normalize Volume (default -14.0 dB) and Trim Silence (default -45.0 dB) with adjustable input fields.
- **FR-010**: Individual file errors MUST NOT abort batch processing; errors reported per-file in completion summary.
- **FR-011**: Silent/empty files MUST be safely skipped (no-op, no panic).
- **FR-012**: All DSP logic MUST reside in `src/services/` with zero GTK/UI imports (Constitution V).
- **FR-013**: `PlaceholderAudioProcessor` MUST be retained for fallback/test usage.

## Success Criteria

- **SC-001**: Normalization reaches target loudness within ±0.5 dB without clipping on synthetic PCM test buffers.
- **SC-002**: Silence trimming removes leading/trailing silence while preserving ~50 ms padding and channel alignment on synthetic PCM test buffers.
- **SC-003**: Real MP3 integration tests using copies of `assets/mp3-sample/` files in `tempfile::tempdir()` produce valid playable MP3 output with preserved ID3 tags and zero leftover temp files.
- **SC-004**: Headless GTK GUI test verifies DSP toolbar button opens dialog and cancel works cleanly.
- **SC-005**: Release binary size remains < 2.5 MB (currently 1.9 MB).
- **SC-006**: All existing 50 tests continue to pass.

## Technical Constraints

- NO FFmpeg or external CLI binaries (Constitution Technical Constraints).
- Use `rodio::Decoder` (already in `Cargo.toml`) for PCM sample decoding.
- Add minimal crates: `ebur128` for loudness analysis, `mp3lame-encoder` for MP3 encoding (fall back to pure-Rust MP3 frame manipulation if binary size exceeds 2.5 MB ceiling).
- `src/platform/windows.rs` and `src/platform/macos.rs` MUST continue compiling cleanly on Linux.

## Edge Cases

- **EC-001**: MP3 file with 0 audio frames → no-op, no error.
- **EC-002**: Track already at target loudness → no-op or negligible gain applied.
- **EC-003**: Entire track is silence → trim produces no-op, file unchanged.
- **EC-004**: Corrupted MP3 file → graceful error per file, batch continues.
- **EC-005**: Disk full during write → temp file cleaned up, error reported.
- **EC-006**: File locked by another process → error reported, batch continues.

## Dependencies

- `rodio` (existing, v0.19) — PCM decoding
- `id3` (existing, v1.14) — tag preservation
- `ebur128` (new) — LUFS loudness measurement
- `mp3lame-encoder` (new) — MP3 encoding from PCM
- `tempfile` (dev-dependency, existing) — test temp dirs

## Out of Scope

- Real-time DSP preview during playback
- Audio format conversion (WAV, FLAC, OGG → MP3)
- Fade in/out effects
- EQ or filter DSP
- DOCX export (superseded by Feature 011: Unified Printable Catalog Exporter PDF)
