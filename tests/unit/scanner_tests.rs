use musicbox_organizer::services::scanner::DirectoryScanner;
use std::fs::{self, File};
use tempfile::tempdir;

#[test]
fn test_scanner_finds_only_mp3_files_in_root() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create mp3 files
    File::create(root.join("b_song.mp3")).unwrap();
    File::create(root.join("a_song.mp3")).unwrap();

    // Create non-mp3 files
    File::create(root.join("notes.txt")).unwrap();
    File::create(root.join("cover.jpg")).unwrap();

    // Create subdirectory with mp3 (should be ignored per FR-001)
    let sub = root.join("subdir");
    fs::create_dir(&sub).unwrap();
    File::create(sub.join("sub_song.mp3")).unwrap();

    let scanner = DirectoryScanner::new();
    let results = scanner.scan_directory(root).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].original_filename, "a_song.mp3");
    assert_eq!(results[0].index, 0);
    assert_eq!(results[0].xxyy_prefix, "AAAA");

    assert_eq!(results[1].original_filename, "b_song.mp3");
    assert_eq!(results[1].index, 1);
    assert_eq!(results[1].xxyy_prefix, "AAAB");
}

#[test]
fn test_scanner_empty_dir() {
    let dir = tempdir().unwrap();
    let scanner = DirectoryScanner::new();
    let results = scanner.scan_directory(dir.path()).unwrap();
    assert!(results.is_empty());
}
