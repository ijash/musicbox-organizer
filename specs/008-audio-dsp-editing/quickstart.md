# Quickstart: Audio DSP Editing Suite Validation

## Validation Scenarios

### 1. Unit Tests for PCM DSP Logic
```bash
cargo test --lib services::audio_editing
```
Expected: Fast headless unit tests pass verifying:
- Volume normalization hits target loudness within ±0.5 dB without clipping.
- Silence trimming strips leading/trailing silence while preserving safety padding.
- Silent/empty audio buffers safely no-op without panics.

### 2. Integration Tests with Sample MP3 Files
```bash
cargo test --test integration_tests
```
Expected: Tests using copies of `assets/mp3-sample/` in `tempfile::tempdir()` verify:
- Output is a valid playable MP3 file.
- All ID3 tags (artist, title, album, APIC artwork) preserved verbatim.
- Zero leftover temporary files in directory.

### 3. Headless GTK GUI Test Execution
```bash
./tests/gui/run_gui_tests.sh
```
Expected: `xvfb-run` runs `gui_tests` verifying:
- "Audio Tools" toolbar button opens DSP dialog.
- Dialog controls (Normalize & Trim checkboxes, entry fields, Apply & Cancel buttons) function without crash.
- Dialog closes cleanly upon cancellation or batch completion.

### 4. Release Binary Size Audit
```bash
cargo build --release
ls -lh target/release/musicbox
```
Expected: Standalone release binary remains < 2.5 MB (Constitution Principle I).
