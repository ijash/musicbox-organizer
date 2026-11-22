use crate::models::{CatalogFieldSelection, OrganizerTrack};
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct RtfExporter;

impl RtfExporter {
    pub fn generate_rtf(playlist_name: &str, tracks: &[OrganizerTrack]) -> String {
        Self::generate_rtf_with_fields(
            playlist_name,
            tracks,
            &CatalogFieldSelection {
                track_number: true,
                title: true,
                artist: true,
                export_filename: false,
                header_title: true,
            },
        )
    }

    pub fn generate_rtf_with_fields(
        playlist_name: &str,
        tracks: &[OrganizerTrack],
        fields: &CatalogFieldSelection,
    ) -> String {
        let mut rtf = String::new();
        rtf.push_str("{\\rtf1\\ansi\\deff0\n");
        rtf.push_str("{\\fonttbl{\\f0\\fnil\\fcharset0 Helvetica;}}\n");
        rtf.push_str("{\\colortbl ;\\red0\\green0\\blue0;\\red100\\green100\\blue100;}\n");

        if fields.header_title {
            rtf.push_str("\\fs22\\b ");
            rtf.push_str(&Self::escape(playlist_name));
            rtf.push_str("\\b0\\fs18\\par\\par\n");
        } else {
            rtf.push_str("\\fs18\n");
        }

        for (idx, t) in tracks.iter().enumerate() {
            let mut parts = Vec::new();
            if fields.track_number {
                parts.push(format!("\\b {:02}.\\b0", idx + 1));
            }
            if fields.artist && !t.metadata.artist.trim().is_empty() {
                parts.push(Self::escape(&t.metadata.artist));
            }
            if fields.title {
                let title = if t.metadata.title.is_empty() {
                    &t.original_filename
                } else {
                    &t.metadata.title
                };
                if fields.artist && !t.metadata.artist.trim().is_empty() {
                    parts.push(format!("- {}", Self::escape(title)));
                } else {
                    parts.push(Self::escape(title));
                }
            }
            if fields.export_filename {
                parts.push(format!("({})", Self::escape(&t.formatted_filename())));
            }

            let line = if parts.is_empty() {
                format!("{:02}. Track {}\\par\n", idx + 1, idx + 1)
            } else {
                format!("{}\\par\n", parts.join(" "))
            };
            rtf.push_str(&line);
        }

        rtf.push_str("}\n");
        rtf
    }

    pub fn export_to_file(
        path: &Path,
        playlist_name: &str,
        tracks: &[OrganizerTrack],
    ) -> Result<(), Box<dyn Error>> {
        let content = Self::generate_rtf(playlist_name, tracks);
        fs::write(path, content)?;
        Ok(())
    }

    pub fn export_to_file_with_fields(
        path: &Path,
        playlist_name: &str,
        tracks: &[OrganizerTrack],
        fields: &CatalogFieldSelection,
    ) -> Result<(), Box<dyn Error>> {
        let content = Self::generate_rtf_with_fields(playlist_name, tracks, fields);
        fs::write(path, content)?;
        Ok(())
    }

    fn escape(text: &str) -> String {
        text.replace('\\', "\\\\")
            .replace('{', "\\{")
            .replace('}', "\\}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TrackMetadata;
    use std::path::PathBuf;

    #[test]
    fn test_rtf_generation() {
        let tracks = vec![OrganizerTrack::new(
            0,
            PathBuf::from("track1.mp3"),
            TrackMetadata {
                title: "My Song".into(),
                artist: "Cool Band".into(),
                ..Default::default()
            },
            "AAAA".into(),
        )];

        let rtf = RtfExporter::generate_rtf("My Playlist", &tracks);
        assert!(rtf.starts_with("{\\rtf1"));
        assert!(rtf.contains("Cool Band - My Song"));
    }
}
