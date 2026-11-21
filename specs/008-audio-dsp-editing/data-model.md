# Data Model: Audio DSP Editing Suite

**Feature Branch**: `008-audio-dsp-editing`
**Date**: 2026-11-21

## Entities

### 1. DspJobConfig
Configuration parameters for an audio editing batch run.

| Field | Type | Description |
|---|---|---|
| `normalize` | `bool` | Whether to apply EBU R128 volume normalization |
| `target_db` | `f32` | Target integrated loudness in dB LUFS (default: `-14.0`) |
| `trim_silence` | `bool` | Whether to trim leading/trailing silence |
| `silence_threshold_db` | `f32` | Silence threshold in dB RMS (default: `-45.0`) |
| `target_paths` | `Vec<PathBuf>` | List of selected audio file paths to process |

### 2. DspProgress
Progress status emitted from background worker to GTK UI thread via GLib sender.

| Field | Type | Description |
|---|---|---|
| `current_index` | `usize` | Index of track currently being processed (0-based internally, mapped to 1-based in UI progress displays) |
| `total_tracks` | `usize` | Total number of tracks in batch |
| `current_file` | `String` | Filename of track in progress |
| `status_message` | `String` | Operational detail ("Trimming silence...", "Normalizing...", "Done") |

### 3. DspBatchReport
Summary of batch operation outcome shown to user on completion.

| Field | Type | Description |
|---|---|---|
| `succeeded` | `usize` | Number of tracks successfully processed |
| `skipped` | `usize` | Number of tracks skipped (silent or non-MP3) |
| `failed` | `Vec<(PathBuf, String)>` | List of tracks that encountered errors with error message |
