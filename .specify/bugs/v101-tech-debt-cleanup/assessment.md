# Bug Assessment: v1.0.1 Tech Debt Cleanup

- **Slug**: v101-tech-debt-cleanup
- **Created**: 2026-11-23
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> Branch from `Development` as `fix/v101-tech-debt-cleanup` and assess these 3 tech-debt defects (do NOT edit source code yet):
> 1. `src/platform/macos.rs`: `format_device` and `eject_device` silently return `Ok(())` no-ops instead of honest unsupported `Err(...)` like `src/platform/windows.rs`.
> 2. `src/services/audio_editing.rs`: DSP entrypoints (`process_track`, `trim_silence`, `normalize_volume`) lack an explicit early extension guard rejecting non-MP3 inputs (`.flac`, `.wav`, etc.) before decode/encode.
> 3. `specs/006-modularize-gtk-ui/spec.md` and `specs/007-gui-test-harness-relm4-icons/spec.md`: obsolete GTK3/Relm4 specs lack a `Superseded by Feature 010` banner.

## Symptom

1. **macOS Device Operations**: `MacOSDeviceManager::format_device` and `eject_device` silently return `Ok(())` despite performing no formatting or ejection, creating false confidence of disk operation success (in contrast to `WindowsDeviceManager` which returns descriptive `Err` values).
2. **Audio DSP Input Rejection**: Invoking `NativeAudioProcessor::trim_silence`, `normalize_volume`, or `process_track` on non-MP3 files (e.g. `.flac`, `.wav`, `.m4a`) silently returns `Ok(false)` or `Ok(())` without warning or error, causing batch DSP callers to falsely record the non-MP3 track as successfully processed instead of reporting an error.
3. **Obsolete GTK3/Relm4 Specifications**: `specs/006-modularize-gtk-ui/spec.md` and `specs/007-gui-test-harness-relm4-icons/spec.md` remain marked as active Drafts describing GTK3 and `xvfb-run`, causing confusion for developers and agents about current architecture.

## Reproduction

1. **macOS Stub No-ops**:
   - Construct a `MacOSDeviceManager`. Call `format_device("/dev/disk2s1", FileSystem::FAT32)`. Observe `Ok(())` is returned without any disk formatting.
   - Call `eject_device("/dev/disk2s1")`. Observe `Ok(())` is returned without device ejection.
2. **Silent Non-MP3 DSP Pass**:
   - Call `processor.normalize_volume(Path::new("song.flac"), -14.0)`.
   - Observe `Ok(())` is returned even though FLAC cannot be encoded or modified by the native MP3 LAME pipeline.
   - Run batch DSP on a playlist containing a non-MP3 track. Observe the track is counted as `succeeded` with 0 bytes modified and zero error message.
3. **Spec Ambiguity**:
   - Inspect `specs/006-modularize-gtk-ui/spec.md` and `specs/007-gui-test-harness-relm4-icons/spec.md`. Observe no indicator that GTK3/Relm4 has been replaced by Feature 010 (`eframe`/`egui`).

## Suspected Code Paths

- `src/platform/macos.rs:35–48` — `format_device` and `eject_device` return `Ok(())` stubs with `// TODO` comments.
- `src/platform/mod.rs:4–5` — `pub mod macos;` is gated by `#[cfg(target_os = "macos")]`, preventing cross-platform unit test validation.
- `src/services/audio_editing.rs:88–95` — `process_track` returns `Ok(false)` silently for non-MP3 extensions; `trim_silence` and `normalize_volume` wrap `process_track` and discard the boolean, converting it to `Ok(())`.
- `specs/006-modularize-gtk-ui/spec.md:1–10` — Missing status banner indicating supersession by Feature 010.
- `specs/007-gui-test-harness-relm4-icons/spec.md:1–10` — Missing status banner indicating supersession by Feature 010.

## Root Cause Hypothesis

Medium confidence.
1. `src/platform/macos.rs` was originally implemented as placeholder stubs returning `Ok(())`. While `WindowsDeviceManager` was upgraded in v0.4.0 audit to return honest errors, macOS remained untouched.
2. `src/services/audio_editing.rs` treated non-MP3 files as skipped (`Ok(false)`), but the trait interface `AudioProcessor` returns `Result<(), Box<dyn Error>>` where `Ok(false)` maps directly to `Ok(())`, hiding the unsupported file condition from callers and users.
3. Specifications 006 and 007 were retained as historical artifacts after the complete migration to `eframe`/`egui` in Feature 010, but were never stamped with a deprecation/superseded notice.

## Proposed Remediation

**Preferred**:

1. **Honest Errors on macOS Format & Eject**:
   - In `src/platform/macos.rs`:
     - Update `format_device` to return `Err("Device formatting is not yet supported on macOS".into())`.
     - Update `eject_device` to return `Err("Safe eject is not yet supported on macOS".into())`.
   - In `src/platform/mod.rs`:
     - Make `pub mod macos;` accessible across platforms (matching `pub mod windows;`) so unit tests can run headlessly on Linux CI.
   - Add unit tests verifying `MacOSDeviceManager` error returns.
2. **Explicit Non-MP3 Rejection in DSP Engine**:
   - In `src/services/audio_editing.rs`:
     - In `process_track`: If the file extension is missing or is not case-insensitively `"mp3"`, return `Err("Only MP3 audio files are supported for DSP editing".into())`.
     - This ensures `trim_silence` and `normalize_volume` return a descriptive `Err`, and the batch worker logs the file in `errors` and increments `failed += 1`.
   - Update tests in `audio_dsp_tests.rs` / `audio_editing_integration_tests.rs` to verify that non-MP3 files fail with descriptive errors.
3. **Superseded Banners on Legacy Specs**:
   - In `specs/006-modularize-gtk-ui/spec.md` and `specs/007-gui-test-harness-relm4-icons/spec.md`:
     - Change `Status: Draft` to `Status: Superseded`.
     - Add prominent header blockquote:
       `> **Superseded by Feature 010 (Cross-Platform GUI)**: This specification describes the legacy Linux-only GTK3/Relm4 architecture, which has been completely superseded by Feature 010 (pure-Rust eframe/egui + egui_kittest). Retained for historical reference only.`

**Files likely to change**:
- `src/platform/macos.rs`
- `src/platform/mod.rs`
- `src/services/audio_editing.rs`
- `specs/006-modularize-gtk-ui/spec.md`
- `specs/007-gui-test-harness-relm4-icons/spec.md`
- `tests/unit/device_tests.rs`
- `tests/gui/audio_dsp_tests.rs`

**Tests to add or update**:
- `test_macos_unsupported_format_and_eject_errors`: Verify `MacOSDeviceManager::format_device` and `eject_device` return explicit unsupported `Err`.
- `test_dsp_rejects_non_mp3_files`: Verify `process_track`, `trim_silence`, and `normalize_volume` return explicit errors when given non-MP3 paths.

## Risks & Considerations

- **Cross-Platform Compilation**: Exposing `pub mod macos;` under `test` must not pull in macOS-specific native dependencies on Linux or Windows runners. `MacOSDeviceManager` currently uses only standard library types, so this is zero-risk.
- **Batch Processing Semantics**: Explicitly failing non-MP3 tracks ensures accurate accounting in `DspProgressMessage::Finished` (`succeeded` vs `failed`).

## Open Questions

None. All 3 defects are clearly located and remediations are self-contained.
