use musicbox_organizer::models::{
    Device, ExportJob, ExportStatus, FileSystem, PlaylistMetadata, ProjectFile, ProjectTrack, Track,
};
use musicbox_organizer::services::{DeviceManager, ExportService};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

struct TestDeviceManager {
    mock_mount: PathBuf,
}

impl DeviceManager for TestDeviceManager {
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        Ok(vec![Device::new(
            "test-sd".to_string(),
            "Test SD Card".to_string(),
            64_000_000,
            60_000_000,
            FileSystem::FAT32,
            Some(self.mock_mount.clone()),
            true,
            false,
        )?])
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

    fn format_device(&self, _device_id: &str, _filesystem: FileSystem) -> Result<(), Box<dyn Error>> {
        // Mock reformat by wiping mock_mount directory
        if self.mock_mount.exists() {
            for entry in fs::read_dir(&self.mock_mount)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    fs::remove_dir_all(entry.path())?;
                } else {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        Ok(())
    }

    fn eject_device(&self, _device_id: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[test]
fn test_end_to_end_export_workflow() {
    let src_dir = tempdir().unwrap();
    let dest_dir = tempdir().unwrap();

    let song1 = src_dir.path().join("first.mp3");
    let song2 = src_dir.path().join("second.mp3");
    File::create(&song1).unwrap().write_all(b"mp3_data_1").unwrap();
    File::create(&song2).unwrap().write_all(b"mp3_data_2").unwrap();

    let tracks = vec![
        Track {
            index: 0,
            source_path: song1,
            filename: "first.mp3".to_string(),
            xxyy_name: "AAAA".to_string(),
            file_size_bytes: 10,
        },
        Track {
            index: 1,
            source_path: song2,
            filename: "second.mp3".to_string(),
            xxyy_name: "AAAB".to_string(),
            file_size_bytes: 10,
        },
    ];

    let manager = Box::new(TestDeviceManager {
        mock_mount: dest_dir.path().to_path_buf(),
    });
    let service = ExportService::new(manager);

    let mut job = ExportJob::new("test-sd".to_string(), tracks, false).unwrap();
    service.export(&mut job, dest_dir.path()).unwrap();

    assert_eq!(job.status, ExportStatus::COMPLETED);
    assert_eq!(job.files_written, 2);

    let file1 = dest_dir.path().join("[AAAA-first.mp3]");
    let file2 = dest_dir.path().join("[AAAB-second.mp3]");
    assert!(file1.exists());
    assert!(file2.exists());
}

#[test]
fn test_reformat_and_export_workflow() {
    let src_dir = tempdir().unwrap();
    let dest_dir = tempdir().unwrap();

    // Place an orphaned file in dest_dir before reformat
    let orphan = dest_dir.path().join("old_junk.txt");
    File::create(&orphan).unwrap().write_all(b"junk").unwrap();
    assert!(orphan.exists());

    let song = src_dir.path().join("fresh.mp3");
    File::create(&song).unwrap().write_all(b"new_mp3").unwrap();

    let tracks = vec![Track {
        index: 0,
        source_path: song,
        filename: "fresh.mp3".to_string(),
        xxyy_name: "AAAA".to_string(),
        file_size_bytes: 7,
    }];

    let manager = Box::new(TestDeviceManager {
        mock_mount: dest_dir.path().to_path_buf(),
    });
    let service = ExportService::new(manager);

    let mut job = ExportJob::new("test-sd".to_string(), tracks, true).unwrap();
    job.reformat_confirmed = true;

    service.export(&mut job, dest_dir.path()).unwrap();

    assert_eq!(job.status, ExportStatus::COMPLETED);
    // Orphaned file was cleaned by reformat
    assert!(!orphan.exists());
    // Exported file exists
    assert!(dest_dir.path().join("[AAAA-fresh.mp3]").exists());
}

#[test]
fn test_save_vs_export_workflow_independence() {
    let project_dir = tempdir().unwrap();
    let project_path = project_dir.path().join("playlist.txt");

    let mut project = ProjectFile::new(PlaylistMetadata {
        name: "My Indie List".to_string(),
        author: Some("Dev".to_string()),
        created_at: Some("2026-11-19".to_string()),
    });
    project.tracks.push(ProjectTrack {
        index: 0,
        title: "Song 1".to_string(),
        artist: "Artist 1".to_string(),
        file_path: "/music/song1.mp3".to_string(),
    });

    // Save project locally
    project.save(&project_path).unwrap();
    assert!(project_path.exists());

    // Export playlist to separate destination
    let dest_dir = tempdir().unwrap();
    let song = project_dir.path().join("song1.mp3");
    File::create(&song).unwrap().write_all(b"audio").unwrap();

    let tracks = vec![Track {
        index: 0,
        source_path: song,
        filename: "song1.mp3".to_string(),
        xxyy_name: "AAAA".to_string(),
        file_size_bytes: 5,
    }];

    let manager = Box::new(TestDeviceManager {
        mock_mount: dest_dir.path().to_path_buf(),
    });
    let service = ExportService::new(manager);
    let mut job = ExportJob::new("test-sd".to_string(), tracks, false).unwrap();
    service.export(&mut job, dest_dir.path()).unwrap();

    // Verify local project file is untouched and does not exist on media
    assert!(project_path.exists());
    assert!(!dest_dir.path().join("playlist.txt").exists());
}
