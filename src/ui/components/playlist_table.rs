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
) -> PlaylistTableComponent {
    let root_box = GtkBox::new(Orientation::Vertical, 6);

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

    // --- TreeView Table ---
    let tree_view = TreeView::new();
    tree_view.set_headers_visible(true);

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

    // Inline title edit
    {
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
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

    // Inline artist edit
    {
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
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

    // Move Up
    {
        let tree_view = tree_view.clone();
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
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

    // Move Down
    {
        let tree_view = tree_view.clone();
        let project = Rc::clone(project);
        let session_state = Rc::clone(session_state);
        let sync_session = Rc::clone(&sync_session);
        let refresh_table = refresh_table.clone();
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
    }
}
