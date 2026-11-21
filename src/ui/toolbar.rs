use crate::models::{ProjectFile, ProjectTrack};
use crate::services::{DirectoryScanner, Id3Service};
use crate::ui::app::MusicBoxApp;
use std::path::PathBuf;

pub fn render_toolbar(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        if ui.button("📁 Scan Folder...").clicked() {
            scan_folder_dialog(app);
        }

        if ui.button("➕ Add MP3").clicked() {
            add_mp3_dialog(app);
        }

        ui.add_space(8.0);
        let has_selection = !app.selected_track_indices.is_empty();

        if ui.add_enabled(has_selection, egui::Button::new("🗑 Remove")).clicked() {
            remove_selected_tracks(app);
        }

        if ui.add_enabled(can_move_up(app), egui::Button::new("⬆ Up")).clicked() {
            move_selected_up(app);
        }

        if ui.add_enabled(can_move_down(app), egui::Button::new("⬇ Down")).clicked() {
            move_selected_down(app);
        }

        ui.add_space(8.0);
        if ui.button("⚙ Audio Tools...").clicked() {
            app.dsp_dialog.is_open = true;
        }

        if ui.button("📄 Export Text/RTF...").clicked() {
            export_text_dialog(app);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("💾 Save").clicked() {
                save_project(app);
            }

            if ui.button("📂 Open").clicked() {
                open_project_dialog(app);
            }
        });
    });
}

fn scan_folder_dialog(app: &mut MusicBoxApp) {
    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
        let scanner = DirectoryScanner::new();
        match scanner.scan_directory(&folder) {
            Ok(found) => {
                if found.is_empty() {
                    app.status_message = format!("No MP3 files found in {}", folder.display());
                } else {
                    app.session.push_undo(app.project.clone());
                    app.project.add_scanned_tracks(found);
                    app.status_message = format!("Added tracks from {}", folder.display());
                }
            }
            Err(e) => {
                app.status_message = format!("Scan error: {}", e);
            }
        }
    }
}

fn add_mp3_dialog(app: &mut MusicBoxApp) {
    if let Some(files) = rfd::FileDialog::new().add_filter("MP3 Audio", &["mp3"]).pick_files()
        && !files.is_empty()
    {
        app.session.push_undo(app.project.clone());
        let id3_svc = Id3Service::new();
        let start_count = app.project.tracks.len();
        for path in files {
            let meta = id3_svc.read_tags(&path).unwrap_or_default();
            let next_idx = app.project.tracks.len() as u32;
            app.project.tracks.push(ProjectTrack {
                index: next_idx,
                title: meta.title,
                artist: meta.artist,
                file_path: path.to_string_lossy().to_string(),
            });
        }
        app.status_message = format!("Added {} track(s)", app.project.tracks.len() - start_count);
    }
}

fn remove_selected_tracks(app: &mut MusicBoxApp) {
    if app.selected_track_indices.is_empty() {
        return;
    }
    app.session.push_undo(app.project.clone());
    let mut new_tracks = Vec::new();
    for (i, track) in app.project.tracks.drain(..).enumerate() {
        if !app.selected_track_indices.contains(&i) {
            new_tracks.push(track);
        }
    }
    app.project.tracks = new_tracks;
    reindex_tracks(&mut app.project.tracks);
    app.selected_track_indices.clear();
    app.editing_cell = None;
    app.status_message = "Selected track(s) removed".to_string();
}

fn can_move_up(app: &MusicBoxApp) -> bool {
    if app.selected_track_indices.is_empty() {
        return false;
    }
    !app.selected_track_indices.contains(&0)
}

fn can_move_down(app: &MusicBoxApp) -> bool {
    if app.selected_track_indices.is_empty() {
        return false;
    }
    let last_idx = app.project.tracks.len().saturating_sub(1);
    !app.selected_track_indices.contains(&last_idx)
}

fn move_selected_up(app: &mut MusicBoxApp) {
    if !can_move_up(app) {
        return;
    }
    app.session.push_undo(app.project.clone());
    let selected: Vec<usize> = app.selected_track_indices.iter().copied().collect();
    for idx in selected {
        app.project.tracks.swap(idx, idx - 1);
    }
    let mut new_selection = std::collections::BTreeSet::new();
    for idx in &app.selected_track_indices {
        new_selection.insert(idx - 1);
    }
    app.selected_track_indices = new_selection;
    reindex_tracks(&mut app.project.tracks);
    app.status_message = "Moved up".to_string();
}

fn move_selected_down(app: &mut MusicBoxApp) {
    if !can_move_down(app) {
        return;
    }
    app.session.push_undo(app.project.clone());
    let selected: Vec<usize> = app.selected_track_indices.iter().rev().copied().collect();
    for idx in selected {
        app.project.tracks.swap(idx, idx + 1);
    }
    let mut new_selection = std::collections::BTreeSet::new();
    for idx in &app.selected_track_indices {
        new_selection.insert(idx + 1);
    }
    app.selected_track_indices = new_selection;
    reindex_tracks(&mut app.project.tracks);
    app.status_message = "Moved down".to_string();
}

fn reindex_tracks(tracks: &mut [ProjectTrack]) {
    for (i, t) in tracks.iter_mut().enumerate() {
        t.index = i as u32;
    }
}

fn open_project_dialog(app: &mut MusicBoxApp) {
    if let Some(file) = rfd::FileDialog::new().add_filter("Playlist Project", &["txt", "json"]).pick_file() {
        match ProjectFile::load(&file) {
            Ok(proj) => {
                app.session.push_undo(app.project.clone());
                app.project = proj;
                app.project_path = Some(file.clone());
                app.selected_track_indices.clear();
                app.editing_cell = None;
                app.status_message = format!("Loaded project from {}", file.display());
            }
            Err(e) => {
                app.status_message = format!("Open error: {}", e);
            }
        }
    }
}

fn save_project(app: &mut MusicBoxApp) {
    let path = app.project_path.clone().unwrap_or_else(|| PathBuf::from("playlist.txt"));
    match app.project.save(&path) {
        Ok(()) => {
            app.project_path = Some(path.clone());
            app.status_message = format!("Project saved to {}", path.display());
        }
        Err(e) => {
            app.status_message = format!("Save error: {}", e);
        }
    }
}

fn export_text_dialog(app: &mut MusicBoxApp) {
    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
        let service = crate::services::ExportConfigService::new();
        let selection = crate::models::ExportFormatSelection {
            txt: true,
            rtf: true,
        };
        match service.export_playlist(&folder, &selection, &app.project) {
            Ok(files) => {
                app.status_message = format!("Exported {} text/RTF playlist file(s) to {}", files.len(), folder.display());
            }
            Err(e) => {
                app.status_message = format!("Text export error: {}", e);
            }
        }
    }
}
