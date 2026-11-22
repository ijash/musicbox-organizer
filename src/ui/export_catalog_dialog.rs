use crate::models::{
    CatalogExportConfig, CatalogFormat, CatalogPagePreset, ComputedCatalogLayout, LengthUnit,
    OrganizerTrack, ProjectTrack,
};
use crate::services::{
    CatalogExportService, CatalogLayoutSolver, CatalogThemeRegistry, track_index_to_xxyy,
};
use crate::ui::app::MusicBoxApp;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ExportCatalogDialog {
    pub is_open: bool,
    pub config: CatalogExportConfig,
    pub error_message: Option<String>,
}

impl Default for ExportCatalogDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportCatalogDialog {
    pub fn new() -> Self {
        Self {
            is_open: false,
            config: CatalogExportConfig::default(),
            error_message: None,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.error_message = None;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.error_message = None;
    }

    pub fn compute_layout(&self, track_count: usize) -> ComputedCatalogLayout {
        self.compute_layout_with_lengths(track_count, None)
    }

    pub fn compute_layout_with_lengths(
        &self,
        track_count: usize,
        line_lengths: Option<&[usize]>,
    ) -> ComputedCatalogLayout {
        let theme = CatalogThemeRegistry::get_theme(&self.config.theme_name);
        CatalogLayoutSolver::solve_with_line_lengths(
            self.config.dimensions.width_pt(),
            self.config.dimensions.height_pt(),
            self.config.dimensions.margin_pt(),
            track_count,
            self.config.fields.header_title,
            &theme,
            line_lengths,
        )
    }
}

pub fn render_export_catalog_dialog(app: &mut MusicBoxApp, ctx: &egui::Context) {
    if !app.export_catalog_dialog.is_open {
        return;
    }

    let is_pdf = app.export_catalog_dialog.config.format == CatalogFormat::Pdf;
    let has_warning_or_error = if is_pdf {
        let track_count = app.project.tracks.len();
        let organizer_tracks = convert_project_tracks(&app.project.tracks);
        let exporter = crate::services::PdfCatalogExporter::new();
        let line_lengths: Vec<usize> = organizer_tracks
            .iter()
            .enumerate()
            .map(|(idx, track)| {
                exporter
                    .format_track_line(idx + 1, track, &app.export_catalog_dialog.config.fields)
                    .chars()
                    .count()
            })
            .collect();
        let layout = app
            .export_catalog_dialog
            .compute_layout_with_lengths(track_count, Some(&line_lengths));
        layout.readability_warning || app.export_catalog_dialog.error_message.is_some()
    } else {
        app.export_catalog_dialog.error_message.is_some()
    };

    let dialog_size = if is_pdf {
        if has_warning_or_error {
            egui::vec2(360.0, 515.0)
        } else {
            egui::vec2(360.0, 485.0)
        }
    } else {
        egui::vec2(360.0, 285.0)
    };

    let parent_rect = ctx.input(|i| i.viewport().outer_rect).unwrap_or_else(|| {
        egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(960.0, 680.0))
    });
    let pos = parent_rect.center() - dialog_size / 2.0;

    let viewport_id = egui::ViewportId::from_hash_of("export_catalog_viewport");
    ctx.show_viewport_immediate(
        viewport_id,
        egui::ViewportBuilder::default()
            .with_title("Export Catalog")
            .with_inner_size(dialog_size)
            .with_position(pos)
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false),
        |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested()) {
                app.export_catalog_dialog.close();
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(dialog_size));
            render_export_catalog_contents(app, ctx);
            if !app.export_catalog_dialog.is_open {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        },
    );
}

pub fn render_export_catalog_contents(app: &mut MusicBoxApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.label(format!(
                    "Playlist: {} ({} tracks)",
                    app.project.metadata.name,
                    app.project.tracks.len()
                ));
                ui.add_space(6.0);

                // 1. Format Selection
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.label("Format:");
                    ui.horizontal_wrapped(|ui| {
                        ui.radio_value(
                            &mut app.export_catalog_dialog.config.format,
                            CatalogFormat::Pdf,
                            "PDF",
                        );
                        ui.radio_value(
                            &mut app.export_catalog_dialog.config.format,
                            CatalogFormat::Txt,
                            "Plain Text",
                        );
                        ui.radio_value(
                            &mut app.export_catalog_dialog.config.format,
                            CatalogFormat::Rtf,
                            "Rich Text",
                        );
                    });
                });

                ui.add_space(4.0);

                // 2. Universal Field Toggles
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.label("Fields to Include:");
                    ui.checkbox(
                        &mut app.export_catalog_dialog.config.fields.track_number,
                        "Track Number (#)",
                    );
                    ui.checkbox(&mut app.export_catalog_dialog.config.fields.title, "Title");
                    ui.checkbox(
                        &mut app.export_catalog_dialog.config.fields.artist,
                        "Artist",
                    );
                    ui.checkbox(
                        &mut app.export_catalog_dialog.config.fields.export_filename,
                        "Export Filename (XXYY-filename.mp3)",
                    );
                    ui.checkbox(
                        &mut app.export_catalog_dialog.config.fields.header_title,
                        "Header Title",
                    );
                });

                ui.add_space(4.0);

                // 3. PDF Page Sizing & Themes (Only for PDF format)
                if app.export_catalog_dialog.config.format == CatalogFormat::Pdf {
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label("Page Sizing & Layout:");

                        // Preset Dropdown
                        let mut selected_preset =
                            app.export_catalog_dialog.config.dimensions.preset;
                        egui::ComboBox::from_label("Preset")
                            .selected_text(selected_preset.display_name())
                            .show_ui(ui, |ui| {
                                for preset in CatalogPagePreset::all() {
                                    ui.selectable_value(
                                        &mut selected_preset,
                                        *preset,
                                        preset.display_name(),
                                    );
                                }
                            });

                        if selected_preset != app.export_catalog_dialog.config.dimensions.preset {
                            app.export_catalog_dialog.config.dimensions.preset = selected_preset;
                            if selected_preset != CatalogPagePreset::Custom {
                                let (w_cm, h_cm) = selected_preset.dimensions_cm();
                                let unit = app.export_catalog_dialog.config.dimensions.unit;
                                app.export_catalog_dialog.config.dimensions.width =
                                    unit.from_points(LengthUnit::Cm.to_points(w_cm));
                                app.export_catalog_dialog.config.dimensions.height =
                                    unit.from_points(LengthUnit::Cm.to_points(h_cm));
                                app.export_catalog_dialog
                                    .config
                                    .dimensions
                                    .clamp_dimensions();
                            }
                        }

                        // Unit Selector
                        let mut selected_unit = app.export_catalog_dialog.config.dimensions.unit;
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Unit:");
                            for unit in LengthUnit::all() {
                                ui.radio_value(&mut selected_unit, *unit, unit.symbol());
                            }
                        });
                        if selected_unit != app.export_catalog_dialog.config.dimensions.unit {
                            app.export_catalog_dialog
                                .config
                                .dimensions
                                .set_unit(selected_unit);
                        }

                        // Width & Height numeric controls
                        let prev_w = app.export_catalog_dialog.config.dimensions.width;
                        let prev_h = app.export_catalog_dialog.config.dimensions.height;
                        let unit_str = app.export_catalog_dialog.config.dimensions.unit.symbol();

                        ui.horizontal_wrapped(|ui| {
                            ui.label("Width:");
                            ui.add(
                                egui::DragValue::new(
                                    &mut app.export_catalog_dialog.config.dimensions.width,
                                )
                                .speed(0.1)
                                .suffix(format!(" {}", unit_str)),
                            );
                            ui.label("Height:");
                            ui.add(
                                egui::DragValue::new(
                                    &mut app.export_catalog_dialog.config.dimensions.height,
                                )
                                .speed(0.1)
                                .suffix(format!(" {}", unit_str)),
                            );
                        });

                        if (app.export_catalog_dialog.config.dimensions.width - prev_w).abs()
                            > 0.001
                            || (app.export_catalog_dialog.config.dimensions.height - prev_h).abs()
                                > 0.001
                        {
                            app.export_catalog_dialog.config.dimensions.preset =
                                CatalogPagePreset::Custom;
                        }
                        app.export_catalog_dialog
                            .config
                            .dimensions
                            .clamp_dimensions();

                        // Margin control
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Margin:");
                            ui.add(
                                egui::DragValue::new(
                                    &mut app.export_catalog_dialog.config.dimensions.margin,
                                )
                                .speed(0.05)
                                .suffix(format!(" {}", unit_str)),
                            );
                        });
                        app.export_catalog_dialog
                            .config
                            .dimensions
                            .clamp_dimensions();

                        // Cut guidelines toggle
                        ui.checkbox(
                            &mut app.export_catalog_dialog.config.cut_border,
                            "Cut Guidelines / Border",
                        );

                        // Theme selector (T022)
                        let mut selected_theme =
                            app.export_catalog_dialog.config.theme_name.clone();
                        egui::ComboBox::from_label("Theme")
                            .selected_text(&selected_theme)
                            .show_ui(ui, |ui| {
                                for theme_name in CatalogThemeRegistry::available_themes() {
                                    ui.selectable_value(
                                        &mut selected_theme,
                                        theme_name.to_string(),
                                        theme_name,
                                    );
                                }
                            });
                        if selected_theme != app.export_catalog_dialog.config.theme_name {
                            app.export_catalog_dialog.config.theme_name = selected_theme;
                        }
                    });

                    ui.add_space(4.0);

                    // 4. Live Layout Summary & Warning (T019)
                    let track_count = app.project.tracks.len();
                    let organizer_tracks = convert_project_tracks(&app.project.tracks);
                    let exporter = crate::services::PdfCatalogExporter::new();
                    let line_lengths: Vec<usize> = organizer_tracks
                        .iter()
                        .enumerate()
                        .map(|(idx, track)| {
                            exporter
                                .format_track_line(
                                    idx + 1,
                                    track,
                                    &app.export_catalog_dialog.config.fields,
                                )
                                .chars()
                                .count()
                        })
                        .collect();
                    let layout = app
                        .export_catalog_dialog
                        .compute_layout_with_lengths(track_count, Some(&line_lengths));
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label("Live Layout Summary:");
                        ui.label(format!(
                            "Columns: {}  |  Font Size: {:.1} pt  |  Sheet: {:.1} × {:.1} cm",
                            layout.column_count,
                            layout.computed_font_size_pt,
                            layout.physical_width_cm,
                            layout.physical_height_cm
                        ));

                        if layout.readability_warning {
                            ui.add_space(2.0);
                            ui.colored_label(
                                egui::Color32::RED,
                                "⚠ Warning: Calculated font size is < 5.0 pt. Text may be illegible when printed.",
                            );
                        }
                    });
                }

                // Error message display
                if let Some(err) = &app.export_catalog_dialog.error_message {
                    ui.add_space(4.0);
                    ui.colored_label(egui::Color32::RED, err);
                }

                ui.add_space(6.0);

                // Action Buttons
                ui.horizontal(|ui| {
                    let has_tracks = !app.project.tracks.is_empty();
                    let has_fields = app.export_catalog_dialog.config.fields.is_valid();
                    let can_export = has_tracks && has_fields;

                    let export_btn = ui.add_enabled(can_export, egui::Button::new("Export..."));
                    if export_btn.clicked() {
                        trigger_catalog_export(app);
                    }

                    if ui.button("Cancel").clicked() {
                        app.export_catalog_dialog.close();
                    }
                });
                ui.add_space(4.0);
            });
    });
}

pub fn convert_project_tracks(tracks: &[ProjectTrack]) -> Vec<OrganizerTrack> {
    tracks
        .iter()
        .enumerate()
        .map(|(i, pt)| {
            let path = PathBuf::from(&pt.file_path);
            let filename = path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| format!("track_{}.mp3", i + 1));
            OrganizerTrack {
                index: (i + 1) as u32,
                file_path: path,
                original_filename: filename,
                xxyy_prefix: track_index_to_xxyy((i + 1) as u32)
                    .unwrap_or_else(|_| format!("{:04}", i + 1)),
                metadata: crate::models::TrackMetadata {
                    track_number: Some((i + 1) as u32),
                    title: pt.title.clone(),
                    artist: pt.artist.clone(),
                    album: String::new(),
                    year: None,
                    duration_secs: 0,
                },
                is_modified: false,
            }
        })
        .collect()
}

fn trigger_catalog_export(app: &mut MusicBoxApp) {
    let ext = app.export_catalog_dialog.config.format.extension();
    let filter_name = app.export_catalog_dialog.config.format.display_name();
    let default_name = format!("catalog.{}", ext);
    let start_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    if let Some(dest_path) = rfd::FileDialog::new()
        .set_directory(start_dir)
        .set_file_name(&default_name)
        .add_filter(filter_name, &[ext])
        .save_file()
    {
        let organizer_tracks = convert_project_tracks(&app.project.tracks);
        let service = CatalogExportService::new();
        let title = if app.export_catalog_dialog.config.fields.header_title {
            Some(app.project.metadata.name.as_str())
        } else {
            None
        };

        match service.export_catalog(
            &dest_path,
            &organizer_tracks,
            title,
            &app.export_catalog_dialog.config,
        ) {
            Ok(()) => {
                app.status_message = format!("Exported catalog to {}", dest_path.display());
                app.export_catalog_dialog.close();
            }
            Err(e) => {
                app.export_catalog_dialog.error_message = Some(format!("Export error: {}", e));
            }
        }
    }
}
