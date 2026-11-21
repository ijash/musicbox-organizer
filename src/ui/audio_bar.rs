use crate::services::AudioPreviewPlayer;
use crate::ui::app::MusicBoxApp;

/// cite: data-model.md: MusicBoxApp.preview_player, data-model.md: MusicBoxApp.playing_track_idx
pub fn render_audio_bar(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.strong("Audio Preview:");
        ui.add_space(8.0);

        let has_tracks = !app.project.tracks.is_empty();
        let target_idx = app
            .selected_track_indices
            .iter()
            .next()
            .copied()
            .or(app.playing_track_idx)
            .or(if has_tracks { Some(0) } else { None });

        // Play Button
        let is_playing = app
            .preview_player
            .as_ref()
            .map(|p| p.is_playing())
            .unwrap_or(false);

        let play_btn_enabled = target_idx.is_some()
            && (!is_playing
                || app
                    .preview_player
                    .as_ref()
                    .map(|p| p.is_paused())
                    .unwrap_or(false));
        if ui
            .add_enabled(play_btn_enabled, egui::Button::new("▶ Play"))
            .clicked()
            && let Some(idx) = target_idx
        {
            play_track(app, idx);
        }

        // Pause Button
        let pause_btn_enabled = is_playing
            && !app
                .preview_player
                .as_ref()
                .map(|p| p.is_paused())
                .unwrap_or(false);
        if ui
            .add_enabled(pause_btn_enabled, egui::Button::new("⏸ Pause"))
            .clicked()
            && let Some(player) = &mut app.preview_player
        {
            player.pause();
        }

        // Stop Button
        let stop_btn_enabled = is_playing
            || app
                .preview_player
                .as_ref()
                .map(|p| p.is_paused())
                .unwrap_or(false);
        if ui
            .add_enabled(stop_btn_enabled, egui::Button::new("⏹ Stop"))
            .clicked()
        {
            if let Some(player) = &mut app.preview_player {
                player.stop();
            }
            app.playing_track_idx = None;
        }

        ui.add_space(16.0);
        // Playback Status Label
        if let Some(idx) = app.playing_track_idx {
            if let Some(track) = app.project.tracks.get(idx) {
                let name = if track.title.is_empty() {
                    track.file_path.as_str()
                } else {
                    track.title.as_str()
                };
                let state_str = if app
                    .preview_player
                    .as_ref()
                    .map(|p| p.is_paused())
                    .unwrap_or(false)
                {
                    "Paused"
                } else {
                    "Playing"
                };
                ui.label(format!("{} [#{:02}]: {}", state_str, idx + 1, name));
            }
        } else if let Some(idx) = target_idx {
            if let Some(track) = app.project.tracks.get(idx) {
                let name = if track.title.is_empty() {
                    track.file_path.as_str()
                } else {
                    track.title.as_str()
                };
                ui.label(format!("Ready [#{:02}]: {}", idx + 1, name));
            }
        } else {
            ui.label("No track selected");
        }
    });
}

fn play_track(app: &mut MusicBoxApp, idx: usize) {
    if let Some(track) = app.project.tracks.get(idx) {
        if app.preview_player.is_none() {
            app.preview_player = Some(AudioPreviewPlayer::new());
        }

        if let Some(player) = &mut app.preview_player {
            if player.is_paused() && app.playing_track_idx == Some(idx) {
                player.resume();
            } else {
                let path = std::path::Path::new(&track.file_path);
                match player.play_file(path) {
                    Ok(()) => {
                        app.playing_track_idx = Some(idx);
                        app.status_message = format!(
                            "Playing: {}",
                            crate::ui::playlist_table::get_export_filename(track)
                        );
                    }
                    Err(e) => {
                        app.status_message = format!("Playback error: {}", e);
                    }
                }
            }
        }
    }
}
