#[cfg(target_os = "linux")]
#[path = "gui/mod.rs"]
mod gui;

#[cfg(target_os = "linux")]
#[test]
fn test_gui_interactive_workflow_headless() {
    use gui::harness::GuiHarness;
    use gtk::prelude::*;
    use id3::TagLike;
    use musicbox_organizer::models::ProjectFile;
    use musicbox_organizer::services::ExportConfigService;
    use musicbox_organizer::ui::icons::AppIcon;
    use rodio::Source;
    use std::cell::RefCell;
    use std::fs;
    use std::io::BufReader;
    use std::path::Path;
    use std::rc::Rc;
    use tempfile::tempdir;

    let harness = GuiHarness::new();

    // ==========================================
    // Phase A: F005–F007 Branding, Themes & Icons
    // ==========================================
    // 1. Verify all 12 AppIcon variants render to GtkImage without error
    let all_icons = [
        AppIcon::FolderOpen,
        AppIcon::ListAdd,
        AppIcon::ListRemove,
        AppIcon::GoUp,
        AppIcon::GoDown,
        AppIcon::Play,
        AppIcon::Pause,
        AppIcon::Stop,
        AppIcon::Save,
        AppIcon::Open,
        AppIcon::ExportText,
        AppIcon::AudioTools,
    ];
    for icon in all_icons {
        assert!(icon.image(14).is_some(), "Icon {:?} should render successfully", icon);
        assert!(icon.image_themed(14, true).is_some(), "Icon {:?} dark mode should render", icon);
        assert!(icon.image_themed(14, false).is_some(), "Icon {:?} light mode should render", icon);
        assert!(!icon.svg_data().is_empty());
    }

    // Verify update_button_icon works
    let test_btn = AppIcon::Play.button("Play");
    AppIcon::update_button_icon(&test_btn, AppIcon::Play, true);
    AppIcon::update_button_icon(&test_btn, AppIcon::Play, false);

    // 2. Toggle Theme (Dark <-> Light)
    harness.header.theme_btn.clicked();
    harness.pump_events();
    harness.header.theme_btn.clicked();
    harness.pump_events();

    // 3. Verify Initial State
    assert_eq!(harness.project.borrow().tracks.len(), 0);
    assert_eq!(harness.export.status_label.text().as_str(), "Ready");
    harness.pump_events();

    // ==========================================
    // Phase B: F001 Organizer, ID3 & Undo/Redo
    // ==========================================
    let temp_dir = tempdir().expect("create temp dir for GUI test");
    let sample1_src = Path::new("mp3-sample/sample-3s.mp3");
    let sample2_src = Path::new("mp3-sample/sample-5s.mp3");

    let track1_path = temp_dir.path().join("track_alpha.mp3");
    let track2_path = temp_dir.path().join("track_beta.mp3");
    fs::copy(sample1_src, &track1_path).expect("copy sample 1");
    fs::copy(sample2_src, &track2_path).expect("copy sample 2");

    // Write initial ID3 tags
    let mut tag1 = id3::Tag::new();
    tag1.set_title("Track Alpha Title");
    tag1.set_artist("Track Alpha Artist");
    tag1.write_to_path(&track1_path, id3::Version::Id3v24).expect("write tag 1");

    let mut tag2 = id3::Tag::new();
    tag2.set_title("Track Beta Title");
    tag2.set_artist("Track Beta Artist");
    tag2.write_to_path(&track2_path, id3::Version::Id3v24).expect("write tag 2");

    // Add tracks to GUI
    harness.add_track("Track Alpha Title", "Track Alpha Artist", &track1_path);
    harness.add_track("Track Beta Title", "Track Beta Artist", &track2_path);
    assert_eq!(harness.project.borrow().tracks.len(), 2);
    assert_eq!(harness.session_state.borrow().tracks.len(), 2);
    assert_eq!(harness.table.list_store.iter_n_children(None), 2);
    harness.pump_events();

    // Test selection and Reorder Up / Down
    let path1 = gtk::TreePath::from_indicesv(&[1]);
    harness.table.tree_view.selection().select_path(&path1);
    harness.pump_events();

    // Move Up
    harness.table.move_up_btn.clicked();
    harness.pump_events();
    assert_eq!(harness.project.borrow().tracks[0].title, "Track Beta Title");
    assert_eq!(harness.project.borrow().tracks[1].title, "Track Alpha Title");

    // Move Down
    let path0 = gtk::TreePath::from_indicesv(&[0]);
    harness.table.tree_view.selection().select_path(&path0);
    harness.pump_events();
    harness.table.move_down_btn.clicked();
    harness.pump_events();
    assert_eq!(harness.project.borrow().tracks[0].title, "Track Alpha Title");
    assert_eq!(harness.project.borrow().tracks[1].title, "Track Beta Title");

    // Remove track and test Undo / Redo
    let path1 = gtk::TreePath::from_indicesv(&[1]);
    harness.table.tree_view.selection().select_path(&path1);
    harness.pump_events();

    harness.table.remove_track_btn.clicked();
    harness.pump_events();
    assert_eq!(harness.project.borrow().tracks.len(), 1);

    // Undo removal
    harness.header.undo_btn.clicked();
    harness.pump_events();
    assert_eq!(harness.project.borrow().tracks.len(), 2);
    assert_eq!(harness.project.borrow().tracks[1].title, "Track Beta Title");

    // Redo removal
    harness.header.redo_btn.clicked();
    harness.pump_events();
    assert_eq!(harness.project.borrow().tracks.len(), 1);

    // Undo again to restore 2 tracks
    harness.header.undo_btn.clicked();
    harness.pump_events();
    assert_eq!(harness.project.borrow().tracks.len(), 2);

    // Test undo stack depth limit (MAX_UNDO_DEPTH = 10)
    for _ in 0..15 {
        harness.session_state.borrow_mut().push_undo_checkpoint();
    }
    assert!(harness.session_state.borrow().undo_stack.len() <= 10);

    // ==========================================
    // Phase C: F002 Save/Open & Multi-Format Export
    // ==========================================
    let project_file_path = temp_dir.path().join("saved_playlist.txt");
    harness.project.borrow().save(&project_file_path).expect("save project file");
    assert!(project_file_path.exists());

    let loaded_proj = ProjectFile::load(&project_file_path).expect("load saved project");
    assert_eq!(loaded_proj.tracks.len(), 2);
    assert_eq!(loaded_proj.tracks[0].title, "Track Alpha Title");

    // Multi-format export (.txt & .rtf)
    let export_service = ExportConfigService::new();
    let selection = musicbox_organizer::models::ExportFormatSelection {
        txt: true,
        rtf: true,
    };
    let exported = export_service
        .export_playlist(temp_dir.path(), &selection, &harness.project.borrow())
        .expect("export formats");

    assert_eq!(exported.len(), 2);
    for out_file in &exported {
        assert!(out_file.exists());
        let meta = fs::metadata(out_file).expect("file metadata");
        assert!(meta.len() > 0, "Exported file {:?} should not be empty", out_file);
    }

    // ==========================================
    // Phase D: F007 AudioBar Playback Preview
    // ==========================================
    let path0 = gtk::TreePath::from_indicesv(&[0]);
    harness.table.tree_view.selection().select_path(&path0);
    harness.pump_events();

    harness.audio.play_btn.clicked();
    harness.pump_events();

    harness.audio.pause_btn.clicked();
    harness.pump_events();
    assert!(harness.audio.status_label.text().contains("Paused"));

    harness.audio.stop_btn.clicked();
    harness.pump_events();
    assert!(harness.audio.status_label.text().contains("Stopped"));

    // ==========================================
    // Phase E: F008 Audio Tools Dialog Open/Cancel & Live DSP Run
    // ==========================================
    // 1. Test DSP Dialog Open & Cancel
    assert!(harness.table.audio_tools_btn.is_sensitive());
    harness.table.audio_tools_btn.clicked();
    harness.pump_events();

    let toplevels = gtk::Window::list_toplevels();
    let dsp_dialog = toplevels.into_iter().find(|w| {
        w.downcast_ref::<gtk::Window>()
            .and_then(|win| win.title())
            .map(|t| t == "Audio DSP Tools")
            .unwrap_or(false)
    });
    assert!(dsp_dialog.is_some(), "Audio DSP Tools dialog should open");

    let dsp_dlg = dsp_dialog.unwrap();
    assert!(dsp_dlg.is_visible(), "DSP dialog should be visible");

    if let Ok(dialog) = dsp_dlg.clone().downcast::<gtk::Dialog>() {
        dialog.response(gtk::ResponseType::Cancel);
    }
    harness.pump_events();
    assert!(!dsp_dlg.is_visible(), "DSP dialog should close on cancel response");

    // 2. Start audio playback on track 0 to test auto-stop guard
    harness.audio_player.borrow_mut().play_file(&track1_path).ok();
    assert!(harness.audio_player.borrow().current_file.is_some());

    // 3. Multi-select and run full DSP pipeline (Trim + Normalize)
    let done_flag = Rc::new(RefCell::new(false));
    let done_clone = Rc::clone(&done_flag);
    let run_dialog = musicbox_organizer::ui::components::show_dsp_dialog(
        &harness.window,
        vec![track1_path.clone(), track2_path.clone()],
        &harness.audio_player,
        &harness.export.status_label,
        move || {
            *done_clone.borrow_mut() = true;
        },
    );
    harness.pump_events();

    // Trigger Apply
    run_dialog.response(gtk::ResponseType::Apply);
    harness.pump_events();

    // Assert player was stopped immediately before DSP processing
    assert!(
        harness.audio_player.borrow().current_file.is_none(),
        "Audio preview player must be stopped when active file is in target batch"
    );

    // Pump GTK events until background worker completes via GLib channel (timeout 10s)
    let start_time = std::time::Instant::now();
    while !*done_flag.borrow() && start_time.elapsed().as_secs() < 10 {
        harness.pump_events();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    assert!(*done_flag.borrow(), "DSP processing should finish within 10s");
    harness.pump_events();

    // Assert status reported batch outcome
    let status_text = harness.export.status_label.text();
    assert!(
        status_text.contains("DSP Complete"),
        "Status should reflect DSP completion, got: {}",
        status_text
    );

    // 4. Verify resulting files are playable decodable MP3s
    let f1 = fs::File::open(&track1_path).expect("open track 1");
    let dec1 = rodio::Decoder::new(BufReader::new(f1)).expect("decode track 1");
    assert!(dec1.sample_rate() > 0);
    assert!(dec1.channels() > 0);

    let f2 = fs::File::open(&track2_path).expect("open track 2");
    let dec2 = rodio::Decoder::new(BufReader::new(f2)).expect("decode track 2");
    assert!(dec2.sample_rate() > 0);
    assert!(dec2.channels() > 0);

    // 5. Verify ID3 tags preserved verbatim
    let read_tag1 = id3::Tag::read_from_path(&track1_path).expect("read tag 1");
    assert_eq!(read_tag1.title(), Some("Track Alpha Title"));
    assert_eq!(read_tag1.artist(), Some("Track Alpha Artist"));

    let read_tag2 = id3::Tag::read_from_path(&track2_path).expect("read tag 2");
    assert_eq!(read_tag2.title(), Some("Track Beta Title"));
    assert_eq!(read_tag2.artist(), Some("Track Beta Artist"));

    // 6. Verify zero stray temporary files
    let remaining_entries: Vec<_> = fs::read_dir(temp_dir.path())
        .expect("read temp dir")
        .filter_map(|e| e.ok())
        .collect();
    for entry in remaining_entries {
        let name = entry.file_name().to_string_lossy().to_string();
        assert!(
            !name.starts_with(".tmp") && !name.ends_with(".tmp"),
            "Zero orphaned temp files must remain, found: {}",
            name
        );
    }

    // ==========================================
    // Clean teardown
    // ==========================================
    harness.window.close();
    harness.pump_events();
}
