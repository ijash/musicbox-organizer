use crate::models::{Device, FileSystem};
use crate::services::DeviceManager;
use std::error::Error;

pub struct WindowsDeviceManager;

impl WindowsDeviceManager {
    pub fn new() -> Self {
        WindowsDeviceManager
    }
}

impl DeviceManager for WindowsDeviceManager {
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        // TODO: Implement Win32 SetupAPI enumeration per research.md
        // Use windows crate SetupDiGetClassDevsW, SetupDiEnumDeviceInfo
        // Filter: DIGCF_PRESENT, GUID_DEVCLASS_DISKDRIVE, StorageDeviceNumber property
        Ok(Vec::new())
    }

    fn detect_write_protected(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        // TODO: Query write-protect flag via Win32 IOCTL_DISK_GET_DRIVE_GEOMETRY_EX
        Ok(false)
    }

    fn validate_removable(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        // TODO: Query removable flag from device properties
        Ok(true)
    }

    fn is_sufficient_space(&self, _required_bytes: u64) -> Result<bool, Box<dyn Error>> {
        // TODO: Query free space via Win32 GetDiskFreeSpaceExW
        Ok(true)
    }

    fn format_device(&self, _device_id: &str, _filesystem: FileSystem) -> Result<(), Box<dyn Error>> {
        // TODO: Implement FAT32/exFAT format via Win32 FormatEx or command-line tool
        Ok(())
    }

    fn eject_device(&self, _device_id: &str) -> Result<(), Box<dyn Error>> {
        // TODO: Implement device ejection via Win32 CM_Request_Device_Eject_Ex
        Ok(())
    }
}
