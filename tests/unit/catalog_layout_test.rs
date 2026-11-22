use musicbox_organizer::models::{CatalogTheme, LengthUnit};
use musicbox_organizer::services::CatalogLayoutSolver;

#[test]
fn test_layout_solver_10x10cm_card_normal() {
    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(10.0);
    let height_pt = LengthUnit::Cm.to_points(10.0);
    let margin_pt = LengthUnit::Cm.to_points(0.5);

    let layout = CatalogLayoutSolver::solve(width_pt, height_pt, margin_pt, 20, false, &theme);
    assert!(layout.column_count >= 1 && layout.column_count <= 3);
    assert!(!layout.readability_warning);
    assert!(layout.computed_font_size_pt >= 5.0);
}

#[test]
fn test_layout_solver_single_track() {
    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(10.0);
    let height_pt = LengthUnit::Cm.to_points(10.0);
    let margin_pt = LengthUnit::Cm.to_points(0.5);

    let layout = CatalogLayoutSolver::solve(width_pt, height_pt, margin_pt, 1, false, &theme);
    assert_eq!(layout.column_count, 1);
    assert_eq!(layout.computed_font_size_pt, theme.font_size);
    assert!(!layout.readability_warning);
}

#[test]
fn test_layout_solver_tiny_1x1cm_readability_warning() {
    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(1.0);
    let height_pt = LengthUnit::Cm.to_points(1.0);
    let margin_pt = LengthUnit::Cm.to_points(0.1);

    let layout = CatalogLayoutSolver::solve(width_pt, height_pt, margin_pt, 25, false, &theme);
    assert!(layout.readability_warning);
    assert!(layout.computed_font_size_pt < 5.0);
}

#[test]
fn test_layout_solver_dense_playlist_multi_column() {
    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(14.8); // A5 width
    let height_pt = LengthUnit::Cm.to_points(21.0); // A5 height
    let margin_pt = LengthUnit::Cm.to_points(0.5);

    let layout = CatalogLayoutSolver::solve(width_pt, height_pt, margin_pt, 60, true, &theme);
    assert!(layout.column_count >= 2);
    assert!(layout.computed_font_size_pt >= 4.0);
}
