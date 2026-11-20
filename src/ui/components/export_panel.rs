use crate::models::{FileSystem, Track};
use crate::services::{create_device_manager, naming, ExportService};
use crate::ui::warnings::ReformatWarningDialog;
use crate::ui::ExportDialog;
use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Button, CheckButton, ComboBoxText, Dialog, DialogFlags, Entry, Frame, Label,
    Orientation, ProgressBar, ResponseType, Window,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct ExportPanelComponent {
    pub container: Frame,
    pub dev_combo: ComboBoxText,
    pub dev_info_label: Label,
    pub export_btn: Button,
    pub eject_btn: Button,
    pub status_label: Label,
}

pub fn build_export_panel(
    window: &Window,
    project: &Rc<RefCell<crate::models::ProjectFile>>,
    dialog_state: &Rc<RefCell<ExportDialog>>,
) -> ExportPanelComponent {
    let export_frame = Frame::new(Some("Export to Removable Media (USB / SD)"));
    let frame_box = GtkBox::new(Orientation::Vertical, 4);
    frame_box.set_margin_start(8);
    frame_box.set_margin_end(8);
    frame_box.set_margin_top(4);
    frame_box.set_margin_bottom(4);

    // Device Selection Row
    let dev_row = GtkBox::new(Orientation::Horizontal, 8);
    let dev_label = Label::new(Some("Target Device:"));
    let dev_combo = ComboBoxText::new();
    let refresh_btn = Button::with_label("↻ Scan");
    dev_row.pack_start(&dev_label, false, false, 0);
    dev_row.pack_start(&dev_combo, true, true, 0);
    dev_row.pack_start(&refresh_btn, false, false, 0);
    frame_box.pack_start(&dev_row, false, false, 0);

    // Device Details / Warning Status
    let dev_info_label = Label::new(None);
    dev_info_label.set_xalign(0.0);
    frame_box.pack_start(&dev_info_label, false, false, 0);

    // Options Row: Reformat Checkbox & Subpath
    let opt_row = GtkBox::new(Orientation::Horizontal, 12);
    let reformat_check = CheckButton::with_label("Reformat before export");
    let subpath_label = Label::new(Some("Subpath:"));
    let subpath_entry = Entry::new();
    subpath_entry.set_text("/");
    opt_row.pack_start(&reformat_check, false, false, 0);
    opt_row.pack_start(&subpath_label, false, false, 0);
    opt_row.pack_start(&subpath_entry, false, false, 0);
    frame_box.pack_start(&opt_row, false, false, 0);

    // Action Buttons Row
    let action_row = GtkBox::new(Orientation::Horizontal, 8);
    let export_btn = Button::with_label("▶ Export Playlist to Media");
    let eject_btn = Button::with_label("⏏ Eject Device");
    action_row.pack_start(&export_btn, true, true, 0);
    action_row.pack_start(&eject_btn, false, false, 0);
    frame_box.pack_start(&action_row, false, false, 0);

    // Progress & Status
    let progress_bar = ProgressBar::new();
    progress_bar.set_fraction(0.0);
    let status_label = Label::new(Some("Ready"));
    status_label.set_xalign(0.0);
    frame_box.pack_start(&progress_bar, false, false, 0);
    frame_box.pack_start(&status_label, false, false, 0);

    export_frame.add(&frame_box);

    // Helper: Update devices
    let update_devices_ui = {
        let dialog_state = Rc::clone(dialog_state);
        let dev_combo = dev_combo.clone();
        let dev_info_label = dev_info_label.clone();
        let export_btn = export_btn.clone();
        let eject_btn = eject_btn.clone();
        let project = Rc::clone(project);
        move || {
            let manager = create_device_manager();
            let mut state = dialog_state.borrow_mut();
            state.refresh_devices(manager.as_ref()).ok();

            dev_combo.remove_all();

            if state.devices.is_empty() {
                dev_info_label.set_markup("<span foreground='#cc3333'><b>No removable devices detected.</b> Connect a USB drive or SD card to proceed.</span>");
                export_btn.set_sensitive(false);
                eject_btn.set_sensitive(false);
            } else {
                for dev in &state.devices {
                    let text = format!("{} [{}] ({:.1} GB)", dev.name, dev.id, dev.capacity_bytes as f64 / 1e9);
                    dev_combo.append(Some(&dev.id), &text);
                }
                if let Some(first) = state.devices.first() {
                    dev_combo.set_active_id(Some(&first.id));
                    let fs_name = match &first.filesystem {
                        FileSystem::FAT32 => "FAT32".to_string(),
                        FileSystem::exFAT => "exFAT".to_string(),
                        FileSystem::Unknown(s) => if s.is_empty() { "Unknown".to_string() } else { s.clone() },
                    };
                    let mount_str = first.mount_point.as_ref()
                        .map(|m| m.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Not mounted".to_string());
                    dev_info_label.set_markup(&format!(
                        "<small>Mount: <b>{}</b> | FileSystem: <b>{}</b> | Free: <b>{:.1} GB</b></small>",
                        mount_str,
                        fs_name,
                        first.available_bytes as f64 / 1e9
                    ));
                }
                let has_tracks = !project.borrow().tracks.is_empty();
                export_btn.set_sensitive(has_tracks);
                eject_btn.set_sensitive(true);
            }
        }
    };

    update_devices_ui();

    // Refresh button
    {
        let update_devices = update_devices_ui.clone();
        refresh_btn.connect_clicked(move |_| {
            update_devices();
        });
    }

    // Export button click
    {
        let window_clone = window.clone();
        let project = Rc::clone(project);
        let dialog_state = Rc::clone(dialog_state);
        let reformat_check = reformat_check.clone();
        let subpath_entry = subpath_entry.clone();
        let progress_bar = progress_bar.clone();
        let status_label = status_label.clone();
        let update_devs = update_devices_ui.clone();

        export_btn.connect_clicked(move |_| {
            let p = project.borrow();
            if p.tracks.is_empty() {
                status_label.set_text("Playlist has no tracks to export.");
                return;
            }

            let tracks: Vec<Track> = p.tracks.iter().filter_map(|pt| {
                let path = PathBuf::from(&pt.file_path);
                let filename = path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_else(|| format!("track_{}.mp3", pt.index));
                let xxyy_name = naming::format_export_filename(pt.index, &filename).ok()?;
                let file_size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                Some(Track {
                    index: pt.index,
                    source_path: path,
                    filename,
                    xxyy_name,
                    file_size_bytes,
                })
            }).collect();
            drop(p);

            let mut state = dialog_state.borrow_mut();
            state.reformat_enabled = reformat_check.is_active();
            let subpath_str = subpath_entry.text().to_string();
            state.export_subpath = PathBuf::from(if subpath_str.trim().is_empty() { "/" } else { subpath_str.trim() });

            if state.reformat_enabled {
                let dev = match state.selected_device() {
                    Some(d) => d,
                    None => {
                        status_label.set_text("No target device selected.");
                        return;
                    }
                };
                let warning_ctrl = ReformatWarningDialog::new(dev.name.clone(), dev.capacity_bytes);

                let warn_dlg = Dialog::with_buttons(
                    Some("⚠ WARNING: Device Will Be Erased"),
                    Some(&window_clone),
                    DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT,
                    &[
                        ("Cancel", ResponseType::Cancel),
                        ("I UNDERSTAND - FORMAT & EXPORT", ResponseType::Accept),
                    ],
                );
                let content_area = warn_dlg.content_area();
                let msg_label = Label::new(Some(&warning_ctrl.message()));
                msg_label.set_margin_start(16);
                msg_label.set_margin_end(16);
                msg_label.set_margin_top(16);
                msg_label.set_margin_bottom(16);
                content_area.pack_start(&msg_label, true, true, 0);
                warn_dlg.show_all();

                let response = warn_dlg.run();
                warn_dlg.close();

                if response == ResponseType::Accept {
                    if let Err(e) = state.trigger_export(tracks) {
                        status_label.set_text(&format!("Export failed to start: {}", e));
                        return;
                    }
                    state.confirm_reformat_warning();
                } else {
                    status_label.set_text("Reformat cancelled. Export aborted.");
                    return;
                }
            } else {
                if let Err(e) = state.trigger_export(tracks) {
                    status_label.set_text(&format!("Export failed to start: {}", e));
                    return;
                }
            }

            status_label.set_text("Export in progress...");
            progress_bar.set_fraction(0.3);

            let manager = create_device_manager();
            let service = ExportService::new(manager);

            match state.execute_export(&service) {
                Ok(_) => {
                    progress_bar.set_fraction(1.0);
                    if let Some(job) = &state.current_job {
                        status_label.set_markup(&format!(
                            "<span foreground='#2e7d32'><b>✓ Export Complete!</b> Successfully wrote {} tracks.</span>",
                            job.files_written
                        ));
                    }
                }
                Err(e) => {
                    progress_bar.set_fraction(0.0);
                    status_label.set_markup(&format!(
                        "<span foreground='#cc3333'><b>✗ Export Failed:</b> {}</span>",
                        e
                    ));
                }
            }
            drop(state);
            update_devs();
        });
    }

    // Eject button
    {
        let dialog_state = Rc::clone(dialog_state);
        let status_label = status_label.clone();
        let update_devs = update_devices_ui.clone();
        eject_btn.connect_clicked(move |_| {
            let manager = create_device_manager();
            let service = ExportService::new(manager);
            let state = dialog_state.borrow();
            match state.eject_selected(&service) {
                Ok(_) => status_label.set_text("Device unmounted/ejected safely."),
                Err(e) => status_label.set_text(&format!("Failed to eject device: {}", e)),
            }
            drop(state);
            update_devs();
        });
    }

    ExportPanelComponent {
        container: export_frame,
        dev_combo,
        dev_info_label,
        export_btn,
        eject_btn,
        status_label,
    }
}
