use crate::services::NativeAudioProcessor;
use crate::ui::DspProgressMessage;
use crate::ui::app::MusicBoxApp;
use std::sync::mpsc;
use std::thread;

/// cite: data-model.md: DspDialogState.normalize_target_db, data-model.md: DspDialogState.trim_threshold_db
#[derive(Debug, Clone)]
pub struct DspDialogState {
    pub is_open: bool,
    pub normalize_enabled: bool,
    pub normalize_target_db: f32, // Default: -14.0
    pub trim_enabled: bool,
    pub trim_threshold_db: f32, // Default: -45.0
    pub is_processing: bool,
    pub progress_percent: f32,
    pub progress_text: String,
    pub error_summary: Option<String>,
}

impl Default for DspDialogState {
    fn default() -> Self {
        Self {
            is_open: false,
            normalize_enabled: true,
            normalize_target_db: -14.0,
            trim_enabled: true,
            trim_threshold_db: -45.0,
            is_processing: false,
            progress_percent: 0.0,
            progress_text: String::new(),
            error_summary: None,
        }
    }
}

pub fn render_dsp_dialog(app: &mut MusicBoxApp, ctx: &egui::Context) {
    if !app.dsp_dialog.is_open {
        return;
    }

    let dialog_size = if app.dsp_dialog.is_processing || app.dsp_dialog.error_summary.is_some() {
        egui::vec2(360.0, 320.0)
    } else {
        egui::vec2(360.0, 270.0)
    };

    let parent_rect = ctx.input(|i| i.viewport().outer_rect).unwrap_or_else(|| {
        egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(960.0, 680.0))
    });
    let pos = parent_rect.center() - dialog_size / 2.0;

    let viewport_id = egui::ViewportId::from_hash_of("dsp_dialog_viewport");
    ctx.show_viewport_immediate(
        viewport_id,
        egui::ViewportBuilder::default()
            .with_title("Audio Tools - Batch DSP")
            .with_inner_size(dialog_size)
            .with_position(pos)
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false),
        |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested()) {
                app.dsp_dialog.is_open = false;
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(dialog_size));
            render_dsp_dialog_contents(app, ctx);
            if !app.dsp_dialog.is_open {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        },
    );
}

pub fn render_dsp_dialog_contents(app: &mut MusicBoxApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Audio DSP Processing");
        ui.add_space(8.0);

        let selected_count = if app.selected_track_indices.is_empty() {
            app.project.tracks.len()
        } else {
            app.selected_track_indices.len()
        };

        ui.label(format!("Target: {} track(s) selected", selected_count));
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.checkbox(
                &mut app.dsp_dialog.normalize_enabled,
                "Normalize Volume (LUFS)",
            );
            if app.dsp_dialog.normalize_enabled {
                ui.horizontal(|ui| {
                    ui.label("Target Loudness (dB LUFS):");
                    ui.add(
                        egui::DragValue::new(&mut app.dsp_dialog.normalize_target_db)
                            .range(-30.0..=0.0)
                            .speed(0.5),
                    );
                });
            }
        });

        ui.add_space(4.0);

        ui.group(|ui| {
            ui.checkbox(
                &mut app.dsp_dialog.trim_enabled,
                "Trim Leading/Trailing Silence",
            );
            if app.dsp_dialog.trim_enabled {
                ui.horizontal(|ui| {
                    ui.label("Silence Threshold (dB):");
                    ui.add(
                        egui::DragValue::new(&mut app.dsp_dialog.trim_threshold_db)
                            .range(-80.0..=-10.0)
                            .speed(1.0),
                    );
                });
            }
        });

        if app.dsp_dialog.is_processing {
            ui.add_space(8.0);
            ui.add(egui::ProgressBar::new(app.dsp_dialog.progress_percent).show_percentage());
            ui.label(&app.dsp_dialog.progress_text);
        }

        if let Some(err) = &app.dsp_dialog.error_summary {
            ui.add_space(4.0);
            ui.colored_label(egui::Color32::RED, err);
        }

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            let apply_btn =
                ui.add_enabled(!app.dsp_dialog.is_processing, egui::Button::new("Apply"));
            if apply_btn.clicked() {
                start_dsp_processing(app);
            }

            if ui.button("Cancel").clicked() {
                app.dsp_dialog.is_open = false;
            }
        });
    });
}

fn start_dsp_processing(app: &mut MusicBoxApp) {
    if !app.dsp_dialog.normalize_enabled && !app.dsp_dialog.trim_enabled {
        app.dsp_dialog.error_summary = Some("Please enable at least one operation.".to_string());
        return;
    }

    // Stop audio player if an edited track is playing
    if let Some(playing_idx) = app.playing_track_idx {
        let is_playing_selected = app.selected_track_indices.is_empty()
            || app.selected_track_indices.contains(&playing_idx);
        if is_playing_selected {
            if let Some(player) = &mut app.preview_player {
                player.stop();
            }
            app.playing_track_idx = None;
        }
    }

    // Save undo state before modifying
    app.session.push_undo(app.project.clone());

    let indices: Vec<usize> = if app.selected_track_indices.is_empty() {
        (0..app.project.tracks.len()).collect()
    } else {
        app.selected_track_indices.iter().copied().collect()
    };

    let tracks_to_process: Vec<(usize, std::path::PathBuf)> = indices
        .into_iter()
        .filter_map(|idx| {
            app.project
                .tracks
                .get(idx)
                .map(|t| (idx, std::path::PathBuf::from(&t.file_path)))
        })
        .collect();

    let normalize_enabled = app.dsp_dialog.normalize_enabled;
    let normalize_target = app.dsp_dialog.normalize_target_db;
    let trim_enabled = app.dsp_dialog.trim_enabled;
    let trim_threshold = app.dsp_dialog.trim_threshold_db;

    let (tx, rx) = mpsc::channel();
    app.dsp_rx = Some(rx);
    app.dsp_dialog.is_processing = true;
    app.dsp_dialog.progress_percent = 0.0;
    app.dsp_dialog.progress_text = "Starting batch DSP...".to_string();
    app.dsp_dialog.error_summary = None;

    thread::spawn(move || {
        let total = tracks_to_process.len();
        let _ = tx.send(DspProgressMessage::Started {
            total_tracks: total,
        });

        let processor = NativeAudioProcessor::new();
        let mut succeeded = 0;
        let mut failed = 0;
        let mut errors = Vec::new();

        for (i, (_idx, path)) in tracks_to_process.into_iter().enumerate() {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("track")
                .to_string();
            let _ = tx.send(DspProgressMessage::Step {
                current: i + 1,
                total,
                filename: filename.clone(),
            });

            // Single pass: trim silence and/or normalize volume in one decode/encode cycle
            match processor.process_track(
                &path,
                trim_enabled,
                trim_threshold,
                normalize_enabled,
                normalize_target,
            ) {
                Ok(_) => {
                    succeeded += 1;
                }
                Err(e) => {
                    errors.push(format!("{}: DSP error: {}", filename, e));
                    failed += 1;
                }
            }
        }

        let _ = tx.send(DspProgressMessage::Finished {
            succeeded,
            failed,
            errors,
        });
    });
}
