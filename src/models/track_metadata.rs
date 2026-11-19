use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// ID3 metadata extracted from or written to an audio file
/// cite: data-model.md TrackMetadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TrackMetadata {
    pub track_number: Option<u32>,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: Option<i32>,
    pub duration_secs: u64,
}

/// Organizer playlist track representation
/// cite: data-model.md OrganizerTrack
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizerTrack {
    pub index: u32,
    pub file_path: PathBuf,
    pub original_filename: String,
    pub xxyy_prefix: String,
    pub metadata: TrackMetadata,
    pub is_modified: bool,
}

impl OrganizerTrack {
    pub fn new(index: u32, file_path: PathBuf, metadata: TrackMetadata, xxyy_prefix: String) -> Self {
        let original_filename = file_path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("track_{}.mp3", index));

        Self {
            index,
            file_path,
            original_filename,
            xxyy_prefix,
            metadata,
            is_modified: false,
        }
    }

    pub fn formatted_filename(&self) -> String {
        format!("[{}-{}]", self.xxyy_prefix, self.original_filename)
    }
}
