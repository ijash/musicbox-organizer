use crate::models::{OrganizerTrack, TrackMetadata};
use crate::services::naming;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct DirectoryScanner;

impl DirectoryScanner {
    pub fn new() -> Self {
        Self
    }

    /// Scan directory non-recursively for .mp3 files (FR-001)
    pub fn scan_directory(&self, dir: &Path) -> Result<Vec<OrganizerTrack>, Box<dyn Error>> {
        let mut entries = Vec::new();

        if !dir.is_dir() {
            return Err(format!("'{}' is not a directory", dir.display()).into());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().eq_ignore_ascii_case("mp3") {
                        entries.push(path);
                    }
                }
            }
        }

        // Sort by filename for deterministic initial order
        entries.sort_by(|a, b| {
            a.file_name()
                .unwrap_or_default()
                .cmp(&b.file_name().unwrap_or_default())
        });

        let mut tracks = Vec::with_capacity(entries.len());
        for (idx, path) in entries.into_iter().enumerate() {
            let index = idx as u32;
            let xxyy = naming::track_index_to_xxyy(index).unwrap_or_else(|_| "AAAA".to_string());
            let title = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| format!("Track {}", index));

            let metadata = TrackMetadata {
                title,
                artist: String::new(),
                album: String::new(),
                track_number: Some(index + 1),
                year: None,
                duration_secs: 0,
            };

            tracks.push(OrganizerTrack::new(index, path, metadata, xxyy));
        }

        Ok(tracks)
    }
}
