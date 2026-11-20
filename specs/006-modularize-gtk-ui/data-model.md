# Data Model: Modular UI Component Architecture

**Feature Branch**: `006-modularize-gtk-ui`
**Date**: 2026-11-20

## UI Component Hierarchy

```text
ApplicationWindow (740x520)
└── root_box (Vertical Box, 6px spacing)
    ├── HeaderComponent (src/ui/components/header.rs)
    │   ├── Logo Image (24x24 px embedded)
    │   ├── Project Label ("MusicBox Organizer — <name> (<N> tracks)")
    │   ├── Undo Button
    │   ├── Redo Button
    │   └── Theme Button ("☀ Theme: Dark" / "🌙 Theme: Light")
    │
    ├── PlaylistTableComponent (src/ui/components/playlist_table.rs)
    │   ├── Toolbar (Scan, Add, Up, Down, Audio Tools, Export Text, Save, Open)
    │   └── ScrolledWindow -> TreeView (Index, ExportFilename, Title, Artist, FilePath)
    │
    ├── AudioBarComponent (src/ui/components/audio_bar.rs)
    │   └── Frame ("Audio Preview Player")
    │       └── Horizontal Box (Play, Pause, Stop, AudioStatusLabel)
    │
    ├── ExportPanelComponent (src/ui/components/export_panel.rs)
    │   └── Frame ("Export to Removable Media (USB / SD)")
    │       ├── Device Selection & Refresh Row
    │       ├── Mount & Free Space Info Label
    │       ├── Options (Reformat Checkbox, Subpath Entry)
    │       ├── Action Buttons (Export Media, Eject)
    │       └── Progress & Status (ProgressBar, StatusLabel)
    │
    └── FooterComponent (src/ui/components/footer.rs)
        └── Italic Label ("Made by Ijash. More at github")
```

## Shared State Context
Components coordinate state through:
- `project: Rc<RefCell<ProjectFile>>`
- `session_state: Rc<RefCell<SessionState>>`
- `dialog_state: Rc<RefCell<ExportDialog>>`
- `audio_player: Rc<RefCell<AudioPreviewPlayer>>`
- `refresh_playlist: Rc<dyn Fn()>`
- `sync_session: Rc<dyn Fn()>`
