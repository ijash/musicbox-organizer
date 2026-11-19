use musicbox_organizer::models::{Device, FileSystem};
use std::path::PathBuf;

#[test]
fn test_device_validation_success() {
    let device = Device::new(
        "/dev/sdb1".to_string(),
        "SanDisk USB".to_string(),
        16_000_000_000,
        8_000_000_000,
        FileSystem::FAT32,
        Some(PathBuf::from("/media/usb")),
        true,
        false,
    );
    assert!(device.is_ok());
    let dev = device.unwrap();
    assert!(dev.is_sufficient_space(4_000_000_000));
    assert!(!dev.is_sufficient_space(10_000_000_000));
}

#[test]
fn test_device_rejects_internal_drive() {
    let result = Device::new(
        "/dev/sda".to_string(),
        "Internal NVMe".to_string(),
        500_000_000_000,
        250_000_000_000,
        FileSystem::exFAT,
        Some(PathBuf::from("/")),
        false, // Not removable
        false,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Device must be removable (USB/SD card)");
}

#[test]
fn test_device_rejects_write_protected() {
    let result = Device::new(
        "/dev/sdc1".to_string(),
        "Read-Only SD".to_string(),
        32_000_000_000,
        16_000_000_000,
        FileSystem::FAT32,
        Some(PathBuf::from("/media/sd")),
        true,
        true, // Write protected
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Device must not be write-protected");
}

#[test]
fn test_device_rejects_zero_capacity() {
    let result = Device::new(
        "/dev/sdd1".to_string(),
        "Empty Device".to_string(),
        0,
        0,
        FileSystem::FAT32,
        None,
        true,
        false,
    );
    assert!(result.is_err());
}
