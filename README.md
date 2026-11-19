# musicbox-organizer

Simple MP3 playlist organizer with ID3 editing for portable audio devices.

## Overview

musicbox-organizer is a lightweight desktop application for organizing MP3 playlists and editing ID3 tags. Designed for portable audio devices (USB/SD card, car head units, microcontrollers) that use lexicographic sorting, ensuring playlists play in correct sequence on any device without custom firmware.

## Features

- **Playlist Organization**: Drag-and-drop file grouping with automatic track numbering
- **ID3 Tag Editing**: Edit title, artist, album, and track numbers
- **Smart Export**: Export to USB/SD card with automatic device selection and optional reformat
- **Device Compatibility**: Naming convention ensures playback order on primitive audio devices
- **Dark Mode**: Default dark theme with light mode option
- **Undo/Redo**: Full edit history with 10-step undo depth
- **Cross-Platform**: Native UI for Windows, macOS, Linux

## Getting Started

### Prerequisites

- Rust 1.70+ (for building from source)
- macOS 10.15+ / Windows 10+ / Linux (GTK3+)

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

## Project Goals

- **Lightweight**: Single executable, <50MB, no runtime dependencies
- **Portable**: Run from any folder, no installer required
- **Offline-First**: Zero network calls, all operations local
- **Simple**: Intuitive UI discoverable in <30 seconds

## Documentation

- [Project Constitution](/.specify/memory/constitution.md) — Development principles & governance
- [Feature Specs](/specs/) — Feature specifications and acceptance criteria
- [Implementation Plan](/specs/001-mp3-file-organizer/plan.md) — Detailed task breakdown

## Development

This project follows strict principles for rapid, reliable development:

- **Native First**: Platform-native UI frameworks, no web technologies
- **DRY & KISS**: Shared logic extracted, simple solutions preferred
- **Testable Core**: Business logic decoupled from UI, unit tested
- **Git-Driven**: All commits by agent following git best practices

See [Constitution](/.specify/memory/constitution.md) for full governance.

## Testing

Tests run locally on the development machine. Cross-platform testing deferred to later phases.

```bash
cargo test
```

## License

[Add license info]

## Support

For issues and feature requests, see [Issues](https://github.com/ijash/musicbox-organizer/issues).
