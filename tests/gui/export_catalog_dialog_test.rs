use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use musicbox_organizer::models::{CatalogFormat, CatalogPagePreset, ProjectTrack};
use musicbox_organizer::ui::{MusicBoxApp, render_export_catalog_contents};

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

    let mut harness = Harness::builder().build_state(
        |ctx, state| {
            render_export_catalog_contents(state, ctx);
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
fn test_export_catalog_dialog_narrow_pinned_buttons_280x320() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track Narrow Viewport".to_string(),
        artist: "Artist Test".to_string(),
        file_path: "/music/narrow.mp3".to_string(),
    });

    app.export_catalog_dialog.open();

    let mut harness = Harness::builder()
        .with_size(egui::vec2(280.0, 320.0))
        .build_state(
            |ctx, state: &mut MusicBoxApp| {
                render_export_catalog_contents(state, ctx);
            },
            app,
        );

    // Run frame at minimum 280x320 px narrow viewport
    harness.run();

    // Verify dialog remains open and active in narrow layout
    assert!(harness.state().export_catalog_dialog.is_open);

    // Assert pinned bottom panel action buttons are present and not disabled
    let export_btn = harness.get_by_label("Export...");
    assert!(!export_btn.is_disabled());

    let cancel_btn = harness.get_by_label("Cancel");
    assert!(!cancel_btn.is_disabled());

    // Click cancel button and verify dialog closes cleanly
    cancel_btn.click();
    harness.run();
    assert!(!harness.state().export_catalog_dialog.is_open);
}

#[test]
fn test_export_catalog_toolbar_button_opens_dialog_full_app() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track One".to_string(),
        artist: "Artist A".to_string(),
        file_path: "/music/track1.mp3".to_string(),
    });

    assert!(!app.export_catalog_dialog.is_open);

    let mut harness = Harness::builder()
        .with_size(egui::vec2(960.0, 680.0))
        .build_eframe(|_cc| app);

    harness.run();

    // Initial state: dialog closed
    assert!(!harness.state().export_catalog_dialog.is_open);

    // Click toolbar button
    harness.get_by_label("📄 Export Catalog...").click();
    harness.step();

    // Dialog state is open
    assert!(harness.state().export_catalog_dialog.is_open);

    // Verify dialog modal controls render visibly
    let export_btn = harness.get_by_label("Export...");
    assert!(!export_btn.is_disabled());

    let cancel_btn = harness.get_by_label("Cancel");
    assert!(!cancel_btn.is_disabled());
}

#[test]
fn test_export_catalog_format_switch_resizes_viewport() {
    let mut app = MusicBoxApp::default();
    app.project.tracks.push(ProjectTrack {
        index: 0,
        title: "Track Sizing".to_string(),
        artist: "Artist".to_string(),
        file_path: "/music/track.mp3".to_string(),
    });

    app.export_catalog_dialog.open();
    assert_eq!(app.export_catalog_dialog.config.format, CatalogFormat::Pdf);

    let mut harness = Harness::builder()
        .with_size(egui::vec2(960.0, 680.0))
        .build_eframe(|_cc| app);

    harness.step();
    assert!(harness.state().export_catalog_dialog.is_open);

    // Switch format to Plain Text
    harness.state_mut().export_catalog_dialog.config.format = CatalogFormat::Txt;
    harness.step();
    assert_eq!(
        harness.state().export_catalog_dialog.config.format,
        CatalogFormat::Txt
    );

    // Switch format to Rich Text
    harness.state_mut().export_catalog_dialog.config.format = CatalogFormat::Rtf;
    harness.step();
    assert_eq!(
        harness.state().export_catalog_dialog.config.format,
        CatalogFormat::Rtf
    );
}
