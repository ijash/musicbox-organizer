use crate::models::{ExportJob, Track};
use crate::services::DeviceManager;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

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
        let total_size: u64 = job.tracks.iter().map(|t| t.file_size_bytes).sum();
        let is_removable = self.device_manager.check_removable(&job.device_id)?;
        if !is_removable {
            job.fail("Device is internal (not removable). Select a USB or SD card instead.".to_string());
            return Err("Device is internal".into());
        }
        if job.total_bytes > 0 && !self.device_manager.is_sufficient_space(job.total_bytes)? {
            job.fail("Insufficient space on device".to_string());
            return Err("Device has insufficient space".into());
        }

        // Mark job as in-progress
        job.start();

        // Copy each track
        for (idx, track) in job.tracks.iter_mut().enumerate() {
            let source = Path::new(&track.source_path);
            let dest_file = dest_path.join(&track.xxyy_name);

            // Copy file
            fs::copy(source, &dest_file)
                .map_err(|e| format!("Failed to copy {}: {}", track.filename, e))?;

            // Update progress
            track.file_size_bytes = fs::metadata(&dest_file)?.len();
            job.bytes_written += track.file_size_bytes;
            job.files_written = idx + 1;
        }

        job.complete();
        Ok(())
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
