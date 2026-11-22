use image::{Rgba, RgbaImage};
use musicbox_organizer::models::{
    CatalogExportConfig, CatalogFieldSelection, CatalogPagePreset, ImageFitMode, LengthUnit,
    OrganizerTrack, PhysicalDimensions, TrackMetadata,
};
use musicbox_organizer::services::{CatalogThemeRegistry, PdfCatalogExporter};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_pdf_theme_with_raster_background_image() {
    let dir = tempdir().expect("tempdir");

    // 1. Create a dummy PNG background image using image crate
    let img_path = dir.path().join("bg.png");
    let mut img = RgbaImage::new(100, 80);
    for pixel in img.pixels_mut() {
        *pixel = Rgba([200, 150, 100, 255]);
    }
    img.save(&img_path).expect("save png image");

    let tracks = vec![OrganizerTrack {
        index: 1,
        file_path: dir.path().join("track1.mp3"),
        original_filename: "track1.mp3".to_string(),
        xxyy_prefix: "0101".to_string(),
        metadata: TrackMetadata {
            track_number: Some(1),
            title: "Theme Song".to_string(),
            artist: "Themed Artist".to_string(),
            album: "Themed Album".to_string(),
            year: Some(2026),
            duration_secs: 190,
        },
        is_modified: false,
    }];

    let config = CatalogExportConfig {
        dimensions: PhysicalDimensions {
            unit: LengthUnit::Cm,
            width: 10.0,
            height: 10.0,
            margin: 0.5,
            preset: CatalogPagePreset::Card10x10,
        },
        cut_border: false,
        fields: CatalogFieldSelection::default(),
        ..Default::default()
    };

    // 2. Test Cover mode with opacity 0.6
    let mut theme = CatalogThemeRegistry::get_theme("Warm Paper");
    theme.bg_image = Some(img_path.to_string_lossy().to_string());
    theme.bg_opacity = 0.6;
    theme.bg_fit = ImageFitMode::Cover;

    let exporter = PdfCatalogExporter::new();
    let pdf_cover_path = dir.path().join("catalog_cover.pdf");
    exporter
        .export_pdf(
            &pdf_cover_path,
            &tracks,
            Some("Cover Theme"),
            &config,
            &theme,
        )
        .expect("pdf export with cover image should succeed");

    assert!(pdf_cover_path.exists());
    let bytes_cover = fs::read(&pdf_cover_path).expect("read pdf");
    assert_eq!(&bytes_cover[0..5], b"%PDF-");

    // 3. Test Contain mode with Dark Slate theme
    let mut dark_theme = CatalogThemeRegistry::get_theme("Dark Slate");
    dark_theme.bg_image = Some(img_path.to_string_lossy().to_string());
    dark_theme.bg_opacity = 0.3;
    dark_theme.bg_fit = ImageFitMode::Contain;

    let pdf_contain_path = dir.path().join("catalog_contain.pdf");
    exporter
        .export_pdf(
            &pdf_contain_path,
            &tracks,
            Some("Dark Slate Contain"),
            &config,
            &dark_theme,
        )
        .expect("pdf export with contain image should succeed");

    assert!(pdf_contain_path.exists());
    let bytes_contain = fs::read(&pdf_contain_path).expect("read pdf");
    assert_eq!(&bytes_contain[0..5], b"%PDF-");
}
