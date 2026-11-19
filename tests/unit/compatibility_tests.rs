use musicbox_organizer::services::compatibility::CompatibilityService;
use musicbox_organizer::services::naming;

#[test]
fn test_lexicographic_ascii_sort_integrity() {
    let service = CompatibilityService::new();

    // Generate 1,000 sequential filenames
    let mut names = Vec::new();
    for i in 0..1000 {
        let name = naming::format_export_filename(i, "track.mp3").unwrap();
        names.push(name);
    }

    let original = names.clone();
    // Raw Rust sort() compares ASCII bytes directly
    names.sort();

    // Verify sort() order strictly matches index generation order (SC-001)
    assert_eq!(names, original);

    let report = service.validate_filenames(&names);
    assert!(report.is_sorted);
    assert!(report.ascii_violations.is_empty());
    assert!(report.length_violations.is_empty());
    assert!(report.format_violations.is_empty());
    assert!(report.overall_compatible);
}

#[test]
fn test_non_ascii_filename_detection_and_sanitization() {
    let service = CompatibilityService::new();

    let clean = "song_title-1.mp3";
    let unicode_cyrillic = "песня.mp3";
    let unicode_emoji = "track_🔥.mp3";

    assert!(service.is_ascii_only(clean));
    assert!(!service.is_ascii_only(unicode_cyrillic));
    assert!(!service.is_ascii_only(unicode_emoji));

    let sanitized = service.sanitize_filename("my_🔥_песня.mp3");
    assert!(service.is_ascii_only(&sanitized));
    assert!(!sanitized.contains('🔥'));
}

#[test]
fn test_path_length_validation() {
    let service = CompatibilityService::new();

    let short_name = "[AAAA-short.mp3]".to_string();
    let long_name = format!("[AAAA-{}.mp3]", "a".repeat(260));

    let report = service.validate_filenames(&[short_name, long_name]);
    assert_eq!(report.length_violations.len(), 1);
    assert!(!report.overall_compatible);
}
