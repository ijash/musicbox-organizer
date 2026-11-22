use crate::models::ProjectTrack;
use crate::ui::app::{MusicBoxApp, TableColumn};
use egui_extras::{Column, TableBuilder};
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

    let num_tracks = app.project.tracks.len();

    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .vscroll(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::exact(32.0))
        .column(Column::initial(190.0).at_least(100.0).clip(true))
        .column(Column::initial(180.0).at_least(100.0).clip(true))
        .column(Column::initial(95.0).at_least(60.0).clip(true))
        .column(Column::remainder().at_least(120.0).clip(true))
        .header(22.0, |mut header| {
            header.col(|ui| {
                ui.strong("#");
            });
            header.col(|ui| {
                ui.strong("Export Filename");
            });
            header.col(|ui| {
                ui.strong("Title");
            });
            header.col(|ui| {
                ui.strong("Artist");
            });
            header.col(|ui| {
                ui.strong("Source File");
            });
        })
        .body(|body| {
            body.rows(22.0, num_tracks, |mut row| {
                let row_idx = row.index();
                let is_selected = app.selected_track_indices.contains(&row_idx);
                let track = &app.project.tracks[row_idx];
                let export_filename = get_export_filename(track);

                // Col 1: #
                row.col(|ui| {
                    let row_label = format!("{:02}", row_idx + 1);
                    let resp = ui.selectable_label(is_selected, row_label);
                    if resp.clicked() {
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
                });

                // Col 2: Export Filename
                row.col(|ui| {
                    ui.label(&export_filename);
                });

                // Col 3: Title
                let is_editing_title = app.editing_cell == Some((row_idx, TableColumn::Title));
                let mut current_title = track.title.clone();
                row.col(|ui| {
                    if is_editing_title {
                        let text_edit = egui::TextEdit::singleline(&mut current_title);
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
                        let label_resp =
                            ui.add(egui::Label::new(display_title).sense(egui::Sense::click()));
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
                });

                // Col 4: Artist
                let is_editing_artist = app.editing_cell == Some((row_idx, TableColumn::Artist));
                let mut current_artist = track.artist.clone();
                row.col(|ui| {
                    if is_editing_artist {
                        let text_edit = egui::TextEdit::singleline(&mut current_artist);
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
                        let label_resp =
                            ui.add(egui::Label::new(display_artist).sense(egui::Sense::click()));
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
                });

                // Col 5: Source File
                row.col(|ui| {
                    ui.label(&track.file_path);
                });
            });
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
