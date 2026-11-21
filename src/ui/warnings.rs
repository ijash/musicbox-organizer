//! Modal warning dialog for device reformatting
//! cite: spec.md FR-004, SC-002, contracts/export-dialog-ui.md

pub struct ReformatWarningDialog {
    pub device_name: String,
    pub capacity_gb: f64,
    pub is_open: bool,
    pub confirmed: bool,
}

impl ReformatWarningDialog {
    pub fn new(device_name: String, capacity_bytes: u64) -> Self {
        let capacity_gb = capacity_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        Self {
            device_name,
            capacity_gb,
            is_open: true,
            confirmed: false,
        }
    }

    pub fn message(&self) -> String {
        format!(
            "WARNING: Device Will Be Erased\n\n\
            Device: {} ({:.1} GB)\n\n\
            All data on this device will be PERMANENTLY DELETED.\n\
            This action cannot be undone.",
            self.device_name, self.capacity_gb
        )
    }

    /// Confirm action: user clicks Confirm
    pub fn confirm(&mut self) {
        self.confirmed = true;
        self.is_open = false;
    }

    /// Cancel action: user clicks Cancel
    pub fn cancel(&mut self) {
        self.confirmed = false;
        self.is_open = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning_dialog_flow() {
        let mut dialog =
            ReformatWarningDialog::new("USB Drive".to_string(), 8 * 1024 * 1024 * 1024);
        assert!(dialog.is_open);
        assert!(!dialog.confirmed);
        assert!(dialog.message().contains("USB Drive"));
        assert!(dialog.message().contains("PERMANENTLY DELETED"));

        dialog.confirm();
        assert!(!dialog.is_open);
        assert!(dialog.confirmed);
    }
}
