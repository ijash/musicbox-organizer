use crate::models::{Device, FileSystem};
use crate::services::DeviceManager;
use std::error::Error;

pub struct MacOSDeviceManager;

impl Default for MacOSDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

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

    fn format_device(
        &self,
        _device_id: &str,
        _filesystem: FileSystem,
    ) -> Result<(), Box<dyn Error>> {
        Err("Device formatting is not yet supported on macOS".into())
    }

    fn eject_device(&self, _device_id: &str) -> Result<(), Box<dyn Error>> {
        Err("Safe eject is not yet supported on macOS".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macos_format_and_eject_unsupported() {
        let mgr = MacOSDeviceManager::new();
        let format_res = mgr.format_device("/dev/disk2s1", FileSystem::FAT32);
        assert!(format_res.is_err());
        assert_eq!(
            format_res.unwrap_err().to_string(),
            "Device formatting is not yet supported on macOS"
        );

        let eject_res = mgr.eject_device("/dev/disk2s1");
        assert!(eject_res.is_err());
        assert_eq!(
            eject_res.unwrap_err().to_string(),
            "Safe eject is not yet supported on macOS"
        );
    }
}
