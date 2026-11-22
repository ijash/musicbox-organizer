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

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let row_height = 24.0;
            let available_width = ui.available_width();
            let num_tracks = app.project.tracks.len();

            // Render table header
            ui.horizontal(|ui| {
                ui.add_sized(
                    [40.0, row_height],
                    egui::Label::new(egui::RichText::new("#").strong()),
                );
                ui.allocate_ui_with_layout(
                    [180.0, row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(egui::RichText::new("Export Filename").strong());
                    },
                );
                ui.allocate_ui_with_layout(
                    [220.0, row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(egui::RichText::new("Title").strong());
                    },
                );
                ui.allocate_ui_with_layout(
                    [180.0, row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(egui::RichText::new("Artist").strong());
                    },
                );
                ui.allocate_ui_with_layout(
                    [(available_width - 640.0).max(100.0), row_height].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(egui::RichText::new("Source File").strong());
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
                    let num_resp = ui.add_sized(
                        [40.0, row_height],
                        egui::SelectableLabel::new(is_selected, row_label),
                    );

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

                    // Export Filename column (left-aligned)
                    ui.allocate_ui_with_layout(
                        [180.0, row_height].into(),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Label::new(&export_filename).truncate());
                        },
                    );

                    // Inline-editable Title column (left-aligned)
                    let is_editing_title = app.editing_cell == Some((row_idx, TableColumn::Title));
                    let mut current_title = track.title.clone();
                    ui.allocate_ui_with_layout(
                        [220.0, row_height].into(),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            if is_editing_title {
                                let text_edit = egui::TextEdit::singleline(&mut current_title)
                                    .desired_width(220.0);
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
                                let label_resp = ui.selectable_label(false, display_title);
                                if label_resp.double_clicked() {
                                    app.editing_cell = Some((row_idx, TableColumn::Title));
                                }
                            }
                        },
                    );

                    // Inline-editable Artist column (left-aligned)
                    let is_editing_artist =
                        app.editing_cell == Some((row_idx, TableColumn::Artist));
                    let mut current_artist = track.artist.clone();
                    ui.allocate_ui_with_layout(
                        [180.0, row_height].into(),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            if is_editing_artist {
                                let text_edit = egui::TextEdit::singleline(&mut current_artist)
                                    .desired_width(180.0);
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
                                let label_resp = ui.selectable_label(false, display_artist);
                                if label_resp.double_clicked() {
                                    app.editing_cell = Some((row_idx, TableColumn::Artist));
                                }
                            }
                        },
                    );

                    // Source File column (left-aligned)
                    ui.allocate_ui_with_layout(
                        [(available_width - 640.0).max(100.0), row_height].into(),
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
