use crate::ui::app::MusicBoxApp;

pub fn render_header(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.heading("🎵 MusicBox Organizer");
        ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));

        ui.add_space(16.0);
        let track_count = app.project.tracks.len();
        ui.label(format!("{} track{}", track_count, if track_count == 1 { "" } else { "s" }));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Theme toggle
            let theme_label = if app.dark_mode { "☀ Light" } else { "🌙 Dark" };
            if ui.button(theme_label).clicked() {
                app.dark_mode = !app.dark_mode;
            }

            ui.add_space(8.0);

            // Redo button
            let can_redo = app.session.can_redo();
            if ui.add_enabled(can_redo, egui::Button::new("⮎ Redo")).clicked()
                && let Some(state) = app.session.redo_project(app.project.clone())
            {
                app.project = state;
                app.selected_track_indices.clear();
                app.editing_cell = None;
                app.status_message = "Redo applied".to_string();
            }

            // Undo button
            let can_undo = app.session.can_undo();
            if ui.add_enabled(can_undo, egui::Button::new("⮌ Undo")).clicked()
                && let Some(state) = app.session.undo_project(app.project.clone())
            {
                app.project = state;
                app.selected_track_indices.clear();
                app.editing_cell = None;
                app.status_message = "Undo applied".to_string();
            }
        });
    });
}
