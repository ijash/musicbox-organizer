use musicbox_organizer::models::{
    CatalogExportConfig, CatalogFieldSelection, CatalogPagePreset, LengthUnit, OrganizerTrack,
    PhysicalDimensions, TrackMetadata,
};
use musicbox_organizer::services::{CatalogThemeRegistry, PdfCatalogExporter};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_pdf_export_atomic_and_header_validity() {
    let dir = tempdir().expect("tempdir");
    let pdf_path = dir.path().join("catalog.pdf");

    let tracks = vec![
        OrganizerTrack {
            index: 1,
            file_path: dir.path().join("0101-track1.mp3"),
            original_filename: "track1.mp3".to_string(),
            xxyy_prefix: "0101".to_string(),
            metadata: TrackMetadata {
                track_number: Some(1),
                title: "Song One".to_string(),
                artist: "Artist A".to_string(),
                album: "Album X".to_string(),
                year: Some(2025),
                duration_secs: 180,
            },
            is_modified: false,
        },
        OrganizerTrack {
            index: 2,
            file_path: dir.path().join("0102-track2.mp3"),
            original_filename: "track2.mp3".to_string(),
            xxyy_prefix: "0102".to_string(),
            metadata: TrackMetadata {
                track_number: Some(2),
                title: "Song Two".to_string(),
                artist: "Artist B".to_string(),
                album: "Album X".to_string(),
                year: Some(2025),
                duration_secs: 200,
            },
            is_modified: false,
        },
    ];

    let config = CatalogExportConfig {
        dimensions: PhysicalDimensions {
            unit: LengthUnit::Cm,
            width: 10.0,
            height: 10.0,
            margin: 0.5,
            preset: CatalogPagePreset::Card10x10,
        },
        cut_border: true,
        fields: CatalogFieldSelection {
            track_number: true,
            title: true,
            artist: true,
            export_filename: true,
            header_title: true,
        },
        ..Default::default()
    };

    let theme = CatalogThemeRegistry::get_theme("Warm Paper");
    let exporter = PdfCatalogExporter::new();

    exporter
        .export_pdf(&pdf_path, &tracks, Some("My Playlist"), &config, &theme)
        .expect("pdf export should succeed");

    assert!(pdf_path.exists());
    let bytes = fs::read(&pdf_path).expect("read pdf bytes");
    assert!(bytes.len() > 100);

    // Verify PDF Magic Header %PDF-
    assert_eq!(&bytes[0..5], b"%PDF-");

    // Verify zero temporary files left in directory
    let entries: Vec<_> = fs::read_dir(dir.path())
        .expect("read_dir")
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].file_name(), "catalog.pdf");
}
