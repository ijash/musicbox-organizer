use musicbox_organizer::models::{Device, ExportJob, ExportStatus, FileSystem, Track};
use musicbox_organizer::services::{DeviceManager, ExportService, FormatService};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

struct MockDeviceManager {
    is_removable: bool,
    has_space: bool,
    write_protected: bool,
}

impl DeviceManager for MockDeviceManager {
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        Ok(vec![Device::new(
            "mock-usb".to_string(),
            "Mock USB".to_string(),
            10_000_000,
            8_000_000,
            FileSystem::FAT32,
            Some(PathBuf::from("/mock/mount")),
            self.is_removable,
            self.write_protected,
        )?])
    }

    fn detect_write_protected(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        Ok(self.write_protected)
    }

    fn validate_removable(&self, _device_id: &str) -> Result<bool, Box<dyn Error>> {
        Ok(self.is_removable)
    }

    fn is_sufficient_space(&self, _required_bytes: u64) -> Result<bool, Box<dyn Error>> {
        Ok(self.has_space)
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

#[test]
fn test_export_successful_file_copy() {
    let temp_src = tempdir().unwrap();
    let temp_dest = tempdir().unwrap();

    let song1_path = temp_src.path().join("song1.mp3");
    let mut file = File::create(&song1_path).unwrap();
    file.write_all(b"audio dummy data 1").unwrap();

    let tracks = vec![Track {
        index: 0,
        source_path: song1_path,
        filename: "song1.mp3".to_string(),
        xxyy_name: "AAAA".to_string(),
        file_size_bytes: 18,
    }];

    let mut job = ExportJob::new("mock-usb".to_string(), tracks, false).unwrap();
    let manager = Box::new(MockDeviceManager {
        is_removable: true,
        has_space: true,
        write_protected: false,
    });
    let service = ExportService::new(manager);

    let res = service.export(&mut job, temp_dest.path());
    assert!(res.is_ok());
    assert_eq!(job.status, ExportStatus::COMPLETED);
    assert_eq!(job.files_written, 1);

    let expected_dest_file = temp_dest.path().join("[AAAA-song1.mp3]");
    assert!(expected_dest_file.exists());
}

#[test]
fn test_export_rejects_internal_device() {
    let temp_dest = tempdir().unwrap();
    let tracks = vec![Track {
        index: 0,
        source_path: PathBuf::from("/nonexistent.mp3"),
        filename: "song.mp3".to_string(),
        xxyy_name: "AAAA".to_string(),
        file_size_bytes: 100,
    }];
    let mut job = ExportJob::new("mock-internal".to_string(), tracks, false).unwrap();
    let manager = Box::new(MockDeviceManager {
        is_removable: false,
        has_space: true,
        write_protected: false,
    });
    let service = ExportService::new(manager);

    let res = service.export(&mut job, temp_dest.path());
    assert!(res.is_err());
    assert_eq!(job.status, ExportStatus::FAILED);
}

#[test]
fn test_reformat_requires_confirmed_flag() {
    let tracks = vec![Track {
        index: 0,
        source_path: PathBuf::from("/fake.mp3"),
        filename: "fake.mp3".to_string(),
        xxyy_name: "AAAA".to_string(),
        file_size_bytes: 10,
    }];
    let mut job = ExportJob::new("mock-usb".to_string(), tracks, true).unwrap();
    assert!(!job.reformat_confirmed);

    let manager = Box::new(MockDeviceManager {
        is_removable: true,
        has_space: true,
        write_protected: false,
    });
    let format_service = FormatService::new(manager);
    let res = format_service.format_device(&mut job, FileSystem::FAT32);
    assert!(res.is_err());
    assert_eq!(job.status, ExportStatus::PENDING);

    job.reformat_confirmed = true;
    let res2 = format_service.format_device(&mut job, FileSystem::FAT32);
    assert!(res2.is_ok());
    assert_eq!(job.status, ExportStatus::IN_PROGRESS);
}
