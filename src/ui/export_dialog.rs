use crate::models::{Device, ExportJob, FileSystem, Track};
use crate::services::{DeviceManager, ExportService};
use crate::ui::warnings::ReformatWarningDialog;
use std::error::Error;
use std::path::{Path, PathBuf};

/// Export Dialog Controller
/// cite: contracts/export-dialog-ui.md, spec.md FR-001, FR-002, FR-012, SC-001, SC-008
pub struct ExportDialog {
    pub devices: Vec<Device>,
    pub selected_device_id: Option<String>,
    pub is_loading: bool,
    pub reformat_enabled: bool,
    pub selected_filesystem: FileSystem,
    pub export_subpath: PathBuf,
    pub warning_dialog: Option<ReformatWarningDialog>,
    pub current_job: Option<ExportJob>,
}

impl ExportDialog {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            selected_device_id: None,
            is_loading: false,
            reformat_enabled: false,
            selected_filesystem: FileSystem::FAT32,
            export_subpath: PathBuf::from("/"),
            warning_dialog: None,
            current_job: None,
        }
    }

    /// Refresh connected removable devices list
    pub fn refresh_devices(&mut self, manager: &dyn DeviceManager) -> Result<(), Box<dyn Error>> {
        self.is_loading = true;
        let devices = manager.enumerate_devices()?;
        self.devices = devices;
        self.is_loading = false;

        // Reset selected device if no longer connected
        if let Some(id) = &self.selected_device_id {
            if !self.devices.iter().any(|d| &d.id == id) {
                self.selected_device_id = None;
            }
        }

        // Auto-select first device if available and none selected
        if self.selected_device_id.is_none() && !self.devices.is_empty() {
            self.selected_device_id = Some(self.devices[0].id.clone());
        }

        Ok(())
    }

    pub fn selected_device(&self) -> Option<&Device> {
        self.selected_device_id
            .as_ref()
            .and_then(|id| self.devices.iter().find(|d| &d.id == id))
    }

    /// Select device by ID
    pub fn select_device(&mut self, device_id: String) {
        if self.devices.iter().any(|d| d.id == device_id) {
            self.selected_device_id = Some(device_id);
        }
    }

    /// Set reformat checkbox state
    pub fn set_reformat(&mut self, enabled: bool) {
        self.reformat_enabled = enabled;
    }

    /// Empty state message when no removable devices detected
    pub fn empty_state_message(&self) -> &'static str {
        "No removable devices detected. Connect a USB drive or SD card to proceed."
    }

    /// Check if export button can be triggered
    pub fn can_export(&self) -> bool {
        self.selected_device().is_some()
    }

    /// Trigger export initiation (shows warning dialog if reformat checked)
    pub fn trigger_export(&mut self, tracks: Vec<Track>) -> Result<(), String> {
        let (device_id, device_name, capacity_bytes) = {
            let device = self.selected_device().ok_or("No device selected")?;
            (device.id.clone(), device.name.clone(), device.capacity_bytes)
        };

        if self.reformat_enabled {
            self.warning_dialog = Some(ReformatWarningDialog::new(
                device_name,
                capacity_bytes,
            ));
            let job = ExportJob::new(device_id, tracks, true)?;
            self.current_job = Some(job);
            Ok(())
        } else {
            let job = ExportJob::new(device_id, tracks, false)?;
            self.current_job = Some(job);
            Ok(())
        }
    }

    /// User confirms reformat warning modal
    pub fn confirm_reformat_warning(&mut self) {
        if let Some(warning) = &mut self.warning_dialog {
            warning.confirm();
            if let Some(job) = &mut self.current_job {
                job.reformat_confirmed = true;
            }
        }
    }

    /// User cancels reformat warning modal
    pub fn cancel_reformat_warning(&mut self) {
        if let Some(warning) = &mut self.warning_dialog {
            warning.cancel();
            if let Some(job) = &mut self.current_job {
                job.reformat_confirmed = false;
                job.status = crate::models::ExportStatus::PENDING;
            }
        }
    }

    /// Execute export using service
    pub fn execute_export(
        &mut self,
        export_service: &ExportService,
    ) -> Result<(), Box<dyn Error>> {
        let device = self.selected_device().ok_or("No device selected")?;
        let mount_path = device.mount_point.clone().unwrap_or_else(|| PathBuf::from("/"));
        let dest = if self.export_subpath == Path::new("/") {
            mount_path
        } else {
            mount_path.join(&self.export_subpath)
        };

        if let Some(job) = &mut self.current_job {
            export_service.export(job, &dest)?;
        }
        Ok(())
    }

    /// Eject selected device
    pub fn eject_selected(&self, export_service: &ExportService) -> Result<(), Box<dyn Error>> {
        if let Some(device) = self.selected_device() {
            export_service.eject(&device.id)?;
        }
        Ok(())
    }
}
