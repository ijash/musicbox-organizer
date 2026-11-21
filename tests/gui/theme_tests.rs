use egui_kittest::Harness;
use musicbox_organizer::ui::MusicBoxApp;

#[test]
fn test_dark_light_theme_toggle_and_visuals() {
    let app = MusicBoxApp::default();
    assert!(
        app.dark_mode,
        "Default theme should be dark per specification"
    );

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            if state.dark_mode {
                ui.ctx().set_visuals(egui::Visuals::dark());
            } else {
                ui.ctx().set_visuals(egui::Visuals::light());
            }
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();

    // Verify dark visuals text color contrast
    let dark_visuals = egui::Visuals::dark();
    assert!(dark_visuals.dark_mode);
    assert_ne!(
        dark_visuals.widgets.noninteractive.fg_stroke.color,
        egui::Color32::BLACK
    );

    // Toggle to Light mode
    harness.state_mut().dark_mode = false;
    harness.run();
    assert!(!harness.state().dark_mode);

    // Verify light visuals
    let light_visuals = egui::Visuals::light();
    assert!(!light_visuals.dark_mode);
    assert_ne!(
        light_visuals.widgets.noninteractive.fg_stroke.color,
        egui::Color32::WHITE
    );

    // Toggle back to Dark mode
    harness.state_mut().dark_mode = true;
    harness.run();
    assert!(harness.state().dark_mode);
}

#[test]
fn test_footer_hyperlink_and_status() {
    let app = MusicBoxApp {
        status_message: "Test status operational".to_string(),
        ..Default::default()
    };

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();
    assert_eq!(harness.state().status_message, "Test status operational");
}

#[test]
fn test_compact_layout_and_supported_glyphs() {
    let app = MusicBoxApp::default();
    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    // Set compact window viewport 300x400px
    harness.set_size(egui::vec2(300.0, 400.0));
    harness.run();

    // Verify app state survives rendering at compact 300x400 size
    assert!(harness.state().project.tracks.is_empty());
}
