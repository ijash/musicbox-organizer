# Implementation Plan: Audio DSP Editing Suite (Feature 008)

**Branch**: `008-audio-dsp-editing` | **Date**: 2026-11-21 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/008-audio-dsp-editing/spec.md` with integrated clarifications.

## Summary
Replace `PlaceholderAudioProcessor` in `src/services/audio_editing.rs` with `NativeAudioProcessor` implementing volume normalization (EBU R128 LUFS analysis, peak-safe gain) and silence trimming (~20ms RMS window, ~50ms safety padding). Re-encode to MP3 matching source bitrate, preserving ID3v1/v2 tags verbatim. Provide native GTK3 DSP dialog triggered from toolbar button (`AudioTools` vector icon), executing on background thread with non-blocking GLib status reporting. Keep binary size < 2.5 MB.

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.75+)
**Primary Dependencies**: `rodio = "0.19"` (PCM decoding), `id3 = "1.14"` (tag preservation), `ebur128 = "0.1"` (loudness measurement), `mp3lame-encoder = "0.1"` (MP3 re-encoding), `gtk = "0.17"` (Linux GTK3)
**Storage**: Atomic file overwrite (`tempfile::NamedTempFile` in target parent dir -> rename), zero orphaned temp files
**Testing**: Unit tests in `src/services/audio_editing.rs`, real sample tests in `tempfile::tempdir()`, headless GUI tests in `tests/gui_tests.rs` with `xvfb-run`
**Target Platform**: Linux (GTK3 desktop & headless CI), stub builds on Win32/macOS
**Project Type**: Desktop Audio Organizer & Native DSP Engine
**Performance Goals**: Fast single-pass processing (<2s for typical 3-5 min MP3 track), GUI event loop stays responsive at 60 fps
**Constraints**: Pure Rust or minimal C FFI (`mp3lame-encoder`), binary size ceiling < 2.5 MB, zero disk temp leftovers, strict ID3 frame preservation (including `APIC`)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Lightweight First)**: PASS. Reuses `rodio::Decoder`. Only adds lightweight crates (`ebur128`, `mp3lame-encoder`). Monitored release binary must remain < 2.5 MB (fallback to pure-Rust frame manipulation if exceeded).
- **Principle II (Cross-Platform Native)**: PASS. Core DSP logic is OS-agnostic pure Rust in `src/services/`.
- **Principle III (User-Centric Simplicity)**: PASS. Discoverable toolbar button, sensible defaults (-14.0 dB LUFS, -45.0 dB silence threshold), in-memory undo session rollback on overwrite.
- **Principle IV (Offline-First & Portable)**: PASS. No network calls, atomic write with instant temp cleanup on success or error.
- **Principle V (Testable Core Logic & GUI Interaction Verification)**: PASS. Full decoupling of DSP logic from GTK. Headless GUI test exercises DSP dialog open/cancel/apply via `tests/gui/run_gui_tests.sh`.
- **Principle VII (Rapid Development & Continuous Rebuild)**: PASS. Binary rebuilt in release mode after completion.
- **Principle VIII (DRY & KISS)**: PASS. Extends existing `AudioProcessor` trait, centralized icon helper in `src/ui/icons.rs`.
- **Principle IX (Device Compatibility & Lexicographical Sorting)**: PASS. Preserves standard MP3 CBR/VBR format compatible with basic hardware players.

## Project Structure

### Documentation

```text
specs/008-audio-dsp-editing/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code

```text
src/
├── services/
│   ├── audio_editing.rs # NativeAudioProcessor implementation & unit tests
│   └── mod.rs           # Export NativeAudioProcessor
├── ui/
│   ├── icons.rs         # AudioTools SVG icon
│   └── components/
│       └── playlist_table.rs # Wire DSP button & DSP dialog
tests/
├── gui/
│   └── harness.rs       # GuiHarness DSP dialog trigger helpers
└── gui_tests.rs         # Headless GUI DSP test cases
```

**Structure Decision**: Single project architecture. Core DSP in `src/services/audio_editing.rs`, UI wiring in `src/ui/components/playlist_table.rs` and `src/ui/icons.rs`.

## Complexity Tracking
*No violations. All design choices strictly align with Constitution.*

