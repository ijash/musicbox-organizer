use crate::models::{CatalogTheme, ComputedCatalogLayout};

/// Estimated unit width (at 1.0 pt font size) for a character in Roboto.
pub fn roboto_char_unit_width(c: char) -> f32 {
    match c {
        ' ' => 0.25,
        'i' | 'l' | 'I' | '1' | '.' | ',' | ':' | ';' | '!' | '|' | '\'' | 't' | 'f' | 'r'
        | 'j' => 0.28,
        'A'..='Z' | 'm' | 'w' => 0.62,
        _ => 0.48,
    }
}

/// Estimated total unit width for a line of text in Roboto, including a 1.04 safety factor.
pub fn estimate_roboto_line_unit_width(s: &str) -> f32 {
    let raw_sum: f32 = s.chars().map(roboto_char_unit_width).sum();
    raw_sum * 1.04
}

/// Pure layout solver for catalog pages.
/// cite: data-model.md ComputedCatalogLayout, research.md §4
pub struct CatalogLayoutSolver;

impl CatalogLayoutSolver {
    /// Solves the optimal column count and font size to fit all tracks on the physical page.
    pub fn solve(
        width_pt: f32,
        height_pt: f32,
        margin_pt: f32,
        track_count: usize,
        has_header: bool,
        theme: &CatalogTheme,
    ) -> ComputedCatalogLayout {
        Self::solve_with_line_widths(
            width_pt,
            height_pt,
            margin_pt,
            track_count,
            has_header,
            theme,
            None,
        )
    }

    /// Solves the optimal column count and font size given character lengths, converting them to estimated unit widths.
    pub fn solve_with_line_lengths(
        width_pt: f32,
        height_pt: f32,
        margin_pt: f32,
        track_count: usize,
        has_header: bool,
        theme: &CatalogTheme,
        line_lengths: Option<&[usize]>,
    ) -> ComputedCatalogLayout {
        let widths: Option<Vec<f32>> =
            line_lengths.map(|lens| lens.iter().map(|&l| (l as f32) * 0.58).collect());
        Self::solve_with_line_widths(
            width_pt,
            height_pt,
            margin_pt,
            track_count,
            has_header,
            theme,
            widths.as_deref(),
        )
    }

    /// Solves the optimal column count and font size to fit all tracks both vertically and horizontally
    /// using measured Roboto unit widths per line.
    pub fn solve_with_line_widths(
        width_pt: f32,
        height_pt: f32,
        margin_pt: f32,
        track_count: usize,
        has_header: bool,
        theme: &CatalogTheme,
        line_widths: Option<&[f32]>,
    ) -> ComputedCatalogLayout {
        let physical_width_cm = width_pt / 28.346_457;
        let physical_height_cm = height_pt / 28.346_457;

        let canvas_width = (width_pt - 2.0 * margin_pt).max(10.0);
        let mut canvas_height = (height_pt - 2.0 * margin_pt).max(10.0);

        // Header height allocation
        if has_header {
            let header_space = theme.header_font_size * 1.5 + 8.0;
            if canvas_height > header_space + 20.0 {
                canvas_height -= header_space;
            }
        }

        if track_count == 0 {
            return ComputedCatalogLayout {
                column_count: 1,
                column_width_pt: canvas_width,
                column_widths: vec![canvas_width],
                row_height_pt: theme.font_size * theme.line_height,
                computed_font_size_pt: theme.font_size,
                total_pages: 1,
                readability_warning: false,
                physical_width_cm,
                physical_height_cm,
            };
        }

        // Evaluate candidate column counts 1..=5
        let mut best_cols = 1;
        let mut best_font_size = 1.0f32;
        let mut best_row_height = 4.0f32;
        let mut best_col_width = canvas_width;
        let mut best_col_widths = vec![canvas_width];

        let max_candidates = if canvas_width < 120.0 {
            2
        } else if canvas_width < 250.0 {
            3
        } else if canvas_width < 400.0 {
            4
        } else {
            5
        };

        // Maximum allowed autofit font size
        const MAX_AUTOFIT_FONT_SIZE: f32 = 24.0;

        for cols in 1..=max_candidates {
            let total_gap = (cols - 1) as f32 * theme.column_gap;
            if canvas_width <= total_gap + 30.0 {
                continue;
            }
            let available_for_cols = canvas_width - total_gap;
            let rows_needed = track_count.div_ceil(cols);

            // Reserve font_size * 0.25 bottom descender space so last row never touches bottom edge
            let f_vert = (canvas_height / (rows_needed as f32 * theme.line_height + 0.25)).max(1.0);
            let row_h = (canvas_height - f_vert * 0.25) / (rows_needed as f32);

            // Determine max unit width in each column
            let mut max_col_unit_widths = Vec::with_capacity(cols);
            if let Some(widths) = line_widths {
                for c_idx in 0..cols {
                    let start = c_idx * rows_needed;
                    let end = ((c_idx + 1) * rows_needed).min(widths.len());
                    let max_w = if start < end {
                        widths[start..end]
                            .iter()
                            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                            .copied()
                            .unwrap_or(15.0)
                    } else {
                        15.0
                    };
                    max_col_unit_widths.push(max_w.max(1.0));
                }
            } else {
                max_col_unit_widths = vec![15.0; cols];
            }

            let sum_widths: f32 = max_col_unit_widths.iter().sum();
            let total_w = sum_widths.max(1.0);

            // Weight column widths proportionally to max_unit_width in each column
            let mut col_widths: Vec<f32> = max_col_unit_widths
                .iter()
                .map(|&w| (w / total_w) * available_for_cols)
                .collect();

            // Enforce minimum column width of 25.0 pt to prevent pathological collapse
            let mut adjusted = false;
            for w in col_widths.iter_mut() {
                if *w < 25.0 {
                    *w = 25.0;
                    adjusted = true;
                }
            }
            if adjusted {
                let current_sum: f32 = col_widths.iter().sum();
                if current_sum > available_for_cols {
                    continue;
                }
                let ratio = available_for_cols / current_sum;
                for w in col_widths.iter_mut() {
                    *w *= ratio;
                }
            }

            // Horizontal font limit for each column: font_size <= col_width / unit_width
            let mut min_horiz_f = f32::MAX;
            for (c_idx, &unit_w) in max_col_unit_widths.iter().enumerate() {
                let col_f = col_widths[c_idx] / unit_w.max(0.1);
                if col_f < min_horiz_f {
                    min_horiz_f = col_f;
                }
            }

            let target_font = f_vert.min(min_horiz_f).min(MAX_AUTOFIT_FONT_SIZE);

            if target_font > best_font_size || (target_font == best_font_size && cols < best_cols) {
                best_font_size = target_font;
                best_cols = cols;
                best_row_height = row_h;
                best_col_width = available_for_cols / cols as f32;
                best_col_widths = col_widths;
            }
        }

        // Floor font size at 1.0 pt
        let computed_font_size_pt = best_font_size.max(1.0);
        let readability_warning = computed_font_size_pt < 5.0;

        ComputedCatalogLayout {
            column_count: best_cols,
            column_width_pt: best_col_width,
            column_widths: best_col_widths,
            row_height_pt: best_row_height,
            computed_font_size_pt,
            total_pages: 1,
            readability_warning,
            physical_width_cm,
            physical_height_cm,
        }
    }
}
