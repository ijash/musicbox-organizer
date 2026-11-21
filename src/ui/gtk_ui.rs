#[cfg(target_os = "linux")]
pub mod linux {
    use gtk::prelude::*;
    use gtk::{
        Box as GtkBox, CssProvider, Orientation, Window, WindowPosition, WindowType,
    };
    use crate::models::{PlaylistMetadata, ProjectFile, SessionState};
    use crate::services::{naming, AudioPreviewPlayer};
    use crate::ui::components::{
        build_audio_bar, build_export_panel, build_footer, build_header, build_playlist_table,
    };
    use crate::ui::theme;
    use crate::ui::ExportDialog;
    use std::cell::RefCell;
    use std::io::Cursor;
    use std::path::PathBuf;
    use std::rc::Rc;

    const APP_LOGO_PNG: &[u8] = include_bytes!("../../img/logo/png/logo_mbo_128.png");

    pub fn run_gtk_app() {
        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK.");
            return;
        }

        // Shared application state
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

        // Main Window setup (740x520 default)
        let window = Window::new(WindowType::Toplevel);
        window.set_title("MusicBox Organizer");
        window.set_default_size(740, 520);
        window.set_position(WindowPosition::Center);

        if let Ok(pixbuf) = gtk::gdk_pixbuf::Pixbuf::from_read(Cursor::new(APP_LOGO_PNG)) {
            window.set_icon(Some(&pixbuf));
        }

        let root_box = GtkBox::new(Orientation::Vertical, 6);
        root_box.set_margin_start(8);
        root_box.set_margin_end(8);
        root_box.set_margin_top(8);
        root_box.set_margin_bottom(8);

        // Helper: Synchronize SessionState from ProjectFile
        let sync_session_from_project = {
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            Rc::new(move || {
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
            })
        };

        sync_session_from_project();

        // 1. Build Export Panel Component
        let export_comp = build_export_panel(&window, &project, &dialog_state);

        // Callback for device refresh
        let update_devices_ui = {
            let refresh_btn = export_comp.container.clone();
            Rc::new(move || {
                let _ = &refresh_btn;
            })
        };

        // 2. Build Header Component with undo/redo callbacks
        let project_name = project.borrow().metadata.name.clone();
        let track_count = project.borrow().tracks.len();
        let status_label = export_comp.status_label.clone();

        let (undo_cb, redo_cb) = {
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            let status_label = status_label.clone();
            (
                {
                    let project = Rc::clone(&project);
                    let session_state = Rc::clone(&session_state);
                    let status_label = status_label.clone();
                    move || {
                        let mut state = session_state.borrow_mut();
                        if state.undo() {
                            let mut p = project.borrow_mut();
                            p.tracks = state
                                .tracks
                                .iter()
                                .map(|t| crate::models::ProjectTrack {
                                    index: t.index,
                                    title: t.metadata.title.clone(),
                                    artist: t.metadata.artist.clone(),
                                    file_path: t.file_path.to_string_lossy().to_string(),
                                })
                                .collect();
                            status_label.set_text("Reverted last edit (Undo).");
                        } else {
                            status_label.set_text("Nothing to undo.");
                        }
                    }
                },
                {
                    let project = Rc::clone(&project);
                    let session_state = Rc::clone(&session_state);
                    let status_label = status_label.clone();
                    move || {
                        let mut state = session_state.borrow_mut();
                        if state.redo() {
                            let mut p = project.borrow_mut();
                            p.tracks = state
                                .tracks
                                .iter()
                                .map(|t| crate::models::ProjectTrack {
                                    index: t.index,
                                    title: t.metadata.title.clone(),
                                    artist: t.metadata.artist.clone(),
                                    file_path: t.file_path.to_string_lossy().to_string(),
                                })
                                .collect();
                            status_label.set_text("Reapplied edit (Redo).");
                        } else {
                            status_label.set_text("Nothing to redo.");
                        }
                    }
                },
            )
        };

        let header_comp = build_header(
            &project_name,
            track_count,
            &css_provider,
            &is_dark_mode,
            &user_prefs,
            undo_cb,
            redo_cb,
        );

        // Apply theme immediately
        theme::apply_theme(&css_provider, &header_comp.theme_btn, *is_dark_mode.borrow());

        // 3. Build Playlist Table Component
        let table_comp = build_playlist_table(
            &window,
            &project,
            &session_state,
            &user_prefs,
            &export_comp.status_label,
            &header_comp.project_label,
            sync_session_from_project,
            update_devices_ui,
            &audio_player,
        );

        // 4. Build Audio Bar Component
        let audio_comp = build_audio_bar(&table_comp.tree_view, &project, &audio_player);

        // 5. Build Footer Component
        let footer_comp = build_footer();

        // Pack all components into root container
        root_box.pack_start(&header_comp.container, false, false, 0);
        root_box.pack_start(&table_comp.container, true, true, 0);
        root_box.pack_start(&audio_comp.container, false, false, 0);
        root_box.pack_start(&export_comp.container, false, false, 0);
        root_box.pack_start(&footer_comp.container, false, false, 0);

        window.add(&root_box);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::glib::signal::Inhibit(false)
        });

        window.show_all();
        gtk::main();
    }
}
