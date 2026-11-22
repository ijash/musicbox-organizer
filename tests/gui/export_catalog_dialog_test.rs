use egui_kittest::Harness;
use musicbox_organizer::models::{CatalogFormat, CatalogPagePreset, ProjectTrack};
use musicbox_organizer::ui::MusicBoxApp;

#[test]
fn test_unified_export_dialog_format_and_field_toggles() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track Alpha".to_string(),
        artist: "Artist Alpha".to_string(),
        file_path: "/music/alpha.mp3".to_string(),
    });

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

    // 1. Verify default format is PDF
    assert_eq!(
        harness.state().export_catalog_dialog.config.format,
        CatalogFormat::Pdf
    );

    // 2. Switch format to Plain Text
    harness.state_mut().export_catalog_dialog.config.format = CatalogFormat::Txt;
    harness.run();
    assert_eq!(
        harness.state().export_catalog_dialog.config.format,
        CatalogFormat::Txt
    );

    // 3. Switch format to Rich Text
    harness.state_mut().export_catalog_dialog.config.format = CatalogFormat::Rtf;
    harness.run();
    assert_eq!(
        harness.state().export_catalog_dialog.config.format,
        CatalogFormat::Rtf
    );

    // 4. Test field toggles
    assert!(
        harness
            .state()
            .export_catalog_dialog
            .config
            .fields
            .track_number
    );
    assert!(harness.state().export_catalog_dialog.config.fields.title);
    assert!(!harness.state().export_catalog_dialog.config.fields.artist);

    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .fields
        .artist = true;
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .fields
        .header_title = true;
    harness.run();

    assert!(harness.state().export_catalog_dialog.config.fields.artist);
    assert!(
        harness
            .state()
            .export_catalog_dialog
            .config
            .fields
            .header_title
    );

    // 5. Switch back to PDF and test preset changes
    harness.state_mut().export_catalog_dialog.config.format = CatalogFormat::Pdf;
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .preset = CatalogPagePreset::CdCover12x12;
    let (w, h) = CatalogPagePreset::CdCover12x12.dimensions_cm();
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .width = w;
    harness
        .state_mut()
        .export_catalog_dialog
        .config
        .dimensions
        .height = h;
    harness.run();

    assert_eq!(
        harness
            .state()
            .export_catalog_dialog
            .config
            .dimensions
            .preset,
        CatalogPagePreset::CdCover12x12
    );
    assert_eq!(
        harness
            .state()
            .export_catalog_dialog
            .config
            .dimensions
            .width,
        12.0
    );
    assert_eq!(
        harness
            .state()
            .export_catalog_dialog
            .config
            .dimensions
            .height,
        12.0
    );
}

#[test]
fn test_export_catalog_dialog_narrow_300x400_viewport() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track Narrow Viewport".to_string(),
        artist: "Artist Test".to_string(),
        file_path: "/music/narrow.mp3".to_string(),
    });

    app.export_catalog_dialog.open();

    let mut harness = Harness::builder()
        .with_size(egui::vec2(300.0, 400.0))
        .build_ui_state(
            |ui, state: &mut MusicBoxApp| {
                state.render_central_content(ui);
            },
            app,
        );

    // Run frame at 300x400 px narrow viewport
    harness.run();

    // Verify dialog remains open and active in narrow layout
    assert!(harness.state().export_catalog_dialog.is_open);

    // Close dialog
    harness.state_mut().export_catalog_dialog.close();
    harness.run();
    assert!(!harness.state().export_catalog_dialog.is_open);
}
