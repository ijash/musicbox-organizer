# Quickstart: MP3 File Organizer & ID3 Editor

## Prerequisites
- Rust toolchain (`cargo`, `rustc`)
- Sample MP3 files (e.g. `assets/mp3-sample/sample-3s.mp3`, `assets/mp3-sample/sample-5s.mp3`)

## Validation Scenarios

### 1. Scan & Load MP3 Files
```sh
cargo test test_scan_directory
```
Expected: All `.mp3` files in the directory are detected, while subdirectories and non-audio files are ignored.

### 2. Read & Write ID3 Tags
```sh
cargo test test_id3_read_and_write
```
Expected: Reading `sample-3s.mp3` extracts ID3 title/artist. Updating title to "Test Song" and saving writes ID3v2 frame to file without modifying audio stream.

### 3. Undo/Redo In-Memory Changes
```sh
cargo test test_undo_redo_stack
```
Expected: Editing tags pushes to undo stack. Invoking undo restores previous metadata; redo reapplies edit. Stack capped at 10 items.

### 4. End-to-End Save with Renaming
```sh
cargo test test_organizer_save_and_rename
```
Expected: Files renamed to `[XXYY-filename.mp3]` based on playlist order with updated ID3 tags.
