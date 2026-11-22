pub mod app;
pub mod audio_bar;
pub mod dsp_dialog;
pub mod export_catalog_dialog;
pub mod export_dialog;
pub mod export_panel;
pub mod footer;
pub mod header;
pub mod playlist_table;
pub mod toolbar;
pub mod warnings;

pub use app::{MusicBoxApp, TableColumn, run_gui};
pub use dsp_dialog::{DspDialogState, render_dsp_dialog, render_dsp_dialog_contents};
pub use export_catalog_dialog::{
    ExportCatalogDialog, render_export_catalog_contents, render_export_catalog_dialog,
};
pub use export_dialog::ExportDialog;
pub use export_panel::{
    ExportPanelState, render_export_confirm_contents, render_export_confirm_dialog,
};
pub use warnings::ReformatWarningDialog;

#[derive(Debug, Clone)]
pub enum DspProgressMessage {
    Started {
        total_tracks: usize,
    },
    Step {
        current: usize,
        total: usize,
        filename: String,
    },
    Finished {
        succeeded: usize,
        failed: usize,
        errors: Vec<String>,
    },
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum ExportProgressMessage {
    Started {
        total_tracks: usize,
    },
    Step {
        current: usize,
        total: usize,
        filename: String,
    },
    Finished {
        total_bytes: u64,
    },
    Failed(String),
}
