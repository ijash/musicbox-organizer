use crate::models::{ProjectFile, ProjectTrack, SessionState};
use crate::services::{naming, DirectoryScanner, Id3Service};
use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Button, CellRendererText, Dialog, DialogFlags, FileChooserAction,
    FileChooserDialog, Label, ListStore, Orientation, ResponseType, ScrolledWindow, TreeView,
    TreeViewColumn, Window,
};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub struct PlaylistTableComponent {
    pub container: GtkBox,
    pub tree_view: TreeView,
    pub list_store: ListStore,
    pub audio_tools_btn: Button,
}

enum DspWorkerMessage {
    Progress(crate::services::audio_editing::DspProgress),
    Complete(crate::services::audio_editing::DspBatchReport),
}

pub fn show_dsp_dialog(
    parent: &Window,
    target_paths: Vec<std::path::PathBuf>,
    audio_player: &Rc<RefCell<crate::services::AudioPreviewPlayer>>,
    status_label: &Label,
    on_complete: impl Fn() + 'static,
) -> Dialog {
    let dialog = Dialog::with_buttons(
        Some("Audio DSP Tools"),
        Some(parent),
        DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT,
        &[("Cancel", ResponseType::Cancel), ("Apply", ResponseType::Apply)],
    );
    dialog.set_default_size(360, 260);

    let content_area = dialog.content_area();
    content_area.set_spacing(10);
    content_area.set_margin_start(16);
    content_area.set_margin_end(16);
    content_area.set_margin_top(12);
    content_area.set_margin_bottom(12);

    let info_text = if target_paths.is_empty() {
        "No tracks selected or available.".to_string()
    } else if target_paths.len() == 1 {
        format!(
            "Target: {}",
            target_paths[0]
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "1 track".to_string())
        )
    } else {
        format!("Target: {} tracks in batch", target_paths.len())
    };
    let info_label = Label::new(Some(&info_text));
    info_label.set_xalign(0.0);
    content_area.pack_start(&info_label, false, false, 0);

    // Normalize Volume Option
    let norm_box = GtkBox::new(Orientation::Vertical, 4);
    let check_normalize = gtk::CheckButton::with_label("Normalize Volume (LUFS)");
    check_normalize.set_active(true);
    norm_box.pack_start(&check_normalize, false, false, 0);

    let norm_sub = GtkBox::new(Orientation::Horizontal, 8);
    norm_sub.set_margin_start(20);
    let norm_lbl = Label::new(Some("Target Loudness (dB LUFS):"));
    let spin_norm = gtk::SpinButton::with_range(-30.0, 0.0, 0.5);
    spin_norm.set_value(-14.0);
    norm_sub.pack_start(&norm_lbl, false, false, 0);
    norm_sub.pack_start(&spin_norm, false, false, 0);
    norm_box.pack_start(&norm_sub, false, false, 0);
    content_area.pack_start(&norm_box, false, false, 0);

    // Trim Silence Option
    let trim_box = GtkBox::new(Orientation::Vertical, 4);
    let check_trim = gtk::CheckButton::with_label("Trim Leading/Trailing Silence");
    check_trim.set_active(true);
    trim_box.pack_start(&check_trim, false, false, 0);

    let trim_sub = GtkBox::new(Orientation::Horizontal, 8);
    trim_sub.set_margin_start(20);
    let trim_lbl = Label::new(Some("Silence Threshold (dB):"));
    let spin_trim = gtk::SpinButton::with_range(-80.0, -10.0, 1.0);
    spin_trim.set_value(-45.0);
    trim_sub.pack_start(&trim_lbl, false, false, 0);
    trim_sub.pack_start(&spin_trim, false, false, 0);
    trim_box.pack_start(&trim_sub, false, false, 0);
    content_area.pack_start(&trim_box, false, false, 0);

    // Progress Bar and Label (hidden initially)
    let progress_bar = gtk::ProgressBar::new();
    progress_bar.set_show_text(true);
    progress_bar.set_fraction(0.0);
    progress_bar.set_no_show_all(true);
    progress_bar.set_visible(false);
    content_area.pack_start(&progress_bar, false, false, 0);

    let progress_label = Label::new(None);
    progress_label.set_no_show_all(true);
    progress_label.set_visible(false);
    content_area.pack_start(&progress_label, false, false, 0);

    if target_paths.is_empty() {
        dialog.set_response_sensitive(ResponseType::Apply, false);
    }

    let audio_player = Rc::clone(audio_player);
    let status_label = status_label.clone();
    let on_complete = Rc::new(on_complete);

    dialog.connect_response(move |dlg, response| {
        if response == ResponseType::Apply {
            let normalize = check_normalize.is_active();
            let target_db = spin_norm.value() as f32;
            let trim = check_trim.is_active();
            let trim_threshold_db = spin_trim.value() as f32;

            if !normalize && !trim {
                progress_label.set_text("Please enable at least one DSP operation.");
                progress_label.set_visible(true);
                return;
            }

            // Stop preview playback if current track is in batch
            {
                let mut player = audio_player.borrow_mut();
                if let Some(ref cur) = player.current_file {
                    if target_paths.iter().any(|p| p == cur) {
                        player.stop();
                    }
                }
            }

            // Disable controls during processing
            dlg.set_response_sensitive(ResponseType::Apply, false);
            dlg.set_response_sensitive(ResponseType::Cancel, false);
            check_normalize.set_sensitive(false);
            spin_norm.set_sensitive(false);
            check_trim.set_sensitive(false);
            spin_trim.set_sensitive(false);

            progress_bar.set_visible(true);
            progress_bar.set_fraction(0.0);
            progress_label.set_visible(true);
            progress_label.set_text("Starting audio processing...");

            let (sender, receiver) = gtk::glib::MainContext::channel(gtk::glib::PRIORITY_DEFAULT);
            let paths = target_paths.clone();

            std::thread::spawn(move || {
                use crate::services::audio_editing::{DspBatchReport, DspProgress, NativeAudioProcessor};
                let processor = NativeAudioProcessor::new();
                let mut report = DspBatchReport::default();
                let total = paths.len();

                for (i, path) in paths.iter().enumerate() {
                    let filename = path
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_else(|| format!("track_{}", i + 1));

                    let _ = sender.send(DspWorkerMessage::Progress(DspProgress {
                        current_index: i + 1,
                        total_tracks: total,
                        current_file: filename.clone(),
                        status_message: format!("Processing {}/{} ({})", i + 1, total, filename),
                    }));

                    match processor.process_track(path, trim, trim_threshold_db, normalize, target_db) {
                        Ok(true) => report.succeeded += 1,
                        Ok(false) => report.skipped += 1,
                        Err(e) => report.failed.push((path.clone(), e.to_string())),
                    }
                }

                let _ = sender.send(DspWorkerMessage::Complete(report));
            });

            let dlg_clone = dlg.clone();
            let status_lbl = status_label.clone();
            let on_comp = Rc::clone(&on_complete);
            let p_bar = progress_bar.clone();
            let p_lbl = progress_label.clone();

            receiver.attach(None, move |msg| {
                match msg {
                    DspWorkerMessage::Progress(prog) => {
                        let fraction = (prog.current_index as f64) / (prog.total_tracks as f64).max(1.0);
                        p_bar.set_fraction(fraction);
                        p_bar.set_text(Some(&format!("{}/{}", prog.current_index, prog.total_tracks)));
                        p_lbl.set_text(&prog.status_message);
                        gtk::glib::Continue(true)
                    }
                    DspWorkerMessage::Complete(report) => {
                        status_lbl.set_text(&format!(
                            "DSP Complete: {} modified, {} skipped, {} failed.",
                            report.succeeded,
                            report.skipped,
                            report.failed.len()
                        ));
                        on_comp();
                        dlg_clone.close();
                        gtk::glib::Continue(false)
                    }
                }
            });
        } else {
            dlg.close();
        }
    });

    dialog.show_all();
    dialog
}

pub fn build_playlist_table(
    window: &Window,
    project: &Rc<RefCell<ProjectFile>>,
    session_state: &Rc<RefCell<SessionState>>,
    user_prefs: &Rc<RefCell<crate::models::UserPreferences>>,
    status_label: &Label,
    project_label: &Label,
    sync_session: Rc<dyn Fn()>,
    update_devices_ui: Rc<dyn Fn()>,
    audio_player: &Rc<RefCell<crate::services::AudioPreviewPlayer>>,
) -> PlaylistTableComponent {
    let root_box = GtkBox::new(Orientation::Vertical, 6);

    // --- Action Buttons Toolbar with Standardized Relm4 Icons ---
    use crate::ui::icons::AppIcon;

    let toolbar_box = GtkBox::new(Orientation::Horizontal, 6);
    let scan_dir_btn = AppIcon::FolderOpen.button("Scan Folder...");
    let add_track_btn = AppIcon::ListAdd.button("Add MP3");
    let remove_track_btn = AppIcon::ListRemove.button("Remove");
    let move_up_btn = AppIcon::GoUp.button("Up");
    let move_down_btn = AppIcon::GoDown.button("Down");
    let audio_tools_btn = AppIcon::AudioTools.button("Audio Tools...");
    let export_formats_btn = AppIcon::ExportText.button("Export Text/RTF...");
    let save_project_btn = AppIcon::Save.button("Save");
    let open_project_btn = AppIcon::Open.button("Open");

    toolbar_box.pack_start(&scan_dir_btn, false, false, 0);
    toolbar_box.pack_start(&add_track_btn, false, false, 0);
    toolbar_box.pack_start(&remove_track_btn, false, false, 0);
    toolbar_box.pack_start(&move_up_btn, false, false, 0);
    toolbar_box.pack_start(&move_down_btn, false, false, 0);
    toolbar_box.pack_start(&audio_tools_btn, false, false, 0);
    toolbar_box.pack_end(&save_project_btn, false, false, 0);
    toolbar_box.pack_end(&open_project_btn, false, false, 0);
    toolbar_box.pack_end(&export_formats_btn, false, false, 0);
    root_box.pack_start(&toolbar_box, false, false, 0);

    // --- TreeView Table ---
    let tree_view = TreeView::new();
    tree_view.set_headers_visible(true);
    tree_view.selection().set_mode(gtk::SelectionMode::Multiple);

    let list_store = ListStore::new(&[
        gtk::glib::Type::STRING,
        gtk::glib::Type::STRING,
        gtk::glib::Type::STRING,
        gtk::glib::Type::STRING,
        gtk::glib::Type::STRING,
    ]);
    tree_view.set_model(Some(&list_store));

    let col_idx = TreeViewColumn::with_attributes("#", &CellRendererText::new(), &[("text", 0)]);
    tree_view.append_column(&col_idx);

    let col_name = TreeViewColumn::with_attributes(
        "Export Filename [XXYY-filename.mp3]",
        &CellRendererText::new(),
        &[("text", 1)],
    );
    tree_view.append_column(&col_name);

    let cell_title = CellRendererText::new();
    cell_title.set_editable(true);
    let col_title = TreeViewColumn::with_attributes("Title (Inline Edit)", &cell_title, &[("text", 2)]);
    tree_view.append_column(&col_title);

    let cell_artist = CellRendererText::new();
    cell_artist.set_editable(true);
    let col_artist = TreeViewColumn::with_attributes("Artist (Inline Edit)", &cell_artist, &[("text", 3)]);
    tree_view.append_column(&col_artist);

    let col_path = TreeViewColumn::with_attributes("Source File", &CellRendererText::new(), &[("text", 4)]);
    tree_view.append_column(&col_path);

    let scrolled = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled.set_min_content_height(160);
    scrolled.add(&tree_view);
    root_box.pack_start(&scrolled, true, true, 0);

    let refresh_table = {
        let project = Rc::clone(project);
        let list_store = list_store.clone();
        let project_label = project_label.clone();
        move || {
            list_store.clear();
            let p = project.borrow();
            project_label.set_markup(&format!(
                "<big><b>MusicBox Organizer</b></big> — <i>{}</i> ({} tracks)",
                p.metadata.name,
                p.tracks.len()
            ));
            for t in &p.tracks {
                let filename = Path::new(&t.file_path)
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_else(|| format!("track_{}.mp3", t.index));
                let xxyy = naming::format_export_filename(t.index, &filename)
                    .unwrap_or_else(|_| filename.clone());
                list_store.insert_with_values(
                    None,
                    &[
                        (0, &t.index.to_string()),
                        (1, &xxyy),
                        (2, &t.title),
                        (3, &t.artist),
                        (4, &t.file_path),
                    ],
                );
            }
        }
    };

    refresh_table();

    // Inline title edit: update row in place to avoid clearing model during cell editing
    {
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let list_store = list_store.clone();
        cell_title.connect_edited(move |_, path, new_text| {
            if let Ok(idx) = path.to_string().parse::<usize>() {
                sync_session();
                session_state.borrow_mut().push_undo_checkpoint();
                let mut p = project.borrow_mut();
                if idx < p.tracks.len() {
                    p.tracks[idx].title = new_text.to_string();
                    let tree_path = gtk::TreePath::from_indicesv(&[idx as i32]);
                    if let Some(iter) = list_store.iter(&tree_path) {
                        list_store.set_value(&iter, 2, &new_text.to_value());
                    }
                }
            }
        });
    }

    // Inline artist edit: update row in place to avoid clearing model during cell editing
    {
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let list_store = list_store.clone();
        cell_artist.connect_edited(move |_, path, new_text| {
            if let Ok(idx) = path.to_string().parse::<usize>() {
                sync_session();
                session_state.borrow_mut().push_undo_checkpoint();
                let mut p = project.borrow_mut();
                if idx < p.tracks.len() {
                    p.tracks[idx].artist = new_text.to_string();
                    let tree_path = gtk::TreePath::from_indicesv(&[idx as i32]);
                    if let Some(iter) = list_store.iter(&tree_path) {
                        list_store.set_value(&iter, 3, &new_text.to_value());
                    }
                }
            }
        });
    }

    // Remove Track button
    {
        let tree_view = tree_view.clone();
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
        let update_devs = Rc::clone(&update_devices_ui);
        let status_label = status_label.clone();
        remove_track_btn.connect_clicked(move |_| {
            let (paths, _) = tree_view.selection().selected_rows();
            if let Some(path) = paths.first() {
                if let Some(indices) = path.indices().first() {
                    let idx = *indices as usize;
                    sync_session();
                    session_state.borrow_mut().push_undo_checkpoint();
                    let mut p = project.borrow_mut();
                    if idx < p.tracks.len() {
                        let removed = p.tracks.remove(idx);
                        for (i, t) in p.tracks.iter_mut().enumerate() {
                            t.index = i as u32;
                        }
                        drop(p);
                        refresh_table();
                        update_devs();
                        status_label.set_text(&format!("Removed track '{}'", removed.title));
                        if idx < tree_view.model().unwrap().iter_n_children(None) as usize {
                            let new_path = gtk::TreePath::from_indicesv(&[idx as i32]);
                            tree_view.selection().select_path(&new_path);
                        } else if idx > 0 {
                            let new_path = gtk::TreePath::from_indicesv(&[(idx - 1) as i32]);
                            tree_view.selection().select_path(&new_path);
                        }
                    }
                }
            }
        });
    }

    // Move Up
    {
        let tree_view = tree_view.clone();
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
        move_up_btn.connect_clicked(move |_| {
            let (paths, _) = tree_view.selection().selected_rows();
            if let Some(path) = paths.first() {
                if let Some(indices) = path.indices().first() {
                    let idx = *indices as usize;
                    if idx > 0 {
                        let new_idx = idx - 1;
                        sync_session();
                        session_state.borrow_mut().push_undo_checkpoint();
                        let mut p = project.borrow_mut();
                        p.tracks.swap(idx, new_idx);
                        for (i, t) in p.tracks.iter_mut().enumerate() {
                            t.index = i as u32;
                        }
                        drop(p);
                        refresh_table();
                        let new_path = gtk::TreePath::from_indicesv(&[new_idx as i32]);
                        tree_view.selection().select_path(&new_path);
                    }
                }
            }
        });
    }

    // Move Down
    {
        let tree_view = tree_view.clone();
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
        move_down_btn.connect_clicked(move |_| {
            let (paths, _) = tree_view.selection().selected_rows();
            if let Some(path) = paths.first() {
                if let Some(indices) = path.indices().first() {
                    let idx = *indices as usize;
                    let len = project.borrow().tracks.len();
                    if idx + 1 < len {
                        let new_idx = idx + 1;
                        sync_session();
                        session_state.borrow_mut().push_undo_checkpoint();
                        let mut p = project.borrow_mut();
                        p.tracks.swap(idx, new_idx);
                        for (i, t) in p.tracks.iter_mut().enumerate() {
                            t.index = i as u32;
                        }
                        drop(p);
                        refresh_table();
                        let new_path = gtk::TreePath::from_indicesv(&[new_idx as i32]);
                        tree_view.selection().select_path(&new_path);
                    }
                }
            }
        });
    }

    // Audio Tools (DSP)
    {
        let window_clone = window.clone();
        let project = Rc::clone(project);
        let audio_player = Rc::clone(audio_player);
        let status_label = status_label.clone();
        let tree_view = tree_view.clone();
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
        audio_tools_btn.connect_clicked(move |_| {
            let (paths, _) = tree_view.selection().selected_rows();
            let mut target_paths: Vec<std::path::PathBuf> = Vec::new();
            let p = project.borrow();
            if paths.is_empty() {
                for t in &p.tracks {
                    target_paths.push(std::path::PathBuf::from(&t.file_path));
                }
            } else {
                for path in &paths {
                    if let Some(idx) = path.indices().first() {
                        let idx = *idx as usize;
                        if let Some(t) = p.tracks.get(idx) {
                            target_paths.push(std::path::PathBuf::from(&t.file_path));
                        }
                    }
                }
            }
            drop(p);

            sync_session();
            session_state.borrow_mut().push_undo_checkpoint();

            let refresh = refresh_table.clone();
            show_dsp_dialog(
                &window_clone,
                target_paths,
                &audio_player,
                &status_label,
                move || {
                    refresh();
                },
            );
        });
    }

    // Scan Folder
    {
        let window_clone = window.clone();
        let project = Rc::clone(project);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
        let update_devs = Rc::clone(&update_devices_ui);
        let status_label = status_label.clone();
        scan_dir_btn.connect_clicked(move |_| {
            let chooser = FileChooserDialog::new(
                Some("Scan Folder for MP3s"),
                Some(&window_clone),
                FileChooserAction::SelectFolder,
            );
            chooser.add_button("Cancel", ResponseType::Cancel);
            chooser.add_button("Scan Folder", ResponseType::Accept);

            if chooser.run() == ResponseType::Accept {
                if let Some(folder) = chooser.filename() {
                    let scanner = DirectoryScanner::new();
                    match scanner.scan_directory(&folder) {
                        Ok(scanned) => {
                            let count = scanned.len();
                            let mut p = project.borrow_mut();
                            let id3_svc = Id3Service::new();
                            for mut t in scanned {
                                if let Ok(meta) = id3_svc.read_tags(&t.file_path) {
                                    t.metadata = meta;
                                }
                                let next_idx = p.tracks.len() as u32;
                                p.tracks.push(ProjectTrack {
                                    index: next_idx,
                                    title: t.metadata.title,
                                    artist: t.metadata.artist,
                                    file_path: t.file_path.to_string_lossy().to_string(),
                                });
                            }
                            drop(p);
                            sync_session();
                            refresh_table();
                            update_devs();
                            status_label.set_text(&format!(
                                "Scanned {} MP3 files from {}",
                                count,
                                folder.display()
                            ));
                        }
                        Err(e) => status_label.set_text(&format!("Scan failed: {}", e)),
                    }
                }
            }
            chooser.close();
        });
    }

    // Add track
    {
        let window_clone = window.clone();
        let project = Rc::clone(project);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
        let update_devs = Rc::clone(&update_devices_ui);
        add_track_btn.connect_clicked(move |_| {
            let chooser = FileChooserDialog::new(
                Some("Select Audio File(s)"),
                Some(&window_clone),
                FileChooserAction::Open,
            );
            chooser.add_button("Cancel", ResponseType::Cancel);
            chooser.add_button("Add", ResponseType::Accept);
            chooser.set_select_multiple(true);

            if chooser.run() == ResponseType::Accept {
                let filenames = chooser.filenames();
                let mut p = project.borrow_mut();
                let id3_svc = Id3Service::new();
                for path in filenames {
                    let path_str = path.to_string_lossy().to_string();
                    let meta = id3_svc.read_tags(&path).unwrap_or_default();
                    let next_idx = p.tracks.len() as u32;
                    p.tracks.push(ProjectTrack {
                        index: next_idx,
                        title: meta.title,
                        artist: meta.artist,
                        file_path: path_str,
                    });
                }
                drop(p);
                sync_session();
                refresh_table();
                update_devs();
            }
            chooser.close();
        });
    }

    // Save project (.txt)
    {
        let window_clone = window.clone();
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let status_label = status_label.clone();
        save_project_btn.connect_clicked(move |_| {
            let chooser = FileChooserDialog::new(
                Some("Save Project File (.txt)"),
                Some(&window_clone),
                FileChooserAction::Save,
            );
            chooser.add_button("Cancel", ResponseType::Cancel);
            chooser.add_button("Save", ResponseType::Accept);
            chooser.set_current_name("playlist.txt");

            if chooser.run() == ResponseType::Accept {
                if let Some(file) = chooser.filename() {
                    let p = project.borrow();
                    let id3_svc = Id3Service::new();
                    for track in &p.tracks {
                        let track_path = Path::new(&track.file_path);
                        if track_path.exists() {
                            let mut meta = id3_svc.read_tags(track_path).unwrap_or_default();
                            meta.title = track.title.clone();
                            meta.artist = track.artist.clone();
                            meta.track_number = Some(track.index + 1);
                            id3_svc.write_tags(track_path, &meta).ok();
                        }
                    }
                    match p.save(&file) {
                        Ok(_) => {
                            session_state.borrow_mut().mark_saved();
                            status_label.set_text(&format!(
                                "Saved project and committed ID3 tags to {}",
                                file.display()
                            ));
                        }
                        Err(e) => {
                            status_label.set_text(&format!("Failed to save project: {}", e))
                        }
                    }
                }
            }
            chooser.close();
        });
    }

    // Open project (.txt)
    {
        let window_clone = window.clone();
        let project = Rc::clone(project);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
        let update_devs = Rc::clone(&update_devices_ui);
        let status_label = status_label.clone();
        open_project_btn.connect_clicked(move |_| {
            let chooser = FileChooserDialog::new(
                Some("Open Project File (.txt)"),
                Some(&window_clone),
                FileChooserAction::Open,
            );
            chooser.add_button("Cancel", ResponseType::Cancel);
            chooser.add_button("Open", ResponseType::Accept);

            if chooser.run() == ResponseType::Accept {
                if let Some(file) = chooser.filename() {
                    match ProjectFile::load(&file) {
                        Ok(loaded) => {
                            *project.borrow_mut() = loaded;
                            sync_session();
                            refresh_table();
                            update_devs();
                            status_label.set_text(&format!("Loaded project from {}", file.display()));
                        }
                        Err(e) => status_label.set_text(&format!("Failed to load project: {}", e)),
                    }
                }
            }
            chooser.close();
        });
    }

    // Export formats (.txt & .rtf)
    {
        let window_clone = window.clone();
        let project = Rc::clone(project);
        let user_prefs = Rc::clone(user_prefs);
        let status_label = status_label.clone();
        export_formats_btn.connect_clicked(move |_| {
            let p = project.borrow();
            if p.tracks.is_empty() {
                status_label.set_text("Playlist is empty; nothing to export.");
                return;
            }

            let dlg = Dialog::with_buttons(
                Some("Export Playlist to 'playlist-text/'"),
                Some(&window_clone),
                DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Export", ResponseType::Accept),
                ],
            );

            let content_area = dlg.content_area();
            content_area.set_spacing(8);
            content_area.set_margin_start(16);
            content_area.set_margin_end(16);
            content_area.set_margin_top(12);
            content_area.set_margin_bottom(12);

            let info = Label::new(Some("Select formats to generate in playlist-text/ folder:"));
            info.set_xalign(0.0);
            content_area.pack_start(&info, false, false, 0);

            let check_txt = gtk::CheckButton::with_label("Plain Text (.txt)");
            let check_rtf = gtk::CheckButton::with_label("Rich Text Format (.rtf)");

            let current_prefs = user_prefs.borrow();
            check_txt.set_active(current_prefs.formats.txt);
            check_rtf.set_active(current_prefs.formats.rtf);
            drop(current_prefs);

            content_area.pack_start(&check_txt, false, false, 0);
            content_area.pack_start(&check_rtf, false, false, 0);

            let confirm_btn = dlg.widget_for_response(ResponseType::Accept).unwrap();
            let validate = {
                let check_txt = check_txt.clone();
                let check_rtf = check_rtf.clone();
                let confirm_btn = confirm_btn.clone();
                move || {
                    let has_any = check_txt.is_active() || check_rtf.is_active();
                    confirm_btn.set_sensitive(has_any);
                }
            };

            validate();
            check_txt.connect_toggled({
                let val = validate.clone();
                move |_| val()
            });
            check_rtf.connect_toggled({
                let val = validate.clone();
                move |_| val()
            });

            dlg.show_all();
            let resp = dlg.run();

            let is_txt = check_txt.is_active();
            let is_rtf = check_rtf.is_active();
            dlg.close();

            if resp == ResponseType::Accept {
                let mut prefs = user_prefs.borrow_mut();
                prefs.formats.txt = is_txt;
                prefs.formats.rtf = is_rtf;
                prefs.save_to_dir(".").ok();
                drop(prefs);

                let service = crate::services::ExportConfigService::new();
                let selection = crate::models::ExportFormatSelection {
                    txt: is_txt,
                    rtf: is_rtf,
                };
                match service.export_playlist(Path::new("."), &selection, &p) {
                    Ok(written) => {
                        let names: Vec<String> = written
                            .iter()
                            .map(|f| {
                                f.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string()
                            })
                            .collect();
                        status_label.set_text(&format!(
                            "Exported to playlist-text/: {}",
                            names.join(", ")
                        ));
                    }
                    Err(e) => status_label.set_text(&format!("Export failed: {}", e)),
                }
            }
        });
    }

    PlaylistTableComponent {
        container: root_box,
        tree_view,
        list_store,
        audio_tools_btn,
    }
}
