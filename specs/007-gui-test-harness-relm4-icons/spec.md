# Feature Specification: Automated GUI Test Harness and Standardized Relm4 Icons

**Feature Branch**: `007-gui-test-harness-relm4-icons`

**Created**: 2026-11-20

**Status**: Draft

**Input**: User description: "- buat spek script/harness untuk testing gui. pake `Relm4`, `xvfb` - buat icon2 terstandarisasi pakai https://github.com/Relm4/icons . dan wajib dimasukan ke binary untuk icon yang terpakai."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Headless & Automated GUI Interaction Testing (Priority: P1)

Developers and automated CI/local pipelines run an automated GUI testing harness in headless environments (using `xvfb-run`). The harness launches the actual desktop interface, dispatches real user actions (row selection, move up/down, tag edits, remove, audio play/stop, theme toggle, and export trigger), and asserts that state updates correctly without crashes, panics, or unhandled exceptions.

**Why this priority**: Directly enforces Constitution Principle V (Testable Core Logic & GUI Interaction Verification). Eliminates regressions such as GTK signal crashes or `RefCell` borrow panics before declaring tasks complete.

**Independent Test**: Can be tested by executing the automated test script (`./tests/gui/run_gui_tests.sh` or `cargo test --test gui_tests`) under `xvfb-run` without a physical display attached.

**Acceptance Scenarios**:

1. **Given** headless Linux environment without active display, **When** running GUI testing harness under `xvfb-run`, **Then** the app initializes cleanly and renders window widgets off-screen without errors.
2. **Given** GUI test suite, **When** executing button clicks (add track, move up/down, remove track, theme toggle, audio controls), **Then** GTK event loops process all signals and assert final UI and model states match expectations.
3. **Given** test harness execution, **When** all interactive steps finish, **Then** application cleanly terminates and outputs a concise pass/fail summary.

---

### User Story 2 - Standardized Embedded Relm4 Icons (Priority: P1)

Application toolbars and buttons display standard, crisp vector/symbolic icons sourced from Relm4 icons (`relm4-icons`). Only the icons actually used by the app are compiled into the binary (via embedded resources / compile-time selection) so the binary remains lightweight (<2 MB) per Constitution Principle I.

**Why this priority**: Replaces ad-hoc unicode symbols (such as `▲`, `▼`, `📁`, `✖`, `💾`, `📂`) with standardized, theme-aware desktop icons that look sharp across dark and light modes.

**Independent Test**: Can be tested by launching the GUI and inspecting the toolbar buttons (Scan, Add, Remove, Move Up, Move Down, Play, Pause, Stop, Save, Open) to verify standardized icons render clearly in both light and dark themes.

**Acceptance Scenarios**:

1. **Given** application toolbar and buttons, **When** rendered, **Then** standard symbolic icons (folder-open, plus, cross/trash, chevron-up, chevron-down, media-play, media-pause, media-stop, document-save, theme-toggle) are displayed.
2. **Given** compiled release binary, **When** checking binary size, **Then** only the selected in-use icons are embedded into the executable with zero unnecessary bloat (<2 MB binary ceiling maintained).
3. **Given** dark and light themes, **When** switching themes, **Then** symbolic icons adapt their foreground fill color dynamically to maintain high contrast.

---

### Edge Cases

- What happens if `xvfb` is missing on a developer's machine? The test runner script should check for `xvfb-run` availability, provide a clear install command (`sudo apt install xvfb`), or fallback to the current display if `$DISPLAY` is set.
- How do embedded Relm4 icons behave on platforms without GNOME icon themes? Because required icons are embedded at compile-time directly in the executable binary, they render 100% reliably on all machines without requiring system icon packs.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a reusable, automated GUI testing harness runnable under `xvfb-run` for headless execution.
- **FR-002**: GUI testing harness MUST programmatically simulate interactive desktop workflows:
  - Add tracks and verify table population.
  - Reorder tracks (Move Up and Move Down) and verify sequence and selection persistence.
  - Inline Title and Artist edits and verify in-place `ListStore` updates.
  - Remove track and verify index recalculation.
  - Audio preview playback triggers (Play, Pause, Stop).
  - Theme toggling (Dark Mode to Light Mode and back).
  - Export dialog open and format selection.
- **FR-003**: System MUST adopt standardized vector icons sourced from `relm4-icons`.
- **FR-004**: Only in-use icons MUST be compiled into the binary to prevent binary bloat (preserving Constitution Principle I: <50 MB target, <2 MB actual).
- **FR-005**: All toolbar buttons (Folder Scan, Add MP3, Remove, Up, Down, Audio Controls, Save, Open, Export) MUST use standardized symbolic icons with appropriate accessibility labels.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Automated GUI test harness executes and passes 100% of simulated interactive click flows under `xvfb-run` in <10 seconds.
- **SC-002**: Release binary size remains under 2.5 MB after embedding Relm4 icons.
- **SC-003**: 100% of existing unit and integration tests (49/49) continue to pass.
- **SC-004**: All toolbar buttons display crisp symbolic icons that render cleanly in both dark and light modes.

## Assumptions

- Headless Linux test runner uses `xvfb` (`xvfb-run -a`).
- `relm4-icons` crate or embedded SVG resource compilation is used for in-use icon embedding.
