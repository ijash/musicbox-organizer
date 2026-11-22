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

#[test]
fn test_app_new_startup_and_device_detection() {
    let app = MusicBoxApp::new(musicbox_organizer::models::ProjectFile::default(), None);
    // Device manager was queried automatically on initialization
    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );
    harness.run();
    assert!(harness.state().dark_mode);
}

#[test]
fn test_toolbar_track_removal_and_move_sync_playing_index() {
    use egui_kittest::kittest::Queryable;

    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track 0".to_string(),
        artist: "Artist 0".to_string(),
        file_path: "/music/track0.mp3".to_string(),
    });
    app.project.tracks.push(ProjectTrack {
        index: 1,
        title: "Track 1".to_string(),
        artist: "Artist 1".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });
    app.project.tracks.push(ProjectTrack {
        index: 2,
        title: "Track 2".to_string(),
        artist: "Artist 2".to_string(),
        file_path: "/music/track2.mp3".to_string(),
    });

    // Track 1 is currently playing
    app.playing_track_idx = Some(1);

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );
    harness.run();

    // 1. Remove track 0 (above playing track) -> playing track moves from 1 to 0
    harness.state_mut().selected_track_indices.insert(0);
    harness.run();
    harness.get_by_label("🗑 Remove").click();
    harness.run();

    assert_eq!(harness.state().project.tracks.len(), 2);
    assert_eq!(harness.state().project.tracks[0].title, "Track 1");
    assert_eq!(harness.state().playing_track_idx, Some(0));

    // 2. Select track 0 (playing track) and move down -> moves to index 1, playing_track_idx follows to 1
    harness.state_mut().selected_track_indices.clear();
    harness.state_mut().selected_track_indices.insert(0);
    harness.run();
    harness.get_by_label("⬇ Down").click();
    harness.run();

    assert_eq!(harness.state().project.tracks[1].title, "Track 1");
    assert_eq!(harness.state().playing_track_idx, Some(1));

    // 3. Select track 1 (playing track) and move up -> moves back to index 0, playing_track_idx follows to 0
    harness.get_by_label("⬆ Up").click();
    harness.run();

    assert_eq!(harness.state().project.tracks[0].title, "Track 1");
    assert_eq!(harness.state().playing_track_idx, Some(0));

    // 4. Select track 0 (playing track) and remove -> playback stopped, playing_track_idx cleared
    harness.get_by_label("🗑 Remove").click();
    harness.run();

    assert_eq!(harness.state().project.tracks.len(), 1);
    assert_eq!(harness.state().project.tracks[0].title, "Track 2");
    assert_eq!(harness.state().playing_track_idx, None);
}

#[test]
fn test_playlist_full_row_selection_and_inline_edit() {
    use egui_kittest::kittest::Queryable;
    use musicbox_organizer::ui::TableColumn;

    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Initial Title".to_string(),
        artist: "Initial Artist".to_string(),
        file_path: "/music/track0.mp3".to_string(),
    });
    app.project.tracks.push(ProjectTrack {
        index: 1,
        title: "Second Title".to_string(),
        artist: "Second Artist".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );
    harness.run();

    // 1. Single click on Title enters edit mode immediately and selects row
    let title_cell = harness.get_by_label("Initial Title");
    title_cell.click();
    harness.run();

    assert!(harness.state().selected_track_indices.contains(&0));
    assert_eq!(harness.state().editing_cell, Some((0, TableColumn::Title)));
    assert_eq!(harness.state().edit_buffer, "Initial Title");

    // 2. Click outside (on row 1 '#' cell) commits edit and selects row 1
    harness.state_mut().edit_buffer = "Edited Title".to_string();
    let row1_num = harness.get_by_label("02");
    row1_num.click();
    harness.run();

    assert_eq!(harness.state().editing_cell, None);
    assert_eq!(harness.state().project.tracks[0].title, "Edited Title");
    assert!(harness.state().selected_track_indices.contains(&1));

    // 3. Single click on Artist of row 1 enters artist edit mode
    let artist_cell = harness.get_by_label("Second Artist");
    artist_cell.click();
    harness.run();

    assert_eq!(harness.state().editing_cell, Some((1, TableColumn::Artist)));
    assert_eq!(harness.state().edit_buffer, "Second Artist");
}
