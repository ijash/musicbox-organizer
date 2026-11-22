use crate::models::TrackMetadata;
use crate::services::naming;
use id3::{Tag, TagLike, Version};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Id3Service;

impl Default for Id3Service {
    fn default() -> Self {
        Self::new()
    }
}

impl Id3Service {
    pub fn new() -> Self {
        Self
    }

    /// Read ID3 tags from file (ID3v2 or fallback)
    pub fn read_tags(&self, path: &Path) -> Result<TrackMetadata, Box<dyn Error>> {
        let tag = Tag::read_from_path(path).ok();

        let title = tag
            .as_ref()
            .and_then(|t| t.title())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                path.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown Title".to_string())
            });

        let artist = tag
            .as_ref()
            .and_then(|t| t.artist())
            .unwrap_or("Unknown Artist")
            .to_string();

        let album = tag
            .as_ref()
            .and_then(|t| t.album())
            .unwrap_or("Unknown Album")
            .to_string();

        let track_number = tag.as_ref().and_then(|t| t.track());
        let year = tag.as_ref().and_then(|t| t.year());

        Ok(TrackMetadata {
            track_number,
            title,
            artist,
            album,
            year,
            duration_secs: 0,
        })
    }

    /// Write ID3v2.4 tags to file
    pub fn write_tags(&self, path: &Path, metadata: &TrackMetadata) -> Result<(), Box<dyn Error>> {
        let mut tag = Tag::read_from_path(path).unwrap_or_else(|_| Tag::new());

        tag.set_title(&metadata.title);
        tag.set_artist(&metadata.artist);
        tag.set_album(&metadata.album);

        if let Some(track) = metadata.track_number {
            tag.set_track(track);
        } else {
            tag.remove_track();
        }

        if let Some(year) = metadata.year {
            tag.set_year(year);
        } else {
            tag.remove_year();
        }

        tag.write_to_path(path, Version::Id3v24)?;
        Ok(())
    }

    /// Rename file to [XXYY-original.mp3] format (FR-008)
    pub fn apply_xxyy_rename(
        &self,
        path: &Path,
        track_index: u32,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let filename = path
            .file_name()
            .ok_or_else(|| format!("Invalid path: {}", path.display()))?
            .to_string_lossy();

        // Strip existing [XXXX-] or XXXX- prefix if present so we don't nest prefixes
        let clean_filename = if filename.starts_with('[') {
            if filename.ends_with(']') {
                let inner = &filename[1..filename.len() - 1];
                if inner.len() >= 5
                    && inner.as_bytes()[..4].iter().all(|b| b.is_ascii_uppercase())
                    && inner.as_bytes()[4] == b'-'
                {
                    &inner[5..]
                } else if let Some(dash) = inner.find('-') {
                    &inner[dash + 1..]
                } else {
                    inner
                }
            } else if let Some(end_bracket) = filename.find(']') {
                let rest = &filename[end_bracket + 1..];
                rest.trim_start_matches('-')
            } else {
                &filename
            }
        } else if filename.len() >= 5
            && filename.as_bytes()[..4]
                .iter()
                .all(|b| b.is_ascii_uppercase())
            && filename.as_bytes()[4] == b'-'
        {
            &filename[5..]
        } else {
            &filename
        };

        let new_filename =
            naming::format_export_filename(track_index, clean_filename.trim_start_matches('-'))
                .map_err(|e| format!("Failed to generate sequence: {}", e))?;

        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let target_path = parent.join(&new_filename);

        fs::rename(path, &target_path)?;
        Ok(target_path)
    }
}
