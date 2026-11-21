use egui_kittest::Harness;
use musicbox_organizer::models::ProjectTrack;
use musicbox_organizer::ui::MusicBoxApp;

#[test]
fn test_playlist_empty_state_and_table_render() {
    let app = MusicBoxApp::default();
    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();
    assert_eq!(harness.state().project.tracks.len(), 0);
}

#[test]
fn test_playlist_add_reorder_and_undo() {
    let mut app = MusicBoxApp::default();

    // Add sample tracks
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track One".to_string(),
        artist: "Artist A".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });
    app.project.tracks.push(ProjectTrack {
        index: 1,
        title: "Track Two".to_string(),
        artist: "Artist B".to_string(),
        file_path: "/music/track2.mp3".to_string(),
    });

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();
    assert_eq!(harness.state().project.tracks.len(), 2);
    assert_eq!(harness.state().project.tracks[0].title, "Track One");

    // Select first track and move down
    harness.state_mut().selected_track_indices.insert(0);
    let current_proj = harness.state().project.clone();
    harness.state_mut().session.push_undo(current_proj);
    harness.state_mut().project.tracks.swap(0, 1);
    harness.state_mut().project.tracks[0].index = 0;
    harness.state_mut().project.tracks[1].index = 1;

    harness.run();
    assert_eq!(harness.state().project.tracks[0].title, "Track Two");
    assert_eq!(harness.state().project.tracks[1].title, "Track One");

    // Undo operation
    let current = harness.state().project.clone();
    if let Some(prev) = harness.state_mut().session.undo_project(current) {
        harness.state_mut().project = prev;
    }
    harness.run();
    assert_eq!(harness.state().project.tracks[0].title, "Track One");
    assert_eq!(harness.state().project.tracks[1].title, "Track Two");
}
