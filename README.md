<p align="center">
  <img src="img/logo/png/logo_mbo_256.png" alt="MusicBox Organizer Logo" width="128" height="128" />
</p>

# MusicBox Organizer

Lightweight, portable desktop MP3 playlist manager and ID3 tag editor designed for portable audio devices and players.

## Overview

MusicBox Organizer is built specifically to solve playlist sorting issues on primitive audio hardware (car head units, portable USB/SD players, microcontrollers, and portable speakers). These devices sort files strictly using raw ASCII byte order rather than smart numerical or metadata sorting.

The application automatically assigns a 4-letter base-26 prefix (`AAAA-filename.mp3` through `ZZZZ-filename.mp3`), guaranteeing exact sequential playback on any target player.

## Features

- **Playlist Reordering**: Add files, reorder tracks up and down with persistent row selection.
- **Inline ID3 Editing**: Double-click to edit Title and Artist directly in the table.
- **Save vs Export Separation**:
  - **Save Project (`.txt`)**: Persists playlist structure locally for future editing without modifying media.
  - **Export Media**: Writes files with `XXYY-filename.mp3` naming directly to USB or SD cards.
- **Device Detection**: Detects connected removable storage, capacity, free space, and filesystem type (FAT32/exFAT).
- **Two-Step Reformat Confirmation**: Optional FAT32/exFAT format before export with an explicit confirmation dialog to prevent accidental data loss.
- **Unified Printable Catalog Exporter**: Exports print-ready PDF catalogs with smart multi-column autofit, physical sizing (10×10 cm card, 12×12 cm CD cover, A6, A5, A4, or custom down to 1×1 cm), embedded Roboto typography, cut guidelines, and JSON visual themes, alongside Plain Text (`.txt`) and Rich Text (`.rtf`) formats.
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
| **Linux (x86_64)** | `musicbox-linux-x86_64.tar.gz` | Extract, `chmod +x musicbox`, run `./musicbox` (pure-Rust `eframe` GUI + CLI). |
| **Windows (x86_64)** | `musicbox-windows-x86_64.zip` | Extract and run `musicbox.exe` (pure-Rust `eframe` GUI + CLI). |
| **macOS (Apple Silicon ARM64)** | `musicbox-macos-arm64.tar.gz` | Extract, `chmod +x musicbox`, run `./musicbox` (pure-Rust `eframe` GUI + CLI). |

Verify archive integrity:
```bash
sha256sum -c SHA256SUMS.txt
```

## Building from Source

### Prerequisites

- **Rust**: 2024 edition (`rustc 1.85+`)
- **Linux**: `libasound2-dev`, `libdbus-1-dev` (pure Rust GUI, zero GTK runtime dependencies)
- **Windows / macOS**: Standard Rust toolchain (no external library dependencies)

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
| `cargo run -- export-catalog [options]` | Export print-ready catalog (PDF, TXT, RTF) with presets, custom dimensions, and themes |
| `cargo run -- export-text [dir]` | Export playlist `.txt` and `.rtf` to `playlist-text/` directory |
| `cargo run -- check-compat [dir]` | Audit directory or media drive for ASCII sort and FAT32 compliance |

### Build Release Binary

To build a standalone, optimized executable (< 20 MB ceiling, ~7.8 MB actual) with no external web runtime dependencies:

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

Run unit, integration, and headless GUI tests across all platforms without requiring display servers (`xvfb-run`):

```bash
cargo test
```

## Creating a Release

Releases are fully automated via GitHub Actions (`.github/workflows/release.yml`). The cross-platform matrix compiles binaries for Linux, Windows, and macOS, validates automated headless `egui_kittest` GUI and binary size tests (< 20 MB), computes `SHA256SUMS.txt`, and publishes a GitHub Release.

To cut an official release:

```bash
# 1. Create an annotated git tag
git tag -a v0.3.0 -m "Release v0.3.0"

# 2. Push the tag to GitHub
git push origin v0.3.0
```

> **Note on CI Quotas**: Standard branch pushes (`Development`, `main`) do not invoke the multi-OS build matrix to preserve private runner minutes. For on-demand pre-release smoke testing across all 3 operating systems without publishing a public release, trigger the workflow manually via GitHub Actions (`workflow_dispatch`).

## Core Principles

1. **Lightweight First**: Single standalone executable (< 20 MB ceiling, ~7.8 MB actual). No Electron, no WebView, no web runtime.
2. **Cross-Platform Native**: Single pure-Rust cross-platform GUI (`eframe`/`egui` + `rfd`) compiled natively for Linux, Windows, and macOS.
3. **Offline-First & Portable**: No network calls, runs from any directory, leaves no temporary files.
4. **Device Compatibility**: Lexicographical `[XXYY-filename.mp3]` sequence guarantees sequential playback on primitive audio hardware.

## Roadmap & Feature Plan

Planned enhancements for upcoming versions:

- **v0.4.0**: Unified Printable Catalog Exporter (PDF, TXT, RTF) with custom physical sizing, multi-column autofit, and themes (supersedes planned DOCX export).
- **v0.4.0**: Custom export path templates (e.g., `/Artist/Album/`).

## Changelog

### v0.3.0 (2026-11-22)

- **Feature 010 (Unified Cross-Platform GUI)**:
  - Migrated UI from Linux-only GTK3 to single pure-Rust cross-platform `eframe`/`egui` (using `glow` backend) and `rfd` native platform file dialogs.
  - Native GUI window runs identically on Linux, Windows, and macOS with scrollable playlist table, inline Title/Artist editing, reordering, and bounded Undo/Redo.
  - Complete removal of legacy GTK3, Cocoa, and Objective-C dependencies and files.
  - Headless cross-platform GUI test harness via `egui_kittest` running under standard `cargo test` with zero display server / `xvfb-run` requirement.
  - Windows console attachment distinguishing CLI subcommands from GUI double-click launches.
  - Platform-native removable media enumeration for Linux (`/sys/block`), Windows (Win32 `GetLogicalDrives`/`GetDriveTypeW`), and macOS.
  - Two-step confirmation guard for media reformatting before export.
  - In-app audio preview playback controls (`▶ Play`, `⏸ Pause`, `⏹ Stop`) and non-blocking batch Audio DSP dialog.
  - Dark and Light theme toggle using high-contrast standard Unicode glyphs.
  - Standalone release binary size strictly constrained under `< 20 MB` (~7.8 MB actual).
  - Multi-OS CI workflow in `.github/workflows/release.yml` executing `cargo test` across Linux, Windows, and macOS.
  - Compact desktop button padding (`2px 6px`) and min-height (`24px`).
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
