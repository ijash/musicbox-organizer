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
    crate::services::naming::format_export_filename_from_title(
        track.index,
        &track.title,
        original_filename,
    )
    .unwrap_or_else(|_| format!("{:04}-{}", track.index + 1, original_filename))
}

pub fn commit_editing_cell(app: &mut MusicBoxApp) {
    if let Some((row_idx, col)) = app.editing_cell.take() {
        app.session.push_undo(app.project.clone());
        if let Some(track) = app.project.tracks.get_mut(row_idx) {
            match col {
                TableColumn::Title => track.title = app.edit_buffer.clone(),
                TableColumn::Artist => track.artist = app.edit_buffer.clone(),
                _ => {}
            }
            app.status_message = format!("Updated metadata for track {}", row_idx + 1);
        }
    }
}

fn handle_row_selection(app: &mut MusicBoxApp, row_idx: usize, is_selected: bool, ui: &egui::Ui) {
    commit_editing_cell(app);
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

pub fn render_playlist_table(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    if app.project.tracks.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label("🎵 Playlist is empty");
            ui.label("Click 'Scan Folder...' or 'Add MP3' on the toolbar to add tracks.");
        });
        return;
    }

    let num_tracks = app.project.tracks.len();

    egui::ScrollArea::horizontal()
        .auto_shrink([false, false])
        .show(ui, |ui| {
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
                        row.set_selected(is_selected);

                        let current_title = app.project.tracks[row_idx].title.clone();
                        let current_artist = app.project.tracks[row_idx].artist.clone();
                        let file_path = app.project.tracks[row_idx].file_path.clone();
                        let export_filename = get_export_filename(&app.project.tracks[row_idx]);

                        // Col 1: #
                        row.col(|ui| {
                            let row_label = format!("{:02}", row_idx + 1);
                            let label_resp = ui.add(
                                egui::Label::new(row_label)
                                    .sense(egui::Sense::click())
                                    .truncate(),
                            );
                            let cell_resp = ui.interact(
                                ui.max_rect(),
                                ui.id().with(("cell", row_idx, 0)),
                                egui::Sense::click(),
                            );
                            if label_resp.clicked() || cell_resp.clicked() {
                                handle_row_selection(app, row_idx, is_selected, ui);
                            }
                        });

                        // Col 2: Export Filename
                        row.col(|ui| {
                            let label_resp = ui.add(
                                egui::Label::new(&export_filename)
                                    .sense(egui::Sense::click())
                                    .truncate(),
                            );
                            let cell_resp = ui.interact(
                                ui.max_rect(),
                                ui.id().with(("cell", row_idx, 1)),
                                egui::Sense::click(),
                            );
                            if label_resp.clicked() || cell_resp.clicked() {
                                handle_row_selection(app, row_idx, is_selected, ui);
                            }
                        });

                        // Col 3: Title
                        let is_editing_title =
                            app.editing_cell == Some((row_idx, TableColumn::Title));
                        row.col(|ui| {
                            if is_editing_title {
                                let text_edit = egui::TextEdit::singleline(&mut app.edit_buffer)
                                    .desired_width(ui.available_width());
                                let text_resp = ui.add(text_edit);
                                if !text_resp.has_focus()
                                    && !ui.input(|i| i.key_pressed(egui::Key::Escape))
                                {
                                    text_resp.request_focus();
                                }

                                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                    app.editing_cell = None;
                                } else if text_resp.lost_focus()
                                    || ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    commit_editing_cell(app);
                                }
                            } else {
                                let display_title = if current_title.is_empty() {
                                    "-"
                                } else {
                                    &current_title
                                };
                                let label_resp = ui.add(
                                    egui::Label::new(display_title)
                                        .sense(egui::Sense::click())
                                        .truncate(),
                                );
                                let cell_resp = ui.interact(
                                    ui.max_rect(),
                                    ui.id().with(("cell", row_idx, 2)),
                                    egui::Sense::click(),
                                );
                                if label_resp.clicked() || cell_resp.clicked() {
                                    commit_editing_cell(app);
                                    handle_row_selection(app, row_idx, is_selected, ui);
                                    app.editing_cell = Some((row_idx, TableColumn::Title));
                                    app.edit_buffer = current_title.clone();
                                }
                            }
                        });

                        // Col 4: Artist
                        let is_editing_artist =
                            app.editing_cell == Some((row_idx, TableColumn::Artist));
                        row.col(|ui| {
                            if is_editing_artist {
                                let text_edit = egui::TextEdit::singleline(&mut app.edit_buffer)
                                    .desired_width(ui.available_width());
                                let text_resp = ui.add(text_edit);
                                if !text_resp.has_focus()
                                    && !ui.input(|i| i.key_pressed(egui::Key::Escape))
                                {
                                    text_resp.request_focus();
                                }

                                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                    app.editing_cell = None;
                                } else if text_resp.lost_focus()
                                    || ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    commit_editing_cell(app);
                                }
                            } else {
                                let display_artist = if current_artist.is_empty() {
                                    "-"
                                } else {
                                    &current_artist
                                };
                                let label_resp = ui.add(
                                    egui::Label::new(display_artist)
                                        .sense(egui::Sense::click())
                                        .truncate(),
                                );
                                let cell_resp = ui.interact(
                                    ui.max_rect(),
                                    ui.id().with(("cell", row_idx, 3)),
                                    egui::Sense::click(),
                                );
                                if label_resp.clicked() || cell_resp.clicked() {
                                    commit_editing_cell(app);
                                    handle_row_selection(app, row_idx, is_selected, ui);
                                    app.editing_cell = Some((row_idx, TableColumn::Artist));
                                    app.edit_buffer = current_artist.clone();
                                }
                            }
                        });

                        // Col 5: Source File
                        row.col(|ui| {
                            let label_resp = ui.add(
                                egui::Label::new(&file_path)
                                    .sense(egui::Sense::click())
                                    .truncate(),
                            );
                            let cell_resp = ui.interact(
                                ui.max_rect(),
                                ui.id().with(("cell", row_idx, 4)),
                                egui::Sense::click(),
                            );
                            if label_resp.clicked() || cell_resp.clicked() {
                                handle_row_selection(app, row_idx, is_selected, ui);
                            }
                        });
                    });
                });
        });
}
