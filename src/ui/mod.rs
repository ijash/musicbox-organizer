pub mod app;
pub mod audio_bar;
pub mod dsp_dialog;
pub mod export_dialog;
pub mod export_panel;
pub mod footer;
pub mod header;
pub mod playlist_table;
pub mod toolbar;
pub mod warnings;

pub use app::{run_gui, MusicBoxApp, TableColumn};
pub use dsp_dialog::DspDialogState;
pub use export_dialog::ExportDialog;
pub use export_panel::ExportPanelState;
pub use warnings::ReformatWarningDialog;

#[derive(Debug, Clone)]
pub enum DspProgressMessage {
    Started { total_tracks: usize },
    Step { current: usize, total: usize, filename: String },
    Finished { succeeded: usize, failed: usize, errors: Vec<String> },
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum ExportProgressMessage {
    Started { total_tracks: usize },
    Step { current: usize, total: usize, filename: String },
    Finished { total_bytes: u64 },
    Failed(String),
}


