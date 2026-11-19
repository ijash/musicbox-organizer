use musicbox_organizer::models::{PlaylistMetadata, ProjectFile, ProjectTrack};
use tempfile::NamedTempFile;

#[test]
fn test_project_file_parse_and_serialize() {
    let raw = r#"[Metadata]
name=Road Trip Mix
author=MusicLover
created_at=2026-11-19T00:00:00Z

[Tracks]
0|Track One|Artist A|/music/song1.mp3
1|Track Two|Artist B|/music/song2.mp3
"#;

    let project = ProjectFile::parse(raw).expect("Parsing should succeed");
    assert_eq!(project.metadata.name, "Road Trip Mix");
    assert_eq!(project.tracks.len(), 2);
    assert_eq!(project.tracks[0].title, "Track One");
    assert_eq!(project.tracks[1].artist, "Artist B");

    let serialized = project.serialize();
    assert!(serialized.contains("name=Road Trip Mix"));
    assert!(serialized.contains("0|Track One|Artist A|/music/song1.mp3"));

    let round_trip = ProjectFile::parse(&serialized).expect("Round trip should succeed");
    assert_eq!(round_trip.tracks.len(), 2);
}

#[test]
fn test_project_file_save_and_load() {
    let tmp_file = NamedTempFile::new().unwrap();
    let path = tmp_file.path();

    let mut project = ProjectFile::new(PlaylistMetadata {
        name: "Test Project".to_string(),
        author: Some("Tester".to_string()),
        created_at: Some("2026-11-19".to_string()),
    });
    project.tracks.push(ProjectTrack {
        index: 0,
        title: "Song 1".to_string(),
        artist: "Artist 1".to_string(),
        file_path: "/path/to/song1.mp3".to_string(),
    });

    project.save(path).expect("Save should succeed");
    let loaded = ProjectFile::load(path).expect("Load should succeed");

    assert_eq!(loaded.metadata.name, "Test Project");
    assert_eq!(loaded.tracks.len(), 1);
    assert_eq!(loaded.tracks[0].title, "Song 1");
}

#[test]
fn test_project_rejects_non_sequential_indices() {
    let invalid = r#"[Metadata]
name=Bad
[Tracks]
0|Song A|Artist|/a.mp3
2|Song B|Artist|/b.mp3
"#;
    let res = ProjectFile::parse(invalid);
    assert!(res.is_err());
}
