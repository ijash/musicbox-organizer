# Contract: GUI Components, CLI Entry Point, and Platform Interop (Feature 010)

## 1. Application Entry Point (`src/main.rs`) Contract

### Invocation Modes

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 1. Determine invocation context
    let args: Vec<String> = std::env::args().collect();
    let is_cli = args.len() > 1 && args[1] != "gui";

    // 2. On Windows: attach parent console if invoked via CLI
    #[cfg(windows)]
    if is_cli {
        unsafe {
            use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
            let _ = AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    // 3. Dispatch to CLI subcommands or eframe GUI
    if is_cli {
        handle_cli_command(&args);
    } else {
        launch_eframe_gui();
    }
}
```

### CLI Subcommand Invariants
All existing subcommands MUST maintain exact arguments and behaviors:
- `musicbox [gui]`: Launches `eframe` window.
- `musicbox scan [dir]`: Scans directory for MP3 files and prints ID3 metadata table to stdout.
- `musicbox tag <file> [title] [artist]`: Reads or updates ID3 tags.
- `musicbox status`: Lists active project details and detected storage devices.
- `musicbox save [path]`: Persists project file locally.
- `musicbox export`: Exports playlist to connected device.
- `musicbox export-text [dir]`: Generates TXT and RTF playlist summaries.
- `musicbox check-compat <dir>`: Validates audio files against primitive device constraints.
- `musicbox help` / `--help`: Displays command usage.

---

## 2. Desktop GUI Component Contract (`src/ui/`)

### Submodule Organization
- `src/ui/mod.rs`: Module declarations and public `MusicBoxApp` exports.
- `src/ui/app.rs`: Implements `eframe::App` with `update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)`.
- `src/ui/header.rs`: Renders top panel with title, track counter, `⮌ Undo`, `⮎ Redo`, and `🌙 Dark` / `☀ Light` theme toggle.
- `src/ui/toolbar.rs`: Compact action bar with `📁 Scan Folder...`, `➕ Add MP3`, `🗑 Remove`, `⬆ Up`, `⬇ Down`, `⚙ Audio Tools...`, `📄 Export Text/RTF...`, `Save`, `Open`.
- `src/ui/playlist_table.rs`: Central table with sortable `#`, `Export Filename`, inline-editable `Title` and `Artist`, and `Source File`.
- `src/ui/audio_bar.rs`: Bottom audio preview dock with `▶ Play`, `⏸ Pause`, `⏹ Stop`, volume slider, and track title indicator.
- `src/ui/dsp_dialog.rs`: Modal window for volume normalization and silence trimming with background worker thread spawning.
- `src/ui/export_panel.rs`: Removable storage controls, device dropdown, `Scan`, `Reformat before export` with two-step confirmation, `Subpath`, and `Export Playlist to Media`.
- `src/ui/footer.rs`: Status line reporting operational messages and clickable GitHub hyperlink.

### Visual Styling Contract
- Visual Theme: Uses `egui::Visuals::dark()` or `egui::Visuals::light()`.
- Spacing: Compact button and item spacing (`ui.spacing_mut().item_spacing = egui::vec2(6.0, 4.0)`).
- Icons: Rendered as Unicode glyphs (`▶`, `⏸`, `⏹`, `⬆`, `⬇`, `➕`, `🗑`, `⚙`, `📁`, `📄`, `⮌`, `⮎`) inheriting `ui.visuals().text_color()`.

---

## 3. Platform Device Enumeration Contract (`src/platform/windows.rs`)

### Windows `DeviceManager` Implementation
```rust
impl DeviceManager for WindowsDeviceManager {
    fn enumerate_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        // 1. Call Win32 GetLogicalDrives() to get bitmask of drive letters A..=Z
        // 2. For each active drive letter (e.g. "D:\\", "E:\\"):
        //    a. Call GetDriveTypeW(drive_path)
        //    b. If DRIVE_REMOVABLE (or DRIVE_FIXED when querying flash media):
        //       - Query volume label via GetVolumeInformationW
        //       - Query free/total space via GetDiskFreeSpaceExW
        //       - Populate Device struct (is_removable: true)
        // 3. Return Vec<Device>
    }
}
```

---

## 4. Headless GUI Test Contract (`tests/gui_tests.rs`)

### `egui_kittest` Integration
```rust
#[test]
fn test_gui_headless_playlist_interactions() {
    let mut app = MusicBoxApp::default();
    let mut harness = egui_kittest::Harness::new_ui(|ui| {
        app.render_central_content(ui);
    });

    // Step 1: Render and assert initial state
    harness.run();
    assert_eq!(app.project.tracks.len(), 0);

    // Step 2: Trigger widget clicks and assert mutations
    // ...
}
```
Invariants:
- Test suite MUST run headlessly across Linux, Windows, and macOS without display servers.
- Tests MUST NOT modify original files in `mp3-sample/`; always use `tempfile::tempdir()`.
- Zero compiler or clippy warnings under `cargo test`.
