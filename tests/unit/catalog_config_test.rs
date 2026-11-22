use musicbox_organizer::models::{CatalogPagePreset, ImageFitMode, LengthUnit, PhysicalDimensions};
use musicbox_organizer::services::CatalogThemeRegistry;

#[test]
fn test_length_unit_conversions() {
    // 1 in = 72 pt
    let in_pt = LengthUnit::Inches.to_points(1.0);
    assert!((in_pt - 72.0).abs() < 1e-4);
    assert!((LengthUnit::Inches.from_points(72.0) - 1.0).abs() < 1e-4);

    // 1 cm = 28.3464567 pt
    let cm_pt = LengthUnit::Cm.to_points(1.0);
    assert!((cm_pt - 28.346_457).abs() < 1e-4);
    assert!((LengthUnit::Cm.from_points(cm_pt) - 1.0).abs() < 1e-4);

    // 1 mm = 2.83464567 pt
    let mm_pt = LengthUnit::Mm.to_points(1.0);
    assert!((mm_pt - 2.834_645_7).abs() < 1e-4);
    assert!((LengthUnit::Mm.from_points(mm_pt) - 1.0).abs() < 1e-4);
}

#[test]
fn test_physical_dimensions_clamping() {
    let mut dims = PhysicalDimensions {
        unit: LengthUnit::Cm,
        width: 0.2, // Below 1.0 cm min
        height: 0.5,
        margin: -1.0,
        preset: CatalogPagePreset::Custom,
    };
    dims.clamp_dimensions();
    assert_eq!(dims.width, 1.0);
    assert_eq!(dims.height, 1.0);
    assert_eq!(dims.margin, 0.0);
}

#[test]
fn test_physical_dimensions_unit_switch() {
    let mut dims = PhysicalDimensions {
        unit: LengthUnit::Cm,
        width: 10.0,
        height: 10.0,
        margin: 0.5,
        preset: CatalogPagePreset::Card10x10,
    };
    let orig_w_pt = dims.width_pt();

    dims.set_unit(LengthUnit::Mm);
    assert_eq!(dims.unit, LengthUnit::Mm);
    assert!((dims.width - 100.0).abs() < 1e-2);
    assert!((dims.width_pt() - orig_w_pt).abs() < 1e-2);

    dims.set_unit(LengthUnit::Inches);
    assert_eq!(dims.unit, LengthUnit::Inches);
    assert!((dims.width - (10.0 / 2.54)).abs() < 1e-2);
    assert!((dims.width_pt() - orig_w_pt).abs() < 1e-2);
}

#[test]
fn test_theme_registry_embedded_themes() {
    let themes = CatalogThemeRegistry::available_themes();
    assert!(themes.contains(&"White Monochrome"));
    assert!(themes.contains(&"Warm Paper"));
    assert!(themes.contains(&"Dark Slate"));

    let white = CatalogThemeRegistry::get_theme("White Monochrome");
    assert_eq!(white.name, "White Monochrome");
    assert_eq!(white.bg_color, [255, 255, 255]);
    assert_eq!(white.font_color, [20, 20, 20]);

    let warm = CatalogThemeRegistry::get_theme("Warm Paper");
    assert_eq!(warm.name, "Warm Paper");
    assert_eq!(warm.bg_color, [253, 251, 247]);

    let dark = CatalogThemeRegistry::get_theme("Dark Slate");
    assert_eq!(dark.name, "Dark Slate");
    assert_eq!(dark.bg_color, [30, 32, 34]);

    // Unknown fallback
    let unknown = CatalogThemeRegistry::get_theme("NonExistent");
    assert_eq!(unknown.name, "White Monochrome");
}

#[test]
fn test_theme_custom_json_parsing() {
    let json = r#"{
        "name": "Custom Green",
        "bg_color": [200, 255, 200],
        "font_color": [0, 50, 0],
        "accent_color": [0, 100, 0],
        "border_color": [150, 200, 150],
        "bg_image": null,
        "bg_opacity": 0.8,
        "bg_fit": "Contain",
        "font_family": "Roboto",
        "font_size": 9.0,
        "header_font_size": 12.0,
        "line_height": 1.4,
        "column_gap": 12.0
    }"#;
    let theme = CatalogThemeRegistry::parse_theme_json(json).expect("valid theme json");
    assert_eq!(theme.name, "Custom Green");
    assert_eq!(theme.bg_fit, ImageFitMode::Contain);
    assert_eq!(theme.bg_opacity, 0.8);
}
