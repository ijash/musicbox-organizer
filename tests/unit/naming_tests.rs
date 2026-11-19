use musicbox_organizer::services::naming::{
    format_export_filename, track_index_to_xxyy, xxyy_to_track_index,
};

#[test]
fn test_track_index_to_xxyy_sequences() {
    assert_eq!(track_index_to_xxyy(0).unwrap(), "AAAA");
    assert_eq!(track_index_to_xxyy(1).unwrap(), "AAAB");
    assert_eq!(track_index_to_xxyy(25).unwrap(), "AAAZ");
    assert_eq!(track_index_to_xxyy(26).unwrap(), "ABAA");
    assert_eq!(track_index_to_xxyy(17575).unwrap(), "ZZZZ");
}

#[test]
fn test_xxyy_to_track_index_round_trip() {
    assert_eq!(xxyy_to_track_index("AAAA").unwrap(), 0);
    assert_eq!(xxyy_to_track_index("AAAB").unwrap(), 1);
    assert_eq!(xxyy_to_track_index("ZZZZ").unwrap(), 17575);

    for idx in [0, 1, 25, 26, 675, 676, 17575] {
        let code = track_index_to_xxyy(idx).unwrap();
        assert_eq!(xxyy_to_track_index(&code).unwrap(), idx);
    }
}

#[test]
fn test_format_export_filename() {
    assert_eq!(
        format_export_filename(0, "song.mp3").unwrap(),
        "[AAAA-song.mp3]"
    );
    assert_eq!(
        format_export_filename(1, "track01.mp3").unwrap(),
        "[AAAB-track01.mp3]"
    );
}

#[test]
fn test_out_of_bounds_index() {
    assert!(track_index_to_xxyy(17576).is_err());
}
