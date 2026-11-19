use crate::models::{Device, FileSystem};
use crate::services::DeviceManager;
use std::error::Error;

/// Linux device manager using /sys/block enumeration (fallback when udisks2 DBus unavailable)
pub struct LinuxDeviceManager;

impl LinuxDeviceManager {
    pub fn new() -> Self {
        LinuxDeviceManager
    }

    /// Parse /sys/block to enumerate removable devices
    fn parse_sys_block_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        let mut devices = Vec::new();

        // Scan /sys/block for disk devices
        if let Ok(entries) = std::fs::read_dir("/sys/block") {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();

                // Skip non-removable candidates (loop, ram, etc.)
                if name_str.starts_with("loop") || name_str.starts_with("ram") || name_str.starts_with("fd") {
                    continue;
                }

                let removable_path = format!("/sys/block/{}/removable", name_str);
                let ro_path = format!("/sys/block/{}/ro", name_str);
                let size_path = format!("/sys/block/{}/size", name_str);

                // Check if removable
                let removable = {
                    let removable_f = std::fs::read_to_string(&removable_path);
                    match removable_f {
                        Ok(content) => content.trim() == "1",
                        Err(_) => continue, // Skip if can't determine
                    }
                };

                if !removable {
                    continue; // Skip internal drives
                }

                // Check read-only (write-protected)
                let is_readonly = {
                    let ro_f = std::fs::read_to_string(&ro_path);
                    match ro_f {
                        Ok(content) => content.trim() == "1",
                        Err(_) => false,
                    }
                };

                // Get size in bytes (1 sector = 512 bytes)
                let capacity_bytes = {
                    let size_f = std::fs::read_to_string(&size_path);
                    match size_f {
                        Ok(content) => {
                            let trimmed = content.trim();
                            trimmed.parse::<u64>().unwrap_or(0) * 512
                        }
                        Err(_) => 0,
                    }
                };

                if capacity_bytes == 0 {
                    continue; // Skip devices with unknown/zero size
                }

                // Get filesystem from /proc/mounts
                let filesystem = Self::get_filesystem_from_mounts(&name_str);

                // Build device ID (e.g., /dev/sda)
                let device_id = format!("/dev/{}", name_str);

                let mount_point = Self::get_mount_point_from_mounts(&name_str);

                devices.push(Device {
                    id: device_id,
                    name: name_str.to_string(),
                    capacity_bytes,
                    available_bytes: capacity_bytes, // Approximation; real available from mount
                    filesystem,
                    mount_point,
                    is_removable: true,
                    is_write_protected: is_readonly,
                    last_seen: chrono::Utc::now(),
                });
            }
        }

        Ok(devices)
    }

    /// Get filesystem type from /proc/mounts for a given disk
    fn get_filesystem_from_mounts(disk_name: &str) -> FileSystem {
        // Look through /proc/mounts for this disk
        if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    // Check if the mount source contains the disk name
                    if line.contains(disk_name) || line.contains(&format!("/dev/{}", disk_name)) {
                        return match parts[2] {
                            "vfat" => FileSystem::FAT32,
                            "exfat" | "EXFAT" => FileSystem::exFAT,
                            _ => FileSystem::Unknown(parts[2].to_string()),
                        };
                    }
                }
            }
        }
        FileSystem::Unknown(String::new())
    }

    /// Get mount point for a given disk from /proc/mounts
    fn get_mount_point_from_mounts(disk_name: &str) -> Option<std::path::PathBuf> {
        if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] != "proc" && parts[2] != "sysfs" && parts[2] != "devtmpfs" {
                    // Check if this mount involves our disk
                    if line.contains(disk_name) || line.contains(&format!("/dev/{}", disk_name)) {
                        return Some(std::path::PathBuf::from(parts[1]));
                    }
                }
            }
        }
        None
    }
}

impl DeviceManager for LinuxDeviceManager {
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        let devices = self.parse_sys_block_devices()?;
        Ok(devices)
    }

    fn detect_write_protected(&self, device_id: &str) -> Result<bool, Box<dyn Error>> {
        // Check /sys/block/sdX/ro for write-protected status
        let path_str = format!("/sys/block/{}/ro", device_id);
        let path = std::path::Path::new(&path_str);
        if let Ok(content) = std::fs::read_to_string(path) {
            return Ok(content.trim() == "1");
        }
        Ok(false)
    }

    fn validate_removable(&self, device_id: &str) -> Result<bool, Box<dyn Error>> {
        // Check /sys/block/sdX/removable flag
        let path_str = format!("/sys/block/{}/removable", device_id);
        let path = std::path::Path::new(&path_str);
        if let Ok(content) = std::fs::read_to_string(path) {
            return Ok(content.trim() == "1");
        }
        Ok(true) // Default to true if path not found
    }

    fn check_removable(&self, device_id: &str) -> Result<bool, Box<dyn Error>> {
        self.validate_removable(device_id)
    }

    fn is_sufficient_space(&self, required_bytes: u64) -> Result<bool, Box<dyn Error>> {
        // Get the device from enumeration to check its available_bytes
        // For now, we enumerate devices and find the matching one
        let devices = self.enumerate_devices()?;
        for device in &devices {
            if device.is_removable && device.capacity_bytes > 0 {
                return Ok(device.available_bytes >= required_bytes);
            }
        }
        // Device not found in current enumeration; default check (conservative)
        Ok(false)
    }

    fn format_device(&self, device_id: &str, filesystem: FileSystem) -> Result<(), Box<dyn Error>> {
        let cmd = match filesystem {
            FileSystem::FAT32 => format!("mkfs.fat -F 32 /dev/{}", device_id),
            FileSystem::exFAT => format!("mkfs.exfat /dev/{}", device_id),
            FileSystem::Unknown(_) => return Err("Unknown filesystem format".into()),
        };
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Format failed: {}", stderr).into());
        }
        Ok(())
    }

    fn eject_device(&self, device_id: &str) -> Result<(), Box<dyn Error>> {
        // Try udisks2 first, then fall back to umount + eject
        let result = std::process::Command::new("udisks2")
            .arg("--unmount")
            .arg(&device_id)
            .output();
        match result {
            Ok(_) => Ok(()),
            Err(_) => {
                // Fallback: unmount then eject
                let _ = std::process::Command::new("umount")
                    .arg(&device_id)
                    .output();
                let _ = std::process::Command::new("eject")
                    .arg(&device_id);
                Ok(())
            }
        }
    }
}
