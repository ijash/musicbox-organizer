<p align="center">
  <img src="img/logo/png/logo_mbo_256.png" alt="MusicBox Organizer Logo" width="128" height="128" />
</p>

# MusicBox Organizer

Lightweight, portable desktop MP3 playlist manager and ID3 tag editor designed for portable audio devices and players.

## Overview

Primitive audio hardware (car head units, portable USB/SD players, microcontrollers, and portable speakers) sorts audio files strictly by raw ASCII byte order rather than smart numerical or metadata sorting.

MusicBox Organizer guarantees exact sequential playback on any target player by prepending a clean 4-letter base-26 prefix with a sanitized title slug (`XXYY-<sanitized_id3_title>.mp3`, e.g., `AAAA-first_song.mp3` through `ZZZZ-last_song.mp3`).

## Key Features

- **Pure-Rust Cross-Platform GUI**: Built with `eframe`/`egui` and native `rfd` file dialogs; runs identically on Linux, Windows, and macOS. Standalone binary ceiling `< 20 MB` (~12 MB actual).
- **1-Click Inline Metadata Editing**: Single click to select rows and instantly edit Title and Artist inline with auto-focus and full keyboard navigation (`Enter` commits, `Escape` cancels).
- **Audio Preview**: In-app playback preview with direct track switching (`▶ Play`, `⏸ Pause`, `⏹ Stop`) without needing to click Stop first.
- **Native 1-Pass Audio DSP**: Leading/trailing silence trimming (~20 ms RMS window, 50 ms safety padding) and EBU R128 volume normalization (-14 LUFS target, true-peak limiter) executed in a single loss-free encode pass with zero FFmpeg dependencies and atomic file replacement.
- **Unified Printable Catalog Exporter**: Exports print-ready PDF catalogs with exact OpenType `cmap`+`hmtx` glyph advance measurement, proportional header scaling, coupled divider metrics, and automatic inter-column gap distribution (flush to margin, zero dead space). Includes presets (10×10 cm card, 12×12 cm CD sleeve, A6, A5, A4, or custom down to 1×1 cm), dashed cut guidelines (`0.75 pt`), and JSON themes, alongside Plain Text (`.txt`) and Rich Text (`.rtf`).
- **Safe Removable Media Export**: Dedicated confirmation child viewport showing target device, mount path, and track count. Selecting "Reformat Media" enforces a high-visibility warning and acknowledgment checkbox before executing FAT32/exFAT formatting.
- **Separation of Concerns**: Saving a project (`.txt`) stores playlist order locally without altering source media; Export Media copies and renames tracks directly to the target device.
- **Undo / Redo**: 10-step bounded history for edits, reordering, and track management.
- **Headless GUI Testing**: Cross-platform verification via `egui_kittest` under standard `cargo test` with zero display server / `xvfb-run` requirements.
- **Device Compatibility Audit**: Programmatically verifies filenames and directory trees against ASCII sorting rules and FAT32 path length limits (`check-compat`).

## Download & Quick Start

Pre-compiled standalone executables are available on the [Latest GitHub Release](https://github.com/ijash/musicbox-organizer/releases/latest).

[![GitHub Release](https://img.shields.io/github/v/release/ijash/musicbox-organizer?style=flat-square)](https://github.com/ijash/musicbox-organizer/releases/latest)

| Operating System | Package Archive | Quick Start Steps |
|---|---|---|
| **Linux (x86_64)** | `musicbox-linux-x86_64.tar.gz` | Extract, `chmod +x musicbox`, run `./musicbox` |
| **Windows (x86_64)** | `musicbox-windows-x86_64.zip` | Extract and run `musicbox.exe` |
| **macOS (Apple Silicon ARM64)** | `musicbox-macos-arm64.tar.gz` | Extract, `chmod +x musicbox`, run `./musicbox` |

Verify archive integrity:
```bash
sha256sum -c SHA256SUMS.txt
```

## Building from Source

### Prerequisites

- **Rust**: 2024 edition (`rustc 1.92+`)
- **Linux**: `libasound2-dev`, `libdbus-1-dev`
- **Windows / macOS**: Standard Rust toolchain (no external library dependencies)

### Run GUI

```bash
cargo run
# or
cargo run -- gui
```

### CLI Commands

All core operations are accessible via command line without a display:

| Command | Description |
|---|---|
| `cargo run -- status` | Show active project and connected removable devices |
| `cargo run -- scan [dir]` | Scan directory for MP3 files and inspect ID3 tags |
| `cargo run -- tag <file.mp3> [title] [artist]` | View or update ID3 tags directly |
| `cargo run -- save [path]` | Save playlist project locally (`.txt`) |
| `cargo run -- export` | Export playlist to connected USB/SD device |
| `cargo run -- export-catalog [options]` | Export print-ready catalog (PDF, TXT, RTF) |
| `cargo run -- export-text [dir]` | Export playlist `.txt` and `.rtf` to directory |
| `cargo run -- check-compat [dir]` | Audit directory or media drive for ASCII sort and FAT32 compliance |

### Build Release Binary

```bash
cargo build --release
```

Binary location: `target/release/musicbox` (`< 20 MB` ceiling, ~12 MB actual).

## Testing

Run unit, integration, and headless GUI tests:

```bash
cargo test --all-targets
```

## Creating a Release

Releases are fully automated via GitHub Actions (`.github/workflows/release.yml`). The cross-platform matrix compiles binaries for Linux, Windows, and macOS, runs headless GUI test suites, validates the `< 20 MB` binary size ceiling, computes `SHA256SUMS.txt`, and publishes a GitHub Release.

To cut an official release:

```bash
# 1. Create an annotated git tag
git tag -a v1.0.0 -m "Release v1.0.0"

# 2. Push the tag to GitHub
git push origin v1.0.0
```

## Core Principles

1. **Lightweight First**: Single standalone executable (< 20 MB ceiling, ~12 MB actual). No Electron, WebView, or external runtime dependencies.
2. **Cross-Platform Native**: Single pure-Rust `eframe`/`egui` + `rfd` codebase compiled natively for Linux, Windows, and macOS.
3. **Offline-First & Portable**: No network calls, runs from any directory, leaves zero temporary files behind.
4. **Device Compatibility**: Clean unbracketed `XXYY-<slug>.mp3` sequence guarantees exact playback order on primitive FAT32/USB/SD audio players.

## Roadmap & Feature Plan

- [x] **v0.1.0**: Core playlist ordering, ID3 editing, removable media export, and undo/redo.
- [x] **v0.2.0**: Native audio preview player and non-blocking batch Audio DSP processing.
- [x] **v0.3.0**: Pure-Rust cross-platform GUI migration (`eframe`/`egui`), headless `egui_kittest` testing, and multi-OS CI/CD release pipeline.
- [x] **v1.0.0**: Unified Printable Catalog Exporter (PDF/TXT/RTF), exact Roboto glyph layout, 1-click table editing, and production polish.
- [ ] **Future**: Custom export path templates (e.g., `/{Artist}/{Album}/`).
- [ ] **Future**: FLAC to MP3 on-the-fly transcoding during export.

## Changelog

### v1.0.0 (2026-11-23)

- **Feature 011 (Unified Printable Catalog Exporter)**:
  - Multi-format catalog export for PDF, Plain Text (`.txt`), and Rich Text (`.rtf`).
  - Print-ready PDF card generator with real-world physical dimensions (10×10 cm card, 12×12 cm CD sleeve, A6, A5, A4, or custom down to 1×1 cm).
  - Exact OpenType glyph advance measurement from embedded `Roboto-Regular.ttf` (`cmap`+`hmtx`), replacing heuristic width guesswork.
  - Coupled single-source header metrics and automatic inter-column gap distribution: rightmost column terminates flush at the right margin with zero dead space and zero text overlap/clipping.
  - Dashed cut guidelines (`0.75 pt` width, `[4.0, 2.5]` pattern, `0.5 pt` edge inset) with high contrast against theme backgrounds.
  - Auto-height OS child viewport with zero vertical scrollbars and editable custom header title text.
  - Selectable visual themes via embedded JSON definitions (White Monochrome, Dark Slate, Warm Paper) and support for raster background images.
- **Naming & Device Compatibility (Constitution Principle IX)**:
  - Transitioned from bracketed `[XXXX-file.mp3]` to clean `XXYY-<sanitized_id3_title>.mp3` format (e.g., `AABA-musang_pandan.mp3`).
  - Prioritizes sanitized ID3 Title (lowercased, underscore-delimited), falling back to original filename stem if empty.
- **Playlist Table & UI Polish**:
  - Full-row selection highlighting via `TableRow::set_selected`.
  - 1-click inline cell editing for Title and Artist with immediate auto-focus and keyboard commit/cancel.
  - Active cell edit automatically committed and cleared on track move up/down/removal.
  - Strictly left-aligned cell contents via `Label::truncate` with full-cell click target hitboxes.
- **Audio DSP & Preview Enhancements**:
  - Direct playback switching in Audio Bar without requiring Stop first.
  - Single-pass `process_track` in batch DSP worker preventing double MP3 lossy decode/encode.
  - DSP modal migrated to dedicated non-resizable centered OS child viewport.
  - Fixed dead Cancel button bug in DSP dialog.
- **Removable Media Export Safety**:
  - Dedicated separate confirmation viewport before Removable Media Export.
  - Two-step destructive warning with mandatory acknowledgment checkbox for drive reformatting.
  - Honest unsupported error reporting on Windows format and eject stubs.

### v0.3.0 (2026-11-22)

- **Feature 010 (Unified Cross-Platform GUI)**:
  - Migrated UI from Linux-only GTK3 to single pure-Rust cross-platform `eframe`/`egui` (glow backend) and `rfd` native platform file dialogs.
  - Native GUI window runs identically on Linux, Windows, and macOS with scrollable playlist table, inline Title/Artist editing, reordering, and bounded Undo/Redo.
  - Complete removal of legacy GTK3, Cocoa, and Objective-C dependencies and files.
  - Headless cross-platform GUI test harness via `egui_kittest` running under standard `cargo test` with zero display server / `xvfb-run` requirement.
  - Windows console attachment distinguishing CLI subcommands from GUI double-click launches.
  - Platform-native removable media enumeration for Linux (`/sys/block`), Windows (Win32 `GetLogicalDrives`/`GetDriveTypeW`), and macOS.
  - Two-step confirmation guard for media reformatting before export.
  - In-app audio preview playback controls (`▶ Play`, `⏸ Pause`, `⏹ Stop`) and non-blocking batch Audio DSP dialog.
  - Dark and Light theme toggle using high-contrast standard Unicode glyphs.
  - Standalone release binary size strictly constrained under `< 20 MB`.
  - Multi-OS CI workflow in `.github/workflows/release.yml` executing `cargo test` across Linux, Windows, and macOS.

### v0.1.0 (2026-11-20)

- Initial core release with base-26 lexicographical sort prefix.
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

Distributed under the GNU General Public License v2.0 (GPL-2.0). See [LICENSE](LICENSE) for more information.
