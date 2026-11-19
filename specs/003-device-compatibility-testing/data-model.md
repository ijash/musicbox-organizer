# Data Model: Device Compatibility & ASCII Verification

**Feature Branch**: `003-device-compatibility-testing`
**Date**: 2026-11-20

## Entities

### 1. CompatibilityReport
Result of validating a folder or playlist against primitive device compatibility rules.

| Field | Type | Description |
|---|---|---|
| `total_files` | `usize` | Total audio files inspected |
| `is_sorted` | `bool` | True if raw ASCII sort equals intended sequence |
| `ascii_violations`| `Vec<String>` | Files containing non-ASCII characters |
| `length_violations`| `Vec<String>`| Files exceeding FAT32 length limits (255 bytes) |
| `format_violations`| `Vec<String>`| Files not matching `[XXYY-filename.mp3]` pattern |
| `overall_compatible`| `bool` | True if zero violations detected |

### 2. DeviceProfile
Compatibility profile for target audio device categories.

| Device Category | Filesystem | Sorting Mode | Max Path | Max Tracks |
|---|---|---|---|---|
| Portable MP3 Player | FAT32 / exFAT | ASCII Byte-Order | 255 chars | 17,576 |
| Car Head Unit | FAT32 | ASCII Byte-Order | 255 chars | 9,999 |
| Microcontroller / Embedded | FAT32 | ASCII Byte-Order (8.3 or LFN) | 64-255 chars | 1,000 |
