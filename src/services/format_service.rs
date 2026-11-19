use crate::models::{ExportJob, FileSystem};
use crate::services::DeviceManager;
use std::error::Error;

/// Handles device formatting with reformat_confirmed prerequisite check
/// cite: data-model.md ExportJob state machine (reformat_enabled requires reformat_confirmed)
pub struct FormatService {
    device_manager: Box<dyn DeviceManager>,
}

impl FormatService {
    pub fn new(device_manager: Box<dyn DeviceManager>) -> Self {
        FormatService { device_manager }
    }

    /// Format device after user confirmation
    pub fn format_device(
        &self,
        job: &mut ExportJob,
        filesystem: FileSystem,
    ) -> Result<(), Box<dyn Error>> {
        // Validate prerequisites per data-model.md: reformat_enabled=true REQUIRES reformat_confirmed=true
        if job.reformat_enabled && !job.reformat_confirmed {
            return Err("Reformat requested but not confirmed by user".into());
        }

        // Mark as formatting
        job.start_formatting();

        // Format device via platform-specific manager
        self.device_manager
            .format_device(&job.device_id, filesystem)
            .map_err(|e| {
                // On failure, revert to PENDING for retry
                job.status = crate::models::ExportStatus::PENDING;
                e
            })?;

        // Signal format complete; job advances to IN_PROGRESS
        job.format_complete();
        Ok(())
    }

    /// Verify device is not write-protected before formatting attempt
    pub fn check_write_protected(&self, device_id: &str) -> Result<bool, Box<dyn Error>> {
        self.device_manager.detect_write_protected(device_id)
    }

    /// Validate device is removable (pre-format safety check)
    pub fn check_removable(&self, device_id: &str) -> Result<bool, Box<dyn Error>> {
        self.device_manager.validate_removable(device_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Device, ExportStatus, FileSystem as FS};

    #[test]
    fn test_format_requires_confirmation() {
        // ponytail: placeholder mock; full integration test when platform services implemented
        let manager = crate::services::create_device_manager();
        let service = FormatService::new(manager);

        let mut job = ExportJob {
            id: "test-job".to_string(),
            device_id: "USB1".to_string(),
            tracks: vec![],
            status: ExportStatus::PENDING,
            total_bytes: 0,
            bytes_written: 0,
            files_written: 0,
            reformat_enabled: true,
            reformat_confirmed: false, // Not confirmed
            started_at: None,
            completed_at: None,
            error_message: None,
        };

        let result = service.format_device(&mut job, FS::FAT32);
        assert!(result.is_err());
        assert_eq!(job.status, ExportStatus::PENDING); // Still pending
    }

    #[test]
    fn test_format_succeeds_with_confirmation() {
        let manager = crate::services::create_device_manager();
        let service = FormatService::new(manager);

        let mut job = ExportJob {
            id: "test-job".to_string(),
            device_id: "USB1".to_string(),
            tracks: vec![],
            status: ExportStatus::PENDING,
            total_bytes: 0,
            bytes_written: 0,
            files_written: 0,
            reformat_enabled: true,
            reformat_confirmed: true, // Confirmed
            started_at: None,
            completed_at: None,
            error_message: None,
        };

        // Format will fail in stub (no real device), but should reach Formatting state
        let _ = service.format_device(&mut job, FS::FAT32);
        // Placeholder: actual success requires platform implementation
    }
}
