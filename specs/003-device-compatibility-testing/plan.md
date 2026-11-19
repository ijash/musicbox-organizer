# Implementation Plan: Device Compatibility Testing

**Branch**: `003-device-compatibility-testing` | **Date**: 2026-11-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/003-device-compatibility-testing/spec.md`

## Summary
Ensure exported playlists and filenames strictly adhere to primitive device playback constraints: ASCII byte-order sorting integrity, non-ASCII/unicode detection and sanitization, FAT32 path length boundaries, and a CLI/GUI compatibility checker.

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.75+)
**Primary Dependencies**: Built-in Rust stdlib (pure string and path manipulation)
**Storage**: N/A (direct filesystem inspection)
**Testing**: `cargo test` unit and integration suites
**Target Platform**: Portable MP3 players, car head units, microcontrollers
**Performance Goals**: Validate 10,000 files in <200ms
**Constraints**: Zero external dependencies; 100% pure ASCII byte sort order

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Lightweight First)**: PASS. Zero new dependencies; pure stdlib string inspection.
- **Principle V (Testable Core Logic)**: PASS. Exhaustive property tests for sequence and ASCII safety.
- **Principle VII (Rapid Development & Continuous Rebuild)**: PASS. Rebuild binary on completion.
- **Principle IX (Device Compatibility & Lexicographic Sorting)**: PASS. Directly implements and guarantees Principle IX.

## Project Structure

### Documentation

```text
specs/003-device-compatibility-testing/
├── plan.md              # This file
├── research.md          # ASCII sort and FAT32 boundaries
├── data-model.md        # CompatibilityReport, DeviceProfile
├── quickstart.md        # Test scenarios
├── contracts/           # DeviceCompatibilityChecker contract
└── tasks.md             # Tasks
```

### Source Code

```text
src/
├── models/
│   └── compatibility.rs    # CompatibilityReport entity
├── services/
│   └── compatibility.rs    # Verification, ASCII check, and sanitization
└── main.rs                 # check-compat CLI subcommand

tests/
├── unit/
│   └── compatibility_tests.rs
```

**Structure Decision**: Single project architecture. Add compatibility model and service, integrate with CLI and test suites.

## Complexity Tracking
*No violations. All design choices strictly align with Constitution.*
