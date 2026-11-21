use egui_kittest::Harness;
use musicbox_organizer::models::ProjectTrack;
use musicbox_organizer::ui::{DspProgressMessage, MusicBoxApp};
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
