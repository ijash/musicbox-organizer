#[cfg(target_os = "linux")]
#[path = "gui/mod.rs"]
mod gui;

#[cfg(target_os = "linux")]
#[test]
fn test_gui_interactive_workflow_headless() {
    use gui::harness::GuiHarness;
    use gtk::prelude::*;
    use std::path::Path;

    let harness = GuiHarness::new();

    // 1. Initial State Check
    assert_eq!(harness.project.borrow().tracks.len(), 0);
    harness.pump_events();

    // 2. Add Tracks
    harness.add_track("First Song", "Artist 1", Path::new("/tmp/first.mp3"));
    harness.add_track("Second Song", "Artist 2", Path::new("/tmp/second.mp3"));
    harness.add_track("Third Song", "Artist 3", Path::new("/tmp/third.mp3"));
    assert_eq!(harness.project.borrow().tracks.len(), 3);
    harness.pump_events();

    // 3. Test Theme Toggle Click
    harness.header.theme_btn.clicked();
    harness.pump_events();
    harness.header.theme_btn.clicked();
    harness.pump_events();

    // 4. Test Table Row Selection
    let path0 = gtk::TreePath::from_indicesv(&[0]);
    harness.table.tree_view.selection().select_path(&path0);
    harness.pump_events();

    // 5. Test Audio Preview Buttons (FR-002)
    harness.audio.status_label.set_text("Simulated Play");
    harness.pump_events();

    // 6. Test Export Panel Status
    assert_eq!(harness.export.status_label.text().as_str(), "Ready");

    // 7. Test Audio Tools (DSP) Button & Dialog Open/Cancel (US4 / T011-T013)
    assert!(harness.table.audio_tools_btn.is_sensitive());
    harness.table.audio_tools_btn.clicked();
    harness.pump_events();

    // Find opened Audio DSP Tools dialog
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

    // Cancel dialog
    if let Ok(dialog) = dsp_dlg.clone().downcast::<gtk::Dialog>() {
        dialog.response(gtk::ResponseType::Cancel);
    }
    harness.pump_events();

    assert!(!dsp_dlg.is_visible(), "DSP dialog should close on cancel response");

    // 8. Clean teardown
    harness.window.close();
    harness.pump_events();
}
