use crate::ui::app::MusicBoxApp;

pub fn render_footer(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(&app.status_message);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.hyperlink_to(
                "GitHub: ijash/musicbox-organizer",
                "https://github.com/ijash/musicbox-organizer",
            );
        });
    });
}
