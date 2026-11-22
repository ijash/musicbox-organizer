use egui_kittest::Harness;
use musicbox_organizer::models::{Device, FileSystem, ProjectTrack};
use musicbox_organizer::ui::{ExportProgressMessage, MusicBoxApp};
use std::path::PathBuf;
use std::sync::mpsc;

#[test]
fn test_export_panel_devices_and_subpath() {
    let mut app = MusicBoxApp::default();
    let dev = Device::new(
        "/dev/sdb1".to_string(),
        "USB DISK".to_string(),
        16_000_000_000,
        8_000_000_000,
        FileSystem::FAT32,
        Some(PathBuf::from("/media/usb")),
        true,
        false,
    )
    .unwrap();

    app.export_panel.available_devices.push(dev);
    app.export_panel.selected_device_id = Some("/dev/sdb1".to_string());
    app.export_panel.subpath = "my_music".to_string();

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();

    assert_eq!(harness.state().export_panel.available_devices.len(), 1);
    assert_eq!(
        harness.state().export_panel.selected_device_id.as_deref(),
        Some("/dev/sdb1")
    );
    assert_eq!(harness.state().export_panel.subpath, "my_music");
}

#[test]
fn test_export_panel_reformat_two_step_guard() {
    use egui_kittest::kittest::Queryable;
    use musicbox_organizer::ui::render_export_confirm_contents;

    let mut app = MusicBoxApp::default();
    let dev = Device::new(
        "/dev/sdb1".to_string(),
        "USB DRIVE".to_string(),
        8_000_000_000,
        4_000_000_000,
        FileSystem::FAT32,
        Some(PathBuf::from("/media/usb")),
        true,
        false,
    )
    .unwrap();

    app.export_panel.available_devices.push(dev);
    app.export_panel.selected_device_id = Some("/dev/sdb1".to_string());
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track 1".to_string(),
        artist: "Artist 1".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });

    // 1. Normal Export (no reformat): clicking Export Playlist opens confirmation dialog
    assert!(!app.export_panel.show_export_confirm);

    let mut harness = Harness::builder().build_state(
        |ctx, state: &mut MusicBoxApp| {
            render_export_confirm_contents(state, ctx);
        },
        app,
    );

    harness.run();

    // Verify "Confirm Export" button is present and clickable
    let confirm_btn = harness.get_by_label("Confirm Export");
    assert!(!confirm_btn.is_disabled());

    let cancel_btn = harness.get_by_label("Cancel");
    assert!(!cancel_btn.is_disabled());

    // 2. Destructive Reformat Gate: requires checkbox acknowledgment
    harness.state_mut().export_panel.reformat_enabled = true;
    harness.state_mut().export_panel.reformat_ack_checkbox = false;
    harness.run();

    let erase_btn = harness.get_by_label("Erase, Format & Export");
    assert!(
        erase_btn.is_disabled(),
        "Erase button must be disabled before checkbox is checked"
    );

    // Check acknowledgment checkbox
    harness.state_mut().export_panel.reformat_ack_checkbox = true;
    harness.run();

    let erase_btn_active = harness.get_by_label("Erase, Format & Export");
    assert!(
        !erase_btn_active.is_disabled(),
        "Erase button must be enabled once acknowledgment checkbox is checked"
    );
}

#[test]
fn test_export_playlist_click_opens_confirmation_full_app() {
    use egui_kittest::kittest::Queryable;

    let mut app = MusicBoxApp::default();
    let dev = Device::new(
        "/dev/sdb1".to_string(),
        "USB FLASH".to_string(),
        8_000_000_000,
        4_000_000_000,
        FileSystem::FAT32,
        Some(PathBuf::from("/media/usb")),
        true,
        false,
    )
    .unwrap();

    app.export_panel.available_devices.push(dev);
    app.export_panel.selected_device_id = Some("/dev/sdb1".to_string());
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track Alpha".to_string(),
        artist: "Artist Alpha".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });

    assert!(!app.export_panel.show_export_confirm);

    let mut harness = Harness::builder()
        .with_size(egui::vec2(960.0, 680.0))
        .build_eframe(|_cc| app);

    harness.run();

    // Click "💾 Export Playlist" button in export panel
    let export_btn = harness.get_by_label("💾 Export Playlist");
    assert!(!export_btn.is_disabled());
    export_btn.click();
    harness.run();

    // Confirmation dialog state must be open
    assert!(harness.state().export_panel.show_export_confirm);

    // Confirmation buttons are queryable and no export has completed yet
    let confirm_btn = harness.get_by_label("Confirm Export");
    assert!(!confirm_btn.is_disabled());
    assert!(!harness.state().export_panel.is_exporting);

    // Click Cancel
    harness.get_by_label("Cancel").click();
    harness.run();
    assert!(!harness.state().export_panel.show_export_confirm);
    assert!(!harness.state().export_panel.is_exporting);
}

#[test]
fn test_export_progress_channel_messages() {
    let mut app = MusicBoxApp::default();
    let (tx, rx) = mpsc::channel();
    app.export_rx = Some(rx);

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    // Started message
    tx.send(ExportProgressMessage::Started { total_tracks: 10 })
        .unwrap();
    harness.state_mut().poll_export_channel();
    assert!(harness.state().export_panel.is_exporting);
    assert_eq!(
        harness.state().export_panel.export_status,
        "Exporting 0/10..."
    );

    // Step message
    tx.send(ExportProgressMessage::Step {
        current: 5,
        total: 10,
        filename: "0001-track.mp3".to_string(),
    })
    .unwrap();
    harness.state_mut().poll_export_channel();
    assert_eq!(harness.state().export_panel.export_progress, 0.5);

    // Finished message
    tx.send(ExportProgressMessage::Finished {
        total_bytes: 10485760,
    })
    .unwrap();
    harness.state_mut().poll_export_channel();
    assert!(!harness.state().export_panel.is_exporting);
    assert!(
        harness
            .state()
            .export_panel
            .export_status
            .contains("10.0 MB")
    );
}
