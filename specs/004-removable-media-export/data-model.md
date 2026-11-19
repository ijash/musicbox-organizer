# Data Model: Removable Media Export (Feature 004)

## Entities

### Device
Represents a connected USB/SD card or other removable storage.

```rust
pub struct Device {
    pub id: String,                    // Platform-specific ID (Win: \\?\..., macOS: disk0s1, Linux: /dev/sda1)
    pub name: String,                  // Human-readable name (e.g., "Kingston USB Drive", "SanDisk SD Card")
    pub capacity_bytes: u64,           // Total capacity (bytes)
    pub available_bytes: u64,          // Free space (bytes)
    pub filesystem: FileSystem,        // FAT32, exFAT, or Unknown
    pub mount_point: Option<PathBuf>,  // Mount path if mounted, else None
    pub is_removable: bool,            // True if USB/SD; false if internal HD
    pub is_write_protected: bool,      // Read-only flag
    pub last_seen: Instant,            // Timestamp for disconnection detection
}

pub enum FileSystem {
    FAT32,
    exFAT,
    Unknown(String),
}
```

**Validation Rules**:
- `capacity_bytes` > 0 (must have size)
- `available_bytes` <= `capacity_bytes` (sanity check)
- `is_removable` must be true (reject internal drives)
- `is_write_protected` = false (reject read-only media for export)

**Relationships**:
- 1 Device → many ExportJobs (history of exports to this device)

---

### ExportJob
Tracks a single export operation (state machine).

```rust
pub struct ExportJob {
    pub id: String,                  // UUID v4
    pub device_id: String,           // Foreign key: Device.id
    pub status: ExportStatus,        // PENDING, IN_PROGRESS, COMPLETED, FAILED, ABORTED
    pub tracks: Vec<Track>,          // Playlist to export
    pub total_bytes: u64,            // Sum of all track file sizes
    pub bytes_written: u64,          // Incremental progress
    pub files_written: usize,        // Count of files successfully copied
    pub reformat_enabled: bool,      // User requested pre-export format
    pub reformat_confirmed: bool,    // User clicked "I understand data loss" warning
    pub started_at: Option<DateTime>,
    pub completed_at: Option<DateTime>,
    pub error_message: Option<String>,
}

pub enum ExportStatus {
    PENDING,        // Waiting for user action
    FORMATTING,     // Device reformatting in progress
    IN_PROGRESS,    // Files copying to device
    COMPLETED,      // All files written
    FAILED,         // Error occurred; partial files may remain on device
    ABORTED,        // User cancelled; partial files remain
}

pub struct Track {
    pub index: u32,              // 0-based playlist position
    pub source_path: PathBuf,    // Local MP3 file
    pub filename: String,        // Original filename
    pub xxyy_name: String,       // Export name (AAAA, AAAB, ..., ZZZZ)
    pub file_size_bytes: u64,
}
```

**State Transitions**:
```
PENDING
  ├─ [user clicks Reformat] → FORMATTING
  │   ├─ [format succeeds] → IN_PROGRESS
  │   └─ [format fails] → FAILED
  └─ [user clicks Export] → IN_PROGRESS
      ├─ [all files copied] → COMPLETED
      ├─ [device full/disconnected] → FAILED
      └─ [user cancels] → ABORTED
```

**Validation Rules**:
- `tracks` not empty (at least 1 track)
- Each Track `index` unique and 0-based sequential
- `reformat_enabled=true` requires `reformat_confirmed=true` before format starts
- `bytes_written` <= `total_bytes` (sanity check)
- Terminal states (COMPLETED, FAILED, ABORTED) have `completed_at` set

---

### ProjectFile
Represents a saved project (.txt format) for offline playlist editing.

```rust
pub struct ProjectFile {
    pub version: String,               // "1.0"
    pub created_at: DateTime,
    pub playlist: PlaylistMetadata,
    pub tracks: Vec<ProjectTrack>,
}

pub struct PlaylistMetadata {
    pub name: String,                  // Project name
    pub artist: Option<String>,        // Curator/collection artist
    pub album: Option<String>,         // Album name
    pub description: Option<String>,   // User notes
}

pub struct ProjectTrack {
    pub index: u32,                    // 0-based position
    pub source_path: String,           // Path to MP3 (relative or absolute)
    pub title: String,                 // ID3 title
    pub artist: String,                // ID3 artist
    pub duration_seconds: u32,
    pub bitrate_kbps: u16,
}
```

**Serialization (Plain Text .txt)**:
```
# MusicBox Project File
# Version: 1.0
# Created: 2026-11-19T20:36:11Z

[Metadata]
name=My Favorite Road Trip
artist=Various
album=Car Playlist
description=Songs for long drives

[Tracks]
001=../music/Song1.mp3|Pink Floyd|Shine On You Crazy Diamond|475
002=../music/Song2.mp3|David Bowie|Space Oddity|269
...
```

**Parsing**:
- Line-based: `#` prefix = comment, ignored
- Section headers: `[SectionName]`
- Key-value pairs: `key=value`
- Track format: `index=path|artist|title|duration_seconds`

**Validation Rules**:
- Version field required and must be "1.0"
- Tracks list not empty
- Track indices sequential and 0-based
- File paths (relative or absolute) parseable

**Relationships**:
- 1 ProjectFile → many Tracks (ordered playlist)
- ProjectFile is independent of Device (saved locally, portable)

---

### ExportResult
Returned after export completes (success or failure).

```rust
pub struct ExportResult {
    pub job_id: String,
    pub device_id: String,
    pub status: ExportStatus,
    pub files_written: usize,
    pub total_files: usize,
    pub bytes_written: u64,
    pub total_bytes: u64,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}
```

---

## Naming Sequence (XXYY Convention)

**Purpose**: Ensure lexicographical (ASCII byte-order) sort on primitive audio devices.

**Algorithm**: Base-26 alphabet sequence (4 chars).
- Index 0 → AAAA
- Index 1 → AAAB
- Index 25 → AAAZ
- Index 26 → ABAA
- Index 675 → ZZAA
- Index 17575 → ZZZZ (maximum for 4-letter base-26)

**Implementation**:
```rust
pub fn track_index_to_xxyy(index: usize) -> String {
    if index > 17575 {
        panic!("Playlist exceeds 17576 tracks (base-26 limit)");
    }
    let mut result = String::with_capacity(4);
    let mut n = index;
    for _ in 0..4 {
        result.push((b'A' + (n % 26) as u8) as char);
        n /= 26;
    }
    result.chars().rev().collect()  // Reverse to AAAA, AAAB, ...
}

pub fn xxyy_to_track_index(xxyy: &str) -> Result<usize> {
    // Inverse function: AAAA → 0, AAAB → 1, ...
    let mut result = 0;
    for (i, ch) in xxyy.chars().rev().enumerate() {
        result += (ch as usize - b'A' as usize) * 26_usize.pow(i as u32);
    }
    if result > 17575 { Err("Out of range") } else { Ok(result) }
}
```

**Test Cases**:
- `track_index_to_xxyy(0)` → `"AAAA"`
- `track_index_to_xxyy(1)` → `"AAAB"`
- `track_index_to_xxyy(25)` → `"AAAZ"`
- `track_index_to_xxyy(26)` → `"ABAA"`
- `track_index_to_xxyy(701)` → `"ZZAZ"`
- `track_index_to_xxyy(17575)` → `"ZZZZ"`
- `xxyy_to_track_index("AAAA")` → `0`
- `xxyy_to_track_index("ZZZZ")` → `17575`

---

## File Layout on Removable Media

After successful export to device root (or user-specified folder):

```
/Volumes/PLAYLIST/  (or X: on Windows, /mnt/usb on Linux)
├── AAAA - Song1.mp3
├── AAAB - Song2.mp3
├── AAAC - Song3.mp3
├── ...
└── ZZZZ - SongN.mp3

# Optional: metadata not exported (project file stays local)
# Device contains only .mp3 files with [XXYY - filename] naming
```

**Lexicographical sort on device**:
- Primitive players read files in ASCII byte-order
- AAAA, AAAB, AAAC, ... ZZZZ ensures correct playback sequence
- No special sort algorithm needed on device

---

## Error Codes & State Recovery

| Error | Cause | User Recovery |
|-------|-------|---------|
| DEVICE_NOT_FOUND | Device disconnected or not enumerated | Reconnect device, retry export |
| DEVICE_FULL | Not enough space for playlist | Free up device space, retry |
| DEVICE_READ_ONLY | Write-protected or permission denied | Check write-protect switch, retry |
| FORMAT_FAILED | Reformat did not complete | Manually reformat on computer, retry |
| FILE_COPY_FAILED | Source MP3 unreadable or permission denied | Verify MP3 file permissions, retry |
| PARTIAL_EXPORT | Export aborted mid-operation | Reconnect device, manual cleanup, retry |

---

## Summary

**Entities**: Device, ExportJob, ProjectFile, Track, ExportResult
**State Machine**: ExportJob status transitions (PENDING → FORMATTING/IN_PROGRESS → COMPLETED/FAILED/ABORTED)
**Naming**: Base-26 XXYY sequence (AAAA–ZZZZ for 17,576 tracks max)
**Storage**: ProjectFile (.txt plain text, local); Device media (MP3 files + naming, removable)
**Validation**: All entities enforce non-null constraints, size bounds, sequential ordering where applicable
