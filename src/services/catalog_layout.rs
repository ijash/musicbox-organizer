use crate::models::{CatalogTheme, ComputedCatalogLayout};

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
        Self::solve_with_line_lengths(
            width_pt,
            height_pt,
            margin_pt,
            track_count,
            has_header,
            theme,
            None,
        )
    }

    /// Solves the optimal column count and font size to fit all tracks both vertically and horizontally.
    pub fn solve_with_line_lengths(
        width_pt: f32,
        height_pt: f32,
        margin_pt: f32,
        track_count: usize,
        has_header: bool,
        theme: &CatalogTheme,
        line_lengths: Option<&[usize]>,
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

        let max_candidates = if canvas_width < 120.0 {
            2
        } else if canvas_width < 250.0 {
            3
        } else if canvas_width < 400.0 {
            4
        } else {
            5
        };

        // Conservative Roboto character width factor
        const CHAR_WIDTH_FACTOR: f32 = 0.58;

        for cols in 1..=max_candidates {
            let total_gap = (cols - 1) as f32 * theme.column_gap;
            if canvas_width <= total_gap + 30.0 {
                continue;
            }
            let col_width = (canvas_width - total_gap) / (cols as f32);
            if col_width < 25.0 {
                continue;
            }

            let rows_needed = track_count.div_ceil(cols);
            let row_h = canvas_height / (rows_needed as f32);
            let f_vert = row_h / theme.line_height;

            let f_horiz = if let Some(lengths) = line_lengths {
                let mut min_col_f = f32::MAX;
                for c_idx in 0..cols {
                    let start = c_idx * rows_needed;
                    let end = ((c_idx + 1) * rows_needed).min(lengths.len());
                    if start < end {
                        let max_chars = lengths[start..end].iter().max().copied().unwrap_or(1);
                        let col_f = col_width / (max_chars as f32 * CHAR_WIDTH_FACTOR).max(1.0);
                        if col_f < min_col_f {
                            min_col_f = col_f;
                        }
                    }
                }
                min_col_f
            } else {
                col_width / (30.0 * CHAR_WIDTH_FACTOR)
            };

            let target_font = f_vert.min(f_horiz).min(theme.font_size);

            if target_font > best_font_size || (target_font == best_font_size && cols < best_cols) {
                best_font_size = target_font;
                best_cols = cols;
                best_row_height = row_h;
                best_col_width = col_width;
            }
        }

        // Floor font size at 1.0 pt
        let computed_font_size_pt = best_font_size.max(1.0);
        let readability_warning = computed_font_size_pt < 5.0;

        ComputedCatalogLayout {
            column_count: best_cols,
            column_width_pt: best_col_width,
            row_height_pt: best_row_height,
            computed_font_size_pt,
            total_pages: 1,
            readability_warning,
            physical_width_cm,
            physical_height_cm,
        }
    }
}
