# Data Model: MP3 File Organizer & ID3 Editor

**Feature Branch**: `001-mp3-file-organizer`
**Date**: 2026-11-20

## Entities

### 1. TrackMetadata
Represents ID3 audio metadata extracted from or written to an MP3 file.

| Field | Type | Description | Validation |
|---|---|---|---|
| `track_number` | `Option<u32>` | ID3 Track Number | Optional, ≥ 0 |
| `title` | `String` | ID3 Song Title | Fallback to file stem if empty |
| `artist` | `String` | ID3 Artist Name | Fallback to "Unknown Artist" |
| `album` | `String` | ID3 Album Name | Fallback to "Unknown Album" |
| `year` | `Option<i32>` | ID3 Year | Optional |
| `duration_secs` | `u64` | Audio duration | Read-only |

### 2. OrganizerTrack
Core playlist track representation inside the app session.

| Field | Type | Description |
|---|---|---|
| `index` | `u32` | 0-based sequential ordering index |
| `file_path` | `PathBuf` | Source audio file path on disk |
| `original_filename`| `String` | Base filename before prefixing |
| `xxyy_prefix` | `String` | 4-letter base-26 sequence (e.g. `AAAA`) |
| `metadata` | `TrackMetadata` | In-memory ID3 tag state |
| `is_modified` | `bool` | True if tags or order changed in current session |

### 3. SessionState
In-memory working state holding working playlist, history stack, and dirty flags.

| Field | Type | Description |
|---|---|---|
| `directory` | `PathBuf` | Current scanned directory |
| `tracks` | `Vec<OrganizerTrack>` | Current track list |
| `undo_stack` | `Vec<Vec<OrganizerTrack>>` | Bounded stack (max 10 entries) |
| `redo_stack` | `Vec<Vec<OrganizerTrack>>` | Redo history |
| `selected_track_index`| `Option<usize>` | Currently selected track for preview/edit |
| `is_playing` | `bool` | Audio player playback status |

## State Transitions

### Track Edit & Undo/Redo
```
[Unmodified Track] 
       │ 
       ▼ (User edits Title/Artist/Album or reorders)
[Push to UndoStack (max 10)]
       │
       ▼
[Modified In-Memory]
       │
       ├─────────────────────────┐
       ▼ (Undo clicked)          ▼ (Save clicked)
[Pop Undo -> Restore]     [Write ID3 tags to MP3]
                          [Rename files with [XXYY-name.mp3]]
                          [Clear Undo/Redo Stacks]
```
