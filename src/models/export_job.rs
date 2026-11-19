use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportStatus {
    PENDING,
    FORMATTING,
    IN_PROGRESS,
    COMPLETED,
    FAILED,
    ABORTED,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub index: u32,
    pub source_path: PathBuf,
    pub filename: String,
    pub xxyy_name: String,
    pub file_size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJob {
    pub id: String,
    pub device_id: String,
    pub status: ExportStatus,
    pub tracks: Vec<Track>,
    pub total_bytes: u64,
    pub bytes_written: u64,
    pub files_written: usize,
    pub reformat_enabled: bool,
    pub reformat_confirmed: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

impl ExportJob {
    pub fn new(device_id: String, tracks: Vec<Track>, reformat_enabled: bool) -> Result<Self, String> {
        if tracks.is_empty() {
            return Err("Export job must have at least 1 track".to_string());
        }

        // Verify tracks have unique, sequential 0-based indices
        let mut sorted_indices: Vec<u32> = tracks.iter().map(|t| t.index).collect();
        sorted_indices.sort();
        for (i, &idx) in sorted_indices.iter().enumerate() {
            if idx != i as u32 {
                return Err(format!("Track indices must be sequential 0-based; found {}", idx));
            }
        }

        let total_bytes = tracks.iter().map(|t| t.file_size_bytes).sum();

        Ok(ExportJob {
            id: uuid::Uuid::new_v4().to_string(),
            device_id,
            status: ExportStatus::PENDING,
            tracks,
            total_bytes,
            bytes_written: 0,
            files_written: 0,
            reformat_enabled,
            reformat_confirmed: false,
            started_at: None,
            completed_at: None,
            error_message: None,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.status != ExportStatus::PENDING {
            return Err(format!("Cannot start export from {:?} state", self.status));
        }
        self.status = ExportStatus::IN_PROGRESS;
        self.started_at = Some(Utc::now());
        Ok(())
    }

    pub fn start_formatting(&mut self) -> Result<(), String> {
        if self.status != ExportStatus::PENDING {
            return Err(format!("Cannot start formatting from {:?} state", self.status));
        }
        if !self.reformat_enabled {
            return Err("Reformat not enabled".to_string());
        }
        if !self.reformat_confirmed {
            return Err("Reformat must be confirmed before starting".to_string());
        }
        self.status = ExportStatus::FORMATTING;
        Ok(())
    }

    pub fn format_complete(&mut self) -> Result<(), String> {
        if self.status != ExportStatus::FORMATTING {
            return Err(format!("Cannot complete format from {:?} state", self.status));
        }
        self.status = ExportStatus::IN_PROGRESS;
        self.started_at = Some(Utc::now());
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), String> {
        if self.status != ExportStatus::IN_PROGRESS {
            return Err(format!("Cannot complete from {:?} state", self.status));
        }
        self.status = ExportStatus::COMPLETED;
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    pub fn fail(&mut self, error_msg: String) -> Result<(), String> {
        match self.status {
            ExportStatus::FORMATTING | ExportStatus::IN_PROGRESS => {
                self.status = ExportStatus::FAILED;
                self.error_message = Some(error_msg);
                self.completed_at = Some(Utc::now());
                // On failure during format: revert to PENDING for retry
                if self.reformat_enabled && self.status == ExportStatus::FAILED {
                    self.status = ExportStatus::PENDING;
                }
                Ok(())
            }
            _ => Err(format!("Cannot fail from {:?} state", self.status)),
        }
    }

    pub fn abort(&mut self) -> Result<(), String> {
        match self.status {
            ExportStatus::FORMATTING | ExportStatus::IN_PROGRESS => {
                self.status = ExportStatus::ABORTED;
                self.completed_at = Some(Utc::now());
                Ok(())
            }
            _ => Err(format!("Cannot abort from {:?} state", self.status)),
        }
    }

    pub fn progress_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.bytes_written as f32 / self.total_bytes as f32) * 100.0
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.status, ExportStatus::COMPLETED | ExportStatus::FAILED | ExportStatus::ABORTED)
    }
}
