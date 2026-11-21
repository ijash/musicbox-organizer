# Research: Audio DSP Editing Suite

**Feature Branch**: `008-audio-dsp-editing`
**Date**: 2026-11-21

## 1. PCM Decoding & Sample Handling

### Decision
Reuse `rodio::Decoder` (already in `Cargo.toml`) wrapping `std::fs::File` to decode input MP3 into 32-bit floating point interleaved PCM buffers (`f32` in `[-1.0, 1.0]`).

### Rationale
- Zero additional decoder crates needed, preserving minimal dependency footprint per Constitution Principle I.
- `rodio::Decoder` gives exact sample rate, channel count, and standard sample iteration.

---

## 2. Loudness Analysis & Volume Normalization (EBU R128)

### Decision
Use `ebur128` crate (pure Rust ITU-R BS.1770 / EBU R128 algorithm). Measure integrated loudness in LUFS. Compute required linear gain:
`gain = 10.0^((target_db - current_lufs) / 20.0)`.
Enforce true-peak ceiling (-1.0 dBTP / linear `0.891`):
`gain = min(gain, 0.891 / max_peak)`.

### Rationale
- Industry-standard loudness normalization prevents listener volume adjustments across disparate tracks.
- True-peak limiter ensures zero digital clipping during subsequent lossy MP3 re-encoding.
- Edge case: silent or near-silent files (LUFS = `-inf`) safely no-op without panic.

---

## 3. Silence Trimming Algorithm

### Decision
Process interleaved PCM frames in ~20 ms blocks (e.g. 882 frames @ 44.1 kHz). Calculate block RMS:
`rms_db = 20.0 * log10(max(rms, 1e-9))`.
- Find first block exceeding `threshold_db` (default -45.0 dB).
- Find last block exceeding `threshold_db`.
- Add ~50 ms safety padding (2205 frames @ 44.1 kHz) before and after, clamping to [0, total_frames].
- Retain full frame boundaries (samples * channels) to avoid channel phase swapping.
- If entire track is below threshold, no-op (file untouched).

---

## 4. MP3 Encoding & Binary Size Impact

### Decision
Encode normalized/trimmed PCM to MP3 using `mp3lame-encoder`. Match source bitrate (or 192 kbps CBR if source bitrate is undetectable).
Audit binary size before/after release build.

### Rationale
- CBR/VBR standard MP3 provides 100% playback compatibility on primitive hardware players (Constitution Principle IX).
- Release binary size target: < 2.5 MB (currently 1.9 MB). If `mp3lame-encoder` exceeds 2.5 MB, fallback to pure-Rust MP3 frame manipulation.

---

## 5. ID3 Tag Preservation & Atomic Writes

### Decision
1. Read existing ID3 tags via `id3::Tag::read_from_path(src_path)` before re-encoding.
2. Write newly encoded MP3 to a temporary file in the same parent directory (`tempfile::NamedTempFile::new_in(parent)`).
3. If original ID3 tag was present, write it directly onto the new MP3 file (`tag.write_to_file(...)`).
4. Atomically persist/rename over original file (`temp_file.persist(src_path)`).
5. On any error, drop temp file immediately so zero orphans remain (Constitution Principle IV).
6. Push pre-edit file bytes into in-memory `SessionState` undo stack (Constitution Principle III).
