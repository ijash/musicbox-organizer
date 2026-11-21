use crate::ui::app::MusicBoxApp;

pub fn render_footer(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    if ui.available_width() < 380.0 {
        ui.add(egui::Label::new(&app.status_message).truncate());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.hyperlink_to(
                egui::RichText::new("musicbox").italics(),
                "https://github.com/ijash/musicbox-organizer",
            );
            ui.label(egui::RichText::new("Made by Ijash. Github: ").italics());
        });
    } else {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.hyperlink_to(
                    egui::RichText::new("musicbox").italics(),
                    "https://github.com/ijash/musicbox-organizer",
                );
                ui.label(egui::RichText::new("Made by Ijash. Github: ").italics());
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add(egui::Label::new(&app.status_message).truncate());
                });
            });
        });
    }
}
