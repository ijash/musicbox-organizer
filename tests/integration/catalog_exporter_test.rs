use musicbox_organizer::models::{
    CatalogExportConfig, CatalogFieldSelection, CatalogFormat, OrganizerTrack, TrackMetadata,
};
use musicbox_organizer::services::CatalogExportService;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_unified_catalog_exporter_txt_rtf_pdf() {
    let dir = tempdir().expect("tempdir");

    let tracks = vec![
        OrganizerTrack {
            index: 1,
            file_path: dir.path().join("track1.mp3"),
            original_filename: "track1.mp3".to_string(),
            xxyy_prefix: "0101".to_string(),
            metadata: TrackMetadata {
                track_number: Some(1),
                title: "First Track".to_string(),
                artist: "Artist One".to_string(),
                album: "Album".to_string(),
                year: Some(2025),
                duration_secs: 150,
            },
            is_modified: false,
        },
        OrganizerTrack {
            index: 2,
            file_path: dir.path().join("track2.mp3"),
            original_filename: "track2.mp3".to_string(),
            xxyy_prefix: "0102".to_string(),
            metadata: TrackMetadata {
                track_number: Some(2),
                title: "Second Track".to_string(),
                artist: "Artist Two".to_string(),
                album: "Album".to_string(),
                year: Some(2025),
                duration_secs: 210,
            },
            is_modified: false,
        },
    ];

    let exporter = CatalogExportService::new();

    // 1. Plain Text Export
    let txt_path = dir.path().join("catalog.txt");
    let config_txt = CatalogExportConfig {
        format: CatalogFormat::Txt,
        fields: CatalogFieldSelection {
            track_number: true,
            title: true,
            artist: true,
            export_filename: true,
            header_title: true,
        },
        ..Default::default()
    };
    exporter
        .export_catalog(&txt_path, &tracks, Some("My Text Catalog"), &config_txt)
        .expect("txt export succeeds");

    assert!(txt_path.exists());
    let txt_content = fs::read_to_string(&txt_path).expect("read txt");
    assert!(txt_content.contains("=== My Text Catalog ==="));
    assert!(txt_content.contains("01. First Track - Artist One (0101-track1.mp3)"));
    assert!(txt_content.contains("02. Second Track - Artist Two (0102-track2.mp3)"));

    // 2. Rich Text Export
    let rtf_path = dir.path().join("catalog.rtf");
    let config_rtf = CatalogExportConfig {
        format: CatalogFormat::Rtf,
        fields: CatalogFieldSelection {
            track_number: true,
            title: true,
            artist: false,
            export_filename: false,
            header_title: false,
        },
        ..Default::default()
    };
    exporter
        .export_catalog(&rtf_path, &tracks, Some("Ignored Title"), &config_rtf)
        .expect("rtf export succeeds");

    assert!(rtf_path.exists());
    let rtf_content = fs::read_to_string(&rtf_path).expect("read rtf");
    assert!(rtf_content.starts_with("{\\rtf1"));
    assert!(rtf_content.contains("First Track"));
    assert!(!rtf_content.contains("Artist One")); // Artist disabled

    // 3. PDF Export
    let pdf_path = dir.path().join("catalog.pdf");
    let config_pdf = CatalogExportConfig {
        format: CatalogFormat::Pdf,
        ..Default::default()
    };
    exporter
        .export_catalog(&pdf_path, &tracks, Some("My PDF"), &config_pdf)
        .expect("pdf export succeeds");

    assert!(pdf_path.exists());
    let pdf_bytes = fs::read(&pdf_path).expect("read pdf bytes");
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}
