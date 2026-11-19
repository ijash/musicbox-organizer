# Technical Research: MP3 File Organizer & ID3 Editor

**Feature Branch**: `001-mp3-file-organizer`
**Date**: 2026-11-20

## 1. ID3 Tag Parsing & Writing

### Decision
Use `id3` crate (`id3 = "1.14"`).

### Rationale
- Pure Rust implementation with zero external C/FFI dependencies.
- Full support for ID3v1, ID3v2.2, ID3v2.3, and ID3v2.4.
- Clean API for reading and writing Title, Artist, Album, and Track number.
- Aligns strictly with Constitution Principle I (Lightweight First, no ffmpeg/bloat) and Principle IV (Offline-First).

### Alternatives Considered
- `symphonia-metadata`: Read-only metadata extraction; cannot write updated ID3 tags back to disk.
- `metaflac`: Already in Cargo.toml for FLAC, but does not support MP3 ID3 container format.
- Hand-rolled parser: High risk of tag corruption on obscure ID3 frames. `id3` crate is battle-tested.

---

## 2. Audio Preview Playback

### Decision
Use `rodio = "0.19"`.

### Rationale
- Standard pure-Rust playback engine built on top of `cpal`.
- Minimal overhead, easy integration for play, pause, stop, and volume/seek controls.
- Works natively on Linux (ALSA), Windows (WASAPI), and macOS (CoreAudio).
- Satisfies SC-005 (audio preview starts playing within 500ms).

### Alternatives Considered
- Direct platform audio APIs: High complexity across 3 platforms for simple 5-second verification playback.
- `kira`: Game-audio oriented, heavier dependency graph.
- Calling external CLI player (`ffplay`/`aplay`): Violates Constitution I (no external runtime dependencies).

---

## 3. Undo/Redo Architecture

### Decision
In-memory Command / Snapshot History Stack (`UndoStack`) bounded to 10 entries per FR-012.

### Rationale
- Each action (tag edit, track reorder, bulk rename) records prior state and next state.
- Max depth of 10 keeps memory footprint negligible (<100 KB for 1,000 tracks).
- Preserves responsive UI with zero disk I/O during editing.
- Cleared upon save per FR-012/FR-013.

### Alternatives Considered
- File-based journal: Excessive disk writes; violates offline-first lightweight guidelines.
- Deep cloning full playlist: For 10 steps of 100 tracks, 10 clones is <200 KB, which is completely trivial and eliminates mutation bugs.

---

## 4. Playlist Export Formats (TXT / RTF / DOCX)

### Decision
- **TXT**: Plain text list with track numbers, titles, and filenames (already supported by `ProjectFile`).
- **RTF**: Built-in pure string template (`{\rtf1\ansi ...}`) without any external crates.
- **DOCX**: Lightweight zipped XML package using `zip` and XML templates, or deferred/delegated cleanly.

### Rationale
- RTF requires no dependencies: it is human-readable, printable, and opens in Word, LibreOffice, and TextEdit.
- Pure Rust string formatting keeps release binary <1 MB.
