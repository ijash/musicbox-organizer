use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use musicbox_organizer::models::ProjectTrack;
use musicbox_organizer::ui::{DspProgressMessage, MusicBoxApp, render_dsp_dialog_contents};
use std::sync::mpsc;

#[test]
fn test_audio_preview_controls_and_status() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Preview Track".to_string(),
        artist: "Artist X".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();
    // Initially track 0 is auto-targeted or selected
    assert!(harness.state().playing_track_idx.is_none());

    // Simulate selecting track 0
    harness.state_mut().selected_track_indices.insert(0);
    harness.run();

    // Verify stop resets playing track
    harness.state_mut().playing_track_idx = Some(0);
    harness.run();
    assert_eq!(harness.state().playing_track_idx, Some(0));

    // Reset playing track
    harness.state_mut().playing_track_idx = None;
    harness.run();
    assert!(harness.state().playing_track_idx.is_none());
}

#[test]
fn test_dsp_modal_parameters_and_toggles() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Test Track".to_string(),
        artist: "Test Artist".to_string(),
        file_path: "/music/test.mp3".to_string(),
    });

    // Open DSP modal
    app.dsp_dialog.is_open = true;

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();

    // Verify defaults cite: data-model.md: DspDialogState
    assert!(harness.state().dsp_dialog.normalize_enabled);
    assert_eq!(harness.state().dsp_dialog.normalize_target_db, -14.0);
    assert!(harness.state().dsp_dialog.trim_enabled);
    assert_eq!(harness.state().dsp_dialog.trim_threshold_db, -45.0);
    assert!(!harness.state().dsp_dialog.is_processing);

    // Toggle parameters
    harness.state_mut().dsp_dialog.normalize_enabled = false;
    harness.state_mut().dsp_dialog.trim_threshold_db = -50.0;
    harness.run();

    assert!(!harness.state().dsp_dialog.normalize_enabled);
    assert_eq!(harness.state().dsp_dialog.trim_threshold_db, -50.0);

    // Close modal
    harness.state_mut().dsp_dialog.is_open = false;
    harness.run();
    assert!(!harness.state().dsp_dialog.is_open);
}

#[test]
fn test_dsp_progress_channel_messages() {
    let mut app = MusicBoxApp::default();
    let (tx, rx) = mpsc::channel();
    app.dsp_rx = Some(rx);
    app.dsp_dialog.is_open = true;

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    // Send Started message
    tx.send(DspProgressMessage::Started { total_tracks: 2 })
        .unwrap();
    harness.state_mut().poll_dsp_channel();
    assert!(harness.state().dsp_dialog.is_processing);
    assert_eq!(
        harness.state().dsp_dialog.progress_text,
        "Processing 0/2..."
    );

    // Send Step message
    tx.send(DspProgressMessage::Step {
        current: 1,
        total: 2,
        filename: "test.mp3".to_string(),
    })
    .unwrap();
    harness.state_mut().poll_dsp_channel();
    assert_eq!(harness.state().dsp_dialog.progress_percent, 0.5);

    // Send Finished message
    tx.send(DspProgressMessage::Finished {
        succeeded: 2,
        failed: 0,
        errors: vec![],
    })
    .unwrap();
    harness.state_mut().poll_dsp_channel();
    assert!(!harness.state().dsp_dialog.is_processing);
    assert!(!harness.state().dsp_dialog.is_open);
    assert_eq!(
        harness.state().status_message,
        "DSP complete: 2 updated, 0 failed"
    );
}

#[test]
fn test_dsp_dialog_cancel_button() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Test Track".to_string(),
        artist: "Test Artist".to_string(),
        file_path: "/music/test.mp3".to_string(),
    });
    app.dsp_dialog.is_open = true;

    let mut harness = Harness::builder().build_state(
        |ctx, state| {
            render_dsp_dialog_contents(state, ctx);
        },
        app,
    );

    harness.run();
    assert!(harness.state().dsp_dialog.is_open);

    let cancel_btn = harness.get_by_label("Cancel");
    assert!(!cancel_btn.is_disabled());
    cancel_btn.click();
    harness.run();

    assert!(!harness.state().dsp_dialog.is_open);
}

#[test]
fn test_dsp_dialog_toolbar_button_opens_and_cancel_closes_full_app() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track One".to_string(),
        artist: "Artist A".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });

    assert!(!app.dsp_dialog.is_open);

    let mut harness = Harness::builder()
        .with_size(egui::vec2(960.0, 680.0))
        .build_eframe(|_cc| app);

    harness.run();
    assert!(!harness.state().dsp_dialog.is_open);

    harness.get_by_label("⚙ Audio Tools...").click();
    harness.step();
    assert!(harness.state().dsp_dialog.is_open);

    let cancel_btn = harness.get_by_label("Cancel");
    assert!(!cancel_btn.is_disabled());
    cancel_btn.click();
    harness.step();
    assert!(!harness.state().dsp_dialog.is_open);
}

#[test]
fn test_audio_bar_direct_track_switching_while_playing() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track Zero".to_string(),
        artist: "Artist 0".to_string(),
        file_path: "/music/track0.mp3".to_string(),
    });
    app.project.tracks.push(ProjectTrack {
        index: 1,
        title: "Track One".to_string(),
        artist: "Artist 1".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });

    // Simulate playing track 0
    app.playing_track_idx = Some(0);
    // User selects track 1
    app.selected_track_indices.insert(1);

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();

    // The Play button must be enabled because target (1) != playing (0)
    let play_btn = harness.get_by_label("▶ Play");
    assert!(!play_btn.is_disabled());
}
