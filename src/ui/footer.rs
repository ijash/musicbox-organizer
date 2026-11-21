use crate::ui::app::MusicBoxApp;

pub fn render_footer(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        ui.label(&app.status_message);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.hyperlink_to(
                egui::RichText::new("musicbox").italics(),
                "https://github.com/ijash/musicbox-organizer",
            );
            ui.label(egui::RichText::new("Made by Ijash. Github: ").italics());
        });
    });
}
