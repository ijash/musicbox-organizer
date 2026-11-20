<p align="center">
  <img src="img/logo/png/logo_mbo_256.png" alt="MusicBox Organizer Logo" width="128" height="128" />
</p>

# MusicBox Organizer

Lightweight, portable desktop MP3 playlist manager and ID3 tag editor designed for portable audio devices and players.

## Overview

MusicBox Organizer is built specifically to solve playlist sorting issues on primitive audio hardware (car head units, portable USB/SD players, microcontrollers, and portable speakers). These devices sort files strictly using raw ASCII byte order rather than smart numerical or metadata sorting.

The application automatically assigns a 4-letter base-26 prefix (`[AAAA-filename.mp3]` through `[ZZZZ-filename.mp3]`), guaranteeing exact sequential playback on any target player.

## Features

- **Playlist Reordering**: Add files, reorder tracks up and down with persistent row selection.
- **Inline ID3 Editing**: Double-click to edit Title and Artist directly in the table.
- **Save vs Export Separation**:
  - **Save Project (`.txt`)**: Persists playlist structure locally for future editing without modifying media.
  - **Export Media**: Writes files with `[XXYY-filename.mp3]` naming directly to USB or SD cards.
- **Device Detection**: Detects connected removable storage, capacity, free space, and filesystem type (FAT32/exFAT).
- **Two-Step Reformat Confirmation**: Optional FAT32/exFAT format before export with an explicit confirmation dialog to prevent accidental data loss.
- **Multi-Format Export**: Exports playlists to `playlist-text/` folder as `.txt` and `.rtf` (Rich Text Format).
- **Audio Preview**: In-app playback preview with Play, Pause, and Stop controls.
- **Undo / Redo**: 10-step bounded history for edits and reordering.
- **Dark Mode**: Default dark theme with a toggleable light theme, saved independently of OS settings.
- **Device Compatibility Audit**: Programmatically verifies filenames and directory trees against ASCII sorting rules and FAT32 path length limits.

## Getting Started

### Prerequisites

- **Rust**: 1.75+ (`cargo`, `rustc`)
- **Linux**: `libgtk-3-dev`, `libasound2-dev`, `libdbus-1-dev`
- **Windows / macOS**: Supported via platform-native compilation

### Launch GUI

```bash
cargo run
# or
cargo run -- gui
```

### CLI Commands

The application provides a full CLI interface without opening a graphical window:

| Command | Description |
|---|---|
| `cargo run -- status` | Show active project and connected removable devices |
| `cargo run -- scan [dir]` | Scan directory for MP3 files and inspect ID3 tags |
| `cargo run -- tag <file.mp3> [title] [artist]` | View or update ID3 tags directly |
| `cargo run -- save [path]` | Save playlist project locally (`.txt`) |
| `cargo run -- export` | Export playlist to connected USB/SD device |
| `cargo run -- export-text [dir]` | Export playlist `.txt` and `.rtf` to `playlist-text/` directory |
| `cargo run -- check-compat [dir]` | Audit directory or media drive for ASCII sort and FAT32 compliance |

## Build Release Binary

To build a standalone, optimized executable (~1.7 MB) with no runtime web dependencies:

```bash
cargo build --release
```

The output binary is located at:
```text
target/release/musicbox
```

Run directly:
```bash
./target/release/musicbox
```

## Testing

Run unit, contract, and integration tests:

```bash
cargo test
```

## Core Principles

1. **Lightweight First**: Single standalone executable (<50 MB target, ~1.7 MB actual). No Electron, no web runtime.
2. **Cross-Platform Native**: Native UI per platform (GTK3 on Linux, Win32 on Windows, Cocoa on macOS).
3. **Offline-First & Portable**: No network calls, runs from any directory, leaves no temporary files.
4. **Device Compatibility**: Lexicographical `[XXYY-filename.mp3]` sequence guarantees sequential playback on primitive audio hardware.

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
