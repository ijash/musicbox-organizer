use musicbox_organizer::models::UserPreferences;
use tempfile::tempdir;

#[test]
fn test_user_preferences_load_save() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Default should be dark mode
    let default_prefs = UserPreferences::load_from_dir(root);
    assert!(default_prefs.dark_mode);
    assert!(default_prefs.formats.txt);
    assert!(default_prefs.formats.rtf);

    // Save modified
    let mut modified = default_prefs;
    modified.dark_mode = false;
    modified.formats.rtf = false;
    modified.save_to_dir(root).expect("Failed to save preferences");

    // Reload and check
    let reloaded = UserPreferences::load_from_dir(root);
    assert!(!reloaded.dark_mode);
    assert!(reloaded.formats.txt);
    assert!(!reloaded.formats.rtf);
}
