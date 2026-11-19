# Quickstart: Device Compatibility Verification

## Validation Scenarios

### 1. ASCII Byte-Order Sort Test
```sh
cargo test test_lexicographic_ascii_sort_integrity
```
Expected: Generating 1,000 sequential `[XXYY-filename.mp3]` names and running raw Rust `sort()` produces 100% exact match to index sequence.

### 2. Non-ASCII Sanitization & Rejection
```sh
cargo test test_non_ascii_filename_detection
```
Expected: Filenames with Cyrillic, Chinese, or unicode emojis are flagged as violations and sanitized cleanly to pure ASCII.

### 3. Folder Compatibility Audit CLI
```sh
cargo run -- check-compat /media/ijash/CCTV
```
Expected: Outputs CompatibilityReport detailing total files, ASCII compliance, sort order integrity, and pass/fail status.
