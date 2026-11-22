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
    assert!(layout.computed_font_size_pt >= theme.font_size);
    assert!(layout.computed_font_size_pt <= 24.0);
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

#[test]
fn test_layout_solver_21_realistic_long_titles_on_10x10cm() {
    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(10.0);
    let height_pt = LengthUnit::Cm.to_points(10.0);
    let margin_pt = LengthUnit::Cm.to_points(0.5);

    // 21 realistic titles (~35 to 65 characters)
    let titles = vec![
        "01. Bohemian Rhapsody - Queen (AAAA-bohemian.mp3)",
        "02. Hotel California - Eagles (AAAB-hotel.mp3)",
        "03. Stairway to Heaven - Led Zeppelin (AAAC-stairway.mp3)",
        "04. Smells Like Teen Spirit - Nirvana (AAAD-smells.mp3)",
        "05. Sweet Child O' Mine - Guns N' Roses (AAAE-sweet.mp3)",
        "06. Billie Jean - Michael Jackson (AAAF-billie.mp3)",
        "07. Imagine - John Lennon (AAAG-imagine.mp3)",
        "08. Like a Rolling Stone - Bob Dylan (AAAH-stone.mp3)",
        "09. One - U2 (AAAI-one.mp3)",
        "10. Comfortably Numb - Pink Floyd (AAAJ-comfortably.mp3)",
        "11. Sympathy for the Devil - The Rolling Stones (AAAK-devil.mp3)",
        "12. Purple Rain - Prince and The Revolution (AAAL-purple.mp3)",
        "13. Good Vibrations - The Beach Boys (AAAM-vibrations.mp3)",
        "14. Johnny B. Goode - Chuck Berry (AAAN-johnny.mp3)",
        "15. Respect - Aretha Franklin (AAAO-respect.mp3)",
        "16. What's Going On - Marvin Gaye (AAAP-whats.mp3)",
        "17. Hey Jude - The Beatles (AAAQ-hey.mp3)",
        "18. London Calling - The Clash (AAAR-london.mp3)",
        "19. Superstition - Stevie Wonder (AAAS-superstition.mp3)",
        "20. Born to Run - Bruce Springsteen (AAAT-born.mp3)",
        "21. Bridge Over Troubled Water - Simon & Garfunkel (AAAU-bridge.mp3)",
    ];

    let line_lengths: Vec<usize> = titles.iter().map(|s| s.chars().count()).collect();
    let layout = CatalogLayoutSolver::solve_with_line_lengths(
        width_pt,
        height_pt,
        margin_pt,
        titles.len(),
        true,
        &theme,
        Some(&line_lengths),
    );

    let rows_per_col = titles.len().div_ceil(layout.column_count);
    for c_idx in 0..layout.column_count {
        let start = c_idx * rows_per_col;
        let end = ((c_idx + 1) * rows_per_col).min(titles.len());
        if start < end {
            let max_len = line_lengths[start..end].iter().max().copied().unwrap();
            let estimated_width = max_len as f32 * layout.computed_font_size_pt * 0.58;
            let col_w = layout
                .column_widths
                .get(c_idx)
                .copied()
                .unwrap_or(layout.column_width_pt);
            assert!(
                estimated_width <= col_w + 0.1,
                "Column {} width {:.1}pt exceeded by estimated text width {:.1}pt (font size {:.1}pt)",
                c_idx,
                col_w,
                estimated_width,
                layout.computed_font_size_pt
            );
        }
    }
    assert!(layout.computed_font_size_pt >= 1.0);
}

#[test]
fn test_roboto_width_estimation_and_solve_with_line_widths() {
    use musicbox_organizer::services::estimate_roboto_line_unit_width;

    let s1 = "11. Tikusan Sawah";
    let w1 = estimate_roboto_line_unit_width(s1);
    // Old uniform 0.58 width was 17 * 0.58 = 9.86
    // New estimate should be ~7.2 - 8.0, eliminating ~25% wasted whitespace
    assert!(w1 < 8.2);
    assert!(w1 > 6.5);

    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(10.0);
    let height_pt = LengthUnit::Cm.to_points(10.0);
    let margin_pt = LengthUnit::Cm.to_points(0.5);

    let titles = [s1, "01. Short", "02. Another Track With Moderate Length"];
    let line_widths: Vec<f32> = titles
        .iter()
        .map(|s| estimate_roboto_line_unit_width(s))
        .collect();

    let layout = CatalogLayoutSolver::solve_with_line_widths(
        width_pt,
        height_pt,
        margin_pt,
        titles.len(),
        false,
        &theme,
        Some(&line_widths),
    );
    assert_eq!(layout.column_count, 1);
    assert!(layout.computed_font_size_pt >= 5.0);
}
