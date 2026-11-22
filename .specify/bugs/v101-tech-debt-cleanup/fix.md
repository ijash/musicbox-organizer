# Bug Fix: v1.0.1 Tech Debt Cleanup

- **Slug**: v101-tech-debt-cleanup
- **Fixed**: 2026-11-23
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Resolved 3 tech debt issues: returned honest unsupported `Err` messages on macOS device format and eject operations, guarded audio DSP processing against non-MP3 files with explicit errors, and added clear `Superseded by Feature 010` banners to obsolete GTK3/Relm4 specifications.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/platform/macos.rs` | modified | Implemented honest unsupported `Err` returns for `format_device` and `eject_device`, plus `Default` trait. |
| `src/platform/mod.rs` | modified | Exposed `pub mod macos;` across all targets matching `windows`. |
| `src/services/audio_editing.rs` | modified | Added extension guard returning explicit `Err` when non-MP3 files are passed to DSP methods. |
| `specs/006-modularize-gtk-ui/spec.md` | modified | Marked `Status: Superseded` with Feature 010 notice. |
| `specs/007-gui-test-harness-relm4-icons/spec.md` | modified | Marked `Status: Superseded` with Feature 010 notice. |
| `tests/unit/device_tests.rs` | modified | Added `test_macos_unsupported_format_and_eject_errors`. |
| `tests/integration/audio_editing_integration_tests.rs` | modified | Added `test_dsp_rejects_non_mp3_files`. |

## Diff Highlights

### 1. macOS Device Operations Return Honest Errors
```rust
fn format_device(&self, _device_id: &str, _filesystem: FileSystem) -> Result<(), Box<dyn Error>> {
    Err("Device formatting is not yet supported on macOS".into())
}

fn eject_device(&self, _device_id: &str) -> Result<(), Box<dyn Error>> {
    Err("Safe eject is not yet supported on macOS".into())
}
```

### 2. Audio DSP Non-MP3 Rejection
```rust
let is_mp3 = path
    .extension()
    .and_then(|s| s.to_str())
    .map(|ext| ext.eq_ignore_ascii_case("mp3"))
    .unwrap_or(false);

if !is_mp3 {
    return Err("Only MP3 audio files are supported for DSP editing".into());
}
```

### 3. Superseded Banners on Legacy Specs
```markdown
> **Superseded by Feature 010 (Cross-Platform GUI)**: This specification describes the legacy Linux-only GTK3/Relm4 architecture, which has been completely superseded by Feature 010 (pure-Rust `eframe`/`egui` + `egui_kittest`). Retained for historical reference only.
```

## Tests Added or Updated

- `tests/unit/device_tests.rs::test_macos_unsupported_format_and_eject_errors` — Verifies `MacOSDeviceManager` format and eject return explicit unsupported errors.
- `tests/integration/audio_editing_integration_tests.rs::test_dsp_rejects_non_mp3_files` — Verifies `process_track`, `normalize_volume`, and `trim_silence` reject non-MP3 inputs with descriptive errors.

## Local Verification

- Commands run:
  - `cargo test --all-targets` → 105 passed, 0 failed.
  - `cargo clippy --all-targets -- -D warnings` → Clean, zero warnings.
  - `cargo fmt --check` → Clean formatting.

## Deviations from Assessment

None. All remediations implemented as proposed.

## Follow-ups

- Proceed to `/speckit-bug-test slug=v101-tech-debt-cleanup`.
