use crate::models::ProjectTrack;
use crate::ui::app::{MusicBoxApp, TableColumn};
use std::path::Path;

pub fn get_export_filename(track: &ProjectTrack) -> String {
    let path = Path::new(&track.file_path);
    let original_filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("track.mp3");
    crate::services::naming::format_export_filename(track.index, original_filename)
        .unwrap_or_else(|_| format!("{:04}-{}", track.index + 1, original_filename))
}

pub fn render_playlist_table(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    if app.project.tracks.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label("🎵 Playlist is empty");
            ui.label("Click 'Scan Folder...' or 'Add MP3' on the toolbar to add tracks.");
        });
        return;
    }

    let mut metadata_modified_idx = None;
    let mut new_title_val = None;
    let mut new_artist_val = None;

    let total_width = ui.available_width().max(600.0);
    let w_num = 32.0;
    let rem = (total_width - w_num - 24.0).max(400.0);
    let w_export = (rem * 0.22).round();
    let w_title = (rem * 0.28).round();
    let w_artist = (rem * 0.12).round();
    let w_source = (rem - w_export - w_title - w_artist).max(80.0);

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let row_height = 24.0;
            let num_tracks = app.project.tracks.len();

            // Render table header
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    [w_num, row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.add(egui::Label::new(egui::RichText::new("#").strong()).truncate());
                    },
                );
                ui.allocate_ui_with_layout(
                    [w_export, row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.add(
                            egui::Label::new(egui::RichText::new("Export Filename").strong())
                                .truncate(),
                        );
                    },
                );
                ui.allocate_ui_with_layout(
                    [w_title, row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.add(egui::Label::new(egui::RichText::new("Title").strong()).truncate());
                    },
                );
                ui.allocate_ui_with_layout(
                    [w_artist, row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.add(egui::Label::new(egui::RichText::new("Artist").strong()).truncate());
                    },
                );
                ui.allocate_ui_with_layout(
                    [w_source, row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.add(
                            egui::Label::new(egui::RichText::new("Source File").strong())
                                .truncate(),
                        );
                    },
                );
            });
            ui.separator();

            for row_idx in 0..num_tracks {
                let is_selected = app.selected_track_indices.contains(&row_idx);
                let track = &app.project.tracks[row_idx];
                let export_filename = get_export_filename(track);

                ui.horizontal(|ui| {
                    // Track Number column
                    let row_label = format!("{:02}", row_idx + 1);
                    let num_resp = ui
                        .allocate_ui_with_layout(
                            [w_num, row_height].into(),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| ui.selectable_label(is_selected, row_label),
                        )
                        .inner;

                    if num_resp.clicked() {
                        if ui.input(|i| i.modifiers.ctrl || i.modifiers.command) {
                            if is_selected {
                                app.selected_track_indices.remove(&row_idx);
                            } else {
                                app.selected_track_indices.insert(row_idx);
                            }
                        } else if ui.input(|i| i.modifiers.shift) {
                            if let Some(&first_sel) = app.selected_track_indices.iter().next() {
                                let start = first_sel.min(row_idx);
                                let end = first_sel.max(row_idx);
                                for i in start..=end {
                                    app.selected_track_indices.insert(i);
                                }
                            } else {
                                app.selected_track_indices.insert(row_idx);
                            }
                        } else {
                            app.selected_track_indices.clear();
                            app.selected_track_indices.insert(row_idx);
                        }
                    }

                    // Export Filename column (left-aligned, truncated)
                    ui.allocate_ui_with_layout(
                        [w_export, row_height].into(),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Label::new(&export_filename).truncate());
                        },
                    );

                    // Inline-editable Title column (left-aligned, truncated)
                    let is_editing_title = app.editing_cell == Some((row_idx, TableColumn::Title));
                    let mut current_title = track.title.clone();
                    ui.allocate_ui_with_layout(
                        [w_title, row_height].into(),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            if is_editing_title {
                                let text_edit = egui::TextEdit::singleline(&mut current_title)
                                    .desired_width(w_title);
                                let text_resp = ui.add(text_edit);
                                if text_resp.lost_focus() {
                                    app.editing_cell = None;
                                    metadata_modified_idx = Some(row_idx);
                                    new_title_val = Some(current_title);
                                }
                            } else {
                                let display_title = if current_title.is_empty() {
                                    "-"
                                } else {
                                    &current_title
                                };
                                let label_resp = ui.add(
                                    egui::Label::new(display_title)
                                        .truncate()
                                        .sense(egui::Sense::click()),
                                );
                                if label_resp.double_clicked() {
                                    app.editing_cell = Some((row_idx, TableColumn::Title));
                                } else if label_resp.clicked() {
                                    if ui.input(|i| i.modifiers.ctrl || i.modifiers.command) {
                                        if is_selected {
                                            app.selected_track_indices.remove(&row_idx);
                                        } else {
                                            app.selected_track_indices.insert(row_idx);
                                        }
                                    } else {
                                        app.selected_track_indices.clear();
                                        app.selected_track_indices.insert(row_idx);
                                    }
                                }
                            }
                        },
                    );

                    // Inline-editable Artist column (left-aligned, truncated)
                    let is_editing_artist =
                        app.editing_cell == Some((row_idx, TableColumn::Artist));
                    let mut current_artist = track.artist.clone();
                    ui.allocate_ui_with_layout(
                        [w_artist, row_height].into(),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            if is_editing_artist {
                                let text_edit = egui::TextEdit::singleline(&mut current_artist)
                                    .desired_width(w_artist);
                                let text_resp = ui.add(text_edit);
                                if text_resp.lost_focus() {
                                    app.editing_cell = None;
                                    metadata_modified_idx = Some(row_idx);
                                    new_artist_val = Some(current_artist);
                                }
                            } else {
                                let display_artist = if current_artist.is_empty() {
                                    "-"
                                } else {
                                    &current_artist
                                };
                                let label_resp = ui.add(
                                    egui::Label::new(display_artist)
                                        .truncate()
                                        .sense(egui::Sense::click()),
                                );
                                if label_resp.double_clicked() {
                                    app.editing_cell = Some((row_idx, TableColumn::Artist));
                                } else if label_resp.clicked() {
                                    if ui.input(|i| i.modifiers.ctrl || i.modifiers.command) {
                                        if is_selected {
                                            app.selected_track_indices.remove(&row_idx);
                                        } else {
                                            app.selected_track_indices.insert(row_idx);
                                        }
                                    } else {
                                        app.selected_track_indices.clear();
                                        app.selected_track_indices.insert(row_idx);
                                    }
                                }
                            }
                        },
                    );

                    // Source File column (left-aligned, truncated)
                    ui.allocate_ui_with_layout(
                        [w_source, row_height].into(),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Label::new(&track.file_path).truncate());
                        },
                    );
                });
            }
        });

    // Apply any inline metadata edit changes
    if let Some(idx) = metadata_modified_idx {
        app.session.push_undo(app.project.clone());
        if let Some(t) = app.project.tracks.get_mut(idx) {
            if let Some(val) = new_title_val {
                t.title = val;
            }
            if let Some(val) = new_artist_val {
                t.artist = val;
            }
        }
        app.status_message = format!("Updated metadata for track {}", idx + 1);
    }
}
