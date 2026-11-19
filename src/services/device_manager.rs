use crate::models::{Device, FileSystem};
use std::error::Error;

/// Platform abstraction for device enumeration and management
/// cite: data-model.md Device validation rules
pub trait DeviceManager: Send + Sync {
    /// Enumerate all removable storage devices currently connected
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>>;

    /// Check if device is write-protected before attempting writes
    fn detect_write_protected(&self, device_id: &str) -> Result<bool, Box<dyn Error>>;

    /// Validate device is removable (not internal drive)
    fn validate_removable(&self, device_id: &str) -> Result<bool, Box<dyn Error>>;

    /// Format device to FAT32 or exFAT
    fn format_device(&self, device_id: &str, filesystem: FileSystem) -> Result<(), Box<dyn Error>>;

    /// Eject/unmount device safely
    fn eject_device(&self, device_id: &str) -> Result<(), Box<dyn Error>>;
}

/// Factory for platform-specific DeviceManager
pub fn create_device_manager() -> Box<dyn DeviceManager> {
    #[cfg(target_os = "windows")]
    {
        Box::new(crate::platform::windows::WindowsDeviceManager::new())
    }

    #[cfg(target_os = "macos")]
    {
        Box::new(crate::platform::macos::MacOSDeviceManager::new())
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(crate::platform::linux::LinuxDeviceManager::new())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        compile_error!("Unsupported platform")
    }
}
