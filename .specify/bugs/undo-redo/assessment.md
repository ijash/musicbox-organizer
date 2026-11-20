# Bug Assessment: Undo/Redo Clears Up Playlist

- **Slug**: undo-redo
- **Created**: 2026-11-20
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> "undo redo not working. it clears up the playlist. slug=undo-redo"

## Symptom

When a user performs edits or moves tracks in the GTK3 GUI and clicks Undo (or Redo), the entire playlist table becomes empty, wiping all loaded tracks from the view.

## Reproduction

1. Launch `cargo run -- gui` or `./target/release/musicbox`.
2. Add audio files or click "📁 Scan Folder..." to load tracks into the playlist.
3. Edit a track title or artist inline, or click "▲ Up" / "▼ Down" to reorder.
4. Click "↺ Undo" in the top header bar.
5. Observed: All tracks disappear from the playlist (empty table). Expected: The last title/artist edit or track reorder is reverted while preserving the rest of the playlist.

## Suspected Code Paths

- `src/ui/gtk_ui.rs:133` — `session_state` initialized with empty `tracks: Vec::new()`.
- `src/ui/gtk_ui.rs:420-455` — Inline edit handlers call `session_state.borrow_mut().push_undo_checkpoint()` without synchronizing `session_state.tracks` from `project.tracks`.
- `src/ui/gtk_ui.rs:475-520` — Move Up and Move Down handlers mutate `project.tracks` directly, leaving `session_state.tracks` empty or stale.
- `src/ui/gtk_ui.rs:490-530` — Undo and Redo handlers overwrite `project.tracks` with `state.tracks`, which is empty `[]`.
- `src/models/session_state.rs:32-60` — `push_undo_checkpoint()` snapshots `self.tracks`, which is never populated by `gtk_ui.rs`.

## Root Cause Hypothesis

**Confidence: High**

In `src/ui/gtk_ui.rs`, application state is split between two separate structs: `project` (`Rc<RefCell<ProjectFile>>`) and `session_state` (`Rc<RefCell<SessionState>>`). All file scanning, file adding, project loading, and row swapping mutate `project.borrow_mut().tracks`, but never update `session_state.borrow_mut().tracks`.

Because `session_state.tracks` remains `[]` (empty), every call to `session_state.push_undo_checkpoint()` saves an empty list onto the undo stack. When the user clicks the Undo button, `undo_btn.connect_clicked` calls `state.undo()` and replaces `p.tracks` with `state.tracks`, wiping out all tracks and rendering an empty table.

## Proposed Remediation

**Preferred**:
Unify playlist tracking in `src/ui/gtk_ui.rs` or ensure `session_state.tracks` is always synchronized with `project.tracks`:
1. In `src/ui/gtk_ui.rs`, create a helper `sync_session_from_project(session_state, project)` called whenever tracks are added, scanned, loaded, or edited.
2. In `push_undo_checkpoint()` callers (inline title/artist edits, Move Up, Move Down), sync `session_state.tracks` from `project.tracks` *before* snapshotting.
3. When `state.undo()` or `state.redo()` completes, restore `project.tracks` and call `refresh_table()` and `update_devices_ui()`.
4. Add unit / UI tests in `tests/unit/undo_tests.rs` ensuring undo/redo preserves existing tracks when reverting metadata or position edits.

**Files likely to change**:
- `src/ui/gtk_ui.rs`
- `tests/unit/undo_tests.rs`

**Tests to add or update**:
- Unit test in `tests/unit/undo_tests.rs` verifying that updating track metadata or positions and calling `undo()` restores the prior track attributes without clearing the playlist or losing other tracks.

## Risks & Considerations

- Low risk. The fix only aligns the state tracking between GTK UI events and `SessionState`. Core models and services remain untouched.

## Open Questions

None. The root cause is fully identified and reproducible.
