use egui_kittest::Harness;
use musicbox_organizer::models::{CatalogPagePreset, LengthUnit, ProjectTrack};
use musicbox_organizer::ui::MusicBoxApp;

#[test]
fn test_catalog_live_layout_summary_and_readability_warning() {
    let mut app = MusicBoxApp::default();
    for i in 0..15 {
        app.project.tracks.push(ProjectTrack {
            index: i,
            title: format!("Test Track {}", i + 1),
            artist: format!("Artist {}", i + 1),
            file_path: format!("/music/track_{}.mp3", i + 1),
        });
    }

    // Open Export Catalog dialog
    app.export_catalog_dialog.open();
    assert!(app.export_catalog_dialog.is_open);

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();

    // Default 10x10 cm card with 15 tracks
    let layout_default = harness
        .state()
        .export_catalog_dialog
        .compute_layout(harness.state().project.tracks.len());
    assert_eq!(layout_default.column_count, 1);
    assert!(layout_default.computed_font_size_pt >= 5.0);
    assert!(!layout_default.readability_warning);

    // 1. Verify 1.0 cm clamping: Attempt to set dimensions below 1.0 cm
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .width = 0.4;
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .height = 0.2;
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .clamp_dimensions();

    assert_eq!(
        harness
            .state()
            .export_catalog_dialog
            .config
            .dimensions
            .width,
        1.0
    );
    assert_eq!(
        harness
            .state()
            .export_catalog_dialog
            .config
            .dimensions
            .height,
        1.0
    );

    // 2. Compute layout on tiny 1.0 x 1.0 cm sheet with 15 tracks -> triggers < 5.0 pt warning
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .preset = CatalogPagePreset::Custom;
    harness.run();

    let layout_tiny = harness
        .state()
        .export_catalog_dialog
        .compute_layout(harness.state().project.tracks.len());
    assert!(layout_tiny.readability_warning);
    assert!(layout_tiny.computed_font_size_pt < 5.0);

    // 3. Close dialog
    harness.state_mut().export_catalog_dialog.close();
    harness.run();
    assert!(!harness.state().export_catalog_dialog.is_open);
}

#[test]
fn test_catalog_dimensions_unit_conversion_and_clamping() {
    let mut app = MusicBoxApp::default();
    app.export_catalog_dialog.open();

    let mut harness = Harness::new_ui_state(
        |ui, state| {
            state.render_central_content(ui);
        },
        app,
    );

    harness.run();

    // Switch to mm
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .set_unit(LengthUnit::Mm);
    harness.run();

    assert_eq!(
        harness.state().export_catalog_dialog.config.dimensions.unit,
        LengthUnit::Mm
    );
    // 10 cm should be approximately 100 mm
    let w_mm = harness
        .state()
        .export_catalog_dialog
        .config
        .dimensions
        .width;
    assert!((w_mm - 100.0).abs() < 0.5);

    // Switch to Inches
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .set_unit(LengthUnit::Inches);
    harness.run();

    assert_eq!(
        harness.state().export_catalog_dialog.config.dimensions.unit,
        LengthUnit::Inches
    );
    // 10 cm is ~3.937 inches
    let w_in = harness
        .state()
        .export_catalog_dialog
        .config
        .dimensions
        .width;
    assert!((w_in - 3.937).abs() < 0.05);
}
