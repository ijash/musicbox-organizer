# Research: Device Compatibility Testing & ASCII Verification

**Feature Branch**: `003-device-compatibility-testing`
**Date**: 2026-11-20

## 1. ASCII Byte-Order Sort Verification

### Decision
Implement programmatic byte-order sort validator in `src/services/compatibility.rs`.

### Rationale
- Primitive firmware (car head units, microcontrollers, cheap MP3 chips like JL/Action) sorts FAT directory entries using raw `strcmp` or ASCII byte comparison without natural sorting (i.e. '10' sorts before '2').
- Base-26 alphabetic scheme `AAAA`..`ZZZZ` (and `[XXYY-...]`) has fixed width (4 characters) and uppercase/lowercase consistency, guaranteeing that byte sort order strictly equals playlist index order.
- Programmatic test generates 1,000+ sequential filenames and verifies `names.sort()` produces an identical array to the original index sequence.

---

## 2. Filename Sanitization & ASCII Guard

### Decision
Add `sanitize_ascii_filename()` and `validate_ascii_filename()` to reject or sanitize non-ASCII/unicode characters in export filenames.

### Rationale
- Many microcontrollers and automotive USB readers crash, skip, or misorder FAT directory entries when unicode (UTF-8 multibyte) characters or special symbols are present.
- Sanitizer replaces non-ASCII with clean ASCII transliteration or underscores, guaranteeing 100% ASCII safety per Principle IX.

---

## 3. FAT32/exFAT Path Length & Cluster Compatibility

### Decision
Enforce 255-character maximum path length for FAT32 LFN and verify 8.3 base compatibility.

### Rationale
- Standard FAT32 allows max 255 UTF-16 code units for Long File Names (LFN).
- Warns if any filename approaches or exceeds length boundaries.
