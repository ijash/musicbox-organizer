# Phase 1 Data Model: Unified Cross-Platform Rust GUI (Feature 010)

## 1. Domain Entities (Reused from `src/models/`)

The core business models are 100% decoupled from the UI and remain untouched:

### `ProjectFile` (`src/models/project.rs`)
- **Fields**:
  - `metadata`: `PlaylistMetadata` (playlist name, author, created_at, updated_at)
  - `tracks`: `Vec<PlaylistTrack>` (ordered sequence of audio tracks)
- **Validation**:
  - Non-empty track list required for media export.
  - Track ordering defines playback sequence and index in export naming `[XXYY-filename.mp3]`.

### `PlaylistTrack` (`src/models/track_metadata.rs`)
- **Fields**:
  - `id`: `String` (unique UUID)
  - `source_path`: `PathBuf` (local path to source audio file)
  - `export_filename`: `String` (computed lexicographical export name)
  - `title`: `Option<String>` (ID3 title tag, inline-editable in UI)
  - `artist`: `Option<String>` (ID3 artist tag, inline-editable in UI)
  - `duration_secs`: `Option<u32>` (track duration in seconds)
- **Validation**:
  - `source_path` must exist on local filesystem to be playable or processable.
  - Title and artist changes trigger metadata dirty state in session.

### `SessionState` (`src/models/session_state.rs`)
- **Fields**:
  - `undo_stack`: `Vec<ProjectFile>` (bounded to 10 historical snapshots)
  - `redo_stack`: `Vec<ProjectFile>`
  - `is_dirty`: `bool`
- **State Transitions**:
  - Any track add, reorder, delete, or metadata edit pushes current snapshot onto `undo_stack` and clears `redo_stack`.
  - `undo()` pops from `undo_stack` and pushes current state to `redo_stack`.
  - `redo()` pops from `redo_stack` and pushes current state to `undo_stack`.

### `Device` (`src/models/device.rs`)
- **Fields**:
  - `id`: `String` (device identifier, e.g. `/dev/sdb1` or `E:`)
  - `name`: `String` (volume label or description)
  - `mount_point`: `PathBuf` (filesystem path where device is accessible)
  - `total_space`: `u64` (total storage capacity in bytes)
  - `free_space`: `u64` (available storage capacity in bytes)
  - `is_removable`: `bool` (flag indicating removable flash/SD media)
  - `filesystem`: `FileSystem` (`Fat32`, `ExFat`, `Ntfs`, `Ext4`, `Unknown`)

---

## 2. User Interface State Entities (`src/ui/`)

### `MusicBoxApp` (`src/ui/app.rs`)
The central state struct owning the UI session directly in immediate mode:

```rust
pub struct MusicBoxApp {
    // Core Domain Data
    pub project: ProjectFile,
    pub session: SessionState,
    pub project_path: Option<PathBuf>,

    // Playlist Table Interaction
    pub selected_track_indices: std::collections::BTreeSet<usize>,
    pub editing_cell: Option<(usize, TableColumn)>,

    // Audio Preview Player
    pub preview_player: Option<AudioPreviewPlayer>,
    pub playing_track_idx: Option<usize>,

    // Sub-dialog & Panel States
    pub dsp_dialog: DspDialogState,
    pub export_panel: ExportPanelState,

    // Global Visuals & Status
    pub dark_mode: bool,
    pub status_message: String,

    // Concurrency Channels (Workers -> UI)
    pub dsp_rx: Option<std::sync::mpsc::Receiver<DspProgressMessage>>,
    pub export_rx: Option<std::sync::mpsc::Receiver<ExportProgressMessage>>,
}
```

### `TableColumn` Enum
- `TrackNumber`: Display index (`#`)
- `ExportFilename`: Read-only computed filename (`[XXYY-filename.mp3]`)
- `Title`: Inline-editable string (`egui::TextEdit::singleline`)
- `Artist`: Inline-editable string (`egui::TextEdit::singleline`)
- `SourceFile`: Path string tooltip / display

### `DspDialogState` (`src/ui/dsp_dialog.rs`)
Manages modal parameters for volume normalization and silence trimming:

```rust
pub struct DspDialogState {
    pub is_open: bool,
    pub normalize_enabled: bool,
    pub normalize_target_db: f32, // Default: -14.0
    pub trim_enabled: bool,
    pub trim_threshold_db: f32,   // Default: -45.0
    pub is_processing: bool,
    pub progress_percent: f32,
    pub progress_text: String,
    pub error_summary: Option<String>,
}
```

### `ExportPanelState` (`src/ui/export_panel.rs`)
Manages device selection and removable media export configuration:

```rust
pub struct ExportPanelState {
    pub available_devices: Vec<Device>,
    pub selected_device_id: Option<String>,
    pub subpath: String,
    pub reformat_enabled: bool,
    pub show_reformat_confirm: bool,
    pub is_exporting: bool,
    pub export_progress: f32,
    pub export_status: String,
}
```

---

## 3. Worker Thread Communication Messages

### `DspProgressMessage`
Communicates background DSP batch execution events to the UI thread:

```rust
pub enum DspProgressMessage {
    Started { total_tracks: usize },
    Step { current: usize, total: usize, filename: String },
    Finished { succeeded: usize, failed: usize, errors: Vec<String> },
    Failed(String),
}
```

### `ExportProgressMessage`
Communicates media export file copy progress to the UI thread:

```rust
pub enum ExportProgressMessage {
    Started { total_tracks: usize },
    Step { current: usize, total: usize, filename: String },
    Finished { total_bytes: u64 },
    Failed(String),
}
```

---

## 4. State Transition Flow

```
[UI Inactive / Normal]
      │
      ├── Scan / Add MP3 (rfd) ─────────► Updates project.tracks, pushes SessionState
      ├── Inline Edit Title/Artist ──────► Updates track metadata, pushes SessionState
      ├── Up / Down / Remove ────────────► Re-indexes tracks, pushes SessionState
      ├── Undo / Redo ───────────────────► Reverts/re-applies ProjectFile from SessionState
      ├── Play Track ────────────────────► Stops previous, starts AudioPreviewPlayer, playing_track_idx = Some(idx)
      │
      ├── Open DSP Dialog ──────────────► dsp_dialog.is_open = true
      │         │
      │         ▼
      │   Click "Apply" ────────────────► If playing_track_idx in selected: stop preview_player
      │                                    dsp_dialog.is_processing = true
      │                                    Spawn background thread with NativeAudioProcessor
      │                                    Poll dsp_rx in update(), ctx.request_repaint()
      │                                    Finished: dsp_dialog.is_processing = false, reload tags
      │
      └── Export Playlist ──────────────► Validate target device & space
                │
                ▼
          If reformat checked ──────────► show_reformat_confirm = true (2-step guard)
                │ (Confirmed)
                ▼
          Spawn background thread ──────► ExportService::export_playlist
                                           Poll export_rx in update(), ctx.request_repaint()
                                           Finished: show completion message, enable Eject
```
