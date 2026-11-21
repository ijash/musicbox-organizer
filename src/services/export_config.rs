use crate::models::{ExportFormatSelection, OrganizerTrack, ProjectFile, TrackMetadata};
use crate::services::RtfExporter;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ExportConfigService;

impl Default for ExportConfigService {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportConfigService {
    pub fn new() -> Self {
        Self
    }

    /// Export playlist to playlist-text directory with clear-all behavior (FR-008, FR-009, FR-010)
    pub fn export_playlist(
        &self,
        base_dir: &Path,
        selection: &ExportFormatSelection,
        project: &ProjectFile,
    ) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        if !selection.is_valid() {
            return Err("At least one export format must be selected".into());
        }

        let target_dir = base_dir.join("playlist-text");

        // Clear-all behavior: delete existing files in playlist-text (FR-008)
        if target_dir.exists() {
            for entry in fs::read_dir(&target_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    fs::remove_file(path)?;
                }
            }
        } else {
            fs::create_dir_all(&target_dir)?;
        }

        let mut written_files = Vec::new();

        // 1. Export TXT
        if selection.txt {
            let txt_path = target_dir.join("playlist.txt");
            project
                .save(&txt_path)
                .map_err(|e| format!("Failed to write playlist.txt: {}", e))?;
            written_files.push(txt_path);
        }

        // 2. Export RTF
        if selection.rtf {
            let rtf_path = target_dir.join("playlist.rtf");
            let organizer_tracks: Vec<OrganizerTrack> = project
                .tracks
                .iter()
                .map(|t| {
                    let meta = TrackMetadata {
                        title: t.title.clone(),
                        artist: t.artist.clone(),
                        ..Default::default()
                    };
                    OrganizerTrack::new(t.index, PathBuf::from(&t.file_path), meta, "AAAA".into())
                })
                .collect();
            RtfExporter::export_to_file(&rtf_path, &project.metadata.name, &organizer_tracks)?;
            written_files.push(rtf_path);
        }

        Ok(written_files)
    }
}
