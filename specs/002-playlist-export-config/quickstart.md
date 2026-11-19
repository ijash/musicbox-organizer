# Quickstart: Playlist Export Config

## Validation Scenarios

### 1. Clear-All Multi-Format Export
```sh
cargo test test_clear_all_export_to_playlist_text
```
Expected: Any existing files in `playlist-text/` are deleted, and only selected files (`playlist.txt`, `playlist.rtf`) are generated.

### 2. Preferences Persistence
```sh
cargo test test_user_preferences_load_save
```
Expected: Toggling dark mode to `false` and saving writes `.musicbox_config.json`. Reloading restores `dark_mode == false`.
