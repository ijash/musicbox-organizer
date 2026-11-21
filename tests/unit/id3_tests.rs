use musicbox_organizer::services::id3_service::Id3Service;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_read_and_write_id3_tags() {
    let dir = tempdir().unwrap();
    let sample_src = Path::new("mp3-sample/sample-3s.mp3");
    let test_file = dir.path().join("test.mp3");

    fs::copy(sample_src, &test_file).expect("Failed to copy sample mp3");

    let service = Id3Service::new();

    // Read initial metadata
    let mut meta = service.read_tags(&test_file).expect("Failed to read tags");
    assert!(!meta.title.is_empty() || meta.title == "test");

    // Modify metadata
    meta.title = "New Song Title".to_string();
    meta.artist = "New Artist Name".to_string();
    meta.album = "New Album Name".to_string();
    meta.track_number = Some(42);

    service
        .write_tags(&test_file, &meta)
        .expect("Failed to write tags");

    // Read back and verify
    let updated = service
        .read_tags(&test_file)
        .expect("Failed to read updated tags");
    assert_eq!(updated.title, "New Song Title");
    assert_eq!(updated.artist, "New Artist Name");
    assert_eq!(updated.album, "New Album Name");
    assert_eq!(updated.track_number, Some(42));
}

#[test]
fn test_rename_with_xxyy_prefix() {
    let dir = tempdir().unwrap();
    let sample_src = Path::new("mp3-sample/sample-3s.mp3");
    let test_file = dir.path().join("song.mp3");
    fs::copy(sample_src, &test_file).unwrap();

    let service = Id3Service::new();
    let renamed = service
        .apply_xxyy_rename(&test_file, 0)
        .expect("Failed to rename");

    assert_eq!(
        renamed.file_name().unwrap().to_string_lossy(),
        "[AAAA-song.mp3]"
    );
    assert!(renamed.exists());
    assert!(!test_file.exists());
}
