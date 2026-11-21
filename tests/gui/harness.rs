use musicbox_organizer::models::{PlaylistMetadata, ProjectFile, ProjectTrack, SessionState};
use musicbox_organizer::ui::components::*;
use musicbox_organizer::ui::theme;
use musicbox_organizer::ui::ExportDialog;
use gtk::prelude::*;
use gtk::{Box as GtkBox, CssProvider, Orientation, Window, WindowPosition, WindowType};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct GuiHarness {
    pub window: Window,
    pub project: Rc<RefCell<ProjectFile>>,
    pub session_state: Rc<RefCell<SessionState>>,
    pub header: HeaderComponent,
    pub table: PlaylistTableComponent,
    pub audio: AudioBarComponent,
    pub export: ExportPanelComponent,
    pub footer: FooterComponent,
    pub audio_player: Rc<RefCell<musicbox_organizer::services::AudioPreviewPlayer>>,
}

impl GuiHarness {
    pub fn new() -> Self {
        if !gtk::is_initialized() {
            gtk::init().expect("Failed to initialize GTK in test environment");
        }

        let project = Rc::new(RefCell::new(ProjectFile::new(PlaylistMetadata {
            name: "Test Playlist".to_string(),
            author: Some("Tester".to_string()),
            created_at: Some("2026-11-20T00:00:00Z".to_string()),
        })));

        let session_state = Rc::new(RefCell::new(SessionState::new(PathBuf::from("."))));
        let dialog_state = Rc::new(RefCell::new(ExportDialog::new()));
        let audio_player = Rc::new(RefCell::new(musicbox_organizer::services::AudioPreviewPlayer::new()));
        let user_prefs = Rc::new(RefCell::new(musicbox_organizer::models::UserPreferences::default()));
        let is_dark_mode = Rc::new(RefCell::new(true));
        let css_provider = CssProvider::new();

        let window = Window::new(WindowType::Toplevel);
        window.set_title("Test Window");
        window.set_default_size(740, 520);
        window.set_position(WindowPosition::Center);

        let root_box = GtkBox::new(Orientation::Vertical, 6);

        let sync_session = {
            let project = Rc::clone(&project);
            let session_state = Rc::clone(&session_state);
            Rc::new(move || {
                let p = project.borrow();
                let mut state = session_state.borrow_mut();
                state.tracks = p
                    .tracks
                    .iter()
                    .map(|t| {
                        let mut meta = musicbox_organizer::models::TrackMetadata::default();
                        meta.title = t.title.clone();
                        meta.artist = t.artist.clone();
                        meta.track_number = Some(t.index + 1);
                        let xxyy = musicbox_organizer::services::naming::track_index_to_xxyy(t.index).unwrap_or_else(|_| "AAAA".into());
                        musicbox_organizer::models::OrganizerTrack::new(
                            t.index,
                            PathBuf::from(&t.file_path),
                            meta,
                            xxyy,
                        )
                    })
                    .collect();
            })
        };

        let export = build_export_panel(&window, &project, &dialog_state);
        let update_devices_ui = Rc::new(|| {});

        let header = build_header(
            "Test Playlist",
            0,
            &css_provider,
            &is_dark_mode,
            &user_prefs,
            || {},
            || {},
        );

        theme::apply_theme(&css_provider, &header.theme_btn, true);

        let table = build_playlist_table(
            &window,
            &project,
            &session_state,
            &user_prefs,
            &export.status_label,
            &header.project_label,
            sync_session,
            update_devices_ui,
            &audio_player,
        );

        let audio = build_audio_bar(&table.tree_view, &project, &audio_player);
        let footer = build_footer();

        root_box.pack_start(&header.container, false, false, 0);
        root_box.pack_start(&table.container, true, true, 0);
        root_box.pack_start(&audio.container, false, false, 0);
        root_box.pack_start(&export.container, false, false, 0);
        root_box.pack_start(&footer.container, false, false, 0);

        window.add(&root_box);
        window.show_all();

        let harness = Self {
            window,
            project,
            session_state,
            header,
            table,
            audio,
            export,
            footer,
            audio_player,
        };
        harness.pump_events();
        harness
    }

    pub fn pump_events(&self) {
        while gtk::events_pending() {
            gtk::main_iteration_do(false);
        }
    }

    pub fn add_track(&self, title: &str, artist: &str, path: &Path) {
        let mut p = self.project.borrow_mut();
        let idx = p.tracks.len() as u32;
        p.tracks.push(ProjectTrack {
            index: idx,
            title: title.to_string(),
            artist: artist.to_string(),
            file_path: path.to_string_lossy().to_string(),
        });
        drop(p);
        self.pump_events();
    }
}
