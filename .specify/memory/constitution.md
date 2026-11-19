<!-- SYNC IMPACT REPORT
Version: 1.4.0 → 1.5.0 (MINOR - add mandatory rule to always rebuild binary after each implementation)
Modified Principles: VII (Rapid Development - add mandatory binary rebuild requirement)
Added Principles: None
Modified Sections: Development Workflow (add Rebuild Binary After Implementation requirement)
Removed Sections: None
Ratification: 2026-11-19
TODO: None
-->

# MusicBox Organizer Constitution

Desktop MP3 playlist & tag manager: lightweight, portable, multi-platform.

## Core Principles

### I. Lightweight First
Zero bloat. Single executable, minimal footprint. No Electron, no web frameworks, no runtime dependencies beyond core platform libraries. Every feature: measured against binary size and startup time. Portability > features.

### II. Cross-Platform Native
Windows, macOS, Linux supported equally from day one. Native UI per platform (not emulated). Platform-specific APIs preferred over abstraction layers when they reduce bundle size or startup latency.

### III. User-Centric Simplicity
Playlist and ID3 tag editing must be intuitive for non-technical users. Drag-and-drop, bulk operations, undo/redo. No buried menus. Core workflows: discoverable in <30 seconds.

### IV. Offline-First & Portable
No network calls required. No installer. App runs from any folder. No temp files left behind. Users move the executable and continue working. Local file system is single source of truth.

### V. Testable Core Logic
Business logic (playlist parsing, ID3 editing, file scanning) decoupled from UI. Unit tests required for all non-trivial logic. UI can be thin; correctness lives in core.

### VI. Git-First Workflow & Conventional Commits
Agent handles all commits following git best practices and Conventional Commits v1.0.0 format. Commits atomic and logically coherent. Branches per feature/fix. PR required before merge to main. **Commit message format**: `type(scope): subject` with optional body and footer (Co-authored-by trailer required). Types: `feat:` (new feature), `fix:` (bug fix), `docs:` (documentation), `refactor:` (code restructuring), `test:` (test additions), `chore:` (build/tooling). Subject: lowercase, <50 chars, imperative mood. Body: wrap <72 chars, explain why not what. Force-push forbidden on shared branches. Agent rebases on main before merge to keep history clean. **Auto-commit on feature completion**: When a feature passes all tests and acceptance criteria, agent commits automatically without explicit instruction, organizing commits by feature (one logical unit per commit).

### VII. Rapid Development & Continuous Rebuild
Speed matters. Short iteration cycles, fast feedback. No premature optimization or over-engineering. Ship working features quickly, then refine. Toolchain must support fast rebuild/reload. Dev builds unoptimized, release builds optimized. Use platform-native tools to maximize build speed. **Mandatory binary rebuild**: Every implementation task or feature completion MUST conclude by rebuilding the runnable binary (`cargo build --release` or `cargo build`) so the application is always ready for immediate execution and testing by the user.

### VIII. DRY & KISS
Don't Repeat Yourself: shared logic extracted into reusable functions/modules. Keep It Simple, Stupid: prefer straightforward solutions over clever ones. No abstraction layers without concrete justification. Code speaks for itself; comments only for non-obvious intent.

### IX. Device Compatibility & Lexicographic Sorting
Exported playlists MUST be compatible with primitive audio devices (portable speakers, car head units, microcontrollers, SD card readers) that use lexicographical (ASCII/alphabetical) sorting. File naming convention [XXYY-filename.mp3] is designed to sort correctly on devices using byte-order ASCII sort, not smart sorting. No unicode edge cases. Files on USB/SD card must play in sequence on any device without renumbering or custom sort algorithms.

## Technical Constraints

**Tech Stack**: Native platform UI frameworks only (Win32/WinRT, Cocoa/SwiftUI, GTK3+). Core logic in Rust, C++, or Go for performance and portability. No web technologies in shipped binary.

**Dependencies**: Minimalist. ID3 parsing: native or minimal pure implementation (no ffmpeg). Build system: platform native (MSVC, Xcode, GCC/Clang).

**Testing**: Core logic requires unit tests before merge. Integration tests per platform use real MP3 files. Local machine (this development machine) is primary test target. Cross-machine testing deferred: validate feature works locally first, then expand to other platforms/machines in later phases. No external test infrastructure required initially.

## Development Workflow

**Rebuild Binary After Implementation**: The agent/developer MUST always rebuild the application binary (`cargo build --release` and `cargo build`) immediately upon completing each implementation task, bug fix, or feature before declaring work done. The runnable binary must always be current and testable by the user without manual rebuild steps.

**Code Review**: Every PR verifies cross-platform compatibility. At least one test run per target OS before merge.

**Release**: Single automated build per platform produces standalone executable. No version check; no auto-update (users download new version manually).

## Governance

Constitution is law. Violations caught in code review and PR testing gates. Amendments require ratification; version bumps follow semver (MAJOR: principle removal/redefinition, MINOR: new principle or constraint, PATCH: clarification/wording).

**Git Governance**: All commits agent-created using Conventional Commits v1.0.0 format per principle VI. No manual commits allowed. Agent creates branches, manages PRs, handles rebasing/merging. Commits include `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>` trailer. Agent auto-commits when features complete (pass tests + acceptance criteria) without requiring explicit instruction. PRs reviewed before merge. No merging without passing CI tests.

**Version**: 1.5.0 | **Ratified**: 2026-11-19 | **Last Amended**: 2026-11-20
