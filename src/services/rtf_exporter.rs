use crate::models::OrganizerTrack;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct RtfExporter;

impl RtfExporter {
    pub fn generate_rtf(playlist_name: &str, tracks: &[OrganizerTrack]) -> String {
        let mut rtf = String::new();
        rtf.push_str("{\\rtf1\\ansi\\deff0\n");
        rtf.push_str("{\\fonttbl{\\f0\\fnil\\fcharset0 Helvetica;}}\n");
        rtf.push_str("{\\colortbl ;\\red0\\green0\\blue0;\\red100\\green100\\blue100;}\n");
        rtf.push_str("\\fs22\\b ");
        rtf.push_str(&Self::escape(playlist_name));
        rtf.push_str("\\b0\\fs18\\par\\par\n");

        for t in tracks {
            let num = t.index + 1;
            let title = if t.metadata.title.is_empty() {
                &t.original_filename
            } else {
                &t.metadata.title
            };
            let artist = if t.metadata.artist.is_empty() {
                "Unknown Artist"
            } else {
                &t.metadata.artist
            };
            let line = format!(
                "\\b {:02}.\\b0  {} - {}\\par\n",
                num,
                Self::escape(artist),
                Self::escape(title)
            );
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
