use musicbox_organizer::models::{OrganizerTrack, SessionState, TrackMetadata};
use std::path::PathBuf;

fn create_dummy_track(idx: u32, title: &str) -> OrganizerTrack {
    let mut meta = TrackMetadata::default();
    meta.title = title.to_string();
    OrganizerTrack::new(idx, PathBuf::from(format!("song_{}.mp3", idx)), meta, "AAAA".into())
}

#[test]
fn test_undo_and_redo_basic_flow() {
    let mut state = SessionState::new(PathBuf::from("/test"));
    state.tracks.push(create_dummy_track(0, "Original"));

    // Modify
    state.push_undo_checkpoint();
    state.tracks[0].metadata.title = "Modified".to_string();

    assert_eq!(state.tracks[0].metadata.title, "Modified");

    // Undo
    assert!(state.undo());
    assert_eq!(state.tracks[0].metadata.title, "Original");

    // Redo
    assert!(state.redo());
    assert_eq!(state.tracks[0].metadata.title, "Modified");
}

#[test]
fn test_undo_stack_bounded_to_10() {
    let mut state = SessionState::new(PathBuf::from("/test"));
    state.tracks.push(create_dummy_track(0, "v0"));

    for i in 1..=15 {
        state.push_undo_checkpoint();
        state.tracks[0].metadata.title = format!("v{}", i);
    }

    // Must be bounded to max 10 entries
    assert_eq!(state.undo_stack.len(), 10);

    // Can only undo 10 times
    for _ in 0..10 {
        assert!(state.undo());
    }
    assert!(!state.undo()); // 11th undo fails
    assert_eq!(state.tracks[0].metadata.title, "v5");
}

#[test]
fn test_save_clears_undo_redo_history() {
    let mut state = SessionState::new(PathBuf::from("/test"));
    state.push_undo_checkpoint();
    state.tracks.push(create_dummy_track(0, "v1"));
    assert_eq!(state.undo_stack.len(), 1);

    state.mark_saved();
    assert!(state.undo_stack.is_empty());
    assert!(state.redo_stack.is_empty());
    assert!(!state.is_dirty);
}
