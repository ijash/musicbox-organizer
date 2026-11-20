#[cfg(target_os = "linux")]
pub mod linux {
    use gtk::prelude::*;
    use gtk::{
        Box as GtkBox, Button, CellRendererText, CheckButton, ComboBoxText, CssProvider, Dialog,
        DialogFlags, Entry, FileChooserAction, FileChooserDialog, Frame, Label, ListStore,
        Orientation, ProgressBar, ResponseType, ScrolledWindow, StyleContext, TreeView,
        TreeViewColumn, Window, WindowPosition, WindowType, STYLE_PROVIDER_PRIORITY_USER,
    };
    use crate::models::{FileSystem, PlaylistMetadata, ProjectFile, ProjectTrack, SessionState, Track};
    use crate::services::{
        create_device_manager, AudioPreviewPlayer, DirectoryScanner, ExportService, Id3Service,
        naming,
    };
    use crate::ui::ExportDialog;
    use crate::ui::warnings::ReformatWarningDialog;
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    const LIGHT_CSS: &str = "
        window, dialog, popover {
            background-color: #f6f8fa;
            color: #24292f;
        }
        frame, frame > border {
            border-color: #d0d7de;
        }
        treeview {
            background-color: #ffffff;
            color: #24292f;
        }
        treeview:selected, treeview:selected:focus {
            background-color: #0969da;
            color: #ffffff;
        }
        treeview header button {
            background-color: #eaeef2;
            color: #24292f;
            font-weight: bold;
            border-color: #d0d7de;
        }
        entry, combobox button {
            background-color: #ffffff;
            color: #24292f;
            border-color: #d0d7de;
        }
        button {
            background-color: #f3f4f6;
            color: #24292f;
            border-color: #d0d7de;
        }
        button:hover {
            background-color: #e5e7eb;
        }
        progressbar progress {
            background-color: #0969da;
        }
        progressbar trough {
            background-color: #eaeef2;
        }
        label {
            color: #24292f;
        }
    ";

    const DARK_CSS: &str = "
        window, dialog, popover {
            background-color: #1e2227;
            color: #abb2bf;
        }
        frame, frame > border {
            border-color: #3e4451;
        }
        treeview {
            background-color: #282c34;
            color: #abb2bf;
        }
        treeview:selected, treeview:selected:focus {
            background-color: #2c5a85;
            color: #ffffff;
        }
        treeview header button {
            background-color: #21252b;
            color: #abb2bf;
            font-weight: bold;
            border-color: #3e4451;
        }
        entry, combobox button {
            background-color: #1b1d23;
            color: #ffffff;
            border-color: #3e4451;
        }
        button {
            background-color: #2c313a;
            color: #dcdfe4;
            border-color: #3e4451;
        }
        button:hover {
            background-color: #3e4451;
        }
        progressbar progress {
            background-color: #61afef;
        }
        progressbar trough {
            background-color: #1b1d23;
        }
        label {
            color: #abb2bf;
        }
    ";

    pub fn run_gtk_app() {
        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK.");
            return;
        }

        let project = Rc::new(RefCell::new(
            ProjectFile::load("playlist.txt").unwrap_or_else(|_| {
                ProjectFile::new(PlaylistMetadata {
                    name: "Default Playlist".to_string(),
                    author: None,
                    created_at: Some(chrono::Utc::now().to_rfc3339()),
                })
            }),
        ));

        let session_state = Rc::new(RefCell::new(SessionState::new(PathBuf::from("."))));
        let dialog_state = Rc::new(RefCell::new(ExportDialog::new()));
        let audio_player = Rc::new(RefCell::new(AudioPreviewPlayer::new()));

        let user_prefs = Rc::new(RefCell::new(crate::models::UserPreferences::load_from_dir(".")));
        let is_dark_mode = Rc::new(RefCell::new(user_prefs.borrow().dark_mode));
        let css_provider = CssProvider::new();

        let window = Window::new(WindowType::Toplevel);
        window.set_title("MusicBox Organizer");
        window.set_default_size(740, 520);
        window.set_position(WindowPosition::Center);

        let root_box = GtkBox::new(Orientation::Vertical, 6);
        root_box.set_margin_start(8);
        root_box.set_margin_end(8);
        root_box.set_margin_top(8);
        root_box.set_margin_bottom(8);

        // --- Top Header Bar ---
        let header_box = GtkBox::new(Orientation::Horizontal, 10);
        let project_label = Label::new(None);
        let p = project.borrow();
        project_label.set_markup(&format!(
            "<big><b>MusicBox Organizer</b></big> — <i>{}</i> ({} tracks)",
            p.metadata.name,
            p.tracks.len()
        ));
        drop(p);
        header_box.pack_start(&project_label, true, true, 0);

        let undo_btn = Button::with_label("↺ Undo");
        let redo_btn = Button::with_label("↻ Redo");
        let theme_btn = Button::with_label("☀ Theme: Dark");
        header_box.pack_end(&theme_btn, false, false, 0);
        header_box.pack_end(&redo_btn, false, false, 0);
        header_box.pack_end(&undo_btn, false, false, 0);
        root_box.pack_start(&header_box, false, false, 0);

        let apply_theme = {
            let css_provider = css_provider.clone();
            let theme_btn = theme_btn.clone();
            move |dark: bool| {
                if let Some(screen) = gtk::gdk::Screen::default() {
                    let css = if dark { DARK_CSS } else { LIGHT_CSS };
                    css_provider.load_from_data(css.as_bytes()).ok();
                    StyleContext::add_provider_for_screen(
                        &screen,
                        &css_provider,
                        STYLE_PROVIDER_PRIORITY_USER,
                    );
                }
                if dark {
                    theme_btn.set_label("☀ Theme: Dark");
                } else {
                    theme_btn.set_label("🌙 Theme: Light");
                }
            }
        };

        // Enforce internal theme immediately on startup (does not follow system theme)
        apply_theme(*is_dark_mode.borrow());

        // --- Action Buttons Toolbar ---
        let toolbar_box = GtkBox::new(Orientation::Horizontal, 6);
        let scan_dir_btn = Button::with_label("📁 Scan Folder...");
        let add_track_btn = Button::with_label("+ Add MP3");
        let move_up_btn = Button::with_label("▲ Up");
        let move_down_btn = Button::with_label("▼ Down");
        let audio_tools_btn = Button::with_label("🎛 Audio Tools (Soon)");
        audio_tools_btn.set_sensitive(false);
        let export_formats_btn = Button::with_label("📄 Export Text/RTF...");
        let save_project_btn = Button::with_label("💾 Save");
        let open_project_btn = Button::with_label("📂 Open");

        toolbar_box.pack_start(&scan_dir_btn, false, false, 0);
        toolbar_box.pack_start(&add_track_btn, false, false, 0);
        toolbar_box.pack_start(&move_up_btn, false, false, 0);
        toolbar_box.pack_start(&move_down_btn, false, false, 0);
        toolbar_box.pack_start(&audio_tools_btn, false, false, 0);
        toolbar_box.pack_end(&save_project_btn, false, false, 0);
        toolbar_box.pack_end(&open_project_btn, false, false, 0);
        toolbar_box.pack_end(&export_formats_btn, false, false, 0);
        root_box.pack_start(&toolbar_box, false, false, 0);

        // --- Playlist Table ---
        let tree_view = TreeView::new();
        tree_view.set_headers_visible(true);

        // Columns: 0=Index, 1=ExportFilename, 2=Title, 3=Artist, 4=FilePath
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

        // --- Audio Preview Bar ---
        let preview_frame = Frame::new(Some("Audio Preview Player"));
        let preview_box = GtkBox::new(Orientation::Horizontal, 8);
        preview_box.set_margin_start(8);
        preview_box.set_margin_end(8);
        preview_box.set_margin_top(4);
        preview_box.set_margin_bottom(4);

        let play_btn = Button::with_label("▶ Play");
        let pause_btn = Button::with_label("⏸ Pause");
        let stop_btn = Button::with_label("⏹ Stop");
        let audio_status_label = Label::new(Some("Select track to preview audio"));
        audio_status_label.set_xalign(0.0);

        preview_box.pack_start(&play_btn, false, false, 0);
        preview_box.pack_start(&pause_btn, false, false, 0);
        preview_box.pack_start(&stop_btn, false, false, 0);
        preview_box.pack_start(&audio_status_label, true, true, 8);

        preview_frame.add(&preview_box);
        root_box.pack_start(&preview_frame, false, false, 0);

        // --- Removable Media Export Frame ---
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
        root_box.pack_start(&export_frame, false, false, 0);

        window.add(&root_box);

        // --- Helper: Synchronize SessionState from ProjectFile ---
        let sync_session_from_project = {
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            move || {
                let p = project.borrow();
                let mut state = session_state.borrow_mut();
                state.tracks = p
                    .tracks
                    .iter()
                    .map(|t| {
                        let mut meta = crate::models::TrackMetadata::default();
                        meta.title = t.title.clone();
                        meta.artist = t.artist.clone();
                        meta.track_number = Some(t.index + 1);
                        let xxyy = naming::track_index_to_xxyy(t.index).unwrap_or_else(|_| "AAAA".into());
                        crate::models::OrganizerTrack::new(
                            t.index,
                            PathBuf::from(&t.file_path),
                            meta,
                            xxyy,
                        )
                    })
                    .collect();
            }
        };

        sync_session_from_project();

        // --- Helper: Refresh Playlist Display ---
        let refresh_playlist_table = {
            let project = Rc::clone(&project);
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

        refresh_playlist_table();

        // --- Helper: Refresh Devices ---
        let update_devices_ui = {
            let dialog_state = Rc::clone(&dialog_state);
            let dev_combo = dev_combo.clone();
            let dev_info_label = dev_info_label.clone();
            let export_btn = export_btn.clone();
            let eject_btn = eject_btn.clone();
            let project = Rc::clone(&project);
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

        // --- Inline Title Editing Handler ---
        {
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            let sync_session = sync_session_from_project.clone();
            let refresh_table = refresh_playlist_table.clone();
            cell_title.connect_edited(move |_, path, new_text| {
                if let Ok(idx) = path.to_string().parse::<usize>() {
                    let mut p = project.borrow_mut();
                    if idx < p.tracks.len() {
                        sync_session();
                        session_state.borrow_mut().push_undo_checkpoint();
                        p.tracks[idx].title = new_text.to_string();
                        drop(p);
                        refresh_table();
                    }
                }
            });
        }

        // --- Inline Artist Editing Handler ---
        {
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            let sync_session = sync_session_from_project.clone();
            let refresh_table = refresh_playlist_table.clone();
            cell_artist.connect_edited(move |_, path, new_text| {
                if let Ok(idx) = path.to_string().parse::<usize>() {
                    let mut p = project.borrow_mut();
                    if idx < p.tracks.len() {
                        sync_session();
                        session_state.borrow_mut().push_undo_checkpoint();
                        p.tracks[idx].artist = new_text.to_string();
                        drop(p);
                        refresh_table();
                    }
                }
            });
        }

        // Theme Toggle (FR-017, FR-018)
        {
            let is_dark_mode = Rc::clone(&is_dark_mode);
            let user_prefs = Rc::clone(&user_prefs);
            let apply_theme = apply_theme.clone();
            theme_btn.connect_clicked(move |_| {
                let mut dark = is_dark_mode.borrow_mut();
                *dark = !*dark;
                apply_theme(*dark);
                let mut prefs = user_prefs.borrow_mut();
                prefs.dark_mode = *dark;
                prefs.save_to_dir(".").ok();
            });
        }

        // Undo Button (FR-012, US5)
        {
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            let refresh_table = refresh_playlist_table.clone();
            let status_label = status_label.clone();
            undo_btn.connect_clicked(move |_| {
                let mut state = session_state.borrow_mut();
                if state.undo() {
                    let mut p = project.borrow_mut();
                    p.tracks = state
                        .tracks
                        .iter()
                        .map(|t| ProjectTrack {
                            index: t.index,
                            title: t.metadata.title.clone(),
                            artist: t.metadata.artist.clone(),
                            file_path: t.file_path.to_string_lossy().to_string(),
                        })
                        .collect();
                    drop(p);
                    refresh_table();
                    status_label.set_text("Reverted last edit (Undo).");
                } else {
                    status_label.set_text("Nothing to undo.");
                }
            });
        }

        // Redo Button (FR-013, US5)
        {
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            let refresh_table = refresh_playlist_table.clone();
            let status_label = status_label.clone();
            redo_btn.connect_clicked(move |_| {
                let mut state = session_state.borrow_mut();
                if state.redo() {
                    let mut p = project.borrow_mut();
                    p.tracks = state
                        .tracks
                        .iter()
                        .map(|t| ProjectTrack {
                            index: t.index,
                            title: t.metadata.title.clone(),
                            artist: t.metadata.artist.clone(),
                            file_path: t.file_path.to_string_lossy().to_string(),
                        })
                        .collect();
                    drop(p);
                    refresh_table();
                    status_label.set_text("Reapplied edit (Redo).");
                } else {
                    status_label.set_text("Nothing to redo.");
                }
            });
        }

        // Move Up Button (FR-005, US3)
        {
            let tree_view = tree_view.clone();
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            let sync_session = sync_session_from_project.clone();
            let refresh_table = refresh_playlist_table.clone();
            move_up_btn.connect_clicked(move |_| {
                let selection = tree_view.selection();
                if let Some((_, iter)) = selection.selected() {
                    let path = tree_view.model().unwrap().path(&iter).unwrap();
                    if let Some(indices) = path.indices().first() {
                        let idx = *indices as usize;
                        if idx > 0 {
                            let new_idx = idx - 1;
                            sync_session();
                            let mut p = project.borrow_mut();
                            session_state.borrow_mut().push_undo_checkpoint();
                            p.tracks.swap(idx, new_idx);
                            for (i, t) in p.tracks.iter_mut().enumerate() {
                                t.index = i as u32;
                            }
                            drop(p);
                            refresh_table();
                            let new_path = gtk::TreePath::from_indicesv(&[new_idx as i32]);
                            tree_view.selection().select_path(&new_path);
                            tree_view.scroll_to_cell(Some(&new_path), None::<&gtk::TreeViewColumn>, false, 0.0, 0.0);
                        }
                    }
                }
            });
        }

        // Move Down Button (FR-005, US3)
        {
            let tree_view = tree_view.clone();
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            let sync_session = sync_session_from_project.clone();
            let refresh_table = refresh_playlist_table.clone();
            move_down_btn.connect_clicked(move |_| {
                let selection = tree_view.selection();
                if let Some((_, iter)) = selection.selected() {
                    let path = tree_view.model().unwrap().path(&iter).unwrap();
                    if let Some(indices) = path.indices().first() {
                        let idx = *indices as usize;
                        let mut p = project.borrow_mut();
                        if idx + 1 < p.tracks.len() {
                            let new_idx = idx + 1;
                            sync_session();
                            session_state.borrow_mut().push_undo_checkpoint();
                            p.tracks.swap(idx, new_idx);
                            for (i, t) in p.tracks.iter_mut().enumerate() {
                                t.index = i as u32;
                            }
                            drop(p);
                            refresh_table();
                            let new_path = gtk::TreePath::from_indicesv(&[new_idx as i32]);
                            tree_view.selection().select_path(&new_path);
                            tree_view.scroll_to_cell(Some(&new_path), None::<&gtk::TreeViewColumn>, false, 0.0, 0.0);
                        }
                    }
                }
            });
        }

        // Scan Folder Button (FR-001, US1)
        {
            let window_clone = window.clone();
            let project = Rc::clone(&project);
            let sync_session = sync_session_from_project.clone();
            let refresh_table = refresh_playlist_table.clone();
            let update_devs = update_devices_ui.clone();
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

        // Audio Preview Play Button (FR-009, FR-010, US4)
        {
            let tree_view = tree_view.clone();
            let project = Rc::clone(&project);
            let audio_player = Rc::clone(&audio_player);
            let audio_status_label = audio_status_label.clone();
            play_btn.connect_clicked(move |_| {
                let selection = tree_view.selection();
                if let Some((_, iter)) = selection.selected() {
                    let path = tree_view.model().unwrap().path(&iter).unwrap();
                    if let Some(indices) = path.indices().first() {
                        let idx = *indices as usize;
                        let p = project.borrow();
                        if let Some(track) = p.tracks.get(idx) {
                            let file_path = PathBuf::from(&track.file_path);
                            let title = track.title.clone();
                            drop(p);
                            let mut player = audio_player.borrow_mut();
                            match player.play_file(&file_path) {
                                Ok(_) => audio_status_label
                                    .set_markup(&format!("▶ <b>Playing:</b> {}", title)),
                                Err(e) => {
                                    audio_status_label.set_text(&format!("Playback error: {}", e))
                                }
                            }
                        }
                    }
                } else {
                    audio_status_label.set_text("Select a track in the table to play.");
                }
            });
        }

        // Pause Button
        {
            let audio_player = Rc::clone(&audio_player);
            let audio_status_label = audio_status_label.clone();
            pause_btn.connect_clicked(move |_| {
                audio_player.borrow().pause();
                audio_status_label.set_text("⏸ Paused");
            });
        }

        // Stop Button
        {
            let audio_player = Rc::clone(&audio_player);
            let audio_status_label = audio_status_label.clone();
            stop_btn.connect_clicked(move |_| {
                audio_player.borrow_mut().stop();
                audio_status_label.set_text("⏹ Stopped");
            });
        }

        // Multi-Format Export Button (FR-004 to FR-010)
        {
            let window_clone = window.clone();
            let project = Rc::clone(&project);
            let user_prefs = Rc::clone(&user_prefs);
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

                let check_txt = CheckButton::with_label("Plain Text (.txt)");
                let check_rtf = CheckButton::with_label("Rich Text Format (.rtf)");

                let current_prefs = user_prefs.borrow();
                check_txt.set_active(current_prefs.formats.txt);
                check_rtf.set_active(current_prefs.formats.rtf);
                drop(current_prefs);

                content_area.pack_start(&check_txt, false, false, 0);
                content_area.pack_start(&check_rtf, false, false, 0);

                // FR-007: Validate at least one format is selected
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

        // Refresh devices button click
        {
            let update_devices = update_devices_ui.clone();
            refresh_btn.connect_clicked(move |_| {
                update_devices();
            });
        }

        // Add track button click
        {
            let window_clone = window.clone();
            let project = Rc::clone(&project);
            let sync_session = sync_session_from_project.clone();
            let refresh_table = refresh_playlist_table.clone();
            let update_devs = update_devices_ui.clone();
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

        // Save Project button click
        {
            let window_clone = window.clone();
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
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
                        // Commit updated ID3 tags to local files per FR-007
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

        // Open Project button click
        {
            let window_clone = window.clone();
            let project = Rc::clone(&project);
            let sync_session = sync_session_from_project.clone();
            let refresh_table = refresh_playlist_table.clone();
            let update_devs = update_devices_ui.clone();
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

        // Export Button Click
        {
            let window_clone = window.clone();
            let project = Rc::clone(&project);
            let dialog_state = Rc::clone(&dialog_state);
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

                // Two-step reformat warning confirmation (FR-004, SC-002)
                if state.reformat_enabled {
                    let dev = match state.selected_device() {
                        Some(d) => d,
                        None => {
                            status_label.set_text("No target device selected.");
                            return;
                        }
                    };
                    let warning_ctrl = ReformatWarningDialog::new(dev.name.clone(), dev.capacity_bytes);

                    // Modal dialog with Cancel and Confirm only
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

        // Eject Button Click
        {
            let dialog_state = Rc::clone(&dialog_state);
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

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::glib::signal::Inhibit(false)
        });

        window.show_all();
        gtk::main();
    }
}
