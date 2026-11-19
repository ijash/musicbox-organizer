use crate::models::{Device, FileSystem};
use crate::services::DeviceManager;
use std::error::Error;

pub struct LinuxDeviceManager;

impl LinuxDeviceManager {
    pub fn new() -> Self {
        LinuxDeviceManager
    }
}

impl DeviceManager for LinuxDeviceManager {
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        // TODO: Implement udisks2 DBus enumeration per research.md
        // Use dbus crate: org.freedesktop.UDisks2.Manager.GetBlockDevices()
        // Fallback: parse /sys/block/sd*, /sys/block/mmc* + /proc/mounts
        // Filter: Drive.Removable == true, Block.ReadOnly == false
        Ok(Vec::new())
    }

    fn detect_write_protected(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        // TODO: Query from /sys/block/sdX/ro or DBus property
        Ok(false)
    }

    fn validate_removable(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        // TODO: Check /sys/block/sdX/removable or DBus property
        Ok(true)
    }

    fn format_device(&self, _device_id: &str, _filesystem: FileSystem) -> Result<(), Box<dyn Error>> {
        // TODO: Format via mkfs or platform tool (mkfs.vfat, mkfs.exfat)
        Ok(())
    }

    fn eject_device(&self, _device_id: &str) -> Result<(), Box<dyn Error>> {
        // TODO: Eject via udisks2 DBus or unmount/eject commands
        Ok(())
    }
}
