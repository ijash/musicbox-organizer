use crate::models::{Device, FileSystem};
use crate::services::DeviceManager;
use std::error::Error;
use std::path::PathBuf;

pub struct WindowsDeviceManager;

impl Default for WindowsDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsDeviceManager {
    pub fn new() -> Self {
        WindowsDeviceManager
    }
}

impl DeviceManager for WindowsDeviceManager {
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        #[cfg(windows)]
        {
            use windows::Win32::Storage::FileSystem::{
                DRIVE_REMOVABLE, GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDrives,
                GetVolumeInformationW,
            };
            use windows::core::PCWSTR;

            let drives_mask = unsafe { GetLogicalDrives() };
            let mut devices = Vec::new();

            for i in 0..26 {
                if (drives_mask & (1 << i)) != 0 {
                    let letter = (b'A' + i) as char;
                    let root_str = format!("{}:\\\0", letter);
                    let root_wide: Vec<u16> = root_str.encode_utf16().collect();
                    let pcwstr = PCWSTR(root_wide.as_ptr());

                    let drive_type = unsafe { GetDriveTypeW(pcwstr) };
                    if drive_type == DRIVE_REMOVABLE {
                        let mut vol_name = [0u16; 256];
                        let mut fs_name = [0u16; 256];
                        let _ = unsafe {
                            GetVolumeInformationW(
                                pcwstr,
                                Some(&mut vol_name),
                                None,
                                None,
                                None,
                                Some(&mut fs_name),
                            )
                        };

                        let name = String::from_utf16_lossy(&vol_name)
                            .trim_matches('\0')
                            .to_string();
                        let display_name = if name.is_empty() {
                            format!("Removable Disk ({}:)", letter)
                        } else {
                            format!("{} ({}:)", name, letter)
                        };

                        let mut free_bytes = 0u64;
                        let mut total_bytes = 0u64;
                        let _ = unsafe {
                            GetDiskFreeSpaceExW(
                                pcwstr,
                                Some(&mut free_bytes),
                                Some(&mut total_bytes),
                                None,
                            )
                        };

                        let fs_str = String::from_utf16_lossy(&fs_name)
                            .trim_matches('\0')
                            .to_uppercase();
                        let fs = match fs_str.as_str() {
                            "FAT32" => FileSystem::FAT32,
                            "EXFAT" => FileSystem::exFAT,
                            other => FileSystem::Unknown(other.to_string()),
                        };

                        devices.push(Device {
                            id: format!("{}:", letter),
                            name: display_name,
                            capacity_bytes: total_bytes,
                            available_bytes: free_bytes,
                            filesystem: fs,
                            mount_point: Some(PathBuf::from(format!("{}:\\", letter))),
                            is_removable: true,
                            is_write_protected: false,
                            last_seen: chrono::Utc::now(),
                        });
                    }
                }
            }

            Ok(devices)
        }
        #[cfg(not(windows))]
        {
            Ok(Vec::new())
        }
    }

    fn detect_write_protected(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        Ok(false)
    }

    fn validate_removable(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn is_sufficient_space(&self, _required_bytes: u64) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn format_device(
        &self,
        _device_id: &str,
        _filesystem: FileSystem,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn eject_device(&self, _device_id: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
