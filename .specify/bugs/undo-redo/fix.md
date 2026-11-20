# Bug Fix: Undo/Redo Clears Up Playlist

- **Slug**: undo-redo
- **Fixed**: 2026-11-20
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Synchronized `SessionState.tracks` with `ProjectFile.tracks` in `src/ui/gtk_ui.rs` across all playlist lifecycle events (initial load, folder scanning, file addition, open project, inline editing, and reordering). Added regression test `test_undo_preserves_tracks_when_reverting_edits` in `tests/unit/undo_tests.rs`.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/ui/gtk_ui.rs` | modified | Added `sync_session_from_project` closure and invoked before snapshotting and upon loading tracks |
| `tests/unit/undo_tests.rs` | modified | Added `test_undo_preserves_tracks_when_reverting_edits` test |

## Diff Highlights

```rust
// Helper: Synchronize SessionState from ProjectFile
let sync_session_from_project = {
    let project = Rc::clone(&project);
    let session_state = Rc::clone(&session_state);
    move || {
        let p = project.borrow();
        let mut state = session_state.borrow_mut();
        state.tracks = p
            .tracks
            .iter()
            .map(|t| {
                let mut meta = crate::models::TrackMetadata::default();
                meta.title = t.title.clone();
                meta.artist = t.artist.clone();
                meta.track_number = Some(t.index + 1);
                let xxyy = naming::track_index_to_xxyy(t.index).unwrap_or_else(|_| "AAAA".into());
                crate::models::OrganizerTrack::new(
                    t.index,
                    PathBuf::from(&t.file_path),
                    meta,
                    xxyy,
                )
            })
            .collect();
    }
};
```

## Tests Added or Updated

- `tests/unit/undo_tests.rs::test_undo_preserves_tracks_when_reverting_edits` — Verifies undo restores previous title/artist state without clearing the playlist or removing other tracks.

## Local Verification

- Commands run:
  - `cargo test --test unit_tests undo_tests` → 4 passed; 0 failed
  - `cargo test` → 49 total tests passed; 0 failed
  - `cargo build --release` → clean compilation

## Deviations from Assessment

None. Implementation followed preferred remediation exactly.

## Follow-ups

- Run `/speckit-bug-test slug=undo-redo` to record verification report.
