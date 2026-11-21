use musicbox_organizer::models::{
    ExportFormatSelection, PlaylistMetadata, ProjectFile, ProjectTrack,
};
use musicbox_organizer::services::export_config::ExportConfigService;
use std::fs::{self, File};
use tempfile::tempdir;

#[test]
fn test_clear_all_export_to_playlist_text() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let out_dir = root.join("playlist-text");
    fs::create_dir_all(&out_dir).unwrap();

    // Create orphaned file from previous export
    let orphaned = out_dir.join("old_export.txt");
    File::create(&orphaned).unwrap();
    assert!(orphaned.exists());

    let mut project = ProjectFile::new(PlaylistMetadata {
        name: "Test Playlist".to_string(),
        author: None,
        created_at: None,
    });
    project.tracks.push(ProjectTrack {
        index: 0,
        title: "Song 1".to_string(),
        artist: "Artist 1".to_string(),
        file_path: "/tmp/song1.mp3".to_string(),
    });

    let service = ExportConfigService::new();
    let selection = ExportFormatSelection {
        txt: true,
        rtf: true,
    };

    let exported_files = service.export_playlist(root, &selection, &project).unwrap();

    // Verify orphaned file was deleted per FR-008 clear-all
    assert!(!orphaned.exists());

    // Verify new files created
    assert_eq!(exported_files.len(), 2);
    assert!(out_dir.join("playlist.txt").exists());
    assert!(out_dir.join("playlist.rtf").exists());
}

#[test]
fn test_selective_format_export() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let out_dir = root.join("playlist-text");

    let project = ProjectFile::new(PlaylistMetadata {
        name: "Test Playlist".to_string(),
        author: None,
        created_at: None,
    });

    let service = ExportConfigService::new();
    let selection = ExportFormatSelection {
        txt: true,
        rtf: false,
    };

    let exported_files = service.export_playlist(root, &selection, &project).unwrap();

    assert_eq!(exported_files.len(), 1);
    assert!(out_dir.join("playlist.txt").exists());
    assert!(!out_dir.join("playlist.rtf").exists());
}
