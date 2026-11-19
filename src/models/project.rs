use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistMetadata {
    pub name: String,
    pub author: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub metadata: PlaylistMetadata,
    pub tracks: Vec<ProjectTrack>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTrack {
    pub index: u32,
    pub title: String,
    pub artist: String,
    pub file_path: String,
}

impl ProjectFile {
    pub fn new(metadata: PlaylistMetadata) -> Self {
        ProjectFile {
            metadata,
            tracks: Vec::new(),
            version: "1.0".to_string(),
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::parse(&content)
    }

    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), String> {
        let content = self.serialize();
        std::fs::write(path, content).map_err(|e| e.to_string())
    }

    pub fn add_scanned_tracks(&mut self, tracks: Vec<crate::models::OrganizerTrack>) {
        for track in tracks {
            let next_idx = self.tracks.len() as u32;
            self.tracks.push(ProjectTrack {
                index: next_idx,
                title: track.metadata.title,
                artist: track.metadata.artist,
                file_path: track.file_path.to_string_lossy().to_string(),
            });
        }
    }

    pub fn parse(content: &str) -> Result<Self, String> {
        let mut metadata = PlaylistMetadata {
            name: String::new(),
            author: None,
            created_at: None,
        };
        let mut tracks = Vec::new();
        let mut in_metadata = false;
        let mut in_tracks = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed == "[Metadata]" {
                in_metadata = true;
                in_tracks = false;
                continue;
            } else if trimmed == "[Tracks]" {
                in_metadata = false;
                in_tracks = true;
                continue;
            }

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if in_metadata {
                if let Some(value) = trimmed.strip_prefix("name=") {
                    metadata.name = value.to_string();
                } else if let Some(value) = trimmed.strip_prefix("author=") {
                    metadata.author = Some(value.to_string());
                } else if let Some(value) = trimmed.strip_prefix("created_at=") {
                    metadata.created_at = Some(value.to_string());
                }
            } else if in_tracks {
                // Format: index|title|artist|file_path
                let parts: Vec<&str> = trimmed.split('|').collect();
                if parts.len() == 4 {
                    if let Ok(index) = parts[0].parse::<u32>() {
                        tracks.push(ProjectTrack {
                            index,
                            title: parts[1].to_string(),
                            artist: parts[2].to_string(),
                            file_path: parts[3].to_string(),
                        });
                    }
                }
            }
        }

        // Validate tracks are sequential 0-based
        let mut indices: Vec<u32> = tracks.iter().map(|t| t.index).collect();
        indices.sort();
        for (i, &idx) in indices.iter().enumerate() {
            if idx != i as u32 {
                return Err(format!("Track indices must be sequential 0-based; found {}", idx));
            }
        }

        Ok(ProjectFile {
            metadata,
            tracks,
            version: "1.0".to_string(),
        })
    }

    pub fn serialize(&self) -> String {
        let mut output = String::new();
        output.push_str("[Metadata]\n");
        output.push_str(&format!("name={}\n", self.metadata.name));
        if let Some(author) = &self.metadata.author {
            output.push_str(&format!("author={}\n", author));
        }
        if let Some(created_at) = &self.metadata.created_at {
            output.push_str(&format!("created_at={}\n", created_at));
        }
        output.push_str("\n[Tracks]\n");
        for track in &self.tracks {
            output.push_str(&format!(
                "{}|{}|{}|{}\n",
                track.index, track.title, track.artist, track.file_path
            ));
        }
        output
    }
}
