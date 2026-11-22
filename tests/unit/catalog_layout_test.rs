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
    use musicbox_organizer::services::measure_roboto_line_unit_width;

    let s1 = "11. Morning Light";
    let w1 = measure_roboto_line_unit_width(s1);
    // Exact Roboto advance with 1.01 guard is ~7.5 - 8.2
    assert!(w1 < 9.0);
    assert!(w1 > 7.0);

    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(10.0);
    let height_pt = LengthUnit::Cm.to_points(10.0);
    let margin_pt = LengthUnit::Cm.to_points(0.5);

    let titles = [s1, "01. Short", "02. Another Track With Moderate Length"];
    let line_widths: Vec<f32> = titles
        .iter()
        .map(|s| measure_roboto_line_unit_width(s))
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

#[test]
fn test_layout_solver_21_titles_10x2cm_zero_overlap_and_zero_clipping() {
    use musicbox_organizer::services::measure_roboto_line_unit_width;

    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(10.0);
    let height_pt = LengthUnit::Cm.to_points(2.0);
    let margin_pt = LengthUnit::Cm.to_points(0.1);
    let content_margin = margin_pt.max(1.5);
    let canvas_width = width_pt - 2.0 * content_margin;

    let titles = [
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

    let line_widths: Vec<f32> = titles
        .iter()
        .map(|s| measure_roboto_line_unit_width(s))
        .collect();

    let layout = CatalogLayoutSolver::solve_with_line_widths(
        width_pt,
        height_pt,
        content_margin,
        titles.len(),
        false,
        &theme,
        Some(&line_widths),
    );

    let cols = layout.column_count;
    assert!(cols >= 1);
    let rows_needed = titles.len().div_ceil(cols);
    let sum_w: f32 = layout.column_widths.iter().sum();
    let gap = if cols > 1 {
        (canvas_width - sum_w) / (cols - 1) as f32
    } else {
        0.0
    };

    if cols > 1 {
        assert!(
            gap >= (layout.computed_font_size_pt * 1.1).max(3.5) - 0.001,
            "Gap {:.2}pt is smaller than minimum required gap",
            gap
        );
    }

    assert!(
        sum_w + (cols.saturating_sub(1) as f32 * gap) <= canvas_width + 0.01,
        "Total layout width exceeds canvas width"
    );

    for c in 0..cols {
        let start = c * rows_needed;
        let end = ((c + 1) * rows_needed).min(titles.len());
        if start < end {
            let max_unit_w = line_widths[start..end]
                .iter()
                .copied()
                .fold(0.0f32, f32::max);
            let measured_max_line_width_pt = max_unit_w * layout.computed_font_size_pt;
            assert!(
                measured_max_line_width_pt <= layout.column_widths[c] + 0.01,
                "Column {} measured text width {:.2}pt exceeds col_width {:.2}pt",
                c,
                measured_max_line_width_pt,
                layout.column_widths[c]
            );
        }
    }
}

#[test]
fn test_layout_solver_21_titles_8x3cm_exact_zero_dead_space() {
    use musicbox_organizer::services::measure_roboto_line_unit_width;

    let theme = CatalogTheme::default();
    let width_pt = LengthUnit::Cm.to_points(8.6);
    let height_pt = LengthUnit::Cm.to_points(3.5);
    let margin_pt = LengthUnit::Cm.to_points(0.0);
    let content_margin = margin_pt.max(1.5);
    let canvas_width = width_pt - 2.0 * content_margin;
    let canvas_height = height_pt - 2.0 * content_margin;

    let titles = [
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

    let line_widths: Vec<f32> = titles
        .iter()
        .map(|s| measure_roboto_line_unit_width(s))
        .collect();

    let layout = CatalogLayoutSolver::solve_with_line_widths(
        width_pt,
        height_pt,
        content_margin,
        titles.len(),
        true, // has_header = true
        &theme,
        Some(&line_widths),
    );

    let cols = layout.column_count;
    assert!(cols >= 1);
    let rows_needed = titles.len().div_ceil(cols);
    let sum_w: f32 = layout.column_widths.iter().sum();
    let gap = if cols > 1 {
        (canvas_width - sum_w) / (cols - 1) as f32
    } else {
        0.0
    };

    // 1. sum(layout.column_widths) + (cols - 1) * gap equals canvas_width within 0.1 pt (zero right dead space)
    let total_h = sum_w + (cols.saturating_sub(1) as f32 * gap);
    assert!(
        (total_h - canvas_width).abs() <= 0.1,
        "Total horizontal layout width {:.2}pt differs from canvas_width {:.2}pt by > 0.1pt",
        total_h,
        canvas_width
    );

    // 2. header_total + rows * layout.row_height_pt + font_size * 0.25 equals canvas_height within 0.5 pt (zero bottom dead space)
    let font_size = layout.computed_font_size_pt;
    let header_total = CatalogLayoutSolver::header_metrics(font_size, height_pt, &theme).2;
    let total_v = header_total + rows_needed as f32 * layout.row_height_pt + font_size * 0.25;
    assert!(
        (total_v - canvas_height).abs() <= 0.5,
        "Total vertical layout height {:.2}pt differs from canvas_height {:.2}pt by > 0.5pt",
        total_v,
        canvas_height
    );

    // 3. Zero column overlap: measured_max_w[c] * font_size <= layout.column_widths[c] + 0.01
    for c in 0..cols {
        let start = c * rows_needed;
        let end = ((c + 1) * rows_needed).min(titles.len());
        if start < end {
            let max_unit_w = line_widths[start..end]
                .iter()
                .copied()
                .fold(0.0f32, f32::max);
            let measured_max_line_width_pt = max_unit_w * font_size;
            assert!(
                measured_max_line_width_pt <= layout.column_widths[c] + 0.01,
                "Column {} measured text width {:.2}pt exceeds col_width {:.2}pt",
                c,
                measured_max_line_width_pt,
                layout.column_widths[c]
            );
        }
    }
}
