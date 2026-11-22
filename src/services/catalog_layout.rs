use crate::models::{CatalogTheme, ComputedCatalogLayout};
use std::sync::LazyLock;

static ROBOTO_BYTES: &[u8] = include_bytes!("../../assets/fonts/Roboto-Regular.ttf");

struct RobotoMetrics {
    units_per_em: f32,
    num_h_metrics: u16,
    format4_offset: usize,
    hmtx_offset: usize,
    ascii_advances: [f32; 128],
}

impl RobotoMetrics {
    fn init() -> Self {
        let data = ROBOTO_BYTES;
        let num_tables = u16::from_be_bytes([data[4], data[5]]) as usize;
        let mut head_offset = 0;
        let mut hhea_offset = 0;
        let mut cmap_offset = 0;
        let mut hmtx_offset = 0;

        for i in 0..num_tables {
            let base = 12 + i * 16;
            if base + 16 > data.len() {
                break;
            }
            let tag = &data[base..base + 4];
            let offset = u32::from_be_bytes([
                data[base + 8],
                data[base + 9],
                data[base + 10],
                data[base + 11],
            ]) as usize;
            match tag {
                b"head" => head_offset = offset,
                b"hhea" => hhea_offset = offset,
                b"cmap" => cmap_offset = offset,
                b"hmtx" => hmtx_offset = offset,
                _ => {}
            }
        }

        let units_per_em = if head_offset + 20 <= data.len() {
            u16::from_be_bytes([data[head_offset + 18], data[head_offset + 19]]) as f32
        } else {
            2048.0
        };

        let num_h_metrics = if hhea_offset + 36 <= data.len() {
            u16::from_be_bytes([data[hhea_offset + 34], data[hhea_offset + 35]])
        } else {
            1
        };

        let mut format4_offset = 0;
        if cmap_offset + 4 <= data.len() {
            let num_subtables =
                u16::from_be_bytes([data[cmap_offset + 2], data[cmap_offset + 3]]) as usize;
            for i in 0..num_subtables {
                let rec = cmap_offset + 4 + i * 8;
                if rec + 8 > data.len() {
                    break;
                }
                let sub_off = u32::from_be_bytes([
                    data[rec + 4],
                    data[rec + 5],
                    data[rec + 6],
                    data[rec + 7],
                ]) as usize;
                let fmt_off = cmap_offset + sub_off;
                if fmt_off + 2 <= data.len()
                    && u16::from_be_bytes([data[fmt_off], data[fmt_off + 1]]) == 4
                {
                    format4_offset = fmt_off;
                    break;
                }
            }
        }

        let mut metrics = Self {
            units_per_em,
            num_h_metrics,
            format4_offset,
            hmtx_offset,
            ascii_advances: [0.0; 128],
        };

        for i in 0..128u8 {
            metrics.ascii_advances[i as usize] = metrics.lookup_advance(i as char);
        }

        metrics
    }

    fn lookup_advance(&self, c: char) -> f32 {
        let cp = c as u32;
        if cp > 0xFFFF || self.format4_offset == 0 || self.hmtx_offset == 0 {
            return 0.5;
        }
        let data = ROBOTO_BYTES;
        if self.format4_offset + 14 > data.len() {
            return 0.5;
        }

        let seg_count = (u16::from_be_bytes([
            data[self.format4_offset + 6],
            data[self.format4_offset + 7],
        ]) / 2) as usize;

        let end_codes = self.format4_offset + 14;
        let start_codes = end_codes + seg_count * 2 + 2;
        let id_deltas = start_codes + seg_count * 2;
        let id_range_offsets = id_deltas + seg_count * 2;

        if id_range_offsets + seg_count * 2 > data.len() {
            return 0.5;
        }

        let cp_u16 = cp as u16;
        let mut glyph_id = 0u16;

        for s in 0..seg_count {
            let end_code =
                u16::from_be_bytes([data[end_codes + s * 2], data[end_codes + s * 2 + 1]]);
            if cp_u16 <= end_code {
                let start_code =
                    u16::from_be_bytes([data[start_codes + s * 2], data[start_codes + s * 2 + 1]]);
                if cp_u16 >= start_code {
                    let id_delta =
                        i16::from_be_bytes([data[id_deltas + s * 2], data[id_deltas + s * 2 + 1]]);
                    let ro = u16::from_be_bytes([
                        data[id_range_offsets + s * 2],
                        data[id_range_offsets + s * 2 + 1],
                    ]) as usize;
                    if ro == 0 {
                        glyph_id = cp_u16.wrapping_add(id_delta as u16);
                    } else {
                        let ptr =
                            id_range_offsets + s * 2 + ro + (cp_u16 - start_code) as usize * 2;
                        if ptr + 2 <= data.len() {
                            let g = u16::from_be_bytes([data[ptr], data[ptr + 1]]);
                            if g != 0 {
                                glyph_id = g.wrapping_add(id_delta as u16);
                            }
                        }
                    }
                }
                break;
            }
        }

        let adv_u16 = if (glyph_id as usize) < self.num_h_metrics as usize {
            let off = self.hmtx_offset + glyph_id as usize * 4;
            if off + 2 <= data.len() {
                u16::from_be_bytes([data[off], data[off + 1]])
            } else {
                1024
            }
        } else {
            let off = self.hmtx_offset + (self.num_h_metrics as usize - 1) * 4;
            if off + 2 <= data.len() {
                u16::from_be_bytes([data[off], data[off + 1]])
            } else {
                1024
            }
        };

        adv_u16 as f32 / self.units_per_em
    }

    fn advance(&self, c: char) -> f32 {
        let cp = c as usize;
        if cp < 128 {
            self.ascii_advances[cp]
        } else {
            self.lookup_advance(c)
        }
    }
}

static ROBOTO_METRICS: LazyLock<RobotoMetrics> = LazyLock::new(RobotoMetrics::init);

/// Exact glyph advance width for a character in Roboto-Regular at 1.0 pt font size.
pub fn measure_roboto_char_unit_width(c: char) -> f32 {
    ROBOTO_METRICS.advance(c)
}

/// Exact total advance width in points (per 1.0 pt of font size) for a line of text,
/// including a 1.01 rounding guard.
pub fn measure_roboto_line_unit_width(text: &str) -> f32 {
    let raw_sum: f32 = text.chars().map(measure_roboto_char_unit_width).sum();
    raw_sum * 1.01
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

        // Header height allocation: scale header font proportionally
        if has_header {
            let est_header_f = (height_pt * 0.14).min(theme.header_font_size).max(2.5);
            let header_space = est_header_f + est_header_f * 0.35 + 4.0;
            if canvas_height > header_space + 10.0 {
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
            let rows_needed = track_count.div_ceil(cols);
            // Reserve font_size * 0.25 bottom descender space so last row never touches bottom edge
            let f_vert = (canvas_height / (rows_needed as f32 * theme.line_height + 0.25)).max(1.0);
            let col_gap = (f_vert * 1.2).clamp(2.0, theme.column_gap);
            let total_gap = (cols - 1) as f32 * col_gap;
            if canvas_width <= total_gap + 20.0 {
                continue;
            }
            let available_for_cols = canvas_width - total_gap;
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
