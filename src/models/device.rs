use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileSystem {
    FAT32,
    exFAT,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub capacity_bytes: u64,
    pub available_bytes: u64,
    pub filesystem: FileSystem,
    pub mount_point: Option<PathBuf>,
    pub is_removable: bool,
    pub is_write_protected: bool,
    pub last_seen: DateTime<Utc>,
}

impl Device {
    pub fn new(
        id: String,
        name: String,
        capacity_bytes: u64,
        available_bytes: u64,
        filesystem: FileSystem,
        mount_point: Option<PathBuf>,
        is_removable: bool,
        is_write_protected: bool,
    ) -> Result<Self, String> {
        // Validation per data-model.md
        if capacity_bytes == 0 {
            return Err("Device capacity must be > 0".to_string());
        }
        if available_bytes > capacity_bytes {
            return Err("Available bytes cannot exceed capacity".to_string());
        }
        if !is_removable {
            return Err("Device must be removable (USB/SD card)".to_string());
        }
        if is_write_protected {
            return Err("Device must not be write-protected".to_string());
        }

        Ok(Device {
            id,
            name,
            capacity_bytes,
            available_bytes,
            filesystem,
            mount_point,
            is_removable,
            is_write_protected,
            last_seen: Utc::now(),
        })
    }

    pub fn is_sufficient_space(&self, required_bytes: u64) -> bool {
        self.available_bytes >= required_bytes
    }

    pub fn is_disconnected_timeout(&self, timeout_secs: i64) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.last_seen)
            .num_seconds();
        elapsed >= timeout_secs
    }
}
