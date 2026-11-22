use crate::models::{PlaylistMetadata, ProjectFile, SessionState};
use crate::services::AudioPreviewPlayer;
use crate::ui::dsp_dialog::DspDialogState;
use crate::ui::export_catalog_dialog::ExportCatalogDialog;
use crate::ui::export_panel::ExportPanelState;
use crate::ui::{DspProgressMessage, ExportProgressMessage};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableColumn {
    TrackNumber,
    ExportFilename,
    Title,
    Artist,
    SourceFile,
}

pub struct MusicBoxApp {
    // Core Domain Data
    pub project: ProjectFile,
    pub session: SessionState,
    pub project_path: Option<PathBuf>,

    // Playlist Table Interaction
    pub selected_track_indices: BTreeSet<usize>,
    pub editing_cell: Option<(usize, TableColumn)>,

    // Audio Preview Player
    pub preview_player: Option<AudioPreviewPlayer>,
    pub playing_track_idx: Option<usize>,

    // Sub-dialog & Panel States
    pub dsp_dialog: DspDialogState,
    pub export_panel: ExportPanelState,
    pub export_catalog_dialog: ExportCatalogDialog,

    // Global Visuals & Status
    pub dark_mode: bool,
    pub status_message: String,

    // Concurrency Channels (Workers -> UI)
    pub dsp_rx: Option<Receiver<DspProgressMessage>>,
    pub export_rx: Option<Receiver<ExportProgressMessage>>,

    // Visual Logo Texture
    pub logo_texture: Option<egui::TextureHandle>,
}

pub const LOGO_PNG_BYTES: &[u8] = include_bytes!("../../img/logo/png/logo_mbo_128.png");

pub fn load_app_icon() -> Option<egui::IconData> {
    let img = image::load_from_memory(LOGO_PNG_BYTES).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    Some(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    })
}

impl Default for MusicBoxApp {
    fn default() -> Self {
        let project = ProjectFile::new(PlaylistMetadata {
            name: "Default Playlist".to_string(),
            author: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
        });
        Self {
            project,
            session: SessionState::default(),
            project_path: None,
            selected_track_indices: BTreeSet::new(),
            editing_cell: None,
            preview_player: None,
            playing_track_idx: None,
            dsp_dialog: DspDialogState::default(),
            export_panel: ExportPanelState::default(),
            export_catalog_dialog: ExportCatalogDialog::default(),
            dark_mode: true,
            status_message: "Ready".to_string(),
            dsp_rx: None,
            export_rx: None,
            logo_texture: None,
        }
    }
}

impl MusicBoxApp {
    pub fn new(project: ProjectFile, project_path: Option<PathBuf>) -> Self {
        let mut app = Self {
            project,
            project_path,
            ..Default::default()
        };
        if app.project.tracks.is_empty() && app.project_path.is_none() {
            app.auto_load_current_dir();
        }
        app.refresh_devices();
        app
    }

    pub fn auto_load_current_dir(&mut self) {
        if let Ok(cwd) = std::env::current_dir() {
            let default_playlist = cwd.join("playlist.txt");
            if default_playlist.exists()
                && let Ok(proj) = ProjectFile::load(&default_playlist)
            {
                self.project = proj;
                self.project_path = Some(default_playlist);
                self.status_message = "Auto-loaded playlist.txt".to_string();
                return;
            }
            let scanner = crate::services::DirectoryScanner::new();
            if let Ok(found) = scanner.scan_directory(&cwd)
                && !found.is_empty()
            {
                self.project.add_scanned_tracks(found);
                self.status_message = format!(
                    "Auto-scanned {} tracks from {}",
                    self.project.tracks.len(),
                    cwd.display()
                );
            }
        }
    }

    pub fn refresh_devices(&mut self) {
        crate::ui::export_panel::scan_devices(self);
    }

    pub fn get_logo_texture(&mut self, ctx: &egui::Context) -> Option<&egui::TextureHandle> {
        if self.logo_texture.is_none()
            && let Ok(img) = image::load_from_memory(LOGO_PNG_BYTES)
        {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                &rgba.into_raw(),
            );
            self.logo_texture =
                Some(ctx.load_texture("app_logo", color_image, egui::TextureOptions::LINEAR));
        }
        self.logo_texture.as_ref()
    }

    pub fn render_central_content(&mut self, ui: &mut egui::Ui) {
        crate::ui::header::render_header(self, ui);
        ui.separator();
        crate::ui::toolbar::render_toolbar(self, ui);
        ui.separator();
        crate::ui::playlist_table::render_playlist_table(self, ui);
        ui.separator();
        crate::ui::audio_bar::render_audio_bar(self, ui);
        ui.separator();
        crate::ui::export_panel::render_export_panel(self, ui);
        ui.separator();
        crate::ui::footer::render_footer(self, ui);
        crate::ui::dsp_dialog::render_dsp_dialog(self, ui.ctx());
        crate::ui::export_catalog_dialog::render_export_catalog_dialog(self, ui.ctx());
    }

    pub fn poll_dsp_channel(&mut self) {
        let msgs: Vec<DspProgressMessage> = if let Some(rx) = &self.dsp_rx {
            let mut list = Vec::new();
            while let Ok(msg) = rx.try_recv() {
                list.push(msg);
            }
            list
        } else {
            Vec::new()
        };

        let mut done = false;
        for msg in msgs {
            match msg {
                DspProgressMessage::Started { total_tracks } => {
                    self.dsp_dialog.is_processing = true;
                    self.dsp_dialog.progress_percent = 0.0;
                    self.dsp_dialog.progress_text = format!("Processing 0/{}...", total_tracks);
                }
                DspProgressMessage::Step {
                    current,
                    total,
                    filename,
                } => {
                    self.dsp_dialog.progress_percent = current as f32 / total.max(1) as f32;
                    self.dsp_dialog.progress_text = format!("({}/{}) {}", current, total, filename);
                }
                DspProgressMessage::Finished {
                    succeeded,
                    failed,
                    errors,
                } => {
                    self.dsp_dialog.is_processing = false;
                    self.dsp_dialog.is_open = false;
                    self.status_message =
                        format!("DSP complete: {} updated, {} failed", succeeded, failed);
                    if !errors.is_empty() {
                        self.dsp_dialog.error_summary = Some(errors.join("\n"));
                    }
                    done = true;
                }
                DspProgressMessage::Failed(err) => {
                    self.dsp_dialog.is_processing = false;
                    self.dsp_dialog.error_summary = Some(err);
                    done = true;
                }
            }
        }
        if done {
            self.dsp_rx = None;
        }
    }

    pub fn poll_export_channel(&mut self) {
        let msgs: Vec<ExportProgressMessage> = if let Some(rx) = &self.export_rx {
            let mut list = Vec::new();
            while let Ok(msg) = rx.try_recv() {
                list.push(msg);
            }
            list
        } else {
            Vec::new()
        };

        let mut done = false;
        for msg in msgs {
            match msg {
                ExportProgressMessage::Started { total_tracks } => {
                    self.export_panel.is_exporting = true;
                    self.export_panel.export_progress = 0.0;
                    self.export_panel.export_status = format!("Exporting 0/{}...", total_tracks);
                }
                ExportProgressMessage::Step {
                    current,
                    total,
                    filename,
                } => {
                    self.export_panel.export_progress = current as f32 / total.max(1) as f32;
                    self.export_panel.export_status =
                        format!("({}/{}) {}", current, total, filename);
                }
                ExportProgressMessage::Finished { total_bytes } => {
                    self.export_panel.is_exporting = false;
                    let mb = total_bytes as f64 / (1024.0 * 1024.0);
                    self.export_panel.export_status = format!("Export completed ({:.1} MB)", mb);
                    self.status_message = format!("Export successful: {:.1} MB written", mb);
                    done = true;
                }
                ExportProgressMessage::Failed(err) => {
                    self.export_panel.is_exporting = false;
                    self.export_panel.export_status = format!("Export failed: {}", err);
                    done = true;
                }
            }
        }
        if done {
            self.export_rx = None;
        }
    }
}

impl eframe::App for MusicBoxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        ctx.style_mut(|style| {
            style.spacing.button_padding = egui::vec2(5.0, 2.0);
            style.spacing.item_spacing = egui::vec2(4.0, 4.0);
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(12.5, egui::FontFamily::Proportional),
            );
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(12.5, egui::FontFamily::Proportional),
            );
        });

        self.poll_dsp_channel();
        if self.dsp_dialog.is_processing {
            ctx.request_repaint();
        }

        self.poll_export_channel();
        if self.export_panel.is_exporting {
            ctx.request_repaint();
        }

        // Top Panels: Header & Toolbar
        egui::TopBottomPanel::top("top_header_panel").show(ctx, |ui| {
            crate::ui::header::render_header(self, ui);
            ui.separator();
            crate::ui::toolbar::render_toolbar(self, ui);
        });

        // Bottom Panels: Footer & Audio Bar & Export Panel
        egui::TopBottomPanel::bottom("bottom_status_panel").show(ctx, |ui| {
            crate::ui::footer::render_footer(self, ui);
        });

        egui::TopBottomPanel::bottom("bottom_audio_panel").show(ctx, |ui| {
            crate::ui::audio_bar::render_audio_bar(self, ui);
            ui.separator();
            crate::ui::export_panel::render_export_panel(self, ui);
        });

        // Central Panel: Playlist Table
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::playlist_table::render_playlist_table(self, ui);
        });

        // Modals / Floating Windows
        crate::ui::dsp_dialog::render_dsp_dialog(self, ctx);
        crate::ui::export_catalog_dialog::render_export_catalog_dialog(self, ctx);
    }
}

pub fn run_gui() {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([960.0, 680.0])
        .with_min_inner_size([300.0, 400.0])
        .with_title("MusicBox Organizer");

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "MusicBox Organizer",
        native_options,
        Box::new(|_cc| Ok(Box::new(MusicBoxApp::new(ProjectFile::default(), None)))),
    );
}
