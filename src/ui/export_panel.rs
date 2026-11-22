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
    pub show_export_confirm: bool,
    pub show_reformat_confirm: bool,
    pub reformat_ack_checkbox: bool,
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
            show_export_confirm: false,
            show_reformat_confirm: false,
            reformat_ack_checkbox: false,
            is_exporting: false,
            export_progress: 0.0,
            export_status: String::new(),
        }
    }
}

pub fn render_export_panel(app: &mut MusicBoxApp, ui: &mut egui::Ui) {
    // Row 1: Header & Scan
    ui.horizontal_wrapped(|ui| {
        ui.strong("Export to Removable Media:");
        if ui.button("🔄 Scan Devices").clicked() {
            scan_devices(app);
        }
    });

    // Row 2: Device Selection ComboBox (adaptive width up to 260px)
    let combo_width = ui.available_width().min(260.0);
    egui::ComboBox::from_id_salt("device_select_combo")
        .width(combo_width)
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

    // Row 3: Subpath & Reformat Media Checkbox
    ui.horizontal_wrapped(|ui| {
        ui.label("Subpath:");
        ui.add(egui::TextEdit::singleline(&mut app.export_panel.subpath).desired_width(90.0));
        ui.checkbox(&mut app.export_panel.reformat_enabled, "Reformat Media");
    });

    // Row 4: Export & Eject Actions
    ui.horizontal_wrapped(|ui| {
        let export_enabled = !app.export_panel.is_exporting
            && app.export_panel.selected_device_id.is_some()
            && !app.project.tracks.is_empty();

        if ui
            .add_enabled(export_enabled, egui::Button::new("💾 Export Playlist"))
            .clicked()
        {
            app.export_panel.show_export_confirm = true;
            app.export_panel.show_reformat_confirm = app.export_panel.reformat_enabled;
            app.export_panel.reformat_ack_checkbox = false;
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
}

pub fn render_export_confirm_dialog(app: &mut MusicBoxApp, ctx: &egui::Context) {
    if !app.export_panel.show_export_confirm {
        return;
    }

    let viewport_id = egui::ViewportId::from_hash_of("export_media_confirm_viewport");
    let is_reformat = app.export_panel.reformat_enabled;
    let title = if is_reformat {
        "⚠️ Warning: Destructive Action - Reformat Media"
    } else {
        "Confirm Export to Removable Media"
    };

    ctx.show_viewport_immediate(
        viewport_id,
        egui::ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([420.0, if is_reformat { 340.0 } else { 260.0 }])
            .with_min_inner_size([320.0, 220.0])
            .with_resizable(true),
        |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested()) {
                app.export_panel.show_export_confirm = false;
                app.export_panel.show_reformat_confirm = false;
            }
            render_export_confirm_contents(app, ctx);
            if !app.export_panel.show_export_confirm {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        },
    );
}

pub fn render_export_confirm_contents(app: &mut MusicBoxApp, ctx: &egui::Context) {
    let (dev_name, mount_path) = match app.export_panel.selected_device_id.as_ref().and_then(|id| {
        app.export_panel
            .available_devices
            .iter()
            .find(|d| &d.id == id)
    }) {
        Some(d) => (
            d.name.clone(),
            d.mount_point
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "-".to_string()),
        ),
        None => ("Unknown Device".to_string(), "-".to_string()),
    };
    let track_count = app.project.tracks.len();
    let subpath = app.export_panel.subpath.clone();
    let is_reformat = app.export_panel.reformat_enabled;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            if is_reformat {
                ui.colored_label(
                    egui::Color32::RED,
                    egui::RichText::new("⚠️ DESTRUCTIVE ACTION WARNING")
                        .heading()
                        .strong(),
                );
                ui.add_space(4.0);
                ui.colored_label(
                    egui::Color32::RED,
                    egui::RichText::new(format!(
                        "ALL DATA ON {} ({}) WILL BE PERMANENTLY ERASED!",
                        dev_name, mount_path
                    ))
                    .strong(),
                );
                ui.add_space(8.0);
            } else {
                ui.heading("Confirm Export to Removable Media");
                ui.add_space(8.0);
            }

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.label(format!("Target Device: {}", dev_name));
                ui.label(format!("Mount Path: {}", mount_path));
                ui.label(format!("Tracks to Export: {}", track_count));
                ui.label(format!("Target Subpath: /{}", subpath));
            });

            ui.add_space(8.0);

            if is_reformat {
                ui.checkbox(
                    &mut app.export_panel.reformat_ack_checkbox,
                    egui::RichText::new("I understand all data on this drive will be erased")
                        .color(egui::Color32::RED)
                        .strong(),
                );
                ui.add_space(8.0);
            }

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    app.export_panel.show_export_confirm = false;
                    app.export_panel.show_reformat_confirm = false;
                }

                if is_reformat {
                    let can_reformat = app.export_panel.reformat_ack_checkbox;
                    let btn = ui.add_enabled(
                        can_reformat,
                        egui::Button::new(
                            egui::RichText::new("Erase, Format & Export")
                                .color(egui::Color32::WHITE),
                        )
                        .fill(if can_reformat {
                            egui::Color32::from_rgb(180, 40, 40)
                        } else {
                            egui::Color32::DARK_GRAY
                        }),
                    );
                    if btn.clicked() {
                        app.export_panel.show_export_confirm = false;
                        app.export_panel.show_reformat_confirm = false;
                        start_export(app);
                    }
                } else {
                    if ui.button("Confirm Export").clicked() {
                        app.export_panel.show_export_confirm = false;
                        start_export(app);
                    }
                }
            });
        });
    });
}

pub fn scan_devices(app: &mut MusicBoxApp) {
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
