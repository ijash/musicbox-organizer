use crate::models::{ExportJob, Track};
use crate::services::DeviceManager;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Handles MP3 export to removable media with progress tracking
/// cite: data-model.md ExportJob state machine (PENDING→IN_PROGRESS→COMPLETED/FAILED/ABORTED)
pub struct ExportService {
    device_manager: Box<dyn DeviceManager>,
}

impl ExportService {
    pub fn new(device_manager: Box<dyn DeviceManager>) -> Self {
        ExportService { device_manager }
    }

    /// Start export job: validate device, check space, begin copy
    pub fn export(&self, job: &mut ExportJob, dest_path: &Path) -> Result<(), Box<dyn Error>> {
        // Validate device exists and has sufficient space
        let is_removable = self.device_manager.check_removable(&job.device_id)?;
        if !is_removable {
            let msg =
                "Device is internal (not removable). Select a USB or SD card instead.".to_string();
            job.fail(msg.clone()).ok();
            return Err(msg.into());
        }
        if job.total_bytes > 0 && !self.device_manager.is_sufficient_space(job.total_bytes)? {
            let msg = format!(
                "Insufficient space. Device does not have enough space for {} bytes.",
                job.total_bytes
            );
            job.fail(msg.clone()).ok();
            return Err(msg.into());
        }

        // Handle reformat before export if enabled
        if job.reformat_enabled {
            if !job.reformat_confirmed {
                let msg = "Reformat requested but not confirmed by user".to_string();
                job.fail(msg.clone()).ok();
                return Err(msg.into());
            }
            job.start_formatting()
                .map_err(|e| -> Box<dyn Error> { e.into() })?;
            self.device_manager
                .format_device(&job.device_id, crate::models::FileSystem::FAT32)
                .map_err(|e| {
                    job.fail(format!("Format failed: {}", e)).ok();
                    e
                })?;
            job.format_complete()
                .map_err(|e| -> Box<dyn Error> { e.into() })?;
        } else {
            job.start().map_err(|e| -> Box<dyn Error> { e.into() })?;
        }

        // Ensure dest_path directory exists
        if !dest_path.exists() {
            fs::create_dir_all(dest_path)?;
        }

        // Copy each track with [XXYY-filename.mp3] naming convention and disconnection check
        let mut copy_err = None;
        for (idx, track) in job.tracks.iter_mut().enumerate() {
            // Check for device disconnection before copying next file
            if !dest_path.exists() || self.device_manager.check_removable(&job.device_id).is_err() {
                copy_err = Some(
                    "Device disconnected during export. Please reconnect and retry.".to_string(),
                );
                break;
            }

            let source = Path::new(&track.source_path);
            let formatted_filename = if track.xxyy_name.ends_with(".mp3") {
                track
                    .xxyy_name
                    .trim_matches(|c| c == '[' || c == ']')
                    .to_string()
            } else {
                format!(
                    "{}-{}",
                    track.xxyy_name.trim_matches(|c| c == '[' || c == ']'),
                    track.filename
                )
            };
            let dest_file = dest_path.join(&formatted_filename);

            // Copy file
            if let Err(e) = fs::copy(source, &dest_file) {
                copy_err = Some(format!("Failed to copy {}: {}", track.filename, e));
                break;
            }

            // Update progress
            match fs::metadata(&dest_file) {
                Ok(meta) => {
                    track.file_size_bytes = meta.len();
                    job.bytes_written += track.file_size_bytes;
                    job.files_written = idx + 1;
                }
                Err(e) => {
                    copy_err = Some(format!(
                        "Failed to read metadata for {}: {}",
                        track.filename, e
                    ));
                    break;
                }
            }
        }

        if let Some(msg) = copy_err {
            job.fail(msg.clone()).ok();
            return Err(msg.into());
        }

        job.complete().map_err(|e| -> Box<dyn Error> { e.into() })?;
        Ok(())
    }

    /// Eject/unmount device safely
    pub fn eject(&self, device_id: &str) -> Result<(), Box<dyn Error>> {
        self.device_manager.eject_device(device_id)
    }

    /// Copy single track with overwrite check
    pub fn copy_track(
        &self,
        source: &Path,
        dest: &Path,
        allow_overwrite: bool,
    ) -> Result<(), Box<dyn Error>> {
        if dest.exists() && !allow_overwrite {
            return Err("Destination file already exists".into());
        }
        fs::copy(source, dest)?;
        Ok(())
    }

    /// Validate source tracks exist and are readable
    pub fn validate_source_files(&self, tracks: &[Track]) -> Result<(), Box<dyn Error>> {
        for track in tracks {
            let source = Path::new(&track.source_path);
            if !source.exists() {
                return Err(format!("Source file not found: {}", track.filename).into());
            }
            if !source.is_file() {
                return Err(format!("Source is not a file: {}", track.filename).into());
            }
            // Verify readable by attempting metadata
            fs::metadata(source)?;
        }
        Ok(())
    }

    /// Calculate total size of tracks for device space check
    pub fn calculate_total_size(&self, tracks: &[Track]) -> u64 {
        tracks.iter().map(|t| t.file_size_bytes).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_calculate_total_size() {
        let tracks = vec![
            Track {
                index: 0,
                filename: "song1.mp3".to_string(),
                source_path: PathBuf::from("/tmp/song1.mp3"),
                xxyy_name: "AAAA.mp3".to_string(),
                file_size_bytes: 1000,
            },
            Track {
                index: 1,
                filename: "song2.mp3".to_string(),
                source_path: PathBuf::from("/tmp/song2.mp3"),
                xxyy_name: "AAAB.mp3".to_string(),
                file_size_bytes: 2000,
            },
        ];

        // ponytail: placeholder; mock DeviceManager when service used in integration tests
        let manager = crate::services::create_device_manager();
        let service = ExportService::new(manager);

        assert_eq!(service.calculate_total_size(&tracks), 3000);
    }
}
