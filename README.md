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
- **Audio DSP Editing Suite**: Pure-Rust volume normalization (EBU R128 LUFS targeting, peak-safe true-peak ceiling) and silence trimming (~20 ms RMS window, 50 ms safety padding) with atomic disk writes and full ID3 tag preservation.
- **Audio Preview**: In-app playback preview with Play, Pause, and Stop controls.
- **Undo / Redo**: 10-step bounded history for edits and reordering.
- **Dark Mode**: Default dark theme with a toggleable light theme, saved independently of OS settings.
- **Device Compatibility Audit**: Programmatically verifies filenames and directory trees against ASCII sorting rules and FAT32 path length limits.

## Download & Quick Start

Pre-compiled standalone executables are available on the [Latest GitHub Release](https://github.com/ijash/musicbox-organizer/releases/latest). No Rust or Cargo installation required.

[![GitHub Release](https://img.shields.io/github/v/release/ijash/musicbox-organizer?style=flat-square)](https://github.com/ijash/musicbox-organizer/releases/latest)

| Operating System | Package Archive | Quick Start Steps |
|---|---|---|
| **Linux (x86_64)** | `musicbox-linux-x86_64.tar.gz` | Extract, `chmod +x musicbox`, run `./musicbox` (full GTK3 GUI + CLI). |
| **Windows (x86_64)** | `musicbox-windows-x86_64.zip` | Extract and run `musicbox.exe` (CLI; native Win32 GUI planned). |
| **macOS (Apple Silicon ARM64)** | `musicbox-macos-arm64.tar.gz` | Extract, `chmod +x musicbox`, run `./musicbox` (CLI; native Cocoa GUI planned). |

Verify archive integrity:
```bash
sha256sum -c SHA256SUMS.txt
```

## Building from Source

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

### Build Release Binary

To build a standalone, optimized executable (~2.2 MB) with no runtime web dependencies:

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

## Creating a Release

Releases are fully automated via GitHub Actions (`.github/workflows/release.yml`). The cross-platform matrix compiles binaries for Linux, Windows, and macOS, validates automated GUI and size tests, computes `SHA256SUMS.txt`, and publishes a GitHub Release.

To cut an official release:

```bash
# 1. Create an annotated git tag
git tag -a v0.2.0 -m "Release v0.2.0"

# 2. Push the tag to GitHub
git push origin v0.2.0
```

> **Note on CI Quotas**: Standard branch pushes (`Development`, `main`) do not invoke the multi-OS build matrix to preserve private runner minutes. For on-demand pre-release smoke testing across all 3 operating systems without publishing a public release, trigger the workflow manually via GitHub Actions (`workflow_dispatch`).

## Core Principles

1. **Lightweight First**: Single standalone executable (<50 MB target, ~1.7 MB actual). No Electron, no web runtime.
2. **Cross-Platform Native**: Native UI per platform (GTK3 on Linux, Win32 on Windows, Cocoa on macOS).
3. **Offline-First & Portable**: No network calls, runs from any directory, leaves no temporary files.
4. **Device Compatibility**: Lexicographical `[XXYY-filename.mp3]` sequence guarantees sequential playback on primitive audio hardware.

## Roadmap & Feature Plan

Planned enhancements for upcoming versions:

- **v0.2.0**: Audio DSP editing suite (volume normalization, silence trimming) with native GTK3 background processing dialog (Completed).
- **v0.2.0**: Native desktop GUI frontends for Windows (Win32) and macOS (Cocoa).
- **v0.3.0**: DOCX printable catalog exporter.
- **v0.3.0**: Custom export path templates (e.g., `/Artist/Album/`).

## Changelog

### v0.2.0 (2026-11-21)

- Pure-Rust audio DSP suite (`NativeAudioProcessor`) featuring:
  - EBU R128 LUFS loudness normalization with -1.0 dBTP true-peak ceiling.
  - Windowed RMS silence trimming with ~50 ms safety padding and channel boundary preservation.
  - Atomic MP3 encoding via `mp3lame-encoder` with verbatim ID3 tag retention.
- Native GTK3 Audio Tools dialog with non-blocking GLib background worker thread and progress streaming.
- Relm4/GNOME symbolic `AudioTools` vector SVG icon embedded in binary.
- Headless GTK GUI testing via `xvfb-run`.
- Release executable size strictly maintained under 2.5 MB ceiling (currently 2.2 MB).

### v0.1.0 (2026-11-20)

- Initial core release with base-26 `[XXYY-filename.mp3]` lexicographical sort prefix.
- Removable media export to connected USB/SD devices with automatic space checks.
- Two-step FAT32/exFAT device reformat with confirmation warning.
- Non-recursive folder scanner and inline ID3v2 tag editor.
- Audio preview player via `rodio` with play, pause, and stop controls.
- 10-step bounded in-memory undo/redo stack.
- Multi-format playlist exporter (`.txt`, `.rtf`) to `playlist-text/` folder with clear-all behavior.
- Primitive device compatibility audit CLI (`check-compat`).
- GTK3 native desktop GUI with independent dark/light themes.
- Application icon branding assets added under `img/logo/`.

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
