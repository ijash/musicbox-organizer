use crate::models::{CatalogExportConfig, CatalogTheme, OrganizerTrack};
use crate::services::CatalogLayoutSolver;
use krilla::Data;
use krilla::color::rgb::Color as RgbColor;
use krilla::document::Document;
use krilla::geom::{PathBuilder, Point, Rect, Size, Transform};
use krilla::image::Image;
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, FillRule, Paint, Stroke};
use krilla::text::Font;
use krilla::text::TextDirection;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

static ROBOTO_REGULAR: LazyLock<Font> = LazyLock::new(|| {
    let bytes = include_bytes!("../../assets/fonts/Roboto-Regular.ttf");
    let data = Data::from(bytes.as_slice());
    Font::new(data, 0).expect("failed to load embedded Roboto-Regular font")
});

static ROBOTO_BOLD: LazyLock<Font> = LazyLock::new(|| {
    let bytes = include_bytes!("../../assets/fonts/Roboto-Bold.ttf");
    let data = Data::from(bytes.as_slice());
    Font::new(data, 0).expect("failed to load embedded Roboto-Bold font")
});

pub struct PdfCatalogExporter;

impl Default for PdfCatalogExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfCatalogExporter {
    pub fn new() -> Self {
        Self
    }

    /// Renders tracks into a PDF catalog using config and theme, writing atomically to dest_path
    pub fn export_pdf(
        &self,
        dest_path: &Path,
        tracks: &[OrganizerTrack],
        playlist_title: Option<&str>,
        config: &CatalogExportConfig,
        theme: &CatalogTheme,
    ) -> Result<(), Box<dyn Error>> {
        let width_pt = config.dimensions.width_pt();
        let height_pt = config.dimensions.height_pt();
        let margin_pt = config.dimensions.margin_pt();

        let has_header = config.fields.header_title;
        let track_lines: Vec<String> = tracks
            .iter()
            .enumerate()
            .map(|(idx, track)| self.format_track_line(idx + 1, track, &config.fields))
            .collect();
        let line_lengths: Vec<usize> = track_lines.iter().map(|l| l.chars().count()).collect();

        let layout = CatalogLayoutSolver::solve_with_line_lengths(
            width_pt,
            height_pt,
            margin_pt,
            tracks.len(),
            has_header,
            theme,
            Some(&line_lengths),
        );

        let mut doc = Document::new();
        let page_settings = PageSettings::from_wh(width_pt, height_pt)
            .ok_or_else(|| format!("Invalid page dimensions: {} x {}", width_pt, height_pt))?;

        let mut page = doc.start_page_with(page_settings);
        let mut surface = page.surface();

        // 1. Draw Background Fill
        let bg_rgb = RgbColor::new(theme.bg_color[0], theme.bg_color[1], theme.bg_color[2]);
        let bg_fill = Fill {
            paint: Paint::from(bg_rgb),
            ..Default::default()
        };
        surface.set_fill(Some(bg_fill));

        let mut bg_path = PathBuilder::new();
        if let Some(r) = Rect::from_xywh(0.0, 0.0, width_pt, height_pt) {
            bg_path.push_rect(r);
            if let Some(path) = bg_path.finish() {
                surface.draw_path(&path);
            }
        }
        surface.set_fill(None);

        // 1b. Draw Optional Raster Background Image
        self.draw_background_image(&mut surface, width_pt, height_pt, theme);

        // 2. Draw Optional Cut Border / Crop Marks
        if config.cut_border {
            let border_rgb = RgbColor::new(
                theme.border_color[0],
                theme.border_color[1],
                theme.border_color[2],
            );
            let border_stroke = Stroke {
                paint: Paint::from(border_rgb),
                width: 0.5,
                ..Default::default()
            };
            surface.set_stroke(Some(border_stroke));

            let mut border_path = PathBuilder::new();
            let b_inset = (margin_pt / 2.0).max(2.0);
            if let Some(r) =
                Rect::from_ltrb(b_inset, b_inset, width_pt - b_inset, height_pt - b_inset)
            {
                border_path.push_rect(r);
                if let Some(path) = border_path.finish() {
                    surface.draw_path(&path);
                }
            }
            surface.set_stroke(None);
        }

        // 3. Draw Header Title if enabled
        let mut current_y = margin_pt;
        let font_rgb = RgbColor::new(
            theme.font_color[0],
            theme.font_color[1],
            theme.font_color[2],
        );
        let text_fill = Fill {
            paint: Paint::from(font_rgb),
            ..Default::default()
        };
        surface.set_fill(Some(text_fill));

        if has_header {
            let header_text = playlist_title.unwrap_or("Playlist Catalog");
            let header_font_size = theme.header_font_size;
            surface.draw_text(
                Point::from_xy(margin_pt, current_y + header_font_size),
                ROBOTO_BOLD.clone(),
                header_font_size,
                header_text,
                false,
                TextDirection::Auto,
            );
            current_y += header_font_size * 1.5 + 4.0;

            // Subtle divider line
            let div_rgb = RgbColor::new(
                theme.accent_color[0],
                theme.accent_color[1],
                theme.accent_color[2],
            );
            let div_stroke = Stroke {
                paint: Paint::from(div_rgb),
                width: 0.5,
                ..Default::default()
            };
            surface.set_stroke(Some(div_stroke));

            let mut div_path = PathBuilder::new();
            div_path.move_to(margin_pt, current_y);
            div_path.line_to(width_pt - margin_pt, current_y);
            if let Some(path) = div_path.finish() {
                surface.draw_path(&path);
            }
            surface.set_stroke(None);

            current_y += 6.0;
        }

        // 4. Draw Tracks in Columns
        let cols = layout.column_count;
        let row_height = layout.row_height_pt;
        let font_size = layout.computed_font_size_pt;

        let tracks_per_col = tracks.len().div_ceil(cols);

        let mut col_x = Vec::with_capacity(cols);
        let mut cur_x = margin_pt;
        for c_idx in 0..cols {
            col_x.push(cur_x);
            let w = layout
                .column_widths
                .get(c_idx)
                .copied()
                .unwrap_or(layout.column_width_pt);
            cur_x += w + theme.column_gap;
        }

        for (idx, _track) in tracks.iter().enumerate() {
            let col_idx = idx.checked_div(tracks_per_col).unwrap_or(0);
            let row_idx = if tracks_per_col > 0 {
                idx % tracks_per_col
            } else {
                0
            };

            let x = col_x.get(col_idx).copied().unwrap_or(margin_pt);
            let col_width = layout
                .column_widths
                .get(col_idx)
                .copied()
                .unwrap_or(layout.column_width_pt);
            let y = current_y + (row_idx as f32) * row_height;

            let line = &track_lines[idx];
            let approx_char_limit = ((col_width / (font_size * 0.58)) as usize).max(4);
            let text_to_draw = if font_size < 1.0 && line.chars().count() > approx_char_limit {
                let mut s: String = line
                    .chars()
                    .take(approx_char_limit.saturating_sub(2))
                    .collect();
                s.push_str("..");
                s
            } else {
                line.clone()
            };

            surface.draw_text(
                Point::from_xy(x, y + font_size),
                ROBOTO_REGULAR.clone(),
                font_size,
                &text_to_draw,
                false,
                TextDirection::Auto,
            );
        }

        surface.finish();
        page.finish();

        let pdf_data = doc
            .finish()
            .map_err(|e| format!("PDF export error: {:?}", e))?;

        // 5. Atomic write via NamedTempFile::new_in(parent)
        let parent_dir = dest_path.parent().unwrap_or_else(|| Path::new("."));
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }

        let mut temp_file = tempfile::NamedTempFile::new_in(parent_dir)?;
        temp_file.write_all(&pdf_data)?;
        temp_file.persist(dest_path)?;

        Ok(())
    }

    pub fn format_track_line(
        &self,
        index: usize,
        track: &OrganizerTrack,
        fields: &crate::models::CatalogFieldSelection,
    ) -> String {
        let mut parts = Vec::new();

        if fields.track_number {
            parts.push(format!("{:02}.", index));
        }

        if fields.title {
            let title = if !track.metadata.title.trim().is_empty() {
                track.metadata.title.as_str()
            } else {
                track.original_filename.as_str()
            };
            parts.push(title.to_string());
        }

        if fields.artist && !track.metadata.artist.trim().is_empty() {
            parts.push(format!("- {}", track.metadata.artist));
        }

        if fields.export_filename {
            parts.push(format!("({})", track.formatted_filename()));
        }

        if parts.is_empty() {
            format!("{:02}. Track {}", index, index)
        } else {
            parts.join(" ")
        }
    }

    fn draw_background_image(
        &self,
        surface: &mut krilla::surface::Surface,
        width_pt: f32,
        height_pt: f32,
        theme: &CatalogTheme,
    ) {
        let Some(bg_image_path) = &theme.bg_image else {
            return;
        };

        let Ok(bytes) = fs::read(Path::new(bg_image_path)) else {
            return;
        };

        let Ok(dyn_img) = image::load_from_memory(&bytes) else {
            return;
        };

        let rgba = dyn_img.to_rgba8();
        let (iw, ih) = rgba.dimensions();
        if iw == 0 || ih == 0 {
            return;
        }

        let krilla_img = Image::from_rgba8(rgba.into_raw(), iw, ih);
        let iw_f = iw as f32;
        let ih_f = ih as f32;
        let img_aspect = iw_f / ih_f;
        let page_aspect = width_pt / height_pt;

        let (draw_w, draw_h, offset_x, offset_y) = match theme.bg_fit {
            crate::models::ImageFitMode::Cover => {
                if img_aspect > page_aspect {
                    let h = height_pt;
                    let w = h * img_aspect;
                    let x = (width_pt - w) / 2.0;
                    (w, h, x, 0.0)
                } else {
                    let w = width_pt;
                    let h = w / img_aspect;
                    let y = (height_pt - h) / 2.0;
                    (w, h, 0.0, y)
                }
            }
            crate::models::ImageFitMode::Contain => {
                if img_aspect > page_aspect {
                    let w = width_pt;
                    let h = w / img_aspect;
                    let y = (height_pt - h) / 2.0;
                    (w, h, 0.0, y)
                } else {
                    let h = height_pt;
                    let w = h * img_aspect;
                    let x = (width_pt - w) / 2.0;
                    (w, h, x, 0.0)
                }
            }
        };

        let mut clip_path = PathBuilder::new();
        if let Some(r) = Rect::from_xywh(0.0, 0.0, width_pt, height_pt) {
            clip_path.push_rect(r);
            if let Some(p) = clip_path.finish() {
                surface.push_clip_path(&p, &FillRule::NonZero);
                let opacity = theme.bg_opacity.clamp(0.0, 1.0);
                if let Some(norm_opacity) = NormalizedF32::new(opacity) {
                    surface.push_opacity(norm_opacity);
                    surface.push_transform(&Transform::from_translate(offset_x, offset_y));
                    if let Some(size) = Size::from_wh(draw_w, draw_h) {
                        surface.draw_image(krilla_img, size);
                    }
                    surface.pop();
                    surface.pop();
                }
                surface.pop();
            }
        }
    }
}
