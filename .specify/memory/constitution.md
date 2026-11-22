<!-- SYNC IMPACT REPORT
Version: 1.8.0 → 1.9.0 (Minor amendment for Removable Media destructive export confirmation gates and clean unbracketed XXYY filename prefix format)
Modified Principles:
- III. User-Centric Simplicity → III. User-Centric Simplicity & Destructive Safety (mandated dedicated separate confirmation dialog window before Removable Media Export, with high-visibility warning and second confirmation gate when Reformat Media is checked)
- IX. Device Compatibility & Lexicographic Sorting → IX. Device Compatibility & Naming (updated mandatory export filename prefix from `[XXYY-filename.mp3]` to `XXYY-filename.mp3` without square brackets for primitive FAT32/USB/SD audio players)
Modified Sections: None
Added Sections: None
Removed Sections: None
Ratification: 2026-11-19
TODO: None
-->

# MusicBox Organizer Constitution

Desktop MP3 playlist & tag manager: lightweight, portable, multi-platform.

## Core Principles

### I. Lightweight First
Zero bloat. Single self-contained executable with `eframe`/`egui`, minimal footprint. Standalone release binary ceiling: `< 20 MB`. No Electron, no WebView, no external runtime dependencies. Every feature: measured against binary size and startup time. Portability > features.

### II. Cross-Platform Native
Windows, macOS, and Linux supported equally from day one. Single pure-Rust cross-platform GUI (`eframe`/`egui` + `rfd` native file dialogs) compiled natively for Linux, Windows, and macOS. Platform-specific APIs preferred over abstraction layers only when required for native integration (e.g. removable media detection, native file dialogs) while keeping UI logic unified.

### III. User-Centric Simplicity & Destructive Safety
Playlist and ID3 tag editing must be intuitive for non-technical users. Drag-and-drop, bulk operations, undo/redo. No buried menus. Core workflows: discoverable in <30 seconds. **Destructive Safety Gates**: Any destructive or drive-level action requires explicit confirmation. A dedicated separate confirmation dialog window MUST be presented before executing Removable Media Export. Whenever "Reformat Media" is selected/checked, the dialog MUST display an explicit, high-visibility destructive warning and require a second confirmation gate to prevent accidental drive erasure.

### IV. Offline-First & Portable
No network calls required. No installer. App runs from any folder. No temp files left behind. Users move the executable and continue working. Local file system is single source of truth.

### V. Testable Core Logic & GUI Interaction Verification
Business logic (playlist parsing, ID3 editing, file scanning) decoupled from UI. Unit tests required for all non-trivial logic. **Mandatory GUI testing**: Because this is a desktop GUI application, test validation MUST include executing GUI verification. Cross-platform headless GUI verification MUST be supported via `egui_kittest` under `cargo test` across Linux, Windows, and macOS. Interactive UI state flows (clicks, row edits, reordering, theme toggling, dialogs) and UI state bindings MUST be verified automatically to catch regressions before declaring tasks done.

### VI. Git-First Workflow & Conventional Commits
Agent handles all commits following git best practices and Conventional Commits v1.0.0 format (`https://www.conventionalcommits.org/en/v1.0.0/`). Commits atomic and logically coherent. **Mandatory Branch per Work Item**: Always create a dedicated new branch off `Development` before starting ANY feature, bugfix, refactor, docs, CI, or chore task. Never commit feature or bugfix work directly onto `Development` or `main`. **Conventional Branch Naming**: Branches MUST be named in `<type>/<kebab-case-slug>` format: `feat/<num>-<slug>` (e.g. `feat/010-cross-platform-gui`), `fix/<slug>` (e.g. `fix/dark-icons-compact-buttons`), `docs/<slug>`, `ci/<slug>`, `refactor/<slug>`, `chore/<slug>`, or `test/<slug>`. **Commit message format**: `type(scope): subject` with optional body and footer (Co-authored-by trailer required). Types: `feat:` (new feature), `fix:` (bug fix), `docs:` (documentation), `refactor:` (code restructuring), `test:` (test additions), `chore:` (build/tooling). Subject: lowercase, <50 chars, imperative mood. Body: wrap <72 chars, explain why not what. Force-push forbidden on shared branches. **Auto-commit on feature completion**: When a feature passes all tests and acceptance criteria, agent commits automatically on the dedicated work branch without explicit instruction, organizing commits by feature (one logical unit per commit). **Explicit Merge Approval Only**: Agents MUST NOT merge any branch into `Development` or `main` (and MUST NOT push release tags) unless explicitly instructed by the user. After completing work on a branch, leave it committed on that branch and report readiness for review/merge.

### VII. Rapid Development & Continuous Rebuild
Speed matters. Short iteration cycles, fast feedback. No premature optimization or over-engineering. Ship working features quickly, then refine. Toolchain must support fast rebuild/reload. Dev builds unoptimized, release builds optimized. Use platform-native tools to maximize build speed. **Mandatory binary rebuild**: Every implementation task or feature completion MUST conclude by rebuilding the runnable binary (`cargo build --release` or `cargo build`) so the application is always ready for immediate execution and testing by the user.

### VIII. DRY & KISS
Don't Repeat Yourself: shared logic extracted into reusable functions/modules. Keep It Simple, Stupid: prefer straightforward solutions over clever ones. No abstraction layers without concrete justification. Code speaks for itself; comments only for non-obvious intent.

### IX. Device Compatibility & Naming
Exported playlists MUST be compatible with primitive audio devices (portable speakers, car head units, microcontrollers, SD card readers) that use lexicographical (ASCII/alphabetical) sorting. Mandatory file naming convention is `XXYY-<slug>.mp3` (no square brackets, e.g., `AAAA-song.mp3`, `AAAB-song.mp3`), where `<slug>` prioritizes the track's ID3 Title (lowercased, spaces and unsafe characters converted to `_`, collapsed/trimmed, e.g., `"Musang Pandan"` → `AABA-musang_pandan.mp3`), falling back to the sanitized original filename stem if ID3 Title is empty, for clean sorting and reliable compatibility across primitive FAT32/USB/SD audio players using byte-order ASCII sort. No unicode edge cases. Files on USB/SD card must play in sequence on any device without renumbering or custom sort algorithms.

## Technical Constraints

**Tech Stack**: Single pure-Rust cross-platform GUI using `eframe`/`egui` with `rfd` for native platform file dialogs. Core logic in Rust. No Electron, WebView, or web technologies in shipped binary.

**Dependencies**: Minimalist. ID3 parsing: native or minimal pure implementation (no ffmpeg). Build system: Cargo native across Linux, Windows, and macOS.

**Testing**: Core logic requires unit tests before merge. Integration tests per platform use real MP3 files. **GUI Testing Harness**: Cross-platform headless GUI verification via `egui_kittest` under `cargo test` across Linux, Windows, and macOS. Exercises UI state, interactions, and component rendering without requiring display servers. Local machine is primary test target.

## Development Workflow

**Agent Skills Routing**: Agents MUST invoke relevant skills before performing task phases: load `rust-gui` before writing, refactoring, or reviewing Rust application or `eframe`/`egui` GUI code; load `egui-gui-testing` before writing or running headless GUI tests (`egui_kittest`) and service regression tests. `.specify/memory/constitution.md` always takes precedence over any generic advice inside external skills.

**Branch Isolation & Conventional Branch Naming**: Before making any code, doc, or test edits for a task, create and checkout a dedicated branch branched off `Development` using Conventional Commits naming (`<type>/<kebab-case-slug>`). Features: `feat/<num>-<slug>` (e.g. `feat/010-cross-platform-gui`). Bugfixes: `fix/<slug>`. Maintenance/Tooling: `chore/<slug>`, `docs/<slug>`, `ci/<slug>`, `refactor/<slug>`, `test/<slug>`. Direct commits to `Development` or `main` are strictly forbidden.

**Rebuild Binary After Implementation**: The agent/developer MUST always rebuild the application binary (`cargo build --release` and `cargo build`) immediately upon completing each implementation task, bug fix, or feature before declaring work done. The runnable binary must always be current and testable by the user without manual rebuild steps.

**GUI Interaction Verification**: When delivering or modifying UI features, the agent/developer MUST execute GUI interaction tests via `egui_kittest` under `cargo test` across platforms to verify that UI flows and event handlers do not panic or regress under user interaction.

**Changelog & Roadmap in README**: Upon each version bump or release boundary, the developer/agent MUST document the release changelog and the upcoming feature plan / roadmap directly inside `README.md`. This maintains single-source transparency for users regarding what was shipped and what is planned next.

**Code Review**: Every PR verifies cross-platform compatibility. At least one test run per target OS before merge.

**Release**: Single automated build per platform produces standalone executable. No version check; no auto-update (users download new version manually). Release tags (`v*`) MUST NOT be pushed without explicit user instruction.

## Governance

Constitution is law. Violations caught in code review and PR testing gates. Amendments require ratification; version bumps follow semver (MAJOR: principle removal/redefinition, MINOR: new principle or constraint, PATCH: clarification/wording).

**Git Governance**: All commits agent-created using Conventional Commits v1.0.0 format per principle VI. No manual commits allowed. Agent creates dedicated branches (`<type>/<kebab-case-slug>`) branched off `Development`. Direct commits to `Development` or `main` are prohibited. Commits include `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>` trailer. Agent auto-commits on the dedicated work branch when features complete (pass tests + acceptance criteria) without requiring explicit instruction. Agents MUST NOT merge into `Development` or `main` without explicit user instruction. PRs reviewed before merge. No merging without passing CI tests.

**Version**: 1.9.0 | **Ratified**: 2026-11-19 | **Last Amended**: 2026-11-22

