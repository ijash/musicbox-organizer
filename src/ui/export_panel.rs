use crate::models::Device;
use crate::services::create_device_manager;
use crate::ui::ExportProgressMessage;
use crate::ui::app::MusicBoxApp;
use std::sync::mpsc;
use std::thread;

/// cite: data-model.md: ExportPanelState.selected_device_id, data-model.md: ExportPanelState.show_reformat_confirm
#[derive(Debug, Clone)]
pub struct ExportPanelState {
    pub available_devices: Vec<Device>,
    pub selected_device_id: Option<String>,
    pub subpath: String,
    pub reformat_enabled: bool,
    pub show_reformat_confirm: bool,
    pub is_exporting: bool,
    pub export_progress: f32,
    pub export_status: String,
}

impl Default for ExportPanelState {
    fn default() -> Self {
        Self {
            available_devices: Vec::new(),
            selected_device_id: None,
            subpath: "music".to_string(),
            reformat_enabled: false,
            show_reformat_confirm: false,
            is_exporting: false,
            export_progress: 0.0,
            export_status: String::new(),
        }
    }
}

pub fn render_export_panel(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.strong("Export to Removable Media:");
        ui.add_space(8.0);

        if ui.button("🔄 Scan Devices").clicked() {
            scan_devices(app);
        }

        ui.add_space(8.0);
        egui::ComboBox::from_id_salt("device_select_combo")
            .selected_text(
                app.export_panel
                    .selected_device_id
                    .as_ref()
                    .and_then(|id| {
                        app.export_panel
                            .available_devices
                            .iter()
                            .find(|d| &d.id == id)
                    })
                    .map(|d| d.name.as_str())
                    .unwrap_or("No device selected"),
            )
            .show_ui(ui, |ui| {
                if app.export_panel.available_devices.is_empty() {
                    ui.label("No removable devices found");
                }
                for dev in &app.export_panel.available_devices {
                    ui.selectable_value(
                        &mut app.export_panel.selected_device_id,
                        Some(dev.id.clone()),
                        &dev.name,
                    );
                }
            });

        ui.add_space(8.0);
        ui.label("Subpath:");
        ui.add(egui::TextEdit::singleline(&mut app.export_panel.subpath).desired_width(70.0));

        ui.add_space(8.0);
        ui.checkbox(&mut app.export_panel.reformat_enabled, "Reformat Media");

        ui.add_space(8.0);
        let export_enabled = !app.export_panel.is_exporting
            && app.export_panel.selected_device_id.is_some()
            && !app.project.tracks.is_empty();

        if ui
            .add_enabled(export_enabled, egui::Button::new("💾 Export Playlist"))
            .clicked()
        {
            if app.export_panel.reformat_enabled {
                app.export_panel.show_reformat_confirm = true;
            } else {
                start_export(app);
            }
        }

        let eject_enabled =
            !app.export_panel.is_exporting && app.export_panel.selected_device_id.is_some();
        if ui
            .add_enabled(eject_enabled, egui::Button::new("⏏ Eject"))
            .clicked()
        {
            eject_device(app);
        }
    });

    if app.export_panel.is_exporting {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add(egui::ProgressBar::new(app.export_panel.export_progress).show_percentage());
            ui.label(&app.export_panel.export_status);
        });
    }

    // Modal Confirmation Dialog for Reformat
    if app.export_panel.show_reformat_confirm {
        egui::Window::new("⚠️ Warning: Destructive Action")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.label("Reformatting will ERASE ALL DATA on the selected device!");
                ui.label("Are you completely sure you want to proceed?");
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("Yes, Reformat & Export").clicked() {
                        app.export_panel.show_reformat_confirm = false;
                        start_export(app);
                    }
                    if ui.button("Cancel").clicked() {
                        app.export_panel.show_reformat_confirm = false;
                    }
                });
            });
    }
}

fn scan_devices(app: &mut MusicBoxApp) {
    let mgr = create_device_manager();
    match mgr.enumerate_devices() {
        Ok(devices) => {
            app.export_panel.available_devices = devices;
            if let Some(first) = app.export_panel.available_devices.first() {
                app.export_panel.selected_device_id = Some(first.id.clone());
            } else {
                app.export_panel.selected_device_id = None;
            }
            app.status_message = format!(
                "Found {} removable device(s)",
                app.export_panel.available_devices.len()
            );
        }
        Err(e) => {
            app.status_message = format!("Device scan failed: {}", e);
        }
    }
}

fn start_export(app: &mut MusicBoxApp) {
    let dev = match app.export_panel.selected_device_id.as_ref().and_then(|id| {
        app.export_panel
            .available_devices
            .iter()
            .find(|d| &d.id == id)
    }) {
        Some(d) => d.clone(),
        None => return,
    };

    let mount_point = match &dev.mount_point {
        Some(m) => m.clone(),
        None => {
            app.status_message = format!("Device {} has no mount point", dev.name);
            return;
        }
    };

    let target_dir = if app.export_panel.subpath.trim().is_empty() {
        mount_point
    } else {
        mount_point.join(&app.export_panel.subpath)
    };

    let tracks = app.project.tracks.clone();
    let reformat = app.export_panel.reformat_enabled;
    let (tx, rx) = mpsc::channel();
    app.export_rx = Some(rx);
    app.export_panel.is_exporting = true;
    app.export_panel.export_progress = 0.0;
    app.export_panel.export_status = "Preparing export...".to_string();

    thread::spawn(move || {
        let total = tracks.len();
        let _ = tx.send(ExportProgressMessage::Started {
            total_tracks: total,
        });

        if reformat {
            let mgr = create_device_manager();
            if let Err(e) = mgr.format_device(&dev.id, dev.filesystem) {
                let _ = tx.send(ExportProgressMessage::Failed(format!(
                    "Reformat failed: {}",
                    e
                )));
                return;
            }
        }

        if !target_dir.exists()
            && let Err(e) = std::fs::create_dir_all(&target_dir)
        {
            let _ = tx.send(ExportProgressMessage::Failed(format!(
                "Create dir error: {}",
                e
            )));
            return;
        }

        let mut total_bytes = 0u64;
        for (i, track) in tracks.iter().enumerate() {
            let export_name = crate::ui::playlist_table::get_export_filename(track);
            let _ = tx.send(ExportProgressMessage::Step {
                current: i + 1,
                total,
                filename: export_name.clone(),
            });

            let dest = target_dir.join(&export_name);
            match std::fs::copy(&track.file_path, &dest) {
                Ok(bytes) => total_bytes += bytes,
                Err(e) => {
                    let _ = tx.send(ExportProgressMessage::Failed(format!(
                        "Export error on {}: {}",
                        track.file_path, e
                    )));
                    return;
                }
            }
        }

        let _ = tx.send(ExportProgressMessage::Finished { total_bytes });
    });
}

fn eject_device(app: &mut MusicBoxApp) {
    if let Some(id) = &app.export_panel.selected_device_id {
        let mgr = create_device_manager();
        match mgr.eject_device(id) {
            Ok(()) => {
                app.status_message = format!("Device {} safely ejected", id);
                scan_devices(app);
            }
            Err(e) => {
                app.status_message = format!("Ejection failed: {}", e);
            }
        }
    }
}
