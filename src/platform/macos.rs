use crate::models::{Device, FileSystem};
use crate::services::DeviceManager;
use std::error::Error;

pub struct MacOSDeviceManager;

impl MacOSDeviceManager {
    pub fn new() -> Self {
        MacOSDeviceManager
    }
}

impl DeviceManager for MacOSDeviceManager {
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        // TODO: Implement DADiskArbitration enumeration per research.md
        // Use objc FFI: DADiskCreateFromBSDName, DADiskCopyDescription
        // Extract: kDADiskDescriptionMediaNameKey, kDADiskDescriptionMediaSizeKey
        // Filter: kDADiskDescriptionMediaRemovableKey == true
        Ok(Vec::new())
    }

    fn detect_write_protected(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        // TODO: Query from DADiskCopyDescription write-protect property
        Ok(false)
    }

    fn validate_removable(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        // TODO: Verify removable flag via DADiskCopyDescription
        Ok(true)
    }

    fn is_sufficient_space(&self, _required_bytes: u64) -> Result<bool, Box<dyn Error>> {
        // TODO: Query free space via statfs
        Ok(true)
    }

    fn format_device(&self, _device_id: &str, _filesystem: FileSystem) -> Result<(), Box<dyn Error>> {
        // TODO: Format via diskutil or platform native API
        Ok(())
    }

    fn eject_device(&self, _device_id: &str) -> Result<(), Box<dyn Error>> {
        // TODO: Eject via diskutil eject or IOKit
        Ok(())
    }
}
